//! Battalion YAML configuration types for multi-agent orchestration
//!
//! This module defines configuration schemas for four Battalion orchestration patterns:
//! - **Formation**: Sequential execution (output N → input N+1)
//! - **Phalanx**: Concurrent execution with result aggregation
//! - **Campaign**: Graph/DAG-based conditional routing
//! - **Chain of Command**: Hierarchical delegation with commander
//!
//! # Example Formation Config
//!
//! ```yaml
//! type: formation
//! name: "document-pipeline"
//! agents:
//!   - name: "analyzer"
//!     system_prompt: "Analyze the input"
//!     model: "gpt-4"
//!     provider:
//!       type: "openai"
//!   - name: "summarizer"
//!     system_prompt: "Summarize the analysis"
//!     model: "gpt-4"
//!     provider:
//!       type: "openai"
//! pass_output_to_next: true
//! ```

use crate::cli::output::errors::CliError;
use serde::{Deserialize, Serialize};

/// Battalion YAML configuration with type discriminator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BattalionYamlConfig {
    #[serde(rename = "formation")]
    Formation(FormationConfig),

    #[serde(rename = "phalanx")]
    Phalanx(PhalanxConfig),

    #[serde(rename = "campaign")]
    Campaign(CampaignConfig),

    #[serde(rename = "chain-of-command")]
    ChainOfCommand(ChainOfCommandConfig),
}

impl BattalionYamlConfig {
    /// Get the battalion type as a string
    pub fn battalion_type(&self) -> &str {
        match self {
            BattalionYamlConfig::Formation(_) => "formation",
            BattalionYamlConfig::Phalanx(_) => "phalanx",
            BattalionYamlConfig::Campaign(_) => "campaign",
            BattalionYamlConfig::ChainOfCommand(_) => "chain-of-command",
        }
    }

    /// Validate the battalion configuration
    pub fn validate(&self) -> Result<(), CliError> {
        match self {
            BattalionYamlConfig::Formation(config) => config.validate(),
            BattalionYamlConfig::Phalanx(config) => config.validate(),
            BattalionYamlConfig::Campaign(config) => config.validate(),
            BattalionYamlConfig::ChainOfCommand(config) => config.validate(),
        }
    }
}

/// Formation configuration - Sequential execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationConfig {
    /// Name of the formation
    pub name: String,

    /// List of Paladins in execution order
    pub paladins: Vec<PaladinReference>,

    /// Whether to pass output from one Paladin to the next
    #[serde(default = "default_true")]
    pub pass_output_to_next: bool,
}

impl FormationConfig {
    fn validate(&self) -> Result<(), CliError> {
        if self.name.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "name".to_string(),
                message: "Campaign name is required".to_string(),
            });
        }

        if self.paladins.is_empty() {
            return Err(CliError::ValidationError {
                message: "Formation must have at least one Paladin".to_string(),
            });
        }

        Ok(())
    }
}

/// Phalanx configuration - Parallel execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhalanxConfig {
    /// Name of the phalanx
    pub name: String,

    /// List of Paladins to execute in parallel
    pub paladins: Vec<PaladinReference>,

    /// Optional inputs for each Paladin (if different)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
}

impl PhalanxConfig {
    fn validate(&self) -> Result<(), CliError> {
        if self.name.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "name".to_string(),
                message: "Phalanx name is required".to_string(),
            });
        }

        if self.paladins.is_empty() {
            return Err(CliError::ValidationError {
                message: "Phalanx must have at least one Paladin".to_string(),
            });
        }

        // If inputs provided, must match paladin count
        if let Some(inputs) = &self.inputs {
            if inputs.len() != self.paladins.len() {
                return Err(CliError::ValidationError {
                    message: format!(
                        "Number of inputs ({}) must match number of Paladins ({})",
                        inputs.len(),
                        self.paladins.len()
                    ),
                });
            }
        }

        Ok(())
    }
}

/// Campaign configuration - DAG-based execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignConfig {
    /// Name of the campaign
    pub name: String,

    /// Nodes in the DAG (Paladins)
    pub nodes: Vec<CampaignNode>,

    /// Edges defining dependencies
    pub edges: Vec<CampaignEdge>,

    /// Starting node ID
    pub start_node: String,
}

impl CampaignConfig {
    fn validate(&self) -> Result<(), CliError> {
        if self.name.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "name".to_string(),
                message: "Campaign name is required".to_string(),
            });
        }

        if self.nodes.is_empty() {
            return Err(CliError::ValidationError {
                message: "Campaign must have at least one node".to_string(),
            });
        }

        // Validate start_node exists
        if !self.nodes.iter().any(|n| n.id == self.start_node) {
            return Err(CliError::ValidationError {
                message: format!("Start node '{}' not found in nodes", self.start_node),
            });
        }

        // Validate all edge references exist
        for edge in &self.edges {
            if !self.nodes.iter().any(|n| n.id == edge.from) {
                return Err(CliError::ValidationError {
                    message: format!("Edge references non-existent node: {}", edge.from),
                });
            }
            if !self.nodes.iter().any(|n| n.id == edge.to) {
                return Err(CliError::ValidationError {
                    message: format!("Edge references non-existent node: {}", edge.to),
                });
            }
        }

        Ok(())
    }
}

/// Campaign node (Paladin in DAG)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignNode {
    /// Unique node ID
    pub id: String,

    /// Paladin reference
    pub paladin: PaladinReference,
}

/// Campaign edge (dependency between nodes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignEdge {
    /// Source node ID
    pub from: String,

    /// Target node ID
    pub to: String,
}

/// Chain of Command configuration - Hierarchical delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCommandConfig {
    /// Name of the chain
    pub name: String,

    /// Commander Paladin (top level)
    pub commander: PaladinReference,

    /// Delegate Paladins
    pub delegates: Vec<PaladinReference>,
}

impl ChainOfCommandConfig {
    fn validate(&self) -> Result<(), CliError> {
        if self.name.is_empty() {
            return Err(CliError::MissingRequiredField {
                field: "name".to_string(),
                message: "Chain of Command name is required".to_string(),
            });
        }

        if self.delegates.is_empty() {
            return Err(CliError::ValidationError {
                message: "Chain of Command must have at least one delegate".to_string(),
            });
        }

        Ok(())
    }
}

/// Reference to a Paladin (inline config or external file)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaladinReference {
    /// Path to external Paladin config file
    File { file: String },

    /// Inline Paladin configuration
    Inline(Box<crate::cli::config::paladin_config::PaladinYamlConfig>),
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation_valid() {
        let config = FormationConfig {
            name: "test".to_string(),
            paladins: vec![PaladinReference::File {
                file: "paladin1.yaml".to_string(),
            }],
            pass_output_to_next: true,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_formation_empty_name() {
        let config = FormationConfig {
            name: "".to_string(),
            paladins: vec![PaladinReference::File {
                file: "paladin1.yaml".to_string(),
            }],
            pass_output_to_next: true,
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::MissingRequiredField { .. })
        ));
    }

    #[test]
    fn test_phalanx_inputs_mismatch() {
        let config = PhalanxConfig {
            name: "test".to_string(),
            paladins: vec![
                PaladinReference::File {
                    file: "paladin1.yaml".to_string(),
                },
                PaladinReference::File {
                    file: "paladin2.yaml".to_string(),
                },
            ],
            inputs: Some(vec!["input1".to_string()]), // Only 1 input for 2 paladins
        };

        assert!(matches!(
            config.validate(),
            Err(CliError::ValidationError { .. })
        ));
    }
}
