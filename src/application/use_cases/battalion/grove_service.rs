//! Grove Execution Service
//!
//! Provides intelligent routing and execution logic for the Grove pattern,
//! directing tasks to specialized Paladins based on expertise matching.

use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::Arc;

use crate::application::ports::output::embedding_port::EmbeddingPort;
use crate::application::ports::output::llm_port::LlmPort;
use crate::application::ports::output::paladin_port::PaladinPort;
use crate::core::platform::container::battalion::BattalionError;
use crate::core::platform::container::battalion::grove::{Grove, RoutingStrategy, Tree, TreeAgent};

/// Decision made by the Grove routing system
///
/// Contains information about which agent was selected and why,
/// along with confidence metrics.
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    /// Name of the selected Tree
    pub selected_tree: String,

    /// ID of the selected agent (Paladin)
    pub selected_agent: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,

    /// Human-readable reasoning for the selection
    pub reasoning: String,
}

/// Result of Grove task execution
///
/// Contains the routing decision and the execution result from
/// the selected Paladin.
#[derive(Debug, Clone)]
pub struct GroveResult {
    /// Routing decision that was made
    pub routing_decision: RoutingDecision,

    /// Result from executing the selected Paladin
    pub execution_result: String,

    /// Additional metadata about the execution
    pub metadata: HashMap<String, String>,
}

/// Service for executing Grove patterns
///
/// Routes incoming tasks to specialized Paladins using various routing strategies
/// (keyword matching, semantic similarity, LLM-based routing).
///
/// # Example
///
/// ```ignore
/// use paladin::application::use_cases::battalion::grove_service::GroveExecutionService;
/// use std::sync::Arc;
///
/// let service = GroveExecutionService::new(
///     paladin_port,
///     Some(embedding_port),
///     Some(llm_port)
/// );
/// let result = service.execute(&grove, "Fix the authentication bug").await?;
/// println!("Routed to: {}", result.routing_decision.selected_agent);
/// ```
pub struct GroveExecutionService {
    /// Paladin execution port (placeholder until integration)
    #[allow(dead_code)]
    paladin_port: Arc<dyn PaladinPort>,

    /// Optional embedding port for semantic similarity routing
    embedding_port: Option<Arc<dyn EmbeddingPort>>,

    /// Optional LLM port for LLM-based routing (placeholder until integration)
    #[allow(dead_code)]
    llm_port: Option<Arc<dyn LlmPort>>,
}

impl GroveExecutionService {
    /// Creates a new GroveExecutionService
    ///
    /// # Arguments
    ///
    /// * `paladin_port` - Port for executing Paladins
    /// * `embedding_port` - Optional port for generating embeddings
    /// * `llm_port` - Optional port for LLM calls (used in LLM routing)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let service = GroveExecutionService::new(
    ///     paladin_port,
    ///     Some(embedding_port),
    ///     Some(llm_port)
    /// );
    /// ```
    pub fn new(
        paladin_port: Arc<dyn PaladinPort>,
        embedding_port: Option<Arc<dyn EmbeddingPort>>,
        llm_port: Option<Arc<dyn LlmPort>>,
    ) -> Self {
        Self {
            paladin_port,
            embedding_port,
            llm_port,
        }
    }

    /// Executes a task using the Grove pattern
    ///
    /// Routes the task to the most appropriate agent based on the Grove's
    /// routing strategy, then executes that agent with the task.
    ///
    /// # Arguments
    ///
    /// * `grove` - The Grove configuration with trees and agents
    /// * `task` - The task description to route and execute
    ///
    /// # Returns
    ///
    /// Returns a `GroveResult` containing the routing decision and execution result.
    ///
    /// # Errors
    ///
    /// Returns `BattalionError` if:
    /// - Routing fails and no fallback is available
    /// - Required embedding port is missing for SemanticSimilarity strategy
    /// - Paladin execution fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = service.execute(&grove, "Analyze customer feedback").await?;
    /// println!("Agent: {}", result.routing_decision.selected_agent);
    /// println!("Result: {}", result.execution_result);
    /// ```
    pub async fn execute(&self, grove: &Grove, task: &str) -> Result<GroveResult, BattalionError> {
        info!("Grove '{}' routing task: {}", grove.node.name, task);

        // Route the task to the appropriate agent
        let routing_decision = self.route_task(grove, task).await?;

        debug!(
            "Routed to agent '{}' in tree '{}' (confidence: {:.2})",
            routing_decision.selected_agent,
            routing_decision.selected_tree,
            routing_decision.confidence
        );

        // Execute the selected Paladin
        // TODO: This is a placeholder - actual Paladin lookup will be implemented during integration
        let execution_result = self
            .execute_agent(&routing_decision.selected_agent, task)
            .await
            .map_err(|e| {
                BattalionError::ExecutionError(format!(
                    "Failed to execute agent '{}': {}",
                    routing_decision.selected_agent, e
                ))
            })?;

        // Build metadata
        let mut metadata = HashMap::new();
        metadata.insert("grove_name".to_string(), grove.node.name.clone());
        metadata.insert(
            "routing_strategy".to_string(),
            format!("{:?}", grove.node.config.routing_strategy),
        );
        metadata.insert(
            "confidence".to_string(),
            routing_decision.confidence.to_string(),
        );

        Ok(GroveResult {
            routing_decision,
            execution_result,
            metadata,
        })
    }

