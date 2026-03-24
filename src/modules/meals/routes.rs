// Change `axum` to `actix_web` if you are using Actix
use axum::Router;
use axum::routing::{get, post};

use crate::state::AppState;

use super::handlers;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/meals", get(handlers::get_meals_handler))
        .route("/meals/{id}", get(handlers::get_meal_handler))
        .route("/meals/scan", post(handlers::scan_meal_handler))
}
