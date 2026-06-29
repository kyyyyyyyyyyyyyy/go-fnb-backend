use actix_web::{web, HttpRequest, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use tracing::error;

use crate::services::table_service::TableService;
use crate::dto::table_dto::{CreateTableDTO, UpdateTableDTO};
use crate::utils::helper::{error_response, get_outlet_id, not_found, success, success_withDatas};

pub async fn create_table(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    payload: web::Json<CreateTableDTO>,
) -> impl Responder {

    let outlet_id = match get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return error_response(&e.to_string()),
    };

    match TableService::create(&pool, outlet_id, payload.into_inner()).await {
        Ok(result) => success_withDatas("Table created successfully", result),
        Err(err) => error_response(&err.to_string()),
    }
}

pub async fn get_table_by_id(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<Uuid>,
) -> impl Responder {

    let _outlet_id = match get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return error_response(&e.to_string()),
    };

    let table_id = path.into_inner();

    match TableService::get_by_id(&pool, table_id).await {
        Ok(result) => {
            success_withDatas("Table fetched successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Get Table By ID",
                error = ?err,
                table_id = %table_id,
            );

            error_response("Failed to get table")
        }
    }
}

pub async fn get_tables_by_outlet(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> impl Responder {

    let outlet_id = match get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return error_response(&e.to_string()),
    };

    match TableService::get_by_outlet_id(&pool, outlet_id).await {
        Ok(result) => {
            success_withDatas("Tables fetched successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Get Tables By Outlet",
                error = ?err,
                outlet_id = %outlet_id,
            );

            error_response("Failed to get tables")
        }
    }
}

pub async fn update_table(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateTableDTO>,
) -> impl Responder {

    let _outlet_id = match get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return error_response(&e.to_string()),
    };

    let table_id = path.into_inner();
    let dto = payload.into_inner();

    match TableService::update(&pool, table_id, dto).await {
        Ok(result) => {
            success_withDatas("Table updated successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Update Table",
                error = ?err,
                table_id = %table_id,
            );

            error_response("Failed to update table")
        }
    }
}

pub async fn delete_table(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<Uuid>,
) -> impl Responder {

    let _outlet_id = match get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return error_response(&e.to_string()),
    };

    let table_id = path.into_inner();

    match TableService::delete(&pool, table_id).await {
        Ok(true) => {
            success("Table deleted successfully")
        }
        Ok(false) => {
            not_found("Table not found")
        }
        Err(err) => {
            error!(
                message = "Failed To Delete Table",
                error = ?err,
                table_id = %table_id,
            );

            error_response("Failed to delete table")
        }
    }
}

pub async fn delete_token(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<Uuid>,
) -> impl Responder {

    let _outlet_id = match get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return error_response(&e.to_string()),
    };

    let table_id = path.into_inner();

    match TableService::delete_token(&pool, table_id).await {
        Ok(()) => {
            success("Table token deleted successfully")
        }
        Err(err) => {
            error!(
                message = "Failed To Delete Table Token",
                error = ?err,
                table_id = %table_id,
            );

            error_response("Failed to delete table token")
        }
    }
}
