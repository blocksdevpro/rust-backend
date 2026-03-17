use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

pub enum AppError {
    BadRequest(Option<String>),
    ItemNotFound(Option<String>),
    Unauthorized(Option<String>),
    InternalServerError(Option<String>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg.unwrap_or("Bad request.".to_string()),
            ),
            AppError::ItemNotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg.unwrap_or("Item not found.".to_string()),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                msg.unwrap_or("Unauthorized.".to_string()),
            ),
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg.unwrap_or("Internal server error.".to_string()),
            ),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
