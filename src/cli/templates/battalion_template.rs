//! Battalion configuration template generator

use crate::cli::output::errors::CliError;

/// Generate a Battalion YAML configuration template
///
/// Creates a template for the specified battalion type with inline documentation
/// and example Paladin references.
///
/// # Arguments
///
/// * `name` - Name for the Battalion
/// * `battalion_type` - Type of battalion (formation, phalanx, campaign, chain-of-command)
///
/// # Returns
///
/// Returns a formatted YAML string with comprehensive documentation
pub fn generate_battalion_template(name: &str, battalion_type: &str) -> Result<String, CliError> {
    match battalion_type {
        "formation" => Ok(generate_formation_template(name)),
        "phalanx" => Ok(generate_phalanx_template(name)),
        "campaign" => Ok(generate_campaign_template(name)),
        "chain-of-command" => Ok(generate_chain_of_command_template(name)),
        _ => Err(CliError::InvalidFieldValue {
            field: "battalion_type".to_string(),
            message: format!(
                "must be one of: formation, phalanx, campaign, chain-of-command. Got: {}",
                battalion_type
            ),
        }),
    }
}

/// Generate Formation template (sequential execution)
fn generate_formation_template(name: &str) -> String {
    format!(
        r#"# Formation Battalion Configuration
# Sequential execution: Output from one Paladin flows to the next

# Battalion type (required)
type: formation

# Formation name
name: "{}"

# List of Paladins to execute in sequence
# Each Paladin can be defined inline or reference an external config file
paladins:
  # Option 1: Reference external Paladin config file
  - file: "paladin1.yaml"
  
  # Option 2: Inline Paladin definition
  - inline:
      name: "AnalyzerPaladin"
      system_prompt: |
        You are an expert analyzer. Review the input and provide structured analysis.
        Focus on key points, patterns, and insights.
      model: "gpt-4"
      temperature: 0.7
      max_loops: 3
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai
  
  # Option 3: Another external reference
  - file: "paladin2.yaml"

# Pass output from each Paladin to the next in sequence
# Set to false if each Paladin should receive the original input
pass_output_to_next: true

# Example usage:
# paladin battalion run -c formation.yaml -i "Analyze this text..."
"#,
        name
    )
}

/// Generate Phalanx template (parallel execution)
fn generate_phalanx_template(name: &str) -> String {
    format!(
        r#"# Phalanx Battalion Configuration
# Parallel execution: All Paladins execute simultaneously

# Battalion type (required)
type: phalanx

# Phalanx name
name: "{}"

# List of Paladins to execute in parallel
paladins:
  - inline:
      name: "SummarizerPaladin"
      system_prompt: |
        You are an expert summarizer. Create a concise summary of the input.
      model: "gpt-4"
      temperature: 0.5
      max_loops: 2
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai
  
  - inline:
      name: "SentimentAnalyzer"
      system_prompt: |
        You are a sentiment analysis expert. Analyze the sentiment and tone.
      model: "gpt-4"
      temperature: 0.3
      max_loops: 2
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai
  
  - inline:
      name: "KeywordExtractor"
      system_prompt: |
        You are a keyword extraction expert. Extract key terms and concepts.
      model: "gpt-4"
      temperature: 0.3
      max_loops: 2
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai

# Optional: Different inputs for each Paladin
# If omitted, all Paladins receive the same input
# inputs:
#   - "Summarize this text"
#   - "Analyze sentiment"
#   - "Extract keywords"

# Example usage:
# paladin battalion run -c phalanx.yaml -i "Process this document..."
"#,
        name
    )
}

/// Generate Campaign template (DAG/graph execution)
fn generate_campaign_template(name: &str) -> String {
    format!(
        r#"# Campaign Battalion Configuration
# Graph-based execution: Directed Acyclic Graph (DAG) of Paladins

# Battalion type (required)
type: campaign

# Campaign name
name: "{}"

# Nodes: Each node represents a Paladin in the graph
nodes:
  - id: "intake"
    paladin:
      inline:
        name: "IntakePaladin"
        system_prompt: |
          You are an intake processor. Validate and structure the input.
        model: "gpt-4"
        temperature: 0.3
        max_loops: 2
        timeout_seconds: 300
        stop_words: []
        provider:
          type: openai
  
  - id: "analyzer"
    paladin:
      inline:
        name: "AnalyzerPaladin"
        system_prompt: |
          You are an analyzer. Process structured data and extract insights.
        model: "gpt-4"
        temperature: 0.5
        max_loops: 3
        timeout_seconds: 300
        stop_words: []
        provider:
          type: openai
  
  - id: "formatter"
    paladin:
      inline:
        name: "FormatterPaladin"
        system_prompt: |
          You are a formatter. Take analysis results and format for final output.
        model: "gpt-4"
        temperature: 0.3
        max_loops: 2
        timeout_seconds: 300
        stop_words: []
        provider:
          type: openai

# Edges: Define dependencies between nodes
# Execution follows topological order respecting these dependencies
edges:
  - from: "intake"
    to: "analyzer"
  - from: "analyzer"
    to: "formatter"

# Start node: Where execution begins
start_node: "intake"

# Example usage:
# paladin battalion run -c campaign.yaml -i "Process this workflow..."
"#,
        name
    )
}

