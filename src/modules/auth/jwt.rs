use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::Config, modules::auth::error::AuthError};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub sub: Uuid,     // user's UUID from your DB (not google_id)
    pub email: String, // handy to have without a DB lookup
    pub iat: usize,    // issued at (unix timestamp)
    pub exp: usize,    // expiry (unix timestamp)
    pub jti: String,   // unique token ID (for revocation later)
}

pub fn decode_token(token: &str, config: &Config) -> Result<JwtPayload, AuthError> {
    let decoded = jsonwebtoken::decode::<JwtPayload>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(decoded) => Ok(decoded.claims),
        Err(_) => Err(AuthError::InvalidToken),
    }
}

pub fn encode_token(payload: &JwtPayload, config: &Config) -> Result<String, AuthError> {
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
