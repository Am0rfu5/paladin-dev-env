//! Core output formatting with colors, boxes, and headers
//!
//! Provides utilities for formatted terminal output that respects NO_COLOR
//! environment variable and supports quiet/verbose modes.

use colored::*;
use std::env;

/// Output styling options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStyle {
    /// Success message (green)
    Success,
    /// Error message (red)
    Error,
    /// Warning message (yellow)
    Warning,
    /// Informational message (blue)
    Info,
    /// Link or reference (cyan)
    Link,
    /// Default styling
    Default,
}

/// Output formatter with color and style support
#[derive(Debug, Clone)]
pub struct OutputFormatter {
    /// Whether colors are enabled
    colors_enabled: bool,
    /// Quiet mode (minimal output)
    quiet: bool,
    /// Verbose mode (detailed output)
    verbose: bool,
}

impl OutputFormatter {
    /// Create a new output formatter
    pub fn new() -> Self {
        Self {
            colors_enabled: !Self::no_color_requested(),
            quiet: false,
            verbose: false,
        }
    }

    /// Create a formatter with quiet mode enabled
    pub fn quiet() -> Self {
        Self {
            colors_enabled: !Self::no_color_requested(),
            quiet: true,
            verbose: false,
        }
    }

    /// Create a formatter with verbose mode enabled
    pub fn with_verbose() -> Self {
        Self {
            colors_enabled: !Self::no_color_requested(),
            quiet: false,
            verbose: true,
        }
    }

    /// Check if NO_COLOR environment variable is set
    fn no_color_requested() -> bool {
        env::var("NO_COLOR").is_ok()
    }

    /// Set quiet mode
    pub fn set_quiet(&mut self, quiet: bool) {
        self.quiet = quiet;
        if quiet {
            self.verbose = false;
        }
    }

    /// Set verbose mode
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
        if verbose {
            self.quiet = false;
        }
    }

    /// Check if quiet mode is enabled
    pub fn is_quiet(&self) -> bool {
        self.quiet
    }

    /// Check if verbose mode is enabled
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /// Format text with the given style
    pub fn style(&self, text: &str, style: OutputStyle) -> String {
        if !self.colors_enabled {
            return text.to_string();
        }

        match style {
            OutputStyle::Success => text.green().to_string(),
            OutputStyle::Error => text.red().to_string(),
            OutputStyle::Warning => text.yellow().to_string(),
            OutputStyle::Info => text.blue().to_string(),
            OutputStyle::Link => text.cyan().to_string(),
            OutputStyle::Default => text.to_string(),
        }
    }

    /// Print a success message
    pub fn success(&self, message: &str) {
        if !self.quiet {
            println!("{} {}", self.style("✓", OutputStyle::Success), message);
        }
    }

    /// Print an error message
    pub fn error(&self, message: &str) {
        eprintln!("{} {}", self.style("✗", OutputStyle::Error), message);
    }

    /// Print a warning message
    pub fn warning(&self, message: &str) {
        if !self.quiet {
            println!("{} {}", self.style("⚠", OutputStyle::Warning), message);
        }
    }

    /// Print an info message
    pub fn info(&self, message: &str) {
        if !self.quiet {
            println!("{} {}", self.style("ℹ", OutputStyle::Info), message);
        }
    }

    /// Print a verbose message (only in verbose mode)
    pub fn verbose(&self, message: &str) {
        if self.verbose {
            println!("{}", self.style(message, OutputStyle::Default));
        }
    }

    /// Print a header with box drawing
    pub fn header(&self, title: &str) {
        if self.quiet {
            return;
        }

        let width = title.len() + 4;
        let border = "═".repeat(width);

        println!("┌{}┐", border);
        println!("│ {} │", self.style(title, OutputStyle::Info));
        println!("└{}┘", border);
    }

    /// Print a section header
    pub fn section(&self, title: &str) {
        if self.quiet {
            return;
        }

        println!(
            "\n{}",
            self.style(&format!("━━ {} ━━", title), OutputStyle::Info)
        );
    }

    /// Print a box with content
    pub fn box_message(&self, content: &[&str]) {
        if self.quiet {
            return;
        }

        let max_width = content.iter().map(|s| s.len()).max().unwrap_or(0);
        let border = "─".repeat(max_width + 2);

        println!("┌{}┐", border);
        for line in content {
            println!("│ {:<width$} │", line, width = max_width);
        }
        println!("└{}┘", border);
    }

    /// Format a key-value pair
    pub fn key_value(&self, key: &str, value: &str) -> String {
        format!("{}: {}", self.style(key, OutputStyle::Info), value)
    }

    /// Print an emoji if colors are enabled, otherwise print alternative text
    pub fn emoji_or<'a>(&self, emoji: &'a str, alt: &'a str) -> &'a str {
        if self.colors_enabled { emoji } else { alt }
    }

    /// Print a separator line
    pub fn separator(&self) {
        if !self.quiet {
            println!("{}", "═".repeat(64));
        }
    }

    /// Print a blank line (unless in quiet mode)
    pub fn blank_line(&self) {
        if !self.quiet {
            println!();
        }
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}
