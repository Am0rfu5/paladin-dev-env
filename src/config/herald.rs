//! Configuration for the Herald output formatting system.

use crate::config::env_utils::EnvOverridable;
use serde::{Deserialize, Serialize};

/// Configuration for JSON Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonHeraldConfig {
    /// Enable pretty-printing (formatted with indentation)
    pub pretty: bool,
    /// Include metadata fields in output
    pub include_metadata: bool,
}

impl Default for JsonHeraldConfig {
    fn default() -> Self {
        Self {
            pretty: true,
            include_metadata: true,
        }
    }
}

/// Configuration for Markdown Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownHeraldConfig {
    /// Enable ANSI color codes for terminal output
    pub include_colors: bool,
    /// Heading level for main sections (1-6)
    pub heading_level: u8,
}

impl Default for MarkdownHeraldConfig {
    fn default() -> Self {
        Self {
            include_colors: true,
            heading_level: 2,
        }
    }
}

/// Configuration for Table Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableHeraldConfig {
    /// Maximum width for table columns (characters)
    pub max_column_width: usize,
    /// Border style preset: "ascii", "rounded", "modern", "sharp", "none"
    pub border_style: String,
}

impl Default for TableHeraldConfig {
    fn default() -> Self {
        Self {
            max_column_width: 60,
            border_style: "rounded".to_string(),
        }
    }
}

/// Configuration for Herald output formatting system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeraldConfig {
    /// Default formatter to use: "json", "markdown", "table"
    pub default_formatter: String,
    /// JSON formatter configuration
    pub json: JsonHeraldConfig,
    /// Markdown formatter configuration
    pub markdown: MarkdownHeraldConfig,
    /// Table formatter configuration
    pub table: TableHeraldConfig,
}

impl Default for HeraldConfig {
    fn default() -> Self {
        Self {
            default_formatter: "json".to_string(),
            json: JsonHeraldConfig::default(),
            markdown: MarkdownHeraldConfig::default(),
            table: TableHeraldConfig::default(),
        }
    }
}

impl HeraldConfig {
    /// Validates herald configuration
    pub fn validate(&self) -> Result<(), String> {
        let valid_formatters = ["json", "markdown", "table"];
        if !valid_formatters.contains(&self.default_formatter.as_str()) {
            return Err(format!(
                "Invalid default_formatter '{}'. Must be one of: {}",
                self.default_formatter,
                valid_formatters.join(", ")
            ));
        }

        if !(1..=6).contains(&self.markdown.heading_level) {
            return Err(format!(
                "Invalid markdown heading_level {}. Must be between 1 and 6",
                self.markdown.heading_level
            ));
        }

        if self.table.max_column_width == 0 {
            return Err("table max_column_width must be greater than 0".to_string());
        }

        Ok(())
    }
}

