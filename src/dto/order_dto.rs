use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::order_model::OrderItems as OrderItemsModel;


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOrder {
    pub order_name: String,
    pub order_type: String,
    pub table_id: Uuid,
    pub outlet_id: Uuid,
    pub notes: Option<String>,
    pub order_items: Vec<OrderItems>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateOrder {
    pub order_name: Option<String>,
    pub order_type: Option<String>,
    pub table_id: Option<Uuid>,
    pub notes: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrderItems {
    pub product_id: Uuid,
    pub qty: i32,
    pub notes: Option<String>, 
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateOrderStatus {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct OrderResponseDTO {
    pub id: Uuid,
    pub order_name: String,
    pub order_type: String,
    pub order_status: String,
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
    pub items: Vec<OrderItemsModel>,
}