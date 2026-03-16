use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

pub enum AppError {
    ItemNotFound(Option<String>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::ItemNotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg.unwrap_or("Item not found.".to_string()),
            ),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
