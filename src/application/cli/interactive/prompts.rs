//! Reusable prompt components using dialoguer

use crate::application::cli::error::{CliError, CliResult};
use dialoguer::{Confirm, Input, MultiSelect, Password, Select};

/// Prompt builder for creating interactive prompts
///
/// # Examples
///
/// Construction only — the returned prompt types read from the terminal
/// when `.prompt()` is called, so that call is intentionally not made here
/// (this doctest must run offline with no TTY attached, T-16-26).
///
/// ```
/// use paladin::application::cli::interactive::prompts::PromptBuilder;
///
/// let _input_prompt = PromptBuilder::input("Enter your name");
/// let _confirm_prompt = PromptBuilder::confirm("Continue?");
/// ```
pub struct PromptBuilder;

impl PromptBuilder {
    /// Create a text input prompt
    pub fn input(prompt: &str) -> InputPrompt {
        InputPrompt {
            prompt: prompt.to_string(),
            default: None,
            allow_empty: false,
        }
    }

    /// Create a password input prompt
    pub fn password(prompt: &str) -> PasswordPrompt {
        PasswordPrompt {
            prompt: prompt.to_string(),
            confirmation: None,
        }
    }

    /// Create a confirmation prompt (yes/no)
    pub fn confirm(prompt: &str) -> ConfirmPrompt {
        ConfirmPrompt {
            prompt: prompt.to_string(),
            default: None,
        }
    }

    /// Create a selection prompt
    pub fn select<T>(prompt: &str, items: Vec<T>) -> SelectPrompt<T>
    where
        T: ToString + Clone,
    {
        SelectPrompt {
            prompt: prompt.to_string(),
            items,
            default: None,
        }
    }

    /// Create a multi-selection prompt
    pub fn multi_select<T>(prompt: &str, items: Vec<T>) -> MultiSelectPrompt<T>
    where
        T: ToString + Clone,
    {
        MultiSelectPrompt {
            prompt: prompt.to_string(),
            items,
            defaults: vec![],
        }
    }
}

/// Text input prompt
pub struct InputPrompt {
    prompt: String,
    default: Option<String>,
    allow_empty: bool,
}

impl InputPrompt {
    /// Set a default value
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Allow empty input
    pub fn allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = allow;
        self
    }

    /// Show the prompt and get user input
    pub fn prompt(self) -> CliResult<String> {
        let mut input = Input::<String>::new().with_prompt(&self.prompt);

        if let Some(default) = self.default {
            input = input.default(default);
        }

        if self.allow_empty {
            input = input.allow_empty(true);
        }

        input
            .interact_text()
            .map_err(|e| CliError::user_input(e.to_string()))
    }
}

/// Password input prompt
pub struct PasswordPrompt {
    prompt: String,
    confirmation: Option<String>,
}

impl PasswordPrompt {
    /// Require password confirmation
    pub fn with_confirmation(mut self, prompt: impl Into<String>) -> Self {
        self.confirmation = Some(prompt.into());
        self
    }

    /// Show the prompt and get user input
    pub fn prompt(self) -> CliResult<String> {
        let mut password = Password::new().with_prompt(&self.prompt);

        if let Some(confirmation) = self.confirmation {
            password = password.with_confirmation(&confirmation, "Passwords do not match");
        }

        password
            .interact()
            .map_err(|e| CliError::user_input(e.to_string()))
    }
}

/// Confirmation prompt
pub struct ConfirmPrompt {
    prompt: String,
    default: Option<bool>,
}

impl ConfirmPrompt {
    /// Set a default value
    pub fn with_default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    /// Show the prompt and get user confirmation
    pub fn prompt(self) -> CliResult<bool> {
        let mut confirm = Confirm::new().with_prompt(&self.prompt);

        if let Some(default) = self.default {
            confirm = confirm.default(default);
        }

        confirm
            .interact()
            .map_err(|e| CliError::user_input(e.to_string()))
    }
}

/// Selection prompt
pub struct SelectPrompt<T>
where
    T: ToString + Clone,
{
    prompt: String,
    items: Vec<T>,
    default: Option<usize>,
}

impl<T> SelectPrompt<T>
where
    T: ToString + Clone,
{
    /// Set a default selection by index
    pub fn with_default(mut self, index: usize) -> Self {
        self.default = Some(index);
        self
    }

    /// Show the prompt and get user selection
    pub fn prompt(self) -> CliResult<T> {
        let mut select = Select::new().with_prompt(&self.prompt);

        for item in &self.items {
            select = select.item(item.to_string());
        }

        if let Some(default) = self.default {
            select = select.default(default);
        }

        let index = select
            .interact()
            .map_err(|e| CliError::user_input(e.to_string()))?;

        Ok(self.items[index].clone())
    }

    /// Get the selected index instead of the value
    pub fn prompt_index(self) -> CliResult<usize> {
        let mut select = Select::new().with_prompt(&self.prompt);

        for item in &self.items {
            select = select.item(item.to_string());
        }

        if let Some(default) = self.default {
            select = select.default(default);
        }

        select
            .interact()
            .map_err(|e| CliError::user_input(e.to_string()))
    }
}

/// Multi-selection prompt
pub struct MultiSelectPrompt<T>
where
    T: ToString + Clone,
{
    prompt: String,
    items: Vec<T>,
    defaults: Vec<bool>,
}

impl<T> MultiSelectPrompt<T>
where
    T: ToString + Clone,
{
    /// Set default selections
    pub fn with_defaults(mut self, defaults: Vec<bool>) -> Self {
        self.defaults = defaults;
        self
    }

    /// Show the prompt and get user selections
    pub fn prompt(self) -> CliResult<Vec<T>> {
        let mut multi_select = MultiSelect::new().with_prompt(&self.prompt);

        for item in &self.items {
            multi_select = multi_select.item(item.to_string());
        }

        if !self.defaults.is_empty() {
            multi_select = multi_select.defaults(&self.defaults);
        }

        let indices = multi_select
            .interact()
            .map_err(|e| CliError::user_input(e.to_string()))?;

        Ok(indices.into_iter().map(|i| self.items[i].clone()).collect())
    }
}

/// Prompt trait for common prompt operations
pub trait Prompt {
    type Output;

    /// Show the prompt and get user input
    fn prompt(self) -> CliResult<Self::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder_creation() {
        // Just verify builders can be created
        let _input = PromptBuilder::input("Enter name");
        let _password = PromptBuilder::password("Enter password");
        let _confirm = PromptBuilder::confirm("Continue?");
        let _select = PromptBuilder::select("Choose one", vec!["A", "B", "C"]);
        let _multi = PromptBuilder::multi_select("Choose many", vec!["X", "Y", "Z"]);
    }

    #[test]
    fn test_input_with_default() {
        let input = PromptBuilder::input("Enter name").with_default("John");
        assert!(input.default.is_some());
        assert_eq!(input.default.unwrap(), "John");
    }

    #[test]
    fn test_confirm_with_default() {
        let confirm = PromptBuilder::confirm("Continue?").with_default(true);
        assert_eq!(confirm.default, Some(true));
    }
}
