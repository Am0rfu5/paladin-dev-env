# Task List: Battalion Orchestration System (Epic 4)

**Epic:** Epic 4 - Battalion Orchestration  
**PRD:** prd-battalion-orchestration.md  
**Priority:** Critical  
**Dependencies:** Epic 1 (Paladin), Epic 2 (Garrison)  
**Phased Implementation:**
- Phase 1: Formation & Phalanx (Weeks 1-3)
- Phase 2: Campaign & Chain of Command (Weeks 4-5)

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**TDD Workflow:**
1. Write tests first (Red)
2. Implement minimal code to pass (Green)
3. Refactor while keeping tests green
4. Run `cargo test`, `cargo fmt`, `cargo clippy` before marking complete

---

## Relevant Files

### Core Domain Layer
- `src/core/platform/container/battalion/mod.rs` - Battalion base types, config, results, error types
- `src/core/platform/container/battalion/formation.rs` - Formation domain entity (sequential pattern) ✓
- `src/core/platform/container/battalion/phalanx.rs` - Phalanx domain entity (concurrent pattern)
- `src/core/platform/container/battalion/campaign.rs` - Campaign domain entity (graph pattern)
- `src/core/platform/container/battalion/chain_of_command.rs` - Chain of Command domain entity (hierarchical pattern)

### Application Layer
- `src/application/ports/output/battalion_port.rs` - BattalionPort trait definition
- `src/application/use_cases/battalion/mod.rs` - Battalion use cases module
- `src/application/use_cases/battalion/retry.rs` - Retry logic utility with exponential backoff
- `src/application/use_cases/battalion/error_aggregation.rs` - Error aggregation utility
- `src/application/use_cases/battalion/formation_service.rs` - Formation execution orchestration ✓
- `src/application/use_cases/battalion/phalanx_service.rs` - Phalanx execution orchestration
- `src/application/use_cases/battalion/campaign_service.rs` - Campaign execution orchestration
- `src/application/use_cases/battalion/chain_of_command_service.rs` - Chain of Command execution orchestration
- `src/application/use_cases/battalion/commander.rs` - Commander strategy router (Epic 5)

### Unit Tests
- `src/core/platform/container/battalion/battalion.rs` - Tests for base types (inline)
- `tests/unit/battalion/formation_tests.rs` - Formation domain tests
- `tests/unit/battalion/phalanx_tests.rs` - Phalanx domain tests
- `tests/unit/battalion/campaign_tests.rs` - Campaign domain tests
- `tests/unit/battalion/chain_of_command_tests.rs` - Chain of Command domain tests

### Integration Tests
- `tests/integration/battalion/formation_integration_test.rs` - Formation end-to-end tests
- `tests/integration/battalion/phalanx_integration_test.rs` - Phalanx end-to-end tests
- `tests/integration/battalion/campaign_integration_test.rs` - Campaign end-to-end tests
- `tests/integration/battalion/chain_of_command_integration_test.rs` - Chain of Command end-to-end tests

### Examples
- `examples/formation_sequential.rs` - Formation usage example
- `examples/phalanx_parallel.rs` - Phalanx usage example
- `examples/campaign_workflow.rs` - Campaign usage example
- `examples/chain_of_command_delegation.rs` - Chain of Command usage example

### Configuration
- `Cargo.toml` - Add dependencies: tokio, futures, petgraph

### Notes

- Follow TDD: Write tests first, implement minimal code, refactor
- Maintain hexagonal architecture: Domain → Application → Infrastructure
- Run `cargo test` after each sub-task completion
- Run `cargo fmt` and `cargo clippy` before marking tasks complete
- All public APIs must have rustdoc with examples
- Target ≥80% unit test coverage per module

---

## Tasks

- [x] 0.0 Create feature branch for Epic 4
  - [x] 0.1 Ensure on main branch and up to date (`git checkout main && git pull`)
  - [x] 0.2 Create and checkout feature branch (`git checkout -b feature/epic4-battalion-orchestration`)
  - [x] 0.3 Verify branch created (`git branch --show-current`)

