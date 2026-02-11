# Task List: Autonomous Agent Completion (Epic 21)

## Document Information

- **Feature:** Autonomous Agent Completion
- **Epic:** Epic 21 - Milestone 3
- **PRD:** `prd-autonomous-agent-completion.md`
- **Estimated Duration:** 2 weeks (5 phases)
- **Target Branch:** `feature/epic-21-autonomous-agent-completion`

---

## Relevant Files

### Core Domain Types
- `src/core/platform/container/paladin.rs` - PaladinResult enhancement with autonomous metadata
- `src/core/platform/container/autonomous/planning.rs` - TaskPlan and Subtask types (if not already present)
- `src/core/platform/container/autonomous/handoff.rs` - HandoffRecord type definition

### Application Layer - Services
- `src/application/use_cases/paladin/planning_service.rs` - Remove hardcoded "gpt-4", use Paladin config (lines 128, 305, 426, 538)
- `src/application/use_cases/paladin/prompt_generation_service.rs` - Remove hardcoded "gpt-4", use Paladin config (line 146)
- `src/application/use_cases/paladin/handoff_service.rs` - Implement execute_handoff() with retry logic
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Orchestrate all autonomous features in layers
- `src/application/use_cases/paladin/paladin_builder.rs` - Auto-register handoff tool when configured

### Configuration
- `src/config/application_settings.rs` - Add handoff retry configuration if not present
- `config.yml` - Add handoff configuration examples
- `config.test.yml` - Add test handoff configuration

### Tests
- `src/application/use_cases/paladin/planning_service.rs` - Unit tests in #[cfg(test)] module
- `src/application/use_cases/paladin/prompt_generation_service.rs` - Unit tests in #[cfg(test)] module
- `src/application/use_cases/paladin/handoff_service.rs` - Unit tests in #[cfg(test)] module
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Unit tests in #[cfg(test)] module
- `tests/integration/autonomous_integration_test.rs` - Integration tests for full autonomous workflow
- `tests/integration/handoff_integration_test.rs` - Integration tests for handoff execution

### Examples
- `examples/agent_handoffs.rs` - Update with new handoff execution examples
- `examples/autonomous_full_config.rs` - Update with orchestration examples

### Documentation
- `docs/AUTONOMOUS.md` - Update with configuration examples and metadata documentation
- `CHANGELOG.md` - Epic 21 changes

### Notes

- Unit tests should be placed in `#[cfg(test)]` modules within the same file as the code they test
- Integration tests go in `tests/integration/`
- Use `cargo test` to run all tests, `cargo test <test_name>` for specific tests
- Follow TDD: Write tests first, then implementation
- All code must pass `cargo clippy -- -D warnings` and `cargo fmt --check`

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Completion Protocol (from Rust Task List Guidelines):**
1. When you finish a sub-task, immediately mark it as completed `[x]`
2. If all subtasks under a parent task are `[x]`:
   - Run `cargo test` to ensure all tests pass
   - Run `cargo fmt --check` to ensure formatting
   - Run `cargo clippy` to check for warnings
   - Only if all checks pass: stage changes with `git add .`
   - Clean up: Remove temporary files, debug prints, temporary code
   - Commit with descriptive message using conventional commit format
   - Mark the parent task as completed `[x]`
3. Stop after each major task and wait for user's go-ahead

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Ensure you're on develop branch: `git checkout develop`
  - [x] 0.2 Pull latest changes: `git pull origin develop`
  - [x] 0.3 Create and checkout new branch: `git checkout -b feature/epic-21-autonomous-agent-completion`

