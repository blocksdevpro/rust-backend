use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct JwtClaims {
    pub sub: Uuid,     // user's UUID from your DB (not google_id)
    pub email: String, // handy to have without a DB lookup
    pub iat: u64,      // issued at (unix timestamp)
    pub exp: u64,      // expiry (unix timestamp)
    pub jti: String,   // unique token ID (for revocation later)
}

#[derive(Deserialize, Serialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Deserialize, Serialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct OAuthUserInfoResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}
