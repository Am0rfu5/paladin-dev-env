//! Agent command implementations for creating and running Paladin agents
//!
//! This module provides CLI commands for:
//! - Creating Paladin configuration templates
//! - Executing Paladins from YAML configuration files
//!
//! # Examples
//!
//! ```bash
//! # Create a new agent template
//! paladin agent new my-assistant --output agent.yaml --provider openai
//!
//! # Run an agent from config
//! paladin agent run --config agent.yaml --input "What is Rust?"
//! ```

use crate::application::cli::config::loader::{instantiate_arsenal, instantiate_garrison};
use crate::application::cli::error::CliError;
use crate::application::cli::templates::paladin_template::generate_paladin_template;
use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;

/// Agent subcommands for Paladin management
#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    /// Create a new Paladin configuration template
    New(AgentNewArgs),
    /// Run a Paladin from configuration
    Run(AgentRunArgs),
}

/// Arguments for creating a new Paladin agent template
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

/// Arguments for executing a Paladin agent
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

    /// Path to image file(s) for vision processing (repeatable)
    #[cfg(feature = "vision")]
    #[arg(long = "image")]
    pub images: Vec<PathBuf>,

    /// Path to document file for processing (PDF, TXT, MD)
    #[arg(long = "document")]
    pub document: Option<PathBuf>,

    // Autonomous feature flags (override config file)
    /// Enable autonomous planning mode (MaxLoops::Auto)
    #[arg(long = "auto-plan")]
    pub auto_plan: bool,

    /// Enable automatic prompt generation
    #[arg(long = "auto-prompt")]
    pub auto_prompt: bool,

    /// Enable dynamic temperature adjustment based on task type
    #[arg(long = "dynamic-temp")]
    pub dynamic_temp: bool,

    /// Enable agent handoff capabilities
    #[arg(long = "enable-handoffs")]
    pub enable_handoffs: bool,
}

