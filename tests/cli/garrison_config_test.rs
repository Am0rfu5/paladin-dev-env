/// Unit tests for garrison configuration parsing and instantiation
use paladin::application::cli::config::loader::instantiate_garrison;
use paladin::application::cli::config::paladin_config::{GarrisonConfig, GarrisonTypeConfig};
use paladin::application::cli::error::CliError;
use tempfile::TempDir;

/// Test 1.7.1: Parse valid in_memory garrison configuration
#[tokio::test]
async fn test_in_memory_garrison_config() {
    // Arrange: Create a valid in_memory garrison config
    let config = GarrisonConfig {
        garrison_type: "in_memory".to_string(),
        config: Some(GarrisonTypeConfig {
            max_entries: Some(100),
            path: None,
        }),
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should succeed and return Some garrison
    assert!(result.is_ok(), "Failed to instantiate in_memory garrison");
    let garrison = result.unwrap();
    assert!(
        garrison.is_some(),
        "Expected Some garrison, got None for in_memory type"
    );
}

/// Test 1.7.2: Parse valid sqlite garrison configuration
#[tokio::test]
async fn test_sqlite_garrison_config() {
    // Arrange: Create temp directory and sqlite config
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db_path = temp_dir.path().join("test_garrison.db");

    let config = GarrisonConfig {
        garrison_type: "sqlite".to_string(),
        config: Some(GarrisonTypeConfig {
            max_entries: Some(500),
            path: Some(db_path.to_string_lossy().to_string()),
        }),
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should succeed and return Some garrison
    assert!(result.is_ok(), "Failed to instantiate sqlite garrison");
    let garrison = result.unwrap();
    assert!(
        garrison.is_some(),
        "Expected Some garrison, got None for sqlite type"
    );

    // Verify database file was created
    assert!(
        db_path.exists(),
        "Expected database file to be created at {:?}",
        db_path
    );
}

/// Test 1.7.3: Validate error for invalid garrison type
#[tokio::test]
async fn test_invalid_garrison_type() {
    // Arrange: Create config with invalid type
    let config = GarrisonConfig {
        garrison_type: "invalid_type".to_string(),
        config: Some(GarrisonTypeConfig {
            max_entries: Some(100),
            path: None,
        }),
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should fail with GarrisonConfigError
    assert!(result.is_err(), "Expected error for invalid garrison type");
    if let Err(CliError::GarrisonConfigError { message }) = result {
        assert!(
            message.contains("must be 'in_memory' or 'sqlite'"),
            "Expected type validation error, got: {}",
            message
        );
    } else {
        panic!("Expected GarrisonConfigError");
    }
}

/// Test 1.7.4: Validate error for sqlite without path
#[tokio::test]
async fn test_sqlite_missing_path() {
    // Arrange: Create sqlite config without path
    let config = GarrisonConfig {
        garrison_type: "sqlite".to_string(),
        config: Some(GarrisonTypeConfig {
            max_entries: Some(100),
            path: None, // Missing path for sqlite
        }),
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should fail with GarrisonConfigError
    assert!(result.is_err(), "Expected error for sqlite without path");
    if let Err(CliError::GarrisonConfigError { message }) = result {
        assert!(
            message.contains("path is required"),
            "Expected 'path is required' error, got: {}",
            message
        );
    } else {
        panic!("Expected GarrisonConfigError");
    }
}

/// Test 1.7.5: Validate None config returns None garrison
#[tokio::test]
async fn test_none_garrison_config() {
    // Arrange: No garrison config provided
    let config: Option<GarrisonConfig> = None;

    // Act: Instantiate garrison
    let result = instantiate_garrison(&config, "test_paladin").await;

    // Assert: Should succeed and return None
    assert!(result.is_ok(), "Failed with None config");
    let garrison = result.unwrap();
    assert!(garrison.is_none(), "Expected None garrison for None config");
}

/// Test 1.7.6: Validate default values are applied
#[tokio::test]
async fn test_garrison_default_values() {
    // Arrange: Create config with minimal fields
    let config = GarrisonConfig {
        garrison_type: "in_memory".to_string(),
        config: None, // No detailed config, should use defaults
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should succeed with defaults applied
    assert!(result.is_ok(), "Failed to instantiate with default values");
    let garrison = result.unwrap();
    assert!(
        garrison.is_some(),
        "Expected Some garrison with default values"
    );
}

/// Test 1.7.7: Validate directory creation for sqlite path
#[tokio::test]
async fn test_sqlite_directory_creation() {
    // Arrange: Create temp directory and nested path
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let nested_path = temp_dir.path().join("nested").join("dir").join("test.db");

    let config = GarrisonConfig {
        garrison_type: "sqlite".to_string(),
        config: Some(GarrisonTypeConfig {
            max_entries: Some(100),
            path: Some(nested_path.to_string_lossy().to_string()),
        }),
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should succeed and create parent directories
    assert!(result.is_ok(), "Failed to instantiate with nested path");
    assert!(
        nested_path.parent().unwrap().exists(),
        "Expected parent directories to be created"
    );
}

/// Test 1.7.8: Validate eviction strategy configuration (using defaults)
#[tokio::test]
async fn test_eviction_strategy_config() {
    // Arrange: Create config (eviction strategy is set internally)
    let config = GarrisonConfig {
        garrison_type: "in_memory".to_string(),
        config: Some(GarrisonTypeConfig {
            max_entries: Some(100),
            path: None,
        }),
    };

    // Act: Instantiate garrison
    let result = instantiate_garrison(&Some(config), "test_paladin").await;

    // Assert: Should succeed (eviction strategy is handled internally)
    assert!(
        result.is_ok(),
        "Failed to instantiate with default eviction strategy"
    );
    let garrison = result.unwrap();
    assert!(garrison.is_some(), "Expected Some garrison");
}
