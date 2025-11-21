use actix_web::{{web, HttpResponse}};
use crate::models::{{name}}::{{name_pascal_case}};

pub async fn create_{{name}}({{name}}_data: web::Json<{{name_pascal_case}}>) -> HttpResponse {
    HttpResponse::Created().json({{name}}_data)
}

pub async fn get_{{name}}() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn update_{{name}}(path: web::Path<String>, {{name}}_data: web::Json<{{name_pascal_case}}>) -> HttpResponse {
    HttpResponse::Ok().json({{name}}_data.clone())
}

pub async fn delete_{{name}}(path: web::Path<String>) -> HttpResponse {
    HttpResponse::NoContent().finish()
}