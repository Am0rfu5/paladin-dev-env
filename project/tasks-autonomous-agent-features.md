# Task List: Epic 14 - Autonomous Agent Features

**Epic:** 14 - Autonomous Agent Features  
**Theme:** Agent Self-Direction and Planning  
**Duration:** 2 weeks (14 days)  
**Priority:** Critical  
**Dependencies:** Epic 1 (Paladin Domain)  
**PRD:** `project/prd-autonomous-agent-features.md`

---

## Relevant Files

### Core Layer (Domain Models)
- `src/core/platform/container/paladin.rs` - Add `MaxLoops::Auto` variant
- `src/core/platform/container/planning.rs` - TaskPlan and Subtask domain types
- `src/core/platform/container/handoff.rs` - HandoffDecision and related domain types
- `src/core/platform/container/autonomous_config.rs` - Configuration structures for autonomous features
- `src/core/error/planning_error.rs` - PlanningError enum
- `src/core/error/prompt_error.rs` - PromptError enum
- `src/core/error/handoff_error.rs` - HandoffError enum

### Application Layer (Services & Ports)
- `src/application/use_cases/paladin/planning_service.rs` - PlanningService implementation
- `src/application/use_cases/paladin/prompt_generation_service.rs` - PromptGenerationService implementation
- `src/application/use_cases/paladin/temperature_service.rs` - TemperatureService implementation
- `src/application/use_cases/paladin/handoff_service.rs` - HandoffService implementation
- `src/application/use_cases/paladin/paladin_builder.rs` - Add autonomous feature builder methods
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Integrate autonomous services

### Arsenal (Tools)
- `src/core/platform/container/arsenal/handoff_tool.rs` - Handoff tool implementation

### Configuration
- `src/config/application_settings.rs` - Add autonomous configuration section
- `config.yml` - Example autonomous configuration

### Documentation
- `docs/AUTONOMOUS.md` - Comprehensive autonomous features guide
- `docs/README.md` - Update with autonomous features link

### Examples
- `examples/autonomous_planning.rs` - Autonomous planning mode example
- `examples/autonomous_prompt_generation.rs` - Auto-prompt generation example
- `examples/dynamic_temperature.rs` - Dynamic temperature adjustment example
- `examples/agent_handoffs.rs` - Agent handoff infrastructure example
- `examples/autonomous_full_config.rs` - Full autonomous configuration example
- `examples/README.md` - Update with autonomous examples

### Tests
- `tests/unit/planning_service_test.rs` - Unit tests for PlanningService
- `tests/unit/prompt_generation_service_test.rs` - Unit tests for PromptGenerationService
- `tests/unit/temperature_service_test.rs` - Unit tests for TemperatureService
- `tests/unit/handoff_service_test.rs` - Unit tests for HandoffService
- `tests/integration/autonomous_planning_test.rs` - Integration tests for planning workflows
- `tests/integration/autonomous_handoff_test.rs` - Integration tests for handoff scenarios
- `tests/integration/autonomous_full_test.rs` - End-to-end autonomous feature tests

### Other
- `CHANGELOG.md` - Epic 14 changelog entry

### Notes
- All tests follow TDD principles: write test first, then implementation
- Unit tests use mocked LLM responses for deterministic testing
- Integration tests use test LLM adapters with realistic scenarios
- All public APIs must have rustdoc comments
- Error handling uses `thiserror` for custom error types
- All autonomous features are opt-in via configuration flags

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

**Completion Protocol:**
1. When you finish a **sub-task**, immediately mark it as completed by changing `[ ]` to `[x]`
2. If **all** subtasks underneath a parent task are now `[x]`, follow this sequence:
   - **First**: Run the full test suite with `cargo test`
   - **Check formatting**: Run `cargo fmt --check`
   - **Run linter**: Run `cargo clippy -- -D warnings`
   - **Only if all tests pass and checks succeed**: Stage changes (`git add .`)
   - **Clean up**: Remove any temporary files, debug prints, and temporary code
   - **Commit**: Use conventional commit format with descriptive message
3. Once all the subtasks are marked completed and changes have been committed, mark the **parent task** as completed

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout new branch `feature/epic-14-autonomous-agent-features`
  - [x] 0.2 Push branch to remote: `git push -u origin feature/epic-14-autonomous-agent-features` (skipped - working locally)
  - [x] 0.3 Verify branch is active: `git branch --show-current`

