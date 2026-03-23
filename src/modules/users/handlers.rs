use axum::{Json, extract::Path};
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::users::{models::UserResponse, services::UserService},
};

pub async fn get_user_handler(
    service: UserService,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    service.get_user(id).await
}

pub async fn get_users_handler(service: UserService) -> Result<Json<Vec<UserResponse>>, AppError> {
    service.get_users().await
}
