use chrono::NaiveDateTime;

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub provider: Option<String>,
    pub provider_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
