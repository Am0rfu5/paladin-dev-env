//! Unit tests for Maneuver domain model

use paladin::core::platform::container::battalion::maneuver::{
    ErrorStrategy, ExecutionStatus, Maneuver, ManeuverConfig, ManeuverError, ManeuverResult,
    OutputFormat,
};
use paladin::core::platform::container::battalion::parser::FlowParser;
use paladin::core::platform::container::paladin::{MaxLoops, Paladin, PaladinData, PaladinStatus};
use std::collections::HashMap;
use std::time::Duration;

fn create_test_paladin(name: &str) -> Paladin {
    let data = PaladinData {
        system_prompt: format!("Test paladin {}", name),
        name: name.to_string(),
        user_name: "test".to_string(),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_loops: MaxLoops::Fixed(1),
        stop_words: vec![],
        status: PaladinStatus::Idle,
        vision_enabled: false,
    };
    Paladin::new(data, None)
}

#[test]
fn test_maneuver_config_defaults() {
    let config = ManeuverConfig::default();
    assert_eq!(config.error_strategy, ErrorStrategy::FailFast);
    assert_eq!(config.output_format, OutputFormat::Concatenate);
    assert!(config.pass_output_as_input);
    assert_eq!(config.timeout, Some(Duration::from_secs(300)));
    assert!(config.collect_timing_metrics);
    assert!(!config.detailed_observability);
}

#[test]
fn test_maneuver_config_builder_pattern() {
    let config = ManeuverConfig::new()
        .with_error_strategy(ErrorStrategy::ContinueParallel)
        .with_output_format(OutputFormat::JsonArray)
        .with_pass_output_as_input(false)
        .with_timeout(Duration::from_secs(120))
        .with_timing_metrics(false)
        .with_detailed_observability(true);

    assert_eq!(config.error_strategy, ErrorStrategy::ContinueParallel);
    assert_eq!(config.output_format, OutputFormat::JsonArray);
    assert!(!config.pass_output_as_input);
    assert_eq!(config.timeout, Some(Duration::from_secs(120)));
    assert!(!config.collect_timing_metrics);
    assert!(config.detailed_observability);
}

#[test]
fn test_maneuver_config_without_timeout() {
    let config = ManeuverConfig::new().without_timeout();
    assert!(config.timeout.is_none());
}

#[test]
fn test_maneuver_config_validation_success() {
    let config = ManeuverConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_maneuver_config_validation_zero_timeout() {
    let config = ManeuverConfig::default().with_timeout(Duration::from_secs(0));
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("greater than zero"));
}

#[test]
fn test_maneuver_construction_with_valid_flow() {
    let flow = FlowParser::parse("agent1 -> agent2").unwrap();
    let mut agents = HashMap::new();
    agents.insert("agent1".to_string(), create_test_paladin("agent1"));
    agents.insert("agent2".to_string(), create_test_paladin("agent2"));

    let maneuver = Maneuver::new("test_maneuver", agents, flow, ManeuverConfig::default());

    assert!(maneuver.is_ok());
    let maneuver = maneuver.unwrap();
    assert_eq!(maneuver.name, "test_maneuver");
    assert_eq!(maneuver.agent_count(), 2);
}

#[test]
fn test_maneuver_validation_missing_agent() {
    let flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
    let mut agents = HashMap::new();
    agents.insert("agent1".to_string(), create_test_paladin("agent1"));
    agents.insert("agent2".to_string(), create_test_paladin("agent2"));
    // agent3 is missing

    let result = Maneuver::new("test_maneuver", agents, flow, ManeuverConfig::default());

    assert!(result.is_err());
    match result.unwrap_err() {
        ManeuverError::AgentNotFound {
            agent_name,
            available_agents,
        } => {
            assert_eq!(agent_name, "agent3");
            assert!(available_agents.contains(&"agent1".to_string()));
            assert!(available_agents.contains(&"agent2".to_string()));
        }
        _ => panic!("Expected AgentNotFound error"),
    }
}

#[test]
fn test_maneuver_validation_excessive_depth() {
    // Create a deeply nested flow (depth > 5)
    let flow = FlowParser::parse("a -> (b -> (c -> (d -> (e -> (f -> g)))))").unwrap();
    let mut agents = HashMap::new();
    for name in &["a", "b", "c", "d", "e", "f", "g"] {
        agents.insert(name.to_string(), create_test_paladin(name));
    }

    let result = Maneuver::new("deep_maneuver", agents, flow, ManeuverConfig::default());

    assert!(result.is_err());
    match result.unwrap_err() {
        ManeuverError::ValidationError(msg) => {
            assert!(msg.contains("depth"));
            assert!(msg.contains("exceeds maximum of 5"));
        }
        _ => panic!("Expected ValidationError for depth"),
    }
}

#[test]
fn test_maneuver_validation_excessive_agent_count() {
    // Create flow with > 30 agents
    let agent_names: Vec<String> = (1..=31).map(|i| format!("agent{}", i)).collect();
    let flow_str = agent_names.join(", ");
    let flow = FlowParser::parse(&flow_str).unwrap();

    let mut agents = HashMap::new();
    for name in &agent_names {
        agents.insert(name.clone(), create_test_paladin(name));
    }

    let result = Maneuver::new("wide_maneuver", agents, flow, ManeuverConfig::default());

    assert!(result.is_err());
    match result.unwrap_err() {
        ManeuverError::ValidationError(msg) => {
            assert!(msg.contains("31 agents"));
            assert!(msg.contains("exceeds maximum of 30"));
        }
        _ => panic!("Expected ValidationError for agent count"),
    }
}

