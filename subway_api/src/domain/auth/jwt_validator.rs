use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

pub fn sign_jwt(secret: &str, id: &str) -> String {
    let iat = Utc::now();
    let exp = iat + Duration::minutes(1);
    let claims = Claims {
        sub: id.to_string(),
        iat: iat.timestamp() as u64,
        exp: exp.timestamp() as u64,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
