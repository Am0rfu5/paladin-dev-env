# Task List: Flow DSL & Agent Rearrangement (Epic 17)

**Epic:** Epic 17 - Flow DSL & Agent Rearrangement  
**Feature:** Maneuver Pattern - String-based workflow orchestration  
**PRD:** `prd-flow-dsl-agent-rearrangement.md`  
**Branch:** `feature/epic-16-flow-dsl-agent-rearrangement`  
**Estimated Duration:** 2 weeks  

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Testing Protocol:**
1. After finishing all sub-tasks for a parent task, run `cargo test`
2. Check formatting: `cargo fmt --check`
3. Run linter: `cargo clippy`
4. Only commit if all checks pass
5. Use conventional commit format

---

## Relevant Files

### Core Layer (Domain Models)
- `src/core/platform/container/battalion/parser/mod.rs` - Main parser module and exports
- `src/core/platform/container/battalion/parser/lexer.rs` - Token generation from flow strings
- `src/core/platform/container/battalion/parser/ast.rs` - FlowExpression AST definition
- `src/core/platform/container/battalion/parser/error.rs` - FlowParseError and error types
- `src/core/platform/container/battalion/maneuver.rs` - Maneuver domain model and config
- `src/core/platform/container/battalion/mod.rs` - Battalion module exports (update)

### Application Layer (Use Cases)
- `src/application/use_cases/battalion/maneuver_service.rs` - ManeuverExecutionService implementation
- `src/application/use_cases/battalion/flow_visualizer.rs` - ASCII and Mermaid visualization
- `src/application/use_cases/battalion/commander.rs` - Update with Maneuver strategy
- `src/application/use_cases/battalion/mod.rs` - Battalion use cases exports (update)

### Tests
- `tests/unit/parser_tests.rs` - Unit tests for lexer and parser
- `tests/unit/maneuver_domain_tests.rs` - Unit tests for domain models
- `tests/integration/maneuver_execution_tests.rs` - Integration tests for execution
- `tests/integration/maneuver_commander_tests.rs` - Integration tests for Commander
- `tests/integration/flow_visualizer_tests.rs` - Tests for visualization

### Documentation
- `docs/guides/flow-dsl.md` - User guide for Flow DSL syntax and usage
- `docs/BATTALION.md` - Update with Maneuver pattern documentation

### Examples
- `examples/maneuver_workflow.rs` - Basic Maneuver usage example
- `examples/maneuver_complex.rs` - Complex nested patterns example
- `examples/maneuver_error_handling.rs` - Error strategy examples
- `examples/cli_configs/maneuver.yaml` - YAML configuration template

### Notes
- Follow hexagonal architecture: Core → Application → Infrastructure
- Parser should be in core layer (pure business logic)
- Use TDD: Write tests first for each component
- All error types use `thiserror` for consistent error handling
- Run `cargo test` after each major component completion
- Use `cargo clippy` to catch warnings before committing

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Branch already created: `feature/epic-16-flow-dsl-agent-rearrangement`

