mod db;
mod modules;
use std::sync::Arc;

use axum::{Router, routing::get, serve};
use rustls::crypto::ring::default_provider;
use tower_http::trace::TraceLayer;

use crate::modules::{
    auth::{self, error::AuthError},
    users::{get_user_handler, get_users_handler},
};
mod config;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
    pub config: Arc<config::Config>,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    // load dotenv.
    dotenvy::dotenv().ok();

    // install default crypto provider.
    default_provider()
        .install_default()
        .expect("Failed to install default crypto provider;");

    // setup tracing.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

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

    // setup & register routes.
    let app = Router::new()
        .route("/auth/google", get(auth::google_handler))
        .route("/auth/callback", get(auth::callback_handler))
        .route("/auth/me", get(auth::get_self_handler))
        .route("/users", get(get_users_handler))
        .route("/users/{id}", get(get_user_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // start the server.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::info!("Listening on http://127.0.0.1:8080");
    serve(listener, app).await.unwrap();
}
