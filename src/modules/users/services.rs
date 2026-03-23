use axum::Json;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::users::{models::UserResponse, repo::UserRepository},
    state::AppState,
};

pub struct UserService {
    users: UserRepository,
}

impl UserService {
    pub fn new(state: AppState) -> Self {
        let users = UserRepository::new(state.pool.clone());
        Self { users }
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Json<UserResponse>, AppError> {
        if let Some(user) = self.users.find_by_id(id).await? {
            Ok(Json(UserResponse::from(user)))
        } else {
            Err(AppError::ItemNotFound(Some("User not found".to_string())))
        }
    }

    pub async fn get_users(&self) -> Result<Json<Vec<UserResponse>>, AppError> {
        let users = self.users.find_all().await?;
        Ok(Json(users.into_iter().map(UserResponse::from).collect()))
    }
}
