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

/// Handle the `paladin agent run` command
///
/// Loads a Paladin configuration and executes it with the given input
pub async fn handle_agent_run(args: AgentRunArgs) -> Result<(), CliError> {
    use crate::application::ports::output::llm_port::LlmPort;
    use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    use crate::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    use crate::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    use crate::cli::config::loader::load_paladin_config;
    use crate::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;
    use std::sync::Arc;
    use std::time::Duration;

    // Load configuration
    let config = load_paladin_config(&args.config)?;

    // Get input (for now, require --input flag; interactive mode in Task 11.0)
    let input = args.input.ok_or_else(|| CliError::MissingRequiredField {
        field: "input".to_string(),
        message:
            "Input text is required. Use --input flag or wait for Task 11.0 for interactive mode."
                .to_string(),
    })?;

    // Load API key from environment variable based on provider
    let env_var_name = match config.provider.provider_type.as_str() {
        "openai" => "OPENAI_API_KEY",
        "deepseek" => "DEEPSEEK_API_KEY",
        "anthropic" => "ANTHROPIC_API_KEY",
        _ => {
            return Err(CliError::InvalidFieldValue {
                field: "provider.type".to_string(),
                message: format!(
                    "Unknown provider: {}. Supported: openai, deepseek, anthropic",
                    config.provider.provider_type
                ),
            });
        }
    };

    // Check if API key is set
    if std::env::var(env_var_name).is_err() {
        return Err(CliError::MissingApiKey {
            provider: config.provider.provider_type.clone(),
            env_var: env_var_name.to_string(),
        });
    }

    if args.verbose {
        println!(
            "{} Using provider: {}",
            "→".cyan().bold(),
            config.provider.provider_type
        );
        println!("{} Model: {}", "→".cyan().bold(), config.model);
    }

    // Create LLM port adapter using provider factory
    let factory = LlmProviderFactory::new();
    let llm_port: Arc<dyn LlmPort> =
        factory
            .create(&config.provider.provider_type)
            .map_err(|e| CliError::LlmProviderError {
                message: e.to_string(),
            })?;

    // Create circuit breaker for fault tolerance
    let circuit_breaker = Arc::new(CircuitBreaker::new(
        3,                       // failure_threshold
        2,                       // success_threshold
        Duration::from_secs(30), // timeout_duration
    ));

    // TODO: Task 5.8 - Configure garrison if specified in config
    let garrison = None; // Stub for now

    // TODO: Task 5.9 - Configure arsenal/MCP servers if specified in config
    let arsenal = None; // Stub for now

    // Create Paladin execution service
    let service =
        PaladinExecutionService::new(llm_port.clone(), circuit_breaker, garrison, arsenal);

    // Build Paladin from configuration using PaladinBuilder
    let mut builder = PaladinBuilder::new(llm_port)
        .system_prompt(&config.system_prompt)
        .name(&config.name)
        .model(&config.model)
        .temperature(config.temperature)
        .max_loops(config.max_loops)
        .timeout_seconds(config.timeout_seconds);

    // Add stop words
    for word in &config.stop_words {
        builder = builder.add_stop_word(word);
    }

    let paladin = builder.build()?;

    if args.verbose {
        println!("{} Executing Paladin: {}", "→".cyan().bold(), config.name);
        println!("{} Input: {}", "→".cyan().bold(), input);
    }

    // Execute Paladin
    let start = std::time::Instant::now();
    let result = service
        .execute(&paladin, &input)
        .await
        .map_err(|e| CliError::ExecutionError {
            message: e.to_string(),
        })?;
    let duration = start.elapsed();

    if args.verbose {
        println!(
            "{} Execution completed in {:.2}s",
            "✓".green().bold(),
            duration.as_secs_f64()
        );
        println!(
            "{} Loops: {}, Tokens: {}",
            "→".cyan().bold(),
            result.loop_count,
            result.token_count
        );
        println!(
            "{} Stop reason: {:?}",
            "→".cyan().bold(),
            result.stop_reason
        );
    }

    // Handle output
    if let Some(output_path) = args.output {
        // Write JSON to file
        let json_output =
            serde_json::to_string_pretty(&result).map_err(|e| CliError::SerializationError {
                message: e.to_string(),
            })?;
        std::fs::write(&output_path, json_output)?;
        println!(
            "{} Output written to: {}",
            "✓".green().bold(),
            output_path.display()
        );
    } else {
        // Print human-readable output to stdout
        println!("\n{}", "─".repeat(60));
        println!("{}", result.output);
        println!("{}", "─".repeat(60));
    }

    Ok(())
}
