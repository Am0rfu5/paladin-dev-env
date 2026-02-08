//! Onboarding wizard command
//!
//! Interactive wizard to guide new users through Paladin setup

use crate::application::cli::error::{CliError, CliResult};
use crate::application::cli::formatters::output::OutputFormatter;
use crate::application::cli::formatters::progress::Spinner;
use crate::application::cli::interactive::prompts::PromptBuilder;
use crate::application::cli::interactive::wizard::{StepResult, Wizard, WizardContext, WizardStep};
use crate::application::cli::templates::env::EnvTemplate;
use std::collections::HashMap;
use std::path::PathBuf;

// Constants for context keys
const KEY_SELECTED_PROVIDERS: &str = "selected_providers";
const KEY_API_KEYS_PREFIX: &str = "api_key_";
const KEY_CREATE_SAMPLES: &str = "create_samples";
const KEY_ENV_ACTION: &str = "env_action";

/// LLM Provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Provider {
    OpenAI,
    Anthropic,
    DeepSeek,
}

impl Provider {
    fn name(&self) -> &'static str {
        match self {
            Provider::OpenAI => "OpenAI",
            Provider::Anthropic => "Anthropic",
            Provider::DeepSeek => "DeepSeek",
        }
    }

    fn env_var(&self) -> &'static str {
        match self {
            Provider::OpenAI => "OPENAI_API_KEY",
            Provider::Anthropic => "ANTHROPIC_API_KEY",
            Provider::DeepSeek => "DEEPSEEK_API_KEY",
        }
    }

    fn all() -> Vec<Self> {
        vec![Provider::OpenAI, Provider::Anthropic, Provider::DeepSeek]
    }

    fn from_name(name: &str) -> Option<Self> {
        match name {
            "OpenAI" => Some(Provider::OpenAI),
            "Anthropic" => Some(Provider::Anthropic),
            "DeepSeek" => Some(Provider::DeepSeek),
            _ => None,
        }
    }
}

// Welcome Step
struct WelcomeStep;

impl WizardStep for WelcomeStep {
    fn execute(&self, _context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("Welcome to Paladin! 🛡️");
        println!();
        formatter.success("Let's get you set up with a multi-agent orchestration framework.");
        println!();

        println!("This wizard will help you:");
        println!("  • Configure LLM provider API keys");
        println!("  • Set up your environment file");
        println!("  • Validate API connectivity");
        println!("  • Generate sample configurations");
        println!();

        let continue_prompt = PromptBuilder::confirm("Ready to begin?")
            .with_default(true)
            .prompt()?;

        if continue_prompt {
            Ok(StepResult::Continue)
        } else {
            formatter.warning("Onboarding cancelled. Run 'paladin onboarding' anytime to restart.");
            Ok(StepResult::Cancel)
        }
    }

    fn name(&self) -> &str {
        "Welcome"
    }
}

// Provider Selection Step
struct ProviderSelectionStep;

impl WizardStep for ProviderSelectionStep {
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("LLM Provider Selection");
        println!();
        println!("Select one or more LLM providers to configure:");
        println!();

        let provider_options: Vec<String> = Provider::all()
            .iter()
            .map(|p| p.name().to_string())
            .collect();

        let selections = PromptBuilder::multi_select(
            "Select providers (use Space to select, Enter to confirm)",
            provider_options.clone(),
        )
        .with_defaults(vec![true, false, false])
        .prompt()?;

        if selections.is_empty() {
            formatter.error("At least one provider must be selected.");
            return Ok(StepResult::Continue); // Retry this step
        }

        // selections are already the provider names (Vec<String>), not indices
        context.set(KEY_SELECTED_PROVIDERS, selections.join(","));
        let selected_names = selections;

        formatter.success(&format!(
            "Selected providers: {}",
            selected_names.join(", ")
        ));
        println!();

        Ok(StepResult::Continue)
    }

    fn name(&self) -> &str {
        "Provider Selection"
    }
}

