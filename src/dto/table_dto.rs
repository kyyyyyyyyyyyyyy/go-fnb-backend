use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTableDTO {
    pub name: String,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTableDTO {
    pub name: Option<String>,
    pub location: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TableResponseDTO {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub name: String,
    pub location: Option<String>,
    pub status: String,
}

pub struct TableDeleteTokenDTO {
    pub table_id: Uuid,
}