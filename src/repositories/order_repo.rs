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
                    $1,$2,$3,$4,
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
            SELECT *
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
            SELECT *
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
                    order_status = $1,
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