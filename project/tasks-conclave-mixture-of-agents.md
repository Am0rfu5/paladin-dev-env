# Task List: Conclave - MixtureOfAgents Pattern

**Epic:** Epic 15  
**PRD:** `project/prd-conclave-mixture-of-agents.md`  
**Duration:** 2 weeks  
**Priority:** High  

---

## Relevant Files

### Core Domain Layer
- `src/core/platform/container/battalion/conclave.rs` - Conclave domain model, config, result, and status types
- `src/core/platform/container/battalion/mod.rs` - Module exports for Conclave

### Application Layer
- `src/application/ports/output/battalion_port.rs` - Battalion port trait (may need Conclave support)
- `src/application/use_cases/battalion/conclave_execution_service.rs` - Conclave execution service with retry logic
- `src/application/use_cases/battalion/commander.rs` - Commander integration for Conclave strategy
- `src/application/use_cases/battalion/mod.rs` - Module exports for battalion use cases

### Infrastructure Layer
- `src/infrastructure/adapters/battalion/` - Potential adapter implementations (if needed)

### CLI Layer
- `src/bin/paladin-cli.rs` - Main CLI entry point
- `src/application/cli/battalion_commands.rs` - Battalion CLI commands for Conclave

### Tests
- `tests/unit/battalion/conclave_domain_test.rs` - Unit tests for Conclave domain model
- `tests/unit/battalion/conclave_execution_test.rs` - Unit tests for execution service with mocks
- `tests/integration/conclave_integration_test.rs` - Integration tests with real execution
- `tests/functional/conclave_cli_test.rs` - CLI functional tests

### Examples
- `examples/conclave_expert_panel.rs` - Basic Conclave example with programmatic API
- `examples/cli_configs/conclave_expert_panel.yaml` - YAML configuration example
- `examples/cli_configs/conclave_code_review.yaml` - Code review use case example

### Documentation
- `docs/guides/conclave-pattern.md` - Comprehensive Conclave pattern guide
- `docs/BATTALION.md` - Update with Conclave section
- `README.md` - Update with Conclave example

### Notes

- Unit tests in Rust are typically placed in the same file using `#[cfg(test)]` modules, but integration tests go in the `tests/` directory
- Run tests with `cargo test` for unit tests, `cargo test --test <test_name>` for specific integration tests
- Follow hexagonal architecture: Core → Application → Infrastructure dependency flow
- All code must pass `cargo fmt`, `cargo clippy`, and have rustdoc comments
- Use TDD approach: write tests first, then implement

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

**Completion Protocol:**
1. Mark sub-task `[x]` when finished
2. When ALL sub-tasks under a parent are `[x]`:
   - Run `cargo test` - all tests must pass
   - Run `cargo fmt --check` - code must be formatted
   - Run `cargo clippy` - no warnings allowed
   - Clean up any temporary code or debug prints
   - Stage changes: `git add .`
   - Commit with descriptive message using conventional commits format
3. Mark parent task `[x]` only after commit

Example commit format:
```bash
git commit -m "feat(conclave): add domain model" -m "- Implements Conclave, ConclaveConfig, ConclaveResult structs" -m "- Adds validation and error types" -m "Related to Epic 15, US-15.1"
```

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch: `git checkout -b feature/epic-15-conclave-pattern`
  - [x] 0.2 Verify current branch: `git branch --show-current`

