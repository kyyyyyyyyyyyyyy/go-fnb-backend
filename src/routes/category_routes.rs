use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::controllers::category_controller;
use crate::middlewares::auth_middleware::validator;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("api/categories")

            // 🔐 PROTECTED ROUTES (butuh auth)
            .service(
                web::scope("")
                    .wrap(auth.clone())
                    .route("", web::post().to(category_controller::create_category))
                    .route("", web::get().to(category_controller::get_categories_by_outlet))
                    .route("/{category_id}",web::get().to(category_controller::get_category_by_id))
                    .route("/{category_id}",web::put().to(category_controller::update_category))
                    .route("/{category_id}",web::delete().to(category_controller::delete_category))
                )
    );
}