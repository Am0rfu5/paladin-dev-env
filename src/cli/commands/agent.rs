// Agent command implementations

use clap::Subcommand;
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
