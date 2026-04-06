//! Snapshot tests for CLI help output
//!
//! Tests help text formatting, command documentation, and usage examples.

use paladin::application::cli::formatters::output::{OutputFormatter, OutputStyle};

use super::ensure_no_color;

#[test]
fn test_command_help_basic() {
    ensure_no_color();
    // Test basic command help format
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("paladin init", OutputStyle::Success)
    ));
    output.push_str("Initialize a new Paladin configuration\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("USAGE:", OutputStyle::Info)
    ));
    output.push_str("    paladin init [OPTIONS]\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("OPTIONS:", OutputStyle::Info)
    ));
    output.push_str("    -c, --config <FILE>    Configuration file path [default: config.yml]\n");
    output.push_str("    -f, --force            Overwrite existing configuration\n");
    output.push_str("    -h, --help             Print help information\n");

    insta::assert_snapshot!("command_help_basic", output);
}

#[test]
fn test_command_help_with_examples() {
    ensure_no_color();
    // Test command help with examples section
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("paladin execute", OutputStyle::Success)
    ));
    output.push_str("Execute a Paladin with a prompt\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("USAGE:", OutputStyle::Info)
    ));
    output.push_str("    paladin execute [OPTIONS] <PROMPT>\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("OPTIONS:", OutputStyle::Info)
    ));
    output.push_str("    -m, --model <MODEL>      LLM model to use [default: gpt-4]\n");
    output.push_str("    -t, --temperature <T>    Temperature (0.0-2.0) [default: 0.7]\n");
    output.push_str("    -v, --verbose            Enable verbose output\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("EXAMPLES:", OutputStyle::Info)
    ));
    output.push_str("    # Execute with default settings\n");
    output.push_str("    paladin execute \"Analyze this data\"\n\n");
    output.push_str("    # Use specific model and temperature\n");
    output.push_str("    paladin execute -m gpt-3.5-turbo -t 0.3 \"Write a poem\"\n\n");
    output.push_str("    # Verbose mode for debugging\n");
    output.push_str("    paladin execute -v \"Complex task\"\n");

    insta::assert_snapshot!("command_help_with_examples", output);
}

#[test]
fn test_subcommand_list() {
    ensure_no_color();
    // Test subcommand listing format
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Available Commands:", OutputStyle::Info)
    ));

    let commands = vec![
        ("init", "Initialize configuration"),
        ("execute", "Execute a Paladin"),
        ("battalion", "Run multiple Paladins"),
        ("config", "Manage configuration"),
        ("version", "Show version information"),
        ("help", "Print help information"),
    ];

    for (cmd, desc) in commands {
        output.push_str(&format!(
            "  {:15} {}\n",
            formatter.style(cmd, OutputStyle::Success),
            desc
        ));
    }

    insta::assert_snapshot!("subcommand_list", output);
}

#[test]
fn test_option_groups() {
    ensure_no_color();
    // Test grouped options display
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Configuration Options:", OutputStyle::Info)
    ));
    output.push_str("  -c, --config <FILE>       Configuration file path\n");
    output.push_str("  -e, --env <ENV>           Environment (dev/staging/prod)\n\n");

    output.push_str(&format!(
        "{}\n\n",
        formatter.style("LLM Options:", OutputStyle::Info)
    ));
    output.push_str("  -m, --model <MODEL>       LLM model name\n");
    output.push_str("  -t, --temperature <T>     Temperature (0.0-2.0)\n");
    output.push_str("  -p, --max-tokens <N>      Maximum tokens\n\n");

    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Output Options:", OutputStyle::Info)
    ));
    output.push_str("  -o, --output <FILE>       Output file path\n");
    output.push_str("  -f, --format <FMT>        Output format (json/markdown/table)\n");
    output.push_str("  -v, --verbose             Verbose output\n");
    output.push_str("  -q, --quiet               Quiet mode\n");

    insta::assert_snapshot!("option_groups", output);
}