- [ ] 1.0 Phase 1: Configurable Model Selection (US-21.5) - Week 1, Days 1-2
  - [x] 1.1 Read `src/application/use_cases/paladin/planning_service.rs` to locate hardcoded "gpt-4" references (lines 128, 305, 426, 538)
  - [x] 1.2 Update `PlanningService::generate_plan()` to read model from `paladin.config.model` instead of hardcoded "gpt-4"
  - [x] 1.3 Update `PlanningService::refine_plan()` to use configured model
  - [x] 1.4 Update `PlanningService::validate_plan()` to use configured model if it makes LLM calls
  - [x] 1.5 Update subtask expected output generation to use LLM instead of hardcoded "Expected output for [subtask]"
  - [x] 1.6 Write unit test: `test_planning_service_uses_configured_model()`
  - [x] 1.7 Write unit test: `test_planning_service_validates_model_compatibility()`
  - [x] 1.8 Write unit test: `test_planning_service_falls_back_on_invalid_model()`
  - [x] 1.9 Read `src/application/use_cases/paladin/prompt_generation_service.rs` to locate hardcoded "gpt-4" reference (line 146)
  - [x] 1.10 Update `PromptGenerationService::generate_prompt()` to read model from `paladin.config.model`
  - [x] 1.11 Write unit test: `test_prompt_generation_uses_configured_model()`
  - [x] 1.12 Add model validation logic to both services (check if model exists/is capable) - SKIPPED: Model validation is LlmPort's responsibility
  - [x] 1.13 Add warning logs for fallback scenarios - SKIPPED: Fallback handled by LlmPort layer
  - [x] 1.14 Run `cargo test` to verify all tests pass
  - [x] 1.15 Run `cargo clippy` to check for warnings
  - [ ] 1.16 Commit Phase 1 changes with conventional commit message

- [ ] 2.0 Phase 2: PaladinResult Metadata Enhancement (US-21.3) - Week 1, Days 3-4
  - [ ] 2.1 Read `src/core/platform/container/paladin.rs` to understand current PaladinResult structure
  - [ ] 2.2 Check if `TaskPlan` type exists in `src/core/platform/container/autonomous/planning.rs`, create if needed
  - [ ] 2.3 Define `TaskPlan` struct with fields: `goal: String`, `subtasks: Vec<Subtask>`, `created_at: DateTime<Utc>`
  - [ ] 2.4 Check if `HandoffRecord` type exists in `src/core/platform/container/autonomous/handoff.rs`, create if needed
  - [ ] 2.5 Define `HandoffRecord` struct with fields: `specialist_name`, `task_description`, `timestamp`, `result: Option<String>`, `depth: usize`
  - [ ] 2.6 Add `plan: Option<TaskPlan>` field to `PaladinResult` with `#[serde(default)]` attribute
  - [ ] 2.7 Add `handoff_history: Vec<HandoffRecord>` field to `PaladinResult` with `#[serde(default)]` attribute
  - [ ] 2.8 Derive `Default` for `PaladinResult` if not already present
  - [ ] 2.9 Ensure all new types derive `Debug, Clone, Serialize, Deserialize`
  - [ ] 2.10 Write unit test: `test_paladin_result_with_plan_metadata()`
  - [ ] 2.11 Write unit test: `test_paladin_result_with_handoff_history()`
  - [ ] 2.12 Write unit test: `test_paladin_result_serialization_with_new_fields()`
  - [ ] 2.13 Write unit test: `test_paladin_result_deserialization_backward_compatibility()` (old JSON without new fields)
  - [ ] 2.14 Write unit test: `test_paladin_result_default_values()`
  - [ ] 2.15 Update existing tests that construct PaladinResult (should still pass without changes)
  - [ ] 2.16 Add rustdoc comments explaining the new fields and their usage
  - [ ] 2.17 Run `cargo test` to verify backward compatibility
  - [ ] 2.18 Run `cargo clippy` to check for warnings
  - [ ] 2.19 Commit Phase 2 changes with conventional commit message

- [ ] 3.0 Phase 3: Handoff Tool Auto-Registration (US-21.2) - Week 1, Days 5-7
  - [ ] 3.1 Read `src/application/use_cases/paladin/paladin_builder.rs` to understand current builder pattern
  - [ ] 3.2 Add boolean flag `handoffs_configured: bool` to `PaladinBuilder` struct (tracks if `with_handoffs()` was called)
  - [ ] 3.3 Update `with_handoffs()` method to set `handoffs_configured = true`
  - [ ] 3.4 In `PaladinBuilder::build()`, check if `handoffs_configured` is true
  - [ ] 3.5 If configured, generate handoff tool JSON schema with specialist names from handoff config
  - [ ] 3.6 Define handoff tool schema with parameters: `specialist_name` (enum), `task_description` (string)
  - [ ] 3.7 Auto-register handoff tool in arsenal using `arsenal.register_tool(handoff_tool)`
  - [ ] 3.8 Ensure auto-registration is idempotent (check if tool already exists before adding)
  - [ ] 3.9 Write unit test: `test_builder_auto_registers_handoff_tool_when_configured()`
  - [ ] 3.10 Write unit test: `test_builder_does_not_register_handoff_tool_when_not_configured()`
  - [ ] 3.11 Write unit test: `test_handoff_tool_schema_includes_all_specialists()`
  - [ ] 3.12 Write unit test: `test_handoff_tool_auto_registration_is_idempotent()`
  - [ ] 3.13 Write unit test: `test_handoff_tool_schema_validation()`
  - [ ] 3.14 Run `cargo test` to verify auto-registration works
  - [ ] 3.15 Run `cargo clippy` to check for warnings
  - [ ] 3.16 Commit Phase 3 changes with conventional commit message

