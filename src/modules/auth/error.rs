use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    CsrfMismatch,
    MissingCookie,
    FailedHttpClient,
    FailedToUpsertUser,

    // OAuth errors
    FailedToGetUserInfo,
    FailedToExchangeToken,

    // JWT errors
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::CsrfMismatch => (StatusCode::FORBIDDEN, "CSRF state mismatch"),
            AuthError::MissingCookie => (StatusCode::UNAUTHORIZED, "Missing cookie"),
            AuthError::FailedHttpClient => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build http client!",
            ),
            AuthError::FailedToUpsertUser => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to upsert user in db!",
            ),
            AuthError::FailedToGetUserInfo => {
                (StatusCode::UNAUTHORIZED, "Failed to get user info!")
            }
            AuthError::FailedToExchangeToken => (StatusCode::UNAUTHORIZED, "Invalid oauth2_token!"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };

        (status, Json(json!({"error": message}))).into_response()
    }
}