- [x] 1.0 Setup Conclave Domain Model (US-15.1)
  - [x] 1.1 Read existing Battalion module structure in `src/core/platform/container/battalion/`
  - [x] 1.2 Create `src/core/platform/container/battalion/conclave.rs` file
  - [x] 1.3 Define `ConclaveConfig` struct with all required fields (timeout_seconds, retry_attempts, synthesis_prompt, include_expert_names, max_expert_output_tokens, observability_level)
  - [x] 1.4 Define `ObservabilityLevel` enum (Minimal, Standard, Verbose) with serde derives
  - [x] 1.5 Define `Conclave` struct (name, experts: Vec<Paladin>, aggregator: Paladin, config: ConclaveConfig)
  - [x] 1.6 Define `ConclaveResult` struct with expert_outputs HashMap, aggregated_output, execution metrics, status
  - [x] 1.7 Define `ConclaveStatus` enum (Success, PartialSuccess, Failed) with serde derives
  - [x] 1.8 Define `ConclaveError` enum using thiserror (AllExpertsFailed, AggregatorFailed, ConfigurationError, Timeout, ExpertError)
  - [x] 1.9 Implement `Conclave::validate()` method to check minimum 2 experts, 1 aggregator, no duplicate names
  - [x] 1.10 Add rustdoc comments for all public types and methods
  - [x] 1.11 Export Conclave types in `src/core/platform/container/battalion/mod.rs`
  - [x] 1.12 Write unit tests for domain model validation logic in `#[cfg(test)]` module
  - [x] 1.13 Test validation errors (duplicate names, insufficient experts, etc.)
  - [x] 1.14 Run tests: `cargo test conclave_domain`
  - [x] 1.15 Run formatting: `cargo fmt`
  - [x] 1.16 Run linter: `cargo clippy -- -D warnings`
  - [x] 1.17 Commit changes: "feat(conclave): add domain model and validation"

- [x] 2.0 Implement Conclave Execution Service (US-15.2)
  - [x] 2.1 Read existing `PaladinPort` trait in `src/application/ports/output/paladin_port.rs`
  - [x] 2.2 Create `src/application/use_cases/battalion/conclave_execution_service.rs` file
  - [x] 2.3 Define `ConclaveExecutionService` struct with paladin_port: Arc<dyn PaladinPort>
  - [x] 2.4 Implement `new()` constructor for ConclaveExecutionService
  - [x] 2.5 Implement `execute()` method signature: `async fn execute(&self, conclave: &Conclave, input: &str) -> Result<ConclaveResult, ConclaveError>`
  - [x] 2.6 Implement parallel expert execution using `tokio::spawn` for each expert
  - [x] 2.7 Implement retry logic with exponential backoff (1s, 2s, 4s, 8s, 16s) using tokio::time::sleep
  - [x] 2.8 Add jitter to retry delays (±20% random variance) to avoid thundering herd
  - [x] 2.9 Implement retry logic that only retries transient errors (network, timeout, rate limit)
  - [x] 2.10 Collect successful expert outputs into HashMap<String, PaladinResult>
  - [x] 2.11 Implement `format_expert_outputs_for_aggregator()` method with name labels (configurable)
  - [x] 2.12 Implement default aggregator prompt template construction
  - [x] 2.13 Allow custom synthesis_prompt override from ConclaveConfig
  - [x] 2.14 Execute aggregator with formatted expert outputs
  - [x] 2.15 Implement timeout for entire Conclave execution using `tokio::time::timeout`
  - [x] 2.16 Calculate execution metrics (per-expert times, total time, retry counts)
  - [x] 2.17 Determine ConclaveStatus based on expert success/failure counts
  - [x] 2.18 Return ConclaveError::AllExpertsFailed if all experts fail after retries
  - [x] 2.19 Implement observability level handling (Minimal, Standard, Verbose) for logging
  - [x] 2.20 Add tracing/logging statements at appropriate levels
  - [x] 2.21 Add rustdoc comments for all public methods
  - [x] 2.22 Export ConclaveExecutionService in `src/application/use_cases/battalion/mod.rs`
  - [x] 2.23 Write unit tests with mocked PaladinPort (successful execution)
  - [x] 2.24 Write unit tests for partial failure scenarios (some experts fail, some succeed)
  - [x] 2.25 Write unit tests for retry logic (simulate transient failures)
  - [x] 2.26 Write unit tests for timeout scenarios
  - [x] 2.27 Write unit tests for all experts failing
  - [x] 2.28 Write unit tests for aggregator failure
  - [ ] 2.29 Write integration test in `tests/integration/conclave_integration_test.rs` with real PaladinPort
  - [x] 2.30 Run tests: `cargo test conclave_execution`
  - [x] 2.31 Run formatting: `cargo fmt`
  - [x] 2.32 Run linter: `cargo clippy -- -D warnings`
  - [ ] 2.33 Commit changes: "feat(conclave): implement execution service with retry logic"

