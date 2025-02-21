use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;
use validator::Validate;
use utoipa::OpenApi;

use crate::models::{CreateProductSchema, Product, UpdateProductSchema};

#[derive(OpenApi)]
#[openapi(
    paths(
        create_product,
        get_products,
        get_product,
        update_product,
        delete_product
    ),
    components(
        schemas(Product, CreateProductSchema, UpdateProductSchema)
    ),
    tags(
        (name = "products", description = "Product management endpoints")
    )
)]
pub struct ApiDoc;

/// Create a new product
#[utoipa::path(
    post,
    path = "/api/products",
    request_body = CreateProductSchema,
    responses(
        (status = 200, description = "Product created successfully", body = Product),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn create_product(
    pool: web::Data<PgPool>,
    body: web::Json<CreateProductSchema>,
) -> impl Responder {
    if let Err(err) = body.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("Validation error: {}", err)
        }));
    }

    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (id, name, description, price, stock, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        Uuid::new_v4(),
        body.name,
        body.description,
        body.price,
        body.stock,
        Utc::now(),
        Utc::now()
    )
    .fetch_one(pool.get_ref())
    .await;

    match product {
        Ok(product) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": product
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Something went wrong"
        }))
    }
}

/// Get all products
#[utoipa::path(
    get,
    path = "/api/products",
    responses(
        (status = 200, description = "List of products retrieved successfully", body = Vec<Product>),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn get_products(pool: web::Data<PgPool>) -> impl Responder {
    let products = sqlx::query_as!(Product, "SELECT * FROM products")
        .fetch_all(pool.get_ref())
        .await;

    match products {
        Ok(products) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": products
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Something went wrong"
        }))
    }
}

/// Get a specific product by ID
#[utoipa::path(
    get,
    path = "/api/products/{id}",
    params(
        ("id" = Uuid, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product found", body = Product),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn get_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
) -> impl Responder {
    let product = sqlx::query_as!(
        Product,
        "SELECT * FROM products WHERE id = $1",
        product_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await;

    match product {
        Ok(Some(product)) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": product
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "message": "Product not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Something went wrong"
        }))
    }
}

/// Update a product
#[utoipa::path(
    patch,
    path = "/api/products/{id}",
    params(
        ("id" = Uuid, Path, description = "Product ID")
    ),
    request_body = UpdateProductSchema,
    responses(
        (status = 200, description = "Product updated successfully", body = Product),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn update_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
    body: web::Json<UpdateProductSchema>,
) -> impl Responder {
    if let Err(err) = body.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("Validation error: {}", err)
        }));
    }

    if let Err(err) = body.validate_price() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": format!("Validation error: {}", err)
        }));
    }

    let product = sqlx::query_as!(
        Product,
        r#"
        UPDATE products
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            price = COALESCE($3, price),
            stock = COALESCE($4, stock),
            updated_at = $5
        WHERE id = $6
        RETURNING *
        "#,
        body.name,
        body.description,
        body.price,
        body.stock,
        Utc::now(),
        product_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await;

    match product {
        Ok(Some(product)) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": product
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "message": "Product not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Something went wrong"
        }))
    }
}

/// Delete a product
#[utoipa::path(
    delete,
    path = "/api/products/{id}",
    params(
        ("id" = Uuid, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product deleted successfully"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn delete_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
) -> impl Responder {
    let result: Result<PgQueryResult, sqlx::Error> = sqlx::query!(
        "DELETE FROM products WHERE id = $1",
        product_id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(ref deleted) if deleted.rows_affected() == 0 => {
            HttpResponse::NotFound().json(serde_json::json!({
                "status": "error",
                "message": "Product not found"
            }))
        }
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Product deleted successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Something went wrong"
        }))
    }
} 