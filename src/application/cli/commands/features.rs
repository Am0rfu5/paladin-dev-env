//! Features discovery command
//!
//! Displays available Paladin features, commands, orchestration patterns,
//! and memory systems with descriptions and documentation links.

use crate::application::cli::error::{CliError, CliResult};
use crate::application::cli::formatters::output::OutputFormatter;
use crate::application::cli::formatters::table::TableFormatter;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Feature category for organizing capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureCategory {
    Agent,
    Battalion,
    Orchestration,
    Memory,
    Utilities,
}

impl FeatureCategory {
    #[cfg(test)]
    fn all() -> Vec<Self> {
        vec![
            Self::Agent,
            Self::Battalion,
            Self::Orchestration,
            Self::Memory,
            Self::Utilities,
        ]
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "agent" => Some(Self::Agent),
            "battalion" => Some(Self::Battalion),
            "orchestration" => Some(Self::Orchestration),
            "memory" => Some(Self::Memory),
            "utilities" => Some(Self::Utilities),
            _ => None,
        }
    }
}

impl fmt::Display for FeatureCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent => write!(f, "Agent"),
            Self::Battalion => write!(f, "Battalion"),
            Self::Orchestration => write!(f, "Orchestration"),
            Self::Memory => write!(f, "Memory"),
            Self::Utilities => write!(f, "Utilities"),
        }
    }
}

/// Availability status of a feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureStatus {
    Available,
    RequiresFlag(#[serde(skip)] &'static str),
    Experimental,
}

impl fmt::Display for FeatureStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Available => write!(f, "Available"),
            Self::RequiresFlag(flag) => write!(f, "Requires: {}", flag),
            Self::Experimental => write!(f, "Experimental"),
        }
    }
}

/// A Paladin feature with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub category: FeatureCategory,
    pub description: String,
    pub status: FeatureStatus,
    pub doc_link: Option<String>,
}

impl Feature {
    fn new(
        name: impl Into<String>,
        category: FeatureCategory,
        description: impl Into<String>,
        status: FeatureStatus,
        doc_link: Option<impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            category,
            description: description.into(),
            status,
            doc_link: doc_link.map(|s| s.into()),
        }
    }
}

/// Registry of all available features
struct FeatureRegistry {
    features: Vec<Feature>,
}