impl EnvOverridable for HeraldConfig {
    fn apply_env_overrides(&mut self) {
        if let Ok(v) = std::env::var("HERALD_DEFAULT_FORMATTER") {
            self.default_formatter = v;
        }
        if let Ok(v) = std::env::var("APP_HERALD_DEFAULT_FORMATTER") {
            self.default_formatter = v;
        }
        if let Ok(v) = std::env::var("APP_HERALD_JSON_PRETTY")
            && let Ok(b) = v.parse::<bool>()
        {
            self.json.pretty = b;
        }
        if let Ok(v) = std::env::var("APP_HERALD_JSON_INCLUDE_METADATA")
            && let Ok(b) = v.parse::<bool>()
        {
            self.json.include_metadata = b;
        }
        if let Ok(v) = std::env::var("APP_HERALD_MARKDOWN_INCLUDE_COLORS")
            && let Ok(b) = v.parse::<bool>()
        {
            self.markdown.include_colors = b;
        }
        if let Ok(v) = std::env::var("APP_HERALD_MARKDOWN_HEADING_LEVEL")
            && let Ok(h) = v.parse::<u8>()
            && (1..=6).contains(&h)
        {
            self.markdown.heading_level = h;
        }
        if let Ok(v) = std::env::var("APP_HERALD_TABLE_MAX_COLUMN_WIDTH")
            && let Ok(w) = v.parse::<usize>()
            && w > 0
        {
            self.table.max_column_width = w;
        }
        if let Ok(v) = std::env::var("APP_HERALD_TABLE_BORDER_STYLE") {
            self.table.border_style = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    fn test_herald_config_default() {
        let config = HeraldConfig::default();
        assert_eq!(config.default_formatter, "json");
        assert!(config.json.pretty);
        assert!(config.json.include_metadata);
        assert!(config.markdown.include_colors);
        assert_eq!(config.markdown.heading_level, 2);
        assert_eq!(config.table.max_column_width, 60);
        assert_eq!(config.table.border_style, "rounded");
    }

    #[test]
    fn test_herald_config_validate_valid() {
        let config = HeraldConfig {
            default_formatter: "markdown".to_string(),
            json: JsonHeraldConfig {
                pretty: true,
                include_metadata: true,
            },
            markdown: MarkdownHeraldConfig {
                include_colors: true,
                heading_level: 3,
            },
            table: TableHeraldConfig {
                max_column_width: 80,
                border_style: "ascii".to_string(),
            },
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_herald_config_validate_invalid_formatter() {
        let config = HeraldConfig {
            default_formatter: "invalid".to_string(),
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid default_formatter"));
    }

    #[test]
    fn test_herald_config_validate_invalid_heading_level_low() {
        let config = HeraldConfig {
            default_formatter: "markdown".to_string(),
            markdown: MarkdownHeraldConfig {
                include_colors: true,
                heading_level: 0,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid markdown heading_level")
        );
    }

    #[test]
    fn test_herald_config_validate_invalid_heading_level_high() {
        let config = HeraldConfig {
            default_formatter: "markdown".to_string(),
            markdown: MarkdownHeraldConfig {
                include_colors: true,
                heading_level: 7,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid markdown heading_level")
        );
    }

    #[test]
    fn test_herald_config_validate_zero_column_width() {
        let config = HeraldConfig {
            default_formatter: "table".to_string(),
            table: TableHeraldConfig {
                max_column_width: 0,
                border_style: "rounded".to_string(),
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("table max_column_width must be greater than 0")
        );
    }

    #[test]
    #[serial]
    fn test_herald_config_env_override() {
        unsafe {
            env::set_var("HERALD_DEFAULT_FORMATTER", "markdown");
            env::set_var("APP_HERALD_JSON_PRETTY", "false");
            env::set_var("APP_HERALD_JSON_INCLUDE_METADATA", "false");
            env::set_var("APP_HERALD_MARKDOWN_INCLUDE_COLORS", "false");
            env::set_var("APP_HERALD_MARKDOWN_HEADING_LEVEL", "4");
            env::set_var("APP_HERALD_TABLE_MAX_COLUMN_WIDTH", "100");
            env::set_var("APP_HERALD_TABLE_BORDER_STYLE", "modern");
        }

        let mut config = HeraldConfig::default();
        config.apply_env_overrides();

        assert_eq!(config.default_formatter, "markdown");
        assert!(!config.json.pretty);
        assert!(!config.json.include_metadata);
        assert!(!config.markdown.include_colors);
        assert_eq!(config.markdown.heading_level, 4);
        assert_eq!(config.table.max_column_width, 100);
        assert_eq!(config.table.border_style, "modern");

        unsafe {
            env::remove_var("HERALD_DEFAULT_FORMATTER");
            env::remove_var("APP_HERALD_JSON_PRETTY");
            env::remove_var("APP_HERALD_JSON_INCLUDE_METADATA");
            env::remove_var("APP_HERALD_MARKDOWN_INCLUDE_COLORS");
            env::remove_var("APP_HERALD_MARKDOWN_HEADING_LEVEL");
            env::remove_var("APP_HERALD_TABLE_MAX_COLUMN_WIDTH");
            env::remove_var("APP_HERALD_TABLE_BORDER_STYLE");
        }
    }

    #[test]
    #[serial]
    fn test_herald_config_app_prefix_override() {
        unsafe {
            env::set_var("APP_HERALD_DEFAULT_FORMATTER", "table");
        }

        let mut config = HeraldConfig::default();
        config.apply_env_overrides();

        assert_eq!(config.default_formatter, "table");

        unsafe {
            env::remove_var("APP_HERALD_DEFAULT_FORMATTER");
        }
    }

    #[test]
    fn test_herald_config_deserialization() {
        let config = HeraldConfig {
            default_formatter: "json".to_string(),
            json: JsonHeraldConfig {
                pretty: false,
                include_metadata: false,
            },
            markdown: MarkdownHeraldConfig {
                include_colors: false,
                heading_level: 1,
            },
            table: TableHeraldConfig {
                max_column_width: 120,
                border_style: "sharp".to_string(),
            },
        };

        assert_eq!(config.default_formatter, "json");
        assert!(!config.json.pretty);
        assert!(!config.json.include_metadata);
        assert!(!config.markdown.include_colors);
        assert_eq!(config.markdown.heading_level, 1);
        assert_eq!(config.table.max_column_width, 120);
        assert_eq!(config.table.border_style, "sharp");
    }
}
