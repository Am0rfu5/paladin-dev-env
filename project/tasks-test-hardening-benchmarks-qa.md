# Task List: Test Hardening, Benchmarks & QA (Epic 24)

## Relevant Files

### Benchmarks
- `benches/battalion_benchmarks.rs` - Battalion orchestration benchmarks (Campaign, ChainOfCommand fixes needed at lines 297, 390, 950)
- `docs/BATTALION_BENCHMARKS.md` - Benchmark results documentation

### Test Files
- `tests/unit/mod.rs` - Unit test module aggregator (prompt generation tests commented at line 22)
- `tests/unit/paladin_execution_service_test.rs` - Execution service tests (timeout test ignored at lines 237, 239)
- `tests/unit/prompt_generation_service_test.rs` - Prompt generation service tests (currently disabled)
- `tests/integration/rag_integration_tests.rs` - Qdrant/RAG integration tests (placeholder at line 147)
- `tests/unit/sanctum/qdrant_sanctum_test.rs` - Qdrant unit tests (incomplete at line 62)
- `tests/integration/llm_live_api_tests.rs` - Live API integration tests (NEW - to be created)
- `tests/cli/` - CLI test directory (NEW - to be created)
- `tests/cli/snapshots/` - Snapshot test directory (NEW - to be created)
- `tests/cli/table_output_test.rs` - CLI table rendering snapshot tests (NEW)
- `tests/cli/progress_output_test.rs` - CLI progress indicator snapshot tests (NEW)
- `tests/cli/error_output_test.rs` - CLI error message snapshot tests (NEW)
- `tests/cli/help_output_test.rs` - CLI help command snapshot tests (NEW)

### Source Files for Coverage
- `src/core/platform/manager/user_service.rs` - User service (4.23% coverage - needs review)
- `src/core/platform/manager/listener_service.rs` - Listener service (57.83% coverage - needs improvement)

### Documentation Files
- `README.md` - Project README (needs Council/Grove pattern examples)
- `docs/QUICKSTART.md` - Quickstart guide (needs Council/Grove quickstart)
- `docs/INSTALLATION.md` - Installation guide (needs CLI setup info)
- `docs/cli/README.md` - CLI documentation (comprehensive update needed)
- `docs/Design/Design_and_Architecture.md` - Architecture documentation (review and update)
- `CONTRIBUTING.md` - Contributing guide (needs testing guidelines)
- `docs/assets/` - Demo assets directory (NEW - for terminal recordings)
- `RELEASE_NOTES_MILESTONE_3.md` - Release notes (NEW - to be created)

### Configuration Files
- `Cargo.toml` - Add `insta` crate dependency for snapshot testing
- `.github/workflows/` - CI/CD configuration (add CLI test jobs)

### Notes

- Unit tests should be placed alongside the code they test or in the `tests/unit/` directory
- Integration tests go in `tests/integration/` directory
- Use `cargo test` to run all tests, `cargo test <pattern>` for specific tests
- Use `cargo bench --no-run` to verify benchmarks compile without running them
- Use `cargo clippy -- -D warnings` and `cargo fmt --check` for quality gates
- Coverage can be measured with `cargo tarpaulin` or `cargo llvm-cov`

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch from develop: `git checkout -b feature/epic-24-test-hardening-benchmarks-qa`
  - [x] 0.2 Verify branch created successfully

- [ ] 1.0 Fix Campaign & ChainOfCommand Benchmarks (US-24.1)
  - [x] 1.1 Read `benches/battalion_benchmarks.rs` to understand current benchmark implementation
  - [x] 1.2 Examine Campaign API in `src/core/platform/container/battalion/campaign.rs` to identify correct method signatures
  - [x] 1.3 Update `benchmark_campaign` function (line ~297) to use current API (add_node/add_edge methods)
  - [x] 1.4 Examine ChainOfCommand API in `src/core/platform/container/battalion/chain_of_command.rs` for constructor signature
  - [x] 1.5 Update `benchmark_chain_of_command` function (line ~390) to match current constructor signature
  - [x] 1.6 Re-enable both benchmarks in criterion group registration (line ~950)
  - [x] 1.7 Run `cargo bench --no-run` to verify benchmarks compile without errors
  - [x] 1.8 Run `cargo clippy` on benchmark file to ensure no warnings
  - [x] 1.9 Run actual benchmarks with `cargo bench` to verify they produce metrics
  - [x] 1.10 Document benchmark results in `docs/BATTALION_BENCHMARKS.md` (append new results)
  - [ ] 1.11 Commit changes: `git commit -m "fix: update Campaign and ChainOfCommand benchmarks to current API"`

