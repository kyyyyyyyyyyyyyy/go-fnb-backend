use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::order_model::{
    Order,
    OrderItems,
};

pub struct OrderRepository;

impl OrderRepository {

    pub async fn create_order(
        pool: &PgPool,

        order_name: String,
        order_type: String,
        order_status: String,
        order_number: String,

        capital_price: i64,
        tax: i64,
        profit: i64,
        total: i64,

        notes: Option<String>,
        table_id: Uuid,
        outlet_id: Uuid,

        items: Vec<OrderItems>,
    ) -> Result<Uuid, sqlx::Error> {

        let mut tx: Transaction<'_, Postgres> =
            pool.begin().await?;

        let order_id: Uuid =
            sqlx::query_scalar(
                r#"
                INSERT INTO orders
                (
                    order_name,
                    order_type,
                    status,
                    order_number,

                    capital_price,
                    tax,
                    profit,
                    total,

                    notes,
                    table_id,
                    outlet_id
                )
                VALUES
                (
                    $1,$2,$3::order_status,$4,
                    $5,$6,$7,$8,
                    $9,$10,$11
                )
                RETURNING id
                "#
            )
            .bind(order_name)
            .bind(order_type)
            .bind(order_status)
            .bind(order_number)

            .bind(capital_price)
            .bind(tax)
            .bind(profit)
            .bind(total)

            .bind(notes)
            .bind(table_id)
            .bind(outlet_id)

            .fetch_one(&mut *tx)
            .await?;

        for item in items {

            sqlx::query(
                r#"
                INSERT INTO order_items
                (
                    order_id,
                    product_id,
                    qty,

                    sub_total,

                    capital_price,
                    profit,
                    tax,

                    discount,
                    notes
                )
                VALUES
                (
                    $1,$2,$3,
                    $4,
                    $5,$6,$7,
                    $8,$9
                )
                "#
            )
            .bind(order_id)
            .bind(item.product_id)
            .bind(item.qty)

            .bind(item.sub_total)

            .bind(item.capital_price)
            .bind(item.profit)
            .bind(item.tax)

            .bind(item.discount)
            .bind(item.notes)

            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(order_id)
    }

    pub async fn get_order_by_id(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Option<Order>, sqlx::Error> {

        sqlx::query_as(
            r#"
            SELECT
                id, order_name, order_type, status::TEXT AS status, order_number,
                capital_price, tax, profit, total, notes, table_id, outlet_id,
                created_at, updated_at, change_by
            FROM orders
            WHERE id = $1
            "#
        )
        .bind(order_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_order_items(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Vec<OrderItems>, sqlx::Error> {

        sqlx::query_as(
            r#"
            SELECT *
            FROM order_items
            WHERE order_id = $1
            "#
        )
        .bind(order_id)
        .fetch_all(pool)
        .await
    }

    pub async fn get_orders_by_outlet(
        pool: &PgPool,
        outlet_id: Uuid,
    ) -> Result<Vec<Order>, sqlx::Error> {

        sqlx::query_as(
            r#"
            SELECT
                id, order_name, order_type, status::TEXT AS status, order_number,
                capital_price, tax, profit, total, notes, table_id, outlet_id,
                created_at, updated_at, change_by
            FROM orders
            WHERE outlet_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(outlet_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update_order_status(
        pool: &PgPool,
        order_id: Uuid,
        status: String,
    ) -> Result<bool, sqlx::Error> {

        let result =
            sqlx::query(
                r#"
                UPDATE orders
                SET
                    status = $1::order_status,
                    updated_at = NOW()

                WHERE id = $2
                "#
            )
            .bind(status)
            .bind(order_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_order_with_items(
        pool: &PgPool,
        order_id: Uuid,
        order_name: Option<String>,
        order_type: Option<String>,
        table_id: Option<Uuid>,
        notes: Option<String>,
        change_by: Uuid,
        update_items: &[(Uuid, i32, i64)],
        add_items: &[OrderItems],
        remove_item_ids: &[Uuid],
    ) -> Result<bool, sqlx::Error> {

        let mut tx: Transaction<'_, Postgres> =
            pool.begin().await?;

        let result =
            sqlx::query(
                r#"
                UPDATE orders
                SET
                    order_name = COALESCE($1, order_name),
                    order_type = COALESCE($2, order_type),
                    table_id = COALESCE($3, table_id),
                    notes = COALESCE($4, notes),
                    change_by = $5,
                    updated_at = NOW()
                WHERE id = $6
                "#
            )
            .bind(order_name)
            .bind(order_type)
            .bind(table_id)
            .bind(notes)
            .bind(change_by)
            .bind(order_id)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            tx.rollback().await?;
            return Ok(false);
        }

        for (item_id, qty, sub_total) in update_items {
            sqlx::query(
                r#"
                UPDATE order_items
                SET qty = $1, sub_total = $2
                WHERE id = $3 AND order_id = $4
                "#
            )
            .bind(qty)
            .bind(sub_total)
            .bind(item_id)
            .bind(order_id)
            .execute(&mut *tx)
            .await?;
        }

        for item in add_items {
            sqlx::query(
                r#"
                INSERT INTO order_items
                (order_id, product_id, qty, sub_total, capital_price, profit, tax, discount, notes)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#
            )
            .bind(order_id)
            .bind(item.product_id)
            .bind(item.qty)
            .bind(item.sub_total)
            .bind(item.capital_price)
            .bind(item.profit)
            .bind(item.tax)
            .bind(item.discount)
            .bind(item.notes.clone())
            .execute(&mut *tx)
            .await?;
        }

        if !remove_item_ids.is_empty() {
            sqlx::query(
                r#"
                DELETE FROM order_items
                WHERE id = ANY($1) AND order_id = $2
                "#
            )
            .bind(remove_item_ids)
            .bind(order_id)
            .execute(&mut *tx)
            .await?;
        }

        let totals: (i64, i64, i64, i64) =
            sqlx::query_as(
                r#"
                SELECT
                    COALESCE(SUM(capital_price * qty)::BIGINT, 0),
                    COALESCE(SUM(tax * qty)::BIGINT, 0),
                    COALESCE(SUM(profit * qty)::BIGINT, 0),
                    COALESCE(SUM(sub_total)::BIGINT, 0)
                FROM order_items
                WHERE order_id = $1
                "#
            )
            .bind(order_id)
            .fetch_one(&mut *tx)
            .await?;

        sqlx::query(
            r#"
            UPDATE orders
            SET
                capital_price = $1,
                tax = $2,
                profit = $3,
                total = $4
            WHERE id = $5
            "#
        )
        .bind(totals.0)
        .bind(totals.1)
        .bind(totals.2)
        .bind(totals.3)
        .bind(order_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(true)
    }

    pub async fn delete_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<bool, sqlx::Error> {

        let result =
            sqlx::query(
                r#"
                DELETE FROM orders
                WHERE id = $1
                "#
            )
            .bind(order_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}