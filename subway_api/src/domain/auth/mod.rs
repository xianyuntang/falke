use crate::domain::auth::dto::SignUpRequestDto;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::response::JsonResponse;
use axum::extract::Json;
use axum::response::IntoResponse;
use axum::routing::{post, Router};
use axum::Extension;
use sea_orm::DatabaseConnection;
use validator::Validate;

mod dto;
mod handlers;

pub fn create_route() -> Router {
    Router::new().nest("/auth", Router::new().route("/sign-up", post(sign_up)))
}

async fn sign_up(
    Extension(db): Extension<DatabaseConnection>,
    Json(dto): Json<SignUpRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    dto.validate()?;
    let response = handlers::sign_up::handler(dto, &db).await?;

    Ok(JsonResponse(response).into_response())
}
