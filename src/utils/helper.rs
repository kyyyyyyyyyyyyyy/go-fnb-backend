use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::middlewares::auth_middleware::AuthUser;
use crate::utils::app_error::AppError;

pub fn get_user_id(req: &HttpRequest) -> Result<Uuid, AppError> {
    req.extensions()
        .get::<AuthUser>()
        .map(|u| u.user_id)
        .ok_or(AppError::Unauthorized)
}

pub async fn ensure_user_has_outlet(
    req: &HttpRequest,
    pool: &PgPool,
    outlet_id: Uuid,
) -> Result<Uuid, AppError> {

    let user_id = req
        .extensions()
        .get::<AuthUser>()
        .map(|u| u.user_id)
        .ok_or(AppError::Unauthorized)?;

    let exists: bool =
        sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_outlets
                WHERE user_id = $1
                AND outlet_id = $2
            )
            "#
        )
        .bind(user_id)
        .bind(outlet_id)
        .fetch_one(pool)
        .await
        .map_err(
            |_| AppError::InternalServerError
        )?;

    if !exists {
        return Err(AppError::Unauthorized);
    }

    Ok(user_id)
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
