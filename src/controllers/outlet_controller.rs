use actix_web::{HttpRequest, Responder, web};
use uuid::Uuid;
use sqlx::PgPool;

use crate::services::outlet_service::OutletService;
use crate::dto::outlet_dto::{CreateOutletDTO, UpdateOutletDTO};
use crate::utils;
use crate::utils::helper::get_user_id;
use crate::utils::app_error::AppError;

//
// 🔹 CREATE OUTLET
//
pub async fn create_outlet(
    pool: web::Data<PgPool>,
    req: web::Json<CreateOutletDTO>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {

    let user_id = get_user_id(&http_req)?;

    let _ = OutletService::create_outlet(
        &pool,
        user_id,
        req.name.clone(),
        req.address_line.clone(),
        req.city.clone(),
        req.province.clone(),
        req.postal_code.clone(),
        req.latitude,
        req.longitude,
    )
    .await?;

    Ok(utils::helper::created("Outlet created successfully"))
}

//
// 🔹 GET ALL
//
pub async fn get_all_outlets(
    pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {

    let data = OutletService::get_all_outlets(&pool).await?;

    Ok(utils::helper::success_withDatas("Outlets fetched successfully", data))
}

//
// 🔹 GET BY ID
//
pub async fn get_outlet_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {

    let data = OutletService::get_outlet_by_id(&pool, path.into_inner()).await?;

    Ok(utils::helper::success_withDatas("Outlet fetched successfully", data))
}

//
// 🔹 GET MY OUTLETS
//
pub async fn get_my_outlets(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {

    let user_id = utils::helper::get_user_id(&http_req)?;

    let data = OutletService::get_my_outlets(&pool, user_id).await?;

    Ok(utils::helper::success_withDatas("My outlets fetched successfully", data))
}

//
// 🔹 UPDATE
//
pub async fn update_outlet(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateOutletDTO>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {

    let user_id = get_user_id(&http_req)?;

    OutletService::update_outlet(
        &pool,
        user_id,
        path.into_inner(),
        req.into_inner(),
    )
    .await?;

    Ok(utils::helper::success("Outlet updated"))
}

//
// 🔹 DELETE
//
pub async fn delete_outlet(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {

    let user_id = get_user_id(&http_req)?;

    OutletService::delete_outlet(
        &pool,
        user_id,
        path.into_inner(),
    )
    .await?;

    Ok(utils::helper::success("Outlet deleted"))
}