- [x] 1.0 Foundation - Domain Models & Configuration
  - [x] 1.1 **TEST**: Write test for `MaxLoops::Auto` variant in `paladin.rs`
  - [x] 1.2 Add `MaxLoops::Auto { max_subtasks: u32 }` variant to `src/core/platform/container/paladin.rs`
  - [x] 1.3 **TEST**: Write tests for `TaskPlan` and `Subtask` domain types
  - [x] 1.4 Create `src/core/platform/container/planning.rs` with `TaskPlan`, `Subtask`, and `TaskDependency` structs
  - [x] 1.5 **TEST**: Write tests for `HandoffDecision` domain type
  - [x] 1.6 Create `src/core/platform/container/handoff.rs` with `HandoffDecision`, `HandoffStrategy`, and `HandoffContext` structs
  - [x] 1.7 **TEST**: Write tests for `PlanningError` enum
  - [x] 1.8 Create `src/core/error/planning_error.rs` with error variants (InvalidPlan, MaxSubtasksExceeded, ExecutionFailed, etc.)
  - [x] 1.9 **TEST**: Write tests for `PromptError` enum
  - [x] 1.10 Create `src/core/error/prompt_error.rs` with error variants (GenerationFailed, InvalidDescription, CacheMiss, etc.)
  - [x] 1.11 **TEST**: Write tests for `HandoffError` enum
  - [x] 1.12 Create `src/core/error/handoff_error.rs` with error variants (InvalidAgent, CircularHandoff, MaxDepthExceeded, etc.)
  - [x] 1.13 **TEST**: Write tests for `AutonomousConfig` structure
  - [x] 1.14 Create `src/core/platform/container/autonomous_config.rs` with config structs (PlanningConfig, PromptConfig, TemperatureConfig, HandoffConfig)
  - [x] 1.15 Add module declarations to `src/core/platform/container/mod.rs`
  - [x] 1.16 Add module declarations to `src/core/error/mod.rs`
  - [x] 1.17 Run tests: `cargo test --lib`
  - [x] 1.18 Run clippy: `cargo clippy -- -D warnings`
  - [x] 1.19 Format code: `cargo fmt`
  - [x] 1.20 Commit: `git commit -m "feat(core): add autonomous feature domain models" -m "- Add MaxLoops::Auto variant" -m "- Create TaskPlan, Subtask, HandoffDecision types" -m "- Create error enums for planning, prompts, handoffs" -m "- Add AutonomousConfig structures" -m "Related to US-14.1, US-14.2, US-14.3, US-14.4, US-14.5"`

- [ ] 2.0 Planning Service (US-14.1: Autonomous Planning Mode)
  - [x] 2.1 **TEST**: Write test for `PlanningService::new()` constructor
  - [x] 2.2 Create `src/application/use_cases/paladin/planning_service.rs` with struct and constructor
  - [x] 2.3 **TEST**: Write test for LLM-based task decomposition (mocked LLM response)
  - [x] 2.4 Implement `PlanningService::create_plan()` method that calls LLM with planning prompt template
  - [x] 2.5 **TEST**: Write test for parsing LLM response into TaskPlan
  - [x] 2.6 Implement `PlanningService::parse_plan_from_llm()` to extract subtasks from LLM JSON response
  - [x] 2.7 **TEST**: Write test for subtask execution with dependency tracking
  - [x] 2.8 Implement `PlanningService::execute_subtasks()` with sequential execution and dependency resolution
  - [x] 2.9 **TEST**: Write test for result synthesis
  - [x] 2.10 Implement `PlanningService::synthesize_results()` to combine subtask outputs
  - [x] 2.11 **TEST**: Write test for max_subtasks limit enforcement
  - [x] 2.12 Add validation in `create_plan()` to enforce `max_subtasks` limit
  - [x] 2.13 **TEST**: Write test for planning failure scenarios (LLM error, invalid plan)
  - [x] 2.14 Add error handling with `PlanningError` enum
  - [x] 2.15 **TEST**: Write test for planning logging
  - [x] 2.16 Add `tracing::info!` logging for planning decisions and execution progress
  - [x] 2.17 **TEST**: Write integration test for full planning workflow (plan → execute → synthesize)
  - [x] 2.18 Create `tests/integration/autonomous_planning_test.rs` with end-to-end planning scenarios
  - [x] 2.19 Add module declaration to `src/application/use_cases/paladin/mod.rs`
  - [x] 2.20 Run tests: `cargo test planning_service`
  - [x] 2.21 Run clippy: `cargo clippy -- -D warnings`
  - [x] 2.22 Format code: `cargo fmt`
  - [ ] 2.23 Commit: `git commit -m "feat(planning): implement PlanningService for autonomous task decomposition" -m "- LLM-based task decomposition with planning prompt" -m "- Subtask execution with dependency tracking" -m "- Result synthesis for cohesive responses" -m "- Max subtasks limit enforcement" -m "- Comprehensive logging and error handling" -m "Implements US-14.1"`

