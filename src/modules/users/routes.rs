use axum::Router;
use axum::routing::get;

use crate::modules::users::handlers;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/users", get(handlers::get_users_handler))
        .route("/users/{id}", get(handlers::get_user_handler))
}
