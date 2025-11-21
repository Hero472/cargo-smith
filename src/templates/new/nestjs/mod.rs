use anyhow::Result;
use async_trait::async_trait;

use crate::templates::Template;

pub struct NestJSTemplate;

#[async_trait]
impl Template for NestJSTemplate {
    fn name(&self) -> &str {
        "nestjs"
    }

    fn description(&self) -> &str {
        "NestJS-inspired structure with controllers, services, and modules"
    }

    async fn generate(&self, project_name: &str) -> Result<()> {
        todo!()
    }
}