#[test]
fn test_help_header() {
    ensure_no_color();
    // Test main help header
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    let title = "Paladin Multi-Agent Orchestration Framework";
    let width = title.len() + 4;
    let border = "═".repeat(width);

    output.push_str(&format!("┌{}┐\n", border));
    output.push_str(&format!(
        "│ {} │\n",
        formatter.style(title, OutputStyle::Info)
    ));
    output.push_str(&format!("└{}┘\n\n", border));

    output.push_str(&format!(
        "{} v1.0.0\n",
        formatter.style("Version:", OutputStyle::Info)
    ));
    output.push_str(&format!(
        "{} https://github.com/DF3NDR/paladin\n",
        formatter.style("Repository:", OutputStyle::Link)
    ));
    output.push_str(&format!(
        "{} https://docs.paladin.rs\n\n",
        formatter.style("Documentation:", OutputStyle::Link)
    ));

    insta::assert_snapshot!("help_header", output);
}

#[test]
fn test_usage_examples_section() {
    ensure_no_color();
    // Test detailed usage examples
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Common Usage Examples", OutputStyle::Info)
    ));

    // Example 1
    output.push_str(&format!(
        "{}\n",
        formatter.style("1. Initialize a new project:", OutputStyle::Success)
    ));
    output.push_str("   $ paladin init --config my-config.yml\n\n");

    // Example 2
    output.push_str(&format!(
        "{}\n",
        formatter.style("2. Execute a simple Paladin:", OutputStyle::Success)
    ));
    output.push_str("   $ paladin execute \"Analyze the quarterly sales data\"\n\n");

    // Example 3
    output.push_str(&format!(
        "{}\n",
        formatter.style(
            "3. Run a Battalion (multiple Paladins):",
            OutputStyle::Success
        )
    ));
    output.push_str("   $ paladin battalion --formation sequential --config battalion.yml\n\n");

    // Example 4
    output.push_str(&format!(
        "{}\n",
        formatter.style("4. Check configuration:", OutputStyle::Success)
    ));
    output.push_str("   $ paladin config validate\n\n");

    // Example 5
    output.push_str(&format!(
        "{}\n",
        formatter.style("5. Export results as JSON:", OutputStyle::Success)
    ));
    output.push_str("   $ paladin execute -f json -o results.json \"Generate report\"\n");

    insta::assert_snapshot!("usage_examples_section", output);
}

#[test]
fn test_error_help_message() {
    ensure_no_color();
    // Test error message with help suggestion
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("✗ Invalid command", OutputStyle::Error)
    ));
    output.push_str("Command 'pal execute' is not recognized.\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("Did you mean?", OutputStyle::Info)
    ));
    output.push_str("  • paladin execute\n");
    output.push_str("  • paladin battalion\n\n");
    output.push_str(&format!(
        "{}\n",
        formatter.style("For help, run:", OutputStyle::Info)
    ));
    output.push_str("  paladin --help\n");

    insta::assert_snapshot!("error_help_message", output);
}

#[test]
fn test_feature_flags_help() {
    ensure_no_color();
    // Test feature flags documentation
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Available Features:", OutputStyle::Info)
    ));

    let features = vec![
        ("redis-queue", "Enable Redis queue support", "Enabled"),
        ("s3-storage", "Enable S3 file storage", "Enabled"),
        ("openai-embeddings", "OpenAI embedding support", "Enabled"),
        ("qdrant", "Qdrant vector database", "Optional"),
        ("integration-tests", "Integration test suite", "Optional"),
    ];

    for (feature, desc, status) in features {
        let status_style = if status == "Enabled" {
            OutputStyle::Success
        } else {
            OutputStyle::Info
        };
        output.push_str(&format!(
            "  {:20} {:35} [{}]\n",
            feature,
            desc,
            formatter.style(status, status_style)
        ));
    }

    insta::assert_snapshot!("feature_flags_help", output);
}

