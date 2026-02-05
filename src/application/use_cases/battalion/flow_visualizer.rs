//! Flow Visualizer
//!
//! Provides ASCII and Mermaid visualization for Maneuver flow expressions.
//! Converts FlowExpression AST into human-readable graph formats.

use crate::core::platform::container::battalion::parser::FlowExpression;

/// Format for flow visualization output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationFormat {
    /// ASCII tree format for terminal display
    Ascii,
    /// Mermaid.js flowchart format for documentation
    Mermaid,
}

/// Visualizer for Maneuver flow expressions
///
/// Converts FlowExpression AST into visual representations suitable for
/// documentation, debugging, and understanding complex workflows.
///
/// # Examples
///
/// ## ASCII Visualization
///
/// ```ignore
/// use paladin::application::use_cases::battalion::flow_visualizer::FlowVisualizer;
/// use paladin::core::platform::container::battalion::parser::FlowParser;
///
/// let flow = FlowParser::parse("agent1 -> agent2 -> (agent3 | agent4)").unwrap();
/// let ascii = FlowVisualizer::to_ascii(&flow);
/// println!("{}", ascii);
/// // Output:
/// // agent1
/// //   └─> agent2
/// //        └─> [PARALLEL]
/// //             ├─> agent3
/// //             └─> agent4
/// ```
///
/// ## Mermaid Visualization
///
/// ```ignore
/// let mermaid = FlowVisualizer::to_mermaid(&flow);
/// println!("{}", mermaid);
/// // Output:
/// // flowchart LR
/// //     agent1 --> agent2
/// //     agent2 --> parallel_1[Parallel]
/// //     parallel_1 --> agent3
/// //     parallel_1 --> agent4
/// ```
pub struct FlowVisualizer;

impl FlowVisualizer {
    /// Generate ASCII tree visualization of a flow expression
    ///
    /// Creates a hierarchical text representation using box-drawing characters
    /// to show the structure of sequential and parallel execution patterns.
    ///
    /// # Arguments
    ///
    /// * `expr` - The flow expression to visualize
    ///
    /// # Returns
    ///
    /// A String containing the ASCII tree representation
    ///
    /// # Examples
    ///
    /// ```
    /// use paladin::application::use_cases::battalion::flow_visualizer::FlowVisualizer;
    /// use paladin::core::platform::container::battalion::parser::FlowParser;
    ///
    /// let flow = FlowParser::parse("a -> b").unwrap();
    /// let ascii = FlowVisualizer::to_ascii(&flow);
    /// assert!(ascii.contains("a"));
    /// assert!(ascii.contains("b"));
    /// ```
    pub fn to_ascii(expr: &FlowExpression) -> String {
        let mut result = String::new();
        Self::render_ascii(expr, &mut result, "", true);
        result
    }

    /// Generate Mermaid.js flowchart syntax for a flow expression
    ///
    /// Creates valid Mermaid flowchart syntax that can be rendered in
    /// documentation, GitHub README files, or Mermaid-compatible tools.
    ///
    /// # Arguments
    ///
    /// * `expr` - The flow expression to visualize
    ///
    /// # Returns
    ///
    /// A String containing the Mermaid flowchart definition
    ///
    /// # Examples
    ///
    /// ```
    /// use paladin::application::use_cases::battalion::flow_visualizer::FlowVisualizer;
    /// use paladin::core::platform::container::battalion::parser::FlowParser;
    ///
    /// let flow = FlowParser::parse("a -> b").unwrap();
    /// let mermaid = FlowVisualizer::to_mermaid(&flow);
    /// assert!(mermaid.starts_with("flowchart LR"));
    /// ```
    pub fn to_mermaid(expr: &FlowExpression) -> String {
        let mut result = String::from("flowchart LR\n");
        let mut node_counter = 0;
        Self::render_mermaid(expr, &mut result, &mut node_counter, None);
        result
    }

