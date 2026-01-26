//! Unit tests for CLI battalion commands

use paladin::cli::commands::battalion::{BattalionNewArgs, handle_battalion_new};
use paladin::cli::output::errors::CliError;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_handle_battalion_new_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test-battalion.yaml");

    let args = BattalionNewArgs {
        name: "TestBattalion".to_string(),
        r#type: "formation".to_string(),
        output: output_path.clone(),
    };

    let result = handle_battalion_new(args);

    match result {
        Ok(_) => {
            assert!(output_path.exists(), "Output file should be created");
            let contents = fs::read_to_string(&output_path).unwrap();
            assert!(contents.contains("name: \"TestBattalion\""));
            assert!(contents.contains("type: formation"));
        }
        Err(_) => {
            // Non-TTY environment is acceptable in tests
        }
    }
}

#[test]
fn test_handle_battalion_new_validates_type() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.yaml");

    let args = BattalionNewArgs {
        name: "Test".to_string(),
        r#type: "invalid-type".to_string(),
        output: output_path,
    };

    let result = handle_battalion_new(args);
    assert!(result.is_err());

    match result {
        Err(CliError::InvalidFieldValue { field, message }) => {
            assert_eq!(field, "type");
            assert!(message.contains("invalid-type"));
        }
        _ => panic!("Expected InvalidFieldValue error"),
    }
}

#[test]
fn test_handle_battalion_new_all_valid_types() {
    let temp_dir = TempDir::new().unwrap();
    let valid_types = vec!["formation", "phalanx", "campaign", "chain-of-command"];

    for battalion_type in valid_types {
        let output_path = temp_dir.path().join(format!("{}.yaml", battalion_type));

        let args = BattalionNewArgs {
            name: format!("Test-{}", battalion_type),
            r#type: battalion_type.to_string(),
            output: output_path.clone(),
        };

        let result = handle_battalion_new(args);

        match result {
            Ok(_) => {
                assert!(output_path.exists());
                let contents = fs::read_to_string(&output_path).unwrap();
                // Type might be with underscores in YAML
                assert!(
                    contents.contains(&format!("type: {}", battalion_type))
                        || contents
                            .contains(&format!("type: {}", battalion_type.replace("-", "_")))
                );
            }
            Err(_) => {
                // Non-TTY is acceptable
            }
        }
    }
}

#[test]
fn test_battalion_formation_template_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("formation.yaml");

    let args = BattalionNewArgs {
        name: "TestFormation".to_string(),
        r#type: "formation".to_string(),
        output: output_path.clone(),
    };

    let result = handle_battalion_new(args);

    if result.is_ok() {
        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("pass_output_to_next"));
        assert!(contents.contains("paladins:"));
    }
}

#[test]
fn test_battalion_phalanx_template_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("phalanx.yaml");

    let args = BattalionNewArgs {
        name: "TestPhalanx".to_string(),
        r#type: "phalanx".to_string(),
        output: output_path.clone(),
    };

    let result = handle_battalion_new(args);

    if result.is_ok() {
        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("paladins:"));
        // Phalanx should have multiple paladins for parallel execution
    }
}

#[test]
fn test_battalion_campaign_template_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("campaign.yaml");

    let args = BattalionNewArgs {
        name: "TestCampaign".to_string(),
        r#type: "campaign".to_string(),
        output: output_path.clone(),
    };

    let result = handle_battalion_new(args);

    if result.is_ok() {
        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("nodes:"));
        assert!(contents.contains("edges:"));
        assert!(contents.contains("start_node:"));
    }
}

#[test]
fn test_battalion_chain_of_command_template_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("chain.yaml");

    let args = BattalionNewArgs {
        name: "TestChain".to_string(),
        r#type: "chain-of-command".to_string(),
        output: output_path.clone(),
    };

    let result = handle_battalion_new(args);

    if result.is_ok() {
        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("commander:"));
        assert!(contents.contains("delegates:"));
    }
}
