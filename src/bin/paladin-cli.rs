/// Paladin CLI - Command-line interface for Paladin multi-agent orchestration
use clap::{Parser, Subcommand};
use paladin::cli::commands::{
    agent::AgentCommands, arsenal::ArsenalCommands, battalion::BattalionCommands,
};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Agent { action } => {
            println!("Agent command: {:?}", action);
            // TODO: implement agent command routing
        }
        Commands::Battalion { action } => {
            println!("Battalion command: {:?}", action);
            // TODO: implement battalion command routing
        }
        Commands::Arsenal { action } => {
            println!("Arsenal command: {:?}", action);
            // TODO: implement arsenal command routing
        }
    }
}