- [ ] 2.0 Enable Deferred Test Modules (US-24.2, US-24.3)
  - [ ] 2.1 Read `tests/unit/mod.rs` line 22 to see commented prompt generation test module
  - [ ] 2.2 Read `src/application/ports/output/llm_port.rs` to understand current `LlmPort` trait signature
  - [ ] 2.3 Create or update `MockLlmPort` in `tests/unit/mod.rs` to match current trait (add missing methods)
  - [ ] 2.4 Uncomment the `prompt_generation_service_test` module in `tests/unit/mod.rs`
  - [ ] 2.5 Read `tests/unit/prompt_generation_service_test.rs` to identify failing tests
  - [ ] 2.6 Fix compilation errors in prompt generation tests (update mock usage)
  - [ ] 2.7 Run `cargo test prompt_generation` to verify tests pass
  - [ ] 2.8 Read `tests/unit/paladin_execution_service_test.rs` lines 237-239 to see ignored timeout test
  - [ ] 2.9 Enhance `MockLlmPort` to support configurable delays (add delay field/method)
  - [ ] 2.10 Implement timeout test logic that verifies 60-second timeout behavior
  - [ ] 2.11 Remove `#[ignore]` attribute from timeout test
  - [ ] 2.12 Add additional timeout edge case tests (0s timeout, timeout > max_duration)
  - [ ] 2.13 Run `cargo test timeout` multiple times to verify no flakiness
  - [ ] 2.14 Run full test suite: `cargo test` to ensure no regressions
  - [ ] 2.15 Commit changes: `git commit -m "test: enable prompt generation and timeout tests"`

- [ ] 3.0 Implement Qdrant Integration Tests (US-24.4)
  - [ ] 3.1 Read existing `tests/integration/rag_integration_tests.rs` line 147 to see placeholder
  - [ ] 3.2 Read `src/infrastructure/adapters/sanctum/qdrant_sanctum.rs` to understand Qdrant adapter API
  - [ ] 3.3 Create helper function to check Qdrant availability (connect to localhost:6333)
  - [ ] 3.4 Implement test setup: create test collection with unique name (e.g., `test_paladin_{uuid}`)
  - [ ] 3.5 Implement test teardown: delete test collection after test completes
  - [ ] 3.6 Write integration test for store operation (insert documents with embeddings)
  - [ ] 3.7 Write integration test for search operation (semantic search with query)
  - [ ] 3.8 Write integration test for delete operation (remove documents)
  - [ ] 3.9 Write integration test for update operation (modify existing documents)
  - [ ] 3.10 Write integration test for vector search with different similarity metrics (cosine, dot product)
  - [ ] 3.11 Write end-to-end test: create RAG-enabled Paladin → execute with context retrieval
  - [ ] 3.12 Write test for token budget limiting (verify context truncation)
  - [ ] 3.13 Write test for context formatting (verify prompt construction with retrieved docs)
  - [ ] 3.14 Add test attribute to skip if Qdrant unavailable: check connection, skip with warning
  - [ ] 3.15 Read `tests/unit/sanctum/qdrant_sanctum_test.rs` line 62 for incomplete unit tests
  - [ ] 3.16 Expand unit tests for Qdrant adapter (mock HTTP responses)
  - [ ] 3.17 Run integration tests: `cargo test --test rag_integration_tests` (requires Docker/Qdrant)
  - [ ] 3.18 Verify integration test coverage ≥70% for RAG workflow
  - [ ] 3.19 Commit changes: `git commit -m "test: implement comprehensive Qdrant integration tests"`

- [ ] 4.0 Add CLI Snapshot Testing Infrastructure (US-24.6)
  - [ ] 4.1 Add `insta = "1.34"` dependency to `Cargo.toml` under `[dev-dependencies]`
  - [ ] 4.2 Run `cargo build` to verify dependency resolves correctly
  - [ ] 4.3 Create directory `tests/cli/` for CLI-specific tests
  - [ ] 4.4 Create directory `tests/cli/snapshots/` for insta snapshots
  - [ ] 4.5 Examine CLI table rendering code in `src/` to identify table formatting functions
  - [ ] 4.6 Create `tests/cli/table_output_test.rs` with snapshot tests for all table formats
  - [ ] 4.7 Create `tests/cli/progress_output_test.rs` with snapshot tests for progress indicators/spinners
  - [ ] 4.8 Create `tests/cli/error_output_test.rs` with snapshot tests for error messages (formatted/colored)
  - [ ] 4.9 Create `tests/cli/help_output_test.rs` with snapshot tests for command help output
  - [ ] 4.10 Run `cargo test --test table_output_test` to generate initial snapshots
  - [ ] 4.11 Review snapshots with `cargo insta review` and accept if correct
  - [ ] 4.12 Run all CLI tests: `cargo test --test '*_output_test'` to verify they pass
  - [ ] 4.13 Read CLI source files and identify all public functions/types
  - [ ] 4.14 Add rustdoc comments to all public CLI functions (use `///` with examples)
  - [ ] 4.15 Add rustdoc comments to all public CLI types (structs, enums)
  - [ ] 4.16 Update `docs/QUICKSTART.md` with CLI usage examples (add new section)
  - [ ] 4.17 Update `docs/INSTALLATION.md` with CLI installation and setup instructions
  - [ ] 4.18 Update `docs/cli/README.md` with comprehensive CLI documentation
  - [ ] 4.19 Run `cargo doc --open` to verify CLI documentation renders correctly
  - [ ] 4.20 Commit changes: `git commit -m "test: add CLI snapshot testing infrastructure" -m "docs: document CLI usage and API"`

