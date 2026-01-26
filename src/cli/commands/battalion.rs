//! Battalion command implementations

use crate::cli::output::errors::CliError;
use crate::cli::templates::battalion_template::generate_battalion_template;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;
use std::sync::Arc;

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
    use crate::cli::interactive::confirm;

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

    // Check if output file already exists and prompt for confirmation
    if args.output.exists() {
        let should_overwrite = confirm(
            &format!(
                "File '{}' already exists. Overwrite?",
                args.output.display()
            ),
            false,
        )?;

        if !should_overwrite {
            return Err(CliError::Cancelled);
        }
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

/// Handle the `paladin battalion run` command
///
/// Loads a Battalion configuration and executes it with appropriate orchestration
pub async fn handle_battalion_run(args: BattalionRunArgs) -> Result<(), CliError> {
    use crate::cli::config::battalion_config::BattalionYamlConfig;
    use crate::cli::config::loader::load_battalion_config;

    // Load battalion configuration
    let battalion_config = load_battalion_config(&args.config)?;

    // Validate config type matches the type argument
    let config_type = match &battalion_config {
        BattalionYamlConfig::Formation(_) => "formation",
        BattalionYamlConfig::Phalanx(_) => "phalanx",
        BattalionYamlConfig::Campaign(_) => "campaign",
        BattalionYamlConfig::ChainOfCommand(_) => "chain-of-command",
    };

    if config_type != args.r#type {
        return Err(CliError::ValidationError {
            message: format!(
                "Configuration type mismatch: expected {}, but config file contains {}",
                args.r#type, config_type
            ),
        });
    }

    if args.verbose {
        println!(
            "{} Loading {} battalion: {}",
            "→".cyan().bold(),
            config_type,
            args.config.display()
        );
    }

    // Execute based on battalion type
    match battalion_config {
        BattalionYamlConfig::Formation(config) => {
            execute_formation(config, args.verbose, args.output).await
        }
        BattalionYamlConfig::Phalanx(config) => {
            execute_phalanx(config, args.verbose, args.output).await
        }
        BattalionYamlConfig::Campaign(_config) => {
            // Campaign execution - to be implemented
            Err(CliError::Other(
                "Campaign execution not yet implemented in CLI. See Task 7.0".to_string(),
            ))
        }
        BattalionYamlConfig::ChainOfCommand(_config) => {
            // Chain of Command execution - to be implemented
            Err(CliError::Other(
                "Chain of Command execution not yet implemented in CLI. See Task 7.0".to_string(),
            ))
        }
    }
}

/// Simple adapter to make PaladinExecutionService compatible with PaladinPort
struct PaladinExecutionAdapter {
    service: Arc<
        crate::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService,
    >,
}

#[async_trait::async_trait]
impl crate::application::ports::output::paladin_port::PaladinPort for PaladinExecutionAdapter {
    async fn execute(
        &self,
        paladin: &crate::core::platform::container::paladin::Paladin,
        input: &str,
    ) -> Result<
        crate::application::ports::output::paladin_port::PaladinResult,
        crate::application::use_cases::paladin::error::PaladinError,
    > {
        self.service.execute(paladin, input).await
    }

    async fn execute_stream(
        &self,
        _paladin: &crate::core::platform::container::paladin::Paladin,
        _input: &str,
    ) -> Result<
        crate::application::ports::output::paladin_port::PaladinStream,
        crate::application::use_cases::paladin::error::PaladinError,
    > {
        Err(
            crate::application::use_cases::paladin::error::PaladinError::ExecutionError(
                "Streaming not supported in CLI adapter".to_string(),
            ),
        )
    }

    fn validate(
        &self,
        _paladin: &crate::core::platform::container::paladin::Paladin,
    ) -> Result<(), crate::application::use_cases::paladin::error::PaladinError> {
        Ok(())
    }
}

