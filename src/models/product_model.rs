use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub capital_price: i64,
    pub tax: i64,
    pub profit: i64,
    pub price: i64,
    pub image_url: String,
    pub outlet_id: Uuid,
    pub created_at: NaiveDateTime,
}