use crate::domain::auth::jwt_validator::sign_jwt;
use bcrypt::verify;
use common::dto::auth::SignInRequestDto;
use common::infrastructure::error::ApiError;
use common::infrastructure::settings::Settings;
use entity::entities::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::{json, Value};

pub async fn handler(
    dto: SignInRequestDto,
    db: DatabaseConnection,
    settings: Settings,
) -> Result<Value, ApiError> {
    let exist = match user::Entity::find()
        .filter(user::Column::Email.eq(&dto.email))
        .one(&db)
        .await?
    {
        Some(model) => model,
        None => return Err(ApiError::NotFoundError),
    };

    let is_password_match = verify(dto.password, exist.password.as_str())?;

    if !is_password_match {
        return Err(ApiError::UnauthorizedError);
    }

    let access_token = sign_jwt(&settings.server_secret, exist.id.as_str());

    Ok(json!({"access_token": access_token}))
}
