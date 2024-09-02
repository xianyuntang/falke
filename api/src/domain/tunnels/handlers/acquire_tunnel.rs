use common::infrastructure::error::ApiError;
use common::infrastructure::settings::Settings;
use entity::entities;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde_json::{json, Value};
use url::Url;

pub async fn handler(
    db: DatabaseConnection,
    settings: Settings,
    user_id: &str,
) -> Result<Value, ApiError> {
    let tunnel = entities::tunnel::ActiveModel {
        user_id: ActiveValue::Set(user_id.to_string()),
        ..Default::default()
    };

    let res = tunnel.insert(&db).await?;

    let proxy_endpoint = Url::parse(&format!(
        "https://{}.{}",
        res.id, settings.reverse_proxy_host,
    ))?
    .to_string();

    Ok(json!({
        "id": res.id,
        "created_at":res.created_at,
        "updated_at":res.updated_at,
        "proxy_endpoint": proxy_endpoint
    }))
}
