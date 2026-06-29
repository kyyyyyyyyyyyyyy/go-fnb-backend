use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::controllers::outlet_controller;
use crate::middlewares::auth_middleware::validator;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("api/outlets")
            .wrap(auth) // 🔥 middleware dipasang di sini
            .route("", web::post().to(outlet_controller::create_outlet))
            // .route("", web::get().to(outlet_controller::get_all_outlets))
            .route("/", web::get().to(outlet_controller::get_my_outlets))
            .route("/{id}", web::get().to(outlet_controller::get_outlet_by_id))
            .route("/{id}", web::patch().to(outlet_controller::update_outlet))
            .route("/{id}", web::delete().to(outlet_controller::delete_outlet))
    );
}
