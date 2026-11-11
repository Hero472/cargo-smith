use crate::templates::{TemplateType, contents::{common, modular}, template_engine::TemplateEngine};
use crate::templates::modular::auth as auth_gen;
use anyhow::Result;

pub async fn run(project_name: &str, with_auth: bool, local: bool) -> Result<()> {
    println!("Generating Modular Rust Template...");

    let folders = ["src/features", "src/server", "src/utils"];
    TemplateEngine::create_project_structure(project_name, &folders).await?;

    if local {
        // Write Cargo.toml pointing at the local cargo-smith source so the
        // generated project builds against the in-development crate.
        let template = include_str!("../../../tpl/common/cargo_toml_local.tpl");
        let cargo_smith_path = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
        let content = template
            .replace("{{name}}", project_name)
            .replace("{{name_snake_case}}", &project_name.replace("-", "_"))
            .replace("{{name_pascal_case}}", &crate::utils::to_pascal_case(project_name))
            .replace("{{cargo_smith_path}}", &cargo_smith_path);
        let cargo_toml_path = format!("{}/Cargo.toml", project_name);
        tokio::fs::write(&cargo_toml_path, content).await?;

        // Still generate the other common files (main.rs, .cargo-smith, .env.example)
        let other_files = [
            ("src/main.rs", modular::MAIN),
            (".cargo-smith", common::CARGO_SMITH),
            (".env.example", common::ENV_EXAMPLE),
        ];
        for (output_path, file_content) in other_files {
            TemplateEngine::generate_from_template(
                project_name,
                output_path,
                file_content,
                &TemplateType::Modular,
                true,
            )
            .await?;
        }
    } else {
        TemplateEngine::generate_common_files(project_name, &TemplateType::Modular).await?;
    }

    // Select server template based on auth option
    let server_tpl = if with_auth {
        modular::SERVER_AUTH
    } else {
        modular::SERVER
    };

    let files = [
        ("src/main.rs", modular::MAIN),
        ("src/lib.rs", modular::LIB),
        ("src/server/server.rs", server_tpl),
    ];

    for (output_path, file_content) in files {
        TemplateEngine::generate_from_template(
            project_name,
            output_path,
            file_content,
            &TemplateType::Modular,
            true,
        )
        .await?;
    }

    let mod_files = [
        ("src/features/mod.rs", modular::FEATURE_MOD),
        ("src/server/mod.rs", "//feature server entry\npub mod server;"),
        ("src/utils/mod.rs", "// feature utils entry"),
    ];

    TemplateEngine::generate_mod_files(project_name, &mod_files, &TemplateType::Modular).await?;

    // Optionally generate the auth feature and register it
    if with_auth {
        auth_gen::run(project_name).await?;

        // Update .cargo-smith to list the auth feature
        let cargo_smith_path = format!("{}/.cargo-smith", project_name);
        let cargo_smith_content = format!(
            "# .cargo-smith\n[project]\nname = \"{}\"\nsmith_version = \"0.4.0\"\ncreated_at = \"{}\"\n\n[generated]\nfeatures = [\"auth\"]\n\n[metadata]\ntemplate = \"Modular (Actix-Web)\"\n",
            project_name,
            chrono::Utc::now().format("%Y-%m-%d")
        );
        tokio::fs::write(&cargo_smith_path, cargo_smith_content).await?;
    }

    Ok(())
}