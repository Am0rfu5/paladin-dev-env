//! Progress indicators (spinners and progress bars)

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

/// Progress indicator types
#[derive(Debug, Clone)]
pub enum ProgressIndicator {
    /// Spinner for indeterminate operations
    Spinner(Spinner),
    /// Progress bar for determinate operations
    Bar { current: u64, total: u64 },
}

/// Spinner for long-running operations
#[derive(Debug, Clone)]
pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    /// Create a new spinner with a message
    pub fn new(message: impl Into<String>) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message(message.into());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self { pb }
    }

    /// Update the spinner message
    pub fn set_message(&self, message: impl Into<String>) {
        self.pb.set_message(message.into());
    }

    /// Finish the spinner with a success message
    pub fn finish_with_message(&self, message: impl Into<String>) {
        self.pb.finish_with_message(message.into());
    }

    /// Finish the spinner and clear it
    pub fn finish_and_clear(&self) {
        self.pb.finish_and_clear();
    }

    /// Get the underlying progress bar
    pub fn progress_bar(&self) -> &ProgressBar {
        &self.pb
    }
}

/// Progress bar builder
pub struct ProgressBarBuilder {
    total: u64,
    message: String,
    template: Option<String>,
}

impl ProgressBarBuilder {
    /// Create a new progress bar builder
    pub fn new(total: u64) -> Self {
        Self {
            total,
            message: String::new(),
            template: None,
        }
    }

    /// Set the progress bar message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Set a custom template
    pub fn with_template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }

    /// Build the progress bar
    pub fn build(self) -> ProgressBar {
        let pb = ProgressBar::new(self.total);

        let template = self
            .template
            .unwrap_or_else(|| "{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)".to_string());

        pb.set_style(
            ProgressStyle::default_bar()
                .template(&template)
                .unwrap()
                .progress_chars("█▓░"),
        );

        if !self.message.is_empty() {
            pb.set_message(self.message);
        }

        pb
    }
}

/// Multi-progress manager for multiple concurrent operations
pub struct MultiProgressManager {
    mp: MultiProgress,
}

impl MultiProgressManager {
    /// Create a new multi-progress manager
    pub fn new() -> Self {
        Self {
            mp: MultiProgress::new(),
        }
    }

    /// Add a spinner to the manager
    pub fn add_spinner(&self, message: impl Into<String>) -> Spinner {
        let pb = self.mp.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message(message.into());
        pb.enable_steady_tick(Duration::from_millis(80));

        Spinner { pb }
    }

    /// Add a progress bar to the manager
    pub fn add_progress_bar(&self, total: u64, message: impl Into<String>) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new(total));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("█▓░"),
        );
        pb.set_message(message.into());
        pb
    }
}

impl Default for MultiProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new("Testing...");
        spinner.finish_and_clear();
    }

    #[test]
    fn test_spinner_update_message() {
        let spinner = Spinner::new("Initial message");
        spinner.set_message("Updated message");
        spinner.finish_with_message("Done!");
    }

    #[test]
    fn test_progress_bar_builder() {
        let pb = ProgressBarBuilder::new(100)
            .with_message("Processing")
            .build();

        pb.inc(50);
        assert_eq!(pb.position(), 50);
        pb.finish_and_clear();
    }

    #[test]
    fn test_multi_progress() {
        let manager = MultiProgressManager::new();
        let spinner = manager.add_spinner("Task 1");
        let pb = manager.add_progress_bar(100, "Task 2");

        spinner.finish_and_clear();
        pb.finish_and_clear();
    }
}
