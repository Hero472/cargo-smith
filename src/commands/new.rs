use anyhow::Result;
use clap::Args;

use crate::templates::TemplateType;

#[derive(Args)]
pub struct NewArgs {
    pub project_name: String,
    pub template_type: TemplateType
}

pub async fn execute(args: NewArgs) -> Result<()> {
    println!("🚀 Creating new project: {}", args.project_name);

    TemplateType::create(&args.template_type)
        .generate(&args.project_name)
        .await?;

    println!("✅ Project '{}' created successfully!", args.project_name);
    println!("📂 Next steps:");
    println!("   cd {}", args.project_name);
    println!("   cargo run");

    Ok(())
}