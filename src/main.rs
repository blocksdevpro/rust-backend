mod db;
mod modules;
use std::sync::Arc;

use axum::{Router, routing::get, serve};
use rustls::crypto::ring::default_provider;

use crate::modules::auth::{self, error::AuthError};
mod config;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
    pub config: Arc<config::Config>,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    default_provider()
        .install_default()
        .expect("Failed to install default crypto provider;");

    let config = Arc::new(config::Config::from_env());
    let pool = db::connect(&config.database_url).await;
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| AuthError::FailedHttpClient)
        .unwrap();

    let state = AppState {
        pool,
        config,
        http_client,
    };

    let app = Router::new()
        .route("/auth/google", get(auth::google_handler))
        .route("/auth/callback", get(auth::callback_handler))
        .route("/auth/dashboard", get(auth::dashboard_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:8080");
    serve(listener, app).await.unwrap();
}
