use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateOutletDTO {
    pub name: String,
    pub address_line: String,
    pub city: String,
    pub province: String,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}



#[derive(Deserialize)]
pub struct UpdateOutletDTO {
    pub name: Option<String>,
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct OutletWithTodayStats {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub address_line: String,
    pub city: String,
    pub province: String,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub today_revenue: i64,
    pub today_orders: i64,
}