- [x] 1.0 Set up Core Battalion Infrastructure (Domain Layer)
  - [x] 1.1 Add required dependencies to `Cargo.toml` (tokio, futures, petgraph, derive_builder)
  - [x] 1.2 Create `src/core/platform/container/battalion/` directory
  - [x] 1.3 Write unit tests for `BattalionConfig` in `mod.rs` (TDD - Red)
  - [x] 1.4 Implement `BattalionConfig` struct with Builder derive
  - [x] 1.5 Write unit tests for `RetryPolicy` struct (TDD - Red)
  - [x] 1.6 Implement `RetryPolicy` with exponential backoff config
  - [x] 1.7 Write unit tests for `ErrorStrategy` enum variants (TDD - Red)
  - [x] 1.8 Implement `ErrorStrategy` enum (FailFast, ContinueOnError, RetryThenContinue)
  - [x] 1.9 Write unit tests for `BattalionStatus` enum (TDD - Red)
  - [x] 1.10 Implement `BattalionStatus` enum (Idle, Running, Paused, Completed, Failed, Cancelled)
  - [x] 1.11 Write unit tests for `BattalionResult` struct (TDD - Red)
  - [x] 1.12 Implement `BattalionResult` with all required fields
  - [x] 1.13 Write unit tests for `BattalionError` enum (TDD - Red)
  - [x] 1.14 Implement `BattalionError` using thiserror with From<PaladinError>
  - [x] 1.15 Update `src/core/platform/container/mod.rs` to export battalion module
  - [x] 1.16 Run tests: `cargo test battalion::mod`
  - [x] 1.17 Run fmt and clippy: `cargo fmt && cargo clippy`
  - [x] 1.18 Commit: "feat(core): add Battalion base infrastructure"

- [x] 2.0 Implement Battalion Application Layer (Ports & Base Services)
  - [x] 2.1 Create `src/application/ports/output/battalion_port.rs`
  - [x] 2.2 Write trait definition tests for `BattalionPort` (TDD - Red)
  - [x] 2.3 Implement `BattalionPort` trait with execute, status, cancel methods
  - [x] 2.4 Add rustdoc with examples for all trait methods
  - [x] 2.5 Update `src/application/ports/output/mod.rs` to export battalion_port
  - [x] 2.6 Create `src/application/use_cases/battalion/mod.rs`
  - [x] 2.7 Write unit tests for retry logic utility (TDD - Red)
  - [x] 2.8 Implement retry logic utility with exponential backoff and jitter
  - [x] 2.9 Write unit tests for error aggregation utility (TDD - Red)
  - [x] 2.10 Implement error aggregation for ContinueOnError strategy
  - [x] 2.11 Update `src/application/use_cases/mod.rs` to export battalion module
  - [x] 2.12 Run tests: `cargo test battalion_port`
  - [x] 2.13 Run fmt and clippy: `cargo fmt && cargo clippy`
  - [x] 2.14 Commit: "feat(application): add BattalionPort and utilities"

