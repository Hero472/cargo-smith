use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct {{name_pascal_case}} {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    // Add your fields here, e.g.:
    // pub name: String,
    // pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Create{{name_pascal_case}} {
    // Add fields here with validation, e.g.:
    // #[validate(length(min = 1))]
    // pub name: String,
    // #[validate(email)]
    // pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Update{{name_pascal_case}} {
    // All fields optional for partial updates, e.g.:
    // pub name: Option<String>,
    // pub email: Option<String>,
}