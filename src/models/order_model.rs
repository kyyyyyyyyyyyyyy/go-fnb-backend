use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_name: String,
    pub order_type: String,
    pub status: String,
    pub order_number: String,
    pub capital_price: i64,
    pub tax: i64,
    pub profit: i64,
    pub total: i64,
    pub notes: Option<String>,
    pub table_id: Uuid,
    pub outlet_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub change_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderItems {
    pub id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub product_id: Uuid,
    pub qty: i32,
    pub sub_total: i64,
    pub capital_price: i64,
    pub profit: i64,
    pub tax: i64,
    pub discount: i64,
    pub notes: Option<String>,
}