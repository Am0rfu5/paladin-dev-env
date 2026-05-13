//! Error types for Flow DSL parsing

use thiserror::Error;

/// Errors that can occur during flow expression parsing
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FlowParseError {
    /// Unexpected token encountered
    #[error("Unexpected token at position {position}: found '{found}', expected {expected}")]
    UnexpectedToken {
        position: usize,
        found: String,
        expected: String,
    },

    /// Unbalanced parentheses
    #[error("Unbalanced parentheses at position {position}: {message}")]
    UnbalancedParentheses { position: usize, message: String },

    /// Empty expression
    #[error("Empty expression")]
    EmptyExpression,

    /// Invalid agent identifier
    #[error("Invalid identifier at position {position}: '{identifier}' - {message}")]
    InvalidIdentifier {
        position: usize,
        identifier: String,
        message: String,
    },

    /// Consecutive operators without operand
    #[error("Consecutive operators at position {position}: operator '{operator}' without operand")]
    ConsecutiveOperators { position: usize, operator: String },

    /// Invalid character in input
    #[error("Invalid character '{character}' at position {position}")]
    InvalidCharacter { position: usize, character: char },

    /// Unexpected end of input
    #[error("Unexpected end of input at position {position}")]
    UnexpectedEndOfInput { position: usize },
}

impl FlowParseError {
    /// Generate a helpful suggestion for fixing the error
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowParseError;
    ///
    /// let error = FlowParseError::EmptyExpression;
    /// let suggestion = error.suggestion();
    /// assert!(suggestion.is_some());
    /// ```
    pub fn suggestion(&self) -> Option<String> {
        match self {
            FlowParseError::UnexpectedToken {
                found, expected, ..
            } => Some(format!("Try replacing '{}' with {}", found, expected)),
            FlowParseError::UnbalancedParentheses { .. } => {
                Some("Check that every '(' has a matching ')'".to_string())
            }
            FlowParseError::EmptyExpression => {
                Some("Provide at least one agent name (e.g., 'agent1 -> agent2')".to_string())
            }
            FlowParseError::InvalidIdentifier { identifier, .. } => {
                if identifier.is_empty() {
                    Some("Agent names must be alphanumeric (may include _ or -)".to_string())
                } else {
                    Some(format!(
                        "Check the identifier '{}'. It should start with a letter or number.",
                        identifier
                    ))
                }
            }
            FlowParseError::ConsecutiveOperators { operator, .. } => Some(format!(
                "Add an agent name between operators (e.g., 'a {} b')",
                operator
            )),
            FlowParseError::InvalidCharacter { character, .. } => Some(format!(
                "Remove the invalid character '{}'. Only alphanumeric, '->', ',', '(', ')' are allowed.",
                character
            )),
            FlowParseError::UnexpectedEndOfInput { .. } => {
                Some("The expression appears incomplete. Add the missing parts.".to_string())
            }
        }
    }

    /// Get the position of the error in the input
    pub fn position(&self) -> Option<usize> {
        match self {
            FlowParseError::UnexpectedToken { position, .. }
            | FlowParseError::UnbalancedParentheses { position, .. }
            | FlowParseError::InvalidIdentifier { position, .. }
            | FlowParseError::ConsecutiveOperators { position, .. }
            | FlowParseError::InvalidCharacter { position, .. }
            | FlowParseError::UnexpectedEndOfInput { position } => Some(*position),
            FlowParseError::EmptyExpression => None,
        }
    }

    /// Format the error with context from the original input
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowParseError;
    ///
    /// let error = FlowParseError::InvalidCharacter {
    ///     position: 5,
    ///     character: '@',
    /// };
    /// let formatted = error.format_with_context("agent@test");
    /// assert!(formatted.contains("agent@test"));
    /// assert!(formatted.contains("^"));
    /// ```
    pub fn format_with_context(&self, input: &str) -> String {
        let mut output = format!("Error: {}\n", self);

        if let Some(pos) = self.position() {
            output.push_str(&format!("  --> {}\n", input));
            output.push_str("      ");
            output.push_str(&" ".repeat(pos));
            output.push_str("^\n");
        }

        if let Some(suggestion) = self.suggestion() {
            output.push_str(&format!("\nSuggestion: {}\n", suggestion));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_suggestion() {
        let error = FlowParseError::EmptyExpression;
        assert!(error.suggestion().is_some());
    }

    #[test]
    fn test_error_position() {
        let error = FlowParseError::InvalidCharacter {
            position: 10,
            character: '@',
        };
        assert_eq!(error.position(), Some(10));
    }

    #[test]
    fn test_format_with_context() {
        let error = FlowParseError::InvalidCharacter {
            position: 5,
            character: '@',
        };
        let formatted = error.format_with_context("agent@test");
        assert!(formatted.contains("agent@test"));
        assert!(formatted.contains("^"));
    }

    #[test]
    fn test_unbalanced_parens_suggestion() {
        let error = FlowParseError::UnbalancedParentheses {
            position: 10,
            message: "Missing closing paren".to_string(),
        };
        let suggestion = error.suggestion().unwrap();
        assert!(suggestion.contains("("));
        assert!(suggestion.contains(")"));
    }

    #[test]
    fn test_consecutive_operators_suggestion() {
        let error = FlowParseError::ConsecutiveOperators {
            position: 5,
            operator: "->".to_string(),
        };
        let suggestion = error.suggestion().unwrap();
        assert!(suggestion.contains("agent name"));
    }
}