- [ ] 5.0 Implement Live API Integration Tests (US-24.7)
  - [ ] 5.1 Add `live-api-tests` feature flag to `Cargo.toml` under `[features]`
  - [ ] 5.2 Create `tests/integration/llm_live_api_tests.rs` file
  - [ ] 5.3 Add `#[cfg(feature = "live-api-tests")]` attribute to test module
  - [ ] 5.4 Create helper function to check for API key env var and skip if missing
  - [ ] 5.5 Read `src/infrastructure/adapters/llm/openai_adapter.rs` to understand OpenAI adapter
  - [ ] 5.6 Write OpenAI live test: completion (requires `OPENAI_API_KEY` env var)
  - [ ] 5.7 Write OpenAI live test: streaming completion
  - [ ] 5.8 Write OpenAI live test: tool calling (function calling)
  - [ ] 5.9 Write OpenAI live test: error handling (invalid model, rate limits)
  - [ ] 5.10 Read `src/infrastructure/adapters/llm/deepseek_adapter.rs` to understand DeepSeek adapter
  - [ ] 5.11 Write DeepSeek live test: completion (requires `DEEPSEEK_API_KEY` env var)
  - [ ] 5.12 Write DeepSeek live test: streaming completion
  - [ ] 5.13 Write DeepSeek live test: tool calling
  - [ ] 5.14 Write DeepSeek live test: error handling
  - [ ] 5.15 Read `src/infrastructure/adapters/llm/anthropic_adapter.rs` to understand Anthropic adapter
  - [ ] 5.16 Write Anthropic live test: completion (requires `ANTHROPIC_API_KEY` env var)
  - [ ] 5.17 Write Anthropic live test: streaming completion
  - [ ] 5.18 Write Anthropic live test: tool calling
  - [ ] 5.19 Write Anthropic live test: error handling
  - [ ] 5.20 Implement rate limiting logic (delays between API calls)
  - [ ] 5.21 Implement retry logic with exponential backoff
  - [ ] 5.22 Test locally with API keys: `cargo test --features live-api-tests`
  - [ ] 5.23 Verify tests skip gracefully when API keys not available (no failures)
  - [ ] 5.24 Update README with section "Running Live API Tests" (document feature flag usage)
  - [ ] 5.25 Commit changes: `git commit -m "test: add live API integration tests (gated by feature flag)"`

- [ ] 6.0 Improve Deferred Module Test Coverage (US-24.5)
  - [ ] 6.1 Generate coverage report: `cargo llvm-cov --html` or `cargo tarpaulin --out Html`
  - [ ] 6.2 Review coverage for `src/core/platform/manager/user_service.rs` (was 4.23%)
  - [ ] 6.3 Analyze user_service.rs code complexity and determine if 80% coverage is achievable
  - [ ] 6.4 Write cost/benefit analysis: effort required vs. value gained for user_service tests
  - [ ] 6.5 If justified, write unit tests for user_service.rs (focus on critical paths)
  - [ ] 6.6 If not justified, document rationale in `project/DEFERRED_COVERAGE.md` (create file)
  - [ ] 6.7 Review coverage for `src/core/platform/manager/listener_service.rs` (was 57.83%)
  - [ ] 6.8 Analyze listener_service.rs code and identify untested branches/functions
  - [ ] 6.9 Write additional unit tests for listener_service.rs to reach ≥80% if feasible
  - [ ] 6.10 If 80% not achievable, document minimum threshold and reasoning
  - [ ] 6.11 Re-generate coverage report to verify improvements
  - [ ] 6.12 Verify overall project coverage: ≥80% unit, ≥70% integration
  - [ ] 6.13 Generate coverage badge/report for PR review
  - [ ] 6.14 Commit changes: `git commit -m "test: improve coverage for user and listener services"`

