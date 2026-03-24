use crate::{
    error::AppError,
    modules::profiles::models::{CreateProfileRequest, Profile, UpdateProfileRequest},
};

pub struct ProfileRepository {
    pool: sqlx::PgPool,
}

impl ProfileRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Profile>, AppError> {
        let profile = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(profile)
    }

    pub async fn find_by_user_id(&self, user_id: uuid::Uuid) -> Result<Option<Profile>, AppError> {
        let profile = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(profile)
    }

    pub async fn create(
        &self,
        user_id: uuid::Uuid,
        profile: CreateProfileRequest,
    ) -> Result<Profile, AppError> {
        let profile = sqlx::query_as::<_, Profile>(
            "INSERT INTO profiles (user_id, age, height, weight, goal, gender, activity, target_fats, target_fiber, target_carbs, target_protein, target_calories) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *"
        )
        .bind(user_id)
        .bind(profile.age)
        .bind(profile.height)
        .bind(profile.weight)
        .bind(profile.goal)
        .bind(profile.gender)
        .bind(profile.activity)
        // TODO: calculate these values based on the profile
        .bind(20.0) // target_fats
        .bind(20.0) // target_fiber
        .bind(20.0) // target_carbs
        .bind(20.0) // target_protein
        .bind(20.0) // target_calories
        .fetch_one(&self.pool)
        .await?;
        Ok(profile)
    }

    pub async fn update(
        &self,
        user_id: uuid::Uuid,
        profile: UpdateProfileRequest,
    ) -> Result<Profile, AppError> {
        let profile = sqlx::query_as::<_, Profile>(
            "UPDATE profiles SET age = COALESCE($1, age), height = COALESCE($2, height), weight = COALESCE($3, weight), goal = COALESCE($4, goal), gender = COALESCE($5, gender), activity = COALESCE($6, activity), target_fats = COALESCE($7, target_fats), target_fiber = COALESCE($8, target_fiber), target_carbs = COALESCE($9, target_carbs), target_protein = COALESCE($10, target_protein), target_calories = COALESCE($11, target_calories) WHERE user_id = $12 RETURNING *"
        )
        .bind(profile.age)
        .bind(profile.height)
        .bind(profile.weight)
        .bind(profile.goal)
        .bind(profile.gender)
        .bind(profile.activity)
        // TODO: calculate these values based on the profile
        .bind(20.0) // target_fats
        .bind(20.0) // target_fiber
        .bind(20.0) // target_carbs
        .bind(20.0) // target_protein
        .bind(20.0) // target_calories
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(profile)
    }
}