- [x] 1.0 Implement Flow DSL Parser (US-17.1)
  - [x] 1.1 Create parser module structure
    - [x] 1.1.1 Create `src/core/platform/container/battalion/parser/` directory
    - [x] 1.1.2 Create `parser/mod.rs` with module declarations
    - [x] 1.1.3 Create placeholder files: `lexer.rs`, `ast.rs`, `error.rs`
  - [x] 1.2 Define FlowParseError types
    - [x] 1.2.1 Create error enum in `parser/error.rs` with variants: UnexpectedToken, UnbalancedParentheses, EmptyExpression, InvalidIdentifier, ConsecutiveOperators
    - [x] 1.2.2 Implement `thiserror` derives with helpful messages
    - [x] 1.2.3 Add position tracking (line, column) to errors
    - [x] 1.2.4 Add suggestion methods for common mistakes
    - [x] 1.2.5 Write unit tests for error formatting
  - [x] 1.3 Implement FlowExpression AST
    - [x] 1.3.1 Define `FlowExpression` enum in `parser/ast.rs`: Agent(String), Sequential(Vec<FlowExpression>), Parallel(Vec<FlowExpression>)
    - [x] 1.3.2 Implement Debug, Clone, PartialEq for FlowExpression
    - [x] 1.3.3 Add Serialize/Deserialize derives from serde
    - [x] 1.3.4 Add helper methods: `agent_names()`, `depth()`, `width()`
    - [x] 1.3.5 Write unit tests for AST construction and traversal
  - [x] 1.4 Implement Lexer
    - [x] 1.4.1 Define Token enum in `parser/lexer.rs`: Arrow, Comma, LParen, RParen, Agent(String)
    - [x] 1.4.2 Implement `Lexer` struct with input string and position tracking
    - [x] 1.4.3 Implement `next_token()` method with whitespace handling
    - [x] 1.4.4 Validate agent identifiers (alphanumeric, underscore, hyphen)
    - [x] 1.4.5 Write unit tests for all token types and edge cases
  - [x] 1.5 Implement Parser
    - [x] 1.5.1 Define `FlowParser` struct in `parser/mod.rs`
    - [x] 1.5.2 Implement `parse(expression: &str) -> Result<FlowExpression, FlowParseError>` method
    - [x] 1.5.3 Implement recursive descent parser for expressions
    - [x] 1.5.4 Handle operator precedence (sequential has lower precedence than parallel)
    - [x] 1.5.5 Implement parentheses grouping
    - [x] 1.5.6 Add validation for balanced parentheses
    - [x] 1.5.7 Add validation for consecutive operators
    - [x] 1.5.8 Add validation for empty groups
  - [x] 1.6 Write comprehensive parser tests
    - [x] 1.6.1 Test simple sequential: `"a -> b"`
    - [x] 1.6.2 Test simple parallel: `"a, b"`
    - [x] 1.6.3 Test fan-out: `"a -> b, c"`
    - [x] 1.6.4 Test fan-in: `"a, b -> c"`
    - [x] 1.6.5 Test nested: `"a -> (b -> c), d"`
    - [x] 1.6.6 Test complex chains: `"a -> b -> c -> d"`
    - [x] 1.6.7 Test error cases: unbalanced parens, consecutive operators, empty expressions
    - [x] 1.6.8 Test edge cases: whitespace variations, long agent names
    - [x] 1.6.9 Add 50+ test cases covering all syntax

- [x] 2.0 Implement Maneuver Domain Model (US-17.2)
  - [x] 2.1 Define ManeuverConfig struct
    - [x] 2.1.1 Create `ErrorStrategy` enum in `maneuver.rs`: FailFast, ContinueParallel, IgnoreErrors
    - [x] 2.1.2 Create `OutputFormat` enum: Concatenate, JsonArray
    - [x] 2.1.3 Define `ManeuverConfig` struct with all fields from PRD
    - [x] 2.1.4 Implement Default trait with sensible defaults
    - [x] 2.1.5 Add Serialize/Deserialize for YAML support
    - [x] 2.1.6 Add validation methods for config values
  - [x] 2.2 Define Maneuver struct
    - [x] 2.2.1 Create `Maneuver` struct with name, agents HashMap, flow, config
    - [x] 2.2.2 Implement validation that all flow agents exist in agents map
    - [x] 2.2.3 Implement validation for max depth (5 levels) and max agents (30)
    - [x] 2.2.4 Add builder methods or constructor
    - [x] 2.2.5 Implement Debug, Clone for Maneuver
  - [x] 2.3 Define ManeuverResult struct
    - [x] 2.3.1 Create `ManeuverResult` with final_output, step_outputs, execution_order
    - [x] 2.3.2 Add optional timing_metrics HashMap<String, Duration>
    - [x] 2.3.3 Add status enum: Success, PartialSuccess, Failed
    - [x] 2.3.4 Implement helper methods: `get_agent_output()`, `total_duration()`
  - [x] 2.4 Define ManeuverError types
    - [x] 2.4.1 Create `ManeuverError` enum using thiserror
    - [x] 2.4.2 Add variants: ParseError, ValidationError, ExecutionError, AgentNotFound, TimeoutError
    - [x] 2.4.3 Implement From<FlowParseError> for ManeuverError
    - [x] 2.4.4 Implement From<PaladinError> for ManeuverError
  - [x] 2.5 Write domain model tests
    - [x] 2.5.1 Test Maneuver construction with valid flow
    - [x] 2.5.2 Test validation catches missing agents
    - [x] 2.5.3 Test validation catches excessive depth
    - [x] 2.5.4 Test validation catches excessive agent count
    - [x] 2.5.5 Test ManeuverConfig defaults
    - [x] 2.5.6 Test serialization/deserialization
  - [x] 2.6 Update Battalion module exports
    - [x] 2.6.1 Add maneuver module to `src/core/platform/container/battalion/mod.rs`
    - [x] 2.6.2 Export Maneuver, ManeuverConfig, ManeuverResult, ManeuverError
    - [x] 2.6.3 Update documentation comments

