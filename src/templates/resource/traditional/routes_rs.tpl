use actix_web::web;
use crate::handlers::{{name}}_handlers;

// You need to add this into the routes.rs file to use
// Either public_routes or private_routes

pub fn {{name_snake_case}}_routes(cfg: &mut web::ServiceConfig) {{
    cfg.service(
        web::scope("/{}")
            .route("", web::get().to({{name_snake_case}}_handlers::get_{{name_snake_case}}))
            .route("", web::post().to({{name_snake_case}}_handlers::create_{{name_snake_case}}))
            .route("/{{id}}", web::get().to({{name_snake_case}}_handlers::get_{{name_snake_case}}))
            .route("/{{id}}", web::put().to({{name_snake_case}}_handlers::update_{{name_snake_case}}))
            .route("/{{id}}", web::delete().to({{name_snake_case}}_handlers::delete_{{name_snake_case}}))
    );
}}