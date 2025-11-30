use crate::templates::{template_engine::TemplateEngine, Template, TemplateType};

use anyhow::Result;
use std::path::Path;
use tokio::fs;
use async_trait::async_trait;

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
        println!("Using Nestjs Rust Template");

        create_project_structure(project_name).await?;

        let files = [("some", "some")];

        for (output_path, file_content) in files {
            TemplateEngine::generate_from_template(
                project_name,
                output_path,
                file_content,
                &TemplateType::Nestjs
            ).await?;
        }

        generate_mod_files(project_name).await?;

        Ok(())
    }
}

async fn create_project_structure(project_name: &str) -> Result<()> {
    todo!()
}

async fn generate_mod_files(project_name: &str) -> Result<()> {
    todo!()
}