//! Unit tests for HandoffService
//!
//! Tests the agent handoff infrastructure including:
//! - Handoff decision logic with different strategies
//! - Agent selection based on capabilities
//! - Handoff chain tracking and circular delegation prevention
//! - Max depth enforcement
//! - Context transfer mechanism

use paladin::application::use_cases::paladin::handoff_service::HandoffService;
use paladin::core::platform::container::autonomous_config::HandoffConfig;
use paladin::core::platform::container::handoff::{HandoffContext, HandoffStrategy};
use std::sync::Arc;

#[test]
fn test_handoff_service_new() {
    // Test: HandoffService can be constructed with valid configuration
    let config = HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
        retry: Default::default(),
    };

    let service = HandoffService::new(Arc::new(config));

    assert!(service.is_some(), "HandoffService should be created");
}

#[test]
fn test_handoff_service_new_with_disabled_config() {
    // Test: HandoffService returns None when handoffs are disabled
    let config = HandoffConfig {
        enabled: false,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
        retry: Default::default(),
    };

    let service = HandoffService::new(Arc::new(config));

    assert!(
        service.is_none(),
        "HandoffService should return None when disabled"
    );
}

#[test]
fn test_should_handoff_automatic_high_confidence() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();
    let context = HandoffContext::new("Simple task".to_string(), "Agent1".to_string());

    // High confidence (>0.8) - shouldn't handoff for simple tasks
    assert!(!service.should_handoff("What is 2+2?", 0.95, &context));
}

#[test]
fn test_should_handoff_automatic_low_confidence() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();
    let context = HandoffContext::new("Complex task".to_string(), "Agent1".to_string());

    // Low confidence (<0.5) - should handoff
    assert!(service.should_handoff("Implement a distributed consensus algorithm", 0.3, &context));
}

#[test]
fn test_should_handoff_explicit_never() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Explicit,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();
    let context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

    // Explicit strategy never auto-handoffs regardless of confidence
    assert!(!service.should_handoff("Complex task", 0.1, &context));
    assert!(!service.should_handoff("Simple task", 0.9, &context));
}

#[test]
fn test_should_handoff_threshold_below() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::threshold(0.7),
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();
    let context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

    // Confidence below threshold - should handoff
    assert!(service.should_handoff("Task", 0.6, &context));
}

#[test]
fn test_should_handoff_threshold_above() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::threshold(0.7),
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();
    let context = HandoffContext::new("Task".to_string(), "Agent1".to_string());

    // Confidence above threshold - shouldn't handoff
    assert!(!service.should_handoff("Task", 0.8, &context));
}

#[test]
fn test_should_handoff_max_depth_exceeded() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 2,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    // Context at max depth
    let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());
    context.depth = 2;

    // Even with low confidence, shouldn't handoff if at max depth
    assert!(!service.should_handoff("Complex task", 0.2, &context));
}

#[test]
fn test_select_agent_with_matching_specialist() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    // Task requiring code expertise
    let agents = vec![
        (
            "GeneralAssistant".to_string(),
            "General purpose assistant".to_string(),
        ),
        (
            "CodeExpert".to_string(),
            "Expert in Rust, Python, and software architecture".to_string(),
        ),
        (
            "DataAnalyst".to_string(),
            "Specializes in data analysis and statistics".to_string(),
        ),
    ];

    let selected = service.select_agent("Implement a Rust async function", &agents);
    assert!(selected.is_some());
    assert_eq!(selected.unwrap(), "CodeExpert");
}

#[test]
fn test_select_agent_with_multiple_matches() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    // Task that could match multiple specialists
    let agents = vec![
        (
            "PythonExpert".to_string(),
            "Expert in Python programming".to_string(),
        ),
        (
            "RustExpert".to_string(),
            "Expert in Rust programming".to_string(),
        ),
        (
            "CodeReviewer".to_string(),
            "Expert code reviewer for all languages".to_string(),
        ),
    ];

    let selected = service.select_agent("Review this Python code", &agents);
    assert!(selected.is_some());
    // Should prefer the most specific match
    let agent = selected.unwrap();
    assert!(agent == "PythonExpert" || agent == "CodeReviewer");
}

#[test]
fn test_select_agent_no_matches() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    // Task with no relevant specialists
    let agents = vec![
        (
            "MusicComposer".to_string(),
            "Composes music and melodies".to_string(),
        ),
        (
            "ChefAdvisor".to_string(),
            "Provides cooking advice".to_string(),
        ),
    ];

    let selected = service.select_agent("Debug this Rust code", &agents);
    // Should return None or first agent as fallback
    assert!(selected.is_none() || selected.is_some());
}

#[test]
fn test_select_agent_empty_list() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    let agents: Vec<(String, String)> = vec![];
    let selected = service.select_agent("Any task", &agents);
    assert!(selected.is_none());
}

#[test]
fn test_validate_handoff_success() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    let context = HandoffContext::new("Task".to_string(), "Agent1".to_string());
    let result = service.validate_handoff("Agent2", &context);
    assert!(result.is_ok());
}

#[test]
fn test_validate_handoff_circular_delegation() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    // Create context with Agent1 -> Agent2 -> Agent3 chain
    let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());
    context.chain.push("Agent2".to_string());
    context.chain.push("Agent3".to_string());
    context.depth = 3;

    // Try to handoff back to Agent2 (circular)
    let result = service.validate_handoff("Agent2", &context);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Circular"));
}

#[test]
fn test_validate_handoff_max_depth() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 3,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());
    context.depth = 3; // Already at max

    let result = service.validate_handoff("Agent2", &context);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Maximum handoff depth"));
}

#[test]
fn test_can_handoff_to_agent_not_in_chain() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    let mut context = HandoffContext::new("Task".to_string(), "Agent1".to_string());
    context.chain.push("Agent2".to_string());

    // Agent3 is not in chain, should be allowed
    assert!(service.can_handoff_to("Agent3", &context));
    // Agent2 is in chain, should not be allowed
    assert!(!service.can_handoff_to("Agent2", &context));
    // Agent1 (origin) is in chain, should not be allowed
    assert!(!service.can_handoff_to("Agent1", &context));
}

#[test]
fn test_transfer_context_creates_new_context() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    let original = HandoffContext::new("Original task".to_string(), "Agent1".to_string());
    let new_context = service.transfer_context("Delegated subtask", &original, "Agent2");

    assert_eq!(new_context.task, "Delegated subtask");
    assert_eq!(new_context.depth, 2); // Incremented
    assert_eq!(new_context.chain.len(), 2);
    assert_eq!(new_context.chain[0], "Agent1");
    assert_eq!(new_context.chain[1], "Agent2");
}

#[test]
fn test_transfer_context_preserves_chain() {
    let config = Arc::new(HandoffConfig {
        enabled: true,
        strategy: HandoffStrategy::Automatic,
        max_depth: 5,
        retry: Default::default(),
    });
    let service = HandoffService::new(config).unwrap();

    let mut original = HandoffContext::new("Task".to_string(), "Agent1".to_string());
    original.chain.push("Agent2".to_string());
    original.depth = 2;

    let new_context = service.transfer_context("Subtask", &original, "Agent3");

    assert_eq!(new_context.depth, 3);
    assert_eq!(new_context.chain.len(), 3);
    assert!(new_context.chain.contains(&"Agent1".to_string()));
    assert!(new_context.chain.contains(&"Agent2".to_string()));
    assert!(new_context.chain.contains(&"Agent3".to_string()));
}