- [ ] 3.0 Prompt Generation Service (US-14.2: Auto-Generate System Prompt)
  - [ ] 3.1 **TEST**: Write test for `PromptGenerationService::new()` constructor
  - [ ] 3.2 Create `src/application/use_cases/paladin/prompt_generation_service.rs` with struct and constructor
  - [ ] 3.3 **TEST**: Write test for LLM-based prompt generation (mocked LLM response)
  - [ ] 3.4 Implement `PromptGenerationService::generate_prompt()` method with prompt generation template
  - [ ] 3.5 **TEST**: Write test for prompt caching mechanism
  - [ ] 3.6 Implement prompt caching using `HashMap<String, String>` keyed by agent name + description
  - [ ] 3.7 **TEST**: Write test for deterministic generation (same inputs → same prompt)
  - [ ] 3.8 Add deterministic prompt generation with fixed LLM parameters (temperature=0.0)
  - [ ] 3.9 **TEST**: Write test for `PaladinBuilder::auto_generate_prompt()` method
  - [ ] 3.10 Add `auto_generate_prompt(bool)` method to `src/application/use_cases/paladin/paladin_builder.rs`
  - [ ] 3.11 **TEST**: Write test for `PaladinBuilder::agent_description()` method
  - [ ] 3.12 Add `agent_description(impl Into<String>)` method to `PaladinBuilder`
  - [ ] 3.13 **TEST**: Write test for `PaladinBuilder::regenerate_prompt()` method
  - [ ] 3.14 Add `regenerate_prompt()` method to invalidate cache and force regeneration
  - [ ] 3.15 **TEST**: Write test for manual override (system_prompt() called after auto_generate_prompt())
  - [ ] 3.16 Implement override logic in builder where manual prompt takes precedence
  - [ ] 3.17 **TEST**: Write test for prompt generation logging
  - [ ] 3.18 Add `tracing::info!` logging for generated prompts (with full content at DEBUG level)
  - [ ] 3.19 **TEST**: Write test for error handling (LLM failure, empty description)
  - [ ] 3.20 Add error handling with `PromptError` enum
  - [ ] 3.21 Add module declaration to `src/application/use_cases/paladin/mod.rs`
  - [ ] 3.22 Run tests: `cargo test prompt_generation`
  - [ ] 3.23 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 3.24 Format code: `cargo fmt`
  - [ ] 3.25 Commit: `git commit -m "feat(prompts): implement PromptGenerationService for auto-prompt generation" -m "- LLM-based contextual prompt generation" -m "- Prompt caching for performance" -m "- Builder methods: auto_generate_prompt, agent_description, regenerate_prompt" -m "- Manual override support" -m "- Deterministic generation for testing" -m "Implements US-14.2"`

- [ ] 4.0 Temperature Service (US-14.3: Dynamic Temperature Adjustment)
  - [ ] 4.1 **TEST**: Write test for `TemperatureService::new()` constructor
  - [ ] 4.2 Create `src/application/use_cases/paladin/temperature_service.rs` with struct and constructor
  - [ ] 4.3 **TEST**: Write test for task classification (Factual, Analytical, Conversational, Creative)
  - [ ] 4.4 Implement `TemperatureService::classify_task()` using heuristic analysis (keywords, question patterns)
  - [ ] 4.5 **TEST**: Write test for temperature recommendations by task type
  - [ ] 4.6 Implement `TemperatureService::recommend_temperature()` with ranges: Factual (0.1-0.3), Analytical (0.3-0.5), Conversational (0.5-0.7), Creative (0.7-1.0)
  - [ ] 4.7 **TEST**: Write test for temperature bounds enforcement
  - [ ] 4.8 Add bounds validation in `recommend_temperature()` to respect min/max limits
  - [ ] 4.9 **TEST**: Write test for `PaladinBuilder::dynamic_temperature()` method
  - [ ] 4.10 Add `dynamic_temperature(bool)` method to `src/application/use_cases/paladin/paladin_builder.rs`
  - [ ] 4.11 **TEST**: Write test for `PaladinBuilder::temperature_bounds()` method
  - [ ] 4.12 Add `temperature_bounds(f32, f32)` method to `PaladinBuilder` with validation (min < max, range 0.0-1.0)
  - [ ] 4.13 **TEST**: Write test for fallback to builder temperature when disabled
  - [ ] 4.14 Implement fallback logic: if `dynamic_temperature == false`, use `PaladinBuilder::temperature()` value
  - [ ] 4.15 **TEST**: Write test for temperature decision logging
  - [ ] 4.16 Add `tracing::info!` logging for temperature decisions with task classification reasoning
  - [ ] 4.17 **TEST**: Write test for edge cases (empty task, ambiguous classification)
  - [ ] 4.18 Add error handling and default classification (Conversational) for ambiguous cases
  - [ ] 4.19 Add module declaration to `src/application/use_cases/paladin/mod.rs`
  - [ ] 4.20 Run tests: `cargo test temperature_service`
  - [ ] 4.21 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 4.22 Format code: `cargo fmt`
  - [ ] 4.23 Commit: `git commit -m "feat(temperature): implement TemperatureService for dynamic adjustment" -m "- Heuristic-based task classification (Factual/Analytical/Conversational/Creative)" -m "- Temperature recommendations by task type" -m "- Temperature bounds enforcement" -m "- Builder methods: dynamic_temperature, temperature_bounds" -m "- Fallback to builder temperature when disabled" -m "Implements US-14.3"`