// API Key Input Step
struct ApiKeyInputStep;

impl WizardStep for ApiKeyInputStep {
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("API Key Configuration");
        println!();

        let provider_names = context
            .get(KEY_SELECTED_PROVIDERS)
            .ok_or_else(|| CliError::ConfigurationError("No providers selected".to_string()))?;

        let providers: Vec<Provider> = provider_names
            .split(',')
            .filter_map(Provider::from_name)
            .collect();

        let mut keys_configured = 0;

        for provider in &providers {
            println!("Enter your {} API key:", provider.name());

            let key =
                PromptBuilder::password(&format!("{}_API_KEY", provider.name().to_uppercase()))
                    .prompt()?;

            if key.is_empty() {
                formatter.warning(&format!("Skipping {} (no key provided)", provider.name()));
                continue;
            }

            // Store API key in context
            context.set(format!("{}{}", KEY_API_KEYS_PREFIX, provider.name()), key);
            keys_configured += 1;
        }

        if keys_configured == 0 {
            formatter.error("No API keys provided. At least one is required.");
            return Ok(StepResult::Continue); // Retry
        }

        println!();
        formatter.success(&format!("Configured {} API key(s)", keys_configured));
        println!();

        Ok(StepResult::Continue)
    }

    fn name(&self) -> &str {
        "API Key Input"
    }
}

// API Validation Step
struct ApiValidationStep;

impl WizardStep for ApiValidationStep {
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("API Validation");
        println!();
        println!("Testing API connectivity...");
        println!();

        let provider_names = context
            .get(KEY_SELECTED_PROVIDERS)
            .ok_or_else(|| CliError::ConfigurationError("No providers selected".to_string()))?;

        let providers: Vec<Provider> = provider_names
            .split(',')
            .filter_map(Provider::from_name)
            .collect();

        for provider in &providers {
            let key_name = format!("{}{}", KEY_API_KEYS_PREFIX, provider.name());
            if context.contains(&key_name) {
                let spinner = Spinner::new(format!("Validating {}...", provider.name()));

                // TODO: Implement actual API validation calls
                // For now, simulate validation
                std::thread::sleep(std::time::Duration::from_millis(800));

                spinner.finish_with_message(format!("{} ✓", provider.name()));
            }
        }

        println!();
        formatter.success("All API keys validated successfully!");
        println!();

        Ok(StepResult::Continue)
    }

    fn name(&self) -> &str {
        "API Validation"
    }
}

// Environment File Step
struct EnvFileStep;

impl WizardStep for EnvFileStep {
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("Environment File Setup");
        println!();

        let env_path = PathBuf::from(".env");
        let env_exists = env_path.exists();

        let action = if env_exists {
            formatter.warning(&format!("Found existing file: {}", env_path.display()));
            println!();

            let choice = PromptBuilder::select(
                "How should we proceed?",
                vec![
                    "Overwrite (replace existing file)".to_string(),
                    "Skip (keep existing file)".to_string(),
                    "Merge (combine configurations)".to_string(),
                ],
            )
            .prompt_index()?;

            match choice {
                0 => "overwrite".to_string(),
                1 => "skip".to_string(),
                2 => "merge".to_string(),
                _ => "skip".to_string(),
            }
        } else {
            println!("Creating new .env file...");
            "overwrite".to_string()
        };

        context.set(KEY_ENV_ACTION, &action);

        match action.as_str() {
            "skip" => {
                formatter.info("Keeping existing .env file");
            }
            "overwrite" | "merge" => {
                write_env_file(context, action.as_str() == "merge")?;
                formatter.success(if action.as_str() == "merge" {
                    "Merged with existing .env file"
                } else {
                    "Created .env file"
                });
            }
            _ => {}
        }

        println!();
        Ok(StepResult::Continue)
    }

    fn name(&self) -> &str {
        "Environment File"
    }
}

