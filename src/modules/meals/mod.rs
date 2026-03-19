pub mod model;

use crate::{
    AppState,
    core::{openai, prompts},
    error::AppError,
    modules::{
        auth::jwt::AuthUser,
        meals::model::{Meal, MealResponse, MealType},
        profiles::model::{Profile, TargetsResponse},
    },
};

use axum::extract::Multipart;
use axum::{
    Json, Router,
    body::Bytes,
    extract::{Path, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use base64::{Engine as _, engine::general_purpose};

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScanMealAIResponse {
    pub name: String,
    pub description: String,
    pub meal_type: MealType,

    pub fats: f32,
    pub carbs: f32,
    pub fiber: f32,
    pub protein: f32,
    pub calories: f32,

    pub confidence: f32,
    pub reasoning: String,
}

pub struct ScanMealRequest {
    pub picture: Bytes,
}

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

async fn scan_meal_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    mut multipart: Multipart,
) -> Result<Json<MealResponse>, AppError> {
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
    let content_type = content_type
        .ok_or_else(|| AppError::BadRequest(Some("Image content_type is required".to_string())))?;

    let content_ext = match content_type.as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        _ => {
            return Err(AppError::BadRequest(Some(
                "Unsupported image type".to_string(),
            )));
        }
    };

    if content_bytes.len() > MAX_IMAGE_SIZE {
        return Err(AppError::BadRequest(Some(
            "Image size is too large (max 10MB)".to_string(),
        )));
    }
    // convert content_bytes to base64.
    let base64_content_bytes = general_purpose::STANDARD.encode(&content_bytes);
    let content = format!(
        "data:{media_type};base64,{base64_string}",
        media_type = content_type,
        base64_string = base64_content_bytes
    );
    // save the file in cloudflare r2.
    let key = format!("meals/{}/{}.{}", user.sub, Uuid::new_v4(), content_ext);
    state
        .r2
        .put_object()
        .bucket(&state.config.cf_r2_bucket)
        .key(&key)
        .body(content_bytes.into())
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to upload object to R2 bucket, {}", e);
            AppError::InternalServerError(Some(
                "Failed to store image inside r2 bucket.".to_string(),
            ))
        })?;

    tracing::info!("Saved image in {}", key);

    // TODO: implement scan meal handler.
    let target = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE user_id = $1")
        .bind(user.sub)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch profile: {}", e);
            AppError::ItemNotFound(Some("Profile not found.".to_string()))
        })?;

    let target = target.map(TargetsResponse::from);

    let user_prompt = prompts::build_user_scan_prompt(None, target.as_ref());
    let request = openai::build_chat_completion_request(
        prompts::SCAN_SYSTEM_PROMPT.to_string(),
        user_prompt,
        content,
    )
    .map_err(|e| {
        tracing::error!("Failed to build chat completion request: {}", e);
        AppError::InternalServerError(Some("Failed to build chat completion request.".to_string()))
    })?;
    let result = state.openai.chat().create(request).await.map_err(|e| {
        tracing::error!("Failed to create chat completion: {}", e);
        AppError::InternalServerError(Some("Failed to create chat completion.".to_string()))
    })?;

    let response = result
        .choices
        .first()
        .and_then(|c| c.message.content.as_ref())
        .ok_or_else(|| {
            tracing::error!("Failed to get chat completion response.");
            AppError::InternalServerError(Some(
                "Failed to get chat completion response.".to_string(),
            ))
        })?;
    let scan_response = serde_json::from_str::<ScanMealAIResponse>(response).map_err(|e| {
        tracing::error!("Failed to parse chat completion response: {}", e);
        AppError::InternalServerError(Some(
            "Failed to parse chat completion response.".to_string(),
        ))
    })?;

    // insert meal
    let meal = sqlx::query_as::<_, Meal>(
        "INSERT INTO meals (user_id, name, meal_type, fats, carbs, fiber, protein, calories, confidence) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(user.sub)
    .bind(scan_response.name)
    .bind(scan_response.meal_type)
    .bind(scan_response.fats)
    .bind(scan_response.carbs)
    .bind(scan_response.fiber)
    .bind(scan_response.protein)
    .bind(scan_response.calories)
    .bind(scan_response.confidence)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert meal: {}", e);
        AppError::InternalServerError(Some("Failed to insert meal.".to_string()))
    })?;

    Ok(Json(MealResponse::from(meal)))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/meals", get(get_meals_handler))
        .route("/meals/scan", post(scan_meal_handler))
        .route("/meals/{id}", get(get_meal_handler))
}
