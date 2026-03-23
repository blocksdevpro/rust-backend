use uuid::Uuid;

use super::models::User;

pub struct UserRepository {
    pool: sqlx::PgPool,
}

impl UserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE google_id = $1")
            .bind(google_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create(
        &self,
        google_id: &str,
        name: &str,
        email: &str,
        picture: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (google_id, name, email, picture) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(google_id)
        .bind(name)
        .bind(email)
        .bind(picture)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn upsert(
        &self,
        google_id: &str,
        name: &str,
        email: &str,
        picture: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (google_id, name, email, picture) VALUES ($1, $2, $3, $4) ON CONFLICT (google_id) DO UPDATE SET name = $2, email = $3, picture = $4 RETURNING *"
        )
        .bind(google_id)
        .bind(name)
        .bind(email)
        .bind(picture)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