- [ ] 3.0 Integrate Conclave with Commander (US-15.3)
  - [ ] 3.1 Read existing Commander implementation in `src/application/use_cases/battalion/commander.rs`
  - [ ] 3.2 Read existing `BattalionStrategy` enum definition
  - [ ] 3.3 Add `Conclave` variant to `BattalionStrategy` enum
  - [ ] 3.4 Update `BattalionStrategy` serde derives if needed
  - [ ] 3.5 Read existing `CommanderBuilder` implementation
  - [ ] 3.6 Add `aggregator: Option<Paladin>` field to CommanderBuilder
  - [ ] 3.7 Implement `aggregator(mut self, paladin: Paladin) -> Self` method on CommanderBuilder
  - [ ] 3.8 Update `build()` method to handle Conclave strategy
  - [ ] 3.9 Implement default aggregator selection logic (use last agent if not specified)
  - [ ] 3.10 Implement Conclave validation in Commander (min 2 experts + 1 aggregator)
  - [ ] 3.11 Read existing auto-strategy detection logic
  - [ ] 3.12 Implement auto-strategy scoring for Conclave (+3 for synthesis keywords, +2 for 3+ agents with different prompts, +1 for comprehensive questions)
  - [ ] 3.13 Add synthesis keyword detection: "compare", "synthesize", "combine perspectives", "expert panel"
  - [ ] 3.14 Implement prompt analysis to detect different expertise (simple heuristic based on system prompts)
  - [ ] 3.15 Update Commander `execute()` method to handle BattalionStrategy::Conclave case
  - [ ] 3.16 Instantiate ConclaveExecutionService when Conclave strategy is selected
  - [ ] 3.17 Call ConclaveExecutionService::execute() and handle result
  - [ ] 3.18 Update rustdoc comments for Commander with Conclave examples
  - [ ] 3.19 Write unit tests for Conclave strategy selection
  - [ ] 3.20 Write unit tests for default aggregator selection
  - [ ] 3.21 Write unit tests for auto-strategy detection with Conclave
  - [ ] 3.22 Write unit tests for Conclave validation in Commander
  - [ ] 3.23 Write integration test for full Commander + Conclave execution
  - [ ] 3.24 Run tests: `cargo test commander`
  - [ ] 3.25 Run formatting: `cargo fmt`
  - [ ] 3.26 Run linter: `cargo clippy -- -D warnings`
  - [ ] 3.27 Commit changes: "feat(conclave): integrate with Commander and auto-strategy"

- [ ] 4.0 Add CLI and YAML Support (US-15.4)
  - [ ] 4.1 Read existing CLI structure in `src/bin/paladin-cli.rs` and `src/application/cli/battalion_commands.rs`
  - [ ] 4.2 Read existing YAML configuration parsing for other Battalion patterns
  - [ ] 4.3 Define `ConclaveYamlConfig` struct for YAML schema with serde derives
  - [ ] 4.4 Add fields: type (must be "conclave"), name, config, aggregator, experts
  - [ ] 4.5 Implement `ConclaveYamlConfig::validate()` method for YAML validation
  - [ ] 4.6 Implement `ConclaveYamlConfig::to_conclave()` method to convert to Conclave domain model
  - [ ] 4.7 Support inline agent definitions in YAML (name, system_prompt, model, temperature)
  - [ ] 4.8 Support agent references in YAML (reference to pre-defined agent configs)
  - [ ] 4.9 Update `battalion run` CLI command to support `--type conclave`
  - [ ] 4.10 Parse Conclave YAML configuration file
  - [ ] 4.11 Build Conclave instance from YAML config
  - [ ] 4.12 Execute Conclave and capture result
  - [ ] 4.13 Implement output formatting for JSON format
  - [ ] 4.14 Implement output formatting for Markdown format (include expert outputs + aggregated result)
  - [ ] 4.15 Implement output formatting for plain text format
  - [ ] 4.16 Implement `battalion new --type conclave --name <name>` command for template generation
  - [ ] 4.17 Create template generator that produces YAML with 3 example experts + 1 aggregator
  - [ ] 4.18 Add helpful comments and documentation to generated template
  - [ ] 4.19 Use sensible defaults in template (gpt-4o, appropriate temperatures, standard config)
  - [ ] 4.20 Add error handling and validation messages for CLI
  - [ ] 4.21 Update CLI help text to include Conclave commands
  - [ ] 4.22 Update rustdoc comments for CLI functions
  - [ ] 4.23 Write unit tests for YAML parsing and validation
  - [ ] 4.24 Write unit tests for YAML to Conclave conversion
  - [ ] 4.25 Write unit tests for template generation
  - [ ] 4.26 Write functional test in `tests/functional/conclave_cli_test.rs` for CLI execution
  - [ ] 4.27 Test template generation manually
  - [ ] 4.28 Run tests: `cargo test cli`
  - [ ] 4.29 Run formatting: `cargo fmt`
  - [ ] 4.30 Run linter: `cargo clippy -- -D warnings`
  - [ ] 4.31 Commit changes: "feat(conclave): add CLI and YAML configuration support"