impl FeatureRegistry {
    fn new() -> Self {
        let features = vec![
            // Agent features
            Feature::new(
                "Paladin",
                FeatureCategory::Agent,
                "Autonomous AI agent with reasoning loops, memory, and tool access",
                FeatureStatus::Available,
                Some::<&str>("docs/QUICKSTART.md"),
            ),
            Feature::new(
                "Dynamic Temperature",
                FeatureCategory::Agent,
                "Automatic temperature adjustment based on task complexity",
                FeatureStatus::Available,
                Some::<&str>("examples/dynamic_temperature.rs"),
            ),
            Feature::new(
                "Stop Words",
                FeatureCategory::Agent,
                "Configurable stop conditions to halt agent execution",
                FeatureStatus::Available,
                Some::<&str>("docs/QUICKSTART.md"),
            ),
            // Battalion orchestration patterns
            Feature::new(
                "Formation",
                FeatureCategory::Battalion,
                "Sequential execution: Output from agent N flows to agent N+1",
                FeatureStatus::Available,
                Some::<&str>("docs/BATTALION.md#formation"),
            ),
            Feature::new(
                "Phalanx",
                FeatureCategory::Battalion,
                "Parallel execution: All agents process input simultaneously",
                FeatureStatus::Available,
                Some::<&str>("docs/BATTALION.md#phalanx"),
            ),
            Feature::new(
                "Campaign",
                FeatureCategory::Battalion,
                "Graph-based DAG execution with conditional routing",
                FeatureStatus::Available,
                Some::<&str>("docs/BATTALION.md#campaign"),
            ),
            Feature::new(
                "Chain of Command",
                FeatureCategory::Battalion,
                "Hierarchical delegation with dynamic task routing",
                FeatureStatus::Available,
                Some::<&str>("docs/BATTALION.md#chain-of-command"),
            ),
            // Advanced orchestration
            Feature::new(
                "Conclave",
                FeatureCategory::Orchestration,
                "Expert panel discussion with voting and consensus mechanisms",
                FeatureStatus::Available,
                Some::<&str>("docs/COUNCIL.md#conclave"),
            ),
            Feature::new(
                "Council",
                FeatureCategory::Orchestration,
                "Structured multi-agent deliberation with roles and facilitation",
                FeatureStatus::Available,
                Some::<&str>("docs/COUNCIL.md"),
            ),
            Feature::new(
                "Grove",
                FeatureCategory::Orchestration,
                "Intent-based routing to specialized agent groups",
                FeatureStatus::Available,
                Some::<&str>("docs/GROVE.md"),
            ),
            Feature::new(
                "Maneuver",
                FeatureCategory::Orchestration,
                "Dynamic workflow adaptation with conditional logic",
                FeatureStatus::Available,
                Some::<&str>("docs/MANEUVER.md"),
            ),
            Feature::new(
                "Commander",
                FeatureCategory::Orchestration,
                "Intelligent battalion strategy selection based on task analysis",
                FeatureStatus::Available,
                Some::<&str>("docs/BATTALION.md#commander"),
            ),
            // Memory systems
            Feature::new(
                "Garrison (In-Memory)",
                FeatureCategory::Memory,
                "Fast ephemeral conversation history storage",
                FeatureStatus::Available,
                Some::<&str>("docs/GARRISON.md#in-memory"),
            ),
            Feature::new(
                "Garrison (SQLite)",
                FeatureCategory::Memory,
                "Persistent conversation history with full-text search",
                FeatureStatus::Available,
                Some::<&str>("docs/GARRISON.md#sqlite"),
            ),
            Feature::new(
                "Garrison Semantic Search",
                FeatureCategory::Memory,
                "Vector-based semantic search over conversation history",
                FeatureStatus::RequiresFlag("qdrant"),
                Some::<&str>("docs/GARRISON.md#semantic-search"),
            ),
            Feature::new(
                "Sanctum",
                FeatureCategory::Memory,
                "Long-term knowledge storage with RAG capabilities",
                FeatureStatus::RequiresFlag("qdrant"),
                Some::<&str>("docs/SANCTUM.md"),
            ),
            // Utility features
            Feature::new(
                "Arsenal (MCP)",
                FeatureCategory::Utilities,
                "External tool integration via Model Context Protocol",
                FeatureStatus::Available,
                Some::<&str>("docs/ARSENAL.md"),
            ),
            Feature::new(
                "Citadel",
                FeatureCategory::Utilities,
                "State persistence and checkpoint/recovery system",
                FeatureStatus::Available,
                Some::<&str>("docs/QUICKSTART.md#citadel"),
            ),
            Feature::new(
                "Herald",
                FeatureCategory::Utilities,
                "Customizable output formatting (JSON, Markdown, custom)",
                FeatureStatus::Available,
                Some::<&str>("docs/HERALD.md"),
            ),
            Feature::new(
                "Sentinel",
                FeatureCategory::Utilities,
                "Rate limiting and cost tracking for LLM API calls",
                FeatureStatus::Available,
                Some::<&str>("docs/SENTINEL.md"),
            ),
            Feature::new(
                "CLI Onboarding",
                FeatureCategory::Utilities,
                "Interactive wizard for initial setup and configuration",
                FeatureStatus::Available,
                Some::<&str>("docs/CLI_USAGE.md#onboarding"),
            ),
            Feature::new(
                "Setup Check",
                FeatureCategory::Utilities,
                "Validate environment configuration and provider connectivity",
                FeatureStatus::Available,
                Some::<&str>("docs/CLI_USAGE.md#setup-check"),
            ),
        ];

        Self { features }
    }

    fn get_all(&self) -> &[Feature] {
        &self.features
    }

