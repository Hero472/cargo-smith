use anyhow::Result;
use clap::Args;

use crate::templates::TemplateType;

/// Arguments for the `new` command.
#[derive(Args)]
pub struct NewArgs {
    /// Name of the project to create.
    pub project_name: Option<String>,
    /// Template type to use (default: modular).
    pub template_type: Option<TemplateType>,
    /// Enable JWT authentication scaffolding (auth feature + JwtMiddleware).
    #[arg(long)]
    pub auth: bool,

    /// Use a path dependency to the local cargo-smith source instead of crates.io.
    #[arg(long)]
    pub local: bool,
}

/// Executes the `new` command, creating a project from the chosen template.
pub async fn execute(args: NewArgs) -> Result<()> {

    let project_name = args.project_name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Project name is required"))?;
        
    let template_type = args.template_type
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Template type is required"))?;

    println!("Creating new project: {}", project_name);

    TemplateType::create(template_type)
        .generate(project_name, args.auth, args.local)
        .await?;

    println!("Project '{}' created successfully!", project_name);
    println!("Next steps:");
    println!("   cd {}", project_name);
    println!("   cargo run");

    Ok(())
}