//! Battalion command implementations

use clap::Subcommand;
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
