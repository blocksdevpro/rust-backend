use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;
pub mod model;

use crate::{
    AppState,
    modules::{
        auth::error::AuthError,
        users::model::{User, UserResponse},
    },
};

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AuthError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error while fetching for user with id: {}, Err: {}", id, e);
            AuthError::FailedToGetUserInfo
        })?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AuthError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users;")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error while fetching users, Err: {}", e.to_string());
            AuthError::FailedToGetUserInfo
        })?;

    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}
