use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::product_dto::CategoryItemDTO;

#[derive(Debug, Serialize)]
pub struct AvailableTableDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ProductScanDTO {
    pub id: Uuid,
    pub name: String,
    pub price: i64,
    pub image_url: String,
    pub categories: Vec<CategoryItemDTO>,
}

#[derive(Debug, Serialize)]
pub struct ScanQrResponseDTO {
    pub qr_id: Uuid,
    pub qr_type: String,

    pub outlet_id: Uuid,

    pub table: Option<AvailableTableDTO>,

    pub tables: Option<Vec<AvailableTableDTO>>,

    pub categories: Vec<CategoryItemDTO>,
    pub products: Vec<ProductScanDTO>,
}

#[derive(Debug, Serialize)]
pub struct TableDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SelectTableDTO {
    pub table_id: Uuid,
}
