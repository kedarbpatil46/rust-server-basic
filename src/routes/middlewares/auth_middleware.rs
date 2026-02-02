use actix_web::{
    Error, HttpMessage, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, error::{ErrorUnauthorized}, http::header::AUTHORIZATION, middleware::Next
};

use crate::utils::jwt::decode_jwt;

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {

    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("Missing Authorization header"))?;

    // Strip "Bearer "
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ErrorUnauthorized("Invalid Authorization format"))?;

    // Decode JWT
    let claim = decode_jwt(token.to_string())
        .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

    // Attach claims to request extensions
    req.extensions_mut().insert(claim.claims);

    // Continue
    next.call(req).await
}
