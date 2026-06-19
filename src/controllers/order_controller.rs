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
        helper::successWithDatas(
            "order berhasil dibuat",
            serde_json::json!({
                "id": order_id
            }),
        )
    )
}