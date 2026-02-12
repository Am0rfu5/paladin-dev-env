//! Integration tests for Grove pattern
//!
//! Tests end-to-end Grove execution with intelligent agent routing

use async_trait::async_trait;
use paladin::application::ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStream, StopReason,
};
use paladin::application::use_cases::battalion::grove_service::GroveExecutionService;
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::battalion::grove::{
    GroveBuilder, RoutingStrategy, Tree, TreeAgent,
};
use paladin::core::platform::container::paladin::{MaxLoops, Paladin, PaladinData, PaladinStatus};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Mock PaladinPort for Grove integration testing
#[derive(Clone)]
struct GroveMockPaladinPort {
    execution_log: Arc<Mutex<Vec<String>>>,
}

impl GroveMockPaladinPort {
    fn new() -> Self {
        Self {
            execution_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    fn get_execution_log(&self) -> Vec<String> {
        self.execution_log.lock().unwrap().clone()
    }
}

#[async_trait]
impl PaladinPort for GroveMockPaladinPort {
    async fn execute(&self, paladin: &Paladin, input: &str) -> Result<PaladinResult, PaladinError> {
        // Log execution
        let log_entry = format!("Agent '{}' handling task", paladin.node.name);
        self.execution_log.lock().unwrap().push(log_entry);

        // Simulate processing delay
        tokio::time::sleep(Duration::from_millis(10)).await;

        Ok(PaladinResult {
            output: format!("[{}]: Handled task: {}", paladin.node.name, input),
            token_count: 100,
            execution_time_ms: 10,
            loop_count: 1,
            stop_reason: StopReason::Completed,
            ..Default::default()
        })
    }

    async fn execute_stream(
        &self,
        _paladin: &Paladin,
        _input: &str,
    ) -> Result<PaladinStream, PaladinError> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok(rx)
    }

    fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
        Ok(())
    }
}

/// Helper function to create a specialized Paladin
fn create_specialist_paladin(name: &str, expertise: &str, _keywords: Vec<&str>) -> Paladin {
    let data = PaladinData {
        system_prompt: format!(
            "You are a {} specialist with expertise in {}",
            name, expertise
        ),
        name: name.to_string(),
        user_name: "User".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_loops: MaxLoops::Fixed(3),
        stop_words: vec![],
        status: PaladinStatus::Idle,
        vision_enabled: false,
            ..Default::default()
    };
    Node::new(data, Some(name.to_string()))
}

/// Helper to create TreeAgent
fn create_tree_agent(paladin_id: &str, keywords: Vec<&str>) -> TreeAgent {
    TreeAgent::new(paladin_id).with_keywords(keywords.iter().map(|s| s.to_string()).collect())
}

#[tokio::test]
async fn test_grove_keyword_match_routing() {
    // Task 8.9: Grove with KeywordMatch routing
    let paladin_port = Arc::new(GroveMockPaladinPort::new());

    // Create Paladins (using agent_N format to match TreeAgent IDs)
    let paladins = vec![
        create_specialist_paladin("agent_0", "authentication", vec!["auth"]),
        create_specialist_paladin("agent_1", "encryption", vec!["crypto"]),
        create_specialist_paladin("agent_2", "caching", vec!["cache"]),
        create_specialist_paladin("agent_3", "database", vec!["database"]),
    ];

    // Create security experts tree
    let security_tree = Tree::new("Security Experts")
        .add_agent(create_tree_agent(
            "agent_0",
            vec!["auth", "authentication", "login", "oauth"],
        ))
        .add_agent(create_tree_agent(
            "agent_1",
            vec!["encryption", "crypto", "tls", "ssl"],
        ));

    // Create performance experts tree
    let performance_tree = Tree::new("Performance Experts")
        .add_agent(create_tree_agent(
            "agent_2",
            vec!["cache", "redis", "memcached"],
        ))
        .add_agent(create_tree_agent(
            "agent_3",
            vec!["database", "query", "index", "sql"],
        ));

    let grove = GroveBuilder::new()
        .name("KeywordGrove")
        .add_tree(security_tree)
        .add_tree(performance_tree)
        .routing_strategy(RoutingStrategy::KeywordMatch)
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port.clone(), None, None);

    // Test routing to security tree
    let security_task = "Review authentication implementation for vulnerabilities";
    let result = service
        .execute(&grove, &paladins, security_task)
        .await
        .expect("Security task should succeed");

    assert!(!result.execution_result.is_empty());
    assert!(
        result.routing_decision.selected_agent == "agent_0"
            || result.routing_decision.selected_agent == "agent_1",
        "Should route to security expert"
    );

    // Test routing to performance tree
    let performance_task = "Optimize database queries for better performance";
    let result2 = service
        .execute(&grove, &paladins, performance_task)
        .await
        .expect("Performance task should succeed");

    assert!(
        result2.routing_decision.selected_agent == "agent_2"
            || result2.routing_decision.selected_agent == "agent_3",
        "Should route to performance expert"
    );
}

