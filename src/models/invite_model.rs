use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Invite {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub role: String,
    pub token: String,
    pub expired_at: Option<NaiveDateTime>,
    pub used: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct CreateResult {
    pub token: String,
}
