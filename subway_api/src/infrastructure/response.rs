use axum::response::{IntoResponse, Response};
use serde_json::Value;

pub struct JsonResponse(pub Value);

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
