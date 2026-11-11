use actix_web::web;
use crate::features::auth::controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(controller::login))
    );
}