use anyhow::Result;
use common::infrastructure::error::ApiError;
use entity::entities::proxy;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

pub async fn handler(
    db: DatabaseConnection,
    proxy_id: &str,
    user_id: &str,
) -> Result<(), ApiError> {
    let exist = proxy::Entity::find_by_id(proxy_id)
        .one(&db)
        .await?
        .ok_or(ApiError::NotFoundError)?;

    if exist.user_id != user_id {
        return Err(ApiError::ForbiddenError);
    }

    exist.delete(&db).await?;

    Ok(())
}
