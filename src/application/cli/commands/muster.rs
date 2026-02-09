//! Muster command - LLM-powered battalion generation
//!
//! This command uses an LLM to analyze a task description and automatically
//! generate an appropriate battalion configuration with recommended patterns
//! and agent roles.

use crate::application::cli::error::CliError;
use crate::application::cli::formatters::output::OutputFormatter;
use crate::application::cli::interactive::prompts::PromptBuilder;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::fs;

/// Battalion pattern types that can be recommended
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BattalionPattern {
    Formation,
    Phalanx,
    Campaign,
    ChainOfCommand,
    Conclave,
    Maneuver,
}

impl BattalionPattern {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Formation => "formation",
            Self::Phalanx => "phalanx",
            Self::Campaign => "campaign",
            Self::ChainOfCommand => "chain-of-command",
            Self::Conclave => "conclave",
            Self::Maneuver => "maneuver",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "formation" => Some(Self::Formation),
            "phalanx" => Some(Self::Phalanx),
            "campaign" => Some(Self::Campaign),
            "chain-of-command" | "chain_of_command" | "chainofcommand" => {
                Some(Self::ChainOfCommand)
            }
            "conclave" => Some(Self::Conclave),
            "maneuver" => Some(Self::Maneuver),
            _ => None,
        }
    }
}

/// Agent role recommendation from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRole {
    pub name: String,
    pub role: String,
    pub system_prompt: String,
}

/// Task analysis result from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalysis {
    pub recommended_pattern: BattalionPattern,
    pub reasoning: String,
    pub agents: Vec<AgentRole>,
    pub battalion_name: String,
}

/// Generate battalion configuration from task description
pub async fn run_muster(
    task: Option<String>,
    output: Option<String>,
    execute: bool,
    provider: Option<String>,
    model: Option<String>,
    no_review: bool,
) -> Result<(), CliError> {
    let formatter = OutputFormatter::new();
    formatter.header("🏰 Muster - Battalion Configuration Generator");

    // Step 1: Get task description
    let task_description = get_task_description(task, &formatter).await?;

    // Step 2: Analyze task with LLM
    formatter.info("Analyzing task requirements with AI...");
    let analysis =
        match analyze_task_with_llm(&task_description, provider.as_deref(), model.as_deref()).await
        {
            Ok(analysis) => analysis,
            Err(e) => {
                formatter.warning(&format!(
                    "LLM analysis failed: {}. Using template fallback.",
                    e
                ));
                analyze_task_with_template(&task_description)?
            }
        };

    // Step 3: Display analysis
    formatter.section("📋 Analysis Results");
    formatter.info(&format!(
        "Pattern: {}",
        analysis.recommended_pattern.as_str()
    ));
    formatter.info(&format!("Battalion: {}", analysis.battalion_name));
    formatter.info(&format!("Reasoning: {}", analysis.reasoning));
    formatter.info(&format!("Agents: {} recommended", analysis.agents.len()));

    // Step 4: Generate YAML configuration
    formatter.info("Generating battalion configuration...");
    let yaml_config = generate_battalion_yaml(&analysis)?;

    // Step 5: Review (unless --no-review)
    let final_config = if no_review {
        yaml_config
    } else {
        review_configuration(&yaml_config, &formatter)?
    };

    // Step 6: Save configuration
    let output_path = determine_output_path(output, &analysis.battalion_name)?;
    save_configuration(&final_config, &output_path).await?;
    formatter.success(&format!("Configuration saved to: {}", output_path));

    // Step 7: Execute immediately if requested
    if execute {
        formatter.info("Executing battalion...");
        execute_battalion(&output_path).await?;
    } else {
        formatter.info(&format!(
            "\nTo run this battalion: paladin battalion run -c {}",
            output_path
        ));
    }

    Ok(())
}

/// Get task description from user or command line
async fn get_task_description(
    task: Option<String>,
    formatter: &OutputFormatter,
) -> Result<String, CliError> {
    if let Some(task) = task {
        Ok(task)
    } else {
        formatter.section("📝 Task Description");
        PromptBuilder::input("Describe the task you want to accomplish")
            .allow_empty(false)
            .prompt()
    }
}

/// Analyze task using LLM
async fn analyze_task_with_llm(
    _task_description: &str,
    _provider: Option<&str>,
    _model: Option<&str>,
) -> Result<TaskAnalysis, CliError> {
    // For now, return error to trigger template fallback
    // Full LLM integration would be implemented here
    Err(CliError::execution(
        "LLM integration in muster - using template fallback",
    ))
}