- [ ] 3.0 Implement Maneuver Execution Service (US-17.3)
  - [ ] 3.1 Create ManeuverExecutionService struct
    - [ ] 3.1.1 Create `src/application/use_cases/battalion/maneuver_service.rs`
    - [ ] 3.1.2 Define struct with paladin_port: Arc<dyn PaladinPort>
    - [ ] 3.1.3 Implement new() constructor
    - [ ] 3.1.4 Add configuration fields if needed
  - [ ] 3.2 Implement main execution method
    - [ ] 3.2.1 Implement `execute(&self, maneuver: &Maneuver, input: &str) -> Result<ManeuverResult, ManeuverError>`
    - [ ] 3.2.2 Set up execution context with timing tracking
    - [ ] 3.2.3 Call recursive expression executor
    - [ ] 3.2.4 Build and return ManeuverResult
    - [ ] 3.2.5 Handle overall timeout
  - [ ] 3.3 Implement sequential execution
    - [ ] 3.3.1 Create `execute_sequential()` method
    - [ ] 3.3.2 Execute agents in order
    - [ ] 3.3.3 Handle output passing based on config.pass_output_as_input
    - [ ] 3.3.4 Respect ErrorStrategy for failures
    - [ ] 3.3.5 Track execution order and timing
  - [ ] 3.4 Implement parallel execution
    - [ ] 3.4.1 Create `execute_parallel()` method
    - [ ] 3.4.2 Use tokio::spawn or join_all for concurrent execution
    - [ ] 3.4.3 Wait for all tasks with timeout
    - [ ] 3.4.4 Aggregate results based on config.output_format
    - [ ] 3.4.5 Handle partial failures based on ErrorStrategy
  - [ ] 3.5 Implement recursive expression evaluator
    - [ ] 3.5.1 Create `execute_expression()` method
    - [ ] 3.5.2 Match on FlowExpression variants
    - [ ] 3.5.3 Handle Agent variant: lookup and execute
    - [ ] 3.5.4 Handle Sequential variant: call execute_sequential
    - [ ] 3.5.5 Handle Parallel variant: call execute_parallel
    - [ ] 3.5.6 Maintain execution context through recursion
  - [ ] 3.6 Implement error handling strategies
    - [ ] 3.6.1 Implement FailFast: stop on first error
    - [ ] 3.6.2 Implement ContinueParallel: continue parallel branches, fail sequences
    - [ ] 3.6.3 Implement IgnoreErrors: log but continue
    - [ ] 3.6.4 Add detailed error context (agent name, step, suggestion)
  - [ ] 3.7 Implement timing metrics
    - [ ] 3.7.1 Add timing capture if config.collect_timing_metrics
    - [ ] 3.7.2 Track per-agent execution time
    - [ ] 3.7.3 Track total workflow time
    - [ ] 3.7.4 Include timing in ManeuverResult
  - [ ] 3.8 Write execution service tests
    - [ ] 3.8.1 Create mock PaladinPort implementation
    - [ ] 3.8.2 Test simple sequential execution
    - [ ] 3.8.3 Test simple parallel execution
    - [ ] 3.8.4 Test fan-out pattern
    - [ ] 3.8.5 Test fan-in pattern with output aggregation
    - [ ] 3.8.6 Test nested expressions
    - [ ] 3.8.7 Test each error strategy behavior
    - [ ] 3.8.8 Test timeout handling
    - [ ] 3.8.9 Test timing metrics collection

