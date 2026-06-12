use sqlx::PgPool;
use uuid::Uuid;

use crate::{dto::product_dto::ProductResponseDTO, repositories::product_repo::ProductRepository, utils::app_error::AppError};


    pub async fn create_product(
        pool: &PgPool,
        name: String,
        capital_price: i64,
        tax: i64,
        profit: i64,
        price: i64,
        category_ids: Vec<Uuid>,
        outlet_id: Uuid,
    ) -> Result<(), AppError> {

        ProductRepository::create_product(
            pool,
            name,
            capital_price,
            tax,
            profit,
            price,
            category_ids,
            outlet_id,
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }

    pub async fn get_product_by_outlet(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<ProductResponseDTO>, AppError> {

        let products =
            ProductRepository::get_product_by_outlet(
                pool,
                outlet_id,
            )
            .await
            .map_err(|e| {

                tracing::error!(
                    error = ?e,
                    outlet_id = %outlet_id,
                    "repository error"
                );

                AppError::InternalServerError
            })?;

        if products.is_empty() {
            return Err(
                AppError::NotFound(
                    "produk tidak ditemukan"
                        .to_string()
                )
            );
        }

        Ok(products)
    }


    pub async fn get_product_by_id(
        pool: &PgPool,
        product_id: Uuid,
    ) -> Result<ProductResponseDTO, AppError> {

        ProductRepository::get_product_by_id(
            pool,
            product_id,
        )
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?
        .ok_or(
            AppError::NotFound(
                "Product tidak ditemukan"
                    .to_string()
            )
        )
    }

    pub async fn update_product(
        pool: &PgPool,
        product_id: Uuid,

        name: Option<String>,
        capital_price: Option<i64>,
        tax: Option<i64>,
        profit: Option<i64>,
        price: Option<i64>,

        add_category_ids: Option<Vec<Uuid>>,
        remove_category_ids: Option<Vec<Uuid>>,
    ) -> Result<(), AppError> {

        let updated =
            ProductRepository::update_product(
                pool,
                product_id,
                name,
                capital_price,
                tax,
                profit,
                price,
                add_category_ids,
                remove_category_ids,
            )
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if !updated {
            return Err(
                AppError::NotFound(
                    "Product tidak ditemukan".to_string()
                )
            );
        }

        Ok(())
    }

    pub async fn delete_product(
        pool: &PgPool,
        product_id: Uuid,
    ) -> Result<(), AppError> {

        let deleted =
            ProductRepository::delete_product(
                pool,
                product_id,
            )
            .await
            .map_err(|_| {
                AppError::InternalServerError
            })?;

        if !deleted {
            return Err(
                AppError::NotFound(
                    "Product tidak ditemukan"
                        .to_string()
                )
            );
        }

        Ok(())
    }
