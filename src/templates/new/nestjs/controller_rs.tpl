//! {{name_pascal_case}} controller.
//!
//! Generated on {{now}}.
//! This acts like a NestJS Controller for the "{{name}}" module.

use actix_web::{get, post, web, HttpResponse};
use super::{{name_snake_case}}_service::{{name_pascal_case}}Service;

#[get("")]
async fn find_all(service: web::Data<{{name_pascal_case}}Service>) -> HttpResponse {
    let items = service.find_all();
    HttpResponse::Ok().json(items)
}

#[post("")]
async fn create(
    service: web::Data<{{name_pascal_case}}Service>,
    payload: web::Json<serde_json::Value>,
) -> HttpResponse {
    let item = service.create(payload.into_inner());
    HttpResponse::Created().json(item)
}

/// Registers all controller routes inside a scope.
/// 
/// Equivalent to:
/// @Controller("{{name}}")
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(find_all)
        .service(create);
}
