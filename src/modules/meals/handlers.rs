use axum::{Json, extract::Path};
use axum_typed_multipart::TypedMultipart;

use crate::{
    core::extractors::AuthUser,
    error::AppError,
    modules::meals::{
        models::{MealResponse, ScanMealAIResponse, ScanMealRequest},
        services::MealService,
    },
};

pub async fn get_meal_handler(
    service: MealService,
    AuthUser(user): AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<MealResponse>, AppError> {
    service.get_meal(id, user.sub).await
}

pub async fn get_meals_handler(
    service: MealService,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<MealResponse>>, AppError> {
    service.get_meals(user.sub).await
}

pub async fn scan_meal_handler(
    service: MealService,
    AuthUser(user): AuthUser,
    TypedMultipart(request): TypedMultipart<ScanMealRequest>,
) -> Result<Json<ScanMealAIResponse>, AppError> {
    tracing::info!(
        "Scanning meal for user {}, {:?}",
        user.sub,
        request.image.metadata.content_type
    );
    service
        .scan_meal(user.sub, request.image.contents, "jpeg".to_string())
        .await
}
