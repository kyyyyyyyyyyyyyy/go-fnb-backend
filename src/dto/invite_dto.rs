use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateInviteDTO {
    pub outlet_id: uuid::Uuid,
    pub role: String, // "admin" / "cashier"
}

#[derive(Deserialize)]
pub struct AcceptInviteDTO {
    pub token: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
