use super::handlers;
use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/auth/login", get(handlers::login_handler))
        .route("/auth/logout", get(handlers::logout_handler))
        .route("/auth/callback", get(handlers::callback_handler))
        .route("/auth/me", get(handlers::me_handler))
}
