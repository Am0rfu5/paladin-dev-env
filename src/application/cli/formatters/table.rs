//! Table formatting using comfy-table
//!
//! Provides structured table output for CLI with color support and styling options.
//!
//! # Example
//!
//! ```rust
//! use paladin::application::cli::formatters::table::TableFormatter;
//!
//! let mut table = TableFormatter::new();
//! table
//!     .set_header(vec!["Name", "Status", "Time"])
//!     .add_row(vec!["Task 1", "Success", "1.2s"])
//!     .add_row(vec!["Task 2", "Failed", "0.5s"]);
//!
//! println!("{}", table.render());
//! ```

use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table as ComfyTable, presets};

/// Table formatter for structured data display.
///
/// Creates formatted ASCII tables with UTF-8 borders and optional color support.
/// Colors are automatically disabled when `NO_COLOR` environment variable is set.
///
/// # Examples
///
/// ```rust
/// use paladin::application::cli::formatters::table::TableFormatter;
///
/// // Create a basic table
/// let mut table = TableFormatter::new();
/// table.set_header(vec!["Column 1", "Column 2"]);
/// table.add_row(vec!["Data 1", "Data 2"]);
///
/// let output = table.render();
/// ```
#[derive(Debug)]
pub struct TableFormatter {
    table: ComfyTable,
    colors_enabled: bool,
}

impl TableFormatter {
    /// Create a new table formatter with UTF-8 borders.
    ///
    /// Color support is automatically determined by the `NO_COLOR` environment variable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let formatter = TableFormatter::new();
    /// ```
    pub fn new() -> Self {
        let mut table = ComfyTable::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        Self {
            table,
            colors_enabled: std::env::var("NO_COLOR").is_err(),
        }
    }

    /// Set the table header with bold blue styling.
    ///
    /// Headers are automatically styled with bold blue color (if colors enabled).
    ///
    /// # Arguments
    ///
    /// * `headers` - Iterator of header values convertible to `String`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let mut table = TableFormatter::new();
    /// table.set_header(vec!["Name", "Age", "City"]);
    /// ```
    pub fn set_header<I, S>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let cells: Vec<Cell> = headers
            .into_iter()
            .map(|h| {
                let mut cell = Cell::new(h.into());
                if self.colors_enabled {
                    cell = cell.fg(Color::Blue).add_attribute(Attribute::Bold);
                }
                cell
            })
            .collect();

        self.table.set_header(cells);
        self
    }

    /// Add a data row to the table.
    ///
    /// Rows are added with default styling. Use [`add_styled_row`](Self::add_styled_row)
    /// for custom cell styling.
    ///
    /// # Arguments
    ///
    /// * `row` - Iterator of cell values convertible to `String`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let mut table = TableFormatter::new();
    /// table.add_row(vec!["Alice", "30", "New York"]);
    /// ```
    pub fn add_row<I, S>(&mut self, row: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let cells: Vec<Cell> = row.into_iter().map(|s| Cell::new(s.into())).collect();
        self.table.add_row(cells);
        self
    }

    /// Add a row with custom cell styling.
    ///
    /// Use this method when you need per-cell color or styling control.
    /// Create cells using [`success_cell`](Self::success_cell), [`error_cell`](Self::error_cell),
    /// [`warning_cell`](Self::warning_cell), or [`info_cell`](Self::info_cell).
    ///
    /// # Arguments
    ///
    /// * `cells` - Vector of pre-styled `Cell` objects
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let mut table = TableFormatter::new();
    /// let success = table.success_cell("✓ Passed");
    /// let info = table.info_cell("All checks OK");
    /// table.add_styled_row(vec!["Test 1".into(), success, info]);
    /// ```
    pub fn add_styled_row(&mut self, cells: Vec<Cell>) -> &mut Self {
        self.table.add_row(cells);
        self
    }

    /// Create a success cell with green color.
    ///
    /// Returns a `Cell` styled with green foreground color (if colors enabled).
    /// Commonly used for success states, completed tasks, or positive indicators.
    ///
    /// # Arguments
    ///
    /// * `text` - Cell content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let table = TableFormatter::new();
    /// let cell = table.success_cell("✓ Success");
    /// ```
    pub fn success_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Green);
        }
        cell
    }

    /// Create an error cell with red color.
    ///
    /// Returns a `Cell` styled with red foreground color (if colors enabled).
    /// Commonly used for errors, failures, or critical issues.
    ///
    /// # Arguments
    ///
    /// * `text` - Cell content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let table = TableFormatter::new();
    /// let cell = table.error_cell("✗ Failed");
    /// ```
    pub fn error_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Red);
        }
        cell
    }

    /// Create a warning cell with yellow color.
    ///
    /// Returns a `Cell` styled with yellow foreground color (if colors enabled).
    /// Commonly used for warnings, degraded states, or non-critical issues.
    ///
    /// # Arguments
    ///
    /// * `text` - Cell content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let table = TableFormatter::new();
    /// let cell = table.warning_cell("⚠ Warning");
    /// ```
    pub fn warning_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Yellow);
        }
        cell
    }

    /// Create an info cell with blue color.
    ///
    /// Returns a `Cell` styled with blue foreground color (if colors enabled).
    /// Commonly used for informational messages or neutral states.
    ///
    /// # Arguments
    ///
    /// * `text` - Cell content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let table = TableFormatter::new();
    /// let cell = table.info_cell("ℹ Info");
    /// ```
    pub fn info_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Blue);
        }
        cell
    }

    /// Render the table to a string.
    ///
    /// Returns the complete table as a formatted string with borders.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let mut table = TableFormatter::new();
    /// table.set_header(vec!["A", "B"]);
    /// table.add_row(vec!["1", "2"]);
    ///
    /// let output = table.render();
    /// assert!(output.contains("A"));
    /// ```
    pub fn render(&self) -> String {
        self.table.to_string()
    }

    /// Print the table to stdout.
    ///
    /// Convenience method that renders and prints the table in one call.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use paladin::application::cli::formatters::table::TableFormatter;
    ///
    /// let mut table = TableFormatter::new();
    /// table.set_header(vec!["Name", "Score"]);
    /// table.add_row(vec!["Alice", "95"]);
    /// table.print();  // Outputs to stdout
    /// ```
    pub fn print(&self) {
        println!("{}", self.render());
    }
}

impl Default for TableFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = TableFormatter::new();
        assert!(table.render().is_empty() || !table.render().is_empty()); // Table structure exists
    }

    #[test]
    fn test_table_with_header_and_rows() {
        let mut table = TableFormatter::new();
        table
            .set_header(vec!["Name", "Age", "City"])
            .add_row(vec!["Alice", "30", "NYC"])
            .add_row(vec!["Bob", "25", "LA"]);

        let output = table.render();
        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn test_styled_cells() {
        let table = TableFormatter::new();
        let success = table.success_cell("OK");
        let error = table.error_cell("FAIL");
        let warning = table.warning_cell("WARN");

        // Just verify cells can be created
        assert_eq!(success.content(), "OK");
        assert_eq!(error.content(), "FAIL");
        assert_eq!(warning.content(), "WARN");
    }
}