- [x] 3.0 Implement Formation Pattern (Phase 1 - Sequential Execution)
  - [x] 3.1 Create `tests/unit/battalion/formation_tests.rs`
  - [x] 3.2 Write failing tests for Formation construction (TDD - Red)
  - [x] 3.3 Implement `Formation` struct in `src/core/platform/container/battalion/formation.rs`
  - [x] 3.4 Write failing tests for Formation::new() and builders (TDD - Red)
  - [x] 3.5 Implement Formation::new(), with_config(), with_shared_context()
  - [x] 3.6 Write failing tests for Formation validation (TDD - Red)
  - [x] 3.7 Implement Formation validation (≥2 Paladins required)
  - [x] 3.8 Update battalion/mod.rs to export Formation
  - [x] 3.9 Run unit tests: `cargo test formation_tests`
  - [x] 3.10 Create `src/application/use_cases/battalion/formation_service.rs`
  - [x] 3.11 Write failing tests for FormationExecutionService (TDD - Red)
  - [x] 3.12 Implement FormationExecutionService struct with PaladinExecutionService dependency
  - [x] 3.13 Write failing tests for sequential execution logic (TDD - Red)
  - [x] 3.14 Implement execute() method with sequential Paladin execution
  - [x] 3.15 Write failing tests for output passing (N → N+1) (TDD - Red)
  - [x] 3.16 Implement output passing between Paladins
  - [x] 3.17 Write failing tests for FailFast error strategy (TDD - Red)
  - [x] 3.18 Implement FailFast error handling
  - [x] 3.19 Write failing tests for ContinueOnError strategy (TDD - Red)
  - [x] 3.20 Implement ContinueOnError with error collection
  - [x] 3.21 Write failing tests for RetryThenContinue strategy (TDD - Red)
  - [x] 3.22 Implement RetryThenContinue with retry logic
  - [x] 3.23 Write failing tests for timeout enforcement (TDD - Red)
  - [x] 3.24 Implement Battalion-level timeout using tokio::time::timeout
  - [x] 3.25 Write failing tests for shared context injection (TDD - Red)
  - [x] 3.26 Implement shared context injection into Paladin prompts
  - [x] 3.27 Add comprehensive rustdoc with examples
  - [x] 3.28 Run unit tests: `cargo test formation`
  - [x] 3.29 Create `tests/integration/battalion/formation_integration_test.rs`
  - [x] 3.30 Write integration test with mock Paladins for end-to-end flow
  - [x] 3.31 Write integration test for error scenarios
  - [x] 3.32 Run integration tests: `cargo test --test formation_integration_test`
  - [x] 3.33 Create `examples/formation_sequential.rs` with complete example
  - [x] 3.34 Test example: `cargo run --example formation_sequential`
  - [x] 3.35 Run fmt and clippy: `cargo fmt && cargo clippy`
  - [x] 3.36 Verify ≥80% coverage for Formation code
  - [x] 3.37 Commit: "test(battalion): add Formation integration tests and examples"