- [ ] 5.0 Handoff Infrastructure (US-14.4: Agent Handoff Infrastructure)
  - [ ] 5.1 **TEST**: Write test for `HandoffService::new()` constructor
  - [ ] 5.2 Create `src/application/use_cases/paladin/handoff_service.rs` with struct and constructor
  - [ ] 5.3 **TEST**: Write test for `HandoffStrategy` enum (Automatic, Explicit, Threshold)
  - [ ] 5.4 Update `src/core/platform/container/handoff.rs` to include `HandoffStrategy` enum variants
  - [ ] 5.5 **TEST**: Write test for handoff decision logic (when to handoff vs execute locally)
  - [ ] 5.6 Implement `HandoffService::should_handoff()` based on strategy and confidence
  - [ ] 5.7 **TEST**: Write test for agent selection (choosing appropriate specialist)
  - [ ] 5.8 Implement `HandoffService::select_agent()` to match task with agent capabilities
  - [ ] 5.9 **TEST**: Write test for handoff chain tracking
  - [ ] 5.10 Implement `HandoffContext::chain` field with `Vec<String>` of agent names in execution path
  - [ ] 5.11 **TEST**: Write test for circular delegation prevention
  - [ ] 5.12 Implement circular handoff check in `should_handoff()` - reject if target agent in chain
  - [ ] 5.13 **TEST**: Write test for max depth enforcement (default: 5)
  - [ ] 5.14 Add depth validation in `should_handoff()` to enforce configurable max_depth limit
  - [ ] 5.15 **TEST**: Write test for context transfer mechanism
  - [ ] 5.16 Implement `HandoffService::transfer_context()` to create `HandoffContext` with task, history, metadata
  - [ ] 5.17 **TEST**: Write test for handoff execution
  - [ ] 5.18 Implement `HandoffService::execute_handoff()` to delegate task to specialist agent
  - [ ] 5.19 **TEST**: Write test for `PaladinBuilder::with_handoffs()` method
  - [ ] 5.20 Add `with_handoffs(Vec<Arc<Paladin>>)` method to `src/application/use_cases/paladin/paladin_builder.rs`
  - [ ] 5.21 **TEST**: Write test for `PaladinBuilder::handoff_strategy()` method
  - [ ] 5.22 Add `handoff_strategy(HandoffStrategy)` method to `PaladinBuilder`
  - [ ] 5.23 **TEST**: Write test for handoff history in `PaladinResult`
  - [ ] 5.24 Update `PaladinResult` struct to include `handoff_history: Vec<HandoffRecord>` field
  - [ ] 5.25 **TEST**: Write test for handoff decision logging
  - [ ] 5.26 Add `tracing::info!` logging for all handoff decisions with reasoning
  - [ ] 5.27 **TEST**: Write integration test for full handoff workflow (decision → transfer → execute → return)
  - [ ] 5.28 Create `tests/integration/autonomous_handoff_test.rs` with multi-agent handoff scenarios
  - [ ] 5.29 **TEST**: Write test for error scenarios (invalid agent, circular handoff, depth exceeded)
  - [ ] 5.30 Add error handling with `HandoffError` enum for all failure cases
  - [ ] 5.31 Add module declaration to `src/application/use_cases/paladin/mod.rs`
  - [ ] 5.32 Run tests: `cargo test handoff_service`
  - [ ] 5.33 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 5.34 Format code: `cargo fmt`
  - [ ] 5.35 Commit: `git commit -m "feat(handoff): implement HandoffService for agent delegation" -m "- Handoff decision logic with strategy support (Automatic/Explicit/Threshold)" -m "- Agent selection based on capabilities" -m "- Handoff chain tracking and circular delegation prevention" -m "- Max depth enforcement (default: 5)" -m "- Context transfer mechanism with task, history, metadata" -m "- Builder methods: with_handoffs, handoff_strategy" -m "- Handoff history in PaladinResult" -m "Implements US-14.4"`

