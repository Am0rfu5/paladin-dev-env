//! Grove Pattern - Tree-Based Agent Routing
//!
//! This module implements the Grove pattern for multi-agent orchestration where
//! incoming requests are intelligently routed to specialized Paladins organized
//! in a tree structure based on their expertise and the request content.
//!
//! # Overview
//!
//! A Grove organizes Paladins into Trees, where each Tree represents a domain of
//! expertise and contains specialized Paladins (TreeAgents). The Grove uses various
//! routing strategies to select the most appropriate agent:
//! - **Keyword Matching**: Route based on keyword overlap
//! - **Semantic Similarity**: Route using embedding-based similarity
//! - **LLM Routing**: Use an LLM to analyze and route intelligently
//!
//! # Example
//!
//! ```ignore
//! use paladin::core::platform::container::battalion::grove::{
//!     GroveBuilder, RoutingStrategy, Tree, TreeAgent
//! };
//!
//! let grove = GroveBuilder::new()
//!     .name("Support Routing Grove")
//!     .add_tree(
//!         Tree::new("technical_support")
//!             .add_agent(TreeAgent::new("backend_expert")
//!                 .with_keywords(vec!["api", "database", "server"]))
//!             .add_agent(TreeAgent::new("frontend_expert")
//!                 .with_keywords(vec!["ui", "react", "css"]))
//!     )
//!     .routing_strategy(RoutingStrategy::KeywordMatch)
//!     .build()?;
//! ```

use serde::{Deserialize, Serialize};

use crate::core::base::entity::node::Node;

use super::BattalionError;

/// Routing strategy for Grove request handling
///
/// Determines how the Grove selects which Tree and TreeAgent should handle
/// an incoming request based on the request content and agent expertise.
///
/// # Strategies
///
/// - **KeywordMatch**: Simple keyword overlap counting (fast, deterministic)
/// - **SemanticSimilarity**: Embedding-based cosine similarity (accurate, requires embeddings)
/// - **LlmRouting**: LLM-based intelligent routing (flexible, requires LLM calls)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RoutingStrategy {
    /// Route based on keyword matching between request and agent expertise
    ///
    /// Counts the number of matching keywords between the request and each
    /// agent's expertise_keywords, selecting the agent with the highest overlap.
    #[default]
    KeywordMatch,

    /// Route based on semantic similarity using embeddings
    ///
    /// Computes cosine similarity between the request embedding and agent
    /// expertise embeddings. Requires agents to have expertise_embedding populated.
    SemanticSimilarity,

    /// Route using an LLM to analyze request and select best agent
    ///
    /// Provides the LLM with request content and agent descriptions, allowing
    /// it to make an intelligent routing decision with reasoning.
    LlmRouting,
}

/// A specialized agent within a Tree with defined expertise
///
/// TreeAgent represents a Paladin with specific domain expertise, identified
/// by keywords and optionally by semantic embeddings for more sophisticated routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TreeAgent {
    /// Unique identifier of the Paladin
    pub paladin_id: String,

    /// Keywords describing the agent's expertise domain
    ///
    /// Used for keyword-based routing. Should include terms that describe
    /// the agent's capabilities, knowledge areas, and specializations.
    pub expertise_keywords: Vec<String>,

    /// Optional embedding vector representing agent's expertise
    ///
    /// Used for semantic similarity routing. Typically generated from
    /// the agent's description and expertise keywords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expertise_embedding: Option<Vec<f32>>,
}

impl TreeAgent {
    /// Creates a new TreeAgent with the given Paladin ID
    ///
    /// # Arguments
    ///
    /// * `paladin_id` - Unique identifier for the associated Paladin
    ///
    /// # Example
    ///
    /// ```ignore
    /// let agent = TreeAgent::new("backend_expert");
    /// ```
    pub fn new(paladin_id: impl Into<String>) -> Self {
        Self {
            paladin_id: paladin_id.into(),
            expertise_keywords: Vec::new(),
            expertise_embedding: None,
        }
    }

    /// Sets the expertise keywords for this agent
    ///
    /// # Arguments
    ///
    /// * `keywords` - Vector of keywords describing agent expertise
    ///
    /// # Example
    ///
    /// ```ignore
    /// let agent = TreeAgent::new("backend_expert")
    ///     .with_keywords(vec!["api", "database", "microservices"]);
    /// ```
    pub fn with_keywords(mut self, keywords: Vec<impl Into<String>>) -> Self {
        self.expertise_keywords = keywords.into_iter().map(|k| k.into()).collect();
        self
    }

