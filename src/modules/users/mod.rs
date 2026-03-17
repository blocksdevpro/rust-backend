use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use uuid::Uuid;
pub mod model;

use crate::{
    AppState,
    error::AppError,
    modules::users::model::{User, UserResponse},
};

async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error while fetching for user with id: {}, Err: {}", id, e);
            AppError::ItemNotFound(Some("User not found".to_string()))
        })?;
    Ok(Json(UserResponse::from(user)))
}

async fn get_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users;")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error while fetching users, Err: {}", e);
            AppError::ItemNotFound(Some("User not found".to_string()))
        })?;

    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/users", get(get_users_handler))
        .route("/users/{id}", get(get_user_handler))
}
