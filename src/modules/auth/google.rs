use reqwest::Client;
use serde::Deserialize;

use crate::{config::Config, modules::auth::error::AuthError};

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
}

pub fn authorize_url(config: &Config) -> (String, String) {
    let state = uuid::Uuid::new_v4().to_string();

    let url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        config.google_auth_uri,
        config.google_client_id,
        config.google_redirect_uri,
        "email profile openid",
        state
    );

    (url, state)
}

pub async fn exchange_code(
    http: &Client,
    code: &str,
    config: &Config,
) -> Result<GoogleTokenResponse, AuthError> {
    let response = http
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", config.google_client_id.as_str()),
            ("client_secret", config.google_client_secret.as_str()),
            ("redirect_uri", config.google_redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| {
            println!("Token exchange request failed: {:#?}", e);
            AuthError::FailedToExchangeToken
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".to_string());
        println!("Token exchange failed — status: {}, body: {}", status, body);
        return Err(AuthError::FailedToExchangeToken);
    }

    let res = response.json::<GoogleTokenResponse>().await.map_err(|e| {
        println!("Failed to deserialize token response: {:#?}", e);
        AuthError::FailedToExchangeToken
    })?;

    println!("\n\nJson: {:#?}\n\n", res);
    Ok(res)
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
