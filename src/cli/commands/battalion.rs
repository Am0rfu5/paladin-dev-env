//! Battalion command implementations

use crate::cli::output::errors::CliError;
use crate::cli::templates::battalion_template::generate_battalion_template;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum BattalionCommands {
    /// Create a new Battalion configuration template
    New(BattalionNewArgs),
    /// Run a Battalion workflow
    Run(BattalionRunArgs),
}

#[derive(Debug, clap::Args)]
pub struct BattalionNewArgs {
    /// Name for the Battalion
    #[arg(short, long)]
    pub name: String,

    /// Battalion type (formation, phalanx, campaign, chain-of-command)
    #[arg(short, long)]
    pub r#type: String,

    /// Output path for the template file
    #[arg(short, long)]
    pub output: PathBuf,
}

#[derive(Debug, clap::Args)]
pub struct BattalionRunArgs {
    /// Path to Battalion YAML configuration file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Battalion type (formation, phalanx, campaign, chain-of-command)
    #[arg(short, long)]
    pub r#type: String,

    /// Path to save output file (prints to stdout if not provided)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

/// Handle the `paladin battalion new` command
///
/// Creates a new Battalion configuration template file with documented options
pub fn handle_battalion_new(args: BattalionNewArgs) -> Result<(), CliError> {
    // Validate battalion type
    let valid_types = ["formation", "phalanx", "campaign", "chain-of-command"];
    if !valid_types.contains(&args.r#type.as_str()) {
        return Err(CliError::InvalidFieldValue {
            field: "type".to_string(),
            message: format!(
                "must be one of: {}. Got: {}",
                valid_types.join(", "),
                args.r#type
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
    let template = generate_battalion_template(&args.name, &args.r#type)?;

    // Write to file
    std::fs::write(&args.output, template)?;

    // Print success message with colored output
    println!(
        "{} Created Battalion template: {}",
        "✓".green().bold(),
        args.output.display()
    );

    Ok(())
}
