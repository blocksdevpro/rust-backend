use axum::Json;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use reqwest::Url;
use serde_json::{Value, json};
use time::Duration;
use uuid::Uuid;

use crate::error::AppError;
use crate::modules::auth::models::{
    JwtClaims, OAuthCallbackQuery, OAuthTokenResponse, OAuthUserInfoResponse,
};
use crate::modules::usersv2::models::UserResponse;
use crate::modules::usersv2::repo::UserRepository;
use crate::state::AppState;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const GOOGLE_SCOPE: &str = "openid email profile";

pub struct JwtService {
    secret: String,
    expiration: u32,
}

impl JwtService {
    pub fn new(secret: String, expiration: u32) -> Self {
        Self { secret, expiration }
    }

    pub fn generate_claims(&self, user_id: Uuid, email: String) -> JwtClaims {
        let iat = jsonwebtoken::get_current_timestamp() as u64;
        JwtClaims {
            sub: user_id,
            email,
            iat,
            exp: iat + self.expiration as u64,
            jti: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn encode(&self, payload: &JwtClaims) -> Result<String, AppError> {
        jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            payload,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| {
            tracing::error!("Failed to encode JWT: {:#?}", e);
            AppError::JwtError("Failed to encode JWT".to_string())
        })
    }

    pub fn decode(&self, token: &str) -> Result<JwtClaims, AppError> {
        let decoded = jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| {
            tracing::error!("Failed to decode JWT: {:#?}", e);
            AppError::JwtError("Failed to decode JWT".to_string())
        })?;

        Ok(decoded.claims)
    }
}

pub struct GoogleOAuthService {
    http: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GoogleOAuthService {
    pub fn new(
        http: reqwest::Client,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            http,
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub fn authorize_url(&self) -> Result<(String, String), AppError> {
        let state = uuid::Uuid::new_v4().to_string();

        let auth_url = Url::parse_with_params(
            GOOGLE_AUTH_URL,
            &[
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("response_type", "code"),
                ("scope", GOOGLE_SCOPE),
                ("state", state.as_str()),
            ],
        )
        .map_err(|e| {
            tracing::error!("Failed to parse auth URL: {:#?}", e);
            AppError::InternalServerError(None)
        })?
        .to_string();

        Ok((auth_url, state))
    }

    pub async fn exchange_code(&self, code: &str) -> Result<OAuthTokenResponse, AppError> {
        self.http
            .post(GOOGLE_TOKEN_URL)
            .form(&[
                ("code", code),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Token exchange request failed: {:#?}", e);
                AppError::InternalServerError(None)
            })?
            .json::<OAuthTokenResponse>()
            .await
            .map_err(|e| {
                tracing::error!("Failed to deserialize token response: {:#?}", e);
                AppError::InternalServerError(None)
            })
    }

    pub async fn userinfo(&self, token: &str) -> Result<OAuthUserInfoResponse, AppError> {
        self.http
            .get(GOOGLE_USERINFO_URL)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Userinfo request failed: {:#?}", e);
                AppError::InternalServerError(None)
            })?
            .json::<OAuthUserInfoResponse>()
            .await
            .map_err(|e| {
                tracing::error!("Failed to deserialize userinfo response: {:#?}", e);
                AppError::InternalServerError(None)
            })
    }
}

pub struct AuthService {
    jwt: JwtService,
    users: UserRepository,
    oauth: GoogleOAuthService,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        Self {
            jwt: JwtService::new(state.config.jwt_secret.clone(), state.config.jwt_expiration),
            users: UserRepository::new(state.pool.clone()),
            oauth: GoogleOAuthService::new(
                state.http.clone(),
                state.config.google_client_id.clone(),
                state.config.google_client_secret.clone(),
                state.config.google_redirect_uri.clone(),
            ),
        }
    }

    pub async fn login(&self, jar: CookieJar) -> Result<(CookieJar, Redirect), AppError> {
        let (url, state) = self.oauth.authorize_url()?;
        let cookie = Cookie::build(("oauth_state", state))
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::minutes(5));

        Ok((jar.add(cookie), Redirect::to(&url)))
    }
    pub async fn logout(&self, jar: CookieJar) -> Result<(CookieJar, Json<Value>), AppError> {
        Ok((
            jar.remove(Cookie::from("access_token")),
            Json(json!({"message": "Logged out"})),
        ))
    }
    pub async fn callback(
        &self,
        jar: CookieJar,
        query: OAuthCallbackQuery,
    ) -> Result<(CookieJar, Redirect), AppError> {
        let state = jar
            .get("oauth_state")
            .ok_or(AppError::CsrfMismatch)?
            .value()
            .to_string();

        if state != query.state {
            Err(AppError::CsrfMismatch)?;
        }

        let token_response = self.oauth.exchange_code(&query.code).await?;
        let userinfo = self.oauth.userinfo(&token_response.access_token).await?;

        let user = self
            .users
            .upsert(
                &userinfo.id,
                &userinfo.name,
                &userinfo.email,
                userinfo.picture.as_deref(),
            )
            .await?;

        let claims = self.jwt.generate_claims(user.id, user.email);
        let token = self.jwt.encode(&claims)?;

        let cookie = Cookie::build(("access_token", token))
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::hours(24));

        Ok((
            jar.add(cookie).remove(Cookie::new("oauth_state", "")),
            Redirect::to("http://localhost:8080/auth/me"),
        ))
    }
    pub async fn me(&self, user: JwtClaims) -> Result<Json<UserResponse>, AppError> {
        let user = self.users.find_by_id(user.sub).await?;

        let user = user.ok_or(AppError::ItemNotFound(Some("User not found".to_string())))?;

        Ok(Json(UserResponse::from(user)))
    }
}
