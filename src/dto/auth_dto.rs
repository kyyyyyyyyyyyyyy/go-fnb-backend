use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}
