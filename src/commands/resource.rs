use clap::Args;
use anyhow::{bail, Result};
use std::path::Path;
use tokio::fs;

use crate::templates::{template_engine::TemplateEngine, types::CargoMold};

#[derive(Args)]
pub struct ResourceArgs {
    pub name: String,
}

const MODEL_RS_TPL: &str = include_str!("../templates/resource/traditional/model_rs.tpl");
const HANDLER_RS_TPL: &str = include_str!("../templates/resource/traditional/handler_rs.tpl");
const ROUTES_RS_TPL: &str = include_str!("../templates/resource/traditional/routes_rs.tpl");

pub async fn execute(args: ResourceArgs) -> anyhow::Result<()> {

    if !Path::new(".cargo-mold").exists() {
        bail!(
            "Not a cargo-mold project.\n\
             Run this command in a project created with `cargo mold new`\n\
             Or create a new project with: `cargo mold new {}`",
            args.name
        );
    }

    println!("Generating resource: {}", args.name);

    let files = [
        ("src/models/{}.rs", MODEL_RS_TPL),
        ("src/handlers/{}_handlers.rs", HANDLER_RS_TPL),
        ("src/routes/{}_routes.rs", ROUTES_RS_TPL),
    ];

    for (output_path_pattern, template_content) in files {
        let output_path = output_path_pattern.replace("{}", &args.name);
        TemplateEngine::generate_resource_template(&args.name, &output_path, template_content).await?;
    }

    update_modules(&args.name).await?;
    update_resource_cargo_mold(args.name.clone()).await?;
    
    println!("Resource '{}' created successfully!", args.name);
    println!("Generated files:");
    println!("   - src/models/{}.rs", args.name);
    println!("   - src/handlers/{}_handlers.rs", args.name);
    println!("   - src/routes/{}_routes.rs", args.name);

    Ok(())
}

async fn update_modules(resource_name: &str) -> Result<()> {
    // Update mod files
    update_mod_file("src/models/mod.rs", &format!("pub mod {};", resource_name)).await?;
    update_mod_file("src/handlers/mod.rs", &format!("pub mod {}_handlers;", resource_name)).await?;
    update_mod_file("src/routes/mod.rs", &format!("pub mod {}_routes;", resource_name)).await?;
    
    Ok(())
}

async fn update_mod_file(file_path: &str, module_declaration: &str) -> Result<()> {
    if !Path::new(file_path).exists() {
        return Ok(());
    }
    
    let mut content = fs::read_to_string(file_path).await?;
    
    if !content.contains(module_declaration) {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(module_declaration);
        content.push('\n');
        
        fs::write(file_path, content).await?;
    }
    
    Ok(())
}

async fn update_resource_cargo_mold(name: String) -> Result<()> {
    
    let content = fs::read_to_string(".cargo-mold").await?;

    let mut config: CargoMold = toml::from_str(&content)?;

    if !config.generated.resources.contains(&name) {
        config.generated.resources.push(name);
        let updated_content = toml::to_string_pretty(&config)?;
        fs::write(".cargo-mold", updated_content).await?;
    }
    Ok(())
} 

// Someday I will do some routing framework os something to make it really well and useful