- [ ] 4.0 Integrate with Commander (US-17.4)
  - [ ] 4.1 Add Maneuver variant to BattalionStrategy
    - [ ] 4.1.1 Read current BattalionStrategy enum definition
    - [ ] 4.1.2 Add `Maneuver` variant with appropriate fields
    - [ ] 4.1.3 Update all match statements handling BattalionStrategy
    - [ ] 4.1.4 Ensure serialization works for YAML configs
  - [ ] 4.2 Extend CommanderBuilder
    - [ ] 4.2.1 Add `flow(expression: &str) -> Self` method
    - [ ] 4.2.2 Add `error_strategy(strategy: ErrorStrategy) -> Self` method
    - [ ] 4.2.3 Add `maneuver_config(config: ManeuverConfig) -> Self` method
    - [ ] 4.2.4 Update build() validation to handle Maneuver
    - [ ] 4.2.5 Ensure flow references are validated against registered agents
  - [ ] 4.3 Update Commander execution
    - [ ] 4.3.1 Read current Commander execute() implementation
    - [ ] 4.3.2 Add match arm for BattalionStrategy::Maneuver
    - [ ] 4.3.3 Construct Maneuver instance from strategy
    - [ ] 4.3.4 Call ManeuverExecutionService
    - [ ] 4.3.5 Convert ManeuverResult to CommanderResult
  - [ ] 4.4 Ensure auto-strategy does NOT select Maneuver
    - [ ] 4.4.1 Review auto-strategy selection logic
    - [ ] 4.4.2 Confirm Maneuver is explicit-only (not in auto selection)
    - [ ] 4.4.3 Add comments explaining why Maneuver is explicit
  - [ ] 4.5 Write Commander integration tests
    - [ ] 4.5.1 Test CommanderBuilder with flow expression
    - [ ] 4.5.2 Test execution through Commander API
    - [ ] 4.5.3 Test validation of flow agent references
    - [ ] 4.5.4 Test error strategy configuration
    - [ ] 4.5.5 Test auto-strategy does not select Maneuver

- [ ] 5.0 Implement Flow Visualization (US-17.5)
  - [ ] 5.1 Create FlowVisualizer struct
    - [ ] 5.1.1 Create `src/application/use_cases/battalion/flow_visualizer.rs`
    - [ ] 5.1.2 Define FlowVisualizer struct
    - [ ] 5.1.3 Add configuration options (indentation, box chars, etc.)
  - [ ] 5.2 Implement ASCII visualization
    - [ ] 5.2.1 Implement `to_ascii(expr: &FlowExpression) -> String`
    - [ ] 5.2.2 Handle Agent nodes: simple box with agent name
    - [ ] 5.2.3 Handle Sequential: vertical layout with arrows
    - [ ] 5.2.4 Handle Parallel: branching layout
    - [ ] 5.2.5 Use box-drawing characters for clean output
    - [ ] 5.2.6 Add indentation for nested structures
  - [ ] 5.3 Implement Mermaid visualization
    - [ ] 5.3.1 Implement `to_mermaid(expr: &FlowExpression) -> String`
    - [ ] 5.3.2 Generate valid Mermaid.js flowchart syntax
    - [ ] 5.3.3 Use appropriate node shapes and connectors
    - [ ] 5.3.4 Handle subgraphs for parallel execution
    - [ ] 5.3.5 Add styling hints (optional)
  - [ ] 5.4 Add timing metrics overlay
    - [ ] 5.4.1 Implement `with_timing(expr: &FlowExpression, metrics: &HashMap<String, Duration>) -> String`
    - [ ] 5.4.2 Overlay execution time next to agent names
    - [ ] 5.4.3 Highlight bottlenecks (slowest agent)
    - [ ] 5.4.4 Show total workflow time
    - [ ] 5.4.5 Format durations human-readably (ms, s)
  - [ ] 5.5 Write visualization tests
    - [ ] 5.5.1 Test ASCII output for simple sequential
    - [ ] 5.5.2 Test ASCII output for parallel
    - [ ] 5.5.3 Test ASCII output for nested
    - [ ] 5.5.4 Test Mermaid output for various patterns
    - [ ] 5.5.5 Test timing overlay
    - [ ] 5.5.6 Verify output is valid (parseable by Mermaid)

