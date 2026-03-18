// TODO: remove this in production.
#![allow(dead_code, unused_variables, unused_assignments)]

mod config;
mod db;
mod error;
mod modules;
mod storage;
use std::sync::Arc;

use axum::{Router, serve};
use rustls::crypto::ring::default_provider;
use tower_http::trace::TraceLayer;

use crate::modules::{
    auth::{error::AuthError, router as auth_router},
    meals::router as meals_router,
    profiles::router as profiles_router,
    users::router as users_router,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
    pub config: Arc<config::Config>,
    pub http: reqwest::Client,
    pub r2: aws_sdk_s3::Client,
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
    let r2_client = storage::create_r2_client(&config).await;

    let state = AppState {
        pool,
        config,
        http: http_client,
        r2: r2_client,
    };

    // setup & register routes.
    let app = Router::new()
        .merge(auth_router())
        .merge(users_router())
        .merge(meals_router())
        .merge(profiles_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // start the server.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::info!("Listening on http://127.0.0.1:8080");
    serve(listener, app).await.unwrap();
}