#[tokio::test]
async fn test_grove_semantic_similarity_routing() {
    // Task 8.10: Grove with SemanticSimilarity routing (mock embeddings)
    let paladin_port = Arc::new(GroveMockPaladinPort::new());

    // Create Paladins using agent_N format
    let paladins = vec![
        create_specialist_paladin("agent_0", "React", vec!["react"]),
        create_specialist_paladin("agent_1", "CSS", vec!["css"]),
        create_specialist_paladin("agent_2", "REST APIs", vec!["api"]),
        create_specialist_paladin("agent_3", "Databases", vec!["database"]),
    ];

    // Create trees
    let frontend_tree = Tree::new("Frontend Experts")
        .add_agent(create_tree_agent(
            "agent_0",
            vec!["react", "jsx", "frontend", "ui"],
        ))
        .add_agent(create_tree_agent("agent_1", vec!["css", "styling", "ui"]));

    let backend_tree = Tree::new("Backend Experts")
        .add_agent(create_tree_agent("agent_2", vec!["api", "rest", "backend"]))
        .add_agent(create_tree_agent(
            "agent_3",
            vec!["database", "sql", "backend"],
        ));

    let grove = GroveBuilder::new()
        .name("SemanticGrove")
        .add_tree(frontend_tree)
        .add_tree(backend_tree)
        .routing_strategy(RoutingStrategy::SemanticSimilarity)
        .similarity_threshold(0.5)
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port.clone(), None, None);

    // Note: In a real test with embeddings, we'd use an actual embedding service
    // For this integration test, we're just verifying the routing mechanism works
    let task = "Design a user interface for the login page";
    let result = service.execute(&grove, &paladins, task).await;

    // Should succeed (will use keyword fallback if embeddings not available)
    assert!(result.is_ok(), "Execution should succeed");
}

#[tokio::test]
async fn test_grove_llm_routing() {
    // Task 8.11: Grove with LlmRouting
    // Note: This would require a mock LLM that returns routing decisions
    // For now, we test that the routing strategy can be configured

    let paladin_port = Arc::new(GroveMockPaladinPort::new());

    let paladins = vec![
        create_specialist_paladin("agent_0", "features", vec!["feature"]),
        create_specialist_paladin("agent_1", "bugs", vec!["bug"]),
    ];

    let tree1 =
        Tree::new("Team A").add_agent(create_tree_agent("agent_0", vec!["feature", "development"]));

    let tree2 = Tree::new("Team B").add_agent(create_tree_agent("agent_1", vec!["bug", "fix"]));

    let grove = GroveBuilder::new()
        .name("LlmRoutingGrove")
        .add_tree(tree1)
        .add_tree(tree2)
        .routing_strategy(RoutingStrategy::LlmRouting)
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port, None, None);

    // Execute task - will use keyword fallback since we don't have real LLM
    let result = service
        .execute(&grove, &paladins, "Fix the login bug")
        .await;

    assert!(result.is_ok(), "Execution should succeed");
}

#[tokio::test]
async fn test_grove_fallback_behavior() {
    // Task 8.12: Grove fallback behavior when no match
    let paladin_port = Arc::new(GroveMockPaladinPort::new());

    let paladins = vec![
        create_specialist_paladin("agent_0", "general", vec!["general"]),
        create_specialist_paladin("agent_1", "general", vec!["general"]),
        create_specialist_paladin("agent_2", "Rust", vec!["rust"]),
    ];

    let fallback_tree = Tree::new("Generalists")
        .add_agent(create_tree_agent("agent_0", vec!["general"]))
        .add_agent(create_tree_agent("agent_1", vec!["general"]));

    let specialist_tree =
        Tree::new("Specialists").add_agent(create_tree_agent("agent_2", vec!["rust", "cargo"]));

    let grove = GroveBuilder::new()
        .name("FallbackGrove")
        .add_tree(fallback_tree)
        .add_tree(specialist_tree)
        .routing_strategy(RoutingStrategy::KeywordMatch)
        .fallback_tree("Generalists")
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port.clone(), None, None);

    // Task that doesn't match any specialist keywords
    let task = "What's the weather like today?";
    let result = service
        .execute(&grove, &paladins, task)
        .await
        .expect("Should succeed using fallback");

    assert!(
        result.routing_decision.selected_agent == "agent_0"
            || result.routing_decision.selected_agent == "agent_1",
        "Should route to fallback tree"
    );
    assert_eq!(result.routing_decision.selected_tree, "Generalists");
}

