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

                    .route(
                        "",
                        web::get()
                        .to(
                            order_controller::get_orders_by_outlet
                        )
                    )

                    .route(
                        "/{id}",
                        web::get()
                        .to(
                            order_controller::get_order_by_id
                        )
                    )

                    .route(
                        "/{id}",
                        web::patch()
                        .to(
                            order_controller::update_order
                        )
                    )

                    .route(
                        "/{id}/status",
                        web::patch()
                        .to(
                            order_controller::update_order_status
                        )
                    )

                    .route(
                        "/{id}",
                        web::delete()
                        .to(
                            order_controller::delete_order
                        )
                    )
            )
    );
}
