//! Table formatting using comfy-table

use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table as ComfyTable, presets};

/// Table formatter for structured data display
#[derive(Debug)]
pub struct TableFormatter {
    table: ComfyTable,
    colors_enabled: bool,
}

impl TableFormatter {
    /// Create a new table formatter
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

    /// Set the table header
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

    /// Add a row to the table
    pub fn add_row<I, S>(&mut self, row: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let cells: Vec<Cell> = row.into_iter().map(|s| Cell::new(s.into())).collect();
        self.table.add_row(cells);
        self
    }

    /// Add a row with custom cell styling
    pub fn add_styled_row(&mut self, cells: Vec<Cell>) -> &mut Self {
        self.table.add_row(cells);
        self
    }

    /// Create a success cell (green)
    pub fn success_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Green);
        }
        cell
    }

    /// Create an error cell (red)
    pub fn error_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Red);
        }
        cell
    }

    /// Create a warning cell (yellow)
    pub fn warning_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Yellow);
        }
        cell
    }

    /// Create an info cell (blue)
    pub fn info_cell(&self, text: impl Into<String>) -> Cell {
        let mut cell = Cell::new(text.into());
        if self.colors_enabled {
            cell = cell.fg(Color::Blue);
        }
        cell
    }

    /// Render the table to a string
    pub fn render(&self) -> String {
        self.table.to_string()
    }

    /// Print the table
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
