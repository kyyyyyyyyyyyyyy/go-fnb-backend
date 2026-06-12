use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::controllers::product_controller;
use crate::middlewares::auth_middleware::validator;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("api/products")

            // 🔐 PROTECTED ROUTES (butuh auth)
            .service(
                web::scope("")
                    .wrap(auth.clone())
                    .route("", web::post().to(product_controller::create_product))
                    .route("/outlet/{outlet_id}", web::get().to(product_controller::get_products_by_outlet))
                    .route("/{outlet_id}/{product_id}",web::get().to(product_controller::get_product_by_id))
                    .route("/{outlet_id}/{product_id}",web::patch().to(product_controller::update_product))
                    .route("/{outlet_id}/{product_id}",web::delete().to(product_controller::delete_product))
                )
    );
}