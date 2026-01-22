use paladin::application::use_cases::paladin::error::PaladinError;

#[test]
fn test_paladin_error_messages() {
    // Test each error variant has correct message formatting
    let config_err = PaladinError::ConfigurationError("bad config".to_string());
    assert_eq!(config_err.to_string(), "Configuration error: bad config");

    let exec_err = PaladinError::ExecutionError("execution failed".to_string());
    assert_eq!(exec_err.to_string(), "Execution error: execution failed");

    let llm_err = PaladinError::LlmError("rate limited".to_string());
    assert_eq!(llm_err.to_string(), "LLM error: rate limited");

    let timeout_err = PaladinError::Timeout(120);
    assert_eq!(timeout_err.to_string(), "Timeout after 120 seconds");

    let stop_word_err = PaladinError::StopWordDetected("STOP".to_string());
    assert_eq!(stop_word_err.to_string(), "Stop word detected: STOP");

    let circuit_err = PaladinError::CircuitBreakerOpen;
    assert_eq!(
        circuit_err.to_string(),
        "Circuit breaker open: too many failures"
    );

    let retry_err = PaladinError::MaxRetriesExceeded(5);
    assert_eq!(retry_err.to_string(), "Maximum retry attempts (5) exceeded");
}

#[test]
fn test_error_is_retryable() {
    // Retryable errors
    assert!(PaladinError::LlmError("temp".to_string()).is_retryable());
    assert!(PaladinError::ExecutionError("temp".to_string()).is_retryable());

    // Non-retryable errors
    assert!(!PaladinError::ConfigurationError("temp".to_string()).is_retryable());
    assert!(!PaladinError::Timeout(100).is_retryable());
    assert!(!PaladinError::StopWordDetected("STOP".to_string()).is_retryable());
    assert!(!PaladinError::CircuitBreakerOpen.is_retryable());
    assert!(!PaladinError::MaxRetriesExceeded(3).is_retryable());
}

#[test]
fn test_error_is_terminal() {
    // Terminal errors (cannot continue)
    assert!(PaladinError::Timeout(100).is_terminal());
    assert!(PaladinError::StopWordDetected("STOP".to_string()).is_terminal());
    assert!(PaladinError::CircuitBreakerOpen.is_terminal());
    assert!(PaladinError::MaxRetriesExceeded(3).is_terminal());

    // Non-terminal errors (can potentially retry or recover)
    assert!(!PaladinError::LlmError("temp".to_string()).is_terminal());
    assert!(!PaladinError::ExecutionError("temp".to_string()).is_terminal());
    assert!(!PaladinError::ConfigurationError("temp".to_string()).is_terminal());
}

#[test]
fn test_error_classification_consistency() {
    // Verify that retryable and terminal are mutually exclusive for logical errors
    let config_err = PaladinError::ConfigurationError("test".to_string());
    assert!(!config_err.is_retryable());
    assert!(!config_err.is_terminal());

    let timeout_err = PaladinError::Timeout(100);
    assert!(!timeout_err.is_retryable());
    assert!(timeout_err.is_terminal());

    let llm_err = PaladinError::LlmError("test".to_string());
    assert!(llm_err.is_retryable());
    assert!(!llm_err.is_terminal());
}

#[test]
fn test_error_debug_formatting() {
    let error = PaladinError::ExecutionError("test error".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ExecutionError"));
    assert!(debug_str.contains("test error"));
}

#[test]
fn test_all_error_variants_covered() {
    // Ensure we can construct all variants
    let _config = PaladinError::ConfigurationError("test".to_string());
    let _exec = PaladinError::ExecutionError("test".to_string());
    let _llm = PaladinError::LlmError("test".to_string());
    let _timeout = PaladinError::Timeout(100);
    let _stop = PaladinError::StopWordDetected("STOP".to_string());
    let _circuit = PaladinError::CircuitBreakerOpen;
    let _retry = PaladinError::MaxRetriesExceeded(3);
}
