use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: String,
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
