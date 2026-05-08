use actix_web::{post, get, web, HttpResponse, Responder};
use sqlx::PgPool;
use serde_json::json;

use crate::{
    dto::auth_dto::{RegisterDTO, LoginDTO, CallbackQuery},
    services::auth_service,
};

#[post("/register")]
pub async fn register(
    db: web::Data<PgPool>,
    body: web::Json<RegisterDTO>,
) -> impl Responder {

    match auth_service::register(
        &db,
        body.name.clone(),
        body.email.clone(),
        body.password.clone(),
    ).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Register berhasil"
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": e
        }))
    }
}

#[post("/login")]
pub async fn login(
    db: web::Data<PgPool>,
    body: web::Json<LoginDTO>,
) -> impl Responder {

    match auth_service::login(
        &db,
        body.email.clone(),
        body.password.clone(),
    ).await {
        Ok(token) => HttpResponse::Ok().json(json!({
            "success": true,
            "token": token
        })),
        Err(e) => HttpResponse::Unauthorized().json(json!({
            "success": false,
            "message": e
        }))
    }
}

#[get("/auth/google/callback")]
pub async fn google_callback(
    db: web::Data<PgPool>,
    query: web::Query<CallbackQuery>,
) -> impl Responder {

    match auth_service::handle_google_callback(&db, &query.code).await {
        Ok(token) => HttpResponse::Ok().json(json!({
            "success": true,
            "token": token
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": e
        }))
    }
}