- [ ] 7.0 Update Documentation and Create Demo Assets (US-24.8)
  - [ ] 7.1 Read existing `README.md` to understand current structure
  - [ ] 7.2 Add Council pattern examples to README (multi-agent discussion scenario)
  - [ ] 7.3 Add Grove pattern examples to README (dynamic routing scenario)
  - [ ] 7.4 Update feature list in README to include all Milestone 3 features
  - [ ] 7.5 Read `docs/QUICKSTART.md` and identify where to add Council/Grove sections
  - [ ] 7.6 Add Council quickstart guide to QUICKSTART.md (step-by-step example)
  - [ ] 7.7 Add Grove quickstart guide to QUICKSTART.md (step-by-step example)
  - [ ] 7.8 Create directory `docs/assets/` for demo assets
  - [ ] 7.9 Install asciinema if not available: check/install terminal recorder
  - [ ] 7.10 Record demo: basic Paladin execution with CLI
  - [ ] 7.11 Record demo: Battalion Formation workflow
  - [ ] 7.12 Record demo: Council discussion pattern
  - [ ] 7.13 Record demo: Grove routing with conditions
  - [ ] 7.14 Save recordings to `docs/assets/` directory (e.g., `demo-basic.cast`, `demo-council.cast`)
  - [ ] 7.15 Update README to link to demo assets
  - [ ] 7.16 Read `docs/Design/Design_and_Architecture.md` thoroughly
  - [ ] 7.17 Update architecture doc with any changes from Milestone 3 (Council, Grove, Maneuver)
  - [ ] 7.18 Read `CONTRIBUTING.md` to understand current structure
  - [ ] 7.19 Add "Testing Guidelines" section to CONTRIBUTING.md (TDD, coverage, snapshot tests)
  - [ ] 7.20 Document how to run different test types (unit, integration, live API, benchmarks)
  - [ ] 7.21 Document snapshot test review process (cargo insta review)
  - [ ] 7.22 Create `RELEASE_NOTES_MILESTONE_3.md` file
  - [ ] 7.23 Draft release notes: summarize all Milestone 3 features (Epics 19-24)
  - [ ] 7.24 List breaking changes (if any) in release notes
  - [ ] 7.25 List deprecations (if any) in release notes
  - [ ] 7.26 Add migration guide to release notes (how to upgrade from Milestone 2)
  - [ ] 7.27 Run `cargo doc --open` to verify all documentation generates cleanly
  - [ ] 7.28 Check for any rustdoc warnings and fix them
  - [ ] 7.29 Verify all public APIs have documentation with examples
  - [ ] 7.30 Commit changes: `git commit -m "docs: update documentation and add demo assets for Milestone 3"`

- [ ] 8.0 Final Quality Verification and CI/CD Integration
  - [ ] 8.1 Run full test suite: `cargo test` (should pass without ignored tests)
  - [ ] 8.2 Run benchmark compilation check: `cargo bench --no-run`
  - [ ] 8.3 Run formatting check: `cargo fmt --check`
  - [ ] 8.4 Run linter with warnings as errors: `cargo clippy -- -D warnings`
  - [ ] 8.5 Run `cargo check` to verify compilation
  - [ ] 8.6 Generate coverage report and verify ≥80% unit, ≥70% integration
  - [ ] 8.7 Run `make clean-code` (if available) to run all quality gates
  - [ ] 8.8 Review all changed files to ensure no debug code, println!, or temporary comments
  - [ ] 8.9 Read `.github/workflows/` files to understand CI structure
  - [ ] 8.10 Add CI job for CLI tests (if not already present)
  - [ ] 8.11 Add CI job for benchmark compilation check
  - [ ] 8.12 Update CI to run coverage reporting on PRs
  - [ ] 8.13 Verify CI configuration syntax is correct
  - [ ] 8.14 Run entire test suite one final time: `cargo test --all-features`
  - [ ] 8.15 Stage all changes: `git add .`
  - [ ] 8.16 Run pre-commit checks (if configured)
  - [ ] 8.17 Final commit: `git commit -m "feat: complete Epic 24 - Test Hardening, Benchmarks & QA" -m "- Fixed Campaign and ChainOfCommand benchmarks" -m "- Enabled all deferred test modules" -m "- Implemented Qdrant integration tests" -m "- Added CLI snapshot testing infrastructure" -m "- Implemented live API tests (feature-gated)" -m "- Improved test coverage for deferred modules" -m "- Updated all documentation and created demo assets" -m "- Integrated quality checks into CI/CD"`
  - [ ] 8.18 Push branch to remote: `git push -u origin epic-24/test-hardening-benchmarks-qa`
  - [ ] 8.19 Create Pull Request targeting develop branch
  - [ ] 8.20 Verify CI passes on PR
  - [ ] 8.21 Request code review
  - [ ] 8.22 Address review feedback if any
  - [ ] 8.23 Merge PR after approval
  - [ ] 8.24 Delete feature branch after successful merge
  - [ ] 8.25 Verify develop branch CI passes after merge
  - [ ] 8.26 Tag release: `git tag -a v3.0.0-milestone3 -m "Milestone 3 Complete: Test Hardening, Benchmarks & QA"`
  - [ ] 8.27 Push tag: `git push origin v3.0.0-milestone3`
