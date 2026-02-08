//! Interactive utility functions for CLI
//!
//! Standalone interactive utilities for user input, confirmation,
//! and validation that don't require the wizard framework.

use crate::application::cli::error::CliError;
use dialoguer::{Confirm, Input};
use std::io::{self, IsTerminal};

/// Check if running in a TTY environment
fn ensure_tty() -> Result<(), CliError> {
    if io::stdin().is_terminal() && io::stdout().is_terminal() {
        Ok(())
    } else {
        Err(CliError::validation(
            "Interactive prompts require a TTY environment (not supported in pipes or scripts)"
                .to_string(),
        ))
    }
}

/// Prompt the user for text input
///
/// Displays a text input prompt and returns the user's input.
/// Handles Ctrl+C gracefully by returning `CliError::Cancelled`.
///
/// # Arguments
/// * `prompt` - The prompt text to display
///
/// # Returns
/// * `Ok(String)` - User input
/// * `Err(CliError::Cancelled)` - User pressed Ctrl+C
/// * `Err(CliError::ValidationError)` - Not running in a TTY environment
///
/// # Example
/// ```no_run
/// use paladin::application::cli::interactive::prompt_for_input;
///
/// let name = prompt_for_input("Enter your name")?;
/// println!("Hello, {}!", name);
/// # Ok::<(), paladin::application::cli::error::CliError>(())
/// ```
pub fn prompt_for_input(prompt: &str) -> Result<String, CliError> {
    ensure_tty()?;

    Input::<String>::new()
        .with_prompt(prompt)
        .interact_text()
        .map_err(|e| match e {
            dialoguer::Error::IO(io_err) if io_err.kind() == io::ErrorKind::Interrupted => {
                CliError::Cancelled
            }
            _ => CliError::user_input(format!("Failed to read input: {}", e)),
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
/// use paladin::application::cli::interactive::confirm;
///
/// if confirm("Overwrite existing file?", false)? {
///     println!("Overwriting file...");
/// } else {
///     println!("Operation cancelled.");
/// }
/// # Ok::<(), paladin::application::cli::error::CliError>(())
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
            _ => CliError::user_input(format!("Failed to read confirmation: {}", e)),
        })
}

/// Prompt the user for input with validation
///
/// Displays the prompt and validates user input using the provided validator function.
/// If validation fails, displays the error message and re-prompts the user.
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
/// use paladin::application::cli::interactive::prompt_with_validation;
///
/// let port = prompt_with_validation("Enter port number", |input| {
///     input.parse::<u16>()
///         .map(|_| ())
///         .map_err(|_| "Port must be a valid number between 0 and 65535".to_string())
/// })?;
/// # Ok::<(), paladin::application::cli::error::CliError>(())
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
            _ => CliError::user_input(format!("Failed to read input: {}", e)),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_tty_detection() {
        // This test just ensures the function is callable
        // Actual TTY detection depends on runtime environment
        let _ = ensure_tty();
    }
}
