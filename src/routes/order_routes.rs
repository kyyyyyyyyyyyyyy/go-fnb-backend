use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    controllers::order_controller,
    middlewares::auth_middleware::validator,
};

pub fn init(cfg: &mut web::ServiceConfig) {

    let auth =
        HttpAuthentication::bearer(
            validator
        );

    cfg.service(
        web::scope("/api/orders")

            // PUBLIC
            .route(
                "/consume",
                web::post()
                    .to(
                        order_controller::create_order
                    )
            )

            // PROTECTED
            .service(
                web::scope("")
                    .wrap(auth)

                    // .route(
                    //     "",
                    //     web::get()
                    //     .to(
                    //         order_controller::get_orders
                    //     )
                    // )

                    // .route(
                    //     "/{id}",
                    //     web::get()
                    //     .to(
                    //         order_controller::get_order
                    //     )
                    // )
            )
    );
}