    /// Routes a task to the most appropriate agent
    ///
    /// Uses the Grove's configured routing strategy to select the best agent
    /// for the given task. Falls back to alternative strategies or fallback
    /// trees if the primary routing fails.
    ///
    /// # Arguments
    ///
    /// * `grove` - The Grove configuration
    /// * `task` - The task to route
    ///
    /// # Returns
    ///
    /// Returns a `RoutingDecision` containing the selected agent and reasoning.
    ///
    /// # Errors
    ///
    /// Returns `BattalionError` if routing fails and no fallback is available.
    async fn route_task(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<RoutingDecision, BattalionError> {
        let strategy = &grove.node.config.routing_strategy;

        // Try the configured strategy
        let result = match strategy {
            RoutingStrategy::KeywordMatch => self.route_by_keywords(grove, task),
            RoutingStrategy::SemanticSimilarity => {
                self.route_by_semantic_similarity(grove, task).await
            }
            RoutingStrategy::LlmRouting => self.route_by_llm(grove, task).await,
        };

        match result {
            Ok(decision) => Ok(decision),
            Err(e) => {
                warn!(
                    "Primary routing strategy failed: {}. Attempting fallback.",
                    e
                );

                // Try fallback tree if configured
                if let Some(ref fallback_tree_name) = grove.node.config.fallback_tree
                    && let Some(fallback_tree) = grove
                        .node
                        .trees
                        .iter()
                        .find(|t| &t.name == fallback_tree_name)
                {
                    info!("Using fallback tree: {}", fallback_tree_name);
                    return self.select_from_tree(fallback_tree, task, "fallback");
                }

                // Final fallback: use first agent in first tree
                if let Some(first_tree) = grove.node.trees.first() {
                    warn!(
                        "Using default fallback: first agent in tree '{}'",
                        first_tree.name
                    );
                    return self.select_from_tree(first_tree, task, "default_fallback");
                }

                Err(BattalionError::RoutingError(
                    "No agents available for routing".to_string(),
                ))
            }
        }
    }

    /// Routes by keyword matching
    ///
    /// Tokenizes the task, counts matching keywords for each agent,
    /// and selects the agent with the highest overlap.
    fn route_by_keywords(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<RoutingDecision, BattalionError> {
        debug!("Routing by keyword matching");

        // Normalize and tokenize the task
        let task_lower = task.to_lowercase();
        let task_tokens: Vec<String> = task_lower
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let mut best_score = 0;
        let mut best_tree: Option<&Tree> = None;
        let mut best_agent: Option<&TreeAgent> = None;

        // Score each agent in each tree
        for tree in &grove.node.trees {
            for agent in &tree.agents {
                let score = self.calculate_keyword_score(&task_tokens, agent);

                // Track best score, but also track first agent as fallback
                if best_agent.is_none() {
                    best_tree = Some(tree);
                    best_agent = Some(agent);
                }

                if score > best_score {
                    best_score = score;
                    best_tree = Some(tree);
                    best_agent = Some(agent);
                }
            }
        }

        // Check if we found a match
        if let (Some(tree), Some(agent)) = (best_tree, best_agent) {
            let confidence = if task_tokens.is_empty() {
                0.5 // No keywords in task
            } else {
                (best_score as f32 / task_tokens.len() as f32).min(1.0)
            };

            Ok(RoutingDecision {
                selected_tree: tree.name.clone(),
                selected_agent: agent.paladin_id.clone(),
                confidence,
                reasoning: format!(
                    "Matched {} keywords from agent expertise: {:?}",
                    best_score, agent.expertise_keywords
                ),
            })
        } else {
            Err(BattalionError::RoutingError(
                "No agents found for keyword matching".to_string(),
            ))
        }
    }

    /// Calculates keyword overlap score for an agent
    ///
    /// Counts how many task tokens match the agent's expertise keywords
    /// using case-insensitive comparison.
    fn calculate_keyword_score(&self, task_tokens: &[String], agent: &TreeAgent) -> usize {
        let agent_keywords: Vec<String> = agent
            .expertise_keywords
            .iter()
            .map(|k| k.to_lowercase())
            .collect();

        task_tokens
            .iter()
            .filter(|token| agent_keywords.contains(token))
            .count()
    }

    /// Routes by semantic similarity using embeddings
    ///
    /// Embeds the task and compares it with agent expertise embeddings
    /// using cosine similarity. Selects the agent with highest similarity
    /// above the configured threshold.
    async fn route_by_semantic_similarity(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<RoutingDecision, BattalionError> {
        debug!("Routing by semantic similarity");

        // Check if embedding port is available
        let embedding_port = self.embedding_port.as_ref().ok_or_else(|| {
            BattalionError::RoutingError(
                "EmbeddingPort required for SemanticSimilarity routing".to_string(),
            )
        })?;

        // Embed the task
        let task_embedding = embedding_port
            .embed_text(task)
            .await
            .map_err(|e| BattalionError::RoutingError(format!("Failed to embed task: {}", e)))?;

        let threshold = grove.node.config.similarity_threshold;
        let mut best_similarity = threshold; // Must meet minimum threshold
        let mut best_tree: Option<&Tree> = None;
        let mut best_agent: Option<&TreeAgent> = None;

        // Calculate similarity with each agent
        for tree in &grove.node.trees {
            for agent in &tree.agents {
                if let Some(ref agent_embedding) = agent.expertise_embedding {
                    let similarity =
                        self.cosine_similarity(&task_embedding.vector, agent_embedding);

                    if similarity > best_similarity {
                        best_similarity = similarity;
                        best_tree = Some(tree);
                        best_agent = Some(agent);
                    }
                }
            }
        }

        // Check if we found a match above threshold
        if let (Some(tree), Some(agent)) = (best_tree, best_agent) {
            Ok(RoutingDecision {
                selected_tree: tree.name.clone(),
                selected_agent: agent.paladin_id.clone(),
                confidence: best_similarity,
                reasoning: format!(
                    "Semantic similarity score: {:.3} (threshold: {:.3})",
                    best_similarity, threshold
                ),
            })
        } else {
            Err(BattalionError::RoutingError(format!(
                "No agents found with similarity above threshold {:.3}",
                threshold
            )))
        }
    }

    /// Calculates cosine similarity between two vectors
    ///
    /// Returns a value between 0.0 and 1.0, where 1.0 means identical direction.
    fn cosine_similarity(&self, vec_a: &[f32], vec_b: &[f32]) -> f32 {
        if vec_a.len() != vec_b.len() {
            warn!(
                "Dimension mismatch in cosine similarity: {} vs {}",
                vec_a.len(),
                vec_b.len()
            );
            return 0.0;
        }

        let dot_product: f32 = vec_a.iter().zip(vec_b.iter()).map(|(a, b)| a * b).sum();

        let magnitude_a: f32 = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = vec_b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        (dot_product / (magnitude_a * magnitude_b)).clamp(0.0, 1.0)
    }

    /// Routes using LLM analysis
    ///
    /// Provides the LLM with task description and agent expertise,
    /// asking it to select the best agent with reasoning.
    async fn route_by_llm(
        &self,
        grove: &Grove,
        task: &str,
    ) -> Result<RoutingDecision, BattalionError> {
        debug!("Routing by LLM analysis");

        // Build prompt with agent information
        let mut prompt = format!(
            "You are a task routing system. Given the following task and available specialized agents, \
            select the most appropriate agent and explain your reasoning.\n\n\
            Task: {}\n\n\
            Available Agents:\n",
            task
        );

        for tree in &grove.node.trees {
            prompt.push_str(&format!("\nTree: {}\n", tree.name));
            for agent in &tree.agents {
                prompt.push_str(&format!(
                    "  - Agent ID: {}\n    Expertise: {}\n",
                    agent.paladin_id,
                    agent.expertise_keywords.join(", ")
                ));
            }
        }

        prompt.push_str(
            "\n\nRespond in JSON format with the following structure:\n\
            {\n  \
              \"tree_name\": \"selected tree name\",\n  \
              \"agent_id\": \"selected agent ID\",\n  \
              \"confidence\": 0.85,\n  \
              \"reasoning\": \"explanation for selection\"\n\
            }\n",
        );

        // TODO: Call LLM with prompt
        // For now, this is a placeholder that will be implemented during integration
        debug!("LLM routing prompt prepared (placeholder implementation)");

        // Placeholder: fallback to keyword matching
        warn!("LLM routing not yet fully implemented, falling back to keyword matching");
        self.route_by_keywords(grove, task)
    }

    /// Selects the first agent from a tree
    ///
    /// Used for fallback scenarios when routing strategies fail.
    fn select_from_tree(
        &self,
        tree: &Tree,
        task: &str,
        reason: &str,
    ) -> Result<RoutingDecision, BattalionError> {
        tree.agents.first().map_or_else(
            || {
                Err(BattalionError::RoutingError(format!(
                    "Tree '{}' has no agents",
                    tree.name
                )))
            },
            |agent| {
                Ok(RoutingDecision {
                    selected_tree: tree.name.clone(),
                    selected_agent: agent.paladin_id.clone(),
                    confidence: 0.5,
                    reasoning: format!("Using {} strategy for task: {}", reason, task),
                })
            },
        )
    }

    /// Executes an agent with the given task
    ///
    /// Placeholder for actual Paladin execution that will be implemented
    /// during integration phase.
    async fn execute_agent(&self, agent_id: &str, task: &str) -> Result<String, BattalionError> {
        // TODO: Implement actual Paladin lookup and execution
        debug!("Executing agent '{}' with task: {}", agent_id, task);

        // Placeholder implementation
        Err(BattalionError::ExecutionError(format!(
            "Agent execution not yet implemented for agent '{}'. \
            This will be completed during integration phase.",
            agent_id
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::output::embedding_port::Embedding;
    use crate::core::platform::container::battalion::grove::{GroveBuilder, Tree, TreeAgent};

    /// Mock embedding port for testing
    struct MockEmbeddingPort;

    #[async_trait::async_trait]
    impl EmbeddingPort for MockEmbeddingPort {
        async fn embed_text(
            &self,
            text: &str,
        ) -> Result<Embedding, crate::application::ports::output::embedding_port::EmbeddingError>
        {
            // Simple mock: return embedding based on text length
            let vector = vec![text.len() as f32 / 100.0; 128];
            Ok(Embedding {
                vector,
                model: "mock-model".to_string(),
                dimension: 128,
                token_count: Some(text.split_whitespace().count() as u32),
            })
        }

        async fn embed_batch(
            &self,
            texts: &[&str],
        ) -> Result<Vec<Embedding>, crate::application::ports::output::embedding_port::EmbeddingError>
        {
            let mut embeddings = Vec::new();
            for text in texts {
                embeddings.push(self.embed_text(text).await?);
            }
            Ok(embeddings)
        }

        fn dimension(&self) -> usize {
            128
        }

        fn model_name(&self) -> &str {
            "mock-model"
        }
    }

    #[test]
    fn test_calculate_keyword_score() {
        let service = create_test_service();
        let agent =
            TreeAgent::new("test_agent").with_keywords(vec!["rust", "backend", "api", "database"]);

        let task_tokens = vec!["rust".to_string(), "api".to_string(), "testing".to_string()];

        let score = service.calculate_keyword_score(&task_tokens, &agent);
        assert_eq!(score, 2); // "rust" and "api" match
    }

    #[test]
    fn test_calculate_keyword_score_case_insensitive() {
        let service = create_test_service();
        let agent = TreeAgent::new("test_agent").with_keywords(vec!["Rust", "Backend", "API"]);

        let task_tokens = vec!["rust".to_string(), "api".to_string()];

        let score = service.calculate_keyword_score(&task_tokens, &agent);
        assert_eq!(score, 2);
    }

    #[test]
    fn test_calculate_keyword_score_no_match() {
        let service = create_test_service();
        let agent = TreeAgent::new("test_agent").with_keywords(vec!["python", "django"]);

        let task_tokens = vec!["rust".to_string(), "api".to_string()];

        let score = service.calculate_keyword_score(&task_tokens, &agent);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let service = create_test_service();
        let vec_a = vec![1.0, 2.0, 3.0];
        let vec_b = vec![1.0, 2.0, 3.0];

        let similarity = service.cosine_similarity(&vec_a, &vec_b);
        assert!((similarity - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let service = create_test_service();
        let vec_a = vec![1.0, 0.0, 0.0];
        let vec_b = vec![0.0, 1.0, 0.0];

        let similarity = service.cosine_similarity(&vec_a, &vec_b);
        assert!((similarity - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let service = create_test_service();
        let vec_a = vec![1.0, 2.0, 3.0];
        let vec_b = vec![-1.0, -2.0, -3.0];

        let similarity = service.cosine_similarity(&vec_a, &vec_b);
        // Cosine similarity is clamped to [0, 1], so opposite vectors give 0
        assert!((similarity - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_dimension_mismatch() {
        let service = create_test_service();
        let vec_a = vec![1.0, 2.0];
        let vec_b = vec![1.0, 2.0, 3.0];

        let similarity = service.cosine_similarity(&vec_a, &vec_b);
        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_route_by_keywords_basic() {
        let service = create_test_service();
        let grove = create_test_grove();

        let result = service.route_by_keywords(&grove, "rust backend api development");

        assert!(result.is_ok());
        let decision = result.unwrap();
        assert_eq!(decision.selected_agent, "backend_expert");
        assert!(decision.confidence > 0.0);
    }

    #[test]
    fn test_route_by_keywords_no_match() {
        let service = create_test_service();
        let grove = create_test_grove();

        let result = service.route_by_keywords(&grove, "quantum physics simulation");

        // Should still route, but with low confidence
        assert!(result.is_ok());
        let decision = result.unwrap();
        assert!(decision.confidence <= 0.5);
    }

    // Helper functions
    fn create_test_service() -> GroveExecutionService {
        GroveExecutionService::new(
            Arc::new(MockPaladinPort),
            Some(Arc::new(MockEmbeddingPort)),
            Some(Arc::new(MockLlmPort)),
        )
    }

    fn create_test_grove() -> Grove {
        GroveBuilder::new()
            .name("Test Grove")
            .add_tree(
                Tree::new("engineering")
                    .add_agent(
                        TreeAgent::new("backend_expert")
                            .with_keywords(vec!["rust", "backend", "api", "database"]),
                    )
                    .add_agent(TreeAgent::new("frontend_expert").with_keywords(vec![
                        "react",
                        "ui",
                        "css",
                        "javascript",
                    ])),
            )
            .build()
            .unwrap()
    }

    // Mock ports for testing
    struct MockPaladinPort;
    struct MockLlmPort;

    #[async_trait::async_trait]
    impl PaladinPort for MockPaladinPort {
        async fn execute(
            &self,
            _paladin: &crate::core::platform::container::paladin::Paladin,
            _input: &str,
        ) -> Result<
            crate::application::ports::output::paladin_port::PaladinResult,
            crate::application::use_cases::paladin::error::PaladinError,
        > {
            unimplemented!("Mock not needed for these tests")
        }

        async fn execute_stream(
            &self,
            _paladin: &crate::core::platform::container::paladin::Paladin,
            _input: &str,
        ) -> Result<
            crate::application::ports::output::paladin_port::PaladinStream,
            crate::application::use_cases::paladin::error::PaladinError,
        > {
            unimplemented!("Mock not needed for these tests")
        }

        fn validate(
            &self,
            _paladin: &crate::core::platform::container::paladin::Paladin,
        ) -> Result<(), crate::application::use_cases::paladin::error::PaladinError> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl LlmPort for MockLlmPort {
        async fn generate(
            &self,
            _request: crate::application::ports::output::llm_port::LlmRequest,
        ) -> Result<
            crate::application::ports::output::llm_port::LlmResponse,
            crate::application::ports::output::llm_port::LlmError,
        > {
            unimplemented!("Mock not needed for these tests")
        }

        async fn generate_stream(
            &self,
            _request: crate::application::ports::output::llm_port::LlmRequest,
        ) -> Result<
            Box<
                dyn futures::Stream<
                        Item = Result<
                            crate::application::ports::output::llm_port::StreamingResponse,
                            crate::application::ports::output::llm_port::LlmError,
                        >,
                    > + Send,
            >,
            crate::application::ports::output::llm_port::LlmError,
        > {
            unimplemented!("Mock not needed for these tests")
        }

        async fn validate_model(
            &self,
            _model: &str,
        ) -> Result<bool, crate::application::ports::output::llm_port::LlmError> {
            Ok(true)
        }

        async fn get_available_models(
            &self,
        ) -> Result<Vec<String>, crate::application::ports::output::llm_port::LlmError> {
            Ok(vec!["mock-model".to_string()])
        }

        fn get_provider_name(&self) -> &'static str {
            "mock"
        }

        fn get_capabilities(
            &self,
        ) -> crate::application::ports::output::llm_port::ProviderCapabilities {
            use crate::application::ports::output::llm_port::ProviderCapabilities;
            ProviderCapabilities {
                supports_streaming: false,
                supports_tool_calling: false,
                supports_function_calling: false,
                supports_vision: false,
                supports_embeddings: false,
                max_context_tokens: Some(4096),
                supports_system_messages: true,
            }
        }
    }
}
