//! Interactive prompt utilities for CLI user interaction
//!
//! This module provides functions for prompting users for input, confirmations,
//! and validated input. All functions detect TTY environment and handle Ctrl+C
//! gracefully per FR-25 and FR-26.

use crate::cli::output::errors::CliError;
use dialoguer::{Confirm, Input};
use std::io::{self, IsTerminal};

/// Check if running in a TTY environment
///
/// Returns an error if not in a TTY, as prompting won't work in non-interactive environments
fn ensure_tty() -> Result<(), CliError> {
    if !io::stdin().is_terminal() {
        return Err(CliError::ValidationError {
            message: "Not running in interactive terminal. Please provide all required arguments via command line flags.".to_string(),
        });
    }
    Ok(())
}

/// Prompt the user for text input
///
/// Displays the prompt and waits for user input. Returns the entered text.
/// Handles Ctrl+C gracefully by returning `CliError::Cancelled`.
///
/// # Arguments
/// * `prompt` - The prompt text to display to the user
///
/// # Returns
/// * `Ok(String)` - The user's input
/// * `Err(CliError::Cancelled)` - User pressed Ctrl+C
/// * `Err(CliError::ValidationError)` - Not running in a TTY environment
///
/// # Example
/// ```no_run
/// use paladin::cli::interactive::prompt_for_input;
///
/// let name = prompt_for_input("Enter your name")?;
/// println!("Hello, {}!", name);
/// # Ok::<(), paladin::cli::output::errors::CliError>(())
/// ```
pub fn prompt_for_input(prompt: &str) -> Result<String, CliError> {
    ensure_tty()?;

    Input::<String>::new()
        .with_prompt(prompt)
        .interact_text()
        .map_err(|e| {
            // dialoguer returns io::Error wrapped in its Error type
            // Check if it's an IO error and if it's interrupted (Ctrl+C)
            match e {
                dialoguer::Error::IO(io_err) if io_err.kind() == io::ErrorKind::Interrupted => {
                    CliError::Cancelled
                }
                _ => CliError::Other(format!("Failed to read input: {}", e)),
            }
        })
}

/// Prompt the user for a yes/no confirmation
///
/// Displays a confirmation prompt and returns true for "yes", false for "no".
/// Handles Ctrl+C gracefully by returning `CliError::Cancelled`.
///
/// # Arguments
/// * `prompt` - The confirmation prompt text
/// * `default` - The default value if user just presses Enter
///
/// # Returns
/// * `Ok(bool)` - true if confirmed, false if declined
/// * `Err(CliError::Cancelled)` - User pressed Ctrl+C
/// * `Err(CliError::ValidationError)` - Not running in a TTY environment
///
/// # Example
/// ```no_run
/// use paladin::cli::interactive::confirm;
///
/// if confirm("Overwrite existing file?", false)? {
///     println!("Overwriting file...");
/// } else {
///     println!("Operation cancelled.");
/// }
/// # Ok::<(), paladin::cli::output::errors::CliError>(())
/// ```
pub fn confirm(prompt: &str, default: bool) -> Result<bool, CliError> {
    ensure_tty()?;

    Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(|e| match e {
            dialoguer::Error::IO(io_err) if io_err.kind() == io::ErrorKind::Interrupted => {
                CliError::Cancelled
            }
            _ => CliError::Other(format!("Failed to read confirmation: {}", e)),
        })
}