    fn get_by_category(&self, category: FeatureCategory) -> Vec<&Feature> {
        self.features
            .iter()
            .filter(|f| f.category == category)
            .collect()
    }
}

/// Output format for features display
#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputFormat {
    Table,
    Json,
}

impl OutputFormat {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "table" => Some(Self::Table),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Render features as a table
fn render_table(features: &[&Feature], _formatter: &OutputFormatter) {
    let mut table = TableFormatter::new();
    table.set_header(vec!["Feature", "Category", "Status", "Description"]);

    for feature in features {
        table.add_row(vec![
            feature.name.clone(),
            feature.category.to_string(),
            feature.status.to_string(),
            feature.description.clone(),
        ]);
    }

    table.render();

    // Show documentation links if available
    let with_docs: Vec<_> = features.iter().filter(|f| f.doc_link.is_some()).collect();
    if !with_docs.is_empty() {
        println!();
        println!("Documentation:");
        for feature in with_docs {
            if let Some(link) = &feature.doc_link {
                println!("  {} → {}", feature.name, link);
            }
        }
    }
}

/// Render features as JSON
fn render_json(features: &[&Feature]) -> CliResult<()> {
    let json = serde_json::to_string_pretty(&features)
        .map_err(|e| CliError::execution(format!("Failed to serialize JSON: {}", e)))?;
    println!("{}", json);
    Ok(())
}

/// Display available features and commands
pub async fn run_features(
    category: Option<String>,
    format: Option<String>,
) -> Result<(), CliError> {
    let formatter = OutputFormatter::new();
    let registry = FeatureRegistry::new();

    // Parse output format
    let output_format = match format.as_deref() {
        Some(f) => OutputFormat::from_str(f).ok_or_else(|| CliError::InvalidFieldValue {
            field: "format".to_string(),
            message: format!("Invalid format '{}'. Use 'table' or 'json'", f),
        })?,
        None => OutputFormat::Table,
    };

    // Filter by category if specified
    let features_to_display: Vec<&Feature> = match category.as_ref() {
        Some(cat) => {
            let category_enum = FeatureCategory::from_str(cat).ok_or_else(|| {
                CliError::InvalidFieldValue {
                    field: "category".to_string(),
                    message: format!(
                        "Invalid category '{}'. Use: agent, battalion, orchestration, memory, utilities",
                        cat
                    ),
                }
            })?;
            registry.get_by_category(category_enum)
        }
        None => registry.get_all().iter().collect(),
    };

    // Render output
    match output_format {
        OutputFormat::Table => {
            formatter.header("Paladin Features");
            println!();

            if let Some(cat) = category {
                formatter.info(&format!("Filtered by category: {}", cat));
                println!();
            }

            render_table(&features_to_display, &formatter);

            // Legend for status
            println!();
            println!("Status Legend:");
            println!("  Available      - Ready to use");
            println!("  Requires: flag - Enable with cargo feature flag");
            println!("  Experimental   - Under development");

            // Help text
            println!();
            formatter.box_message(&[
                "Usage:",
                "  --category <cat>  Filter by category (agent, battalion, orchestration, memory, utilities)",
                "  --format json     Output as JSON",
                "",
                "Examples:",
                "  paladin features --category battalion",
                "  paladin features --format json",
            ]);
        }
        OutputFormat::Json => {
            render_json(&features_to_display)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_category_from_str() {
        assert_eq!(
            FeatureCategory::from_str("agent"),
            Some(FeatureCategory::Agent)
        );
        assert_eq!(
            FeatureCategory::from_str("BATTALION"),
            Some(FeatureCategory::Battalion)
        );
        assert_eq!(
            FeatureCategory::from_str("Orchestration"),
            Some(FeatureCategory::Orchestration)
        );
        assert_eq!(
            FeatureCategory::from_str("memory"),
            Some(FeatureCategory::Memory)
        );
        assert_eq!(
            FeatureCategory::from_str("utilities"),
            Some(FeatureCategory::Utilities)
        );
        assert_eq!(FeatureCategory::from_str("invalid"), None);
    }

    #[test]
    fn test_feature_category_all() {
        let categories = FeatureCategory::all();
        assert_eq!(categories.len(), 5);
        assert!(categories.contains(&FeatureCategory::Agent));
        assert!(categories.contains(&FeatureCategory::Battalion));
        assert!(categories.contains(&FeatureCategory::Orchestration));
        assert!(categories.contains(&FeatureCategory::Memory));
        assert!(categories.contains(&FeatureCategory::Utilities));
    }

    #[test]
    fn test_feature_category_display() {
        assert_eq!(FeatureCategory::Agent.to_string(), "Agent");
        assert_eq!(FeatureCategory::Battalion.to_string(), "Battalion");
        assert_eq!(FeatureCategory::Orchestration.to_string(), "Orchestration");
        assert_eq!(FeatureCategory::Memory.to_string(), "Memory");
        assert_eq!(FeatureCategory::Utilities.to_string(), "Utilities");
    }

    #[test]
    fn test_feature_status_display() {
        assert_eq!(FeatureStatus::Available.to_string(), "Available");
        assert_eq!(
            FeatureStatus::RequiresFlag("qdrant").to_string(),
            "Requires: qdrant"
        );
        assert_eq!(FeatureStatus::Experimental.to_string(), "Experimental");
    }

    #[test]
    fn test_output_format_from_str() {
        assert!(matches!(
            OutputFormat::from_str("table"),
            Some(OutputFormat::Table)
        ));
        assert!(matches!(
            OutputFormat::from_str("json"),
            Some(OutputFormat::Json)
        ));
        assert!(matches!(
            OutputFormat::from_str("JSON"),
            Some(OutputFormat::Json)
        ));
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_feature_registry_all() {
        let registry = FeatureRegistry::new();
        let features = registry.get_all();

        // Should have multiple features
        assert!(features.len() > 20);

        // Check some key features exist
        assert!(features.iter().any(|f| f.name == "Paladin"));
        assert!(features.iter().any(|f| f.name == "Formation"));
        assert!(features.iter().any(|f| f.name == "Phalanx"));
        assert!(features.iter().any(|f| f.name == "Campaign"));
        assert!(features.iter().any(|f| f.name == "Garrison (In-Memory)"));
    }

    #[test]
    fn test_feature_registry_by_category() {
        let registry = FeatureRegistry::new();

        let agent_features = registry.get_by_category(FeatureCategory::Agent);
        assert!(!agent_features.is_empty());
        assert!(
            agent_features
                .iter()
                .all(|f| f.category == FeatureCategory::Agent)
        );

        let battalion_features = registry.get_by_category(FeatureCategory::Battalion);
        assert!(!battalion_features.is_empty());
        assert!(
            battalion_features
                .iter()
                .all(|f| f.category == FeatureCategory::Battalion)
        );

        let orchestration_features = registry.get_by_category(FeatureCategory::Orchestration);
        assert!(!orchestration_features.is_empty());

        let memory_features = registry.get_by_category(FeatureCategory::Memory);
        assert!(!memory_features.is_empty());

        let utility_features = registry.get_by_category(FeatureCategory::Utilities);
        assert!(!utility_features.is_empty());
    }

    #[test]
    fn test_feature_has_documentation() {
        let registry = FeatureRegistry::new();
        let features = registry.get_all();

        // Most features should have documentation links
        let with_docs = features.iter().filter(|f| f.doc_link.is_some()).count();
        assert!(with_docs > features.len() / 2);
    }

    #[tokio::test]
    async fn test_run_features_default() {
        // Should not panic with default arguments
        let result = run_features(None, None).await;
        // Will fail in test environment without formatter, but shouldn't panic
        let _ = result;
    }

    #[tokio::test]
    async fn test_run_features_with_category() {
        let result = run_features(Some("agent".to_string()), None).await;
        let _ = result;
    }

    #[tokio::test]
    async fn test_run_features_invalid_category() {
        let result = run_features(Some("invalid".to_string()), None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_features_invalid_format() {
        let result = run_features(None, Some("invalid".to_string())).await;
        assert!(result.is_err());
    }
}
