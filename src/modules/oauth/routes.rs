use axum::{Router, routing::get};

use super::handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/auth/login", get(handlers::login_handler))
        .route("/auth/logout", get(handlers::logout_handler))
        .route("/auth/callback", get(handlers::callback_handler))
        .route("/auth/me", get(handlers::me_handler))
}
