use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AvailableTableDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ScanQrResponseDTO {
    pub qr_id: Uuid,
    pub qr_type: String,

    pub token: Option<String>,

    pub table: Option<AvailableTableDTO>,

    pub tables: Option<Vec<AvailableTableDTO>>,
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