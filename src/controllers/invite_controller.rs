use std::env;

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use sqlx::PgPool;

use crate::{dto::invite_dto::CreateInviteDTO, services::invite_service::InviteService};


//
// 🔥 CREATE INVITE
//
pub async fn create_invite(
    pool: web::Data<PgPool>,
    body: web::Json<CreateInviteDTO>,
) -> impl Responder {

        let port = env::var("APP_PORT").unwrap_or("8080".to_string());

    let result = InviteService::create_invite(
        &pool,
        body.outlet_id,
        &body.role,
    )
    .await;

    match result {
        Ok(invite) => HttpResponse::Ok().json(json!({
            "success": true,
            "url": format!(
                    "http://localhost:{}/api/invite/use/{}",
                    port,
                    invite.token
                )
    })),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err
        })),
    }
}

//
// 🔍 VALIDATE INVITE (GET /invite/{token})
//
pub async fn validate_invite(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {

    let token = path.into_inner();

    let result = InviteService::validate_invite(&pool, &token).await;

    match result {
        Ok(invite) => HttpResponse::Ok().json(invite),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err
        })),
    }
}

//
// ✅ USE INVITE (POST /invite/use/{token})
//
pub async fn use_invite(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {

    let token = path.into_inner();

    let result = InviteService::use_invite(&pool, &token).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Invite successfully used"
        })),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err
        })),
    }
}

//
// ❌ DELETE INVITE
//
pub async fn delete_invite(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {

    let id = path.into_inner();

    let result = InviteService::delete_invite(&pool, id).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Invite deleted"
        })),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err
        })),
    }
}