/// Execute a Formation battalion
async fn execute_formation(
    config: crate::cli::config::battalion_config::FormationConfig,
    verbose: bool,
    output: Option<PathBuf>,
) -> Result<(), CliError> {
    use crate::application::ports::output::paladin_port::PaladinPort;
    use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    use crate::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    use crate::core::platform::container::battalion::BattalionConfig;
    use crate::core::platform::container::battalion::formation::Formation;
    use std::sync::Arc;
    use std::time::Duration;

    if verbose {
        println!("{} Building Formation: {}", "→".cyan().bold(), config.name);
        println!("{} Paladins: {}", "→".cyan().bold(), config.paladins.len());
    }

    // Build all Paladins
    let mut paladins = Vec::new();
    for (idx, paladin_ref) in config.paladins.iter().enumerate() {
        let paladin = build_paladin_from_reference(paladin_ref, verbose, idx + 1).await?;
        paladins.push(paladin);
    }

    // Create Battalion configuration
    let battalion_config = BattalionConfig::new(&config.name)
        .with_timeout(600) // Default 10 minute timeout
        .with_description(format!("Formation with {} Paladins", paladins.len()));

    // Create Formation
    let formation =
        Formation::new(paladins, battalion_config).map_err(|e| CliError::BattalionError {
            message: e.to_string(),
        })?;

    if verbose {
        println!("{} Formation built successfully", "✓".green().bold());
    }

    // Create Paladin execution service with circuit breaker
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));

    // Create a dummy LLM port (each Paladin already has its own configured)
    use crate::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;
    let factory = LlmProviderFactory::new();
    let dummy_llm_port = factory
        .create("openai")
        .map_err(|e| CliError::LlmProviderError {
            message: format!("Failed to create LLM port: {}", e),
        })?;

    let paladin_service = Arc::new(PaladinExecutionService::new(
        dummy_llm_port,
        circuit_breaker,
        None,
        None,
    ));

    // Wrap in adapter
    let paladin_port = Arc::new(PaladinExecutionAdapter {
        service: paladin_service,
    });

    // Create Formation execution service
    let formation_service =
        crate::application::use_cases::battalion::formation_service::FormationExecutionService::new(
            paladin_port as Arc<dyn PaladinPort>,
        );

    // Get input from user
    println!("\n{} Enter input for Formation:", "?".cyan().bold());
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .next()
        .ok_or_else(|| CliError::Other("No input provided".to_string()))?
        .map_err(|e| CliError::IoError {
            message: "Failed to read input".to_string(),
            source: e,
        })?;

    if verbose {
        println!(
            "{} Executing Formation with input: {}",
            "→".cyan().bold(),
            input
        );
    }

    // Execute Formation
    let start = std::time::Instant::now();
    let result = formation_service
        .execute(&formation, &input)
        .await
        .map_err(|e| CliError::BattalionError {
            message: e.to_string(),
        })?;
    let duration = start.elapsed();

    if verbose {
        println!(
            "{} Formation completed in {:.2}s",
            "✓".green().bold(),
            duration.as_secs_f64()
        );
        println!(
            "{} Paladin executions: {}",
            "→".cyan().bold(),
            result.paladin_results.len()
        );
    }

    // Handle output
    if let Some(output_path) = output {
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
        println!("\n{}", "═".repeat(80));
        println!("{} Formation Result", "📊".cyan());
        println!("{}", "═".repeat(80));
        println!("\n{} Final Output:", "→".cyan().bold());
        println!("{}\n", result.final_output);

        if verbose {
            println!("{} Individual Paladin Outputs:", "→".cyan().bold());
            for (idx, paladin_result) in result.paladin_results.iter().enumerate() {
                println!(
                    "\n  {}. Paladin {} ({} loops, {} tokens):",
                    idx + 1,
                    idx + 1,
                    paladin_result.loop_count,
                    paladin_result.token_count
                );
                println!(
                    "     {}",
                    paladin_result.output.lines().next().unwrap_or("")
                );
            }
        }

        println!("\n{}", "═".repeat(80));
    }

    Ok(())
}

