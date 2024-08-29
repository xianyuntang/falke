use crate::domain::auth::dto::SignUpRequestDto;
use ::entity::entities::user;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use serde_json::{json, Value};

pub async fn handler(dto: SignUpRequestDto, db: &DatabaseConnection) -> Result<Value, DbErr> {
    let user = user::ActiveModel {
        id: ActiveValue::NotSet,
        email: ActiveValue::Set(dto.email),
        password: ActiveValue::Set(dto.password),
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
    };
    user.insert(db).await?;
    Ok(json!({"message":"ok"}))
}