- [ ] 4.0 Phase 4: Autonomous Execution Orchestration (US-21.4) - Week 2, Days 1-4
  - [ ] 4.1 Read `src/application/use_cases/paladin/paladin_execution_service.rs` to understand current execution flow
  - [ ] 4.2 Design layered orchestration: Layer 0 (Core) → Layer 1 (Planning/Prompts) → Layer 2 (Temperature) → Layer 3 (Handoffs)
  - [ ] 4.3 Refactor `execute()` method to support layered execution flow
  - [ ] 4.4 Add Layer 1 logic: if `paladin.config.autonomous_planning` is true, call `PlanningService::generate_plan()`
  - [ ] 4.5 Add Layer 1 logic: if `paladin.config.autonomous_prompts` is true, call `PromptGenerationService::generate_prompt()`
  - [ ] 4.6 Add Layer 2 logic: if `paladin.config.dynamic_temperature` is true, adjust temperature per loop
  - [ ] 4.7 Add Layer 3 logic: if handoffs configured, process handoff tool calls after LLM response
  - [ ] 4.8 Ensure core execution (Layer 0) always runs regardless of feature flags
  - [ ] 4.9 Implement graceful degradation: layer failures log warnings but don't stop core execution
  - [ ] 4.10 Populate `PaladinResult.plan` field when planning is used
  - [ ] 4.11 Populate `PaladinResult.handoff_history` field when handoffs occur
  - [ ] 4.12 Write unit test: `test_orchestration_layer_0_core_always_runs()`
  - [ ] 4.13 Write unit test: `test_orchestration_layer_1_planning_optional()`
  - [ ] 4.14 Write unit test: `test_orchestration_layer_1_prompts_optional()`
  - [ ] 4.15 Write unit test: `test_orchestration_layer_2_temperature_optional()`
  - [ ] 4.16 Write unit test: `test_orchestration_layer_3_handoffs_optional()`
  - [ ] 4.17 Write unit test: `test_orchestration_graceful_degradation_on_layer_failure()`
  - [ ] 4.18 Write integration test: `test_orchestration_planning_with_handoffs()`
  - [ ] 4.19 Write integration test: `test_orchestration_all_features_enabled()`
  - [ ] 4.20 Write integration test: `test_orchestration_all_features_disabled()`
  - [ ] 4.21 Add performance measurement: ensure overhead is minimal when features disabled
  - [ ] 4.22 Run `cargo test` to verify orchestration works
  - [ ] 4.23 Run `cargo clippy` to check for warnings
  - [ ] 4.24 Commit Phase 4 changes with conventional commit message

