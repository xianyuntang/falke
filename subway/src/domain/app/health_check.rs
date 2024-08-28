use axum::response::Json;
use serde_json::{json, Value};

pub fn handler() -> Json<Value> {
    Json(json!({"message": "ok"}))
}