/// Build the LLM system prompt for task analysis
#[allow(dead_code)]
fn build_analysis_prompt() -> String {
    r#"You are an expert in multi-agent orchestration patterns. Analyze the given task and recommend the best battalion pattern and agent roles.

Available Patterns:
- Formation: Sequential execution where output flows from one agent to the next. Use for tasks requiring step-by-step processing (e.g., analyze → summarize → format).
- Phalanx: Parallel execution where all agents process the same input simultaneously. Use for tasks requiring multiple perspectives or independent analyses.
- Campaign: Graph-based DAG execution with conditional routing. Use for complex workflows with branching logic.
- ChainOfCommand: Hierarchical delegation where a coordinator routes to specialists. Use for tasks requiring dynamic delegation.
- Conclave: Expert panel discussion with voting. Use for decision-making requiring consensus.
- Maneuver: Dynamic workflow adaptation. Use for tasks requiring runtime flow changes.

Respond with JSON in this exact format:
{
  "recommended_pattern": "formation|phalanx|campaign|chainofcommand|conclave|maneuver",
  "reasoning": "Brief explanation of why this pattern fits",
  "battalion_name": "descriptive_name_for_battalion",
  "agents": [
    {
      "name": "AgentName",
      "role": "Brief role description",
      "system_prompt": "Detailed system prompt defining agent behavior"
    }
  ]
}

Guidelines:
- Choose the simplest pattern that accomplishes the task
- Formation for 2-4 sequential steps
- Phalanx for 2-5 parallel analyses
- Recommend 2-5 agents typically
- System prompts should be specific and actionable"#
        .to_string()
}

/// Fallback task analysis using template matching
fn analyze_task_with_template(task_description: &str) -> Result<TaskAnalysis, CliError> {
    let task_lower = task_description.to_lowercase();

    // Detect sequential patterns
    if task_lower.contains("then")
        || task_lower.contains("after")
        || task_lower.contains("followed by")
        || (task_lower.contains("analyze") && task_lower.contains("summarize"))
    {
        return Ok(TaskAnalysis {
            recommended_pattern: BattalionPattern::Formation,
            reasoning: "Detected sequential steps in task description".to_string(),
            battalion_name: "sequential_workflow".to_string(),
            agents: vec![
                AgentRole {
                    name: "Analyzer".to_string(),
                    role: "Initial Analysis".to_string(),
                    system_prompt: "Analyze the input and identify key points.".to_string(),
                },
                AgentRole {
                    name: "Processor".to_string(),
                    role: "Process Results".to_string(),
                    system_prompt: "Process the analysis and generate output.".to_string(),
                },
            ],
        });
    }

    // Detect parallel patterns
    if task_lower.contains("multiple")
        || task_lower.contains("different perspectives")
        || task_lower.contains("various")
        || task_lower.contains("compare")
    {
        return Ok(TaskAnalysis {
            recommended_pattern: BattalionPattern::Phalanx,
            reasoning: "Detected need for multiple parallel analyses".to_string(),
            battalion_name: "parallel_analysis".to_string(),
            agents: vec![
                AgentRole {
                    name: "Analyst1".to_string(),
                    role: "First Perspective".to_string(),
                    system_prompt: "Analyze from first perspective.".to_string(),
                },
                AgentRole {
                    name: "Analyst2".to_string(),
                    role: "Second Perspective".to_string(),
                    system_prompt: "Analyze from second perspective.".to_string(),
                },
            ],
        });
    }

    // Detect discussion patterns
    if task_lower.contains("discuss")
        || task_lower.contains("debate")
        || task_lower.contains("consensus")
        || task_lower.contains("decide")
    {
        return Ok(TaskAnalysis {
            recommended_pattern: BattalionPattern::Conclave,
            reasoning: "Detected need for group discussion and consensus".to_string(),
            battalion_name: "discussion_panel".to_string(),
            agents: vec![
                AgentRole {
                    name: "Advocate".to_string(),
                    role: "Supporting Arguments".to_string(),
                    system_prompt: "Present arguments in favor.".to_string(),
                },
                AgentRole {
                    name: "Critic".to_string(),
                    role: "Critical Analysis".to_string(),
                    system_prompt: "Present critical analysis.".to_string(),
                },
                AgentRole {
                    name: "Moderator".to_string(),
                    role: "Facilitate Discussion".to_string(),
                    system_prompt: "Moderate the discussion and synthesize.".to_string(),
                },
            ],
        });
    }

    // Default to formation
    Ok(TaskAnalysis {
        recommended_pattern: BattalionPattern::Formation,
        reasoning: "General purpose sequential processing".to_string(),
        battalion_name: "general_workflow".to_string(),
        agents: vec![AgentRole {
            name: "Processor".to_string(),
            role: "Process Task".to_string(),
            system_prompt: format!("Process the following task: {}", task_description),
        }],
    })
}