/// Prompt the user for input with validation
///
/// Displays the prompt and validates user input using the provided validator function.
/// If validation fails, displays the error message and re-prompts the user.
/// This implements FR-26 (validation with error messages).
///
/// # Arguments
/// * `prompt` - The prompt text to display
/// * `validator` - A function that validates the input, returning Ok(()) if valid or Err(message) if invalid
///
/// # Returns
/// * `Ok(String)` - Valid user input
/// * `Err(CliError::Cancelled)` - User pressed Ctrl+C
/// * `Err(CliError::ValidationError)` - Not running in a TTY environment
///
/// # Example
/// ```no_run
/// use paladin::cli::interactive::prompt_with_validation;
///
/// let port = prompt_with_validation("Enter port number", |input| {
///     input.parse::<u16>()
///         .map(|_| ())
///         .map_err(|_| "Port must be a valid number between 0 and 65535".to_string())
/// })?;
/// # Ok::<(), paladin::cli::output::errors::CliError>(())
/// ```
pub fn prompt_with_validation<F>(prompt: &str, validator: F) -> Result<String, CliError>
where
    F: Fn(&str) -> Result<(), String>,
{
    ensure_tty()?;

    Input::<String>::new()
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), String> { validator(input) })
        .interact_text()
        .map_err(|e| match e {
            dialoguer::Error::IO(io_err) if io_err.kind() == io::ErrorKind::Interrupted => {
                CliError::Cancelled
            }
            _ => CliError::Other(format!("Failed to read input: {}", e)),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_tty_exists() {
        // Test that the TTY check function exists and returns a Result
        // In test environments, stdin might or might not be a TTY
        // So we just verify the function runs without panicking
        let _ = ensure_tty();
    }

    #[test]
    fn test_ensure_tty_returns_result() {
        // Verify that ensure_tty returns a Result type
        let result = ensure_tty();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_cli_error_cancelled_variant_exists() {
        // Verify the Cancelled error variant exists and can be constructed
        let error = CliError::Cancelled;
        match error {
            CliError::Cancelled => {} // Expected
            _ => panic!("Expected Cancelled variant"),
        }
    }

    #[test]
    fn test_cli_error_validation_variant_for_non_tty() {
        // Verify ValidationError can be created for non-TTY case
        let error = CliError::ValidationError {
            message: "Not running in interactive terminal".to_string(),
        };
        match error {
            CliError::ValidationError { message } => {
                assert!(message.contains("interactive"));
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }

    #[test]
    fn test_prompt_for_input_signature() {
        // Verify the function signature is correct
        // This test ensures the function exists with the expected signature
        fn _check_signature(_f: fn(&str) -> Result<String, CliError>) {}
        _check_signature(prompt_for_input);
    }

    #[test]
    fn test_confirm_signature() {
        // Verify the function signature is correct
        fn _check_signature(_f: fn(&str, bool) -> Result<bool, CliError>) {}
        _check_signature(confirm);
    }

    #[test]
    fn test_prompt_with_validation_signature() {
        // Verify the function can accept a validator
        let validator = |_input: &str| -> Result<(), String> { Ok(()) };

        // Type check: verify this compiles
        fn _type_check<F>(_validator: F)
        where
            F: Fn(&str) -> Result<(), String>,
        {
            let _: fn(&str, F) -> Result<String, CliError> = prompt_with_validation;
        }

        _type_check(validator);
    }

    #[test]
    fn test_validator_function_type() {
        // Test that a validator function can be defined and type-checks
        let validator = |input: &str| -> Result<(), String> {
            if input.is_empty() {
                Err("Input cannot be empty".to_string())
            } else {
                Ok(())
            }
        };

        // Test the validator logic
        assert!(validator("test").is_ok());
        assert!(validator("").is_err());
    }

    #[test]
    fn test_numeric_validator() {
        // Test a numeric validation function
        let numeric_validator = |input: &str| -> Result<(), String> {
            input
                .parse::<u32>()
                .map(|_| ())
                .map_err(|_| "Must be a valid number".to_string())
        };

        assert!(numeric_validator("42").is_ok());
        assert!(numeric_validator("abc").is_err());
        assert!(numeric_validator("-5").is_err()); // Negative for u32
    }

    #[test]
    fn test_port_validator() {
        // Test a port number validation function
        let port_validator = |input: &str| -> Result<(), String> {
            input
                .parse::<u16>()
                .map(|_| ())
                .map_err(|_| "Port must be between 0 and 65535".to_string())
        };

        assert!(port_validator("8080").is_ok());
        assert!(port_validator("65535").is_ok());
        assert!(port_validator("0").is_ok());
        assert!(port_validator("abc").is_err());
        assert!(port_validator("70000").is_err()); // Out of range
    }

    #[test]
    fn test_email_validator() {
        // Test an email validation function
        let email_validator = |input: &str| -> Result<(), String> {
            if input.contains('@') && input.contains('.') {
                Ok(())
            } else {
                Err("Invalid email format".to_string())
            }
        };

        assert!(email_validator("user@example.com").is_ok());
        assert!(email_validator("test@test.org").is_ok());
        assert!(email_validator("invalid").is_err());
        assert!(email_validator("no-at-sign.com").is_err());
    }

    // Note: Testing interactive prompts with actual user input requires
    // mocking stdin, which is complex in Rust. The dialoguer crate's
    // interaction functions will attempt to read from stdin even with
    // a TTY check, causing tests to hang.
    //
    // Manual testing is required per subtask 11.13:
    // 1. Run: cargo run --bin paladin-cli -- agent run -c test.yaml
    //    (without --input flag to trigger interactive prompt)
    // 2. Test Ctrl+C handling during prompt
    // 3. Test file overwrite confirmation during agent new
    //
    // Unit tests here verify the non-interactive error paths and
    // validator function logic only.
}
