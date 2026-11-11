use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct {{name_pascal_case}} {
    // Add fields here, e.g.:
    // #[validate(length(min = 1))]
    // pub name: String,
}

impl {{name_pascal_case}} {
    pub fn new() -> Self {
        Self {
            // Initialize fields
        }
    }
}