//! Wizard framework for multi-step interactive flows

use crate::application::cli::error::{CliError, CliResult};
use crate::application::cli::formatters::OutputFormatter;
use std::collections::HashMap;

/// A step in a wizard flow
pub trait WizardStep: Send + Sync {
    /// Get the step name/identifier
    fn name(&self) -> &str;

    /// Execute the step and return the result
    fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult>;

    /// Check if this step can be skipped based on context
    fn can_skip(&self, _context: &WizardContext) -> bool {
        false
    }

    /// Get a description of what this step does
    fn description(&self) -> Option<&str> {
        None
    }
}

/// Result of executing a wizard step
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepResult {
    /// Continue to the next step
    Continue,
    /// Skip to a specific step by name
    SkipTo(String),
    /// Go back to the previous step
    Back,
    /// Complete the wizard successfully
    Complete,
    /// Cancel the wizard
    Cancel,
}

/// Context shared between wizard steps
#[derive(Debug, Clone)]
pub struct WizardContext {
    /// Data storage for steps to share information
    data: HashMap<String, String>,
    /// Whether the wizard can be resumed
    resumable: bool,
    /// Completed steps for resume functionality
    completed_steps: Vec<String>,
}

impl WizardContext {
    /// Create a new wizard context
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            resumable: false,
            completed_steps: Vec::new(),
        }
    }

    /// Enable resume functionality
    pub fn enable_resume(&mut self) {
        self.resumable = true;
    }

    /// Check if resume is enabled
    pub fn is_resumable(&self) -> bool {
        self.resumable
    }

    /// Set a value in the context
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    /// Get a value from the context
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    /// Check if a key exists in the context
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Remove a value from the context
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    /// Mark a step as completed
    pub fn mark_completed(&mut self, step_name: impl Into<String>) {
        self.completed_steps.push(step_name.into());
    }

    /// Check if a step is completed
    pub fn is_completed(&self, step_name: &str) -> bool {
        self.completed_steps.contains(&step_name.to_string())
    }

    /// Get all completed steps
    pub fn completed_steps(&self) -> &[String] {
        &self.completed_steps
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.data.clear();
        self.completed_steps.clear();
    }
}

impl Default for WizardContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-step wizard for interactive flows
pub struct Wizard {
    steps: Vec<Box<dyn WizardStep>>,
    context: WizardContext,
    formatter: OutputFormatter,
}

impl Wizard {
    /// Create a new wizard
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            context: WizardContext::new(),
            formatter: OutputFormatter::new(),
        }
    }

    /// Add a step to the wizard
    pub fn add_step(mut self, step: Box<dyn WizardStep>) -> Self {
        self.steps.push(step);
        self
    }

    /// Enable resume functionality
    pub fn with_resume(mut self) -> Self {
        self.context.enable_resume();
        self
    }

    /// Set the output formatter
    pub fn with_formatter(mut self, formatter: OutputFormatter) -> Self {
        self.formatter = formatter;
        self
    }

    /// Run the wizard
    pub fn run(mut self) -> CliResult<WizardContext> {
        if self.steps.is_empty() {
            return Err(CliError::execution("Wizard has no steps"));
        }

        let mut current_index = 0;
        let mut history: Vec<usize> = Vec::new();

        while current_index < self.steps.len() {
            let step = &self.steps[current_index];

            // Skip already completed steps if resuming
            if self.context.is_resumable() && self.context.is_completed(step.name()) {
                self.formatter
                    .info(&format!("Skipping completed step: {}", step.name()));
                current_index += 1;
                continue;
            }

            // Check if step can be skipped
            if step.can_skip(&self.context) {
                self.formatter
                    .info(&format!("Skipping step: {}", step.name()));
                current_index += 1;
                continue;
            }

            // Show step description if available
            if let Some(desc) = step.description() {
                self.formatter
                    .section(&format!("{}: {}", step.name(), desc));
            } else {
                self.formatter.section(step.name());
            }

            // Execute the step
            match step.execute(&mut self.context) {
                Ok(StepResult::Continue) => {
                    self.context.mark_completed(step.name().to_string());
                    history.push(current_index);
                    current_index += 1;
                }
                Ok(StepResult::SkipTo(target)) => {
                    // Find the target step
                    if let Some(pos) = self.steps.iter().position(|s| s.name() == target) {
                        self.context.mark_completed(step.name().to_string());
                        history.push(current_index);
                        current_index = pos;
                    } else {
                        return Err(CliError::execution(format!("Step '{}' not found", target)));
                    }
                }
                Ok(StepResult::Back) => {
                    if let Some(prev) = history.pop() {
                        current_index = prev;
                    } else {
                        self.formatter.warning("Already at first step");
                    }
                }
                Ok(StepResult::Complete) => {
                    self.context.mark_completed(step.name().to_string());
                    break;
                }
                Ok(StepResult::Cancel) => {
                    return Err(CliError::Cancelled);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(self.context)
    }

    /// Get a reference to the context
    pub fn context(&self) -> &WizardContext {
        &self.context
    }

    /// Get a mutable reference to the context
    pub fn context_mut(&mut self) -> &mut WizardContext {
        &mut self.context
    }
}

impl Default for Wizard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStep {
        name: String,
        result: StepResult,
    }

    impl TestStep {
        fn new(name: &str, result: StepResult) -> Box<Self> {
            Box::new(Self {
                name: name.to_string(),
                result,
            })
        }
    }

    impl WizardStep for TestStep {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute(&self, context: &mut WizardContext) -> CliResult<StepResult> {
            context.set(self.name.clone(), "executed");
            Ok(self.result.clone())
        }
    }

    #[test]
    fn test_wizard_context() {
        let mut context = WizardContext::new();
        context.set("key", "value");
        assert_eq!(context.get("key"), Some("value"));
        assert!(context.contains("key"));
        assert_eq!(context.remove("key"), Some("value".to_string()));
        assert!(!context.contains("key"));
    }

    #[test]
    fn test_wizard_completed_steps() {
        let mut context = WizardContext::new();
        context.mark_completed("step1");
        context.mark_completed("step2");
        assert!(context.is_completed("step1"));
        assert!(context.is_completed("step2"));
        assert!(!context.is_completed("step3"));
    }

    #[test]
    fn test_wizard_execution() {
        let wizard = Wizard::new()
            .add_step(TestStep::new("step1", StepResult::Continue))
            .add_step(TestStep::new("step2", StepResult::Continue))
            .add_step(TestStep::new("step3", StepResult::Complete));

        let context = wizard.run().unwrap();
        assert!(context.is_completed("step1"));
        assert!(context.is_completed("step2"));
        assert!(context.is_completed("step3"));
    }

    #[test]
    fn test_wizard_cancel() {
        let wizard = Wizard::new()
            .add_step(TestStep::new("step1", StepResult::Continue))
            .add_step(TestStep::new("step2", StepResult::Cancel));

        let result = wizard.run();
        assert!(matches!(result, Err(CliError::Cancelled)));
    }
}
