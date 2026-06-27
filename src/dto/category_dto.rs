use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateCategoryDTO {
    pub name: String,
    pub outlet_id: Uuid,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDTO {
    pub name: String,
    pub image_url: Option<String>,
}