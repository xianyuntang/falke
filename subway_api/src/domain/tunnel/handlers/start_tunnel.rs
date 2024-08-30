use crate::infrastructure::error::ApiError;
use serde_json::{json, Value};

pub async fn handler() -> Result<Value, ApiError> {
    Ok(json!({"message":"ok"}))
}
