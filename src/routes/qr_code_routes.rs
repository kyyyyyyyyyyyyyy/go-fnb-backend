use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::controllers::qr_code_controller;
use crate::middlewares::auth_middleware::validator;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/api/qrcodes")
            // 🔐 protected routes
            .wrap(auth.clone())
            .route("", web::post().to(qr_code_controller::create_qr))
            .route("/auto", web::post().to(qr_code_controller::create_qr_with_tables))
            .route("/outlet/{outlet_id}", web::get().to(qr_code_controller::get_qr_by_outlet))
            .route("/{qr_id}/tables", web::get().to(qr_code_controller::get_qr_tables))
            .route("/{qr_id}/regenerate", web::patch().to(qr_code_controller::regenerate_qr_slug))
    );

    cfg.service(
        web::scope("/api/scan")
            // 🔓 public route
            .route(
                "/select-table" ,
                web::post().to(qr_code_controller::select_table)
            )

            .route(
                "/{slug}",
                web::get().to(qr_code_controller::scan_qr)
            )
    );
}