use actix_web::web;

pub mod health_routes;
pub mod auth_routes;
pub mod outlet_routes;
pub mod qr_code_routes;
pub mod table_routes;
pub mod category_routes;
pub mod invite_routes;
pub mod product_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.configure(health_routes::init);
    cfg.configure(auth_routes::init);
    cfg.configure(outlet_routes::init);
    cfg.configure(qr_code_routes::init);
    cfg.configure(table_routes::init);
    cfg.configure(category_routes::init);
    cfg.configure(invite_routes::init);
    cfg.configure(product_routes::init);
}