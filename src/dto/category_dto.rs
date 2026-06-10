use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateCategoryDTO {
    pub name: String,
    pub outlet_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDTO {
    pub name: String,
}