use actix_web::{web, Responder};
use cargo_smith::common::response::ApiResponse;
use cargo_smith::common::extractors::validated::ValidatedJson;
use crate::features::{{name}}::service;
use crate::features::{{name}}::model::{ {{name_pascal_case}}, Create{{name_pascal_case}}, Update{{name_pascal_case}} };

pub async fn get_all() -> impl actix_web::Responder {
    let data = service::get_all().await;
    ApiResponse::ok("{{name}} retrieved", data)
}

pub async fn get_by_id(path: web::Path<String>) -> impl actix_web::Responder {
    let id = path.into_inner();
    match service::get_by_id(&id).await {
        Some(item) => ApiResponse::ok(format!("{{name_pascal_case}} {} retrieved", id), item),
        None => ApiResponse::not_found(format!("{{name_pascal_case}} {} not found", id)),
    }
}

pub async fn create(body: ValidatedJson<Create{{name_pascal_case}}>) -> impl actix_web::Responder {
    let new_item = service::create(body.into_inner()).await;
    ApiResponse::created("{{name}} created", new_item)
}

pub async fn update(path: web::Path<String>, body: ValidatedJson<Update{{name_pascal_case}}>) -> impl actix_web::Responder {
    let id = path.into_inner();
    match service::update(&id, body.into_inner()).await {
        Some(item) => ApiResponse::ok(format!("{{name_pascal_case}} {} updated", id), item),
        None => ApiResponse::not_found(format!("{{name_pascal_case}} {} not found", id)),
    }
}

pub async fn delete(path: web::Path<String>) -> impl actix_web::Responder {
    let id = path.into_inner();
    if service::delete(&id).await {
        ApiResponse::<()>::no_content("{{name}} deleted")
    } else {
        ApiResponse::not_found(format!("{{name_pascal_case}} {} not found", id))
    }
}