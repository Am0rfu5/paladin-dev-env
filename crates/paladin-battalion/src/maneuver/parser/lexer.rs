//! Lexer for Flow DSL
//!
//! Tokenizes flow expression strings into a sequence of tokens.

use super::error::FlowParseError;

/// Token types in the Flow DSL
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Sequential operator: `->`
    Arrow,
    /// Parallel operator: `,`
    Comma,
    /// Opening parenthesis: `(`
    LParen,
    /// Closing parenthesis: `)`
    RParen,
    /// Agent identifier
    Agent(String),
}

/// Lexer for tokenizing flow expressions
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    /// Create a new lexer for the given input string
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().copied();
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    /// Get the current position in the input
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advance to the next character
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        };
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Peek at the next character without advancing
    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    /// Read an agent identifier
    fn read_identifier(&mut self) -> Result<String, FlowParseError> {
        let start_pos = self.position;
        let mut identifier = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if identifier.is_empty() {
            return Err(FlowParseError::InvalidIdentifier {
                position: start_pos,
                identifier: String::new(),
                message: "Empty identifier".to_string(),
            });
        }

        // Validate identifier (must start with alphanumeric)
        if let Some(first_char) = identifier.chars().next()
            && !first_char.is_alphanumeric()
        {
            return Err(FlowParseError::InvalidIdentifier {
                position: start_pos,
                identifier: identifier.clone(),
                message: "Identifier must start with alphanumeric character".to_string(),
            });
        }

        Ok(identifier)
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Result<Option<Token>, FlowParseError> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(None),
            Some('(') => {
                self.advance();
                Ok(Some(Token::LParen))
            }
            Some(')') => {
                self.advance();
                Ok(Some(Token::RParen))
            }
            Some(',') => {
                self.advance();
                Ok(Some(Token::Comma))
            }
            Some('-') => {
                if self.peek() == Some('>') {
                    self.advance(); // skip '-'
                    self.advance(); // skip '>'
                    Ok(Some(Token::Arrow))
                } else {
                    // Could be start of identifier like "agent-1"
                    let identifier = self.read_identifier()?;
                    Ok(Some(Token::Agent(identifier)))
                }
            }
            Some(ch) if ch.is_alphanumeric() || ch == '_' => {
                let identifier = self.read_identifier()?;
                Ok(Some(Token::Agent(identifier)))
            }
            Some(ch) => Err(FlowParseError::InvalidCharacter {
                position: self.position,
                character: ch,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_sequential() {
        let mut lexer = Lexer::new("a -> b");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("a".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Arrow));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("b".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), None);
    }

    #[test]
    fn test_tokenize_simple_parallel() {
        let mut lexer = Lexer::new("a, b");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("a".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Comma));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("b".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), None);
    }

    #[test]
    fn test_tokenize_parentheses() {
        let mut lexer = Lexer::new("(a)");
        assert_eq!(lexer.next_token().unwrap(), Some(Token::LParen));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("a".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::RParen));
        assert_eq!(lexer.next_token().unwrap(), None);
    }

    #[test]
    fn test_tokenize_with_whitespace() {
        let mut lexer = Lexer::new("  a  ->  b  ");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("a".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Arrow));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("b".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), None);
    }

    #[test]
    fn test_tokenize_identifier_with_underscore() {
        let mut lexer = Lexer::new("agent_1");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("agent_1".to_string()))
        );
    }

    #[test]
    fn test_tokenize_identifier_with_hyphen() {
        let mut lexer = Lexer::new("agent-1");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("agent-1".to_string()))
        );
    }

    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::new("a @ b");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("a".to_string()))
        );
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("a -> (b -> c), d");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("a".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Arrow));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::LParen));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("b".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Arrow));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("c".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), Some(Token::RParen));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Comma));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Agent("d".to_string()))
        );
        assert_eq!(lexer.next_token().unwrap(), None);
    }
}
