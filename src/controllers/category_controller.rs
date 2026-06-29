use actix_web::{HttpRequest, Responder, web};
use sqlx::PgPool;

use crate::{dto::category_dto::{CreateCategoryDTO, UpdateCategoryDTO}, services::category_service, utils::{app_error::AppError, helper}};

pub async fn create_category(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    req: web::Json<CreateCategoryDTO>,
) -> impl Responder {

    let body = req.into_inner();

    let outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    match category_service::create_category(
        &pool,
        body.name,
        outlet_id,
    ).await {
        Ok(_) => helper::created("category berhasil dibuat"),
        Err(e) => helper::error_response(&e),
    }
}

pub async fn get_categories_by_outlet(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> impl Responder {

    let outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    match category_service::get_categories_by_outlet(
        &pool,
        outlet_id,
    ).await {
        Ok(data) =>
            helper::success_withDatas(
                "categories fetched successfully",
                data
            ),
        Err(e) =>
            helper::error_response(&e),
    }
}

pub async fn get_category_by_id(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> Result<impl Responder, AppError> {

    let category_id = path.into_inner();

    let _outlet_id =
        helper::get_outlet_id(&http_req)?;

    let category =
        category_service::get_category_by_id(
            &pool,
            category_id,
        )
        .await?;

    Ok(
        helper::success_withDatas(
            "category fetched successfully",
            category,
        )
    )
}

pub async fn update_category(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    req: web::Json<UpdateCategoryDTO>,
) -> impl Responder {

    let category_id = path.into_inner();

    let body = req.into_inner();

    let _outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => {
            return helper::error_response(
                &e.to_string()
            )
        }
    };

    let result =
        category_service::update_category(
            &pool,
            category_id,
            body.name,
        )
        .await;

    match result {
        Ok(_) =>
            helper::success(
                "category updated successfully"
            ),

        Err(e) =>
            helper::error_response(
                &e.to_string()
            ),
    }
}

pub async fn delete_category(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {

    let category_id = path.into_inner();

    let _outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => {
            return helper::error_response(
                &e.to_string()
            )
        }
    };

    let result =
        category_service::delete_category(
            &pool,
            category_id,
        )
        .await;

    match result {
        Ok(_) =>
            helper::success(
                "category deleted successfully"
            ),

        Err(e) =>
            helper::error_response(
                &e.to_string()
            ),
    }
}
