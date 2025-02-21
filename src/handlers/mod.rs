pub mod auth;
pub mod product;

pub use auth::{login, register};
pub use product::{create_product, delete_product, get_product, get_products, update_product}; 