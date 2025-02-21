use actix_web::{
    error::ErrorUnauthorized,
    http::header::{self},
    Error, HttpRequest,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn auth_middleware(req: HttpRequest) -> Result<Uuid, Error> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| ErrorUnauthorized("No valid auth token found"))?;

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ErrorUnauthorized("Invalid token"))?
    .claims;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ErrorUnauthorized("Invalid token subject"))?;

    Ok(user_id)
} 