- [ ] 5.0 Create Examples and Documentation
  - [ ] 5.1 Create `examples/conclave_expert_panel.rs` file
  - [ ] 5.2 Implement basic Conclave example with 3 experts (Technical, Business, Security) + 1 aggregator
  - [ ] 5.3 Use programmatic API (no YAML) to demonstrate ConclaveBuilder usage
  - [ ] 5.4 Add comprehensive comments explaining each step
  - [ ] 5.5 Test example compiles: `cargo build --example conclave_expert_panel`
  - [ ] 5.6 Test example runs successfully (may need to skip if requires API keys): `cargo run --example conclave_expert_panel` or mark with `#[ignore]`
  - [ ] 5.7 Create `examples/cli_configs/conclave_expert_panel.yaml`
  - [ ] 5.8 Define 3 experts with distinct roles and system prompts
  - [ ] 5.9 Define 1 aggregator with synthesis-focused system prompt
  - [ ] 5.10 Add comments explaining each section of YAML
  - [ ] 5.11 Create `examples/cli_configs/conclave_code_review.yaml` for code review use case
  - [ ] 5.12 Define experts: SecurityExpert, PerformanceExpert, MaintainabilityExpert
  - [ ] 5.13 Define aggregator: LeadReviewer
  - [ ] 5.14 Add comments and example task in YAML
  - [ ] 5.15 Validate both YAML files are well-formed
  - [ ] 5.16 Create `docs/guides/conclave-pattern.md` file
  - [ ] 5.17 Write introduction explaining MixtureOfAgents pattern and benefits
  - [ ] 5.18 Write "Quick Start" section with minimal example
  - [ ] 5.19 Write "Configuration" section explaining all ConclaveConfig options
  - [ ] 5.20 Write "Programmatic API" section with Rust code examples
  - [ ] 5.21 Write "YAML Configuration" section with schema reference
  - [ ] 5.22 Write "CLI Usage" section with command examples
  - [ ] 5.23 Write "Use Cases" section with real-world scenarios
  - [ ] 5.24 Write "Error Handling" section explaining retry logic and partial failures
  - [ ] 5.25 Write "Observability" section explaining logging and metrics
  - [ ] 5.26 Write "Best Practices" section (recommended number of experts, prompt engineering, etc.)
  - [ ] 5.27 Write "Troubleshooting" section for common issues
  - [ ] 5.28 Add links to examples and other relevant documentation
  - [ ] 5.29 Read `docs/BATTALION.md` and identify where to add Conclave section
  - [ ] 5.30 Add new section "Conclave Pattern" to BATTALION.md with overview and link to full guide
  - [ ] 5.31 Update pattern comparison table in BATTALION.md to include Conclave
  - [ ] 5.32 Read project README.md
  - [ ] 5.33 Add Conclave example to README features section
  - [ ] 5.34 Add quick Conclave example in README (if appropriate)
  - [ ] 5.35 Run `cargo doc --open` to verify rustdoc generates correctly
  - [ ] 5.36 Review all documentation for clarity, typos, and completeness
  - [ ] 5.37 Commit changes: "docs(conclave): add comprehensive documentation and examples"

