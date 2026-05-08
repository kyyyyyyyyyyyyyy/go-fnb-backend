use actix_web::web;
use crate::controllers::auth_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(auth_controller::register);
    cfg.service(auth_controller::login);
    cfg.service(auth_controller::google_callback);
}
