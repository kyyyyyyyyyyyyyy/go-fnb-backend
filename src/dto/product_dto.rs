use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateProductDTO {
    pub name: String,
    pub capital_price: i64,
    pub tax: i64,
    pub profit: i64,
    pub image_url: String,
    pub outlet_id: Uuid,
    pub category_ids: Vec<Uuid>
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateProductDTO {
    pub name: Option<String>,
    pub capital_price: Option<i64>,
    pub tax: Option<i64>,
    pub profit: Option<i64>,
    pub image_url: Option<String>,
    pub add_category_ids: Option<Vec<Uuid>>,
    pub remove_category_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductResponseDTO {
    pub id: Uuid,

    pub name: String,

    pub capital_price: i64,
    pub tax: i64,
    pub profit: i64,
    pub price: i64,
    pub image_url: String,

    pub categories: Vec<CategoryItemDTO>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryItemDTO {
    pub id: Uuid,
    pub name: String,
}