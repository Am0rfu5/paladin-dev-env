//! Result formatting utilities
//!
//! This module provides formatters for Paladin and Battalion execution results.
//! Supports both human-readable and JSON output formats.

use crate::application::ports::output::paladin_port::{PaladinResult, StopReason};
use crate::core::platform::container::battalion::{BattalionResult, BattalionStatus};
use colored::Colorize;
use serde_json::{Value, json};

/// Output formatter for Paladin and Battalion results
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format a Paladin result for human-readable output
    ///
    /// # Arguments
    ///
    /// * `result` - The Paladin execution result
    /// * `verbose` - Whether to include detailed information (loops, timing, etc.)
    ///
    /// # Example Output (Normal Mode)
    ///
    /// ```text
    /// ════════════════════════════════════════════════════════════════════
    /// 📊 Paladin Execution Result
    /// ════════════════════════════════════════════════════════════════════
    ///
    /// → Output:
    /// The analysis shows that...
    ///
    /// → Statistics:
    ///   • Execution Time: 1.25s
    ///   • Tokens Used: 150
    ///   • Status: Completed ✓
    /// ════════════════════════════════════════════════════════════════════
    /// ```
    ///
    /// # Example Output (Verbose Mode)
    ///
    /// Includes additional information about loops and stop reason.
    pub fn format_paladin_result(result: &PaladinResult, verbose: bool) -> String {
        let mut output = String::new();

        output.push_str(&"═".repeat(80));
        output.push_str(&format!(
            "\n{} Paladin Execution Result\n",
            "📊".cyan().bold()
        ));
        output.push_str(&"═".repeat(80));
        output.push('\n');

        // Main output
        output.push_str(&format!("\n{} Output:\n", "→".cyan().bold()));
        output.push_str(&format!("{}\n", result.output));

        // Statistics section
        output.push_str(&format!("\n{} Statistics:\n", "→".cyan().bold()));
        output.push_str(&format!(
            "  {} Execution Time: {:.2}s\n",
            "•".cyan(),
            result.execution_time_ms as f64 / 1000.0
        ));
        output.push_str(&format!(
            "  {} Tokens Used: {}\n",
            "•".cyan(),
            result.token_count
        ));

        // Status with color coding
        let status_str = match &result.stop_reason {
            StopReason::Completed => format!("{} {}", "Completed".green(), "✓".green()),
            StopReason::StopWord(word) => {
                format!("{} {} ({})", "Stopped".yellow(), "⚠".yellow(), word)
            }
            StopReason::MaxLoops => format!("{} {}", "Max Loops Reached".yellow(), "⚠".yellow()),
            StopReason::Timeout => format!("{} {}", "Timeout".red(), "✗".red()),
        };
        output.push_str(&format!("  {} Status: {}\n", "•".cyan(), status_str));

        // Verbose mode: additional details
        if verbose {
            output.push_str(&format!(
                "  {} Reasoning Loops: {}\n",
                "•".cyan(),
                result.loop_count
            ));
            output.push_str(&format!(
                "  {} Stop Reason: {:?}\n",
                "•".cyan(),
                result.stop_reason
            ));
        }

        output.push_str(&"═".repeat(80));
        output.push('\n');

        output
    }

    /// Format a Paladin result as JSON for file output
    ///
    /// Includes comprehensive metadata, timing information, and execution details.
    ///
    /// # Arguments
    ///
    /// * `result` - The Paladin execution result
    ///
    /// # JSON Structure
    ///
    /// ```json
    /// {
    ///   "output": "The generated response...",
    ///   "metadata": {
    ///     "token_count": 150,
    ///     "execution_time_ms": 1250,
    ///     "loop_count": 1,
    ///     "stop_reason": "Completed"
    ///   },
    ///   "timestamp": "2026-01-26T10:30:00Z"
    /// }
    /// ```
    pub fn format_paladin_result_json(result: &PaladinResult) -> Value {
        json!({
            "output": result.output,
            "metadata": {
                "token_count": result.token_count,
                "execution_time_ms": result.execution_time_ms,
                "execution_time_seconds": result.execution_time_ms as f64 / 1000.0,
                "loop_count": result.loop_count,
                "stop_reason": format!("{:?}", result.stop_reason),
                "is_successful": result.stop_reason.is_successful(),
                "is_limit_reached": result.stop_reason.is_limit(),
            },
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Format a Battalion result for human-readable output
    ///
    /// # Arguments
    ///
    /// * `result` - The Battalion execution result
    /// * `verbose` - Whether to include detailed per-Paladin information
    ///
    /// # Example Output (Normal Mode)
    ///
    /// ```text
    /// ════════════════════════════════════════════════════════════════════
    /// 🏰 Battalion Execution Result: research_battalion
    /// ════════════════════════════════════════════════════════════════════
    ///
    /// → Final Output:
    /// [Aggregated result from all Paladins...]
    ///
    /// → Statistics:
    ///   • Total Paladins: 3
    ///   • Successful: 3 ✓
    ///   • Failed: 0
    ///   • Total Time: 5.25s
    ///   • Strategy: Formation
    /// ════════════════════════════════════════════════════════════════════
    /// ```
    pub fn format_battalion_result(result: &BattalionResult, verbose: bool) -> String {
        let mut output = String::new();

        output.push_str(&"═".repeat(80));
        output.push_str(&format!(
            "\n{} Battalion Execution Result: {}\n",
            "🏰".cyan().bold(),
            result.battalion_name.bold()
        ));
        output.push_str(&"═".repeat(80));
        output.push('\n');

        // Final aggregated output
        output.push_str(&format!("\n{} Final Output:\n", "→".cyan().bold()));
        output.push_str(&format!("{}\n", result.final_output));

        // Statistics section
        output.push_str(&format!("\n{} Statistics:\n", "→".cyan().bold()));
        output.push_str(&format!(
            "  {} Total Paladins: {}\n",
            "•".cyan(),
            result.paladin_results.len()
        ));

        let success_color = if result.paladin_success_count == result.paladin_results.len() {
            "green"
        } else {
            "yellow"
        };
        output.push_str(&format!(
            "  {} Successful: {} {}\n",
            "•".cyan(),
            result.paladin_success_count,
            if success_color == "green" {
                "✓".green()
            } else {
                "⚠".yellow()
            }
        ));

        if result.paladin_failure_count > 0 {
            output.push_str(&format!(
                "  {} Failed: {} {}\n",
                "•".cyan(),
                result.paladin_failure_count,
                "✗".red()
            ));
        }

        let total_time = (result.completed_at - result.started_at)
            .num_milliseconds()
            .max(0) as u64;
        output.push_str(&format!(
            "  {} Total Time: {:.2}s\n",
            "•".cyan(),
            total_time as f64 / 1000.0
        ));

        output.push_str(&format!(
            "  {} Strategy: {:?}\n",
            "•".cyan(),
            result.strategy_used
        ));

        // Strategy selection reasoning (Auto mode)
        if let Some(reasoning) = &result.strategy_selection_reasoning {
            output.push_str(&format!(
                "  {} Strategy Selection: {}\n",
                "•".cyan(),
                reasoning.dimmed()
            ));
        }

        // Status
        let status_str = match result.status {
            BattalionStatus::Completed => format!("{} {}", "Completed".green(), "✓".green()),
            BattalionStatus::Failed => format!("{} {}", "Failed".red(), "✗".red()),
            BattalionStatus::Cancelled => format!("{} {}", "Cancelled".yellow(), "⚠".yellow()),
            _ => format!("{:?}", result.status),
        };
        output.push_str(&format!("  {} Status: {}\n", "•".cyan(), status_str));

        // Verbose mode: show individual Paladin results
        if verbose && !result.paladin_results.is_empty() {
            output.push_str(&format!(
                "\n{} Individual Paladin Results:\n",
                "→".cyan().bold()
            ));
            output.push_str(&"─".repeat(80));
            output.push('\n');

            for (idx, paladin_result) in result.paladin_results.iter().enumerate() {
                let timing = result
                    .per_paladin_times
                    .get(idx)
                    .copied()
                    .unwrap_or(paladin_result.execution_time_ms);

                output.push_str(&format!(
                    "\n{} Paladin {} - {} loops, {:.2}s, {} tokens\n",
                    format!("{}.", idx + 1).cyan().bold(),
                    idx + 1,
                    paladin_result.loop_count,
                    timing as f64 / 1000.0,
                    paladin_result.token_count
                ));

                let status = match &paladin_result.stop_reason {
                    StopReason::Completed => "✓".green(),
                    StopReason::StopWord(_) => "⚠".yellow(),
                    StopReason::MaxLoops => "⚠".yellow(),
                    StopReason::Timeout => "✗".red(),
                };
                output.push_str(&format!(
                    "   Status: {} {:?}\n",
                    status, paladin_result.stop_reason
                ));

                // Show first 200 chars of output
                let preview = if paladin_result.output.len() > 200 {
                    format!("{}...", &paladin_result.output[..200])
                } else {
                    paladin_result.output.clone()
                };
                output.push_str(&format!("   Output: {}\n", preview.dimmed()));
            }

            output.push_str(&"─".repeat(80));
            output.push('\n');
        }

        output.push_str(&"═".repeat(80));
        output.push('\n');

        output
    }

    /// Format a Battalion result as JSON for file output
    ///
    /// Includes comprehensive metadata, all individual Paladin results, and timing data.
    pub fn format_battalion_result_json(result: &BattalionResult) -> Value {
        json!({
            "battalion_id": result.battalion_id,
            "battalion_name": result.battalion_name,
            "started_at": result.started_at.to_rfc3339(),
            "completed_at": result.completed_at.to_rfc3339(),
            "total_time_ms": (result.completed_at - result.started_at).num_milliseconds().max(0),
            "final_output": result.final_output,
            "status": format!("{:?}", result.status),
            "strategy_used": format!("{:?}", result.strategy_used),
            "strategy_selection_reasoning": result.strategy_selection_reasoning,
            "strategy_selection_time_ms": result.strategy_selection_time_ms,
            "paladin_results": result.paladin_results.iter().enumerate().map(|(idx, r)| {
                let timing = result.per_paladin_times.get(idx).copied().unwrap_or(r.execution_time_ms);
                json!({
                    "index": idx,
                    "output": r.output,
                    "token_count": r.token_count,
                    "execution_time_ms": timing,
                    "loop_count": r.loop_count,
                    "stop_reason": format!("{:?}", r.stop_reason),
                    "is_successful": r.stop_reason.is_successful(),
                })
            }).collect::<Vec<_>>(),
            "summary": {
                "total_paladins": result.paladin_results.len(),
                "successful": result.paladin_success_count,
                "failed": result.paladin_failure_count,
            },
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_format_paladin_result_success() {
        let result = PaladinResult {
            output: "Test output".to_string(),
            token_count: 100,
            execution_time_ms: 1500,
            loop_count: 1,
            stop_reason: StopReason::Completed,
        };

        let formatted = OutputFormatter::format_paladin_result(&result, false);
        assert!(formatted.contains("Test output"));
        assert!(formatted.contains("100"));
        assert!(formatted.contains("1.50s"));
    }

    #[test]
    fn test_format_paladin_result_verbose() {
        let result = PaladinResult {
            output: "Test output".to_string(),
            token_count: 100,
            execution_time_ms: 1500,
            loop_count: 3,
            stop_reason: StopReason::MaxLoops,
        };

        let formatted = OutputFormatter::format_paladin_result(&result, true);
        assert!(formatted.contains("Reasoning Loops: 3"));
    }

    #[test]
    fn test_format_paladin_result_json() {
        let result = PaladinResult {
            output: "Test output".to_string(),
            token_count: 100,
            execution_time_ms: 1500,
            loop_count: 1,
            stop_reason: StopReason::Completed,
        };

        let json = OutputFormatter::format_paladin_result_json(&result);
        assert_eq!(json["output"], "Test output");
        assert_eq!(json["metadata"]["token_count"], 100);
        assert_eq!(json["metadata"]["loop_count"], 1);
    }

    #[test]
    fn test_format_battalion_result() {
        let paladin_results = vec![
            PaladinResult {
                output: "Result 1".to_string(),
                token_count: 50,
                execution_time_ms: 500,
                loop_count: 1,
                stop_reason: StopReason::Completed,
            },
            PaladinResult {
                output: "Result 2".to_string(),
                token_count: 75,
                execution_time_ms: 750,
                loop_count: 1,
                stop_reason: StopReason::Completed,
            },
        ];

        let started = Utc::now();
        let completed = started + chrono::Duration::milliseconds(2000);

        let result = BattalionResult {
            battalion_id: Uuid::new_v4(),
            battalion_name: "test_battalion".to_string(),
            started_at: started,
            completed_at: completed,
            final_output: "Final aggregated output".to_string(),
            paladin_results: paladin_results.clone(),
            status: BattalionStatus::Completed,
            strategy_used:
                crate::core::platform::container::battalion::BattalionStrategy::Formation,
            strategy_selection_reasoning: None,
            strategy_selection_time_ms: 0,
            per_paladin_times: vec![500, 750],
            paladin_success_count: 2,
            paladin_failure_count: 0,
        };

        let formatted = OutputFormatter::format_battalion_result(&result, false);
        assert!(formatted.contains("test_battalion"));
        assert!(formatted.contains("Final aggregated output"));
        assert!(formatted.contains("Successful: 2"));
    }

    #[test]
    fn test_format_battalion_result_verbose() {
        let paladin_results = vec![PaladinResult {
            output: "Result 1".to_string(),
            token_count: 50,
            execution_time_ms: 500,
            loop_count: 1,
            stop_reason: StopReason::Completed,
        }];

        let started = Utc::now();
        let completed = started + chrono::Duration::milliseconds(1000);

        let result = BattalionResult {
            battalion_id: Uuid::new_v4(),
            battalion_name: "test".to_string(),
            started_at: started,
            completed_at: completed,
            final_output: "Final".to_string(),
            paladin_results: paladin_results.clone(),
            status: BattalionStatus::Completed,
            strategy_used: crate::core::platform::container::battalion::BattalionStrategy::Phalanx,
            strategy_selection_reasoning: None,
            strategy_selection_time_ms: 0,
            per_paladin_times: vec![500],
            paladin_success_count: 1,
            paladin_failure_count: 0,
        };

        let formatted = OutputFormatter::format_battalion_result(&result, true);
        assert!(formatted.contains("Individual Paladin Results"));
        assert!(formatted.contains("Result 1"));
    }
}
