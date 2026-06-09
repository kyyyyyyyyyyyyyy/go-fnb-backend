use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::models::table_model::Table;

pub struct TableRepository;

impl TableRepository {
    pub async fn create(
        pool: &PgPool,
        outlet_id: Uuid,
        name: String,
        location: Option<String>,
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

    pub async fn find_by_id(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<Option<Table>, sqlx::Error> {

        let table = sqlx::query_as::<_, Table>(
            r#"
            SELECT * FROM tables WHERE id = $1
            "#
        )
        .bind(table_id)
        .fetch_optional(pool)
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

    pub async fn get_available_tables_by_qr(
        pool: &PgPool,
        qr_id: Uuid,
    ) -> Result<Vec<Table>, sqlx::Error> {

        let tables = sqlx::query_as::<_, Table>(
            r#"
            SELECT
                t.*
            FROM
                tables t
                JOIN qr_code_tables qct
                    ON qct.table_id = t.id
            WHERE
                qct.qr_id = $1
                AND t.token IS NULL
            "#
        )
        .bind(qr_id)
        .fetch_all(pool)
        .await?;

        Ok(tables)
    }

    pub async fn set_table_token(
        pool: &PgPool,
        table_id: Uuid,
        token: &str,
    ) -> Result<(), sqlx::Error> {

        sqlx::query(
            r#"
            UPDATE tables
            SET token = $1,
                status = 'occupied'
            WHERE id = $2
            "#
        )
        .bind(token)
        .bind(table_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(
    pool: &PgPool,
    table_id: Uuid,
    name: Option<String>,
    location: Option<String>,
    status: Option<String>,
) -> Result<Table, sqlx::Error> {

    // Minimal ada 1 field yang diupdate
    if name.is_none()
        && location.is_none()
        && status.is_none()
    {
        return Err(
            sqlx::Error::Protocol(
                "No fields to update".into()
            )
        );
    }

    let mut qb =
        QueryBuilder::<sqlx::Postgres>::new(
            "UPDATE tables SET "
        );

    let mut has_previous = false;

    // Update name
    if let Some(name) = name {

        if has_previous {
            qb.push(", ");
        }

        qb.push("name = ");
        qb.push_bind(name);

        has_previous = true;
    }

    // Update location
    if let Some(location) = location {

        if has_previous {
            qb.push(", ");
        }

        qb.push("location = ");
        qb.push_bind(location);

        has_previous = true;
    }

    // Update status
    if let Some(status) = status {

        if has_previous {
            qb.push(", ");
        }

        qb.push("status = ");
        qb.push_bind(status);
    }

    qb.push(" WHERE id = ");
    qb.push_bind(table_id);

    qb.push(" RETURNING *");

    // Debug query
    println!("{}", qb.sql());

    let table = qb
        .build_query_as::<Table>()
        .fetch_one(pool)
        .await?;

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

    pub async fn delete_token(
        pool: &PgPool,
        table_id: Uuid,
    ) -> Result<(), sqlx::Error> {

        sqlx::query(
            r#"
            UPDATE tables
            SET token = NULL,
                status = 'available'
            WHERE id = $1
            "#
        )
        .bind(table_id)
        .execute(pool)
        .await?;

        Ok(())
    }

}