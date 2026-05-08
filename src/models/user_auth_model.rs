// models/user_auth.rs
#[allow(dead_code)]
pub struct UserAuth {
    pub id: String,
    pub password: Option<String>,
    pub provider: Option<String>,
}
