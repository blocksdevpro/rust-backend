// Change `axum` to `actix_web` if you are using Actix
use axum::Router;

pub fn routes() -> Router {
    Router::new()
    // .route("/", get(handlers::example_handler))
}
