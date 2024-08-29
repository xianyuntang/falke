use crate::infrastructure::response::JsonResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Error;
use bcrypt::BcryptError;
use sea_orm::DbErr;
use serde_json::json;
use validator::ValidationErrors;

pub enum ApiError {
    ValidationErrors(ValidationErrors),
    InternalServerError,
    Conflict,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ValidationErrors(validation_errors) => (
                StatusCode::BAD_REQUEST,
                JsonResponse(json!(validation_errors)),
            )
                .into_response(),
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::Conflict => StatusCode::CONFLICT.into_response(),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(validation_errors: ValidationErrors) -> Self {
        Self::ValidationErrors(validation_errors)
    }
}

impl From<DbErr> for ApiError {
    fn from(db_err: DbErr) -> Self {
        tracing::error!("{}", db_err);
        Self::InternalServerError
    }
}

impl From<BcryptError> for ApiError {
    fn from(bcrypt_error: BcryptError) -> Self {
        tracing::error!("{}", bcrypt_error);
        Self::InternalServerError
    }
}