/// Generate YAML configuration from analysis
fn generate_battalion_yaml(analysis: &TaskAnalysis) -> Result<String, CliError> {
    let mut yaml = String::new();

    // Header comment
    yaml.push_str(&format!(
        "# Battalion Configuration: {}\n",
        analysis.battalion_name
    ));
    yaml.push_str(&format!(
        "# Pattern: {}\n",
        analysis.recommended_pattern.as_str()
    ));
    yaml.push_str(&format!("# Reasoning: {}\n", analysis.reasoning));
    yaml.push_str(&format!(
        "# Generated by: paladin muster at {}\n\n",
        Utc::now()
    ));

    // Battalion type and name
    yaml.push_str(&format!(
        "type: {}\n",
        analysis.recommended_pattern.as_str()
    ));
    yaml.push_str(&format!("name: \"{}\"\n\n", analysis.battalion_name));

    // Agents
    yaml.push_str("paladins:\n");
    for agent in &analysis.agents {
        yaml.push_str("  - inline:\n");
        yaml.push_str(&format!("      name: \"{}\"\n", agent.name));
        yaml.push_str("      system_prompt: |\n");
        for line in agent.system_prompt.lines() {
            yaml.push_str(&format!("        {}\n", line));
        }
        yaml.push_str("      model: \"gpt-4\"\n");
        yaml.push_str("      temperature: 0.7\n");
        yaml.push_str("      max_loops: 3\n");
        yaml.push_str("      timeout_seconds: 300\n");
        yaml.push_str("      stop_words: []\n");
        yaml.push_str("      provider:\n");
        yaml.push_str("        type: openai\n");
        yaml.push('\n');
    }

    // Pattern-specific configuration
    match analysis.recommended_pattern {
        BattalionPattern::Formation => {
            yaml.push_str("# Pass output from each agent to the next\n");
            yaml.push_str("pass_output_to_next: true\n");
        }
        BattalionPattern::Phalanx => {
            yaml.push_str("# All agents execute in parallel on same input\n");
            yaml.push_str("aggregate_results: true\n");
        }
        _ => {}
    }

    Ok(yaml)
}

/// Review and optionally edit configuration
fn review_configuration(
    yaml_config: &str,
    formatter: &OutputFormatter,
) -> Result<String, CliError> {
    formatter.section("📄 Generated Configuration");
    println!("\n{}\n", yaml_config);

    if PromptBuilder::confirm("Accept this configuration?")
        .with_default(true)
        .prompt()?
    {
        Ok(yaml_config.to_string())
    } else if PromptBuilder::confirm("Edit configuration?")
        .with_default(true)
        .prompt()?
    {
        // In full implementation, would open editor
        formatter.warning("Configuration editing not yet implemented. Using generated config.");
        Ok(yaml_config.to_string())
    } else {
        Err(CliError::Cancelled)
    }
}

/// Determine output file path
fn determine_output_path(output: Option<String>, battalion_name: &str) -> Result<String, CliError> {
    if let Some(path) = output {
        Ok(path)
    } else {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        Ok(format!("muster_{}_{}.yaml", battalion_name, timestamp))
    }
}

/// Save configuration to file
async fn save_configuration(config: &str, path: &str) -> Result<(), CliError> {
    fs::write(path, config)
        .await
        .map_err(|e| CliError::IoError {
            message: format!("Failed to write configuration to {}", path),
            source: e,
        })?;
    Ok(())
}