- [ ] 6.0 Add CLI Support
  - [ ] 6.1 Add battalion run command support
    - [ ] 6.1.1 Locate CLI battalion commands module
    - [ ] 6.1.2 Add `--type maneuver` option to run command
    - [ ] 6.1.3 Add `--flow <expression>` flag
    - [ ] 6.1.4 Parse flow from command line or YAML config
    - [ ] 6.1.5 Pass through to Commander with Maneuver strategy
  - [ ] 6.2 Add visualize command
    - [ ] 6.2.1 Add new `paladin battalion visualize` subcommand
    - [ ] 6.2.2 Add `--flow <expression>` required argument
    - [ ] 6.2.3 Add `--format <ascii|mermaid>` option (default: ascii)
    - [ ] 6.2.4 Add `--output <file>` option to save to file
    - [ ] 6.2.5 Call FlowVisualizer and print/save result
  - [ ] 6.3 Add template generation
    - [ ] 6.3.1 Add `paladin battalion new --type maneuver` support
    - [ ] 6.3.2 Generate YAML template with example flow
    - [ ] 6.3.3 Include comments explaining syntax
    - [ ] 6.3.4 Include all config options with defaults
  - [ ] 6.4 Update YAML schema
    - [ ] 6.4.1 Define YAML structure for Maneuver in config
    - [ ] 6.4.2 Add flow field parsing
    - [ ] 6.4.3 Add error_strategy field parsing
    - [ ] 6.4.4 Add all ManeuverConfig fields
    - [ ] 6.4.5 Add validation for YAML structure
  - [ ] 6.5 Write CLI integration tests
    - [ ] 6.5.1 Test run command with inline flow
    - [ ] 6.5.2 Test run command with YAML config
    - [ ] 6.5.3 Test visualize command ASCII output
    - [ ] 6.5.4 Test visualize command Mermaid output
    - [ ] 6.5.5 Test template generation

- [ ] 7.0 Write Documentation
  - [ ] 7.1 Create Flow DSL user guide
    - [ ] 7.1.1 Create `docs/guides/flow-dsl.md`
    - [ ] 7.1.2 Write introduction and motivation section
    - [ ] 7.1.3 Document complete syntax reference with examples
    - [ ] 7.1.4 Explain operator precedence and grouping
    - [ ] 7.1.5 Document error handling strategies
    - [ ] 7.1.6 Add visualization guide
    - [ ] 7.1.7 Add best practices section
    - [ ] 7.1.8 Add troubleshooting common issues
    - [ ] 7.1.9 Add performance considerations
  - [ ] 7.2 Update Battalion documentation
    - [ ] 7.2.1 Open `docs/BATTALION.md`
    - [ ] 7.2.2 Add Maneuver pattern section
    - [ ] 7.2.3 Compare with other patterns (Formation, Phalanx, Campaign)
    - [ ] 7.2.4 Add when to use Maneuver guidance
    - [ ] 7.2.5 Link to flow-dsl.md guide
  - [ ] 7.3 Write API documentation
    - [ ] 7.3.1 Add comprehensive rustdoc to FlowParser
    - [ ] 7.3.2 Add rustdoc examples to FlowExpression
    - [ ] 7.3.3 Add rustdoc to Maneuver and ManeuverConfig
    - [ ] 7.3.4 Add rustdoc to ManeuverExecutionService
    - [ ] 7.3.5 Add rustdoc to FlowVisualizer
    - [ ] 7.3.6 Add doc tests that run as part of `cargo test`
  - [ ] 7.4 Update README and CHANGELOG
    - [ ] 7.4.1 Update main README.md with Maneuver feature
    - [ ] 7.4.2 Add to CHANGELOG.md under appropriate version
    - [ ] 7.4.3 Update feature list and capabilities

