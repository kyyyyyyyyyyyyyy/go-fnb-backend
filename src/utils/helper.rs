use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::middlewares::auth_middleware::AuthUser;
use crate::utils::app_error::AppError;

pub fn get_user_id(req: &HttpRequest) -> Result<Uuid, AppError> {
    req.extensions()
        .get::<AuthUser>()
        .map(|u| u.user_id)
        .ok_or(AppError::Unauthorized)
}

pub fn get_outlet_id(req: &HttpRequest) -> Result<Uuid, AppError> {
    req.extensions()
        .get::<AuthUser>()
        .and_then(|u| u.outlet_id)
        .ok_or(AppError::Unauthorized)
}

pub fn created(message: &str) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "message": message,
    }))
}

pub fn created_withDatas<T: serde::Serialize>(message: &str, data: T) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "message": message,
        "data": data
    }))
}

pub fn success_withDatas<T: serde::Serialize>(message: &str, data: T) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": message,
        "data": data
    }))
}

pub fn success(message: &str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": message,
    }))
}

pub fn error_response(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(serde_json::json!({
        "success": false,
        "message": message
    }))
}

pub fn not_found(message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "success": false,
        "message": message
    }))
}
