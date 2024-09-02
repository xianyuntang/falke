use serde_json::{json, Value};

pub async fn handler() -> Value {
    json!({"message": "ok"})
}
