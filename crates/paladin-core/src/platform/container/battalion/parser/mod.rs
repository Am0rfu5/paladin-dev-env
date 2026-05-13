//! Flow DSL Parser
//!
//! This module provides parsing capabilities for the Flow DSL (Domain-Specific Language)
//! used to define agent workflows in the Maneuver pattern.
//!
//! # Syntax
//!
//! The Flow DSL supports the following operators:
//! - `->` : Sequential execution (e.g., `"a -> b"`)
//! - `,`  : Parallel execution (e.g., `"a, b"`)
//! - `()` : Grouping (e.g., `"a -> (b -> c), d"`)
//!
//! # Examples
//!
//! ```
//! use paladin::core::platform::container::battalion::parser::FlowParser;
//!
//! // Simple sequential
//! let expr = FlowParser::parse("agent1 -> agent2").unwrap();
//!
//! // Parallel execution
//! let expr = FlowParser::parse("agent1, agent2").unwrap();
//!
//! // Fan-out pattern
//! let expr = FlowParser::parse("researcher -> writer, editor").unwrap();
//! ```

pub mod ast;
pub mod error;
pub mod lexer;

pub use ast::FlowExpression;
pub use error::FlowParseError;
pub use lexer::{Lexer, Token};

/// Parser for Flow DSL expressions
///
/// The `FlowParser` converts string-based flow expressions into an Abstract Syntax Tree (AST)
/// represented by `FlowExpression`.
///
/// # Example
///
/// ```
/// use paladin::core::platform::container::battalion::parser::FlowParser;
///
/// let result = FlowParser::parse("planner -> (coder -> tester), docs");
/// assert!(result.is_ok());
/// ```
pub struct FlowParser;

impl FlowParser {
    /// Parse a flow expression string into a FlowExpression AST
    ///
    /// # Arguments
    ///
    /// * `expression` - The flow expression string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(FlowExpression)` - Successfully parsed expression
    /// * `Err(FlowParseError)` - Parse error with details
    ///
    /// # Errors
    ///
    /// Returns `FlowParseError` if:
    /// - The expression has unbalanced parentheses
    /// - Invalid agent identifiers are used
    /// - Consecutive operators appear
    /// - Empty expressions or groups are found
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowParser;
    ///
    /// let expr = FlowParser::parse("agent1 -> agent2").unwrap();
    /// ```
    pub fn parse(expression: &str) -> Result<FlowExpression, FlowParseError> {
        if expression.trim().is_empty() {
            return Err(FlowParseError::EmptyExpression);
        }

        let mut lexer = Lexer::new(expression);
        let mut parser = Parser::new(&mut lexer)?;
        let expr = parser.parse_expression()?;

        // Ensure we've consumed all tokens - any remaining tokens are unexpected
        if let Some(token) = parser.current_token {
            return Err(FlowParseError::UnexpectedToken {
                position: parser.lexer.position(),
                found: format!("{:?}", token),
                expected: "end of input".to_string(),
            });
        }

        Ok(expr)
    }
}

/// Internal parser state
struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Option<Token>,
    paren_depth: usize,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Result<Self, FlowParseError> {
        let current_token = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current_token,
            paren_depth: 0,
        })
    }

    fn advance(&mut self) -> Result<(), FlowParseError> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    /// Parse the top-level expression
    /// Grammar: expression := sequential
    fn parse_expression(&mut self) -> Result<FlowExpression, FlowParseError> {
        self.parse_sequential()
    }

    /// Parse sequential expressions (lowest precedence)
    /// Grammar: sequential := parallel ( '->' parallel )*
    fn parse_sequential(&mut self) -> Result<FlowExpression, FlowParseError> {
        let mut expressions = vec![self.parse_parallel()?];

        while matches!(self.current_token, Some(Token::Arrow)) {
            self.advance()?;
            expressions.push(self.parse_parallel()?);
        }

        if expressions.len() == 1 {
            Ok(expressions.into_iter().next().unwrap())
        } else {
            Ok(FlowExpression::Sequential(expressions))
        }
    }

    /// Parse parallel expressions (higher precedence than sequential)
    /// Grammar: parallel := primary ( ',' primary )*
    fn parse_parallel(&mut self) -> Result<FlowExpression, FlowParseError> {
        let mut expressions = vec![self.parse_primary()?];

        while matches!(self.current_token, Some(Token::Comma)) {
            self.advance()?;
            expressions.push(self.parse_primary()?);
        }

        if expressions.len() == 1 {
            Ok(expressions.into_iter().next().unwrap())
        } else {
            Ok(FlowExpression::Parallel(expressions))
        }
    }

    /// Parse primary expressions (highest precedence)
    /// Grammar: primary := Agent | '(' expression ')'
    fn parse_primary(&mut self) -> Result<FlowExpression, FlowParseError> {
        match &self.current_token {
            Some(Token::Agent(name)) => {
                let agent_name = name.clone();
                self.advance()?;
                Ok(FlowExpression::Agent(agent_name))
            }
            Some(Token::LParen) => {
                self.paren_depth += 1;
                self.advance()?;

                let expr = self.parse_expression()?;

                match &self.current_token {
                    Some(Token::RParen) => {
                        self.paren_depth -= 1;
                        self.advance()?;
                        Ok(expr)
                    }
                    _ => Err(FlowParseError::UnbalancedParentheses {
                        position: self.lexer.position(),
                        message: "Expected closing parenthesis".to_string(),
                    }),
                }
            }
            Some(Token::RParen) => Err(FlowParseError::UnexpectedToken {
                position: self.lexer.position(),
                found: "closing parenthesis".to_string(),
                expected: "agent name or opening parenthesis".to_string(),
            }),
            Some(Token::Arrow) | Some(Token::Comma) => Err(FlowParseError::ConsecutiveOperators {
                position: self.lexer.position(),
                operator: if matches!(self.current_token, Some(Token::Arrow)) {
                    "->".to_string()
                } else {
                    ",".to_string()
                },
            }),
            None => Err(FlowParseError::UnexpectedEndOfInput {
                position: self.lexer.position(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_sequential() {
        let expr = FlowParser::parse("a -> b").unwrap();
        match expr {
            FlowExpression::Sequential(exprs) => {
                assert_eq!(exprs.len(), 2);
            }
            _ => panic!("Expected Sequential"),
        }
    }

    #[test]
    fn test_parse_simple_parallel() {
        let expr = FlowParser::parse("a, b").unwrap();
        match expr {
            FlowExpression::Parallel(exprs) => {
                assert_eq!(exprs.len(), 2);
            }
            _ => panic!("Expected Parallel"),
        }
    }

    #[test]
    fn test_parse_empty_expression() {
        let result = FlowParser::parse("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FlowParseError::EmptyExpression
        ));
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = FlowParser::parse("   ");
        assert!(result.is_err());
    }
}
