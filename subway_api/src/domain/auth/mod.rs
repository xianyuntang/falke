use crate::infrastructure::error::ApiError;
use crate::infrastructure::response::JsonResponse;
use crate::infrastructure::server::AppState;
use crate::infrastructure::settings::Settings;
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use axum::routing::{post, Router};
use axum::Extension;
use common::dto::auth::{SignInRequestDto, SignUpRequestDto};
use sea_orm::DatabaseConnection;
use validator::Validate;

mod handlers;
mod jwt_validator;

pub fn create_route() -> Router<AppState> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/sign-up", post(sign_up))
            .route("/sign-in", post(sign_in)),
    )
}

async fn sign_up(
    State(state): State<AppState>,
    Json(dto): Json<SignUpRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    dto.validate()?;

    let response = handlers::sign_up::handler(dto, state.db).await?;

    Ok(JsonResponse(response).into_response())
}

async fn sign_in(
    State(state): State<AppState>,
    Json(dto): Json<SignInRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    dto.validate()?;

    let response = handlers::sign_in::handler(dto, state.db, state.settings).await?;

    Ok(JsonResponse(response).into_response())
}
