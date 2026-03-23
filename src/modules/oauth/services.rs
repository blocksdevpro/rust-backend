use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use reqwest::Url;

use crate::core::config::Config;
use crate::error::AppError;
use crate::modules::oauth::models::{JwtClaims, OAuthTokenResponse, OAuthUserInfoResponse};
use crate::state::AppState;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const GOOGLE_SCOPE: &str = "openid email profile";

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
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
    jwt_service: JwtService,
    oauth_service: GoogleOAuthService,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        Self {
            jwt_service: JwtService::new(state.config.jwt_secret.clone()),
            oauth_service: GoogleOAuthService::new(
                state.http.clone(),
                state.config.google_client_id.clone(),
                state.config.google_client_secret.clone(),
                state.config.google_redirect_uri.clone(),
            ),
        }
    }

    pub async fn login() {}
    pub async fn logout() {}
    pub async fn callback() {}
    pub async fn me() {}
}
