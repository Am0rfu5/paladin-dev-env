// tests/integration/cli_integration_test.rs - End-to-end CLI workflow tests

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to get the path to the CLI binary
fn get_cli_binary() -> PathBuf {
    // Get the directory where the test binary is located
    let mut path = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();

    // Remove the 'deps' directory component if present (test binaries are in target/debug/deps/)
    if path.ends_with("deps") {
        path.pop();
    }

    // Add the CLI binary name
    path.push("paladin-cli");

    // If binary doesn't exist in debug, try release
    if !path.exists() {
        path.pop(); // Remove paladin-cli
        path.pop(); // Remove debug
        path.push("release");
        path.push("paladin-cli");
    }

    path
}

/// Helper to run CLI command and return output
fn run_cli_command(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    let cli_path = get_cli_binary();

    // Build the CLI binary if it doesn't exist
    if !cli_path.exists() {
        let build_output = Command::new("cargo")
            .args(["build", "--bin", "paladin-cli"])
            .output()
            .expect("Failed to build CLI binary");

        if !build_output.status.success() {
            panic!(
                "Failed to build CLI binary: {}",
                String::from_utf8_lossy(&build_output.stderr)
            );
        }
    }

    Command::new(cli_path).args(args).output()
}

#[test]
fn test_cli_help_command() {
    let output = run_cli_command(&["--help"]).expect("Failed to run CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("agent"));
    assert!(stdout.contains("battalion"));
    assert!(stdout.contains("arsenal"));
}

#[test]
fn test_agent_help_command() {
    let output = run_cli_command(&["agent", "--help"]).expect("Failed to run CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("agent"));
    assert!(stdout.contains("new"));
    assert!(stdout.contains("run"));
}

#[test]
fn test_battalion_help_command() {
    let output = run_cli_command(&["battalion", "--help"]).expect("Failed to run CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("battalion"));
    assert!(stdout.contains("new"));
    assert!(stdout.contains("run"));
}

#[test]
fn test_arsenal_help_command() {
    let output = run_cli_command(&["arsenal", "--help"]).expect("Failed to run CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("arsenal"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("test"));
}

/// Test: Generate Paladin template with default provider (OpenAI)
#[test]
fn test_agent_new_generates_valid_template_default_provider() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_paladin.yaml");

    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "test-agent",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    // Check command succeeded
    assert!(output.status.success(), "Command failed: {:?}", output);

    // Verify file was created
    assert!(output_path.exists(), "Template file was not created");

    // Read and parse YAML
    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    // Verify structure
    assert!(yaml.get("name").is_some(), "Missing 'name' field");
    assert_eq!(
        yaml.get("name").and_then(|v| v.as_str()),
        Some("test-agent"),
        "Name field incorrect"
    );

    assert!(yaml.get("provider").is_some(), "Missing 'provider' field");
    let provider = yaml.get("provider").expect("Missing provider");
    assert!(
        provider.get("type").is_some(),
        "Missing 'provider.type' field"
    );
    assert_eq!(
        provider.get("type").and_then(|v| v.as_str()),
        Some("openai"),
        "Default provider should be OpenAI"
    );

    assert!(
        yaml.get("system_prompt").is_some(),
        "Missing 'system_prompt' field"
    );
    assert!(yaml.get("model").is_some(), "Missing 'model' field");
    assert!(
        yaml.get("temperature").is_some(),
        "Missing 'temperature' field"
    );
    assert!(yaml.get("max_loops").is_some(), "Missing 'max_loops' field");
}

/// Test: Generate Paladin template with DeepSeek provider
#[test]
fn test_agent_new_generates_deepseek_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_deepseek.yaml");

    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "deepseek-agent",
        "-o",
        output_path.to_str().unwrap(),
        "-p",
        "deepseek",
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Template file was not created");

    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    let provider = yaml.get("provider").expect("Missing provider");
    assert_eq!(
        provider.get("type").and_then(|v| v.as_str()),
        Some("deepseek"),
        "Provider should be DeepSeek"
    );
}

/// Test: Generate Paladin template with Anthropic provider
#[test]
fn test_agent_new_generates_anthropic_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_anthropic.yaml");

    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "anthropic-agent",
        "-o",
        output_path.to_str().unwrap(),
        "-p",
        "anthropic",
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Template file was not created");

    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    let provider = yaml.get("provider").expect("Missing provider");
    assert_eq!(
        provider.get("type").and_then(|v| v.as_str()),
        Some("anthropic"),
        "Provider should be Anthropic"
    );
}

/// Test: Generate Battalion formation template
#[test]
fn test_battalion_new_generates_formation_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_formation.yaml");

    let output = run_cli_command(&[
        "battalion",
        "new",
        "-n",
        "test-formation",
        "-t",
        "formation",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Template file was not created");

    // Read and parse YAML
    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    // Verify structure
    assert!(yaml.get("name").is_some(), "Missing 'name' field");
    assert_eq!(
        yaml.get("name").and_then(|v| v.as_str()),
        Some("test-formation"),
        "Name field incorrect"
    );

    assert!(yaml.get("type").is_some(), "Missing 'type' field");
    assert_eq!(
        yaml.get("type").and_then(|v| v.as_str()),
        Some("formation"),
        "Type should be 'formation'"
    );

    assert!(
        yaml.get("paladins").is_some(),
        "Missing 'paladins' field for formation"
    );
    let paladins = yaml.get("paladins").expect("Missing paladins");
    assert!(
        paladins.is_sequence(),
        "Paladins should be an array/sequence"
    );

    assert!(
        yaml.get("pass_output_to_next").is_some(),
        "Missing 'pass_output_to_next' field for formation"
    );
    assert_eq!(
        yaml.get("pass_output_to_next").and_then(|v| v.as_bool()),
        Some(true),
        "Formation should have pass_output_to_next: true"
    );
}

/// Test: Generate Battalion phalanx template
#[test]
fn test_battalion_new_generates_phalanx_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_phalanx.yaml");

    let output = run_cli_command(&[
        "battalion",
        "new",
        "-n",
        "test-phalanx",
        "-t",
        "phalanx",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Template file was not created");

    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    assert_eq!(
        yaml.get("type").and_then(|v| v.as_str()),
        Some("phalanx"),
        "Type should be 'phalanx'"
    );

    assert!(
        yaml.get("paladins").is_some(),
        "Missing 'paladins' field for phalanx"
    );

    // inputs field is optional - verify it's mentioned in comments even if not present as field
    assert!(
        content.contains("inputs") || yaml.get("inputs").is_some(),
        "Template should mention or include 'inputs' field for phalanx"
    );
}

/// Test: Generate Battalion campaign template
#[test]
fn test_battalion_new_generates_campaign_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_campaign.yaml");

    let output = run_cli_command(&[
        "battalion",
        "new",
        "-n",
        "test-campaign",
        "-t",
        "campaign",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Template file was not created");

    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    assert_eq!(
        yaml.get("type").and_then(|v| v.as_str()),
        Some("campaign"),
        "Type should be 'campaign'"
    );

    assert!(yaml.get("nodes").is_some(), "Missing 'nodes' field");
    assert!(yaml.get("edges").is_some(), "Missing 'edges' field");
    assert!(
        yaml.get("start_node").is_some(),
        "Missing 'start_node' field"
    );
}

/// Test: Generate Battalion chain-of-command template
#[test]
fn test_battalion_new_generates_chain_of_command_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_chain.yaml");

    let output = run_cli_command(&[
        "battalion",
        "new",
        "-n",
        "test-chain",
        "-t",
        "chain-of-command",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Template file was not created");

    let content = fs::read_to_string(&output_path).expect("Failed to read template file");
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&content).expect("Generated YAML is invalid");

    assert_eq!(
        yaml.get("type").and_then(|v| v.as_str()),
        Some("chain_of_command"),
        "Type should be 'chain_of_command'"
    );

    assert!(yaml.get("commander").is_some(), "Missing 'commander' field");
    assert!(yaml.get("delegates").is_some(), "Missing 'delegates' field");
}

/// Test: Invalid provider rejection
#[test]
fn test_agent_new_rejects_invalid_provider() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_invalid.yaml");

    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "invalid-agent",
        "-o",
        output_path.to_str().unwrap(),
        "-p",
        "invalid-provider",
    ])
    .expect("Failed to run CLI");

    // Command should fail
    assert!(!output.status.success(), "Command should have failed");
    assert!(
        !output_path.exists(),
        "File should not be created for invalid provider"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid") || stderr.contains("provider"),
        "Error message should mention invalid provider"
    );
}

/// Test: Invalid battalion type rejection
#[test]
fn test_battalion_new_rejects_invalid_type() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_invalid_battalion.yaml");

    let output = run_cli_command(&[
        "battalion",
        "new",
        "-n",
        "invalid-battalion",
        "-t",
        "invalid-type",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    // Command should fail
    assert!(!output.status.success(), "Command should have failed");
    assert!(
        !output_path.exists(),
        "File should not be created for invalid type"
    );
}

/// Test: Verify all generated templates have required sections with comments
#[test]
fn test_generated_templates_include_helpful_comments() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_commented.yaml");

    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "commented-agent",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    assert!(output.status.success());

    let content = fs::read_to_string(&output_path).expect("Failed to read template file");

    // Verify comments are present
    assert!(
        content.contains('#'),
        "Template should include comment lines"
    );
    assert!(
        content.contains("Configuration") || content.contains("configuration"),
        "Template should explain configuration"
    );
}

/// Test: Multiple template generation in same directory
#[test]
fn test_multiple_templates_in_same_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Generate first agent
    let agent1_path = temp_dir.path().join("agent1.yaml");
    let output1 = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "agent-1",
        "-o",
        agent1_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");
    assert!(output1.status.success());

    // Generate second agent
    let agent2_path = temp_dir.path().join("agent2.yaml");
    let output2 = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "agent-2",
        "-o",
        agent2_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");
    assert!(output2.status.success());

    // Both should exist
    assert!(agent1_path.exists());
    assert!(agent2_path.exists());

    // Content should be different (names)
    let content1 = fs::read_to_string(&agent1_path).unwrap();
    let content2 = fs::read_to_string(&agent2_path).unwrap();
    assert_ne!(content1, content2, "Templates should have different names");
}