/// Handle the `paladin agent new` command
///
/// Creates a new Paladin configuration template file with documented options
pub fn handle_agent_new(args: AgentNewArgs) -> Result<(), CliError> {
    use crate::application::cli::interactive::confirm;

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
    use crate::application::cli::config::loader::load_paladin_config;
    use crate::application::cli::interactive::prompt_for_input;
    use crate::application::services::paladin::paladin_builder::PaladinBuilder;
    use crate::application::services::paladin::paladin_execution_service::PaladinExecutionService;
    #[cfg(feature = "vision")]
    use crate::core::platform::container::vision::{ImageDetail, VisionContent};
    #[cfg(feature = "content-processing")]
    use crate::infrastructure::adapters::document::DocumentAdapter;
    use crate::infrastructure::resilience::circuit_breaker::CircuitBreaker;
    use paladin_llm::provider_factory::LlmProviderFactory;
    #[cfg(feature = "content-processing")]
    use paladin_ports::input::document_port::{DocumentPort, DocumentSource};
    use paladin_ports::output::llm_port::LlmPort;
    use std::sync::Arc;
    use std::time::Duration;

    // Validate image paths if provided
    #[cfg(feature = "vision")]
    if !args.images.is_empty() {
        for image_path in &args.images {
            if !image_path.exists() {
                return Err(CliError::InvalidFilePath {
                    path: image_path.display().to_string(),
                    message: "Image file does not exist".to_string(),
                });
            }

            // Validate image format
            let extension = image_path
                .extension()
                .and_then(|e| e.to_str())
                .ok_or_else(|| CliError::InvalidFilePath {
                    path: image_path.display().to_string(),
                    message: "No file extension found".to_string(),
                })?;

            let valid_image_formats = ["png", "jpg", "jpeg", "gif", "webp"];
            if !valid_image_formats.contains(&extension.to_lowercase().as_str()) {
                return Err(CliError::UnsupportedFormat {
                    format: extension.to_string(),
                    supported: "png, jpg, jpeg, gif, webp".to_string(),
                });
            }
        }

        if args.verbose {
            println!(
                "{} {} image(s) provided",
                "→".cyan().bold(),
                args.images.len()
            );
        }
    }

    // Validate document path if provided
    if let Some(doc_path) = &args.document {
        if !doc_path.exists() {
            return Err(CliError::InvalidFilePath {
                path: doc_path.display().to_string(),
                message: "Document file does not exist".to_string(),
            });
        }

        // Validate document format
        let extension = doc_path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| CliError::InvalidFilePath {
                path: doc_path.display().to_string(),
                message: "No file extension found".to_string(),
            })?;

        let valid_doc_formats = ["pdf", "txt", "md", "markdown"];
        if !valid_doc_formats.contains(&extension.to_lowercase().as_str()) {
            return Err(CliError::UnsupportedFormat {
                format: extension.to_string(),
                supported: "pdf, txt, md, markdown".to_string(),
            });
        }

        if args.verbose {
            println!(
                "{} Document provided: {}",
                "→".cyan().bold(),
                doc_path.display()
            );
        }
    }

    // Load configuration
    let config = load_paladin_config(&args.config)?;

    // Get input - prompt interactively if not provided per FR-6
    let input = if let Some(input_text) = args.input {
        input_text
    } else {
        prompt_for_input("Enter input for Paladin")?
    };

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

    // Configure garrison if specified in config (Task 5.8)
    let garrison = instantiate_garrison(&config.garrison, &config.name).await?;

    // Configure arsenal/MCP servers if specified in config (Task 5.9)
    let arsenal = instantiate_arsenal(&config.arsenal).await?;

    // Create Paladin execution service
    let service =
        PaladinExecutionService::new(llm_port.clone(), circuit_breaker, garrison, arsenal);

    // Build Paladin from configuration using PaladinBuilder
    let mut builder = PaladinBuilder::new(llm_port)
        .system_prompt(&config.system_prompt)
        .name(&config.name)
        .model(&config.model)
        .temperature(config.temperature)
        .max_loops(config.max_loops.as_u32())
        .timeout_seconds(config.timeout_seconds);

    // Add stop words
    for word in &config.stop_words {
        builder = builder.add_stop_word(word);
    }

    // Enable vision if images are provided
    #[cfg(feature = "vision")]
    if !args.images.is_empty() {
        builder = builder.enable_vision(true);
        if args.verbose {
            println!("{} Vision mode enabled", "→".cyan().bold());
        }
    }

    let paladin = builder.build().await?;

    // Process document if provided
    #[allow(unused_mut)]
    let mut combined_input = input.clone();
    #[cfg(feature = "content-processing")]
    {
        if let Some(doc_path) = &args.document {
            if args.verbose {
                println!(
                    "{} Processing document: {}",
                    "→".cyan().bold(),
                    doc_path.display()
                );
            }

            let doc_adapter = DocumentAdapter::new();
            let document = doc_adapter
                .ingest(DocumentSource::File(doc_path.clone()))
                .await
                .map_err(|e| CliError::DocumentProcessingError {
                    message: e.to_string(),
                })?;

            // Extract text from all pages
            let doc_text: String = document
                .pages
                .iter()
                .map(|p| p.content.as_str())
                .collect::<Vec<_>>()
                .join("\n\n");

            if args.verbose {
                println!(
                    "{} Document processed: {} pages, {} words",
                    "✓".green().bold(),
                    document.page_count(),
                    document.word_count()
                );
            }

            // Combine input with document text
            combined_input = format!("{}\n\nDocument content:\n{}\n", input, doc_text);
        }
    }
    #[cfg(not(feature = "content-processing"))]
    {
        if args.document.is_some() {
            return Err(CliError::DocumentProcessingError {
                message: "Document processing requires the 'content-processing' feature flag"
                    .to_string(),
            });
        }
    }

    if args.verbose {
        println!("{} Executing Paladin: {}", "→".cyan().bold(), config.name);
        #[cfg(feature = "vision")]
        if !args.images.is_empty() {
            println!(
                "{} Input: {} (with {} image(s))",
                "→".cyan().bold(),
                input,
                args.images.len()
            );
        } else {
            println!("{} Input: {}", "→".cyan().bold(), input);
        }
    }

    // Execute Paladin with vision support if images provided
    let start = std::time::Instant::now();
    let result = {
        #[cfg(feature = "vision")]
        {
            if !args.images.is_empty() {
                // Load images and convert to VisionContent
                let mut vision_contents = Vec::new();
                for image_path in &args.images {
                    if args.verbose {
                        println!("Loading image: {}", image_path.display());
                    }

                    // Create VisionContent from file path
                    let vision_content = VisionContent::ImageFile {
                        path: image_path.clone(),
                        detail: ImageDetail::Auto,
                    };

                    // Validate format
                    vision_content.validate_format().map_err(|e| {
                        CliError::VisionProcessingError {
                            message: e.to_string(),
                        }
                    })?;

                    vision_contents.push(vision_content);
                }

                // Execute with vision
                service
                    .execute_with_vision(&paladin, &combined_input, vision_contents)
                    .await
                    .map_err(|e| CliError::ExecutionError {
                        message: e.to_string(),
                    })?
            } else {
                // Standard execution without vision
                service
                    .execute(&paladin, &combined_input)
                    .await
                    .map_err(|e| CliError::ExecutionError {
                        message: e.to_string(),
                    })?
            }
        }
        #[cfg(not(feature = "vision"))]
        {
            // Standard execution without vision
            service
                .execute(&paladin, &combined_input)
                .await
                .map_err(|e| CliError::ExecutionError {
                    message: e.to_string(),
                })?
        }
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_agent_new_args_creation() {
        let args = AgentNewArgs {
            name: "test-agent".to_string(),
            output: PathBuf::from("test.yaml"),
            provider: Some("openai".to_string()),
        };

        assert_eq!(args.name, "test-agent");
        assert_eq!(args.output, PathBuf::from("test.yaml"));
        assert_eq!(args.provider, Some("openai".to_string()));
    }

    #[test]
    fn test_agent_run_args_creation() {
        let args = AgentRunArgs {
            config: PathBuf::from("config.yaml"),
            input: Some("test input".to_string()),
            output: Some(PathBuf::from("output.json")),
            verbose: true,
            #[cfg(feature = "vision")]
            images: vec![],
            document: None,
            auto_plan: false,
            auto_prompt: false,
            dynamic_temp: false,
            enable_handoffs: false,
        };

        assert_eq!(args.config, PathBuf::from("config.yaml"));
        assert_eq!(args.input, Some("test input".to_string()));
        assert_eq!(args.output, Some(PathBuf::from("output.json")));
        assert!(args.verbose);
        #[cfg(feature = "vision")]
        assert!(args.images.is_empty());
        assert!(args.document.is_none());
    }

    #[test]
    #[cfg(feature = "vision")]
    fn test_agent_run_args_with_images() {
        let args = AgentRunArgs {
            config: PathBuf::from("config.yaml"),
            input: Some("analyze these".to_string()),
            output: None,
            verbose: false,
            #[cfg(feature = "vision")]
            images: vec![PathBuf::from("image1.png"), PathBuf::from("image2.jpg")],
            document: None,
            auto_plan: false,
            auto_prompt: false,
            dynamic_temp: false,
            enable_handoffs: false,
        };

        assert_eq!(args.images.len(), 2);
        assert_eq!(args.images[0], PathBuf::from("image1.png"));
        assert_eq!(args.images[1], PathBuf::from("image2.jpg"));
    }

    #[test]
    fn test_agent_run_args_with_document() {
        let args = AgentRunArgs {
            config: PathBuf::from("config.yaml"),
            input: Some("summarize this".to_string()),
            output: None,
            verbose: false,
            #[cfg(feature = "vision")]
            images: vec![],
            document: Some(PathBuf::from("document.pdf")),
            auto_plan: false,
            auto_prompt: false,
            dynamic_temp: false,
            enable_handoffs: false,
        };

        assert_eq!(args.document, Some(PathBuf::from("document.pdf")));
    }

    #[test]
    fn test_handle_agent_new_success() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("agent.yaml");

        let args = AgentNewArgs {
            name: "test-paladin".to_string(),
            output: output_path.clone(),
            provider: Some("openai".to_string()),
        };

        let result = handle_agent_new(args);
        assert!(result.is_ok());
        assert!(output_path.exists());

        // Verify file content contains expected elements
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("test-paladin"));
        assert!(content.contains("openai"));
    }

    #[test]
    fn test_handle_agent_new_default_provider() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("agent.yaml");

        let args = AgentNewArgs {
            name: "test-paladin".to_string(),
            output: output_path.clone(),
            provider: None, // Default to openai
        };

        let result = handle_agent_new(args);
        assert!(result.is_ok());
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("openai"));
    }

    #[test]
    fn test_handle_agent_new_invalid_provider() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("agent.yaml");

        let args = AgentNewArgs {
            name: "test-paladin".to_string(),
            output: output_path.clone(),
            provider: Some("invalid_provider".to_string()),
        };

        let result = handle_agent_new(args);
        assert!(result.is_err());

        match result {
            Err(CliError::InvalidFieldValue { field, message }) => {
                assert_eq!(field, "provider");
                assert!(message.contains("invalid_provider"));
            }
            _ => panic!("Expected InvalidFieldValue error"),
        }
    }

    #[test]
    fn test_handle_agent_new_deepseek_provider() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("agent.yaml");

        let args = AgentNewArgs {
            name: "deepseek-paladin".to_string(),
            output: output_path.clone(),
            provider: Some("deepseek".to_string()),
        };

        let result = handle_agent_new(args);
        assert!(result.is_ok());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("deepseek"));
    }

    #[test]
    fn test_handle_agent_new_anthropic_provider() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("agent.yaml");

        let args = AgentNewArgs {
            name: "anthropic-paladin".to_string(),
            output: output_path.clone(),
            provider: Some("anthropic".to_string()),
        };

        let result = handle_agent_new(args);
        assert!(result.is_ok());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("anthropic"));
    }

    #[test]
    fn test_handle_agent_new_file_write_error() {
        // Try to write to an invalid path (directory that doesn't exist)
        let invalid_path = PathBuf::from("/nonexistent/directory/agent.yaml");

        let args = AgentNewArgs {
            name: "test-paladin".to_string(),
            output: invalid_path,
            provider: Some("openai".to_string()),
        };

        let result = handle_agent_new(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_commands_enum_new_variant() {
        let new_args = AgentNewArgs {
            name: "test".to_string(),
            output: PathBuf::from("test.yaml"),
            provider: None,
        };
        let command = AgentCommands::New(new_args);

        match command {
            AgentCommands::New(args) => {
                assert_eq!(args.name, "test");
            }
            _ => panic!("Expected New variant"),
        }
    }

    #[test]
    fn test_agent_commands_enum_run_variant() {
        let run_args = AgentRunArgs {
            config: PathBuf::from("config.yaml"),
            input: None,
            output: None,
            verbose: false,
            #[cfg(feature = "vision")]
            images: vec![],
            document: None,
            auto_plan: false,
            auto_prompt: false,
            dynamic_temp: false,
            enable_handoffs: false,
        };
        let command = AgentCommands::Run(run_args);

        match command {
            AgentCommands::Run(args) => {
                assert_eq!(args.config, PathBuf::from("config.yaml"));
            }
            _ => panic!("Expected Run variant"),
        }
    }
}
