use crate::infrastructure::error::ApiError;
use crate::infrastructure::response::JsonResponse;
use axum::extract::{Json, OriginalUri, Path};

use crate::infrastructure::server::AppState;
use axum::body::Body;
use axum::http::Request;
use axum::response::IntoResponse;
use axum::routing::{any, get, post};
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;
use serde_json::json;

pub mod handlers;

pub fn create_route() -> Router<AppState> {
    Router::new().nest(
        "/tunnel",
        Router::new()
            .route("/start", post(start_tunnel))
            .route("/proxy", any(proxy)),
    )
}

async fn start_tunnel(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    let response = handlers::start_tunnel::handler().await?;

    Ok(JsonResponse(response))
}

async fn proxy(
    OriginalUri(uri): OriginalUri,
    mut req: Request<Body>,
) -> Result<impl IntoResponse, ApiError> {
    println!("{:#?}", req);
    Ok(JsonResponse(json!({"message":"ok"})))
}
