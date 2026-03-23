use axum::Router;
use tower_http::trace::TraceLayer;

use crate::{modules::auth::routes::routes as auth_routes, state::AppState};

pub fn create_server(state: AppState) -> Router {
    Router::new()
        .merge(auth_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

pub async fn run_server(app: Router) {
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    tracing::info!("Listening on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
