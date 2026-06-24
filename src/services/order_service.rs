use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{
    dto::order_dto::{
        OrderItems as OrderItemDTO,
        OrderResponseDTO,
    },
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

        let qty = item.qty as i64;

        let item_capital =
            product.capital_price * qty;

        let item_tax =
            product.tax * qty;

        let item_profit =
            product.profit * qty;

        let item_total =
            (
                product.price * qty
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

pub async fn get_orders_by_outlet(
    pool: &PgPool,
    outlet_id: Uuid,
) -> Result<Vec<OrderResponseDTO>, AppError> {

    let orders =
        OrderRepository::get_orders_by_outlet(
            pool,
            outlet_id,
        )
        .await
        .map_err(|e| {
            error!(
                error = ?e,
                outlet_id = %outlet_id,
                "failed get orders"
            );
            AppError::InternalServerError
        })?;

    if orders.is_empty() {
        return Err(
            AppError::NotFound(
                "order tidak ditemukan"
                    .to_string()
            )
        );
    }

    let mut result = Vec::new();

    for order in orders {
        let items =
            OrderRepository::get_order_items(
                pool,
                order.id,
            )
            .await
            .map_err(|e| {
                error!(
                    error = ?e,
                    order_id = %order.id,
                    "failed get order items"
                );
                AppError::InternalServerError
            })?;

        result.push(
            OrderResponseDTO {
                id: order.id,
                order_name: order.order_name,
                order_type: order.order_type,
                order_status: order.status,
                order_number: order.order_number,
                capital_price: order.capital_price,
                tax: order.tax,
                profit: order.profit,
                total: order.total,
                notes: order.notes,
                table_id: order.table_id,
                outlet_id: order.outlet_id,
                created_at: order.created_at,
                updated_at: order.updated_at,
                change_by: order.change_by,
                items,
            }
        );
    }

    Ok(result)
}

pub async fn get_order_by_id(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<OrderResponseDTO, AppError> {

    let order =
        OrderRepository::get_order_by_id(
            pool,
            order_id,
        )
        .await
        .map_err(|e| {
            error!(
                error = ?e,
                order_id = %order_id,
                "failed get order"
            );
            AppError::InternalServerError
        })?
        .ok_or(
            AppError::NotFound(
                "order tidak ditemukan".to_string()
            )
        )?;

    let items =
        OrderRepository::get_order_items(
            pool,
            order.id,
        )
        .await
        .map_err(|e| {
            error!(
                error = ?e,
                order_id = %order.id,
                "failed get order items"
            );
            AppError::InternalServerError
        })?;

    Ok(
        OrderResponseDTO {
            id: order.id,
            order_name: order.order_name,
            order_type: order.order_type,
            order_status: order.status,
            order_number: order.order_number,
            capital_price: order.capital_price,
            tax: order.tax,
            profit: order.profit,
            total: order.total,
            notes: order.notes,
            table_id: order.table_id,
            outlet_id: order.outlet_id,
            created_at: order.created_at,
            updated_at: order.updated_at,
            change_by: order.change_by,
            items,
        }
    )
}

pub async fn update_order(
    pool: &PgPool,
    order_id: Uuid,

    order_name: Option<String>,
    order_type: Option<String>,
    table_id: Option<Uuid>,
    notes: Option<String>,
    change_by: Uuid,
) -> Result<(), AppError> {

    let updated =
        OrderRepository::update_order(
            pool,
            order_id,
            order_name,
            order_type,
            table_id,
            notes,
            change_by,
        )
        .await
        .map_err(|e| {
            error!(
                error = ?e,
                order_id = %order_id,
                "failed update order"
            );
            AppError::InternalServerError
        })?;

    if !updated {
        return Err(
            AppError::NotFound(
                "order tidak ditemukan".to_string()
            )
        );
    }

    Ok(())
}

pub async fn update_order_status(
    pool: &PgPool,
    order_id: Uuid,
    status: String,
) -> Result<(), AppError> {

    let updated =
        OrderRepository::update_order_status(
            pool,
            order_id,
            status,
        )
        .await
        .map_err(|e| {
            error!(
                error = ?e,
                order_id = %order_id,
                "failed update order status"
            );
            AppError::InternalServerError
        })?;

    if !updated {
        return Err(
            AppError::NotFound(
                "order tidak ditemukan".to_string()
            )
        );
    }

    Ok(())
}

pub async fn delete_order(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<(), AppError> {

    let deleted =
        OrderRepository::delete_order(
            pool,
            order_id,
        )
        .await
        .map_err(|e| {
            error!(
                error = ?e,
                order_id = %order_id,
                "failed delete order"
            );
            AppError::InternalServerError
        })?;

    if !deleted {
        return Err(
            AppError::NotFound(
                "order tidak ditemukan".to_string()
            )
        );
    }

    Ok(())
}
