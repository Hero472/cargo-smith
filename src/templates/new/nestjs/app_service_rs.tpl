// {{name_pascal_case}} service.
//
// Generated on {{now}}.
// Provides business logic similar to a NestJS Injectable service.

pub struct AppService;

impl AppService {
    pub fn new() -> Self {
        AppService
    }

    pub fn get_status(&self) -> &'static str {
        "App is running"
    }
}