use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select, Input};

use crate::templates::TemplateType;

mod commands;
mod templates;
mod utils;

#[derive(Parser)]
#[command(name = "cargo-smith")]
#[command(about = "NestJS-inspired code generator for Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New,
    /// Generate code components (shortcut: g)
    #[command(name = "g")]
    Generate(GenerateArgs),
}

// Wrapper struct for generate subcommands
#[derive(Parser)]
pub struct GenerateArgs {
    #[command(subcommand)]
    command: GenerateCommands,
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate a resource module
    Resource(commands::resource::ResourceArgs),
    /// Generate a service
    Service(commands::service::ServiceArgs),
    /// Generate a controller  
    Controller(commands::controller::ControllerArgs),
    /// Generate a module
    Module(commands::module::ModuleArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    if cli.interactive || cli.command.is_none() {
        run_interactive().await
    } else {
        match cli.command.unwrap() {
            Commands::New => create_new_project_interactive().await,
            Commands::Generate(args) => match args.command {
                GenerateCommands::Resource(args) => commands::resource::execute(args).await,
                GenerateCommands::Service(args) => commands::service::execute(args).await,
                GenerateCommands::Controller(args) => commands::controller::execute(args).await,
                GenerateCommands::Module(args) => commands::module::execute(args).await,
            },
        }
    }
}

async fn run_interactive() -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    
    println!("Welcome to cargo-smith interactive mode!");
    println!("Let's generate some code step by step...\n");
    
    // Step 1: Choose action
    let actions = &["Create new project"];
    let action_choice = Select::with_theme(&theme)
        .with_prompt("What would you like to do?")
        .items(actions)
        .default(0)
        .interact()?;
    
    match action_choice {
        0 => create_new_project_interactive().await,
        _ => unreachable!(),
    }
}

async fn create_new_project_interactive() -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    
    println!("\nCreating a new project...");
    
    let project_name: String = Input::with_theme(&theme)
        .with_prompt("Project name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.is_empty() {
                Err("Project name cannot be empty")
            } else if input.contains(' ') {
                Err("Project name cannot contain spaces")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let templates = &[
        TemplateType::Traditional,
        // TemplateType::Nestjs,
        // "web-api",
        // "cli-tool",
        // "library",
        // "microservice"
        ];
    let template_choice = Select::with_theme(&theme)
        .with_prompt("Choose template type")
        .items(templates)
        .default(0)
        .interact()?;
    
    let template = templates[template_choice].clone();
    
    // // Step 3: Additional features
    // let features = &["database", "authentication", "logging", "testing"];
    // let feature_choices = dialoguer::MultiSelect::with_theme(&theme)
    //     .with_prompt("Select additional features (space to select, enter to confirm)")
    //     .items(features)
    //     .interact()?;
    
    // let selected_features: Vec<&str> = feature_choices.iter()
    //     .map(|&i| features[i])
    //     .collect();
    
    println!("\nSummary:");
    println!("  Project: {}", project_name);
    println!("  Template: {}", template);
    // println!("  Features: {}", selected_features.join(", "));
    
    // Step 4: Confirm
    let confirm = dialoguer::Confirm::with_theme(&theme)
        .with_prompt("Create project with these settings?")
        .default(true)
        .interact()?;
    println!();
    
    if confirm {
        let args = commands::new::NewArgs { 
            project_name: project_name,
            template_type: template,
        };
        commands::new::execute(args).await
    } else {
        println!("Project creation cancelled.");
        Ok(())
    }
}