use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError};
use utoipa::ToSchema;

fn validate_price(price: &Decimal) -> Result<(), ValidationError> {
    if price <= &Decimal::from(0) {
        return Err(ValidationError::new("Price must be positive"));
    }
    Ok(())
}

/// Product model representing a product in the system
#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Product {
    /// Unique identifier for the product
    pub id: Uuid,
    /// Name of the product
    pub name: String,
    /// Detailed description of the product
    pub description: String,
    /// Price of the product
    pub price: Decimal,
    /// Available stock quantity
    pub stock: i32,
    /// When the product was created
    pub created_at: DateTime<Utc>,
    /// When the product was last updated
    pub updated_at: DateTime<Utc>,
}

/// Schema for creating a new product
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductSchema {
    /// Name of the product (required)
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    /// Detailed description of the product (required)
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
    /// Price of the product (must be positive)
    #[validate(custom = "validate_price")]
    pub price: Decimal,
    /// Initial stock quantity (must be non-negative)
    #[validate(range(min = 0, message = "Stock must be positive"))]
    pub stock: i32,
}

/// Schema for updating an existing product
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProductSchema {
    /// Optional new name for the product
    pub name: Option<String>,
    /// Optional new description for the product
    pub description: Option<String>,
    /// Optional new price for the product
    pub price: Option<Decimal>,
    /// Optional new stock quantity
    pub stock: Option<i32>,
}

impl UpdateProductSchema {
    pub fn validate_price(&self) -> Result<(), ValidationError> {
        if let Some(price) = &self.price {
            validate_price(price)
        } else {
            Ok(())
        }
    }
} 