- [ ] 6.0 Handoff Tool (US-14.5: Handoff Tool for Agents)
  - [ ] 6.1 **TEST**: Write test for handoff tool schema generation
  - [ ] 6.2 Create `src/core/platform/container/arsenal/handoff_tool.rs` with tool schema struct
  - [ ] 6.3 **TEST**: Write test for tool schema includes agent_name enum and message
  - [ ] 6.4 Implement `HandoffTool::schema()` to generate OpenAI function schema with agent_name enum from available agents
  - [ ] 6.5 **TEST**: Write test for auto-registration in PaladinBuilder when handoffs configured
  - [ ] 6.6 Add auto-registration logic in `PaladinBuilder::build()` to register handoff tool when `with_handoffs()` called
  - [ ] 6.7 **TEST**: Write test for agent_name validation against available agents
  - [ ] 6.8 Implement `HandoffTool::validate_agent()` to check agent_name in registered agents list
  - [ ] 6.9 **TEST**: Write test for tool execution via HandoffService
  - [ ] 6.10 Implement `HandoffTool::execute()` that delegates to `HandoffService::execute_handoff()`
  - [ ] 6.11 **TEST**: Write test for specialist result return to original agent
  - [ ] 6.12 Implement result flow: specialist executes → result returned → original agent continues
  - [ ] 6.13 **TEST**: Write test for handoff chain tracking across tool invocations
  - [ ] 6.14 Update tool execution to maintain and update handoff chain in context
  - [ ] 6.15 **TEST**: Write test for error handling (invalid agent_name)
  - [ ] 6.16 Add error handling for invalid agent with `HandoffError::InvalidAgent`
  - [ ] 6.17 **TEST**: Write test for circular handoff error
  - [ ] 6.18 Add validation in tool execution to detect and reject circular handoffs
  - [ ] 6.19 **TEST**: Write test for max depth exceeded error
  - [ ] 6.20 Add validation for handoff depth limit with `HandoffError::MaxDepthExceeded`
  - [ ] 6.21 **TEST**: Write test for tool visibility in execution trace
  - [ ] 6.22 Update execution trace to include handoff tool calls with agent_name and message
  - [ ] 6.23 Add module declaration to `src/core/platform/container/arsenal/mod.rs`
  - [ ] 6.24 Run tests: `cargo test handoff_tool`
  - [ ] 6.25 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 6.26 Format code: `cargo fmt`
  - [ ] 6.27 Commit: `git commit -m "feat(handoff): implement handoff_to_agent tool for mid-execution delegation" -m "- Tool schema with agent_name enum and message parameters" -m "- Auto-registration in PaladinBuilder when handoffs configured" -m "- Agent validation and execution via HandoffService" -m "- Handoff chain tracking across tool invocations" -m "- Error handling for invalid agent, circular handoffs, depth exceeded" -m "- Tool visibility in execution trace" -m "Implements US-14.5"`

- [ ] 7.0 Integration with PaladinExecutionService
  - [ ] 7.1 **TEST**: Write test for PlanningService integration in execution flow
  - [ ] 7.2 Update `src/application/use_cases/paladin/paladin_execution_service.rs` to detect `MaxLoops::Auto` and delegate to `PlanningService`
  - [ ] 7.3 **TEST**: Write test for PromptGenerationService integration in builder
  - [ ] 7.4 Update `PaladinBuilder::build()` to call `PromptGenerationService` when `auto_generate_prompt == true`
  - [ ] 7.5 **TEST**: Write test for TemperatureService integration in execution
  - [ ] 7.6 Update `PaladinExecutionService::execute()` to call `TemperatureService` when `dynamic_temperature == true`
  - [ ] 7.7 **TEST**: Write test for HandoffService integration in execution
  - [ ] 7.8 Update `PaladinExecutionService::execute()` to check for handoff decisions when handoffs configured
  - [ ] 7.9 **TEST**: Write test for handoff tool registration
  - [ ] 7.10 Update `PaladinBuilder::build()` to auto-register handoff tool when `with_handoffs()` called
  - [ ] 7.11 **TEST**: Write test for PaladinResult includes planning metadata
  - [ ] 7.12 Update `PaladinResult` to include `plan: Option<TaskPlan>` field
  - [ ] 7.13 **TEST**: Write test for PaladinResult includes handoff metadata
  - [ ] 7.14 Ensure `PaladinResult` includes `handoff_history: Vec<HandoffRecord>` (from Task 5.0)
  - [ ] 7.15 **TEST**: Write test for orchestration logic - planning + prompts + temperature + handoffs
  - [ ] 7.16 Implement orchestration in `PaladinExecutionService` to coordinate all autonomous features
  - [ ] 7.17 **TEST**: Write integration test for full autonomous workflow
  - [ ] 7.18 Create `tests/integration/autonomous_full_test.rs` with end-to-end scenarios using all features
  - [ ] 7.19 **TEST**: Write test for feature interaction edge cases (e.g., planning with handoffs)
  - [ ] 7.20 Add tests for complex scenarios where multiple autonomous features interact
  - [ ] 7.21 Run tests: `cargo test paladin_execution_service`
  - [ ] 7.22 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 7.23 Format code: `cargo fmt`
  - [ ] 7.24 Commit: `git commit -m "feat(integration): integrate autonomous services into PaladinExecutionService" -m "- PlanningService integration for MaxLoops::Auto execution" -m "- PromptGenerationService integration in builder" -m "- TemperatureService integration in execution" -m "- HandoffService integration for delegation decisions" -m "- Handoff tool auto-registration" -m "- PaladinResult enhanced with planning and handoff metadata" -m "- Orchestration logic for coordinated autonomous behavior"`