#[test]
fn test_maneuver_depth_calculation() {
    let flow = FlowParser::parse("a -> (b -> c)").unwrap();
    let mut agents = HashMap::new();
    agents.insert("a".to_string(), create_test_paladin("a"));
    agents.insert("b".to_string(), create_test_paladin("b"));
    agents.insert("c".to_string(), create_test_paladin("c"));

    let maneuver = Maneuver::new("test", agents, flow, ManeuverConfig::default()).unwrap();

    assert_eq!(maneuver.depth(), 3);
}

#[test]
fn test_maneuver_width_calculation() {
    let flow = FlowParser::parse("a -> b, c, d").unwrap();
    let mut agents = HashMap::new();
    agents.insert("a".to_string(), create_test_paladin("a"));
    agents.insert("b".to_string(), create_test_paladin("b"));
    agents.insert("c".to_string(), create_test_paladin("c"));
    agents.insert("d".to_string(), create_test_paladin("d"));

    let maneuver = Maneuver::new("test", agents, flow, ManeuverConfig::default()).unwrap();

    assert_eq!(maneuver.width(), 3); // b, c, d in parallel
}

#[test]
fn test_maneuver_result_construction() {
    let mut step_outputs = HashMap::new();
    step_outputs.insert("agent1".to_string(), "output1".to_string());
    step_outputs.insert("agent2".to_string(), "output2".to_string());

    let result = ManeuverResult::new(
        "final output".to_string(),
        step_outputs,
        vec!["agent1".to_string(), "agent2".to_string()],
    );

    assert_eq!(result.final_output, "final output");
    assert_eq!(result.execution_order.len(), 2);
    assert_eq!(result.status, ExecutionStatus::Success);
    assert!(result.timing_metrics.is_none());
}

#[test]
fn test_maneuver_result_with_timing_metrics() {
    let mut step_outputs = HashMap::new();
    step_outputs.insert("agent1".to_string(), "output1".to_string());

    let mut timing = HashMap::new();
    timing.insert("agent1".to_string(), Duration::from_millis(150));
    timing.insert("agent2".to_string(), Duration::from_millis(200));

    let result = ManeuverResult::with_timing(
        "final".to_string(),
        step_outputs,
        vec!["agent1".to_string()],
        timing,
    );

    assert!(result.timing_metrics.is_some());
    assert_eq!(result.total_duration(), Some(Duration::from_millis(350)));
}

#[test]
fn test_maneuver_result_with_status() {
    let result = ManeuverResult::new("output".to_string(), HashMap::new(), vec![])
        .with_status(ExecutionStatus::PartialSuccess);

    assert_eq!(result.status, ExecutionStatus::PartialSuccess);
}

#[test]
fn test_maneuver_result_get_agent_output() {
    let mut step_outputs = HashMap::new();
    step_outputs.insert("agent1".to_string(), "output1".to_string());
    step_outputs.insert("agent2".to_string(), "output2".to_string());

    let result = ManeuverResult::new(
        "final".to_string(),
        step_outputs,
        vec!["agent1".to_string(), "agent2".to_string()],
    );

    assert_eq!(
        result.get_agent_output("agent1"),
        Some(&"output1".to_string())
    );
    assert_eq!(
        result.get_agent_output("agent2"),
        Some(&"output2".to_string())
    );
    assert_eq!(result.get_agent_output("agent3"), None);
}

#[test]
fn test_maneuver_result_total_duration_none() {
    let result = ManeuverResult::new("output".to_string(), HashMap::new(), vec![]);
    assert_eq!(result.total_duration(), None);
}

#[test]
fn test_error_strategy_serialization() {
    let strategies = vec![
        ErrorStrategy::FailFast,
        ErrorStrategy::ContinueParallel,
        ErrorStrategy::IgnoreErrors,
    ];

    for strategy in strategies {
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: ErrorStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(strategy, deserialized);
    }
}

#[test]
fn test_output_format_serialization() {
    let formats = vec![OutputFormat::Concatenate, OutputFormat::JsonArray];

    for format in formats {
        let json = serde_json::to_string(&format).unwrap();
        let deserialized: OutputFormat = serde_json::from_str(&json).unwrap();
        assert_eq!(format, deserialized);
    }
}

#[test]
fn test_maneuver_config_serialization() {
    let config = ManeuverConfig::new()
        .with_error_strategy(ErrorStrategy::ContinueParallel)
        .with_timeout(Duration::from_secs(180));

    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ManeuverConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.error_strategy, deserialized.error_strategy);
    assert_eq!(config.timeout, deserialized.timeout);
}

#[test]
fn test_execution_status_values() {
    assert_eq!(ExecutionStatus::Success, ExecutionStatus::Success);
    assert_ne!(ExecutionStatus::Success, ExecutionStatus::Failed);
    assert_ne!(ExecutionStatus::PartialSuccess, ExecutionStatus::Failed);
}

#[test]
fn test_maneuver_with_complex_flow() {
    // Test: a -> (b, c -> d), e
    let flow = FlowParser::parse("a -> (b, c -> d), e").unwrap();
    let mut agents = HashMap::new();
    for name in &["a", "b", "c", "d", "e"] {
        agents.insert(name.to_string(), create_test_paladin(name));
    }

    let maneuver = Maneuver::new("complex", agents, flow, ManeuverConfig::default());

    assert!(maneuver.is_ok());
    let maneuver = maneuver.unwrap();
    assert_eq!(maneuver.agent_count(), 5);
    assert!(maneuver.depth() >= 3);
}
