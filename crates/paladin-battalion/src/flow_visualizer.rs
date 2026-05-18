//! Flow Visualizer
//!
//! Provides ASCII and Mermaid visualization for Maneuver flow expressions.
//! Converts FlowExpression AST into human-readable graph formats.

use paladin_core::platform::container::battalion::parser::FlowExpression;
use std::collections::HashMap;
use std::time::Duration;

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
/// use paladin_battalion::flow_visualizer::FlowVisualizer;
/// use paladin_core::platform::container::battalion::parser::FlowParser;
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
    /// use paladin_battalion::flow_visualizer::FlowVisualizer;
    /// use paladin_core::platform::container::battalion::parser::FlowParser;
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
    /// use paladin_battalion::flow_visualizer::FlowVisualizer;
    /// use paladin_core::platform::container::battalion::parser::FlowParser;
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

    /// Generate ASCII visualization with timing metrics overlay
    ///
    /// Displays execution time next to each agent, highlights the slowest agent
    /// (bottleneck), and shows total workflow time at the bottom.
    ///
    /// # Arguments
    ///
    /// * `expr` - The flow expression to visualize
    /// * `metrics` - HashMap mapping agent names to their execution durations
    ///
    /// # Returns
    ///
    /// A String containing the ASCII tree with timing information
    ///
    /// # Examples
    ///
    /// ```
    /// use paladin_battalion::flow_visualizer::FlowVisualizer;
    /// use paladin_core::platform::container::battalion::parser::FlowParser;
    /// use std::collections::HashMap;
    /// use std::time::Duration;
    ///
    /// let flow = FlowParser::parse("agent1 -> agent2").unwrap();
    /// let mut metrics = HashMap::new();
    /// metrics.insert("agent1".to_string(), Duration::from_millis(100));
    /// metrics.insert("agent2".to_string(), Duration::from_millis(250));
    ///
    /// let ascii_with_timing = FlowVisualizer::with_timing(&flow, &metrics);
    /// assert!(ascii_with_timing.contains("100ms"));
    /// assert!(ascii_with_timing.contains("250ms"));
    /// assert!(ascii_with_timing.contains("Total:"));
    /// ```
    pub fn with_timing(expr: &FlowExpression, metrics: &HashMap<String, Duration>) -> String {
        let mut result = String::new();

        // Find the slowest agent for highlighting
        let slowest = metrics
            .iter()
            .max_by_key(|(_, duration)| *duration)
            .map(|(name, _)| name.as_str());

        Self::render_ascii_with_timing(expr, &mut result, "", true, metrics, slowest);

        // Add total time at the bottom
        let total: Duration = metrics.values().sum();
        result.push_str(&format!("\nTotal: {}\n", Self::format_duration(total)));

        result
    }

    /// Recursive helper for ASCII rendering with timing
    fn render_ascii_with_timing(
        expr: &FlowExpression,
        output: &mut String,
        prefix: &str,
        is_last: bool,
        metrics: &HashMap<String, Duration>,
        slowest: Option<&str>,
    ) {
        match expr {
            FlowExpression::Agent(name) => {
                output.push_str(prefix);
                output.push_str(if is_last { "└─> " } else { "├─> " });
                output.push_str(name);

                // Add timing info if available
                if let Some(duration) = metrics.get(name) {
                    let time_str = Self::format_duration(*duration);
                    output.push_str(&format!(" [{}]", time_str));

                    // Mark bottleneck
                    if slowest == Some(name.as_str()) {
                        output.push_str(" ⚠️  BOTTLENECK");
                    }
                }

                output.push('\n');
            }
            FlowExpression::Sequential(exprs) => {
                for (i, sub_expr) in exprs.iter().enumerate() {
                    let is_last_in_seq = i == exprs.len() - 1;

                    if i == 0 {
                        Self::render_ascii_with_timing(
                            sub_expr, output, prefix, is_last, metrics, slowest,
                        );
                    } else {
                        let new_prefix = if is_last {
                            format!("{}    ", prefix)
                        } else {
                            format!("{}│   ", prefix)
                        };
                        Self::render_ascii_with_timing(
                            sub_expr,
                            output,
                            &new_prefix,
                            is_last_in_seq,
                            metrics,
                            slowest,
                        );
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
                    Self::render_ascii_with_timing(
                        sub_expr,
                        output,
                        &new_prefix,
                        is_last_parallel,
                        metrics,
                        slowest,
                    );
                }
            }
        }
    }

    /// Format duration in human-readable form
    fn format_duration(duration: Duration) -> String {
        let ms = duration.as_millis();
        if ms < 1000 {
            format!("{}ms", ms)
        } else {
            format!("{:.2}s", duration.as_secs_f64())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_core::platform::container::battalion::parser::FlowParser;

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

    #[test]
    fn test_with_timing_simple() {
        let flow = FlowParser::parse("agent1").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("agent1".to_string(), Duration::from_millis(100));

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("100ms"));
        assert!(ascii.contains("Total:"));
        assert!(ascii.contains("BOTTLENECK"));
    }

    #[test]
    fn test_with_timing_sequential() {
        let flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("agent1".to_string(), Duration::from_millis(100));
        metrics.insert("agent2".to_string(), Duration::from_millis(250));
        metrics.insert("agent3".to_string(), Duration::from_millis(150));

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("agent2"));
        assert!(ascii.contains("agent3"));
        assert!(ascii.contains("100ms"));
        assert!(ascii.contains("250ms"));
        assert!(ascii.contains("150ms"));
        assert!(ascii.contains("Total:"));

        // agent2 should be marked as bottleneck (250ms is slowest)
        let lines: Vec<&str> = ascii.lines().collect();
        let agent2_line = lines.iter().find(|l| l.contains("agent2")).unwrap();
        assert!(agent2_line.contains("BOTTLENECK"));
    }

    #[test]
    fn test_with_timing_parallel() {
        let flow = FlowParser::parse("(agent1, agent2)").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("agent1".to_string(), Duration::from_millis(100));
        metrics.insert("agent2".to_string(), Duration::from_millis(200));

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        assert!(ascii.contains("PARALLEL"));
        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("agent2"));
        assert!(ascii.contains("100ms"));
        assert!(ascii.contains("200ms"));
    }

    #[test]
    fn test_with_timing_missing_metrics() {
        let flow = FlowParser::parse("agent1 -> agent2").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("agent1".to_string(), Duration::from_millis(100));
        // agent2 has no metrics

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("agent2"));
        assert!(ascii.contains("100ms"));
        // agent2 should still appear, just without timing
    }

    #[test]
    fn test_with_timing_nested() {
        let flow = FlowParser::parse("agent1 -> (agent2, agent3) -> agent4").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("agent1".to_string(), Duration::from_millis(100));
        metrics.insert("agent2".to_string(), Duration::from_millis(200));
        metrics.insert("agent3".to_string(), Duration::from_millis(150));
        metrics.insert("agent4".to_string(), Duration::from_millis(50));

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        assert!(ascii.contains("agent1"));
        assert!(ascii.contains("PARALLEL"));
        assert!(ascii.contains("agent2"));
        assert!(ascii.contains("agent3"));
        assert!(ascii.contains("agent4"));
        assert!(ascii.contains("200ms"));
        assert!(ascii.contains("BOTTLENECK"));
    }

    #[test]
    fn test_format_duration_milliseconds() {
        assert_eq!(
            FlowVisualizer::format_duration(Duration::from_millis(100)),
            "100ms"
        );
        assert_eq!(
            FlowVisualizer::format_duration(Duration::from_millis(999)),
            "999ms"
        );
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(
            FlowVisualizer::format_duration(Duration::from_millis(1000)),
            "1.00s"
        );
        assert_eq!(
            FlowVisualizer::format_duration(Duration::from_millis(1500)),
            "1.50s"
        );
        assert_eq!(
            FlowVisualizer::format_duration(Duration::from_secs(5)),
            "5.00s"
        );
    }

    #[test]
    fn test_timing_total_calculation() {
        let flow = FlowParser::parse("a -> b -> c").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("a".to_string(), Duration::from_millis(100));
        metrics.insert("b".to_string(), Duration::from_millis(200));
        metrics.insert("c".to_string(), Duration::from_millis(300));

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        // Total should be 600ms
        assert!(ascii.contains("Total: 600ms"));
    }

    #[test]
    fn test_timing_bottleneck_identification() {
        let flow = FlowParser::parse("fast -> slow -> medium").unwrap();
        let mut metrics = HashMap::new();
        metrics.insert("fast".to_string(), Duration::from_millis(50));
        metrics.insert("slow".to_string(), Duration::from_millis(500));
        metrics.insert("medium".to_string(), Duration::from_millis(200));

        let ascii = FlowVisualizer::with_timing(&flow, &metrics);

        // Only 'slow' should be marked as bottleneck
        let lines: Vec<&str> = ascii.lines().collect();

        let slow_line = lines.iter().find(|l| l.contains("slow")).unwrap();
        assert!(slow_line.contains("BOTTLENECK"));

        let fast_line = lines.iter().find(|l| l.contains("fast")).unwrap();
        assert!(!fast_line.contains("BOTTLENECK"));

        let medium_line = lines.iter().find(|l| l.contains("medium")).unwrap();
        assert!(!medium_line.contains("BOTTLENECK"));
    }
}