- [ ] 8.0 YAML & CLI Configuration Support
  - [ ] 8.1 **TEST**: Write test for autonomous section in PaladinConfig
  - [ ] 8.2 Add `autonomous: Option<AutonomousConfig>` field to `PaladinConfig` in `src/config/application_settings.rs`
  - [ ] 8.3 **TEST**: Write test for YAML parsing of planning configuration
  - [ ] 8.4 Implement YAML deserialization for `planning` section (enabled, max_subtasks)
  - [ ] 8.5 **TEST**: Write test for YAML parsing of prompt_generation configuration
  - [ ] 8.6 Implement YAML deserialization for `prompt_generation` section (enabled, description)
  - [ ] 8.7 **TEST**: Write test for YAML parsing of dynamic_temperature configuration
  - [ ] 8.8 Implement YAML deserialization for `dynamic_temperature` section (enabled, min, max)
  - [ ] 8.9 **TEST**: Write test for YAML parsing of handoffs configuration
  - [ ] 8.10 Implement YAML deserialization for `handoffs` section (enabled, strategy, max_depth, specialists)
  - [ ] 8.11 **TEST**: Write test for CLI flags for autonomous features
  - [ ] 8.12 Add CLI flags to `src/bin/paladin-cli.rs`: `--auto-plan`, `--auto-prompt`, `--dynamic-temp`, `--enable-handoffs`
  - [ ] 8.13 **TEST**: Write test for configuration validation (bounds, required fields)
  - [ ] 8.14 Implement `AutonomousConfig::validate()` to check temperature bounds, max_subtasks > 0, etc.
  - [ ] 8.15 **TEST**: Write test for opt-in defaults (all features disabled by default)
  - [ ] 8.16 Set default values: `enabled: false` for all autonomous features in config structs
  - [ ] 8.17 **TEST**: Write test for configuration override priority (CLI > YAML > defaults)
  - [ ] 8.18 Implement configuration merging logic with proper precedence
  - [ ] 8.19 Update `config.yml` with example autonomous configuration section (commented out)
  - [ ] 8.20 Run tests: `cargo test config`
  - [ ] 8.21 Run clippy: `cargo clippy -- -D warnings`
  - [ ] 8.22 Format code: `cargo fmt`
  - [ ] 8.23 Commit: `git commit -m "feat(config): add YAML and CLI support for autonomous features" -m "- Autonomous section in PaladinConfig with all feature configs" -m "- YAML deserialization for planning, prompts, temperature, handoffs" -m "- CLI flags: --auto-plan, --auto-prompt, --dynamic-temp, --enable-handoffs" -m "- Configuration validation with bounds checking" -m "- Opt-in defaults (all features disabled)" -m "- Example config.yml section"`