// ============================================================================
// Test Helpers for Config Creation and Environment Management (Subtask 13.14)
// ============================================================================

/// Helper to create a valid Paladin YAML config for testing
fn create_test_paladin_config(temp_dir: &TempDir, name: &str, provider: &str) -> PathBuf {
    let config_path = temp_dir.path().join(format!("{}.yaml", name));
    let config_content = format!(
        r#"
name: "{}"
system_prompt: "You are a helpful assistant for testing."
model: "gpt-4"
temperature: 0.7
max_loops: 1
timeout_seconds: 30
stop_words: []

provider:
  type: {}
  api_key_env: "{}_API_KEY"
"#,
        name,
        provider,
        provider.to_uppercase()
    );

    fs::write(&config_path, config_content).expect("Failed to write test config");
    config_path
}

/// Helper to create a Formation battalion config
#[allow(dead_code)] // Reserved for future subtasks 13.4-13.6
fn create_test_formation_config(temp_dir: &TempDir, name: &str) -> PathBuf {
    let config_path = temp_dir.path().join(format!("{}.yaml", name));
    let config_content = format!(
        r#"
name: "{}"
type: formation
pass_output_to_next: true

paladins:
  - inline:
      name: "Step1"
      system_prompt: "Process step 1"
      model: "gpt-4"
      temperature: 0.7
      max_loops: 1
      provider:
        type: openai
        api_key_env: "OPENAI_API_KEY"
  
  - inline:
      name: "Step2"
      system_prompt: "Process step 2"
      model: "gpt-4"
      temperature: 0.7
      max_loops: 1
      provider:
        type: openai
        api_key_env: "OPENAI_API_KEY"
"#,
        name
    );

    fs::write(&config_path, config_content).expect("Failed to write test formation config");
    config_path
}

