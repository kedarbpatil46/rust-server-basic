use std::future;

use actix_web::{FromRequest, HttpMessage};
use actix_web_lab::FromRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::utils;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: Uuid,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claims")))
        }
    }
}

pub fn encode_jwt(email: String, id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
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