- [ ] 8.0 Create Examples
  - [ ] 8.1 Create basic Maneuver example
    - [ ] 8.1.1 Create `examples/maneuver_workflow.rs`
    - [ ] 8.1.2 Show simple sequential workflow
    - [ ] 8.1.3 Show parallel execution
    - [ ] 8.1.4 Show basic error handling
    - [ ] 8.1.5 Add comments explaining each step
    - [ ] 8.1.6 Ensure example compiles and runs successfully
  - [ ] 8.2 Create complex nested example
    - [ ] 8.2.1 Create `examples/maneuver_complex.rs`
    - [ ] 8.2.2 Show fan-out and fan-in patterns
    - [ ] 8.2.3 Show nested grouping with parentheses
    - [ ] 8.2.4 Demonstrate realistic use case (e.g., document processing pipeline)
    - [ ] 8.2.5 Add extensive comments
  - [ ] 8.3 Create error handling example
    - [ ] 8.3.1 Create `examples/maneuver_error_handling.rs`
    - [ ] 8.3.2 Demonstrate FailFast strategy
    - [ ] 8.3.3 Demonstrate ContinueParallel strategy
    - [ ] 8.3.4 Demonstrate IgnoreErrors strategy
    - [ ] 8.3.5 Show how to inspect partial results
    - [ ] 8.3.6 Show timing metrics usage
  - [ ] 8.4 Create YAML configuration example
    - [ ] 8.4.1 Create `examples/cli_configs/maneuver.yaml`
    - [ ] 8.4.2 Include complete configuration template
    - [ ] 8.4.3 Show all available options with comments
    - [ ] 8.4.4 Provide realistic example workflow
    - [ ] 8.4.5 Include multiple agents with different configs
  - [ ] 8.5 Test all examples
    - [ ] 8.5.1 Run `cargo run --example maneuver_workflow`
    - [ ] 8.5.2 Run `cargo run --example maneuver_complex`
    - [ ] 8.5.3 Run `cargo run --example maneuver_error_handling`
    - [ ] 8.5.4 Verify output is correct and helpful
    - [ ] 8.5.5 Test YAML config with CLI

- [ ] 9.0 Testing and Quality Assurance
  - [ ] 9.1 Run comprehensive test suite
    - [ ] 9.1.1 Run `cargo test` for all unit tests
    - [ ] 9.1.2 Run `cargo test --test integration` for integration tests
    - [ ] 9.1.3 Verify all tests pass
    - [ ] 9.1.4 Check test coverage with `cargo tarpaulin` (target: ≥80%)
  - [ ] 9.2 Performance testing
    - [ ] 9.2.1 Create benchmark for parser with 30+ agent flows
    - [ ] 9.2.2 Verify parsing < 1ms for 99% of flows
    - [ ] 9.2.3 Create benchmark for execution overhead
    - [ ] 9.2.4 Verify orchestration overhead < 2% of total time
    - [ ] 9.2.5 Test 30-agent workflow memory usage (< 100MB)
  - [ ] 9.3 Error handling validation
    - [ ] 9.3.1 Test all error paths have helpful messages
    - [ ] 9.3.2 Verify no panics from malformed input
    - [ ] 9.3.3 Test boundary conditions (max depth, max agents)
    - [ ] 9.3.4 Test timeout behavior
    - [ ] 9.3.5 Verify graceful degradation
  - [ ] 9.4 Code quality checks
    - [ ] 9.4.1 Run `cargo clippy` and fix all warnings
    - [ ] 9.4.2 Run `cargo fmt --check` and format if needed
    - [ ] 9.4.3 Run `cargo audit` for security vulnerabilities
    - [ ] 9.4.4 Review all TODOs and FIXMEs
    - [ ] 9.4.5 Ensure all public APIs have rustdoc
  - [ ] 9.5 Integration testing
    - [ ] 9.5.1 Test with real LLM providers (OpenAI)
    - [ ] 9.5.2 Test with existing Battalion patterns
    - [ ] 9.5.3 Test CLI commands end-to-end
    - [ ] 9.5.4 Test YAML configuration loading
    - [ ] 9.5.5 Test visualization output validity