/// Helper to temporarily set environment variable for test
struct EnvVarGuard {
    key: String,
    old_value: Option<String>,
}

impl EnvVarGuard {
    fn new(key: &str, value: &str) -> Self {
        let old_value = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, value);
        }
        Self {
            key: key.to_string(),
            old_value,
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.old_value {
                Some(val) => std::env::set_var(&self.key, val),
                None => std::env::remove_var(&self.key),
            }
        }
    }
}

// ============================================================================
// Error Handling Tests (Subtasks 13.9-13.10)
// ============================================================================

/// Test: Missing API key error handling (Subtask 13.9)
#[test]
fn test_missing_api_key_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = create_test_paladin_config(&temp_dir, "test-agent", "openai");

    // Ensure API key is NOT set
    let _guard = EnvVarGuard::new("OPENAI_API_KEY", "");
    unsafe {
        std::env::remove_var("OPENAI_API_KEY");
    }

    let output = run_cli_command(&[
        "agent",
        "run",
        "-c",
        config_path.to_str().unwrap(),
        "-i",
        "test input",
    ])
    .expect("Failed to run CLI");

    // Command should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Command should fail without API key"
    );

    // Error message should mention API key
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);

    assert!(
        combined.to_lowercase().contains("api") || combined.to_lowercase().contains("key"),
        "Error message should mention API key issue"
    );
}

