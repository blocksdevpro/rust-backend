use axum::response::IntoResponse;
use reqwest::StatusCode;

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
        let (status, error_message) = match self {
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

        (status, error_message).into_response()
    }
}
