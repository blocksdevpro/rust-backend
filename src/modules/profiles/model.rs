use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum_macros::Display;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Display, Deserialize)]
#[sqlx(type_name = "profile_activity_enum", rename_all = "lowercase")]
pub enum Activity {
    Sedentary,
    Light,
    Moderate,
    Active,
    Vigorous,
}
#[derive(Debug, sqlx::Type, Display, Deserialize)]
#[sqlx(type_name = "profile_gender_enum", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, sqlx::Type, Display, Deserialize)]
#[sqlx(type_name = "profile_goal_enum", rename_all = "lowercase")]
pub enum Goal {
    Lose,
    Gain,
    Maintain,
}

#[derive(Debug, FromRow)]
pub struct Profile {
    // ids
    id: Uuid,
    user_id: Uuid,

    // numbers
    age: i32,
    height: i32,
    weight: i32,

    // enums
    goal: Goal,
    gender: Gender,
    activity: Activity,

    // targets based on the profile.
    target_fats: f32,
    target_fiber: f32,
    target_carbs: f32,
    target_protein: f32,
    target_calories: f32,

    // timestamps
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    id: Uuid,

    age: i32,
    height: i32,
    weight: i32,

    goal: String,
    gender: String,
    activity: String,

    target_fats: f32,
    target_fiber: f32,
    target_carbs: f32,
    target_protein: f32,
    target_calories: f32,

    created_at: String,
}
#[derive(Serialize)]
pub struct TargetsResponse {
    pub target_fats: f32,
    pub target_fiber: f32,
    pub target_carbs: f32,
    pub target_protein: f32,
    pub target_calories: f32,
}

impl From<Profile> for ProfileResponse {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id,
            age: profile.age,
            height: profile.height,
            weight: profile.weight,
            goal: profile.goal.to_string(),
            gender: profile.gender.to_string(),
            activity: profile.activity.to_string(),

            target_fats: profile.target_fats,
            target_fiber: profile.target_fiber,
            target_carbs: profile.target_carbs,
            target_protein: profile.target_protein,
            target_calories: profile.target_calories,

            created_at: profile.created_at.format(&Rfc3339).unwrap(),
        }
    }
}

impl From<Profile> for TargetsResponse {
    fn from(profile: Profile) -> Self {
        Self {
            target_fats: profile.target_fats,
            target_fiber: profile.target_fiber,
            target_carbs: profile.target_carbs,
            target_protein: profile.target_protein,
            target_calories: profile.target_calories,
        }
    }
}