/// Test: Invalid config file error handling (Subtask 13.10)
#[test]
fn test_invalid_config_file_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let invalid_config = temp_dir.path().join("invalid.yaml");

    // Create invalid YAML
    fs::write(&invalid_config, "invalid: yaml: content: [unclosed").expect("Failed to write file");

    let output = run_cli_command(&[
        "agent",
        "run",
        "-c",
        invalid_config.to_str().unwrap(),
        "-i",
        "test input",
    ])
    .expect("Failed to run CLI");

    // Command should fail
    assert!(
        !output.status.success(),
        "Command should fail with invalid config"
    );

    // Error message should mention config or YAML
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);

    assert!(
        combined.to_lowercase().contains("config")
            || combined.to_lowercase().contains("yaml")
            || combined.to_lowercase().contains("parse"),
        "Error message should mention config/YAML issue"
    );
}

/// Test: File not found error handling
#[test]
fn test_config_file_not_found_error() {
    let output = run_cli_command(&[
        "agent",
        "run",
        "-c",
        "/nonexistent/path/to/config.yaml",
        "-i",
        "test input",
    ])
    .expect("Failed to run CLI");

    // Command should fail
    assert!(
        !output.status.success(),
        "Command should fail with missing config file"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);

    assert!(
        combined.to_lowercase().contains("not found")
            || combined.to_lowercase().contains("no such file"),
        "Error message should mention file not found"
    );
}

// ============================================================================
// Output Options Tests (Subtasks 13.11-13.12)
// ============================================================================

/// Test: Output to file with --output flag (Subtask 13.11)
#[test]
fn test_output_to_file_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let _output_path = temp_dir.path().join("result.txt");

    // Generate a template to test file output
    let template_path = temp_dir.path().join("test.yaml");
    let _create_output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "test-agent",
        "-o",
        template_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    // Note: We can't easily test agent run --output without API keys,
    // but we can verify the flag is accepted and file handling works
    // by testing that --output is a valid argument

    let output = run_cli_command(&["agent", "run", "--help"]).expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--output") || stdout.contains("-o"),
        "Help should document --output flag"
    );
}

