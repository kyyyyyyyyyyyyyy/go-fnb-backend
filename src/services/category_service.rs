use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::category_model::Category, repositories::category_repo::CategoryRepository, utils::app_error::AppError};

    pub async fn create_category(
        pool: &PgPool,
        name: String,
        outlet_id: Uuid,
        image_url: Option<String>,
    ) -> Result<Category, String> {

        let category = CategoryRepository::create_category(pool, name, outlet_id, image_url)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(category)
    }

    pub async fn get_categories_by_outlet(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<Category>, String> {

        let categories = CategoryRepository::get_categories_by_outlet(pool, outlet_id)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(categories)
    }

    pub async fn get_category_by_id(
        pool: &PgPool,
        category_id: Uuid,
    ) -> Result<Category, AppError> {

        CategoryRepository::get_category_by_id(pool, category_id)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(
                AppError::NotFound(
                    "Category tidak ditemukan".to_string()
                )
            )
    }

    pub async fn update_category(
        pool: &PgPool,
        category_id: Uuid,
        name: String,
        image_url: Option<String>,
    ) -> Result<Category, String> {

        let category = CategoryRepository::update_category(pool, category_id, name, image_url)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(category)
    }

    pub async fn delete_category(
        pool: &PgPool,
        category_id: Uuid,
    ) -> Result<(), String> {

        CategoryRepository::delete_category(pool, category_id)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(())
    }
