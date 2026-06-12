use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductCategory {
    pub category_id: Uuid,
    pub product_id: Uuid
}