use crate::domain::auth::jwt_validator::verify;
use sea_orm::{DatabaseConnection, EntityTrait};

use shared::dto::auth::ValidateTokenRequestDto;
use shared::infrastructure::error::ApiError;
use shared::infrastructure::settings::Settings;
use entity::entities::user;
use serde_json::{json, Value};

pub async fn handler(
    db: DatabaseConnection,
    settings: Settings,
    dto: ValidateTokenRequestDto,
) -> Result<Value, ApiError> {
    if let Ok(user_id) = verify(&settings.api_secret, &dto.access_token) {
        let user = user::Entity::find_by_id(&user_id).one(&db).await?;
        Ok(json!({ "is_valid": user.is_some() }))
    } else {
        Ok(json!({ "is_valid": false }))
    }
}
