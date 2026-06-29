use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::dto::qr_scan_dto::SelectTableDTO;
use crate::services::qr_code_service::QrCodeService;
use crate::dto::qr_code_dto::{CreateQrDTO, CreateQrWithTablesDTO};
use crate::utils::helper;

pub async fn create_qr_with_tables(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    payload: web::Json<CreateQrWithTablesDTO>,
) -> impl Responder {

    let outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    match QrCodeService::create_with_tables(&pool, outlet_id, payload.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            eprintln!("CREATE QR WITH TABLES ERROR: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to create QR with tables")
        }
    }
}

pub async fn create_qr(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    payload: web::Json<CreateQrDTO>,
) -> impl Responder {

    let outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    let dto = payload.into_inner();

    match QrCodeService::create(&pool, outlet_id, &dto).await {
        Ok(result) => {
            helper::success_withDatas("qr created successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Create QR",
                error = ?err,
                payload = ?dto,
            );

            helper::error_response("Failed to create QR")
        }
    }
}

pub async fn scan_qr(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {

    let slug = path.into_inner();

    match QrCodeService::scan(&pool, &slug).await {
        Ok(Some(result)) => {
            helper::success_withDatas("QR found", result)
        }
        Ok(None) => {
            helper::not_found("QR not found or inactive")
        }
        Err(err) => {
            error!(
                message = "Failed To Scan QR",
                error = ?err,
                slug = %slug,
            );

            helper::error_response(&err.to_string())
        }
    }
}

pub async fn select_table(
    pool: web::Data<PgPool>,
    payload: web::Json<SelectTableDTO>,
) -> impl Responder {

    let dto = payload.into_inner();

    match QrCodeService::select_table(
        &pool,
        dto.table_id,
    )
    .await
    {
        Ok(token) => {
            helper::success_withDatas("Table selected", token)
        }

        Err(err) => {

            error!(
                error = ?err,
                table_id = %dto.table_id,
                "Failed To Select Table"
            );

            helper::error_response("Failed to select table")
        }
    }
}

pub async fn get_qr_tables(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<Uuid>,
) -> impl Responder {

    let _outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    let qr_id = path.into_inner();

    match QrCodeService::get_tables(&pool, qr_id).await {
        Ok(result) => {
            helper::success_withDatas("Tables fetched successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Get Tables By QR",
                error = ?err,
                qr_id = %qr_id,
            );

            helper::error_response("Failed to get tables")
        }
    }
}

pub async fn get_qr_by_outlet(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> impl Responder {

    let outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    match QrCodeService::get_by_outlet(&pool, outlet_id).await {
        Ok(result) => {
            helper::success_withDatas("QRs fetched successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Get QR By Outlet",
                error = ?err,
                outlet_id = %outlet_id,
            );

            helper::error_response("Failed to get QR")
        }
    }
}

pub async fn regenerate_qr_slug(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    path: web::Path<Uuid>,
) -> impl Responder {

    let _outlet_id = match helper::get_outlet_id(&http_req) {
        Ok(id) => id,
        Err(e) => return helper::error_response(&e.to_string()),
    };

    let qr_id = path.into_inner();

    match QrCodeService::regenerate_slug(&pool, qr_id).await {
        Ok(result) => {
            helper::success_withDatas("QR slug regenerated successfully", result)
        }
        Err(sqlx::Error::RowNotFound) => {
            helper::not_found("QR not found")
        }
        Err(err) => {
            error!(
                message = "Failed To Regenerate QR Slug",
                error = ?err,
                qr_id = %qr_id,
            );

            helper::error_response("Failed to regenerate slug")
        }
    }
}