/// Execute battalion immediately
async fn execute_battalion(_config_path: &str) -> Result<(), CliError> {
    // In full implementation, would load and execute the battalion
    // For now, just indicate it's not implemented
    Err(CliError::execution(
        "Immediate battalion execution - use 'paladin battalion run' manually",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battalion_pattern_from_str() {
        assert_eq!(
            BattalionPattern::parse("formation"),
            Some(BattalionPattern::Formation)
        );
        assert_eq!(
            BattalionPattern::parse("phalanx"),
            Some(BattalionPattern::Phalanx)
        );
        assert_eq!(
            BattalionPattern::parse("chain-of-command"),
            Some(BattalionPattern::ChainOfCommand)
        );
        assert_eq!(BattalionPattern::parse("invalid"), None);
    }

    #[test]
    fn test_battalion_pattern_as_str() {
        assert_eq!(BattalionPattern::Formation.as_str(), "formation");
        assert_eq!(BattalionPattern::Phalanx.as_str(), "phalanx");
        assert_eq!(
            BattalionPattern::ChainOfCommand.as_str(),
            "chain-of-command"
        );
    }

    #[test]
    fn test_template_fallback_sequential() {
        let result =
            analyze_task_with_template("First analyze the text, then summarize the key points")
                .unwrap();
        assert_eq!(result.recommended_pattern, BattalionPattern::Formation);
        assert_eq!(result.agents.len(), 2);
    }

    #[test]
    fn test_template_fallback_parallel() {
        let result =
            analyze_task_with_template("Compare multiple perspectives on this topic").unwrap();
        assert_eq!(result.recommended_pattern, BattalionPattern::Phalanx);
        assert_eq!(result.agents.len(), 2);
    }

    #[test]
    fn test_template_fallback_discussion() {
        let result =
            analyze_task_with_template("Discuss and reach consensus on the proposal").unwrap();
        assert_eq!(result.recommended_pattern, BattalionPattern::Conclave);
        assert_eq!(result.agents.len(), 3);
    }

    #[test]
    fn test_template_fallback_default() {
        let result = analyze_task_with_template("Process this generic task").unwrap();
        assert_eq!(result.recommended_pattern, BattalionPattern::Formation);
        assert!(!result.agents.is_empty());
    }

    #[test]
    fn test_generate_battalion_yaml_formation() {
        let analysis = TaskAnalysis {
            recommended_pattern: BattalionPattern::Formation,
            reasoning: "Test reasoning".to_string(),
            battalion_name: "test_battalion".to_string(),
            agents: vec![AgentRole {
                name: "TestAgent".to_string(),
                role: "Test Role".to_string(),
                system_prompt: "Test prompt".to_string(),
            }],
        };

        let yaml = generate_battalion_yaml(&analysis).unwrap();
        assert!(yaml.contains("type: formation"));
        assert!(yaml.contains("name: \"test_battalion\""));
        assert!(yaml.contains("TestAgent"));
        assert!(yaml.contains("pass_output_to_next: true"));
    }

    #[test]
    fn test_generate_battalion_yaml_phalanx() {
        let analysis = TaskAnalysis {
            recommended_pattern: BattalionPattern::Phalanx,
            reasoning: "Test reasoning".to_string(),
            battalion_name: "test_parallel".to_string(),
            agents: vec![
                AgentRole {
                    name: "Agent1".to_string(),
                    role: "Role1".to_string(),
                    system_prompt: "Prompt1".to_string(),
                },
                AgentRole {
                    name: "Agent2".to_string(),
                    role: "Role2".to_string(),
                    system_prompt: "Prompt2".to_string(),
                },
            ],
        };

        let yaml = generate_battalion_yaml(&analysis).unwrap();
        assert!(yaml.contains("type: phalanx"));
        assert!(yaml.contains("Agent1"));
        assert!(yaml.contains("Agent2"));
        assert!(yaml.contains("aggregate_results: true"));
    }

    #[test]
    fn test_determine_output_path_with_custom() {
        let result = determine_output_path(Some("custom.yaml".to_string()), "test").unwrap();
        assert_eq!(result, "custom.yaml");
    }

    #[test]
    fn test_determine_output_path_with_default() {
        let result = determine_output_path(None, "test_battalion").unwrap();
        assert!(result.starts_with("muster_test_battalion_"));
        assert!(result.ends_with(".yaml"));
    }

    #[test]
    fn test_build_analysis_prompt() {
        let prompt = build_analysis_prompt();
        assert!(prompt.contains("Formation"));
        assert!(prompt.contains("Phalanx"));
        assert!(prompt.contains("Campaign"));
        assert!(prompt.contains("JSON"));
    }
}
