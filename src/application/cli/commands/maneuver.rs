//! Maneuver command implementations for Flow DSL operations
//!
//! This module provides CLI commands for Maneuver-specific operations:
//! - **Visualize**: Display flow graph in ASCII or Mermaid format
//! - **Validate**: Validate flow expression and Paladin configuration
//! - **Execute**: Run a Maneuver workflow (alias to battalion run)
//!
//! # Examples
//!
//! ```bash
//! # Visualize a flow expression
//! paladin maneuver visualize -c maneuver.yaml --format ascii
//!
//! # Validate flow and configuration
//! paladin maneuver validate -c maneuver.yaml
//!
//! # Execute a maneuver
//! paladin maneuver execute -c maneuver.yaml -i "Process this input"
//! ```

use crate::application::cli::error::CliError;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

/// Maneuver subcommands for Flow DSL operations
#[derive(Debug, Subcommand)]
pub enum ManeuverCommands {
    /// Visualize a flow expression from config
    Visualize(ManeuverVisualizeArgs),
    /// Validate flow expression and configuration
    Validate(ManeuverValidateArgs),
}

/// Arguments for visualizing a flow
#[derive(Debug, clap::Args)]
pub struct ManeuverVisualizeArgs {
    /// Path to Maneuver YAML configuration file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Visualization format (ascii or mermaid)
    #[arg(short, long, default_value = "ascii")]
    pub format: String,

