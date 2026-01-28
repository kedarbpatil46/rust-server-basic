use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::utils;



#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: String,
}

pub fn encode_jwt(email: String, id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret = (*utils::constants::JWT_SECRET).clone();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = (*utils::constants::JWT_SECRET).clone();
    let claim_data: Result<TokenData<_>, jsonwebtoken::errors::Error> = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
    claim_data
}