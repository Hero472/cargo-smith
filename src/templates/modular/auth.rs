use crate::templates::{TemplateType, contents::modular, template_engine::TemplateEngine};
use anyhow::Result;
use std::path::Path;
use tokio::fs;

pub async fn run(project_name: &str) -> Result<()> {
    let base_path = format!("{}/src/features/auth", project_name);

    println!("Generating auth feature: auth");
    fs::create_dir_all(&base_path).await?;

    let files = [
        (format!("{}/mod.rs", base_path), modular::auth::MOD),
        (format!("{}/controller.rs", base_path), modular::auth::CONTROLLER),
        (format!("{}/service.rs", base_path), modular::auth::SERVICE),
        (format!("{}/model.rs", base_path), modular::auth::MODEL),
        (format!("{}/routes.rs", base_path), modular::auth::ROUTES),
    ];

    for (path, content) in files {
        TemplateEngine::generate_from_template(
            "auth",
            &path,
            content,
            &TemplateType::Modular,
            false, // path already includes project_name
        )
        .await?;
    }

    // Register in the parent module (project-scoped)
    register_in_parent_mod(project_name).await?;

    Ok(())
}

async fn register_in_parent_mod(project_name: &str) -> Result<()> {
    let mod_file = format!("{}/src/features/mod.rs", project_name);
    let mod_path = Path::new(&mod_file);

    let mut content = if mod_path.exists() {
        fs::read_to_string(&mod_file).await?
    } else {
        String::new()
    };

    if !content.contains("// [SMITH-MOD]") {
        content = format!(
            "use actix_web::web;\n\n// [SMITH-MOD]\n\npub fn init(cfg: &mut web::ServiceConfig) {{\n    // [SMITH-INIT]\n}}\n"
        );
    }

    if content.contains("_cfg: &mut web::ServiceConfig") {
        content = content.replace("_cfg: &mut web::ServiceConfig", "cfg: &mut web::ServiceConfig");
    }

    let mod_declaration = "pub mod auth;";
    let init_call = "    auth::init(cfg);";

    if !content.contains(mod_declaration) {
        content = content.replace("// [SMITH-MOD]", &format!("// [SMITH-MOD]\n{}", mod_declaration));
    }

    if !content.contains(init_call) {
        content = content.replace("// [SMITH-INIT]", &format!("// [SMITH-INIT]\n{}", init_call));
    }

    fs::write(&mod_file, content).await?;
    Ok(())
}