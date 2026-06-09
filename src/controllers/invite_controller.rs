use std::env;

use actix_web::{web, Responder};
use serde_json::json;
use uuid::Uuid;
use sqlx::PgPool;

use crate::{dto::invite_dto::{AcceptInviteDTO, CreateInviteDTO}, services::invite_service::InviteService, utils::helper};


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
        Ok(invite) => {
            helper::createdWithDatas(
                "Invite created successfully",
                json!({
                    "url": format!(
                        "http://localhost:{}/api/invites/validate/{}",
                        port,
                        invite.token
                    )
                }),
            )
        }

        Err(err) => {
            helper::error_response(
                &err.to_string()
            )
        }
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
        Ok(invite) => helper::successWithDatas("Invite valid", invite),
        Err(err) => helper::error_response(&err.to_string()),
    }
}

//
// ✅ USE INVITE (POST /invite/use/{token})
//
pub async fn use_invite(
    pool: web::Data<PgPool>,
    body: web::Json<AcceptInviteDTO>,
) -> impl Responder {
    let result = InviteService::use_invite(&pool, &body.token, &body.name, &body.email, &body.password).await;

    match result {
        Ok(_) => helper::created("User berhasil dibuat"),
        Err(err) => helper::error_response(&err.to_string()),
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
        Ok(_) => helper::success("Invite deleted"),
        Err(err) => helper::error_response(&err.to_string()),
    }
}