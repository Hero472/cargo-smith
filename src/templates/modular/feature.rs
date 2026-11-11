use crate::templates::{TemplateType, contents::modular, template_engine::TemplateEngine};
use anyhow::{Result, bail};
use std::path::Path;
use tokio::fs;

/// Returns `true` when `name` is a safe feature identifier.
///
/// A valid name starts with an ASCII lowercase letter and contains only
/// lowercase letters, digits, underscores and hyphens. This prevents path
/// traversal and keeps generated module/file names valid Rust identifiers.
pub fn is_valid_feature_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => return false,
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

pub async fn run(name: &str) -> Result<()> {
    let feature_name = name.to_lowercase();

    if !is_valid_feature_name(&feature_name) {
        bail!(
            "invalid feature name '{}': use lowercase letters, digits, '_' or '-'",
            name
        );
    }

    let base_path = format!("src/features/{}", feature_name);

    // 1. Validation
    if Path::new(&base_path).exists() {
        bail!("Folder '{}' already exists. Manual cleanup required.", base_path);
    }

    println!("Generating feature: {}", feature_name);
    fs::create_dir_all(&base_path).await?;

    // 2. File Generation
    let files = [
        (format!("{}/mod.rs", base_path), modular::feature::MOD),
        (format!("{}/controller.rs", base_path), modular::feature::CONTROLLER),
        (format!("{}/service.rs", base_path), modular::feature::SERVICE),
        (format!("{}/model.rs", base_path), modular::feature::MODEL),
        (format!("{}/routes.rs", base_path), modular::feature::ROUTES),
    ];

    for (path, content) in files {
        TemplateEngine::generate_from_template(
            &feature_name, 
            &path,
            content,
            &TemplateType::Modular,
            false
        ).await?;
    }

    // 3. Register in the parent module
    register_in_parent_mod(&feature_name).await?;

    Ok(())
}

async fn register_in_parent_mod(name: &str) -> Result<()> {
    let mod_file = "src/features/mod.rs";
    let mod_path = Path::new(mod_file);

    let mut content = if mod_path.exists() {
        fs::read_to_string(mod_file).await?
    } else {
        String::new()
    };

    // Ensure markers exist for injection
    if !content.contains("// [SMITH-MOD]") {
        content = "use actix_web::web;\n\n// [SMITH-MOD]\n\npub fn init(cfg: &mut web::ServiceConfig) {\n    // [SMITH-INIT]\n}\n"
            .to_string();
    }

    if content.contains("_cfg: &mut web::ServiceConfig") {
        content = content.replace("_cfg: &mut web::ServiceConfig", "cfg: &mut web::ServiceConfig");
    }

    let mod_declaration = format!("pub mod {};", name);
    let init_call = format!("    {}::init(cfg);", name);

    if !content.contains(&mod_declaration) {
        content = content.replace("// [SMITH-MOD]", &format!("// [SMITH-MOD]\n{}", mod_declaration));
    }

    if !content.contains(&init_call) {
        content = content.replace("// [SMITH-INIT]", &format!("// [SMITH-INIT]\n{}", init_call));
    }

    fs::write(mod_file, content).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_feature_names() {
        assert!(is_valid_feature_name("users"));
        assert!(is_valid_feature_name("user_profile"));
        assert!(is_valid_feature_name("user-profile"));
        assert!(is_valid_feature_name("u2"));
    }

    #[test]
    fn invalid_feature_names() {
        assert!(!is_valid_feature_name(""));
        assert!(!is_valid_feature_name("Users"));
        assert!(!is_valid_feature_name("1users"));
        assert!(!is_valid_feature_name("../etc"));
        assert!(!is_valid_feature_name("a/b"));
        assert!(!is_valid_feature_name("a\\b"));
        assert!(!is_valid_feature_name("-foo"));
        assert!(!is_valid_feature_name("_foo"));
    }
}