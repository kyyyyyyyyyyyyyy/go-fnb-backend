use actix_web::{HttpRequest, Responder, web, HttpResponse};
use uuid::Uuid;
use sqlx::PgPool;

use crate::services::outlet_service::OutletService;
use crate::dto::outlet_dto::{CreateOutletDTO, UpdateOutletDTO};
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

    let id = OutletService::create_outlet(
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

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": { "id": id }
    })))
}

//
// 🔹 GET ALL
//
pub async fn get_all_outlets(
    pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {

    let data = OutletService::get_all_outlets(&pool).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}

//
// 🔹 GET BY ID
//
pub async fn get_outlet_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {

    let data = OutletService::get_outlet_by_id(&pool, path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}

//
// 🔹 GET MY OUTLETS
//
pub async fn get_my_outlets(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> Result<impl Responder, AppError> {

    let user_id = get_user_id(&http_req)?;

    let data = OutletService::get_my_outlets(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
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

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Outlet updated"
    })))
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

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Outlet deleted"
    })))
}