use crate::templates::{template_engine::TemplateEngine, Template, TemplateType};

use anyhow::Result;
use std::path::Path;
use tokio::fs;
use async_trait::async_trait;

pub struct TraditionalTemplate;

const CARGO_TOML_TPL: &str = include_str!("../traditional/cargo_toml.tpl");
const MAIN_RS_TPL: &str = include_str!("../traditional/main_rs.tpl");
const LIB_RS_TPL: &str = include_str!("../traditional/lib_rs.tpl");
const CARGO_MOLD_TPL: &str = include_str!("../../cargo_mold.tpl");
const ENV_EXAMPLE_TPL: &str = include_str!("../traditional/env_example.tpl");
const ROUTES_ROUTES_TPL: &str = include_str!("../traditional/routes_routes_rs.tpl");
const SERVER_SERVER_TPL: &str = include_str!("../traditional/server_server_rs.tpl");
const HANDLERS_HANDLERS_TPL: &str = include_str!("../traditional/handlers_handlers_rs.tpl");

const MODELS_MOD_RS_TPL: &str = include_str!("../traditional/mod_files/models_mod_rs.tpl");
const UTILS_MOD_RS_TPL: &str = include_str!("../traditional/mod_files/utils_mod_rs.tpl");
const HANDLERS_MOD_RS_TPL: &str = include_str!("../traditional/mod_files/handlers_mod_rs.tpl");
const ROUTES_MOD_RS_TPL: &str = include_str!("../traditional/mod_files/routes_mod_rs.tpl");
const SERVER_MOD_RS_TPL: &str = include_str!("../traditional/mod_files/server_mod_rs.tpl");

#[async_trait]
impl Template for TraditionalTemplate {

    fn name(&self) -> &str {
        "traditional"
    }

    fn description(&self) -> &str {
        "Traditional Rust/Actix Web structure with routes, handlers, and modules"
    }

    async fn generate(&self, project_name: &str) -> Result<()> {
        println!("🏗️  Using Traditional Rust template...");
        
        create_project_structure(project_name).await?;
        
        let files = [
                ("Cargo.toml", CARGO_TOML_TPL),
                ("src/main.rs", MAIN_RS_TPL),
                ("src/lib.rs", LIB_RS_TPL),
                (".cargo-mold", CARGO_MOLD_TPL ),
                ("env-example", ENV_EXAMPLE_TPL),
                ("src/routes/routes.rs", ROUTES_ROUTES_TPL),
                ("src/server/server.rs", SERVER_SERVER_TPL),
                ("src/handlers/handlers.rs", HANDLERS_HANDLERS_TPL)
            ];

        for (output_path, file_content) in files {
            TemplateEngine::generate_from_template(
                project_name,
                output_path,
                file_content,
                TemplateType::Traditional
            ).await?;
        }

        generate_mod_files(project_name).await?;

        Ok(())
    }
}

async fn create_project_structure(project_name: &str) -> Result<()> {
    let project_path = Path::new(project_name);
    fs::create_dir_all(project_path.join("src/routes")).await?;
    fs::create_dir_all(project_path.join("src/models")).await?;
    fs::create_dir_all(project_path.join("src/handlers")).await?;
    fs::create_dir_all(project_path.join("src/server")).await?;
    fs::create_dir_all(project_path.join("src/utils")).await?;
    Ok(())
}

async fn generate_mod_files(project_name: &str) -> Result<()> {
    let mod_files = [
        ("src/models/mod.rs", MODELS_MOD_RS_TPL),
        ("src/utils/mod.rs", UTILS_MOD_RS_TPL),
        ("src/handlers/mod.rs", HANDLERS_MOD_RS_TPL),
        ("src/routes/mod.rs", ROUTES_MOD_RS_TPL),
        ("src/server/mod.rs", SERVER_MOD_RS_TPL),
    ];

    for (output_path, file_content) in mod_files {
        TemplateEngine::generate_from_template(
            project_name,
            output_path,
            file_content,
            TemplateType::Traditional
        ).await?;
    }

    Ok(())
}
