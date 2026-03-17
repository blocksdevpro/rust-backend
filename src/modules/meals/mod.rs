mod model;

use crate::{
    AppState,
    error::AppError,
    modules::{
        auth::jwt::AuthUser,
        meals::model::{Meal, MealResponse},
    },
};
use axum::extract::Multipart;
use axum::{
    Json, Router,
    body::Bytes,
    extract::{Path, State},
    routing::{get, post},
};
use uuid::Uuid;

pub struct ScanMealRequest {
    pub picture: Bytes,
}

#[axum::debug_handler]
async fn get_meals_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<MealResponse>>, AppError> {
    let meals: Vec<Meal> = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE user_id = $1")
        .bind(user.sub)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch meals: {}", e);
            AppError::ItemNotFound(Some("Meals not found.".to_string()))
        })?;

    Ok(Json(meals.into_iter().map(MealResponse::from).collect()))
}

async fn get_meal_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<MealResponse>, AppError> {
    let meal = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.sub)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch meal: {}", e);
            AppError::ItemNotFound(Some("Meal not found.".to_string()))
        })?;

    Ok(Json(MealResponse::from(meal)))
}

#[axum::debug_handler]
async fn scan_meal_handler(
    State(_state): State<AppState>,
    AuthUser(_user): AuthUser,
    mut multipart: Multipart,
) -> Result<(), AppError> {
    // TODO: handle the image.
    let mut content_bytes: Option<Bytes> = None;
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest(Some("Failed to read multipart".to_string())))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "image" {
            content_type = field.content_type().map(|ct| ct.to_string());

            let bytes = field.bytes().await.map_err(|e| {
                tracing::error!("Failed to read image bytes: {}", e);
                AppError::BadRequest(Some("Failed to read image bytes".to_string()))
            })?;
            content_bytes = Some(bytes);
        }
    }

    let content_bytes =
        content_bytes.ok_or_else(|| AppError::BadRequest(Some("Image is required".to_string())))?;

    // TEMP: save the file in cloudflare r2.

    // TODO: implement scan meal handler.

    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/meals", get(get_meals_handler))
        .route("/meals/scan", post(scan_meal_handler))
        .route("/meals/{id}", get(get_meal_handler))
}
