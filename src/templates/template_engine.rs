use anyhow::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use chrono::Utc;

use crate::templates::types::CargoToml;
use crate::templates::TemplateType;

const CARGO_TOML: &str = include_str!("../../Cargo.toml");

pub struct TemplateEngine;

impl TemplateEngine {

    pub async fn generate_from_template(
        name: &str,
        output_path: &str,
        template_content: &str,
        template_type: &TemplateType
    ) -> Result<()> {

        let config: CargoToml = toml::from_str(CARGO_TOML)?;

        let content = template_content
            .replace("{{name}}", name)
            .replace("{{name_snake_case}}", &name.replace("-", "_"))
            .replace("{{name_pascal_case}}", &to_pascal_case(&name))
            .replace("{{now}}",  &Utc::now().format("%Y-%m-%d").to_string())
            .replace("{{template_type}}", &template_type.to_string())
            .replace("{{cargo_version}}", &config.package.version);

        let full_output_path = format!("{}/{}", name, output_path);
        
        if let Some(parent) = Path::new(&full_output_path).parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let mut file = fs::File::create(full_output_path).await?;
        file.write_all(content.as_bytes()).await?;
        
        Ok(())
    }

    pub async fn generate_resource_template(
        name: &str,
        output_path: &str,
        template_content: &str,
    ) -> Result<()> {
        let content = template_content
            .replace("{{name}}", name)
            .replace("{{name_snake_case}}", &name.replace("-", "_"))
            .replace("{{name_pascal_case}}", &to_pascal_case(&name));

        // Use output_path directly without prefixing
        let full_output_path = output_path.to_string();
        
        if let Some(parent) = Path::new(&full_output_path).parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let mut file = fs::File::create(full_output_path).await?;
        file.write_all(content.as_bytes()).await?;
        
        Ok(())
    }

}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
