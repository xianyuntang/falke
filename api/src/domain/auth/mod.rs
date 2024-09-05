use crate::infrastructure::server::AppState;
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use axum::routing::{post, Router};
use common::dto::auth::{SignInRequestDto, SignUpRequestDto, ValidateTokenRequestDto};
use common::infrastructure::error::ApiError;
use common::infrastructure::response::JsonResponse;
use validator::Validate;

mod handlers;
pub mod jwt_validator;

pub fn create_route() -> Router<AppState> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/sign-up", post(sign_up))
            .route("/sign-in", post(sign_in))
            .route("/validate-token", post(validate_token)),
    )
}

async fn sign_up(
    State(state): State<AppState>,
    Json(dto): Json<SignUpRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    dto.validate()?;

    let response = handlers::sign_up::handler(dto, state.db).await?;

    Ok(JsonResponse(response))
}

async fn sign_in(
    State(state): State<AppState>,
    Json(dto): Json<SignInRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    dto.validate()?;

    let response = handlers::sign_in::handler(dto, state.db, state.settings).await?;

    Ok(JsonResponse(response))
}

async fn validate_token(
    State(AppState { settings, db }): State<AppState>,
    Json(dto): Json<ValidateTokenRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    dto.validate()?;

    let response = handlers::validate_token::handler(db, settings, dto).await?;

    Ok(JsonResponse(response))
}
