use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::controllers::invite_controller;
use crate::middlewares::auth_middleware::validator;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("api/invites")

            // 🔥 PUBLIC ROUTES (tidak perlu login)
            .route("/validate/{token}", web::get().to(invite_controller::validate_invite))
            .route("/use", web::post().to(invite_controller::use_invite))

            // 🔐 PROTECTED ROUTES (butuh auth)
            .service(
                web::scope("")
                    .wrap(auth.clone())
                    .route("", web::post().to(invite_controller::create_invite))
                    .route("/{id}", web::delete().to(invite_controller::delete_invite))
            )
    );
}