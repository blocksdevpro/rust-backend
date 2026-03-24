use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Display, Deserialize, Serialize)]
#[sqlx(type_name = "meal_type_enum", rename_all = "lowercase")]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

#[derive(Debug, FromRow)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub picture: Option<String>,
    pub meal_type: MealType,
    pub fats: f32,
    pub carbs: f32,
    pub fiber: f32,
    pub protein: f32,
    pub calories: f32,
    pub confidence: f32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize)]
pub struct MealResponse {
    pub id: Uuid,
    pub name: String,
    pub picture: Option<String>,
    pub meal_type: String,

    pub fats: f32,
    pub carbs: f32,
    pub fiber: f32,
    pub protein: f32,
    pub calories: f32,

    pub created_at: String,
}

impl From<Meal> for MealResponse {
    fn from(meal: Meal) -> Self {
        Self {
            id: meal.id,
            name: meal.name,
            picture: meal.picture,
            meal_type: meal.meal_type.to_string(),

            fats: meal.fats,
            carbs: meal.carbs,
            fiber: meal.fiber,
            protein: meal.protein,
            calories: meal.calories,

            created_at: meal.created_at.to_string(),
        }
    }
}

#[derive(TryFromMultipart)]
pub struct ScanMealRequest {
    pub label: Option<String>,
    #[form_data(limit = "2MB")]
    pub image: FieldData<Bytes>,
}

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
