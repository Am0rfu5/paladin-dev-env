//! Armory CLI - Command-line interface for Paladin management
//!
//! This module provides a comprehensive CLI for creating, configuring, and executing
//! Paladin agents and Battalion multi-agent workflows without writing code.
//!
//! # Features
//!
//! - **Agent Commands**: Create and run Paladin agents from YAML configs
//! - **Battalion Commands**: Orchestrate multi-agent workflows (Formation, Phalanx, Campaign, Chain of Command)
//! - **Arsenal Commands**: Discover and test MCP tools
//! - **Interactive Mode**: Prompts for missing configuration
//! - **Multiple Output Formats**: JSON, Markdown, or file output
//! - **Template Generation**: Quick-start YAML templates
//!
//! # Command Structure
//!
//! ```bash
//! paladin <command> <subcommand> [options]
//! ```
//!
//! ## Agent Commands
//!
//! ```bash
//! # Create agent template
//! paladin agent new <name> --output <path> [--provider <provider>]
//!
//! # Run agent
//! paladin agent run --config <path> [--input <text>] [--output <path>]
//! ```
//!
//! ## Battalion Commands
//!
//! ```bash
//! # Create battalion template
//! paladin battalion new <name> --type <type> --output <path>
//!
//! # Run battalion
//! paladin battalion run --config <path> --type <type> [--output <path>]
//! ```
//!
//! ## Arsenal Commands
//!
//! ```bash
//! # List available tools
//! paladin arsenal list
//!
//! # Test MCP server
//! paladin arsenal test --mcp-stdio <command> | --mcp-sse <url>
//! ```
//!
//! # See Also
//!
//! - [CLI Usage Guide](../../docs/CLI_USAGE.md) - Comprehensive documentation
//! - [Examples](../../examples/cli_configs/) - Sample configuration files
//!
//! # Epic 9: Armory CLI Tools
//!
//! This module implements the complete CLI tooling for Paladin, enabling
//! command-line creation and execution of agents and multi-agent workflows.

pub mod user_commands;

// Armory CLI modules (Epic 9)
pub mod commands;
pub mod config;
pub mod interactive;
pub mod output;
pub mod templates;

/*
CLI Module Tests

Tests for the CLI user commands module to ensure proper integration
with the User service and correct command line argument parsing.
*/

#[cfg(test)]
mod cli_tests {
    use crate::cli::user_commands::UserCommands;
    use clap::Parser;

    #[derive(Parser)]
    #[command(name = "test")]
    struct TestCli {
        #[command(subcommand)]
        user: Option<UserCommands>,
    }

    #[test]
    fn test_register_command_parsing() {
        let args = vec![
            "test",
            "register",
            "--username",
            "testuser",
            "--email",
            "test@example.com",
            "--password",
            "securepassword",
            "--first-name",
            "Test",
            "--last-name",
            "User",
            "--bio",
            "Test bio",
        ];

        let cli = TestCli::try_parse_from(args).unwrap();

        if let Some(UserCommands::Register(register_args)) = cli.user {
            assert_eq!(register_args.username, "testuser");
            assert_eq!(register_args.email, "test@example.com");
            assert_eq!(register_args.password, "securepassword");
            assert_eq!(register_args.first_name, Some("Test".to_string()));
            assert_eq!(register_args.last_name, Some("User".to_string()));
            assert_eq!(register_args.bio, Some("Test bio".to_string()));
            assert_eq!(register_args.timezone, "UTC");
            assert_eq!(register_args.locale, "en-US");
        } else {
            panic!("Expected Register command");
        }
    }

    #[test]
    fn test_login_command_parsing() {
        let args = vec![
            "test",
            "login",
            "--email",
            "test@example.com",
            "--password",
            "securepassword",
        ];

        let cli = TestCli::try_parse_from(args).unwrap();

        if let Some(UserCommands::Login(login_args)) = cli.user {
            assert_eq!(login_args.email, "test@example.com");
            assert_eq!(login_args.password, "securepassword");
        } else {
            panic!("Expected Login command");
        }
    }

    #[test]
    fn test_get_user_command_parsing() {
        let args = vec!["test", "get", "--identifier", "user123"];

        let cli = TestCli::try_parse_from(args).unwrap();

        if let Some(UserCommands::Get(get_args)) = cli.user {
            assert_eq!(get_args.identifier, "user123");
        } else {
            panic!("Expected Get command");
        }
    }

    #[test]
    fn test_register_command_with_minimal_args() {
        let args = vec![
            "test",
            "register",
            "--username",
            "testuser",
            "--email",
            "test@example.com",
            "--password",
            "securepassword",
        ];

        let cli = TestCli::try_parse_from(args).unwrap();

        if let Some(UserCommands::Register(register_args)) = cli.user {
            assert_eq!(register_args.username, "testuser");
            assert_eq!(register_args.email, "test@example.com");
            assert_eq!(register_args.password, "securepassword");
            assert_eq!(register_args.first_name, None);
            assert_eq!(register_args.last_name, None);
            assert_eq!(register_args.bio, None);
            // Default values should be set
            assert_eq!(register_args.timezone, "UTC");
            assert_eq!(register_args.locale, "en-US");
        } else {
            panic!("Expected Register command");
        }
    }
}
