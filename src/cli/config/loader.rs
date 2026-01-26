//! Configuration file loading utilities

use crate::cli::config::battalion_config::BattalionYamlConfig;
use crate::cli::config::paladin_config::{PaladinYamlConfig, Validate};
use crate::cli::output::errors::CliError;
use std::fs;
use std::path::Path;

/// Load Paladin configuration from YAML file
///
/// # Arguments
/// * `path` - Path to the YAML configuration file
///
/// # Returns
/// * `Ok(PaladinYamlConfig)` - Loaded and validated configuration
/// * `Err(CliError)` - File not found, invalid YAML, or validation error
pub fn load_paladin_config(path: &Path) -> Result<PaladinYamlConfig, CliError> {
    // Check if file exists
    if !path.exists() {
        return Err(CliError::ConfigFileNotFound {
            path: path.to_path_buf(),
        });
    }

    // Read file contents
    let contents = fs::read_to_string(path).map_err(|e| CliError::IoError {
        message: format!("Failed to read config file: {}", path.display()),
        source: e,
    })?;

    // Parse YAML
    let config: PaladinYamlConfig =
        serde_yaml::from_str(&contents).map_err(|e| CliError::InvalidYaml {
            path: path.to_path_buf(),
            source: e,
        })?;

    // Validate configuration
    config.validate()?;

    Ok(config)
}

/// Load Battalion configuration from YAML file
///
/// # Arguments
/// * `path` - Path to the YAML configuration file
///
/// # Returns
/// * `Ok(BattalionYamlConfig)` - Loaded and validated configuration
/// * `Err(CliError)` - File not found, invalid YAML, or validation error
pub fn load_battalion_config(path: &Path) -> Result<BattalionYamlConfig, CliError> {
    // Check if file exists
    if !path.exists() {
        return Err(CliError::ConfigFileNotFound {
            path: path.to_path_buf(),
        });
    }

    // Read file contents
    let contents = fs::read_to_string(path).map_err(|e| CliError::IoError {
        message: format!("Failed to read config file: {}", path.display()),
        source: e,
    })?;

    // Parse YAML
    let config: BattalionYamlConfig =
        serde_yaml::from_str(&contents).map_err(|e| CliError::InvalidYaml {
            path: path.to_path_buf(),
            source: e,
        })?;

    // Validate configuration
    config.validate()?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_paladin_config() {
        let yaml = r#"
name: test-paladin
system_prompt: "You are a helpful assistant"
model: gpt-4
temperature: 0.7
max_loops: 3
timeout_seconds: 300
stop_words: []
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let config = load_paladin_config(file.path()).unwrap();
        assert_eq!(config.name, "test-paladin");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.provider.provider_type, "openai");
    }

    #[test]
    fn test_load_paladin_config_file_not_found() {
        let result = load_paladin_config(Path::new("/nonexistent/file.yaml"));
        assert!(matches!(result, Err(CliError::ConfigFileNotFound { .. })));
    }

    #[test]
    fn test_load_paladin_config_invalid_yaml() {
        let yaml = r#"
name: test
invalid yaml syntax: [unclosed
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        assert!(matches!(result, Err(CliError::InvalidYaml { .. })));
    }

    #[test]
    fn test_load_paladin_config_missing_required_field() {
        let yaml = r#"
name: test
# Missing system_prompt
model: gpt-4
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        // Serde will fail on missing required field during deserialization
        assert!(matches!(result, Err(CliError::InvalidYaml { .. })));
    }

    #[test]
    fn test_load_paladin_config_validation_error() {
        // Test validation errors (empty fields that pass deserialization)
        let yaml = r#"
name: ""
system_prompt: "test"
model: gpt-4
provider:
  type: openai
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let result = load_paladin_config(file.path());
        assert!(matches!(result, Err(CliError::MissingRequiredField { .. })));
    }

    #[test]
    fn test_load_valid_formation_config() {
        let yaml = r#"
type: formation
name: test-formation
pass_output_to_next: true
paladins:
  - file: paladin1.yaml
  - file: paladin2.yaml
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
        file.flush().unwrap();

        let config = load_battalion_config(file.path()).unwrap();
        assert_eq!(config.battalion_type(), "formation");
    }
}