- [ ] 9.0 Documentation (AUTONOMOUS.md)
  - [ ] 9.1 Create `docs/AUTONOMOUS.md` with document structure (11 sections minimum)
  - [ ] 9.2 Write Introduction section explaining autonomous features overview and benefits
  - [ ] 9.3 Write Autonomous Planning Mode section (US-14.1) with:
    - [ ] 9.3.1 Concept explanation and use cases
    - [ ] 9.3.2 Code example using `MaxLoops::Auto`
    - [ ] 9.3.3 Configuration options (max_subtasks)
    - [ ] 9.3.4 How it works (planning prompt, decomposition, execution, synthesis)
  - [ ] 9.4 Write Auto-Generate System Prompt section (US-14.2) with:
    - [ ] 9.4.1 Concept explanation and benefits
    - [ ] 9.4.2 Code example with `auto_generate_prompt()` and `agent_description()`
    - [ ] 9.4.3 Regeneration example with `regenerate_prompt()`
    - [ ] 9.4.4 Manual override pattern
    - [ ] 9.4.5 Caching behavior explanation
  - [ ] 9.5 Write Dynamic Temperature Adjustment section (US-14.3) with:
    - [ ] 9.5.1 Concept explanation and task type classification
    - [ ] 9.5.2 Temperature ranges table (Factual/Analytical/Conversational/Creative)
    - [ ] 9.5.3 Code example with `dynamic_temperature()` and `temperature_bounds()`
    - [ ] 9.5.4 Classification heuristics explanation
  - [ ] 9.6 Write Agent Handoff Infrastructure section (US-14.4) with:
    - [ ] 9.6.1 Concept explanation and delegation patterns
    - [ ] 9.6.2 Code example with `with_handoffs()` and `handoff_strategy()`
    - [ ] 9.6.3 HandoffStrategy options (Automatic/Explicit/Threshold) explanation
    - [ ] 9.6.4 Circular handoff prevention explanation
    - [ ] 9.6.5 Max depth configuration
    - [ ] 9.6.6 Context transfer details
  - [ ] 9.7 Write Handoff Tool section (US-14.5) with:
    - [ ] 9.7.1 Tool schema documentation
    - [ ] 9.7.2 Example LLM tool call JSON
    - [ ] 9.7.3 Auto-registration behavior
    - [ ] 9.7.4 Error scenarios (invalid agent, circular, depth exceeded)
  - [ ] 9.8 Write Configuration section with:
    - [ ] 9.8.1 YAML configuration example (all features)
    - [ ] 9.8.2 CLI flags documentation
    - [ ] 9.8.3 Builder API reference
    - [ ] 9.8.4 Configuration precedence explanation
  - [ ] 9.9 Write Best Practices section with:
    - [ ] 9.9.1 When to use each autonomous feature
    - [ ] 9.9.2 Performance considerations
    - [ ] 9.9.3 Token budget management
    - [ ] 9.9.4 Combining features effectively
  - [ ] 9.10 Write Error Handling section with:
    - [ ] 9.10.1 PlanningError variants and handling
    - [ ] 9.10.2 PromptError variants and handling
    - [ ] 9.10.3 HandoffError variants and handling
    - [ ] 9.10.4 Graceful degradation patterns
  - [ ] 9.11 Write Troubleshooting Guide section with:
    - [ ] 9.11.1 Common issues and solutions
    - [ ] 9.11.2 Debugging tips (logging, tracing)
    - [ ] 9.11.3 Performance optimization tips
  - [ ] 9.12 Write Advanced Usage section with:
    - [ ] 9.12.1 Combining autonomous features
    - [ ] 9.12.2 Custom agent configurations
    - [ ] 9.12.3 Integration with Battalion patterns
  - [ ] 9.13 Write API Reference section with all public types and methods documented
  - [ ] 9.14 Add code examples to each section (inline or linked to examples/)
  - [ ] 9.15 Update `docs/README.md` to include link to AUTONOMOUS.md
  - [ ] 9.16 Verify all links work and all code examples compile
  - [ ] 9.17 Run spell check and grammar review
  - [ ] 9.18 Commit: `git commit -m "docs: create comprehensive AUTONOMOUS.md guide" -m "- Introduction and autonomous features overview" -m "- Documentation for all 5 user stories with examples" -m "- Configuration guide (YAML, CLI, Builder)" -m "- Best practices and performance considerations" -m "- Error handling and troubleshooting" -m "- Advanced usage and API reference"`

