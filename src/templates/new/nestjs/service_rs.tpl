//! {{name_pascal_case}} service.
//!
//! Generated on {{now}}.
//! Provides business logic similar to a NestJS Injectable service.

pub struct {{name_pascal_case}}Service;

impl {{name_pascal_case}}Service {
    pub fn new() -> Self {
        {{name_pascal_case}}Service
    }

    pub fn find_all(&self) -> Vec<serde_json::Value> {
        vec![
            serde_json::json!({ "example": "item1" }),
            serde_json::json!({ "example": "item2" }),
        ]
    }

    pub fn create(&self, data: serde_json::Value) -> serde_json::Value {
        serde_json::json!({
            "created": true,
            "data": data
        })
    }
}
