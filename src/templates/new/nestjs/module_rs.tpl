//! {{name_pascal_case}} module.
//!
//! Generated on {{now}}.
//! This is the NestJS-style module for the "{{name}}" feature.

use actix_web::{web};

use super::{
    {{name_snake_case}}_controller,
    {{name_snake_case}}_service::{{name_pascal_case}}Service,
};

pub struct {{name_pascal_case}}Module;

impl {{name_pascal_case}}Module {
    pub fn register(cfg: &mut web::ServiceConfig) {
        cfg
            // Provide the service as application data (similar to NestJS providers)
            .app_data(web::Data::new({{name_pascal_case}}Service::new()))

            // Register controller routes
            .configure({{name_snake_case}}_controller::routes);
    }
}