    /// Recursive helper for ASCII rendering
    fn render_ascii(expr: &FlowExpression, output: &mut String, prefix: &str, is_last: bool) {
        match expr {
            FlowExpression::Agent(name) => {
                output.push_str(prefix);
                output.push_str(if is_last { "└─> " } else { "├─> " });
                output.push_str(name);
                output.push('\n');
            }
            FlowExpression::Sequential(exprs) => {
                for (i, sub_expr) in exprs.iter().enumerate() {
                    let is_last_in_seq = i == exprs.len() - 1;

                    // For sequential, each item extends downward
                    if i == 0 {
                        // First item uses current prefix
                        Self::render_ascii(sub_expr, output, prefix, is_last);
                    } else {
                        // Subsequent items get extended prefix
                        let new_prefix = if is_last {
                            format!("{}    ", prefix)
                        } else {
                            format!("{}│   ", prefix)
                        };
                        Self::render_ascii(sub_expr, output, &new_prefix, is_last_in_seq);
                    }
                }
            }
            FlowExpression::Parallel(exprs) => {
                output.push_str(prefix);
                output.push_str(if is_last { "└─> " } else { "├─> " });
                output.push_str("[PARALLEL]\n");

                let new_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };

                for (i, sub_expr) in exprs.iter().enumerate() {
                    let is_last_parallel = i == exprs.len() - 1;
                    Self::render_ascii(sub_expr, output, &new_prefix, is_last_parallel);
                }
            }
        }
    }

    /// Recursive helper for Mermaid rendering
    ///
    /// Returns the node ID of the last node in the expression
    fn render_mermaid(
        expr: &FlowExpression,
        output: &mut String,
        counter: &mut usize,
        parent_id: Option<String>,
    ) -> String {
        match expr {
            FlowExpression::Agent(name) => {
                let node_id = format!("agent_{}", name);

                if let Some(parent) = parent_id {
                    output.push_str(&format!("    {} --> {}\n", parent, node_id));
                } else {
                    // Standalone node - write it explicitly
                    output.push_str(&format!("    {}\n", node_id));
                }

                node_id
            }
            FlowExpression::Sequential(exprs) => {
                let mut current_parent = parent_id;
                let mut last_id = String::new();

                for expr in exprs {
                    last_id = Self::render_mermaid(expr, output, counter, current_parent);
                    current_parent = Some(last_id.clone());
                }

                last_id
            }
            FlowExpression::Parallel(exprs) => {
                // Create parallel junction node
                *counter += 1;
                let parallel_id = format!("parallel_{}", counter);

                if let Some(parent) = parent_id {
                    output.push_str(&format!("    {} --> {}[Parallel]\n", parent, parallel_id));
                } else {
                    // Standalone parallel node - define it explicitly
                    output.push_str(&format!("    {}[Parallel]\n", parallel_id));
                }

                // Render all parallel branches
                for expr in exprs {
                    Self::render_mermaid(expr, output, counter, Some(parallel_id.clone()));
                }

                parallel_id
            }
        }
    }

    /// Visualize a flow expression in the specified format
    ///
    /// # Arguments
    ///
    /// * `expr` - The flow expression to visualize
    /// * `format` - The desired output format (ASCII or Mermaid)
    ///
    /// # Returns
    ///
    /// A String containing the visualization in the requested format
    pub fn visualize(expr: &FlowExpression, format: VisualizationFormat) -> String {
        match format {
            VisualizationFormat::Ascii => Self::to_ascii(expr),
            VisualizationFormat::Mermaid => Self::to_mermaid(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::battalion::parser::FlowParser;

    #[test]
    fn test_ascii_simple_agent() {
        let flow = FlowParser::parse("agent1").unwrap();
        let ascii = FlowVisualizer::to_ascii(&flow);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("└─>") || ascii.contains("├─>"));
    }

    #[test]
    fn test_ascii_sequential() {
        let flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
        let ascii = FlowVisualizer::to_ascii(&flow);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("agent2"));
        assert!(ascii.contains("agent3"));
    }

    #[test]
    fn test_ascii_parallel() {
        let flow = FlowParser::parse("(agent1, agent2)").unwrap();
        let ascii = FlowVisualizer::to_ascii(&flow);

        assert!(ascii.contains("PARALLEL"));
        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("agent2"));
    }

    #[test]
    fn test_ascii_nested() {
        let flow = FlowParser::parse("agent1 -> (agent2, agent3)").unwrap();
        let ascii = FlowVisualizer::to_ascii(&flow);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("PARALLEL"));
        assert!(ascii.contains("agent2"));
        assert!(ascii.contains("agent3"));
    }

    #[test]
    fn test_mermaid_simple_agent() {
        let flow = FlowParser::parse("agent1").unwrap();
        let mermaid = FlowVisualizer::to_mermaid(&flow);

        assert!(mermaid.starts_with("flowchart LR"));
        assert!(mermaid.contains("agent1"));
    }

    #[test]
    fn test_mermaid_sequential() {
        let flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
        let mermaid = FlowVisualizer::to_mermaid(&flow);

        assert!(mermaid.starts_with("flowchart LR"));
        assert!(mermaid.contains("agent1"));
        assert!(mermaid.contains("agent2"));
        assert!(mermaid.contains("agent3"));
        assert!(mermaid.contains("-->"));
    }

    #[test]
    fn test_mermaid_parallel() {
        let flow = FlowParser::parse("(agent1, agent2)").unwrap();
        let mermaid = FlowVisualizer::to_mermaid(&flow);

        assert!(mermaid.starts_with("flowchart LR"));
        assert!(mermaid.contains("parallel_"));
        assert!(mermaid.contains("Parallel"));
    }

    #[test]
    fn test_mermaid_nested() {
        let flow = FlowParser::parse("agent1 -> (agent2, agent3)").unwrap();
        let mermaid = FlowVisualizer::to_mermaid(&flow);

        assert!(mermaid.starts_with("flowchart LR"));
        assert!(mermaid.contains("agent1"));
        assert!(mermaid.contains("parallel_"));
        assert!(mermaid.contains("agent2"));
        assert!(mermaid.contains("agent3"));
    }

    #[test]
    fn test_mermaid_complex() {
        let flow = FlowParser::parse("a -> (b, c) -> d").unwrap();
        let mermaid = FlowVisualizer::to_mermaid(&flow);

        assert!(mermaid.starts_with("flowchart LR"));
        assert!(mermaid.contains("agent_a"));
        assert!(mermaid.contains("agent_b"));
        assert!(mermaid.contains("agent_c"));
        assert!(mermaid.contains("agent_d"));
        assert!(mermaid.contains("parallel_"));
    }

    #[test]
    fn test_visualize_with_format() {
        let flow = FlowParser::parse("a -> b").unwrap();

        let ascii = FlowVisualizer::visualize(&flow, VisualizationFormat::Ascii);
        assert!(ascii.contains("a"));
        assert!(ascii.contains("b"));

        let mermaid = FlowVisualizer::visualize(&flow, VisualizationFormat::Mermaid);
        assert!(mermaid.starts_with("flowchart LR"));
    }

    #[test]
    fn test_ascii_is_not_empty() {
        let flow = FlowParser::parse("single").unwrap();
        let ascii = FlowVisualizer::to_ascii(&flow);
        assert!(!ascii.is_empty());
    }

    #[test]
    fn test_mermaid_is_valid() {
        let flow = FlowParser::parse("first -> second").unwrap();
        let mermaid = FlowVisualizer::to_mermaid(&flow);

        // Basic validation: should have flowchart declaration and arrows
        assert!(mermaid.contains("flowchart"));
        assert!(mermaid.contains("-->"));

        // Should not have empty lines at start
        assert!(!mermaid.starts_with('\n'));
    }
}
