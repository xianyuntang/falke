use crate::domain::auth::jwt_validator::verify;

use common::dto::auth::ValidateTokenRequestDto;
use common::infrastructure::error::ApiError;
use common::infrastructure::settings::Settings;
use serde_json::{json, Value};

pub async fn handler(settings: Settings, dto: ValidateTokenRequestDto) -> Result<Value, ApiError> {
    let valid = verify(&settings.api_secret, &dto.access_token).is_ok();

    Ok(json!({"is_valid":valid}))
}