#[tokio::test]
async fn test_grove_no_fallback_default_behavior() {
    // Test default fallback when no fallback_tree configured
    let paladin_port = Arc::new(GroveMockPaladinPort::new());

    let paladins = vec![create_specialist_paladin(
        "agent_0",
        "specific domain",
        vec!["specific"],
    )];

    let tree1 = Tree::new("Specialists").add_agent(create_tree_agent("agent_0", vec!["specific"]));

    let grove = GroveBuilder::new()
        .name("NoFallbackGrove")
        .add_tree(tree1)
        .routing_strategy(RoutingStrategy::KeywordMatch)
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port, None, None);

    // Task that doesn't match keywords
    let task = "Completely unrelated task";
    let result = service.execute(&grove, &paladins, task).await;

    // Should still succeed by routing to first tree as default
    assert!(result.is_ok(), "Should succeed with default fallback");
}

#[tokio::test]
async fn test_grove_multiple_trees() {
    // Task 8.13: Grove with multiple trees
    let paladin_port = Arc::new(GroveMockPaladinPort::new());

    // Create Paladins
    let paladins = vec![
        create_specialist_paladin("agent_0", "security", vec!["security"]),
        create_specialist_paladin("agent_1", "performance", vec!["performance"]),
        create_specialist_paladin("agent_2", "data", vec!["data"]),
        create_specialist_paladin("agent_3", "infrastructure", vec!["infra"]),
    ];

    // Create 4 different expert trees
    let security_tree = Tree::new("Security").add_agent(create_tree_agent(
        "agent_0",
        vec!["security", "vulnerability"],
    ));

    let performance_tree = Tree::new("Performance").add_agent(create_tree_agent(
        "agent_1",
        vec!["performance", "optimization"],
    ));

    let data_tree =
        Tree::new("Data").add_agent(create_tree_agent("agent_2", vec!["data", "analytics"]));

    let infrastructure_tree = Tree::new("Infrastructure").add_agent(create_tree_agent(
        "agent_3",
        vec!["infrastructure", "deployment", "kubernetes"],
    ));

    let grove = GroveBuilder::new()
        .name("MultiTreeGrove")
        .add_tree(security_tree)
        .add_tree(performance_tree)
        .add_tree(data_tree)
        .add_tree(infrastructure_tree)
        .routing_strategy(RoutingStrategy::KeywordMatch)
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port.clone(), None, None);

    // Test routing to each tree
    let tasks = vec![
        ("Fix security vulnerability in auth", "agent_0"),
        ("Optimize query performance", "agent_1"),
        ("Analyze user data trends", "agent_2"),
        ("Deploy to Kubernetes cluster", "agent_3"),
    ];

    for (task, expected_expert) in tasks {
        let result = service
            .execute(&grove, &paladins, task)
            .await
            .expect("Task should succeed");

        assert_eq!(
            result.routing_decision.selected_agent, expected_expert,
            "Task '{}' should route to {}",
            task, expected_expert
        );
    }
}

#[tokio::test]
async fn test_grove_error_handling() {
    // Test error handling when agent fails
    struct FailingMockPort;

    #[async_trait]
    impl PaladinPort for FailingMockPort {
        async fn execute(
            &self,
            paladin: &Paladin,
            _input: &str,
        ) -> Result<PaladinResult, PaladinError> {
            if paladin.node.name == "agent_1" {
                return Err(PaladinError::ExecutionError(
                    "Simulated failure".to_string(),
                ));
            }

            Ok(PaladinResult {
                output: format!("[{}]: Success", paladin.node.name),
                token_count: 50,
                execution_time_ms: 10,
                loop_count: 1,
                stop_reason: StopReason::Completed,
                ..Default::default()
            })
        }

        async fn execute_stream(
            &self,
            _paladin: &Paladin,
            _input: &str,
        ) -> Result<PaladinStream, PaladinError> {
            let (_tx, rx) = tokio::sync::mpsc::channel(1);
            Ok(rx)
        }

        fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
            Ok(())
        }
    }

    let paladin_port = Arc::new(FailingMockPort);

    let paladins = vec![
        create_specialist_paladin("agent_0", "good", vec!["good"]),
        create_specialist_paladin("agent_1", "failing", vec!["failing"]),
    ];

    let tree = Tree::new("Team")
        .add_agent(create_tree_agent("agent_0", vec!["good", "team"]))
        .add_agent(create_tree_agent("agent_1", vec!["failing", "team"]));

    let grove = GroveBuilder::new()
        .name("ErrorTestGrove")
        .add_tree(tree)
        .routing_strategy(RoutingStrategy::KeywordMatch)
        .build()
        .expect("Grove build should succeed");

    let service = GroveExecutionService::new(paladin_port, None, None);

    // Task with "failing" keyword should route to agent_1 which will fail
    let result = service
        .execute(&grove, &paladins, "failing team task")
        .await;

    // Should return an error since agent_1 fails
    assert!(result.is_err(), "Should propagate execution error");
}
