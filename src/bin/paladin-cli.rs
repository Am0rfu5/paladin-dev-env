/// Paladin CLI - Command-line interface for Paladin multi-agent orchestration
use clap::{Parser, Subcommand};
use paladin::cli::commands::{
    agent::{AgentCommands, handle_agent_new, handle_agent_run},
    arsenal::ArsenalCommands,
    battalion::{BattalionCommands, handle_battalion_new},
};
use std::process;

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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Agent { action } => match action {
            AgentCommands::New(args) => handle_agent_new(args),
            AgentCommands::Run(args) => handle_agent_run(args).await,
        },
        Commands::Battalion { action } => match action {
            BattalionCommands::New(args) => handle_battalion_new(args),
            BattalionCommands::Run(args) => {
                println!("Battalion run command: {:?}", args);
                // TODO: implement in Task 7.0
                Ok(())
            }
        },
        Commands::Arsenal { action } => {
            println!("Arsenal command: {:?}", action);
            // TODO: implement arsenal command routing
            Ok(())
        }
    };

    // Handle errors and exit with appropriate code
    if let Err(e) = result {
        eprintln!("{}", e.format_detailed());
        process::exit(1);
    }
}
