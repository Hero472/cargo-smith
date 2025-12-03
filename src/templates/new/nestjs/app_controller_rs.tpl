// {{name_pascal_case}} controller.
//
// Generated on {{now}}.
// This acts like a NestJS Controller for the "{{name}}" module.

use actix_web::{get, web, HttpResponse};
use super::AppService;

#[get("/status")]
pub async fn get_status(service: web::Data<AppService>) -> HttpResponse {
    let status = service.get_status();
    HttpResponse::Ok().body(status)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status);
}
