use crate::infrastructure::server::AppState;
use axum::response::{IntoResponse, Json};
use axum::routing::{get, Router};
mod handlers;

pub fn create_route() -> Router<AppState> {
    Router::new().route("/ping", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    let response = handlers::health_check::handler().await;
    Json(response)
}
