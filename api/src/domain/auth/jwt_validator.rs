use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

pub fn sign_jwt(secret: &str, id: &str) -> String {
    let iat = Utc::now();
    let exp = iat + Duration::days(7);
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

pub fn verify(secret: &str, token: &str) -> String {
    let token: TokenData<Claims> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Default::default(),
    )
    .unwrap();
    token.claims.sub
}
