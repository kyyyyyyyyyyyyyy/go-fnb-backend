use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Outlet {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub address_line: String,
    pub city: String,
    pub province: String,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
