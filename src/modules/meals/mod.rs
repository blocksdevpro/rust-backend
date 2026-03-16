use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    modules::meals::model::{Meal, MealResponse},
};

mod model;

#[axum::debug_handler]
async fn get_meals_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<MealResponse>>, AppError> {
    let meals: Vec<Meal> = sqlx::query_as::<_, Meal>("SELECT * FROM meals")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch meals: {}", e);
            AppError::ItemNotFound(Some("Meals not found.".to_string()))
        })?;

    Ok(Json(
        meals
            .into_iter()
            .map(|meal| MealResponse::from(meal))
            .collect(),
    ))
}

async fn get_meal_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MealResponse>, AppError> {
    let meal = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch meal: {}", e);
            AppError::ItemNotFound(Some("Meal not found.".to_string()))
        })?;

    Ok(Json(MealResponse::from(meal)))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/meals", get(get_meals_handler))
        .route("/meals/{id}", get(get_meal_handler))
}
