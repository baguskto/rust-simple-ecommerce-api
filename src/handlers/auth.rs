use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    middleware::TokenClaims,
    models::{CreateUserSchema, LoginUserSchema, User},
};

pub async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUserSchema>,
) -> impl Responder {
    let hashed_password = match hash(body.password.as_bytes(), DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Failed to hash password"
            }))
        }
    };

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, password, full_name, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        Uuid::new_v4(),
        body.email,
        hashed_password,
        body.full_name,
        "user",
        Utc::now(),
        Utc::now()
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "User created successfully",
                "data": {
                    "user": {
                        "id": user.id,
                        "email": user.email,
                        "full_name": user.full_name,
                        "role": user.role,
                        "created_at": user.created_at,
                        "updated_at": user.updated_at
                    }
                }
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                HttpResponse::Conflict().json(serde_json::json!({
                    "status": "error",
                    "message": "User with that email already exists"
                }))
            } else {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": "Something went wrong"
                }))
            }
        }
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginUserSchema>,
) -> impl Responder {
    let user_result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        body.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "error",
                "message": "Invalid email or password"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Something went wrong"
            }));
        }
    };

    let is_valid = match verify(body.password.as_bytes(), &user.password) {
        Ok(valid) => valid,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Something went wrong"
            }));
        }
    };

    if !is_valid {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": "Invalid email or password"
        }));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Failed to create token"
            }));
        }
    };

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "token": token
    }))
} 