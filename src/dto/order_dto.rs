use serde::{Deserialize, Serialize};
use uuid::Uuid;


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
    pub qty: i64,
    pub notes: Option<String>, 
}