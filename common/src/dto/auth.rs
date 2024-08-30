use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct SignUpRequestDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignInRequestDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