- [ ] 10.0 Final Integration and Cleanup
  - [ ] 10.1 Code review and refactoring
    - [ ] 10.1.1 Review all new code for clarity and maintainability
    - [ ] 10.1.2 Refactor any duplicated code
    - [ ] 10.1.3 Ensure consistent error handling patterns
    - [ ] 10.1.4 Verify hexagonal architecture boundaries maintained
    - [ ] 10.1.5 Remove debug prints and temporary code
  - [ ] 10.2 Update project configuration
    - [ ] 10.2.1 Update `config.yml` with maneuver defaults
    - [ ] 10.2.2 Update `Cargo.toml` if new dependencies added
    - [ ] 10.2.3 Update feature flags if applicable
  - [ ] 10.3 Final testing
    - [ ] 10.3.1 Run full test suite: `make test-all`
    - [ ] 10.3.2 Run clean code checks: `make clean-code`
    - [ ] 10.3.3 Build release: `cargo build --release`
    - [ ] 10.3.4 Verify no warnings or errors
  - [ ] 10.4 Documentation review
    - [ ] 10.4.1 Review all documentation for accuracy
    - [ ] 10.4.2 Check all links work
    - [ ] 10.4.3 Verify examples compile and run
    - [ ] 10.4.4 Proofread for typos and clarity
  - [ ] 10.5 Commit and prepare for PR
    - [ ] 10.5.1 Stage all changes: `git add .`
    - [ ] 10.5.2 Commit with message: `git commit -m "feat: implement Epic 17 - Flow DSL & Maneuver pattern" -m "- Add flow DSL parser with lexer, AST, error handling" -m "- Implement Maneuver domain model and execution service" -m "- Integrate with Commander and CLI" -m "- Add ASCII and Mermaid visualization" -m "- Complete documentation and examples" -m "- All tests passing, coverage ≥80%"`
    - [ ] 10.5.3 Push to remote: `git push origin feature/epic-16-flow-dsl-agent-rearrangement`
    - [ ] 10.5.4 Create pull request with Epic 17 description
    - [ ] 10.5.5 Request code review

---

## Completion Checklist

Before marking Epic 17 as complete, verify:

- [ ] All 5 user stories (US-17.1 through US-17.5) acceptance criteria met
- [ ] Flow DSL parser handles all specified syntax from PRD
- [ ] Maneuver execution supports 10-30 agent workflows
- [ ] All three error strategies (FailFast, ContinueParallel, IgnoreErrors) implemented and tested
- [ ] Commander integration complete with CLI support
- [ ] Visualization working for both ASCII and Mermaid formats
- [ ] Test coverage ≥ 80% for new code
- [ ] All public APIs have rustdoc with examples
- [ ] Documentation complete in `docs/guides/flow-dsl.md`
- [ ] At least 3 working examples in `/examples`
- [ ] Performance benchmarks passing (parse < 1ms, overhead < 2%)
- [ ] Zero clippy warnings
- [ ] All code formatted with `cargo fmt`
- [ ] Integration tests passing in CI

---
