use sqlx::{Pool, Postgres};

pub async fn connect(database_url: &str) -> Pool<Postgres> {
    Pool::connect(database_url)
        .await
        .expect("Failed to connect to database")
}
