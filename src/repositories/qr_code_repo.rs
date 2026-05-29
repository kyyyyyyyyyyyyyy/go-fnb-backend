use sqlx::{PgPool};
use uuid::Uuid;

use crate::models::qr_code_model::QrCode;
use crate::models::table_model::Table;

pub struct QrCodeRepository;

impl QrCodeRepository {

    // 🔥 Create QR
    pub async fn create_qr(
        pool: &PgPool,
        outlet_id: Uuid,
        slug: String,
    ) -> Result<QrCode, sqlx::Error> {
        let qr = sqlx::query_as::<_, QrCode>(
            r#"
            INSERT INTO qr_codes (outlet_id, slug)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
        .bind(outlet_id)
        .bind(slug)
        .fetch_one(pool)
        .await?;

        Ok(qr)
    }

    // 🔥 Insert pivot (relasi QR ke tables)
    pub async fn attach_tables(
        pool: &PgPool,
        qr_id: Uuid,
        table_ids: Vec<Uuid>,
    ) -> Result<(), sqlx::Error> {

        for table_id in table_ids {
            sqlx::query(
                r#"
                INSERT INTO qr_code_tables (qr_id, table_id)
                VALUES ($1, $2)
                ON CONFLICT (qr_id, table_id) DO NOTHING
                "#
            )
            .bind(qr_id)
            .bind(table_id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    // 🔍 Scan QR (by slug)
    pub async fn find_by_slug(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<QrCode>, sqlx::Error> {

        let qr = sqlx::query_as::<_, QrCode>(
            "SELECT * FROM qr_codes WHERE slug = $1"
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;

        Ok(qr)
    }

    // 🔥 Get tables dari QR
    pub async fn get_tables_by_qr(
        pool: &PgPool,
        qr_id: Uuid,
    ) -> Result<Vec<Table>, sqlx::Error> {

        let tables = sqlx::query_as::<_, Table>(
            r#"
            SELECT t.*
            FROM tables t
            JOIN qr_code_tables qct ON qct.table_id = t.id
            WHERE qct.qr_id = $1
            "#
        )
        .bind(qr_id)
        .fetch_all(pool)
        .await?;

        Ok(tables)
    }

    pub async fn regenerate_slug(
        pool: &PgPool,
        qr_id: Uuid,
        new_slug: String,
    ) -> Result<QrCode, sqlx::Error> {

        let qr = sqlx::query_as::<_, QrCode>(
            r#"
            UPDATE qr_codes
            SET slug = $2
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(qr_id)
        .bind(new_slug)
        .fetch_one(pool)
        .await?;

        Ok(qr)
    }

    pub async fn get_by_outlet_id(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<QrCode>, sqlx::Error> {

        let qrs = sqlx::query_as::<_, QrCode>(
            r#"
            SELECT *
            FROM qr_codes
            WHERE outlet_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(outlet_id)
        .fetch_all(pool)
        .await?;

        Ok(qrs)
    }

    pub async fn delete_qr(
        pool: &PgPool,
        qr_id: Uuid,
    ) -> Result<(sqlx::postgres::PgQueryResult), sqlx::Error> {

        let result = sqlx::query(
            "DELETE FROM qr_codes WHERE id = $1"
        )
        .bind(qr_id)
        .execute(pool)
        .await?;

        Ok(result)
    }
}