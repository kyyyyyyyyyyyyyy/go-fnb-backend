use sqlx::PgPool;
use uuid::Uuid;

use crate::models::category_model::Category;

pub struct CategoryRepository;

impl CategoryRepository {

    pub async fn create_category(
        pool: &PgPool,
        name: String,
        outlet_id: Uuid,
        image_url: Option<String>,
    ) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            INSERT INTO categories (name, outlet_id, image_url)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(name)
        .bind(outlet_id)
        .bind(image_url)
        .fetch_one(pool)
        .await?;

        Ok(category)
    }

    pub async fn get_categories_by_outlet(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as::<_, Category>(
            r#"
            SELECT * FROM categories
            WHERE outlet_id = $1
            "#
        )
        .bind(outlet_id)
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn get_category_by_id(
        pool: &PgPool,
        category_id: Uuid,
    ) -> Result<Option<Category>, sqlx::Error> {

        let category =
            sqlx::query_as::<_, Category>(
                r#"
                SELECT *
                FROM categories
                WHERE id = $1
                "#
            )
            .bind(category_id)
            .fetch_optional(pool)
            .await?;

        Ok(category)
    }

    pub async fn update_category(
        pool: &PgPool,
        category_id: Uuid,
        name: String,
        image_url: Option<String>,
    ) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            UPDATE categories
            SET name = $1, image_url = $2
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(name)
        .bind(image_url)
        .bind(category_id)
        .fetch_one(pool)
        .await?;

        Ok(category)
    }

    pub async fn delete_category(
        pool: &PgPool,
        category_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM categories
            WHERE id = $1
            "#
        )
        .bind(category_id)
        .execute(pool)
        .await?;

        Ok(())
    }

}