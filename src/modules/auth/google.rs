use reqwest::Client;
use serde::Deserialize;

use crate::modules::auth::error::AuthError;

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

pub async fn fetch_userinfo(http: &Client, token: &str) -> Result<GoogleUserInfo, AuthError> {
    let res = http
        .get("https://www.googleapis.com/oauth2/v1/userinfo")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AuthError::FailedToGetUserInfo)?
        .json::<GoogleUserInfo>()
        .await
        .map_err(|_| AuthError::FailedToGetUserInfo)?;

    Ok(res)
}
