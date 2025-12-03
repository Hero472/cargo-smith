use crate::{templates::{Template, TemplateType, template_engine::TemplateEngine}, utils::{create_project_structure, generate_mod_files}};

use anyhow::Result;
use async_trait::async_trait;

pub struct NestJSTemplate;

const CARGO_TOML_TPL: &str = include_str!("../../cargo_toml.tpl");
const MAIN_RS_TPL: &str = include_str!("../../main_rs.tpl");
const CARGO_SMITH_TPL: &str = include_str!("../../cargo_smith.tpl");
const ENV_EXAMPLE_TPL: &str = include_str!("../../env_example.tpl");
const LIB_RS_TPL: &str = include_str!("../nestjs/lib_rs.tpl");
const SERVER_SERVER_RS_TPL: &str = include_str!("./server_server_rs.tpl");
const APP_MODULE_RS_TPL: &str = include_str!("./app_module_rs.tpl");
const APP_SERVICE_RS_TPL: &str = include_str!("./app_service_rs.tpl");
const APP_CONTROLLER_RS_TPL: &str = include_str!("./app_controller_rs.tpl");

const APP_MOD_RS_TPL: &str = include_str!("../nestjs/mod_files/app_mod_rs.tpl");
const COMMON_MOD_RS_TPL: &str = include_str!("../nestjs/mod_files/common_mod_rs.tpl");
const CONFIG_MOD_RS_TPL: &str = include_str!("../nestjs/mod_files/config_mod_rs.tpl");
const MODULES_MOD_RS_TPL: &str = include_str!("../nestjs/mod_files/modules_mod_rs.tpl");
const EMPTY_MOD_RS_TPL: &str = include_str!("../../empty_mod_rs.tpl");

// app, common, config, modules

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
            (".cargo-smith", CARGO_SMITH_TPL ),
            ("env-example", ENV_EXAMPLE_TPL),
            ("src/server/server.rs", SERVER_SERVER_RS_TPL),

            // app
            ("src/app/app_module.rs", APP_MODULE_RS_TPL),
            ("src/app/app_controller.rs", APP_CONTROLLER_RS_TPL),
            ("src/app/app_service.rs", APP_SERVICE_RS_TPL),
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
            ("src/server/mod.rs", SERVER_MOD_RS_TPL),
            ("src/app/mod.rs", APP_MOD_RS_TPL),
            ("src/common/mod.rs", COMMON_MOD_RS_TPL),
            ("src/config/mod.rs", CONFIG_MOD_RS_TPL),
            ("src/modules/mod.rs", MODULES_MOD_RS_TPL),
            ("src/common/guards/mod.rs", EMPTY_MOD_RS_TPL),
            ("src/common/filters/mod.rs", EMPTY_MOD_RS_TPL),
            ("src/common/interceptors/mod.rs", EMPTY_MOD_RS_TPL),
            ("src/common/utils/mod.rs", EMPTY_MOD_RS_TPL)
        ];

        generate_mod_files(project_name, &mod_files, &TemplateType::Nestjs).await?;

        Ok(())
    }
}

// TODO: I need to do the env file and the settings file