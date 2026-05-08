use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::models::table_model::Table;

pub struct TableRepository;

impl TableRepository {
    pub async fn create(
        pool: &PgPool,
        outlet_id: Uuid,
        name: String,
        location: String,
    ) -> Result<Table, sqlx::Error> {

        let table = sqlx::query_as::<_, Table>(
            r#"
            INSERT INTO tables (outlet_id, name, location)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(outlet_id)
        .bind(name)
        .bind(location)
        .fetch_one(pool)
        .await?;

        Ok(table)
    }

    pub async fn get_by_id(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<Table, sqlx::Error> {

        let table = sqlx::query_as::<_, Table>(
            r#"
            SELECT * FROM tables WHERE id = $1
            "#
        )
        .bind(table_id)
        .fetch_one(pool)
        .await?;

        Ok(table)
    }

    pub async fn get_by_outlet_id(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<Table>, sqlx::Error> {

        let tables = sqlx::query_as::<_, Table>(
            r#"
            SELECT * FROM tables WHERE outlet_id = $1
            "#
        )
        .bind(outlet_id)
        .fetch_all(pool)
        .await?;

        Ok(tables)
    }

    pub async fn update(
        pool: &PgPool,
        table_id: Uuid,
        name: Option<String>,
        location: Option<String>,
        status: Option<String>,
    ) -> Result<Table, sqlx::Error> {

        let mut qb = QueryBuilder::new("UPDATE tables SET ");

        let mut separated = qb.separated(", ");

        if let Some(name) = name {
            separated.push("name = ").push_bind(name);
        }

        if let Some(location) = location {
            separated.push("location = ").push_bind(location);
        }

        if let Some(status) = status {
            separated.push("status = ").push_bind(status);
        }

        qb.push(" WHERE id = ").push_bind(table_id);
        qb.push(" RETURNING *");

        let query = qb.build_query_as::<Table>();

        let table = query.fetch_one(pool).await?;

        Ok(table)
    }

    pub async fn delete(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {

        let result = sqlx::query(
            "DELETE FROM tables WHERE id = $1"
        )
        .bind(table_id)
        .execute(pool)
        .await?;

        Ok(result)
    }

}