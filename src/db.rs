use sqlx::{
    Pool, Postgres,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;

pub async fn connect(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(
            PgConnectOptions::from_str(database_url)
                .expect("Invalid database URL")
                .statement_cache_capacity(0),
        )
        .await
        .expect("Failed to connect to database")
}
