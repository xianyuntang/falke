use crate::infrastructure::response::JsonResponse;
use axum::http::status::InvalidStatusCode;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bcrypt::BcryptError;
use reqwest;
use sea_orm::DbErr;
use serde_json::json;
use std::io;
use url;
use validator::ValidationErrors;

pub enum ApiError {
    ValidationErrors(ValidationErrors),
    IoError(io::Error),
    AxumError(axum::Error),
    UnauthorizedError,
    NotFoundError,
    ConflictError,
    InternalServerError,
    InvalidStatusCode(InvalidStatusCode),
    JsonError(serde_json::Error),
    ParserError(url::ParseError),
    ReqError(reqwest::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ValidationErrors(validation_errors) => (
                StatusCode::BAD_REQUEST,
                JsonResponse(json!(validation_errors)),
            )
                .into_response(),
            ApiError::UnauthorizedError => StatusCode::UNAUTHORIZED.into_response(),
            ApiError::NotFoundError => StatusCode::NOT_FOUND.into_response(),
            ApiError::ConflictError => StatusCode::CONFLICT.into_response(),
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::IoError(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::AxumError(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::InvalidStatusCode(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::JsonError(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::ParserError(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            ApiError::ReqError(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
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

impl From<io::Error> for ApiError {
    fn from(error: io::Error) -> Self {
        tracing::error!("{}", error);
        Self::InternalServerError
    }
}

impl From<axum::Error> for ApiError {
    fn from(error: axum::Error) -> Self {
        tracing::error!("{}", error);
        Self::InternalServerError
    }
}

impl From<InvalidStatusCode> for ApiError {
    fn from(invalid_status_code: InvalidStatusCode) -> Self {
        tracing::error!("{}", invalid_status_code);
        Self::InternalServerError
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        tracing::error!("{}", error);
        Self::InternalServerError
    }
}

impl From<url::ParseError> for ApiError {
    fn from(error: url::ParseError) -> Self {
        tracing::error!("{}", error);
        Self::InternalServerError
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("{}", error);
        Self::InternalServerError
    }
}
