//! Abstract Syntax Tree for Flow DSL
//!
//! Defines the AST structure for representing parsed flow expressions.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Flow expression AST node
///
/// Represents a parsed flow expression as a tree structure that can be
/// executed by the Maneuver execution service.
///
/// # Examples
///
/// ```
/// use paladin::core::platform::container::battalion::parser::FlowExpression;
///
/// // Agent node
/// let agent = FlowExpression::Agent("researcher".to_string());
///
/// // Sequential execution
/// let seq = FlowExpression::Sequential(vec![
///     FlowExpression::Agent("a".to_string()),
///     FlowExpression::Agent("b".to_string()),
/// ]);
///
/// // Parallel execution
/// let par = FlowExpression::Parallel(vec![
///     FlowExpression::Agent("x".to_string()),
///     FlowExpression::Agent("y".to_string()),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowExpression {
    /// A single agent node
    Agent(String),
    /// Sequential execution of multiple expressions
    Sequential(Vec<FlowExpression>),
    /// Parallel execution of multiple expressions
    Parallel(Vec<FlowExpression>),
}

impl FlowExpression {
    /// Get all unique agent names referenced in this expression
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowExpression;
    ///
    /// let expr = FlowExpression::Sequential(vec![
    ///     FlowExpression::Agent("a".to_string()),
    ///     FlowExpression::Agent("b".to_string()),
    ///     FlowExpression::Agent("a".to_string()), // duplicate
    /// ]);
    ///
    /// let names = expr.agent_names();
    /// assert_eq!(names.len(), 2);
    /// assert!(names.contains("a"));
    /// assert!(names.contains("b"));
    /// ```
    pub fn agent_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        self.collect_agent_names(&mut names);
        names
    }

    fn collect_agent_names(&self, names: &mut HashSet<String>) {
        match self {
            FlowExpression::Agent(name) => {
                names.insert(name.clone());
            }
            FlowExpression::Sequential(exprs) | FlowExpression::Parallel(exprs) => {
                for expr in exprs {
                    expr.collect_agent_names(names);
                }
            }
        }
    }

    /// Calculate the maximum nesting depth of this expression
    ///
    /// Returns the depth of the deepest nested expression.
    /// - Agent nodes have depth 1
    /// - Sequential/Parallel nodes have depth 1 + max(child depths)
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowExpression;
    ///
    /// let expr = FlowExpression::Sequential(vec![
    ///     FlowExpression::Agent("a".to_string()), // depth 1
    ///     FlowExpression::Parallel(vec![          // depth 2
    ///         FlowExpression::Agent("b".to_string()),
    ///         FlowExpression::Agent("c".to_string()),
    ///     ]),
    /// ]);
    ///
    /// assert_eq!(expr.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        match self {
            FlowExpression::Agent(_) => 1,
            FlowExpression::Sequential(exprs) | FlowExpression::Parallel(exprs) => {
                1 + exprs.iter().map(|e| e.depth()).max().unwrap_or(0)
            }
        }
    }

    /// Calculate the maximum width (number of parallel branches) at any level
    ///
    /// Returns the maximum number of parallel branches at any level in the expression tree.
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowExpression;
    ///
    /// let expr = FlowExpression::Parallel(vec![
    ///     FlowExpression::Agent("a".to_string()),
    ///     FlowExpression::Agent("b".to_string()),
    ///     FlowExpression::Agent("c".to_string()),
    /// ]);
    ///
    /// assert_eq!(expr.width(), 3);
    /// ```
    pub fn width(&self) -> usize {
        match self {
            FlowExpression::Agent(_) => 1,
            FlowExpression::Sequential(exprs) => exprs.iter().map(|e| e.width()).max().unwrap_or(1),
            FlowExpression::Parallel(exprs) => {
                let child_widths: usize = exprs.iter().map(|e| e.width()).sum();
                child_widths.max(exprs.len())
            }
        }
    }

    /// Count the total number of agent nodes in this expression
    ///
    /// # Example
    ///
    /// ```
    /// use paladin::core::platform::container::battalion::parser::FlowExpression;
    ///
    /// let expr = FlowExpression::Sequential(vec![
    ///     FlowExpression::Agent("a".to_string()),
    ///     FlowExpression::Parallel(vec![
    ///         FlowExpression::Agent("b".to_string()),
    ///         FlowExpression::Agent("c".to_string()),
    ///     ]),
    /// ]);
    ///
    /// assert_eq!(expr.agent_count(), 3);
    /// ```
    pub fn agent_count(&self) -> usize {
        match self {
            FlowExpression::Agent(_) => 1,
            FlowExpression::Sequential(exprs) | FlowExpression::Parallel(exprs) => {
                exprs.iter().map(|e| e.agent_count()).sum()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_names_simple() {
        let expr = FlowExpression::Agent("test".to_string());
        let names = expr.agent_names();
        assert_eq!(names.len(), 1);
        assert!(names.contains("test"));
    }

    #[test]
    fn test_agent_names_sequential() {
        let expr = FlowExpression::Sequential(vec![
            FlowExpression::Agent("a".to_string()),
            FlowExpression::Agent("b".to_string()),
        ]);
        let names = expr.agent_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains("a"));
        assert!(names.contains("b"));
    }

    #[test]
    fn test_agent_names_deduplication() {
        let expr = FlowExpression::Sequential(vec![
            FlowExpression::Agent("a".to_string()),
            FlowExpression::Agent("a".to_string()),
        ]);
        let names = expr.agent_names();
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn test_depth_simple() {
        let expr = FlowExpression::Agent("test".to_string());
        assert_eq!(expr.depth(), 1);
    }

    #[test]
    fn test_depth_nested() {
        let expr = FlowExpression::Sequential(vec![
            FlowExpression::Agent("a".to_string()),
            FlowExpression::Parallel(vec![
                FlowExpression::Agent("b".to_string()),
                FlowExpression::Sequential(vec![
                    FlowExpression::Agent("c".to_string()),
                    FlowExpression::Agent("d".to_string()),
                ]),
            ]),
        ]);
        // Depth calculation:
        // 1: Top-level Sequential
        // 2: Parallel inside Sequential
        // 3: Inner Sequential inside Parallel
        // 4: Agent inside inner Sequential
        assert_eq!(expr.depth(), 4);
    }

    #[test]
    fn test_width_simple() {
        let expr = FlowExpression::Agent("test".to_string());
        assert_eq!(expr.width(), 1);
    }

    #[test]
    fn test_width_parallel() {
        let expr = FlowExpression::Parallel(vec![
            FlowExpression::Agent("a".to_string()),
            FlowExpression::Agent("b".to_string()),
            FlowExpression::Agent("c".to_string()),
        ]);
        assert_eq!(expr.width(), 3);
    }

    #[test]
    fn test_agent_count() {
        let expr = FlowExpression::Sequential(vec![
            FlowExpression::Agent("a".to_string()),
            FlowExpression::Parallel(vec![
                FlowExpression::Agent("b".to_string()),
                FlowExpression::Agent("c".to_string()),
            ]),
            FlowExpression::Agent("d".to_string()),
        ]);
        assert_eq!(expr.agent_count(), 4);
    }

    #[test]
    fn test_serialization() {
        let expr = FlowExpression::Sequential(vec![
            FlowExpression::Agent("a".to_string()),
            FlowExpression::Agent("b".to_string()),
        ]);

        let json = serde_json::to_string(&expr).unwrap();
        let deserialized: FlowExpression = serde_json::from_str(&json).unwrap();

        assert_eq!(expr, deserialized);
    }
}
