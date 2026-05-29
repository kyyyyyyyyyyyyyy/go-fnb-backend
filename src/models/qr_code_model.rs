use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QrCode {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub slug: String,
    pub created_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
}