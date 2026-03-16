use serde::Serialize;
use sqlx::prelude::FromRow;
use strum_macros::Display;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Display)]
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
    pub fats: f64,
    pub carbs: f64,
    pub fiber: f64,
    pub protein: f64,
    pub calories: f64,
    pub confidence: f64,
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

            fats: meal.fats as f32,
            carbs: meal.carbs as f32,
            fiber: meal.fiber as f32,
            protein: meal.protein as f32,
            calories: meal.calories as f32,

            created_at: meal.created_at.to_string(),
        }
    }
}
