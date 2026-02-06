/// Paladin CLI - Command-line interface for Paladin multi-agent orchestration
use clap::{Parser, Subcommand};
use paladin::cli::commands::{
    agent::{AgentCommands, handle_agent_new, handle_agent_run},
    arsenal::{ArsenalCommands, handle_arsenal_command},
    battalion::{BattalionCommands, handle_battalion_new, handle_battalion_run},
    maneuver::{ManeuverCommands, handle_maneuver_command},
};
use std::process;
use tokio::signal;

#[derive(Parser)]
#[command(name = "paladin")]
#[command(version, about = "Paladin Multi-Agent Orchestration CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Paladin agent operations (create, run)
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },
    /// Battalion multi-agent operations (create, run)
    Battalion {
        #[command(subcommand)]
        action: BattalionCommands,
    },
    /// Arsenal tool management (list, test)
    Arsenal {
        #[command(subcommand)]
        action: ArsenalCommands,
    },
    /// Maneuver flow DSL operations (visualize, validate, execute)
    Maneuver {
        #[command(subcommand)]
        action: ManeuverCommands,
    },
}

#[tokio::main]
async fn main() {
    // Setup SIGINT handler for graceful shutdown (Ctrl+C)
    let _sigint_handler = tokio::spawn(async {
        if signal::ctrl_c().await.is_ok() {
            eprintln!("\n\nReceived interrupt signal (Ctrl+C). Exiting...");
            process::exit(130); // Standard SIGINT exit code
        }
    });

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Agent { action } => match action {
            AgentCommands::New(args) => handle_agent_new(args),
            AgentCommands::Run(args) => handle_agent_run(args).await,
        },
        Commands::Battalion { action } => match action {
            BattalionCommands::New(args) => handle_battalion_new(args),
            BattalionCommands::Run(args) => handle_battalion_run(args).await,
        },
        Commands::Arsenal { action } => handle_arsenal_command(action).await,
        Commands::Maneuver { action } => handle_maneuver_command(action).await,
    };

    // Handle errors and exit with appropriate code per FR-21
    // - 0: Success
    // - 1: User errors (config, validation, missing args)
    // - 2: Runtime errors (LLM, execution, tools)
    // - 130: SIGINT (handled by signal handler above)
    if let Err(e) = result {
        eprintln!("{}", e.format_detailed());
        process::exit(e.exit_code());
    }
}
