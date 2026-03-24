use axum::Router;
use axum::routing::{get, post, put};

use crate::state::AppState;

use super::handlers;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/profiles", get(handlers::get_profile_handler))
        .route("/profiles", put(handlers::update_profile_handler))
        .route("/profiles", post(handlers::create_profile_handler))
        .route("/profiles/targets", get(handlers::get_targets_handler))
}