#[test]
fn test_environment_variables_help() {
    ensure_no_color();
    // Test environment variables documentation
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Environment Variables:", OutputStyle::Info)
    ));

    let env_vars = vec![
        ("PALADIN_CONFIG", "Configuration file path", "config.yml"),
        ("PALADIN_LOG_LEVEL", "Logging level", "info"),
        ("OPENAI_API_KEY", "OpenAI API key", "(required)"),
        (
            "REDIS_URL",
            "Redis connection URL",
            "redis://localhost:6379",
        ),
        ("NO_COLOR", "Disable colored output", "0"),
    ];

    for (var, desc, default) in env_vars {
        output.push_str(&format!(
            "  {}\n",
            formatter.style(var, OutputStyle::Success)
        ));
        output.push_str(&format!("      {}\n", desc));
        output.push_str(&format!(
            "      Default: {}\n\n",
            formatter.style(default, OutputStyle::Info)
        ));
    }

    insta::assert_snapshot!("environment_variables_help", output);
}

#[test]
fn test_configuration_help() {
    ensure_no_color();
    // Test configuration file help
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Configuration File (config.yml):", OutputStyle::Info)
    ));
    output.push_str("Example configuration:\n\n");
    output.push_str(&formatter.style("```yaml\n", OutputStyle::Default));
    output.push_str("paladin:\n");
    output.push_str("  default_model: gpt-4\n");
    output.push_str("  default_temperature: 0.7\n");
    output.push_str("  max_loops: 5\n\n");
    output.push_str("llm:\n");
    output.push_str("  openai:\n");
    output.push_str("    api_key: ${OPENAI_API_KEY}\n");
    output.push_str("    base_url: https://api.openai.com/v1\n\n");
    output.push_str("garrison:\n");
    output.push_str("  type: in_memory\n");
    output.push_str("  max_entries: 1000\n");
    output.push_str(&formatter.style("```\n", OutputStyle::Default));

    insta::assert_snapshot!("configuration_help", output);
}

#[test]
fn test_troubleshooting_help() {
    ensure_no_color();
    // Test troubleshooting section
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n\n",
        formatter.style("Troubleshooting:", OutputStyle::Info)
    ));

    output.push_str(&format!(
        "{}\n",
        formatter.style("Common Issues:", OutputStyle::Warning)
    ));
    output.push_str("\n1. API Key not found\n");
    output.push_str("   Run: export OPENAI_API_KEY=your-key-here\n\n");

    output.push_str("2. Configuration file missing\n");
    output.push_str("   Run: paladin init to create default config\n\n");

    output.push_str("3. Redis connection failed\n");
    output.push_str("   Check: docker-compose up -d redis\n\n");

    output.push_str(&format!(
        "{}\n",
        formatter.style("For more help:", OutputStyle::Info)
    ));
    output.push_str(&format!(
        "  Documentation: {}\n",
        formatter.style("https://docs.paladin.rs", OutputStyle::Link)
    ));
    output.push_str(&format!(
        "  Issues: {}\n",
        formatter.style(
            "https://github.com/DF3NDR/paladin/issues",
            OutputStyle::Link
        )
    ));

    insta::assert_snapshot!("troubleshooting_help", output);
}

#[test]
fn test_version_output() {
    ensure_no_color();
    // Test version information output
    let formatter = OutputFormatter::new();

    let mut output = String::new();
    output.push_str(&format!(
        "{} {}\n",
        formatter.style("Paladin", OutputStyle::Success),
        formatter.style("v1.0.0", OutputStyle::Info)
    ));
    output.push_str(&format!(
        "{} 2024\n",
        formatter.style("Rust Edition:", OutputStyle::Info)
    ));
    output.push_str(&format!(
        "{} x86_64-unknown-linux-gnu\n\n",
        formatter.style("Target:", OutputStyle::Info)
    ));

    output.push_str(&format!(
        "{}\n",
        formatter.style("Enabled Features:", OutputStyle::Info)
    ));
    output.push_str("  • redis-queue\n");
    output.push_str("  • s3-storage\n");
    output.push_str("  • openai-embeddings\n");

    insta::assert_snapshot!("version_output", output);
}