- [ ] 10.0 Examples Implementation
  - [ ] 10.1 **Create** `examples/autonomous_planning.rs` with:
    - [ ] 10.1.1 Basic planning mode example (MaxLoops::Auto)
    - [ ] 10.1.2 Complex task decomposition example
    - [ ] 10.1.3 Comments explaining planning workflow
  - [ ] 10.2 **Create** `examples/autonomous_prompt_generation.rs` with:
    - [ ] 10.2.1 Auto-generate prompt example with agent_description
    - [ ] 10.2.2 Regenerate prompt example
    - [ ] 10.2.3 Manual override example
    - [ ] 10.2.4 Comments explaining prompt generation
  - [ ] 10.3 **Create** `examples/dynamic_temperature.rs` with:
    - [ ] 10.3.1 Dynamic temperature example with different task types
    - [ ] 10.3.2 Temperature bounds configuration example
    - [ ] 10.3.3 Comparison with fixed temperature
    - [ ] 10.3.4 Comments explaining classification
  - [ ] 10.4 **Create** `examples/agent_handoffs.rs` with:
    - [ ] 10.4.1 Coordinator agent with multiple specialists
    - [ ] 10.4.2 Handoff strategy configuration example
    - [ ] 10.4.3 Task delegation flow example
    - [ ] 10.4.4 Comments explaining handoff mechanism
  - [ ] 10.5 **Create** `examples/autonomous_full_config.rs` with:
    - [ ] 10.5.1 Full configuration using all autonomous features
    - [ ] 10.5.2 Complex workflow combining planning + prompts + temperature + handoffs
    - [ ] 10.5.3 Result inspection showing all metadata
    - [ ] 10.5.4 Comprehensive comments
  - [ ] 10.6 Update `examples/README.md` with autonomous examples section
  - [ ] 10.7 Test all examples compile: `cargo build --examples`
  - [ ] 10.8 Test all examples run successfully (with test LLM or mocks)
  - [ ] 10.9 Run clippy on examples: `cargo clippy --examples`
  - [ ] 10.10 Format examples: `cargo fmt`
  - [ ] 10.11 Commit: `git commit -m "docs: add autonomous features examples" -m "- autonomous_planning.rs: planning mode with task decomposition" -m "- autonomous_prompt_generation.rs: auto-prompt with regeneration" -m "- dynamic_temperature.rs: temperature adjustment by task type" -m "- agent_handoffs.rs: delegation with specialists" -m "- autonomous_full_config.rs: comprehensive example with all features" -m "- Updated examples/README.md"`

- [ ] 11.0 Testing & Quality Assurance
  - [ ] 11.1 Run full test suite: `cargo test`
  - [ ] 11.2 Verify all autonomous feature tests passing
  - [ ] 11.3 Run integration tests: `cargo test --test '*'`
  - [ ] 11.4 Run unit tests with coverage: `cargo test --lib`
  - [ ] 11.5 Run clippy in strict mode: `cargo clippy --lib -- -D warnings`
  - [ ] 11.6 Run clippy on examples: `cargo clippy --examples -- -D warnings`
  - [ ] 11.7 Verify code formatting: `cargo fmt --check`
  - [ ] 11.8 Format all code: `cargo fmt`
  - [ ] 11.9 Run security audit: `cargo audit`
  - [ ] 11.10 Document any security findings in CHANGELOG
  - [ ] 11.11 Test all examples compile: `cargo build --examples`
  - [ ] 11.12 Run each example and verify output (with test LLM)
  - [ ] 11.13 Verify AUTONOMOUS.md completeness (all sections, examples, links)
  - [ ] 11.14 Verify all code has rustdoc comments
  - [ ] 11.15 Generate and review documentation: `cargo doc --open`
  - [ ] 11.16 Create CHANGELOG.md entry for Epic 14 with:
    - [ ] 11.16.1 Version number and release date
    - [ ] 11.16.2 All 5 user stories documented
    - [ ] 11.16.3 New features list (planning, prompts, temperature, handoffs, handoff tool)
    - [ ] 11.16.4 Configuration additions (YAML, CLI)
    - [ ] 11.16.5 Documentation updates (AUTONOMOUS.md, examples)
    - [ ] 11.16.6 Dependencies added (if any)
    - [ ] 11.16.7 Breaking changes (if any - should be none)
    - [ ] 11.16.8 Migration guide (if needed)
    - [ ] 11.16.9 Security audit results
    - [ ] 11.16.10 Test coverage statistics
  - [ ] 11.17 Review all code for TODOs and debug prints
  - [ ] 11.18 Run final test suite: `cargo test --all-features`
  - [ ] 11.19 Run benchmark tests (if applicable): `cargo bench`
  - [ ] 11.20 Verify no regression in existing functionality
  - [ ] 11.21 Final commit: `git commit -m "chore: Epic 14 QA complete" -m "- All tests passing (1146+ tests)" -m "- Zero clippy warnings" -m "- Code formatted and documented" -m "- Security audit complete" -m "- CHANGELOG.md updated" -m "- Ready for pull request"`

---

## Completion Checklist

When all tasks are complete, verify:

- [ ] All 11 tasks complete (0-11)
- [ ] All subtasks marked `[x]`
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] All tests passing (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Documentation complete (`docs/AUTONOMOUS.md`)
- [ ] Examples working and tested
- [ ] CHANGELOG.md updated
- [ ] Security audit completed (`cargo audit`)
- [ ] Final commit created with comprehensive message

**Estimated Duration:** 14 days  
**Actual Duration:** _[To be filled upon completion]_  
**Status:** Ready for Implementation
