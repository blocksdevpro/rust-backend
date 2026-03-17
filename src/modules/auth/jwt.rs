use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

use crate::{AppState, config::Config, modules::auth::error::AuthError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,     // user's UUID from your DB (not google_id)
    pub email: String, // handy to have without a DB lookup
    pub iat: usize,    // issued at (unix timestamp)
    pub exp: usize,    // expiry (unix timestamp)
    pub jti: String,   // unique token ID (for revocation later)
}

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = get_access_token(parts)?;
        let claims = decode_token(&token, &state.config)?;
        Ok(AuthUser(claims))
    }
}

pub fn get_access_token(parts: &Parts) -> Result<String, AuthError> {
    let jar = CookieJar::from_headers(&parts.headers);

    jar.get("access_token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::MissingCookie)
}

pub fn set_access_token(jar: CookieJar, token: String) -> CookieJar {
    let cookie = Cookie::build(("access_token", token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::hours(24))
        .build();

    jar.add(cookie)
}

pub fn decode_token(token: &str, config: &Config) -> Result<Claims, AuthError> {
    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(decoded) => Ok(decoded.claims),
        Err(_) => Err(AuthError::InvalidToken),
    }
}

pub fn encode_token(payload: &Claims, config: &Config) -> Result<String, AuthError> {
    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        payload,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::InvalidToken),
    }
}
