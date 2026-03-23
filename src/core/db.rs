use std::time::Duration;

use sqlx::{Error, Pool, Postgres, postgres::PgPoolOptions};

pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2) // Keep warm connections
        .acquire_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(30 * 60)) // Recycle connections
        .idle_timeout(Duration::from_secs(10 * 60))
        .connect(database_url)
        .await
        .inspect_err(|e| tracing::error!("Database connection failed: {}", e))
}
