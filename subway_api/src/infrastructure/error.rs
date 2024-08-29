use crate::infrastructure::response::JsonResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use serde_json::json;
use validator::ValidationErrors;

pub enum ApiError {
    ValidationErrors(ValidationErrors),
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ValidationErrors(validation_errors) => (
                StatusCode::BAD_REQUEST,
                JsonResponse(json!(validation_errors)),
            )
                .into_response(),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(validation_errors: ValidationErrors) -> Self {
        Self::ValidationErrors(validation_errors)
    }
}

impl From<DbErr> for ApiError {
    fn from(_: DbErr) -> Self {
        Self::InternalServerError
    }
}