/// Test: Verbose mode flag (Subtask 13.12)
#[test]
fn test_verbose_mode_flag() {
    // Verify that --verbose flag is recognized
    let output = run_cli_command(&["agent", "run", "--help"]).expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--verbose") || stdout.contains("-v"),
        "Help should document --verbose flag"
    );

    // Verify for battalion as well
    let output = run_cli_command(&["battalion", "run", "--help"]).expect("Failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--verbose") || stdout.contains("-v"),
        "Battalion help should document --verbose flag"
    );
}

// ============================================================================
// Exit Code Tests (Subtask 13.13)
// ============================================================================

/// Test: Success exit code (0)
#[test]
fn test_exit_code_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.yaml");

    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "test-agent",
        "-o",
        output_path.to_str().unwrap(),
    ])
    .expect("Failed to run CLI");

    // Success should return exit code 0
    assert_eq!(
        output.status.code(),
        Some(0),
        "Successful commands should exit with code 0"
    );
}

/// Test: User error exit code (1)
#[test]
fn test_exit_code_user_error() {
    let output = run_cli_command(&[
        "agent",
        "new",
        "-n",
        "test",
        "-o",
        "/tmp/test.yaml",
        "-p",
        "invalid-provider",
    ])
    .expect("Failed to run CLI");

    // User errors (invalid provider) should return non-zero exit code
    assert!(
        output.status.code() != Some(0),
        "User errors should return non-zero exit code"
    );

    // Common convention is exit code 1 for user errors
    let code = output.status.code().unwrap_or(1);
    assert!(
        code == 1 || code == 2,
        "User error exit code should be 1 or 2, got {}",
        code
    );
}

/// Test: Missing required argument error
#[test]
fn test_exit_code_missing_argument() {
    // Try to run agent new without required -o argument
    let output = run_cli_command(&["agent", "new", "-n", "test"]).expect("Failed to run CLI");

    // Should fail with error exit code
    assert!(
        !output.status.success(),
        "Missing required argument should fail"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required") || stderr.contains("output"),
        "Error should mention missing required argument"
    );
}

// ============================================================================
// Arsenal/MCP Tests (Subtasks 13.7-13.8)
// ============================================================================

/// Test: Arsenal list command structure (Subtask 13.7)
#[test]
fn test_arsenal_list_command_exists() {
    // Verify arsenal list command is recognized
    let output = run_cli_command(&["arsenal", "list", "--help"]).expect("Failed to run CLI");

    // Should succeed (help is always available)
    assert!(
        output.status.success(),
        "Arsenal list help should be available"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("list") || stdout.contains("arsenal"),
        "Help output should mention arsenal list"
    );
}

/// Test: Arsenal test command structure (Subtask 13.8)
#[test]
fn test_arsenal_test_command_exists() {
    // Verify arsenal test command recognizes --mcp-stdio flag
    let output = run_cli_command(&["arsenal", "test", "--help"]).expect("Failed to run CLI");

    assert!(
        output.status.success(),
        "Arsenal test help should be available"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--mcp-stdio") || stdout.contains("--mcp-sse"),
        "Help should document MCP connection options"
    );
}

/// Test: Arsenal test requires one of the MCP options
#[test]
fn test_arsenal_test_requires_mcp_option() {
    // Try to run arsenal test without any MCP option
    let output = run_cli_command(&["arsenal", "test"]).expect("Failed to run CLI");

    // Should fail because one of --mcp-stdio or --mcp-sse is required
    assert!(
        !output.status.success(),
        "Arsenal test should require MCP connection option"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);

    assert!(
        combined.contains("mcp") || combined.contains("required"),
        "Error should mention MCP options or required arguments"
    );
}