/// Generate Chain of Command template (hierarchical delegation)
fn generate_chain_of_command_template(name: &str) -> String {
    format!(
        r#"# Chain of Command Battalion Configuration
# Hierarchical execution: Commander delegates to specialized Paladins

# Battalion type (required)
type: chain_of_command

# Chain name
name: "{}"

# Commander: Top-level Paladin that orchestrates delegation
commander:
  inline:
    name: "CommanderPaladin"
    system_prompt: |
      You are a commander agent. Analyze the task and decide which specialized
      agents should handle different aspects. Break down complex tasks and
      delegate appropriately.
    model: "gpt-4"
    temperature: 0.7
    max_loops: 5
    timeout_seconds: 600
    stop_words: []
    provider:
      type: openai

# Delegates: Specialized Paladins available for delegation
delegates:
  - inline:
      name: "ResearchDelegate"
      system_prompt: |
        You are a research specialist. Gather information and provide detailed
        research on assigned topics.
      model: "gpt-4"
      temperature: 0.5
      max_loops: 3
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai
  
  - inline:
      name: "WritingDelegate"
      system_prompt: |
        You are a writing specialist. Create well-structured documents and
        content based on research and requirements.
      model: "gpt-4"
      temperature: 0.7
      max_loops: 3
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai
  
  - inline:
      name: "ReviewDelegate"
      system_prompt: |
        You are a review specialist. Evaluate quality, accuracy, and
        completeness of work products.
      model: "gpt-4"
      temperature: 0.3
      max_loops: 2
      timeout_seconds: 300
      stop_words: []
      provider:
        type: openai

# Example usage:
# paladin battalion run -c chain-of-command.yaml -i "Create a research report on AI safety"
"#,
        name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_formation_template() {
        let template = generate_battalion_template("TestFormation", "formation");
        assert!(template.is_ok());
        let yaml = template.unwrap();
        assert!(yaml.contains("type: formation"));
        assert!(yaml.contains("name: \"TestFormation\""));
        assert!(yaml.contains("pass_output_to_next: true"));
    }

    #[test]
    fn test_generate_phalanx_template() {
        let template = generate_battalion_template("TestPhalanx", "phalanx");
        assert!(template.is_ok());
        let yaml = template.unwrap();
        assert!(yaml.contains("type: phalanx"));
        assert!(yaml.contains("name: \"TestPhalanx\""));
        assert!(yaml.contains("SummarizerPaladin"));
    }

    #[test]
    fn test_generate_campaign_template() {
        let template = generate_battalion_template("TestCampaign", "campaign");
        assert!(template.is_ok());
        let yaml = template.unwrap();
        assert!(yaml.contains("type: campaign"));
        assert!(yaml.contains("name: \"TestCampaign\""));
        assert!(yaml.contains("nodes:"));
        assert!(yaml.contains("edges:"));
        assert!(yaml.contains("start_node: \"intake\""));
    }

    #[test]
    fn test_generate_chain_of_command_template() {
        let template = generate_battalion_template("TestChain", "chain-of-command");
        assert!(template.is_ok());
        let yaml = template.unwrap();
        assert!(yaml.contains("type: chain_of_command"));
        assert!(yaml.contains("name: \"TestChain\""));
        assert!(yaml.contains("commander:"));
        assert!(yaml.contains("delegates:"));
    }

    #[test]
    fn test_invalid_battalion_type() {
        let result = generate_battalion_template("Test", "invalid");
        assert!(result.is_err());
        match result {
            Err(CliError::InvalidFieldValue { field, .. }) => {
                assert_eq!(field, "battalion_type");
            }
            _ => panic!("Expected InvalidFieldValue error"),
        }
    }

    #[test]
    fn test_all_templates_are_valid_yaml() {
        let types = vec!["formation", "phalanx", "campaign", "chain-of-command"];
        for battalion_type in types {
            let template = generate_battalion_template("Test", battalion_type).unwrap();
            let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
            assert!(
                parsed.is_ok(),
                "Template for {} should be valid YAML",
                battalion_type
            );
        }
    }
}
