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
    // Unit tests here verify the non-interactive error paths only.
}
