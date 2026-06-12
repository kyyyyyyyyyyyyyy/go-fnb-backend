use actix_web::{HttpRequest, Responder, web};
use sqlx::PgPool;

use crate::{dto::product_dto::{CreateProductDTO, UpdateProductDTO}, services::product_service, utils::{app_error::AppError, helper}};


pub async fn create_product(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    body: web::Json<CreateProductDTO>,
) -> Result<impl Responder, AppError> {

    let body =
        body.into_inner();

    let price =  body.capital_price + body.tax + body.profit;

    helper::ensure_user_has_outlet(
        &http_req,
        &pool,
        body.outlet_id,
    )
    .await?;

    product_service::create_product(
        &pool,
        body.name,
        body.capital_price,
        body.tax,
        body.profit,
        price,
        body.category_ids,
        body.outlet_id,
    )
    .await?;

    Ok(
        helper::created(
            "product berhasil dibuat"
        )
    )
}

pub async fn get_products_by_outlet(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {

    let outlet_id = path.into_inner();

    helper::ensure_user_has_outlet(
        &http_req,
        &pool,
        outlet_id,
    )
    .await?;

    let products =
        product_service::get_product_by_outlet(
            &pool,
            outlet_id,
        )
        .await?;

    Ok(
        helper::successWithDatas(
            "products fetched successfully",
            products,
        )
    )
}

pub async fn get_product_by_id(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<impl Responder, AppError> {

    let (outlet_id, product_id) =
        path.into_inner();

    helper::ensure_user_has_outlet(
        &http_req,
        &pool,
        outlet_id,
    )
    .await?;

    let product =
        product_service::get_product_by_id(
            &pool,
            product_id,
        )
        .await?;

    Ok(
        helper::successWithDatas(
            "product fetched successfully",
            product,
        )
    )
}

pub async fn update_product(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    body: web::Json<UpdateProductDTO>,
) -> impl Responder {

    let (outlet_id, product_id) =
        path.into_inner();

    let req =
        body.into_inner();

    match helper::ensure_user_has_outlet(
        &http_req,
        &pool,
        outlet_id,
    )
    .await
    {
        Ok(_) => {}

        Err(e) => {
            return helper::error_response(
                &e.to_string()
            )
        }
    }

    // ambil product lama
    let old =
        match product_service::get_product_by_id(
            &pool,
            product_id,
        )
        .await
    {
        Ok(product) => product,

        Err(e) => {
            return helper::error_response(
                &e.to_string()
            )
        }
    };

    // merge
    let capital_price =
        req.capital_price
            .unwrap_or(old.capital_price);

    let tax =
        req.tax
            .unwrap_or(old.tax);

    let profit =
        req.profit
            .unwrap_or(old.profit);

    // hitung ulang
    let price =
        capital_price
        + tax
        + profit;

    let result =
        product_service::update_product(
            &pool,

            product_id,

            req.name,

            Some(capital_price),
            Some(tax),
            Some(profit),

            Some(price),

            req.add_category_ids,
            req.remove_category_ids,
        )
        .await;

    match result {

        Ok(_) => {
            helper::success(
                "product updated successfully",
            )
        }

        Err(e) => {
            helper::error_response(
                &e.to_string()
            )
        }
    }
}

pub async fn delete_product(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
) -> impl Responder {

    let (outlet_id, product_id) =
        path.into_inner();

    match helper::ensure_user_has_outlet(
        &http_req,
        &pool,
        outlet_id,
    ).await {
        Ok(_) => {}
        Err(e) => {
            return helper::error_response(
                &e.to_string()
            )
        }
    }

    let result =
        product_service::delete_product(
            &pool,
            product_id,
        )
        .await;

    match result {
        Ok(_) =>
            helper::success(
                "product deleted successfully"
            ),

        Err(e) =>
            helper::error_response(
                &e.to_string()
            ),
    }
}