    /// Output to file instead of stdout
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

/// Arguments for validating a maneuver configuration
#[derive(Debug, clap::Args)]
pub struct ManeuverValidateArgs {
    /// Path to Maneuver YAML configuration file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Enable verbose validation output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Handle maneuver commands
pub async fn handle_maneuver_command(command: ManeuverCommands) -> Result<(), CliError> {
    match command {
        ManeuverCommands::Visualize(args) => handle_maneuver_visualize(args),
        ManeuverCommands::Validate(args) => handle_maneuver_validate(args).await,
    }
}

/// Handle the `paladin maneuver visualize` command
///
/// Displays the flow graph in the specified format (ASCII or Mermaid)
fn handle_maneuver_visualize(args: ManeuverVisualizeArgs) -> Result<(), CliError> {
    use crate::application::cli::config::battalion_config::BattalionYamlConfig;
    use crate::application::cli::config::loader::load_battalion_config;
    use crate::application::services::battalion::flow_visualizer::{
        FlowVisualizer, VisualizationFormat,
    };
    use crate::core::platform::container::battalion::parser::FlowParser;

    // Load configuration
    let battalion_config = load_battalion_config(&args.config)?;

    // Ensure it's a Maneuver config
    let maneuver_config = match battalion_config {
        BattalionYamlConfig::Maneuver(config) => config,
        _ => {
            return Err(CliError::ValidationError {
                message: format!(
                    "Expected maneuver config, but got: {}",
                    battalion_config.battalion_type()
                ),
            });
        }
    };

    // Parse flow expression
    let flow = FlowParser::parse(&maneuver_config.flow).map_err(|e| CliError::ValidationError {
        message: format!("Invalid flow expression '{}': {}", maneuver_config.flow, e),
    })?;

    // Determine visualization format
    let format = match args.format.to_lowercase().as_str() {
        "ascii" => VisualizationFormat::Ascii,
        "mermaid" => VisualizationFormat::Mermaid,
        _ => {
            return Err(CliError::InvalidFieldValue {
                field: "format".to_string(),
                message: format!("must be 'ascii' or 'mermaid', got: {}", args.format),
            });
        }
    };

    // Generate visualization
    let visualization = FlowVisualizer::visualize(&flow, format);

    // Output
    if let Some(output_path) = args.output {
        std::fs::write(&output_path, &visualization)?;
        println!(
            "{} Visualization written to: {}",
            "✓".green().bold(),
            output_path.display()
        );
    } else {
        println!(
            "\n{} Flow Visualization ({}):",
            "📊".cyan().bold(),
            args.format
        );
        println!("{}", "═".repeat(80));
        println!("{}", visualization);
        println!("{}", "═".repeat(80));
    }

    Ok(())
}

/// Handle the `paladin maneuver validate` command
///
/// Validates the flow expression syntax and configuration structure
async fn handle_maneuver_validate(args: ManeuverValidateArgs) -> Result<(), CliError> {
    use crate::application::cli::config::battalion_config::BattalionYamlConfig;
    use crate::application::cli::config::loader::load_battalion_config;
    use crate::core::platform::container::battalion::parser::FlowParser;

    if args.verbose {
        println!(
            "{} Validating Maneuver configuration: {}",
            "→".cyan().bold(),
            args.config.display()
        );
    }

    // Load and validate configuration
    let battalion_config = load_battalion_config(&args.config)?;

    // Ensure it's a Maneuver config
    let maneuver_config = match battalion_config {
        BattalionYamlConfig::Maneuver(config) => config,
        _ => {
            return Err(CliError::ValidationError {
                message: format!(
                    "Expected maneuver config, but got: {}",
                    battalion_config.battalion_type()
                ),
            });
        }
    };

    // Validate flow expression
    if args.verbose {
        println!(
            "{} Validating flow expression: {}",
            "→".cyan().bold(),
            maneuver_config.flow
        );
    }

    let flow = FlowParser::parse(&maneuver_config.flow).map_err(|e| CliError::ValidationError {
        message: format!("Invalid flow expression '{}': {}", maneuver_config.flow, e),
    })?;

    println!("{} Flow expression is valid", "✓".green().bold());

    // Extract agent names from flow
    let agent_names = extract_agent_names(&flow);
    if args.verbose {
        println!(
            "{} Agents referenced in flow: {}",
            "→".cyan().bold(),
            agent_names.join(", ")
        );
    }

    // Validate all referenced agents exist in paladins list
    let paladin_names: std::collections::HashSet<_> = maneuver_config
        .paladins
        .iter()
        .filter_map(|p| match p {
            crate::application::cli::config::battalion_config::PaladinReference::Inline(inline) => {
                Some(inline.name.clone())
            }
            crate::application::cli::config::battalion_config::PaladinReference::File {
                ..
            } => {
                // For file references, we'd need to load them to get the name
                // For now, we skip validation of file references
                None
            }
        })
        .collect();

    let mut missing_agents = Vec::new();
    for agent_name in &agent_names {
        if !paladin_names.contains(agent_name) {
            missing_agents.push(agent_name.clone());
        }
    }

    if !missing_agents.is_empty() {
        return Err(CliError::ValidationError {
            message: format!(
                "Flow references agents not found in configuration: {}",
                missing_agents.join(", ")
            ),
        });
    }

    println!(
        "{} All {} referenced agents found in configuration",
        "✓".green().bold(),
        agent_names.len()
    );

    // Validate paladin configuration structure
    if args.verbose {
        println!(
            "{} Validating {} Paladin configurations...",
            "→".cyan().bold(),
            maneuver_config.paladins.len()
        );
    }

    // The config was already validated during load, so if we got here, it's valid
    println!("{} Configuration is valid", "✓".green().bold());

    if args.verbose {
        println!("\n{} Validation Summary:", "→".cyan().bold());
        println!("   Config file: {}", args.config.display());
        println!("   Maneuver name: {}", maneuver_config.name);
        println!("   Flow: {}", maneuver_config.flow);
        println!("   Agents in flow: {}", agent_names.len());
        println!(
            "   Total Paladins defined: {}",
            maneuver_config.paladins.len()
        );
    }

    Ok(())
}

/// Extract all agent names from a FlowExpression
fn extract_agent_names(
    flow: &crate::core::platform::container::battalion::parser::FlowExpression,
) -> Vec<String> {
    use crate::core::platform::container::battalion::parser::FlowExpression;

    let mut names = Vec::new();
    match flow {
        FlowExpression::Agent(name) => names.push(name.clone()),
        FlowExpression::Sequential(exprs) | FlowExpression::Parallel(exprs) => {
            for expr in exprs {
                names.extend(extract_agent_names(expr));
            }
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::battalion::parser::FlowParser;

    #[test]
    fn test_extract_agent_names_simple() {
        let flow = FlowParser::parse("agent1").unwrap();
        let names = extract_agent_names(&flow);
        assert_eq!(names, vec!["agent1"]);
    }

    #[test]
    fn test_extract_agent_names_sequential() {
        let flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
        let names = extract_agent_names(&flow);
        assert_eq!(names, vec!["agent1", "agent2", "agent3"]);
    }

    #[test]
    fn test_extract_agent_names_parallel() {
        let flow = FlowParser::parse("(agent1, agent2)").unwrap();
        let names = extract_agent_names(&flow);
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"agent1".to_string()));
        assert!(names.contains(&"agent2".to_string()));
    }

    #[test]
    fn test_extract_agent_names_nested() {
        let flow = FlowParser::parse("a -> (b, c) -> d").unwrap();
        let names = extract_agent_names(&flow);
        assert_eq!(names.len(), 4);
        assert_eq!(names[0], "a");
        assert_eq!(names[3], "d");
    }
}
