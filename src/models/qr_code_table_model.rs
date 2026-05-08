use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QrCodeTable {
    pub id: Uuid,
    pub qr_id: Uuid,
    pub table_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}