- [x] 4.0 Implement Phalanx Pattern (Phase 1 - Concurrent Execution)
  - [x] 4.1 Create `tests/unit/battalion/phalanx_tests.rs`
  - [x] 4.2 Write failing tests for Phalanx construction (TDD - Red)
  - [x] 4.3 Implement `Phalanx` struct in `src/core/platform/container/battalion/phalanx.rs`
  - [x] 4.4 Write failing tests for AggregationStrategy enum (TDD - Red)
  - [x] 4.5 Implement `AggregationStrategy` enum (CollectAll, FirstSuccess, Majority, Custom)
  - [x] 4.6 Write failing tests for Phalanx builders (TDD - Red)
  - [x] 4.7 Implement Phalanx::new() and with_aggregation()
  - [x] 4.8 Write failing tests for Phalanx validation (TDD - Red)
  - [x] 4.9 Implement Phalanx validation (≥2 Paladins, valid aggregation)
  - [x] 4.10 Update battalion/mod.rs to export Phalanx
  - [x] 4.11 Run unit tests: `cargo test phalanx_tests`
  - [x] 4.12 Create `src/application/use_cases/battalion/phalanx_service.rs`
  - [x] 4.13 Write failing tests for PhalanxExecutionService (TDD - Red)
  - [x] 4.14 Implement PhalanxExecutionService struct
  - [x] 4.15 Write failing tests for concurrent execution (TDD - Red)
  - [x] 4.16 Implement concurrent execution using tokio::spawn for each Paladin
  - [x] 4.17 Write failing tests for CollectAll aggregation (TDD - Red)
  - [x] 4.18 Implement CollectAll aggregation strategy
  - [x] 4.19 Write failing tests for FirstSuccess aggregation (TDD - Red)
  - [x] 4.20 Implement FirstSuccess with early termination (tokio::select!)
  - [x] 4.21 Write failing tests for Majority aggregation (TDD - Red)
  - [x] 4.22 Implement Majority consensus algorithm (requires ≥3 Paladins)
  - [x] 4.23 Write failing tests for Custom aggregation (TDD - Red)
  - [x] 4.24 Implement Custom aggregation with user function
  - [x] 4.25 Write failing tests for concurrency limiting (TDD - Red)
  - [x] 4.26 Implement semaphore-based concurrency limiting (max 10 concurrent)
  - [x] 4.27 Write failing tests for partial failure handling (TDD - Red)
  - [x] 4.28 Implement partial failure handling per error strategy
  - [x] 4.29 Write failing tests for cancellation support (TDD - Red)
  - [x] 4.30 Implement cancellation using tokio::sync::CancellationToken
  - [x] 4.31 Add comprehensive rustdoc with examples
  - [x] 4.32 Run unit tests: `cargo test phalanx`
  - [x] 4.33 Create `tests/integration/battalion/phalanx_integration_test.rs`
  - [x] 4.34 Write integration test with 10 mock Paladins for concurrency
  - [x] 4.35 Write integration test for each aggregation strategy
  - [x] 4.36 Write performance test: verify <1s orchestration overhead
  - [x] 4.37 Run integration tests: `cargo test --test phalanx_integration_test`
  - [x] 4.38 Create `examples/phalanx_parallel.rs` with complete example
  - [x] 4.39 Test example: `cargo run --example phalanx_parallel`
  - [x] 4.40 Run fmt and clippy: `cargo fmt && cargo clippy`
  - [x] 4.41 Verify ≥80% coverage for Phalanx code
  - [x] 4.42 Commit: "test(battalion): add Phalanx integration tests and examples"
  - [x] 4.43 **PHASE 1 MILESTONE:** Run full test suite: `cargo test`
  - [x] 4.44 **PHASE 1 MILESTONE:** Create PR for Phase 1 review
  