    /// Sets the expertise embedding for this agent
    ///
    /// # Arguments
    ///
    /// * `embedding` - Vector of f32 values representing the agent's expertise
    ///
    /// # Example
    ///
    /// ```ignore
    /// let agent = TreeAgent::new("backend_expert")
    ///     .with_embedding(vec![0.1, 0.2, 0.3, ...]);
    /// ```
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.expertise_embedding = Some(embedding);
        self
    }
}

/// A tree of specialized agents for a specific domain
///
/// Tree groups related TreeAgents that share a common domain of expertise.
/// For example, all technical support agents might be in one Tree, while
/// all sales agents are in another.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    /// Name of this Tree (e.g., "technical_support", "sales")
    pub name: String,

    /// Collection of agents within this Tree
    pub agents: Vec<TreeAgent>,
}

impl Tree {
    /// Creates a new Tree with the given name
    ///
    /// # Arguments
    ///
    /// * `name` - Identifier for this Tree
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tree = Tree::new("technical_support");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agents: Vec::new(),
        }
    }

    /// Adds an agent to this Tree
    ///
    /// # Arguments
    ///
    /// * `agent` - TreeAgent to add to this Tree
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tree = Tree::new("support")
    ///     .add_agent(TreeAgent::new("backend_expert"));
    /// ```
    pub fn add_agent(mut self, agent: TreeAgent) -> Self {
        self.agents.push(agent);
        self
    }
}

/// Configuration for Grove behavior
///
/// Controls how the Grove performs routing and handles edge cases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroveConfig {
    /// Strategy to use for routing requests to agents
    pub routing_strategy: RoutingStrategy,

    /// Optional fallback Tree name if no good match is found
    ///
    /// If routing fails to find a suitable agent (e.g., no keywords match,
    /// similarity too low), the Grove can route to a fallback Tree's first agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_tree: Option<String>,

    /// Minimum similarity threshold for semantic routing (0.0 to 1.0)
    ///
    /// When using SemanticSimilarity routing, agents must meet this threshold
    /// to be considered. Default: 0.7
    pub similarity_threshold: f32,
}

impl Default for GroveConfig {
    fn default() -> Self {
        Self {
            routing_strategy: RoutingStrategy::default(),
            fallback_tree: None,
            similarity_threshold: 0.7,
        }
    }
}

/// Core data for a Grove
///
/// Contains all the Trees and configuration needed for intelligent routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroveData {
    /// Name of this Grove
    pub name: String,

    /// Collection of Trees within this Grove
    pub trees: Vec<Tree>,

    /// Configuration for routing behavior
    pub config: GroveConfig,
}

/// Grove aggregate root using the Node pattern
///
/// A Grove is a tree-based multi-agent routing system that intelligently
/// directs requests to specialized agents based on expertise and content.
pub type Grove = Node<GroveData>;

/// Builder for constructing Grove instances
///
/// Provides a fluent interface for configuring and building Grove instances
/// with validation.
///
/// # Example
///
/// ```ignore
/// let grove = GroveBuilder::new()
///     .name("Customer Support Grove")
///     .add_tree(
///         Tree::new("technical")
///             .add_agent(TreeAgent::new("tech_expert_1"))
///     )
///     .routing_strategy(RoutingStrategy::KeywordMatch)
///     .similarity_threshold(0.75)
///     .build()?;
/// ```
pub struct GroveBuilder {
    name: String,
    trees: Vec<Tree>,
    routing_strategy: RoutingStrategy,
    fallback_tree: Option<String>,
    similarity_threshold: f32,
}

