use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignUpRequestDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignUpResponseDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct SignInRequestDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInResponseDto {
    pub access_token: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ValidateTokenRequestDto {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponseDto {
    pub is_valid: bool,
}