- [x] 5.0 Implement Campaign Pattern (Phase 2 - Graph Orchestration)
  - [x] 5.1 Create `tests/unit/battalion/campaign_tests.rs`
  - [x] 5.2 Write failing tests for Campaign construction (TDD - Red)
  - [x] 5.3 Implement `Campaign` struct in `src/core/platform/container/battalion/campaign.rs`
  - [x] 5.4 Write failing tests for CampaignEdge struct (TDD - Red)
  - [x] 5.5 Implement `CampaignEdge` with condition and transform
  - [x] 5.6 Write failing tests for EdgeCondition enum (TDD - Red)
  - [x] 5.7 Implement `EdgeCondition` (Always, Contains, Regex, Custom)
  - [x] 5.8 Write failing tests for Campaign::new() and builders (TDD - Red)
  - [x] 5.9 Implement Campaign::new(), add_paladin(), add_edge()
  - [x] 5.10 Write failing tests for graph validation (TDD - Red)
  - [x] 5.11 Implement Campaign::validate() with cycle detection using petgraph
  - [x] 5.12 Write failing tests for entry point validation (TDD - Red)
  - [x] 5.13 Implement entry point management
  - [x] 5.14 Update battalion/mod.rs to export Campaign
  - [x] 5.15 Run unit tests: `cargo test campaign_tests`
  - [x] 5.16 Create `src/application/use_cases/battalion/campaign_service.rs`
  - [x] 5.17 Write failing tests for CampaignExecutionService (TDD - Red)
  - [x] 5.18 Implement CampaignExecutionService struct
  - [x] 5.19 Write failing tests for topological sort execution (TDD - Red)
  - [x] 5.20 Implement topological sort using petgraph::algo::toposort
  - [x] 5.21 Write failing tests for edge condition evaluation (TDD - Red)
  - [x] 5.22 Implement edge condition evaluation logic
  - [x] 5.23 Write failing tests for output transformation on edges (TDD - Red)
  - [x] 5.24 Implement edge transform application
  - [x] 5.25 Write failing tests for parallel branch execution (TDD - Red)
  - [x] 5.26 Implement parallel execution of independent branches
  - [x] 5.27 Write failing tests for fan-out pattern (1 → N) (TDD - Red)
  - [x] 5.28 Implement fan-out with concurrent execution
  - [x] 5.29 Write failing tests for fan-in pattern (N → 1) (TDD - Red)
  - [x] 5.30 Implement fan-in with result collection
  - [x] 5.31 Write failing tests for multiple entry points (TDD - Red)
  - [x] 5.32 Implement multiple entry point handling
  - [x] 5.33 Add comprehensive rustdoc with graph examples
  - [ ] 5.34 Run unit tests: `cargo test campaign`
  - [ ] 5.35 Create `tests/integration/battalion/campaign_integration_test.rs`
  - [ ] 5.36 Write integration test for linear graph (chain)
  - [ ] 5.37 Write integration test for branching graph (conditional routing)
  - [ ] 5.38 Write integration test for complex DAG with fan-out/fan-in
  - [ ] 5.39 Write integration test for cycle detection
  - [ ] 5.40 Run integration tests: `cargo test --test campaign_integration_test`
  - [ ] 5.41 Create `examples/campaign_workflow.rs` with complex DAG example
  - [ ] 5.42 Test example: `cargo run --example campaign_workflow`
  - [ ] 5.43 Run fmt and clippy: `cargo fmt && cargo clippy`
  - [ ] 5.44 Verify ≥80% coverage for Campaign code
  - [ ] 5.45 Commit: "feat(battalion): implement Campaign graph pattern"

- [ ] 6.0 Implement Chain of Command Pattern (Phase 2 - Hierarchical Delegation)
  - [ ] 6.1 Create `tests/unit/battalion/chain_of_command_tests.rs`
  - [ ] 6.2 Write failing tests for ChainOfCommand construction (TDD - Red)
  - [ ] 6.3 Implement `ChainOfCommand` struct in `src/core/platform/container/battalion/chain_of_command.rs`
  - [ ] 6.4 Write failing tests for DelegationStrategy enum (TDD - Red)
  - [ ] 6.5 Implement `DelegationStrategy` (Automatic, Broadcast, RoundRobin, Custom)
  - [ ] 6.6 Write failing tests for ChainOfCommand builders (TDD - Red)
  - [ ] 6.7 Implement ChainOfCommand::new() and with_strategy()
  - [ ] 6.8 Write failing tests for validation (TDD - Red)
  - [ ] 6.9 Implement validation (1 commander, ≥1 specialist)
  - [ ] 6.10 Update battalion/mod.rs to export ChainOfCommand
  - [ ] 6.11 Run unit tests: `cargo test chain_of_command_tests`
  - [ ] 6.12 Create `src/application/use_cases/battalion/chain_of_command_service.rs`
  - [ ] 6.13 Write failing tests for ChainOfCommandExecutionService (TDD - Red)
  - [ ] 6.14 Implement ChainOfCommandExecutionService struct
  - [ ] 6.15 Write failing tests for Automatic delegation (TDD - Red)
  - [ ] 6.16 Implement Automatic delegation (commander analyzes and selects)
  - [ ] 6.17 Write failing tests for Broadcast delegation (TDD - Red)
  - [ ] 6.18 Implement Broadcast (all specialists concurrently)
  - [ ] 6.19 Write failing tests for RoundRobin delegation (TDD - Red)
  - [ ] 6.20 Implement RoundRobin with state tracking
  - [ ] 6.21 Write failing tests for Custom delegation (TDD - Red)
  - [ ] 6.22 Implement Custom delegation with user function
  - [ ] 6.23 Write failing tests for result aggregation (TDD - Red)
  - [ ] 6.24 Implement commander aggregation of specialist results
  - [ ] 6.25 Write failing tests for context injection (TDD - Red)
  - [ ] 6.26 Implement specialist result injection into commander context
  - [ ] 6.27 Write failing tests for specialist failure handling (TDD - Red)
  - [ ] 6.28 Implement fallback logic for specialist failures
  - [ ] 6.29 Add comprehensive rustdoc with delegation examples
  - [ ] 6.30 Run unit tests: `cargo test chain_of_command`
  - [ ] 6.31 Create `tests/integration/battalion/chain_of_command_integration_test.rs`
  - [ ] 6.32 Write integration test for Automatic delegation
  - [ ] 6.33 Write integration test for Broadcast to all specialists
  - [ ] 6.34 Write integration test for RoundRobin cycling
  - [ ] 6.35 Write integration test for specialist failure scenarios
  - [ ] 6.36 Run integration tests: `cargo test --test chain_of_command_integration_test`
  - [ ] 6.37 Create `examples/chain_of_command_delegation.rs` with specialist example
  - [ ] 6.38 Test example: `cargo run --example chain_of_command_delegation`
  - [ ] 6.39 Run fmt and clippy: `cargo fmt && cargo clippy`
  - [ ] 6.40 Verify ≥80% coverage for ChainOfCommand code
  - [ ] 6.41 Commit: "feat(battalion): implement Chain of Command hierarchical pattern"
  - [ ] 6.42 **PHASE 2 MILESTONE:** Run full test suite: `cargo test`

