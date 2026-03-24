use uuid::Uuid;

use crate::modules::meals::models::Meal;

pub struct MealRepository {
    pool: sqlx::PgPool,
}

impl MealRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Meal>, sqlx::Error> {
        let meal = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(meal)
    }

    pub async fn find_by_user_id(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Meal>, sqlx::Error> {
        let meal = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(meal)
    }

    pub async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<Meal>, sqlx::Error> {
        let meals = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(meals)
    }
}