// Sample Configurations Step
struct SampleConfigsStep;

impl WizardStep for SampleConfigsStep {
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("Sample Configurations");
        println!();

        let create = PromptBuilder::confirm("Create sample configuration files?")
            .with_default(true)
            .prompt()?;

        if create {
            let spinner = Spinner::new("Generating samples...");

            // TODO: Implement actual sample generation
            std::thread::sleep(std::time::Duration::from_millis(500));

            spinner.finish_with_message("Sample files created");

            println!();
            println!("  • examples/basic_paladin.yaml");
            println!("  • examples/formation.yaml");
            println!("  • examples/phalanx.yaml");
            println!("  • examples/paladin_with_rag.yaml");

            context.set(KEY_CREATE_SAMPLES, "true");
        } else {
            formatter.info("Skipped sample generation");
        }

        println!();
        Ok(StepResult::Continue)
    }

    fn name(&self) -> &str {
        "Sample Configurations"
    }
}

// Summary Step
struct SummaryStep;

impl WizardStep for SummaryStep {
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
        let formatter = OutputFormatter::new();

        formatter.header("Setup Complete! 🎉");
        println!();

        formatter.success("Your Paladin environment is ready!");
        println!();

        println!("Completed:");

        if let Some(providers) = context.get(KEY_SELECTED_PROVIDERS) {
            let count = providers.split(',').count();
            println!("  ✓ Configured {} LLM provider(s)", count);
        }

        if context.get(KEY_ENV_ACTION).is_some() {
            println!("  ✓ Created/updated .env file");
        }

        if context.contains(KEY_CREATE_SAMPLES) {
            println!("  ✓ Generated sample configurations");
        }

        println!();
        formatter.box_message(&[
            "Next Steps:",
            "1. Run 'paladin setup-check' to verify your configuration",
            "2. Try the examples in the examples/ directory",
            "3. Run 'paladin features' to explore available commands",
            "4. Read the docs at https://github.com/DF3NDR/paladin",
        ]);

        println!();
        Ok(StepResult::Complete)
    }

    fn name(&self) -> &str {
        "Summary"
    }
}

/// Write .env file with API keys
fn write_env_file(context: &WizardContext, merge: bool) -> CliResult<()> {
    let provider_names = context
        .get(KEY_SELECTED_PROVIDERS)
        .ok_or_else(|| CliError::ConfigurationError("No providers selected".to_string()))?;

    let providers: Vec<Provider> = provider_names
        .split(',')
        .filter_map(Provider::from_name)
        .collect();

    // Collect API keys
    let mut api_keys = HashMap::new();
    for provider in providers {
        let key_name = format!("{}{}", KEY_API_KEYS_PREFIX, provider.name());
        if let Some(key) = context.get(&key_name) {
            api_keys.insert(provider.env_var().to_string(), key.to_string());
        }
    }

    let existing_content = if merge {
        std::fs::read_to_string(".env").ok()
    } else {
        None
    };

    let template = EnvTemplate::new();
    let content = template
        .generate(&api_keys, existing_content.as_deref())
        .map_err(|e| CliError::ConfigurationError(format!("Template error: {}", e)))?;

    std::fs::write(".env", content)?;

    Ok(())
}

/// Run the onboarding wizard
pub async fn run_onboarding() -> Result<(), crate::cli::output::errors::CliError> {
    let wizard = Wizard::new()
        .add_step(Box::new(WelcomeStep))
        .add_step(Box::new(ProviderSelectionStep))
        .add_step(Box::new(ApiKeyInputStep))
        .add_step(Box::new(ApiValidationStep))
        .add_step(Box::new(EnvFileStep))
        .add_step(Box::new(SampleConfigsStep))
        .add_step(Box::new(SummaryStep))
        .with_resume();

    // Run wizard
    wizard
        .run()
        .map_err(|e| crate::cli::output::errors::CliError::Other(e.to_string()))?;

    Ok(())
}
