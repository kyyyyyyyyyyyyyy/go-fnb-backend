use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateQrDTO {
    pub outlet_id: Uuid,
    pub table_ids: Vec<Uuid>, // 🔥 support multi table
}

#[derive(Debug, Serialize)]
pub struct QrResponseDTO {
    pub id: Uuid,
    pub slug: String,
    pub table_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQrWithTablesDTO {
    pub outlet_id: Uuid,

    // 🔥 list nama meja (fleksibel)
    pub tables: Vec<CreateTableItemDTO>,}

#[derive(Debug, Deserialize)]
pub struct CreateTableItemDTO {
    pub name: String,
    pub location: String,
}