impl GroveBuilder {
    /// Creates a new GroveBuilder with default configuration
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = GroveBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            name: String::new(),
            trees: Vec::new(),
            routing_strategy: RoutingStrategy::default(),
            fallback_tree: None,
            similarity_threshold: 0.7,
        }
    }

    /// Sets the name of the Grove
    ///
    /// # Arguments
    ///
    /// * `name` - Name for this Grove
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = GroveBuilder::new()
    ///     .name("Support Grove");
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Adds a Tree to the Grove
    ///
    /// # Arguments
    ///
    /// * `tree` - Tree to add to this Grove
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = GroveBuilder::new()
    ///     .add_tree(Tree::new("technical_support"));
    /// ```
    pub fn add_tree(mut self, tree: Tree) -> Self {
        self.trees.push(tree);
        self
    }

    /// Sets the routing strategy
    ///
    /// # Arguments
    ///
    /// * `strategy` - RoutingStrategy to use
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = GroveBuilder::new()
    ///     .routing_strategy(RoutingStrategy::SemanticSimilarity);
    /// ```
    pub fn routing_strategy(mut self, strategy: RoutingStrategy) -> Self {
        self.routing_strategy = strategy;
        self
    }

    /// Sets the fallback Tree name
    ///
    /// # Arguments
    ///
    /// * `tree_name` - Name of the Tree to use as fallback
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = GroveBuilder::new()
    ///     .fallback_tree("general_support");
    /// ```
    pub fn fallback_tree(mut self, tree_name: impl Into<String>) -> Self {
        self.fallback_tree = Some(tree_name.into());
        self
    }

    /// Sets the similarity threshold for semantic routing
    ///
    /// # Arguments
    ///
    /// * `threshold` - Float between 0.0 and 1.0
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = GroveBuilder::new()
    ///     .similarity_threshold(0.8);
    /// ```
    pub fn similarity_threshold(mut self, threshold: f32) -> Self {
        self.similarity_threshold = threshold;
        self
    }

    /// Sets the complete configuration
    ///
    /// # Arguments
    ///
    /// * `config` - GroveConfig instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let config = GroveConfig::default();
    /// let builder = GroveBuilder::new()
    ///     .config(config);
    /// ```
    pub fn config(mut self, config: GroveConfig) -> Self {
        self.routing_strategy = config.routing_strategy;
        self.fallback_tree = config.fallback_tree;
        self.similarity_threshold = config.similarity_threshold;
        self
    }

    /// Validates and builds the Grove
    ///
    /// # Errors
    ///
    /// Returns `BattalionError::ValidationError` if:
    /// - Name is empty
    /// - No Trees are defined
    /// - Any Tree has no agents
    /// - Similarity threshold is not in range [0.0, 1.0]
    /// - Fallback tree name doesn't exist in Trees
    ///
    /// # Example
    ///
    /// ```ignore
    /// let grove = GroveBuilder::new()
    ///     .name("Support Grove")
    ///     .add_tree(Tree::new("tech").add_agent(TreeAgent::new("expert")))
    ///     .build()?;
    /// ```
    pub fn build(self) -> Result<Grove, BattalionError> {
        // Validate name
        if self.name.is_empty() {
            return Err(BattalionError::ValidationError(
                "Grove name cannot be empty".to_string(),
            ));
        }

        // Validate trees
        if self.trees.is_empty() {
            return Err(BattalionError::ValidationError(
                "Grove must have at least one Tree".to_string(),
            ));
        }

        // Validate each tree has agents
        for tree in &self.trees {
            if tree.agents.is_empty() {
                return Err(BattalionError::ValidationError(format!(
                    "Tree '{}' must have at least one agent",
                    tree.name
                )));
            }
        }

        // Validate similarity threshold
        if !(0.0..=1.0).contains(&self.similarity_threshold) {
            return Err(BattalionError::ValidationError(
                "Similarity threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate fallback tree exists if specified
        if let Some(ref fallback) = self.fallback_tree
            && !self.trees.iter().any(|t| &t.name == fallback)
        {
            return Err(BattalionError::ValidationError(format!(
                "Fallback tree '{}' not found in Grove trees",
                fallback
            )));
        }

        let name = self.name.clone();
        let data = GroveData {
            name: self.name,
            trees: self.trees,
            config: GroveConfig {
                routing_strategy: self.routing_strategy,
                fallback_tree: self.fallback_tree,
                similarity_threshold: self.similarity_threshold,
            },
        };

        Ok(Node::new(data, Some(name)))
    }
}

impl Default for GroveBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_strategy_default() {
        let strategy = RoutingStrategy::default();
        assert_eq!(strategy, RoutingStrategy::KeywordMatch);
    }

    #[test]
    fn test_tree_agent_creation() {
        let agent = TreeAgent::new("test_agent")
            .with_keywords(vec!["rust", "backend", "api"])
            .with_embedding(vec![0.1, 0.2, 0.3]);

        assert_eq!(agent.paladin_id, "test_agent");
        assert_eq!(agent.expertise_keywords, vec!["rust", "backend", "api"]);
        assert!(agent.expertise_embedding.is_some());
        assert_eq!(agent.expertise_embedding.unwrap().len(), 3);
    }

    #[test]
    fn test_tree_creation() {
        let tree = Tree::new("technical_support")
            .add_agent(TreeAgent::new("backend_expert"))
            .add_agent(TreeAgent::new("frontend_expert"));

        assert_eq!(tree.name, "technical_support");
        assert_eq!(tree.agents.len(), 2);
    }

    #[test]
    fn test_grove_config_default() {
        let config = GroveConfig::default();
        assert_eq!(config.routing_strategy, RoutingStrategy::KeywordMatch);
        assert!(config.fallback_tree.is_none());
        assert_eq!(config.similarity_threshold, 0.7);
    }

    #[test]
    fn test_grove_builder_basic() {
        let grove = GroveBuilder::new()
            .name("Test Grove")
            .add_tree(
                Tree::new("support")
                    .add_agent(TreeAgent::new("agent1").with_keywords(vec!["help", "support"])),
            )
            .build();

        assert!(grove.is_ok());
        let grove = grove.unwrap();
        assert_eq!(grove.node.name, "Test Grove");
        assert_eq!(grove.node.trees.len(), 1);
    }

    #[test]
    fn test_grove_builder_validation_empty_name() {
        let result = GroveBuilder::new()
            .add_tree(Tree::new("support").add_agent(TreeAgent::new("agent1")))
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(BattalionError::ValidationError(_))));
    }

    #[test]
    fn test_grove_builder_validation_no_trees() {
        let result = GroveBuilder::new().name("Test Grove").build();

        assert!(result.is_err());
        assert!(matches!(result, Err(BattalionError::ValidationError(_))));
    }

    #[test]
    fn test_grove_builder_validation_tree_without_agents() {
        let result = GroveBuilder::new()
            .name("Test Grove")
            .add_tree(Tree::new("empty_tree"))
            .build();

        assert!(result.is_err());
        match result {
            Err(BattalionError::ValidationError(msg)) => {
                assert!(msg.contains("must have at least one agent"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_grove_builder_validation_invalid_threshold() {
        let result = GroveBuilder::new()
            .name("Test Grove")
            .add_tree(Tree::new("support").add_agent(TreeAgent::new("agent1")))
            .similarity_threshold(1.5)
            .build();

        assert!(result.is_err());
        match result {
            Err(BattalionError::ValidationError(msg)) => {
                assert!(msg.contains("between 0.0 and 1.0"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_grove_builder_validation_invalid_fallback() {
        let result = GroveBuilder::new()
            .name("Test Grove")
            .add_tree(Tree::new("support").add_agent(TreeAgent::new("agent1")))
            .fallback_tree("nonexistent")
            .build();

        assert!(result.is_err());
        match result {
            Err(BattalionError::ValidationError(msg)) => {
                assert!(msg.contains("Fallback tree"));
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_grove_builder_with_valid_fallback() {
        let grove = GroveBuilder::new()
            .name("Test Grove")
            .add_tree(Tree::new("support").add_agent(TreeAgent::new("agent1")))
            .add_tree(Tree::new("general").add_agent(TreeAgent::new("agent2")))
            .fallback_tree("general")
            .build();

        assert!(grove.is_ok());
        let grove = grove.unwrap();
        assert_eq!(grove.node.config.fallback_tree, Some("general".to_string()));
    }

    #[test]
    fn test_grove_builder_full_config() {
        let grove = GroveBuilder::new()
            .name("Advanced Grove")
            .add_tree(
                Tree::new("technical")
                    .add_agent(
                        TreeAgent::new("expert1")
                            .with_keywords(vec!["rust", "systems"])
                            .with_embedding(vec![0.1, 0.2]),
                    )
                    .add_agent(TreeAgent::new("expert2").with_keywords(vec!["web", "api"])),
            )
            .add_tree(Tree::new("general").add_agent(TreeAgent::new("generalist")))
            .routing_strategy(RoutingStrategy::SemanticSimilarity)
            .similarity_threshold(0.8)
            .fallback_tree("general")
            .build();

        assert!(grove.is_ok());
        let grove = grove.unwrap();
        assert_eq!(grove.node.name, "Advanced Grove");
        assert_eq!(grove.node.trees.len(), 2);
        assert_eq!(
            grove.node.config.routing_strategy,
            RoutingStrategy::SemanticSimilarity
        );
        assert_eq!(grove.node.config.similarity_threshold, 0.8);
        assert_eq!(grove.node.config.fallback_tree, Some("general".to_string()));
    }

    #[test]
    fn test_grove_serialization() {
        let grove = GroveBuilder::new()
            .name("Serialization Test")
            .add_tree(
                Tree::new("test_tree")
                    .add_agent(TreeAgent::new("agent1").with_keywords(vec!["test"])),
            )
            .build()
            .unwrap();

        let json = serde_json::to_string(&grove.node).unwrap();
        let deserialized: GroveData = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Serialization Test");
        assert_eq!(deserialized.trees.len(), 1);
    }
}
