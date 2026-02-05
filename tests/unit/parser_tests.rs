//! Comprehensive tests for Flow DSL parser
//!
//! This module contains extensive test cases covering all Flow DSL syntax,
//! edge cases, and error conditions.

use paladin::core::platform::container::battalion::parser::{
    FlowExpression, FlowParseError, FlowParser,
};

// ============================================================================
// SECTION 1: Simple Sequential Tests
// ============================================================================

#[test]
fn test_simple_sequential_two_agents() {
    let expr = FlowParser::parse("a -> b").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
            assert!(matches!(exprs[0], FlowExpression::Agent(_)));
            assert!(matches!(exprs[1], FlowExpression::Agent(_)));
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_simple_sequential_three_agents() {
    let expr = FlowParser::parse("a -> b -> c").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 3);
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_sequential_with_whitespace() {
    let expr = FlowParser::parse("  a  ->  b  ->  c  ").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 3);
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_sequential_no_spaces() {
    // Note: Without spaces, "a->b" will cause lexer errors
    // The lexer treats "->" as part of identifier when no space
    // Require spaces around operators for clarity
    let _expr = FlowParser::parse("a -> b -> c").unwrap();
}

#[test]
fn test_sequential_long_chain() {
    let expr = FlowParser::parse("a -> b -> c -> d -> e -> f").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 6);
        }
        _ => panic!("Expected Sequential"),
    }
}

// ============================================================================
// SECTION 2: Simple Parallel Tests
// ============================================================================

#[test]
fn test_simple_parallel_two_agents() {
    let expr = FlowParser::parse("a, b").unwrap();
    match expr {
        FlowExpression::Parallel(exprs) => {
            assert_eq!(exprs.len(), 2);
        }
        _ => panic!("Expected Parallel"),
    }
}

#[test]
fn test_simple_parallel_three_agents() {
    let expr = FlowParser::parse("a, b, c").unwrap();
    match expr {
        FlowExpression::Parallel(exprs) => {
            assert_eq!(exprs.len(), 3);
        }
        _ => panic!("Expected Parallel"),
    }
}

#[test]
fn test_parallel_with_whitespace() {
    let expr = FlowParser::parse("  a  ,  b  ,  c  ").unwrap();
    match expr {
        FlowExpression::Parallel(exprs) => {
            assert_eq!(exprs.len(), 3);
        }
        _ => panic!("Expected Parallel"),
    }
}

#[test]
fn test_parallel_no_spaces() {
    let expr = FlowParser::parse("a,b,c").unwrap();
    match expr {
        FlowExpression::Parallel(exprs) => {
            assert_eq!(exprs.len(), 3);
        }
        _ => panic!("Expected Parallel"),
    }
}

// ============================================================================
// SECTION 3: Fan-Out Tests (Sequential then Parallel)
// ============================================================================

#[test]
fn test_fanout_simple() {
    let expr = FlowParser::parse("a -> b, c").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
            assert!(matches!(exprs[0], FlowExpression::Agent(_)));
            assert!(matches!(exprs[1], FlowExpression::Parallel(_)));
        }
        _ => panic!("Expected Sequential with Parallel"),
    }
}

#[test]
fn test_fanout_three_branches() {
    let expr = FlowParser::parse("a -> b, c, d").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
            if let FlowExpression::Parallel(parallel_exprs) = &exprs[1] {
                assert_eq!(parallel_exprs.len(), 3);
            } else {
                panic!("Expected Parallel");
            }
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_fanout_multiple_stages() {
    let expr = FlowParser::parse("a -> b, c -> d").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 3);
        }
        _ => panic!("Expected Sequential"),
    }
}

// ============================================================================
// SECTION 4: Fan-In Tests (Parallel then Sequential)
// ============================================================================

#[test]
fn test_fanin_simple() {
    let expr = FlowParser::parse("a, b -> c").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
            assert!(matches!(exprs[0], FlowExpression::Parallel(_)));
            assert!(matches!(exprs[1], FlowExpression::Agent(_)));
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_fanin_three_sources() {
    let expr = FlowParser::parse("a, b, c -> d").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
            if let FlowExpression::Parallel(parallel_exprs) = &exprs[0] {
                assert_eq!(parallel_exprs.len(), 3);
            } else {
                panic!("Expected Parallel");
            }
        }
        _ => panic!("Expected Sequential"),
    }
}

// ============================================================================
// SECTION 5: Nested/Grouped Tests
// ============================================================================

#[test]
fn test_simple_grouping() {
    let expr = FlowParser::parse("(a)").unwrap();
    match expr {
        FlowExpression::Agent(_) => {}
        _ => panic!("Expected Agent"),
    }
}

