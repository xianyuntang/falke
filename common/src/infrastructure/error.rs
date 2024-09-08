use crate::infrastructure::response::JsonResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum ApiError {
    ValidationErrors(ValidationErrors),
    UnauthorizedError,
    ForbiddenError,
    NotFoundError,
    ConflictError,
    InternalServerError(anyhow::Error),
    ProxyClientNotConnectError,
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
            ApiError::ForbiddenError => StatusCode::FORBIDDEN.into_response(),
            ApiError::NotFoundError => StatusCode::NOT_FOUND.into_response(),
            ApiError::ProxyClientNotConnectError => StatusCode::NOT_FOUND.into_response(),
            ApiError::ConflictError => StatusCode::CONFLICT.into_response(),
            ApiError::InternalServerError(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let err = err.into();
        tracing::error!("{:#?}", err);
        Self::InternalServerError(err)
    }
}
