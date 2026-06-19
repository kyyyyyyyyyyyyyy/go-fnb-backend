use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{
    dto::order_dto::OrderItems as OrderItemDTO,
    models::order_model::OrderItems,
    repositories::{
        order_repo::OrderRepository,
        product_repo::ProductRepository,
    },
    utils::app_error::AppError,
};

pub async fn create_order(
    pool: &PgPool,

    order_name: String,
    order_type: String,
    notes: Option<String>,
    table_id: Uuid,
    outlet_id: Uuid,

    items: Vec<OrderItemDTO>,
) -> Result<Uuid, AppError> {

    let mut db_items = Vec::new();

    let mut capital_price = 0;
    let mut tax = 0;
    let mut profit = 0;
    let mut total = 0;

    for item in items {

        let product =
            ProductRepository::get_product_by_id(
                pool,
                item.product_id,
            )
            .await
            .map_err(|e| {

                error!(
                    error = ?e,
                    product_id = %item.product_id,
                    "failed get product"
                );

                AppError::InternalServerError
            })?
            .ok_or(
                AppError::NotFound(
                    "produk tidak ditemukan".into()
                )
            )?;

        let item_capital =
            product.capital_price * item.qty;

        let item_tax =
            product.tax * item.qty;

        let item_profit =
            product.profit * item.qty;

        let item_total =
            (
                product.price
                * item.qty
            )
            + item_tax;

        capital_price += item_capital;
        tax += item_tax;
        profit += item_profit;
        total += item_total;

        db_items.push(
            OrderItems {
                id: None,

                order_id: None,

                product_id: item.product_id,

                qty: item.qty,

                sub_total: item_total,

                capital_price:
                    product.capital_price,

                profit:
                    product.profit,

                tax:
                    product.tax,

                discount: 0,

                notes:
                    item.notes,
            }
        );
    }

    let order_number =
        format!(
            "{}",
            rand::random::<u32>()
        );

    let order_id =
        OrderRepository::create_order(
            pool,

            order_name,
            order_type,

            "pending".into(),

            order_number,

            capital_price,
            tax,
            profit,
            total,

            notes,

            table_id,
            outlet_id,

            db_items,
        )
        .await
        .map_err(|e| {

            error!(
                error = ?e,
                outlet_id = %outlet_id,
                table_id = %table_id,
                "failed create order"
            );

            AppError::InternalServerError
        })?;

    Ok(order_id)
}