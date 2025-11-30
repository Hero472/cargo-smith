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

pub fn to_pascal_case(s: &str) -> String {
    s.split_whitespace()
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

pub fn to_camel_case(s: &str) -> String {
    let mut words = s.split_whitespace()
        .filter(|w| !w.is_empty());

    match words.next() {
        Some(first) => {
            let mut result = first.to_lowercase();
            for w in words {
                let mut c = w.chars();
                if let Some(first_char) = c.next() {
                    result.push_str(
                        &(first_char.to_uppercase().collect::<String>() + c.as_str())
                    );
                }
            }
            result
        }
        None => String::new(),
    }
}