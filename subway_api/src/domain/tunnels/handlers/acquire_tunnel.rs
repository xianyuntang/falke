use crate::infrastructure::error::ApiError;

use entity::entities;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde_json::{json, Value};

pub async fn handler(db: DatabaseConnection) -> Result<Value, ApiError> {
    let tunnel = entities::tunnel::ActiveModel {
        user_id: ActiveValue::Set("28rDqPP69QTPOJdgI-yG6".to_string()),
        ..Default::default()
    };

    let res = tunnel.insert(&db).await?;

    Ok(json!({"id": res.id, "created_at":res.created_at, "updated_at":res.updated_at}))
}
