use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::controllers::table_controller;
use crate::middlewares::auth_middleware::validator;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/api/tables")
            .wrap(auth)
            
            // 🔥 CREATE
            .route("", web::post().to(table_controller::create_table))

            // 🔥 DELETE TOKEN
            .route("/{id}/token", web::delete().to(table_controller::delete_token))

            // 🔥 GET ALL BY OUTLET
            .route("/outlet/{outlet_id}", web::get().to(table_controller::get_tables_by_outlet))
            
            // 🔥 GET BY ID
            .route("/{id}", web::get().to(table_controller::get_table_by_id))
            
            // 🔥 UPDATE (PATCH)
            .route("/{id}", web::patch().to(table_controller::update_table))
            
            // 🔥 DELETE
            .route("/{id}", web::delete().to(table_controller::delete_table))
    );
}