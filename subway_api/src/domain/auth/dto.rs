use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

static STRONG_PASSWORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap()
});

#[derive(Debug, Validate, Deserialize)]
pub struct SignUpRequestDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}