- [ ] 7.0 Integration Testing, Performance Validation & Documentation
  - [ ] 7.1 Create comprehensive load test: `tests/integration/battalion/load_test.rs`
  - [ ] 7.2 Implement load test: 50 concurrent Battalions, 10 Paladins each
  - [ ] 7.3 Run load test and verify performance targets met
  - [ ] 7.4 Create stress test for concurrency limits
  - [ ] 7.5 Verify <1s orchestration overhead for typical workloads
  - [ ] 7.6 Run all tests with coverage: `cargo tarpaulin` or equivalent
  - [ ] 7.7 Verify ≥80% unit test coverage across all Battalion modules
  - [ ] 7.8 Update `src/lib.rs` to properly export Battalion types
  - [ ] 7.9 Create `docs/BATTALION.md` with comprehensive usage guide
  - [ ] 7.10 Document all four Battalion patterns with examples
  - [ ] 7.11 Document error strategies and retry policies
  - [ ] 7.12 Document performance characteristics and limitations
  - [ ] 7.13 Update main `README.md` with Battalion section
  - [ ] 7.14 Add Battalion configuration section to `config.yml`
  - [ ] 7.15 Update `Cargo.toml` metadata and feature flags
  - [ ] 7.16 Run final code quality checks: `make clean-code`
  - [ ] 7.17 Verify all examples run successfully
  - [ ] 7.18 Run security audit: `cargo audit`
  - [ ] 7.19 Run Snyk scan on new code: `snyk_code_scan`
  - [ ] 7.20 Address any security issues found
  - [ ] 7.21 Create benchmark suite for performance tracking
  - [ ] 7.22 Run benchmarks and document baseline performance
  - [ ] 7.23 Review all rustdoc for completeness and accuracy
  - [ ] 7.24 Commit: "docs(battalion): add comprehensive documentation"
  - [ ] 7.25 Final commit: "feat(epic4): complete Battalion Orchestration System"
  - [ ] 7.26 Push feature branch to remote
  - [ ] 7.27 Create Pull Request with Epic 4 completion summary
  - [ ] 7.28 Address PR review feedback
  - [ ] 7.29 Merge to main after approval
  - [ ] 7.30 Update Epic 4 acceptance criteria in epic4.md
  - [ ] 7.31 Close Epic 4 in project tracker

---

