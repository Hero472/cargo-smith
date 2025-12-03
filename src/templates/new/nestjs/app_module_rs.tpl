// {{name_pascal_case}} module.
//
// Generated on {{now}}.
// This is the NestJS-style module for the "{{name}}" feature.

use actix_web::web;

use super::{AppService, app_controller};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    // Register Service (DI)
    cfg.app_data(web::Data::new(AppService::new()));

    // Register Routes
    cfg.configure(app_controller::routes);
}
