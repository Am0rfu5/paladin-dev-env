//! Test utilities for CLI module
//!
//! This module provides common test utilities, fixtures, and helpers
//! for testing CLI functionality.

#[cfg(test)]
pub mod formatter_tests;

#[cfg(test)]
pub mod command_tests;

#[cfg(test)]
pub(crate) mod test_utils {
    /// Create a temporary test directory
    #[allow(dead_code)]
    pub fn create_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("Failed to create temp dir")
    }

    /// Create a mock config file
    #[allow(dead_code)]
    pub fn create_mock_config(dir: &std::path::Path, content: &str) -> std::path::PathBuf {
        let config_path = dir.join("config.yml");
        std::fs::write(&config_path, content).expect("Failed to write mock config");
        config_path
    }

    /// Mock API key validation responses
    #[allow(dead_code)]
    pub fn mock_valid_openai_key() -> &'static str {
        "sk-valid-openai-key-for-testing-12345678901234567890"
    }

    #[allow(dead_code)]
    pub fn mock_valid_anthropic_key() -> &'static str {
        "sk-ant-valid-anthropic-key-for-testing-12345678"
    }

    #[allow(dead_code)]
    pub fn mock_valid_deepseek_key() -> &'static str {
        "sk-valid-deepseek-key-for-testing-1234567890"
    }

    /// Strip ANSI color codes from strings for testing
    #[allow(dead_code)]
    pub fn strip_ansi(s: &str) -> String {
        let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        re.replace_all(s, "").to_string()
    }
}
