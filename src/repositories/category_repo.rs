use sqlx::PgPool;
use uuid::Uuid;

use crate::models::category_model::Category;

pub struct CategoryRepository;

impl CategoryRepository {

    pub async fn create_category(
        pool: &PgPool,
        name: String,
        outlet_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            INSERT INTO categories (name, outlet_id)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
        .bind(name)
        .bind(outlet_id)
        .fetch_one(pool)
        .await?;

        Ok(())
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
    ) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            UPDATE categories
            SET name = $1
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(name)
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