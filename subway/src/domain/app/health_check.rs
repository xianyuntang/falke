use axum::response::Json;
use serde_json::json;

pub fn handler() -> Json<serde_json::Value> {
    Json(json!({"message": "ok"}))
}
