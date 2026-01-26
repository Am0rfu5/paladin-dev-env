// Agent command implementations

use crate::cli::output::errors::CliError;
use crate::cli::templates::paladin_template::generate_paladin_template;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    /// Create a new Paladin configuration template
    New(AgentNewArgs),
    /// Run a Paladin from configuration
    Run(AgentRunArgs),
}

#[derive(Debug, clap::Args)]
pub struct AgentNewArgs {
    /// Name for the Paladin
    #[arg(short, long)]
    pub name: String,

    /// Output path for the template file
    #[arg(short, long)]
    pub output: PathBuf,

    /// LLM provider (openai, deepseek, anthropic)
    #[arg(short, long)]
    pub provider: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct AgentRunArgs {
    /// Path to Paladin YAML configuration file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Input text for the Paladin (prompts if not provided)
    #[arg(short, long)]
    pub input: Option<String>,

    /// Path to save output file (prints to stdout if not provided)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

/// Handle the `paladin agent new` command
///
/// Creates a new Paladin configuration template file with documented options
pub fn handle_agent_new(args: AgentNewArgs) -> Result<(), CliError> {
    // Validate and normalize provider
    let provider = args.provider.as_deref().unwrap_or("openai");
    let valid_providers = ["openai", "deepseek", "anthropic"];

    if !valid_providers.contains(&provider) {
        return Err(CliError::InvalidFieldValue {
            field: "provider".to_string(),
            message: format!(
                "must be one of: {}. Got: {}",
                valid_providers.join(", "),
                provider
            ),
        });
    }

    // Check if output file already exists
    if args.output.exists() {
        // For now, return error. Interactive confirmation will be added in Task 11.0
        return Err(CliError::FileAlreadyExists {
            path: args.output.clone(),
        });
    }

    // Generate template
    let template = generate_paladin_template(&args.name, provider);

    // Write to file
    std::fs::write(&args.output, template)?;

    // Print success message with colored output
    println!(
        "{} Created Paladin template: {}",
        "✓".green().bold(),
        args.output.display()
    );

    Ok(())
}
