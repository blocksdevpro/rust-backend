use axum::Json;

use crate::{
    error::AppError,
    modules::profiles::{
        models::{CreateProfileRequest, ProfileResponse, TargetsResponse, UpdateProfileRequest},
        repo::ProfileRepository,
    },
    state::AppState,
};

pub struct ProfileService {
    profiles: ProfileRepository,
}

impl ProfileService {
    pub fn new(state: AppState) -> Self {
        Self {
            profiles: ProfileRepository::new(state.pool.clone()),
        }
    }

    pub async fn get_profile(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Json<ProfileResponse>, AppError> {
        let profile = self.profiles.find_by_user_id(user_id).await?;

        profile.map_or(
            Err(AppError::ItemNotFound(Some(
                "Profile not found".to_string(),
            ))),
            |p| Ok(Json(ProfileResponse::from(p))),
        )
    }

    pub async fn get_profile_targets(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Json<TargetsResponse>, AppError> {
        let profile = self.profiles.find_by_user_id(user_id).await?;

        profile.map_or(
            Err(AppError::ItemNotFound(Some(
                "Targets not found".to_string(),
            ))),
            |p| Ok(Json(TargetsResponse::from(p))),
        )
    }

    pub async fn create_profile(
        &self,
        user_id: uuid::Uuid,
        profile: CreateProfileRequest,
    ) -> Result<Json<ProfileResponse>, AppError> {
        let profile = self.profiles.create(user_id, profile).await?;

        Ok(Json(ProfileResponse::from(profile)))
    }

    pub async fn update_profile(
        &self,
        user_id: uuid::Uuid,
        profile: UpdateProfileRequest,
    ) -> Result<Json<ProfileResponse>, AppError> {
        let profile = self.profiles.update(user_id, profile).await?;

        Ok(Json(ProfileResponse::from(profile)))
    }
}
