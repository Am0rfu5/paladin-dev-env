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
            .args(&["build", "--bin", "paladin-cli"])
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
