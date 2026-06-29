use actix_web::{web, HttpRequest, Responder};
use sqlx::PgPool;

use crate::{
    dto::order_dto::CreateOrder,
    services::order_service,
    utils::{
        app_error::AppError,
        helper,
    },
};

pub async fn create_order(
    pool: web::Data<PgPool>,
    body: web::Json<CreateOrder>,
) -> Result<impl Responder, AppError> {

    let req = body.into_inner();

    let order_id =
        order_service::create_order(
            &pool,
            req.order_name,
            req.order_type,
            req.notes,
            req.table_id,
            req.outlet_id,
            req.order_items,
        )
        .await?;

    Ok(
        helper::success_withDatas(
            "order berhasil dibuat",
            serde_json::json!({
                "id": order_id
            }),
        )
    )
}

pub async fn get_orders_by_outlet(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {

    let outlet_id =
        helper::get_outlet_id(&http_req)?;

    let orders =
        order_service::get_orders_by_outlet(
            &pool,
            outlet_id,
        )
        .await?;

    Ok(
        helper::success_withDatas(
            "orders fetched successfully",
            orders,
        )
    )
}

pub async fn get_order_by_id(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {

    let order_id = path.into_inner();

    let _outlet_id =
        helper::get_outlet_id(&http_req)?;

    let order =
        order_service::get_order_by_id(
            &pool,
            order_id,
        )
        .await?;

    Ok(
        helper::success_withDatas(
            "order fetched successfully",
            order,
        )
    )
}

pub async fn update_order(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<crate::dto::order_dto::UpdateOrder>,
) -> impl Responder {

    let order_id = path.into_inner();

    let user_id =
        match helper::get_user_id(&http_req) {
            Ok(uid) => uid,
            Err(e) => {
                return helper::error_response(
                    &e.to_string()
                )
            }
        };

    let req = body.into_inner();

    let result =
        order_service::update_order(
            &pool,
            order_id,
            req.order_name,
            req.order_type,
            req.table_id,
            req.notes,
            user_id,
            req.update_items,
            req.add_items,
            req.remove_item_ids,
        )
        .await;

    match result {
        Ok(_) =>
            helper::success(
                "order updated successfully"
            ),

        Err(e) =>
            helper::error_response(
                &e.to_string()
            ),
    }
}

pub async fn update_order_status(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    body: web::Json<crate::dto::order_dto::UpdateOrderStatus>,
) -> impl Responder {

    let order_id = path.into_inner();

    let _outlet_id =
        match helper::get_outlet_id(&http_req) {
            Ok(id) => id,
            Err(e) => {
                return helper::error_response(
                    &e.to_string()
                )
            }
        };

    let req = body.into_inner();

    let result =
        order_service::update_order_status(
            &pool,
            order_id,
            req.status,
        )
        .await;

    match result {
        Ok(_) =>
            helper::success(
                "order status updated successfully"
            ),

        Err(e) =>
            helper::error_response(
                &e.to_string()
            ),
    }
}

pub async fn delete_order(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {

    let order_id = path.into_inner();

    let _outlet_id =
        match helper::get_outlet_id(&http_req) {
            Ok(id) => id,
            Err(e) => {
                return helper::error_response(
                    &e.to_string()
                )
            }
        };

    let result =
        order_service::delete_order(
            &pool,
            order_id,
        )
        .await;

    match result {
        Ok(_) =>
            helper::success(
                "order deleted successfully"
            ),

        Err(e) =>
            helper::error_response(
                &e.to_string()
            ),
    }
}
