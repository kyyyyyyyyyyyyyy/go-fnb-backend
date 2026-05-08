use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::services::qr_code_service::QrCodeService;
use crate::dto::qr_code_dto::{CreateQrDTO, CreateQrWithTablesDTO};
use crate::dto::qr_scan_dto::ScanQrDTO;
use crate::utils::helper::{created, error_response, success, not_found};

pub async fn create_qr_with_tables(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateQrWithTablesDTO>,
) -> impl Responder {

    match QrCodeService::create_with_tables(&pool, payload.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            eprintln!("CREATE QR WITH TABLES ERROR: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to create QR with tables")
        }
    }
}

//
// 🔥 CREATE QR
//
pub async fn create_qr(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateQrDTO>,
) -> impl Responder {

    let dto = payload.into_inner();

    match QrCodeService::create(&pool, &dto).await {
        Ok(result) => {
            created("qr created successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Create QR",
                error = ?err,
                payload = ?dto,
            );
            
            error_response("Failed to create QR")
        }
    }
}

//
// 🔍 SCAN QR
//
pub async fn scan_qr(
    pool: web::Data<PgPool>,
    payload: web::Json<ScanQrDTO>,
) -> impl Responder {

    let slug = &payload.slug;

    match QrCodeService::scan(&pool, slug).await {
        Ok(Some(result)) => {
            success("QR found", result)
        }
        Ok(None) => {
            not_found("QR not found or inactive")
        }
        Err(err) => {
            error!(
                message = "Failed To Scan QR",
                error = ?err,
                slug = %slug,
            );

            error_response("Failed to scan QR")
        }
    }
}

// 🔍 Get tables by QR ID
pub async fn get_qr_tables(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {

    let qr_id = path.into_inner();

    match QrCodeService::get_tables(&pool, qr_id).await {
        Ok(result) => {
            success("Tables fetched successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Get Tables By QR",
                error = ?err,
                qr_id = %qr_id,
            );

            error_response("Failed to get tables")
        }
    }
}


// 🔍 Get all QR by outlet
pub async fn get_qr_by_outlet(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {

    let outlet_id = path.into_inner();

    match QrCodeService::get_by_outlet(&pool, outlet_id).await {
        Ok(result) => {
            success("QRs fetched successfully", result)
        }
        Err(err) => {
            error!(
                message = "Failed To Get QR By Outlet",
                error = ?err,
                outlet_id = %outlet_id,
            );

            error_response("Failed to get QR")
        }
    }
}


// 🔄 Regenerate slug
pub async fn regenerate_qr_slug(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {

    let qr_id = path.into_inner();

    match QrCodeService::regenerate_slug(&pool, qr_id).await {
        Ok(result) => {
            success("QR slug regenerated successfully", result)
        }
        Err(sqlx::Error::RowNotFound) => {
            not_found("QR not found")
        }
        Err(err) => {
            error!(
                message = "Failed To Regenerate QR Slug",
                error = ?err,
                qr_id = %qr_id,
            );

            error_response("Failed to regenerate slug")
        }
    }
}