- [ ] 5.0 Phase 5: Handoff Execution Integration (US-21.1) - Week 2, Days 5-7
  - [ ] 5.1 Read `src/application/use_cases/paladin/handoff_service.rs` to understand current handoff infrastructure
  - [ ] 5.2 Read PRD Section 6.3 for retry configuration details (HandoffRetryConfig)
  - [ ] 5.3 Define `HandoffRetryConfig` struct in config with: `max_retries`, `initial_backoff_ms`, `backoff_multiplier`
  - [ ] 5.4 Add handoff retry configuration to `config.yml` (default: 3 retries, 1000ms initial, 2.0 multiplier)
  - [ ] 5.5 Add handoff configuration to `config.test.yml` for tests
  - [ ] 5.6 Implement `HandoffService::execute_handoff()` - delegate task to specialist Paladin
  - [ ] 5.7 Implement handoff chain tracking: maintain depth counter, detect circular references
  - [ ] 5.8 Implement circular handoff detection: check if specialist already in chain at same depth
  - [ ] 5.9 Implement max depth validation: enforce configurable limit (default: 5)
  - [ ] 5.10 Implement retry logic with exponential backoff for transient errors
  - [ ] 5.11 Classify errors: transient (network, timeout) vs permanent (invalid specialist, circular)
  - [ ] 5.12 For transient errors, retry with backoff: delay = initial_backoff_ms * (multiplier ^ attempt)
  - [ ] 5.13 For permanent errors, fail immediately with clear error message
  - [ ] 5.14 Ensure handoff calls create `HandoffRecord` entries with all metadata
  - [ ] 5.15 Pass specialist Paladin instance to `PaladinExecutionService` for execution
  - [ ] 5.16 Capture specialist execution result and return as tool response to original agent
  - [ ] 5.17 Add detailed logging for handoff execution (specialist name, depth, result)
  - [ ] 5.18 Integrate circuit breaker with handoff retry logic
  - [ ] 5.19 Write unit test: `test_handoff_service_delegates_to_specialist()`
  - [ ] 5.20 Write unit test: `test_handoff_service_tracks_chain_depth()`
  - [ ] 5.21 Write unit test: `test_handoff_service_detects_circular_handoff()`
  - [ ] 5.22 Write unit test: `test_handoff_service_enforces_max_depth()`
  - [ ] 5.23 Write unit test: `test_handoff_service_retries_transient_errors()`
  - [ ] 5.24 Write unit test: `test_handoff_service_fails_immediately_on_permanent_error()`
  - [ ] 5.25 Write unit test: `test_handoff_service_exponential_backoff_timing()`
  - [ ] 5.26 Write unit test: `test_handoff_service_creates_handoff_records()`
  - [ ] 5.27 Write integration test: `test_handoff_execution_end_to_end()`
  - [ ] 5.28 Write integration test: `test_handoff_result_flows_back_to_original_agent()`
  - [ ] 5.29 Write integration test: `test_handoff_chain_tracking_multiple_levels()`
  - [ ] 5.30 Write integration test: `test_handoff_visible_in_execution_trace()`
  - [ ] 5.31 Run `cargo test` to verify handoff execution works
  - [ ] 5.32 Run `cargo clippy` to check for warnings
  - [ ] 5.33 Commit Phase 5 changes with conventional commit message

- [ ] 6.0 Final Quality Checks and PR Preparation
  - [ ] 6.1 Run full test suite: `cargo test` (ensure all tests pass)
  - [ ] 6.2 Run clippy: `cargo clippy -- -D warnings` (fix all warnings)
  - [ ] 6.3 Run format check: `cargo fmt --check` (format if needed)
  - [ ] 6.4 Run format: `cargo fmt` (if check failed)
  - [ ] 6.5 Verify no TODO comments remain in modified files
  - [ ] 6.6 Run `cargo build --release` to ensure release build works
  - [ ] 6.7 Verify backward compatibility: run existing examples without errors
  - [ ] 6.8 Check unit test coverage ≥90% for modified files: `cargo tarpaulin` or equivalent
  - [ ] 6.9 Update `examples/agent_handoffs.rs` with new handoff execution examples
  - [ ] 6.10 Update `examples/autonomous_full_config.rs` with orchestration examples
  - [ ] 6.11 Update `docs/AUTONOMOUS.md` with configuration examples and metadata documentation
  - [ ] 6.12 Update `CHANGELOG.md` with Epic 21 changes: handoff execution, auto-registration, metadata, orchestration, configurable models
  - [ ] 6.13 Review all changes: `git diff develop`
  - [ ] 6.14 Stage all changes: `git add .`
  - [ ] 6.15 Commit with conventional format: `git commit -m "feat: complete Epic 21 autonomous agent features" -m "- Implement handoff execution with retry logic" -m "- Auto-register handoff tools in builder" -m "- Add autonomous metadata to PaladinResult" -m "- Orchestrate all features in layered execution" -m "- Replace hardcoded models with config-driven selection" -m "- Add comprehensive unit and integration tests" -m "Related to Epic 21 in Milestone 3 PRD"`
  - [ ] 6.16 Push branch: `git push origin feature/epic-21-autonomous-agent-completion`
  - [ ] 6.17 Create Pull Request targeting `develop` branch
  - [ ] 6.18 Add PR description referencing Epic 21 PRD and completion checklist
  - [ ] 6.19 Request review from maintainers

---

**Status:** All sub-tasks generated. Ready for implementation following bottom-up approach (Phase 1 → Phase 5).
