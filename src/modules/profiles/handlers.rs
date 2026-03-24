use axum::Json;

use crate::{
    core::extractors::AuthUser,
    error::AppError,
    modules::profiles::{
        models::{CreateProfileRequest, ProfileResponse, TargetsResponse, UpdateProfileRequest},
        services::ProfileService,
    },
};

pub async fn get_profile_handler(
    service: ProfileService,
    AuthUser(user): AuthUser,
) -> Result<Json<ProfileResponse>, AppError> {
    service.get_profile(user.sub).await
}

pub async fn get_targets_handler(
    service: ProfileService,
    AuthUser(user): AuthUser,
) -> Result<Json<TargetsResponse>, AppError> {
    service.get_profile_targets(user.sub).await
}

pub async fn create_profile_handler(
    service: ProfileService,
    AuthUser(user): AuthUser,
    Json(profile): Json<CreateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    service.create_profile(user.sub, profile).await
}

pub async fn update_profile_handler(
    service: ProfileService,
    AuthUser(user): AuthUser,
    Json(profile): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    service.update_profile(user.sub, profile).await
}
