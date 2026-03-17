use axum::{
    Json, Router,
    extract::State,
    routing::{get, post, put},
};
use serde::Deserialize;

use crate::{
    AppState,
    error::AppError,
    modules::{
        auth::jwt::AuthUser,
        profiles::model::{Activity, Gender, Goal, Profile, ProfileResponse, TargetsResponse},
    },
};

mod model;

#[derive(Deserialize)]
struct CreateProfileRequest {
    age: i32,
    height: i32,
    weight: i32,
    goal: Goal,
    activity: Activity,
    gender: Gender,
}

#[derive(Deserialize)]
struct UpdateProfileRequest {
    age: Option<i32>,
    height: Option<i32>,
    weight: Option<i32>,
    goal: Option<Goal>,
    activity: Option<Activity>,
    gender: Option<Gender>,
}

async fn get_profile_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE user_id = $1")
        .bind(user.sub)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch profile, Err: {}", e);
            AppError::InternalServerError(Some("Failed to fetch profile.".to_string()))
        })?
        .ok_or(AppError::ItemNotFound(Some(
            "Profile not found.".to_string(),
        )))?;

    Ok(Json(ProfileResponse::from(profile)))
}

async fn get_targets_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<TargetsResponse>, AppError> {
    let profile = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE user_id = $1")
        .bind(user.sub)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch profile, Err: {}", e);
            AppError::InternalServerError(Some("Failed to fetch profile.".to_string()))
        })?
        .ok_or(AppError::ItemNotFound(Some(
            "Profile not found.".to_string(),
        )))?;

    Ok(Json(TargetsResponse::from(profile)))
}

async fn update_profile_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = sqlx::query_as::<_, Profile>(
        "
        UPDATE profiles
        SET
            age = COALESCE($1, age),
            height = COALESCE($2, height),
            weight = COALESCE($3, weight),
            goal = COALESCE($4, goal),
            activity = COALESCE($5, activity),
            gender = COALESCE($6, gender)
        WHERE user_id = $7
        RETURNING *
    ",
    )
    .bind(payload.age)
    .bind(payload.height)
    .bind(payload.weight)
    .bind(payload.goal)
    .bind(payload.activity)
    .bind(payload.gender)
    .bind(user.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update profile, Err: {}", e);
        AppError::InternalServerError(Some("Failed to update profile.".to_string()))
    })?
    .ok_or(AppError::ItemNotFound(Some(
        "Profile not found.".to_string(),
    )))?;

    Ok(Json(ProfileResponse::from(profile)))
}

async fn create_profile_handler(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = sqlx::query_as::<_, Profile>(
        "
        INSERT INTO profiles (user_id, age, height, weight, goal, activity, gender, target_fats, target_fiber, target_carbs, target_protein, target_calories)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
    ",
    )
    .bind(user.sub)
    .bind(payload.age)
    .bind(payload.height)
    .bind(payload.weight)
    .bind(payload.goal)
    .bind(payload.activity)
    .bind(payload.gender)
    .bind(20.0)
    .bind(20.0)
    .bind(20.0)
    .bind(20.0)
    .bind(20.0)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create profile, Err: {}", e);
        AppError::InternalServerError(Some("Failed to create profile.".to_string()))
    })?
    .ok_or(AppError::ItemNotFound(Some(
        "Profile not found.".to_string(),
    )))?;

    Ok(Json(ProfileResponse::from(profile)))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/profiles", get(get_profile_handler))
        .route("/profiles", put(update_profile_handler))
        .route("/profiles", post(create_profile_handler))
        .route("/targets", get(get_targets_handler))
}
