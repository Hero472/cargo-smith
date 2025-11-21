use core::fmt;

use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;

pub mod new;
pub mod template_engine;
pub mod types;

use crate::templates::new::{traditional, nestjs};

#[async_trait]
pub trait Template {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn generate(&self, project_name: &str) -> Result<()>;
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TemplateType {
    Traditional,
    Nestjs,
}

impl fmt::Display for TemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateType::Traditional => write!(f, "traditional"),
            TemplateType::Nestjs => write!(f, "nestjs"),
        }
    }
}

impl TemplateType {
    pub fn create(&self) -> Box<dyn Template> {
        match self {
            TemplateType::Traditional => Box::new(traditional::TraditionalTemplate),
            TemplateType::Nestjs => Box::new(nestjs::NestJSTemplate),
        }
    }
    
    pub fn list_available() -> Vec<(&'static str, &'static str)> {
        vec![
            ("traditional", "Traditional Rust/Actix structure (default)"),
            ("nestjs", "NestJS-inspired modular structure"),
        ]
    }
}