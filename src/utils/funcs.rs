use std::{fs, path::{Path, PathBuf}};

use anyhow::Result;

use crate::templates::{TemplateType, template_engine::TemplateEngine};

pub fn find_matching_parenthesis(content: &str, start_pos: usize) -> Option<usize> {
    let mut count = 1;
    let chars: Vec<char> = content[start_pos..].chars().collect();
    
    for (i, c) in chars.iter().enumerate().skip(1) {
        match c {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count == 0 {
                    return Some(start_pos + i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Create a project folder structure based on a list of directory paths.
///
/// Example:
/// create_project_structure("myapp", &["src/routes", "src/models"]);
pub fn create_project_structure(
    project_name: &str,
    dirs: &[&str],
) -> Result<()> {
    let base = Path::new(project_name);

    for dir in dirs {
        let full_path: PathBuf = base.join(dir);
        fs::create_dir_all(&full_path)?;
    }

    Ok(())
}

// Create the mod generated files in the
pub async fn generate_mod_files(
    project_name: &str,
    files: &[(&str, &str)],
    template: &TemplateType
) -> Result<()> {
    for (path, content) in files {
        TemplateEngine::generate_from_template(
            project_name,
            path,
            content,
            template,
        ).await?;
    }
    Ok(())
}