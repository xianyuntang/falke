use crate::infrastructure::error::ApiError;
use crate::infrastructure::response::JsonResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;
use serde_json::json;

pub mod handlers;

pub fn create_route() -> Router {
    Router::new().nest("/tunnel", Router::new().route("/start", post(start_tunnel)))
}

async fn start_tunnel(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    let response = handlers::start_tunnel::handler().await?;

    Ok(JsonResponse(response))
}
