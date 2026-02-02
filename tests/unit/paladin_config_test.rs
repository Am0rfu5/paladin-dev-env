use paladin::core::platform::container::paladin_config::{OutputFormat, PaladinConfig};

#[test]
fn test_paladin_config_defaults() {
    let config = PaladinConfig::default();

    assert_eq!(config.retry_attempts, 3);
    assert_eq!(config.timeout_seconds, 300);
    assert!(!config.enable_planning);
    assert_eq!(config.planning_prompt, None);
    assert_eq!(config.output_format, OutputFormat::Text);
}

#[test]
fn test_paladin_config_builder() {
    let config = PaladinConfig::builder()
        .retry_attempts(5)
        .timeout_seconds(600)
        .enable_planning(true)
        .planning_prompt("Think step by step")
        .output_format(OutputFormat::Json)
        .build()
        .unwrap();

    assert_eq!(config.retry_attempts, 5);
    assert_eq!(config.timeout_seconds, 600);
    assert!(config.enable_planning);
    assert_eq!(
        config.planning_prompt,
        Some("Think step by step".to_string())
    );
    assert_eq!(config.output_format, OutputFormat::Json);
}

#[test]
fn test_paladin_config_builder_partial() {
    let config = PaladinConfig::builder().retry_attempts(10).build().unwrap();

    assert_eq!(config.retry_attempts, 10);
    assert_eq!(config.timeout_seconds, 300); // default
    assert!(!config.enable_planning); // default
    assert_eq!(config.output_format, OutputFormat::Text); // default
}

#[test]
fn test_paladin_config_builder_all_options() {
    let config = PaladinConfig::builder()
        .retry_attempts(7)
        .timeout_seconds(1200)
        .enable_planning(false)
        .planning_prompt("Custom prompt")
        .output_format(OutputFormat::Structured)
        .build()
        .unwrap();

    assert_eq!(config.retry_attempts, 7);
    assert_eq!(config.timeout_seconds, 1200);
    assert!(!config.enable_planning);
    assert_eq!(config.planning_prompt, Some("Custom prompt".to_string()));
    assert_eq!(config.output_format, OutputFormat::Structured);
}

#[test]
fn test_output_format_equality() {
    assert_eq!(OutputFormat::Text, OutputFormat::Text);
    assert_eq!(OutputFormat::Json, OutputFormat::Json);
    assert_eq!(OutputFormat::Structured, OutputFormat::Structured);

    assert_ne!(OutputFormat::Text, OutputFormat::Json);
    assert_ne!(OutputFormat::Json, OutputFormat::Structured);
}

#[test]
fn test_paladin_config_serialization() {
    let config = PaladinConfig {
        retry_attempts: 5,
        timeout_seconds: 600,
        enable_planning: true,
        planning_prompt: Some("test prompt".to_string()),
        output_format: OutputFormat::Json,
        autonomous: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&config).expect("Failed to serialize");

    // Deserialize back
    let deserialized: PaladinConfig = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(config.retry_attempts, deserialized.retry_attempts);
    assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    assert_eq!(config.enable_planning, deserialized.enable_planning);
    assert_eq!(config.planning_prompt, deserialized.planning_prompt);
    assert_eq!(config.output_format, deserialized.output_format);
}
