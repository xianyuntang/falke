use common::dto::proxy::AcquireProxyRequestDto;
use common::infrastructure::error::ApiError;
use common::infrastructure::settings::Settings;
use entity::entities::proxy;
use random_word::Lang;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::{json, Value};

pub async fn handler(
    db: DatabaseConnection,
    settings: Settings,
    dto: AcquireProxyRequestDto,
    user_id: &str,
) -> Result<Value, ApiError> {
    let exist = proxy::Entity::find()
        .filter(proxy::Column::Id.eq(dto.subdomain.clone()))
        .one(&db)
        .await?;

    if let Some(exist) = exist {
        if exist.user_id == user_id {
            let proxy_endpoint = &format!("{}.{}", exist.id, settings.reverse_proxy_host);
            Ok(json!({
                "id": exist.id,
                "created_at":exist.created_at,
                "updated_at":exist.updated_at,
                "proxy_endpoint": proxy_endpoint
            }))
        } else {
            Err(ApiError::ForbiddenError)
        }
    } else {
        let id = dto.subdomain.unwrap_or_else(|| {
            format!(
                "{}-{}",
                random_word::gen(Lang::En).to_string(),
                random_word::gen(Lang::En).to_string()
            )
        });

        let new_proxy = proxy::ActiveModel {
            id: ActiveValue::Set(id),
            user_id: ActiveValue::Set(user_id.to_string()),
            ..Default::default()
        };
        let res = new_proxy.insert(&db).await?;
        Ok(json!({
            "id": res.id,
            "created_at":res.created_at,
            "updated_at":res.updated_at,
        }))
    }
}
