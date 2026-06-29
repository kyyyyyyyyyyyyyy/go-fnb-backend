use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use uuid::Uuid;

use crate::utils::jwt;

#[derive(Clone, Copy)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub outlet_id: Option<Uuid>,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    match jwt::verify_token(token) {
        Some(claims) => {
            match Uuid::parse_str(&claims.sub) {
                Ok(user_id) => {
                    let outlet_id = claims
                        .outlet_id
                        .as_deref()
                        .and_then(|s| Uuid::parse_str(s).ok());

                    req.extensions_mut().insert(AuthUser { user_id, outlet_id });
                    Ok(req)
                }
                Err(_) => Err((
                    actix_web::error::ErrorUnauthorized("Invalid user id"),
                    req,
                )),
            }
        }
        None => Err((
            actix_web::error::ErrorUnauthorized("Invalid token"),
            req,
        )),
    }
}