/// Execute a Phalanx battalion
async fn execute_phalanx(
    config: crate::cli::config::battalion_config::PhalanxConfig,
    verbose: bool,
    output: Option<PathBuf>,
) -> Result<(), CliError> {
    use crate::application::ports::output::paladin_port::PaladinPort;
    use crate::application::use_cases::paladin::circuit_breaker::CircuitBreaker;
    use crate::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
    use crate::core::platform::container::battalion::BattalionConfig;
    use crate::core::platform::container::battalion::phalanx::Phalanx;
    use std::sync::Arc;
    use std::time::Duration;

    if verbose {
        println!("{} Building Phalanx: {}", "→".cyan().bold(), config.name);
        println!(
            "{} Paladins: {} (parallel)",
            "→".cyan().bold(),
            config.paladins.len()
        );
    }

    // Build all Paladins
    let mut paladins = Vec::new();
    for (idx, paladin_ref) in config.paladins.iter().enumerate() {
        let paladin = build_paladin_from_reference(paladin_ref, verbose, idx + 1).await?;
        paladins.push(paladin);
    }

    // Create Battalion configuration
    let battalion_config = BattalionConfig::new(&config.name)
        .with_timeout(600)
        .with_description(format!("Phalanx with {} parallel Paladins", paladins.len()));

    // Create Phalanx
    let phalanx =
        Phalanx::new(paladins, battalion_config).map_err(|e| CliError::BattalionError {
            message: e.to_string(),
        })?;

    if verbose {
        println!("{} Phalanx built successfully", "✓".green().bold());
    }

    // Create Paladin execution service
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, Duration::from_secs(30)));
    use crate::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;
    let factory = LlmProviderFactory::new();
    let dummy_llm_port = factory
        .create("openai")
        .map_err(|e| CliError::LlmProviderError {
            message: format!("Failed to create LLM port: {}", e),
        })?;

    let paladin_service = Arc::new(PaladinExecutionService::new(
        dummy_llm_port,
        circuit_breaker,
        None,
        None,
    ));

    // Wrap in adapter
    let paladin_port = Arc::new(PaladinExecutionAdapter {
        service: paladin_service,
    });

    // Create Phalanx execution service
    let phalanx_service =
        crate::application::use_cases::battalion::phalanx_service::PhalanxExecutionService::new(
            paladin_port as Arc<dyn PaladinPort>,
        );

    // Get input from user
    println!("\n{} Enter input for Phalanx:", "?".cyan().bold());
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .next()
        .ok_or_else(|| CliError::Other("No input provided".to_string()))?
        .map_err(|e| CliError::IoError {
            message: "Failed to read input".to_string(),
            source: e,
        })?;

    if verbose {
        println!(
            "{} Executing Phalanx with {} parallel Paladins",
            "→".cyan().bold(),
            phalanx.paladins().len()
        );
    }

    // Execute Phalanx
    let start = std::time::Instant::now();
    let result = phalanx_service
        .execute(&phalanx, &input)
        .await
        .map_err(|e| CliError::BattalionError {
            message: e.to_string(),
        })?;
    let duration = start.elapsed();

    if verbose {
        println!(
            "{} Phalanx completed in {:.2}s",
            "✓".green().bold(),
            duration.as_secs_f64()
        );
    }

    // Handle output
    if let Some(output_path) = output {
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
        println!("\n{}", "═".repeat(80));
        println!("{} Phalanx Results (Parallel Execution)", "📊".cyan());
        println!("{}", "═".repeat(80));

        for (idx, paladin_result) in result.paladin_results.iter().enumerate() {
            println!(
                "\n{} Paladin {}:",
                format!("{}.", idx + 1).cyan().bold(),
                idx + 1
            );
            println!(
                "   Loops: {}, Tokens: {}",
                paladin_result.loop_count, paladin_result.token_count
            );
            println!("   {}\n", "─".repeat(76));
            println!("   {}", paladin_result.output);
        }

        println!("\n{}", "═".repeat(80));
    }

    Ok(())
}

/// Build a Paladin from a PaladinReference (inline or file)
async fn build_paladin_from_reference(
    reference: &crate::cli::config::battalion_config::PaladinReference,
    verbose: bool,
    index: usize,
) -> Result<crate::core::platform::container::paladin::Paladin, CliError> {
    use crate::application::use_cases::paladin::paladin_builder::PaladinBuilder;
    use crate::cli::config::battalion_config::PaladinReference;
    use crate::cli::config::loader::load_paladin_config;
    use crate::infrastructure::adapters::llm::provider_factory::LlmProviderFactory;

    let paladin_config = match reference {
        PaladinReference::File { file } => {
            if verbose {
                println!("  {} Loading Paladin {} from: {}", "→".cyan(), index, file);
            }
            load_paladin_config(&std::path::PathBuf::from(file))?
        }
        PaladinReference::Inline(inline) => {
            if verbose {
                println!(
                    "  {} Building inline Paladin {}: {}",
                    "→".cyan(),
                    index,
                    inline.name
                );
            }
            (&**inline).clone()
        }
    };

    // Check API key
    let env_var_name = match paladin_config.provider.provider_type.as_str() {
        "openai" => "OPENAI_API_KEY",
        "deepseek" => "DEEPSEEK_API_KEY",
        "anthropic" => "ANTHROPIC_API_KEY",
        _ => {
            return Err(CliError::InvalidFieldValue {
                field: "provider.type".to_string(),
                message: format!(
                    "Unknown provider: {}",
                    paladin_config.provider.provider_type
                ),
            });
        }
    };

    if std::env::var(env_var_name).is_err() {
        return Err(CliError::MissingApiKey {
            provider: paladin_config.provider.provider_type.clone(),
            env_var: env_var_name.to_string(),
        });
    }

    // Create LLM port
    let factory = LlmProviderFactory::new();
    let llm_port = factory
        .create(&paladin_config.provider.provider_type)
        .map_err(|e| CliError::LlmProviderError {
            message: e.to_string(),
        })?;

    // Build Paladin
    let mut builder = PaladinBuilder::new(llm_port)
        .system_prompt(&paladin_config.system_prompt)
        .name(&paladin_config.name)
        .model(&paladin_config.model)
        .temperature(paladin_config.temperature)
        .max_loops(paladin_config.max_loops)
        .timeout_seconds(paladin_config.timeout_seconds);

    // Add stop words
    for word in &paladin_config.stop_words {
        builder = builder.add_stop_word(word);
    }

    let paladin = builder.build()?;

    if verbose {
        println!(
            "    {} {} ready ({})",
            "✓".green(),
            paladin_config.name,
            paladin_config.model
        );
    }

    Ok(paladin)
}