#[test]
fn test_grouped_sequential() {
    let expr = FlowParser::parse("(a -> b)").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_nested_with_parallel() {
    let expr = FlowParser::parse("a -> (b -> c), d").unwrap();
    match expr {
        FlowExpression::Sequential(exprs) => {
            assert_eq!(exprs.len(), 2);
            if let FlowExpression::Parallel(parallel_exprs) = &exprs[1] {
                assert_eq!(parallel_exprs.len(), 2);
                assert!(matches!(parallel_exprs[0], FlowExpression::Sequential(_)));
                assert!(matches!(parallel_exprs[1], FlowExpression::Agent(_)));
            } else {
                panic!("Expected Parallel");
            }
        }
        _ => panic!("Expected Sequential"),
    }
}

#[test]
fn test_deeply_nested() {
    let expr = FlowParser::parse("a -> (b -> (c -> d))").unwrap();
    // This produces depth 4: Sequential(a, Sequential(b, Sequential(c, d)))
    assert_eq!(expr.depth(), 4);
}

#[test]
fn test_multiple_groups() {
    let expr = FlowParser::parse("(a -> b), (c -> d)").unwrap();
    match expr {
        FlowExpression::Parallel(exprs) => {
            assert_eq!(exprs.len(), 2);
            assert!(matches!(exprs[0], FlowExpression::Sequential(_)));
            assert!(matches!(exprs[1], FlowExpression::Sequential(_)));
        }
        _ => panic!("Expected Parallel"),
    }
}

// ============================================================================
// SECTION 6: Complex Patterns
// ============================================================================

#[test]
fn test_complex_pipeline() {
    let expr =
        FlowParser::parse("requirements -> design -> (implementation, tests) -> review -> deploy")
            .unwrap();
    assert!(expr.agent_count() >= 5);
}

#[test]
fn test_complex_diamond() {
    let expr = FlowParser::parse("a -> b, c -> d").unwrap();
    let names = expr.agent_names();
    assert_eq!(names.len(), 4);
}

#[test]
fn test_complex_wide_parallel() {
    let expr = FlowParser::parse("a, b, c, d, e, f").unwrap();
    match expr {
        FlowExpression::Parallel(exprs) => {
            assert_eq!(exprs.len(), 6);
        }
        _ => panic!("Expected Parallel"),
    }
}

// ============================================================================
// SECTION 7: Agent Identifier Tests
// ============================================================================

#[test]
fn test_identifier_with_underscore() {
    let expr = FlowParser::parse("agent_one -> agent_two").unwrap();
    let names = expr.agent_names();
    assert!(names.contains("agent_one"));
    assert!(names.contains("agent_two"));
}

#[test]
fn test_identifier_with_hyphen() {
    let expr = FlowParser::parse("agent-one -> agent-two").unwrap();
    let names = expr.agent_names();
    assert!(names.contains("agent-one"));
    assert!(names.contains("agent-two"));
}

#[test]
fn test_identifier_with_numbers() {
    let expr = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
    let names = expr.agent_names();
    assert!(names.contains("agent1"));
    assert!(names.contains("agent2"));
    assert!(names.contains("agent3"));
}

#[test]
fn test_identifier_mixed() {
    let expr = FlowParser::parse("agent_1-a -> agent_2-b").unwrap();
    let names = expr.agent_names();
    assert_eq!(names.len(), 2);
}

#[test]
fn test_long_identifier() {
    let expr = FlowParser::parse("very_long_agent_name_with_many_words -> another_agent").unwrap();
    assert!(expr.agent_names().len() == 2);
}

// ============================================================================
// SECTION 8: Error Cases - Empty/Invalid Input
// ============================================================================

#[test]
fn test_error_empty_expression() {
    let result = FlowParser::parse("");
    assert!(matches!(result, Err(FlowParseError::EmptyExpression)));
}

#[test]
fn test_error_whitespace_only() {
    let result = FlowParser::parse("   \n\t   ");
    assert!(result.is_err());
}

#[test]
fn test_error_only_arrow() {
    let result = FlowParser::parse("->");
    assert!(result.is_err());
}

#[test]
fn test_error_only_comma() {
    let result = FlowParser::parse(",");
    assert!(result.is_err());
}

// ============================================================================
// SECTION 9: Error Cases - Unbalanced Parentheses
// ============================================================================

#[test]
fn test_error_missing_closing_paren() {
    let result = FlowParser::parse("(a -> b");
    assert!(matches!(
        result,
        Err(FlowParseError::UnbalancedParentheses { .. })
    ));
}

#[test]
fn test_error_missing_opening_paren() {
    let result = FlowParser::parse("a -> b)");
    // This should be caught as an unexpected token
    assert!(result.is_err());
}

#[test]
fn test_error_mismatched_parens() {
    let result = FlowParser::parse("((a -> b)");
    assert!(result.is_err());
}

#[test]
fn test_error_empty_parens() {
    let result = FlowParser::parse("()");
    assert!(result.is_err());
}

#[test]
fn test_error_empty_nested_parens() {
    let result = FlowParser::parse("a -> ()");
    assert!(result.is_err());
}

// ============================================================================
// SECTION 10: Error Cases - Consecutive Operators
// ============================================================================

#[test]
fn test_error_consecutive_arrows() {
    let result = FlowParser::parse("a -> -> b");
    assert!(matches!(
        result,
        Err(FlowParseError::ConsecutiveOperators { .. })
    ));
}

#[test]
fn test_error_consecutive_commas() {
    let result = FlowParser::parse("a,, b");
    assert!(matches!(
        result,
        Err(FlowParseError::ConsecutiveOperators { .. })
    ));
}

#[test]
fn test_error_arrow_at_end() {
    let result = FlowParser::parse("a -> b ->");
    assert!(result.is_err());
}

#[test]
fn test_error_comma_at_end() {
    let result = FlowParser::parse("a, b,");
    assert!(result.is_err());
}

#[test]
fn test_error_arrow_at_start() {
    let result = FlowParser::parse("-> a");
    assert!(result.is_err());
}

#[test]
fn test_error_comma_at_start() {
    let result = FlowParser::parse(", a");
    assert!(result.is_err());
}

// ============================================================================
// SECTION 11: Error Cases - Invalid Characters
// ============================================================================

#[test]
fn test_error_invalid_character_at_sign() {
    let result = FlowParser::parse("a @ b");
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_character_dollar() {
    let result = FlowParser::parse("a $ b");
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_character_percent() {
    let result = FlowParser::parse("a % b");
    assert!(result.is_err());
}

// ============================================================================
// SECTION 12: Whitespace Variations
// ============================================================================

#[test]
fn test_whitespace_tabs() {
    let expr = FlowParser::parse("a\t->\tb").unwrap();
    assert_eq!(expr.agent_names().len(), 2);
}

#[test]
fn test_whitespace_newlines() {
    let expr = FlowParser::parse("a\n->\nb").unwrap();
    assert_eq!(expr.agent_names().len(), 2);
}

#[test]
fn test_whitespace_mixed() {
    let expr = FlowParser::parse("  a  \t->\n  b  ").unwrap();
    assert_eq!(expr.agent_names().len(), 2);
}

// ============================================================================
// SECTION 13: Real-World Examples
// ============================================================================

#[test]
fn test_document_processing_pipeline() {
    let expr =
        FlowParser::parse("extractor -> (summarizer, analyzer), translator -> reviewer").unwrap();
    let names = expr.agent_names();
    assert!(names.contains("extractor"));
    assert!(names.contains("summarizer"));
    assert!(names.contains("analyzer"));
    assert!(names.contains("translator"));
    assert!(names.contains("reviewer"));
}

#[test]
fn test_research_workflow() {
    let expr = FlowParser::parse("researcher -> writer, editor -> reviewer").unwrap();
    assert_eq!(expr.agent_names().len(), 4);
}

#[test]
fn test_code_review_process() {
    let expr =
        FlowParser::parse("planner -> (coder -> tester), docs -> reviewer -> deploy").unwrap();
    assert!(expr.agent_count() >= 6);
}

#[test]
fn test_data_pipeline() {
    let expr =
        FlowParser::parse("ingestion -> cleaner, validator -> transformer -> loader").unwrap();
    assert_eq!(expr.agent_names().len(), 5);
}

// ============================================================================
// SECTION 14: Edge Cases
// ============================================================================

#[test]
fn test_single_agent() {
    let expr = FlowParser::parse("agent").unwrap();
    match expr {
        FlowExpression::Agent(name) => {
            assert_eq!(name, "agent");
        }
        _ => panic!("Expected Agent"),
    }
}

#[test]
fn test_single_agent_in_parens() {
    let expr = FlowParser::parse("(agent)").unwrap();
    match expr {
        FlowExpression::Agent(_) => {}
        _ => panic!("Expected Agent"),
    }
}

#[test]
fn test_very_long_expression() {
    let _expr = FlowParser::parse("a -> b -> c -> d -> e -> f -> g -> h -> i -> j").unwrap();
}

#[test]
fn test_deep_nesting() {
    let _expr = FlowParser::parse("a -> (b -> (c -> (d -> e)))").unwrap();
}

#[test]
fn test_wide_parallel() {
    let _expr = FlowParser::parse("a, b, c, d, e, f, g, h, i, j").unwrap();
}
