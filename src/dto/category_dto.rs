use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateCategoryDTO {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDTO {
    pub name: String,
}