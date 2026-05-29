use actix_web::{web, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use tracing::error;

use crate::services::table_service::TableService;
use crate::dto::table_dto::{CreateTableDTO, UpdateTableDTO};
use crate::utils::helper::{error_response, success, not_found};

pub async fn create_table(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateTableDTO>,
) -> impl Responder {

    match TableService::create(&pool, payload.into_inner()).await {
        Ok(result) => success("Table created successfully", result),
        Err(err) => error_response(&err.to_string()),
    }
}

pub async fn get_table_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {

    let table_id = path.into_inner();

    match TableService::get_by_id(&pool, table_id).await {
        Ok(result) => {
            success("Table fetched successfully", result)
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
    path: web::Path<Uuid>,
) -> impl Responder {

    let outlet_id = path.into_inner();

    match TableService::get_by_outlet_id(&pool, outlet_id).await {
        Ok(result) => {
            success("Tables fetched successfully", result)
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
    path: web::Path<Uuid>,
    payload: web::Json<UpdateTableDTO>,
) -> impl Responder {

    let table_id = path.into_inner();
    let dto = payload.into_inner();

    match TableService::update(&pool, table_id, dto).await {
        Ok(result) => {
            success("Table updated successfully", result)
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
    path: web::Path<Uuid>,
) -> impl Responder {

    let table_id = path.into_inner();

    match TableService::delete(&pool, table_id).await {
        Ok(true) => {
            success("Table deleted successfully", ())
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
    path: web::Path<Uuid>,
) -> impl Responder {

    let table_id = path.into_inner();

    match TableService::delete_token(&pool, table_id).await {
        Ok(()) => {
            success("Table token deleted successfully", ())
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
