use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ScanQrDTO {
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct ScanQrResponseDTO {
    pub qr_id: Uuid,
    pub tables: Vec<TableDTO>,
}

#[derive(Debug, Serialize)]
pub struct TableDTO {
    pub id: Uuid,
    pub name: String,
}