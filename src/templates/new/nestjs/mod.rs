use crate::{templates::{Template, TemplateType, template_engine::TemplateEngine}, utils::utils::{create_project_structure, generate_mod_files}};

use anyhow::Result;
use async_trait::async_trait;

pub struct NestJSTemplate;

const CARGO_TOML_TPL: &str = include_str!("../../cargo_toml.tpl");
const MAIN_RS_TPL: &str = include_str!("../../main_rs.tpl");
const CARGO_MOLD_TPL: &str = include_str!("../../cargo_mold.tpl");
const ENV_EXAMPLE_TPL: &str = include_str!("../../env_example.tpl");
const LIB_RS_TPL: &str = include_str!("../nestjs/lib_rs.tpl");

const SERVER_MOD_RS_TPL: &str = include_str!("../nestjs/mod_files/server_mod_rs.tpl");

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

        let folders = [
            "src/server",
            "src/modules",
            "src/app",
            "src/common",
            "src/common/guards",
            "src/common/filters",
            "src/common/interceptors",
            "src/common/utils",
            "src/config"
        ];

        create_project_structure(project_name, &folders)?;

        let files = [
            ("Cargo.toml", CARGO_TOML_TPL),
            ("src/main.rs", MAIN_RS_TPL),
            ("src/lib.rs", LIB_RS_TPL),
            (".cargo-mold", CARGO_MOLD_TPL ),
            ("env-example", ENV_EXAMPLE_TPL),
        ];

        for (output_path, file_content) in files {
            TemplateEngine::generate_from_template(
                project_name,
                output_path,
                file_content,
                &TemplateType::Nestjs
            ).await?;
        }

        let mod_files = [
            ("src/server/mod.rs", SERVER_MOD_RS_TPL)
        ];

        generate_mod_files(project_name, &mod_files, &TemplateType::Nestjs).await?;

        Ok(())
    }
}