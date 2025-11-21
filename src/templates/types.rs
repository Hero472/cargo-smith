use serde::{Deserialize, Serialize};

// CargoMold

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoMold {
    pub project: Project,
    pub generated: Generated,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub mold_version: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generated {
    pub resources: Vec<String>,
    pub modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub template: String,
}

// CargoToml

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: Package,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub version: String,
}