- [ ] 6.0 Final Testing and Quality Assurance
  - [ ] 6.1 Run full test suite: `cargo test`
  - [ ] 6.2 Check for any failing tests and fix issues
  - [ ] 6.3 Run unit tests specifically: `cargo test --lib`
  - [ ] 6.4 Run integration tests: `cargo test --test '*'`
  - [ ] 6.5 Run example tests: `cargo test --examples`
  - [ ] 6.6 Test with output: `cargo test -- --nocapture` (if needed for debugging)
  - [ ] 6.7 Run `make test-all` if available
  - [ ] 6.8 Check code formatting: `cargo fmt --check`
  - [ ] 6.9 Fix any formatting issues: `cargo fmt`
  - [ ] 6.10 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 6.11 Fix all clippy warnings
  - [ ] 6.12 Run `cargo check` to ensure all code compiles
  - [ ] 6.13 Run `make clean-code` if available (combines fmt + clippy + check)
  - [ ] 6.14 Manually test CLI command: `cargo run -- battalion new --type conclave --name test-panel`
  - [ ] 6.15 Verify generated YAML template is correct
  - [ ] 6.16 Manually test CLI execution (if API keys available): `cargo run -- battalion run --type conclave --config examples/cli_configs/conclave_expert_panel.yaml`
  - [ ] 6.17 Verify output formatting (JSON, Markdown, text)
  - [ ] 6.18 Test all examples compile: `cargo build --examples`
  - [ ] 6.19 Run each example manually (if API keys available) or verify they're properly marked with `#[ignore]`
  - [ ] 6.20 Review all rustdoc comments for completeness
  - [ ] 6.21 Generate documentation: `cargo doc --no-deps`
  - [ ] 6.22 Review generated docs for correctness
  - [ ] 6.23 Check test coverage (if tooling available): `cargo tarpaulin` or similar
  - [ ] 6.24 Verify ≥80% unit test coverage, ≥70% integration test coverage
  - [ ] 6.25 Review error messages are clear and actionable
  - [ ] 6.26 Test edge cases: 0 experts, 1 expert, 10+ experts
  - [ ] 6.27 Test with different observability levels
  - [ ] 6.28 Test timeout scenarios
  - [ ] 6.29 Test retry logic manually with simulated failures
  - [ ] 6.30 Clean up any debug prints, temporary code, or commented-out code
  - [ ] 6.31 Review all commits have good messages following conventional commits
  - [ ] 6.32 Commit any final cleanup: "chore(conclave): final cleanup and polish"

- [ ] 7.0 Security Scan and Performance Validation
  - [ ] 7.1 Run security audit: `cargo audit`
  - [ ] 7.2 Fix any security vulnerabilities found
  - [ ] 7.3 Run Snyk scan if configured: `snyk test` or use snyk_code_scan tool
  - [ ] 7.4 Address any security issues identified by Snyk
  - [ ] 7.5 Review code for potential security issues (API key exposure, injection vulnerabilities, etc.)
  - [ ] 7.6 Verify no secrets or API keys are hardcoded in examples or tests
  - [ ] 7.7 Test performance with realistic workloads (3-7 experts)
  - [ ] 7.8 Measure execution time overhead (should be ≤10% over max expert time + aggregation time)
  - [ ] 7.9 Test with different LLM providers if available
  - [ ] 7.10 Verify retry backoff doesn't cause excessive delays
  - [ ] 7.11 Test memory usage is reasonable for parallel execution
  - [ ] 7.12 Run benchmarks if available: `cargo bench` (optional)
  - [ ] 7.13 Profile with `cargo flamegraph` if performance issues detected (optional)
  - [ ] 7.14 Document any known performance limitations
  - [ ] 7.15 Verify thread safety with `cargo clippy` for Send/Sync bounds
  - [ ] 7.16 Create Epic 15 completion report documenting what was implemented
  - [ ] 7.17 List all acceptance criteria met for each user story
  - [ ] 7.18 Document any deviations from original PRD
  - [ ] 7.19 Document known limitations or future enhancements
  - [ ] 7.20 Commit any final changes: "chore(conclave): security scan and performance validation"
  - [ ] 7.21 Push branch to remote: `git push origin feature/epic-15-conclave-pattern`
  - [ ] 7.22 Create pull request with comprehensive description
  - [ ] 7.23 Link PR to Epic 15 issue/ticket
  - [ ] 7.24 Request code review from team

---

**Status:** Detailed sub-tasks generated. Ready for implementation!
