use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Table {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub name: String,
    pub location: String,
    pub status: String,
}