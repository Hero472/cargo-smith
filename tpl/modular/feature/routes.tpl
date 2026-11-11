use actix_web::web;
use crate::features::{{name}}::controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{{name}}")
            .route("", web::get().to(controller::get_all))
            .route("", web::post().to(controller::create))
            .route("/{id}", web::get().to(controller::get_by_id))
            .route("/{id}", web::put().to(controller::update))
            .route("/{id}", web::delete().to(controller::delete))
    );
}