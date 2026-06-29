use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateQrDTO {
    pub table_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct QrResponseDTO {
    pub id: Uuid,
    pub slug: String,
    pub table_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQrWithTablesDTO {
    pub tables: Vec<CreateTableItemDTO>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTableItemDTO {
    pub name: String,
    pub location: String,
}