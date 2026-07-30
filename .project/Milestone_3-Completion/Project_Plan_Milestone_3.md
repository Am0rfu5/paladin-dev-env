# Paladin Project Plan: Milestone 3 — Completion & Polish

## Document Information

- **Version:** 1.0
- **Created:** February 9, 2026
- **Status:** Draft
- **Milestone:** 3 — Completion & Polish
- **Total Estimated Duration:** 8–10 weeks
- **Total New Epics:** 6 (Epics 19–24)

---

## Executive Summary

Milestone 3 addresses all deferred tasks from Epics 1–18 and outstanding inline `TODO` items across the codebase. These items were intentionally deferred during Milestones 1 and 2 due to cross-cutting dependencies, missing infrastructure, or low ROI at the time. With the full feature set now in place, we can systematically complete them.

This plan organizes the remaining work into six logically grouped epics that follow the project's established practices: Test-Driven Development, Domain-Driven Design, Hexagonal Architecture, and professional engineering standards.

### Scope Origin

All items in this plan originate from:

1. **Deferred task list items** — identified via `grep -ir "deferred" ./project/Milestone_*/**/tasks-*.md`
2. **Inline `TODO` comments** — identified in `./src/`, `./tests/`, and `./benches/` directories

### What Is Excluded

The following files contain `TODO` items that predate the multi-agent epics and are out of scope for this milestone:

- `src/application/storage/sql_store.rs`
- `src/application/use_cases/content/content_filtering_service.rs`
- `src/application/use_cases/content/content_llm_analysis_service.rs`
- `src/application/use_cases/content/content_nlp_analysis_service.rs`
- `src/config/setup/service_runner.rs`
- `src/core/platform/container/trigger.rs` (payload condition matching & cooldown — unrelated to agent epics)
- `src/core/platform/manager/queue_service.rs`
- `src/infrastructure/adapters/input/file_content_fetcher.rs`
- `src/infrastructure/adapters/notification/email_notification_adapter.rs`
- `src/infrastructure/repositories/file_content_repository.rs`
- `src/infrastructure/repositories/mysql_content_repository.rs`
- `src/infrastructure/repositories/sqlite_content_repository.rs`
- `tests/integration/system_log_integration_test.rs`

---

## Timeline Overview

```
Week 1–2:   Epic 19 (Herald & Domain Type Consolidation)
Week 2–3:   Epic 20 (Vision Pipeline Completion)
Week 3–5:   Epic 21 (Autonomous Agent Completion — Handoffs & Planning Integration)
Week 5–7:   Epic 22 (Battalion & Commander Hardening)
Week 7–8:   Epic 23 (CLI, Config & Infrastructure Completion)
Week 9–10:  Epic 24 (Test Hardening, Benchmarks & Documentation)
```

---

## Epic Overview

| Epic | Name | Priority | Duration | Dependencies | Origin |
|------|------|----------|----------|--------------|--------|
| 19 | Herald & Domain Type Consolidation | Critical | 1–2 weeks | None | Epic 8 TODOs |
| 20 | Vision Pipeline Completion | High | 1–2 weeks | Epic 19 | Epic 13 TODOs, deferred |
| 21 | Autonomous Agent Completion | Critical | 2 weeks | Epic 19 | Epic 14 deferred (23 items) |
| 22 | Battalion & Commander Hardening | High | 2 weeks | Epic 19 | Epics 4, 5, 15, 16 TODOs/deferred |
| 23 | CLI, Config & Infrastructure Completion | Medium | 1–2 weeks | Epics 19–22 | Epics 9, 10, 18 deferred |
| 24 | Test Hardening, Benchmarks & QA | High | 1–2 weeks | All above | All milestones deferred tests |

**Total Estimated Duration:** 8–10 weeks

---

## Epic 19: Herald & Domain Type Consolidation

**Theme:** Replace placeholder domain types, harden Herald system  
**Duration:** 1–2 weeks  
**Priority:** Critical  
**Dependencies:** None  
**Origin:** Epic 8 (Herald Output Formatting) inline TODOs

### Description

During Epic 8, several domain types in `herald.rs` were defined as placeholder structs with `TODO` comments indicating they should be replaced with actual types from Epics 1 and 4. Additionally, the Herald registry has a `TODO` to auto-register built-in formatters. Now that all domain types exist (`PaladinResult`, `BattalionResult`, `PaladinError`), these placeholders must be consolidated and the formatter pipeline completed.

### User Stories

#### US-19.1: Consolidate Herald Domain Types

**As a** framework developer  
**I want** Herald to use the actual domain result and error types  
**So that** there are no duplicate or placeholder structs in the codebase

**Acceptance Criteria:**
- [ ] Remove placeholder `PaladinResult` from `src/core/platform/container/herald.rs` (line 147)
- [ ] Remove placeholder `BattalionResult` from `src/core/platform/container/herald.rs` (line 158)
- [ ] Remove placeholder `PaladinError` from `src/core/platform/container/herald.rs` (line 187)
- [ ] Replace with imports from actual domain types in `paladin.rs` and `battalion/`
- [ ] Complete `StreamChunk` structure with full streaming metadata (line 169)
- [ ] Complete `ExecutionMetadata` structure with full telemetry fields (line 178)
- [ ] Update all Herald traits and implementations to use real types
- [ ] All existing Herald tests continue to pass
- [ ] No duplicate type definitions remain in the codebase

**Source Files:**
- `src/core/platform/container/herald.rs` — lines 147, 158, 169, 178, 187

---

#### US-19.2: Register Built-in Herald Formatters

**As a** developer  
**I want** built-in formatters auto-registered in the Herald registry  
**So that** JSON, Markdown, and Table formatting work out of the box

**Acceptance Criteria:**
- [ ] `HeraldRegistry::default()` auto-registers `JsonHerald`, `MarkdownHerald`, and `TableHerald`
- [ ] Formatters are retrievable by name from the registry
- [ ] Unit tests verify all built-in formatters are present after construction
- [ ] Documentation updated for Herald usage

**Source Files:**
- `src/application/use_cases/herald/herald_registry.rs` — line 186

---

### Epic 19 Completion Criteria

- [ ] All placeholder types in `herald.rs` replaced with real domain types
- [ ] `StreamChunk` and `ExecutionMetadata` fully defined
- [ ] Built-in formatters auto-registered
- [ ] All existing tests pass; new tests cover consolidated types
- [ ] `cargo clippy` clean, `cargo fmt` clean

---

## Epic 20: Vision Pipeline Completion

**Theme:** Complete multi-modal vision API integration  
**Duration:** 1–2 weeks  
**Priority:** High  
**Dependencies:** Epic 19  
**Origin:** Epic 13 (Sentinel Vision System) inline TODOs

### Description

Epic 13 established the vision type system and adapter scaffolding, but left the actual API calls and execution service integration as `TODO` stubs. This epic completes the vision pipeline end-to-end: making real API calls to OpenAI and Anthropic vision endpoints and wiring the execution service.

### User Stories

#### US-20.1: Implement OpenAI Vision API Call

**As a** developer  
**I want** the OpenAI vision adapter to make actual API calls  
**So that** image analysis works with GPT-4 Vision

**Acceptance Criteria:**
- [ ] `OpenAIVisionAdapter` sends multimodal content (image_url parts) to OpenAI Chat Completions API
- [ ] Supports both URL-based and base64-encoded images
- [ ] Parses and returns structured vision response
- [ ] Handles API errors (rate limits, invalid images, unsupported formats)
- [ ] Implements retry with exponential backoff
- [ ] Unit tests with mocked HTTP responses
- [ ] Integration test structure for live API validation (gated behind feature flag or env var)

**Source Files:**
- `src/infrastructure/adapters/llm/openai_vision.rs` — line 212

---

#### US-20.2: Implement Anthropic Vision API Call

**As a** developer  
**I want** the Anthropic vision adapter to make actual API calls  
**So that** image analysis works with Claude Vision

**Acceptance Criteria:**
- [ ] `AnthropicVisionAdapter` sends multimodal content blocks to Anthropic Messages API
- [ ] Supports both URL-based and base64-encoded images
- [ ] Handles Anthropic-specific content block format (type: "image", source: {type, media_type, data})
- [ ] Parses and returns structured vision response
- [ ] Handles API errors appropriately
- [ ] Unit tests with mocked HTTP responses

**Source Files:**
- `src/infrastructure/adapters/llm/anthropic_vision.rs` — line 220

---

#### US-20.3: Complete Vision Execution in PaladinExecutionService

**As a** developer  
**I want** the execution service to perform full vision processing  
**So that** Paladins can analyze images end-to-end

**Acceptance Criteria:**
- [ ] `PaladinExecutionService::execute_with_vision()` builds multimodal prompts
- [ ] Calls the appropriate vision adapter based on provider
- [ ] Parses vision results and integrates into the reasoning loop
- [ ] Respects existing execution configuration (max_loops, stop_words, timeout)
- [ ] Unit tests with mock vision adapter
- [ ] Example updated: `examples/sentinel_vision.rs` or equivalent

**Source Files:**
- `src/application/use_cases/paladin/paladin_execution_service.rs` — line 371

---

### Epic 20 Completion Criteria

- [ ] OpenAI and Anthropic vision adapters make real HTTP API calls
- [ ] `PaladinExecutionService` vision execution fully implemented
- [ ] All tests pass with mocked responses
- [ ] `cargo clippy` clean, `cargo fmt` clean

---

## Epic 21: Autonomous Agent Completion

**Theme:** Complete handoff execution, planning integration, and PaladinResult enhancements  
**Duration:** 2 weeks  
**Priority:** Critical  
**Dependencies:** Epic 19  
**Origin:** Epic 14 (Autonomous Agent Features) — 23 deferred task items

### Description

Epic 14 established the autonomous agent foundation (planning, prompt generation, dynamic temperature, handoffs) but deferred all items requiring cross-service integration, `PaladinResult` modifications, and full execution orchestration. This epic completes that work.

### User Stories

#### US-21.1: Handoff Execution Integration

**As a** developer  
**I want** handoffs to actually execute specialist agents  
**So that** multi-agent delegation works end-to-end

**Acceptance Criteria:**
- [ ] `HandoffService::execute_handoff()` delegates task to specialist Paladin via `PaladinExecutionService`
- [ ] Result flows back: specialist executes → result returned → original agent continues
- [ ] Handoff chain tracking maintained across tool invocations
- [ ] Circular handoff detection validated in integration context
- [ ] Max depth exceeded validation works end-to-end
- [ ] Handoff tool calls visible in execution trace

**Source — Deferred Tasks:**
- Epic 14, Task 5.17: Test for handoff execution
- Epic 14, Task 5.18: Implement `HandoffService::execute_handoff()`
- Epic 14, Task 5.27–5.28: Integration tests for full handoff workflow
- Epic 14, Task 6.9–6.14: Tool execution via HandoffService, result flow, chain tracking
- Epic 14, Task 6.17–6.22: Circular handoff, max depth, trace visibility

---

#### US-21.2: Handoff Tool Auto-Registration

**As a** developer  
**I want** handoff tools auto-registered when handoffs are configured  
**So that** the LLM can invoke handoffs via tool calling

**Acceptance Criteria:**
- [ ] `PaladinBuilder::build()` auto-registers handoff tool when `with_handoffs()` was called
- [ ] Handoff tool appears in arsenal with correct schema
- [ ] Unit tests verify auto-registration behavior

**Source — Deferred Tasks:**
- Epic 14, Task 6.5: Test for auto-registration in PaladinBuilder
- Epic 14, Task 6.6: Auto-registration logic in `PaladinBuilder::build()`

---

#### US-21.3: PaladinResult Metadata Enhancement

**As a** developer  
**I want** `PaladinResult` to include planning and handoff metadata  
**So that** execution traces contain full autonomous context

**Acceptance Criteria:**
- [ ] `PaladinResult` includes `plan: Option<TaskPlan>` field
- [ ] `PaladinResult` includes `handoff_history: Vec<HandoffRecord>` field
- [ ] Backward compatibility maintained (fields are `Option` or `Vec` with defaults)
- [ ] Serialization/deserialization tests pass
- [ ] All existing tests updated to accommodate new fields

**Source — Deferred Tasks:**
- Epic 14, Task 5.23–5.24: Handoff history in PaladinResult
- Epic 14, Task 7.11–7.14: Planning and handoff metadata in PaladinResult

---

#### US-21.4: Autonomous Execution Orchestration

**As a** developer  
**I want** all autonomous features orchestrated in the execution service  
**So that** planning, prompts, temperature, and handoffs work together

**Acceptance Criteria:**
- [ ] `PaladinExecutionService` coordinates: planning → prompt generation → dynamic temperature → LLM call → handoff handling
- [ ] Feature interaction edge cases handled (e.g., planning with handoffs)
- [ ] Integration tests cover full autonomous workflow
- [ ] End-to-end test with all features interacting

**Source — Deferred Tasks:**
- Epic 14, Task 7.15–7.20: Orchestration logic and integration tests

---

#### US-21.5: Configurable Model in Autonomous Services

**As a** developer  
**I want** the LLM model used by planning and prompt generation to be configurable  
**So that** services respect the Paladin's configured model

**Acceptance Criteria:**
- [ ] `PlanningService` reads model from Paladin config instead of hardcoded `"gpt-4"`
- [ ] `PromptGenerationService` reads model from Paladin config instead of hardcoded `"gpt-4"`
- [ ] Subtask expected output generated by LLM instead of hardcoded string
- [ ] Unit tests verify model is read from config

**Source Files:**
- `src/application/use_cases/paladin/planning_service.rs` — lines 128, 305, 426, 538
- `src/application/use_cases/paladin/prompt_generation_service.rs` — line 146

---

### Epic 21 Completion Criteria

- [ ] Handoff execution works end-to-end with real delegation
- [ ] Handoff tools auto-registered via builder
- [ ] `PaladinResult` includes planning and handoff metadata
- [ ] Autonomous execution orchestration tested in integration
- [ ] Hardcoded models replaced with config-driven values
- [ ] All tests pass; `cargo clippy` clean

---

## Epic 22: Battalion & Commander Hardening

**Theme:** Complete Battalion integration gaps, fix Commander delegation  
**Duration:** 2 weeks  
**Priority:** High  
**Dependencies:** Epic 19  
**Origin:** Epics 4, 5, 15, 16 — deferred tasks and inline TODOs

### Description

Several Battalion subsystems have integration gaps: Council and Grove store Paladin IDs but can't resolve them to actual Paladin instances, Phalanx doesn't track per-paladin timing, Grove's LLM-based routing is stubbed out, and Commander metadata export is unimplemented. This epic hardens all Battalion patterns.

### User Stories

#### US-22.1: Paladin Registry for Council and Grove

**As a** developer  
**I want** Council and Grove to resolve Paladin IDs to actual Paladin instances  
**So that** multi-agent patterns can execute with real agents

**Acceptance Criteria:**
- [ ] Define a `PaladinRegistry` trait or lookup mechanism
- [ ] Council resolves participant IDs to `Paladin` instances for execution
- [ ] Grove resolves agent IDs to `Paladin` instances for routing
- [ ] Commander populates registries when creating Council/Grove battalions
- [ ] Unit tests verify resolution works
- [ ] Integration tests verify execution with resolved Paladins

**Source Files:**
- `src/application/use_cases/battalion/council_service.rs` — line 160
- `src/application/use_cases/battalion/commander.rs` — lines 562, 617

---

#### US-22.2: Grove LLM-Based Routing Implementation

**As a** developer  
**I want** Grove to use the LLM for intelligent routing  
**So that** input is routed to the best-matching agent based on semantic understanding

**Acceptance Criteria:**
- [ ] `GroveService` sends routing prompt to LLM
- [ ] Parses JSON response: `tree_name`, `agent_id`, `confidence`, `reasoning`
- [ ] Falls back to keyword matching if LLM call fails
- [ ] Confidence threshold configurable
- [ ] Unit tests with mock LLM responses
- [ ] Integration test for LLM-based routing flow

**Source Files:**
- `src/application/use_cases/battalion/grove_service.rs` — line 475

---

#### US-22.3: Phalanx Per-Paladin Timing & Metrics

**As a** developer  
**I want** Phalanx to track individual Paladin execution times  
**So that** performance metrics are accurate and actionable

**Acceptance Criteria:**
- [ ] `PhalanxService` records each Paladin's execution time
- [ ] `per_paladin_times` populated in `BattalionMetadata`
- [ ] `paladin_success_count` and `paladin_failure_count` calculated correctly
- [ ] Unit tests verify timing and counts are accurate

**Source Files:**
- `src/application/use_cases/battalion/phalanx_service.rs` — line 270

---

#### US-22.4: Commander Metadata Export

**As a** developer  
**I want** Commander to export execution metadata to files  
**So that** I can analyze orchestration performance

**Acceptance Criteria:**
- [ ] If `metadata_output_dir` is configured, Commander writes metadata JSON to file
- [ ] File naming includes timestamp and strategy for traceability
- [ ] Unit test verifies file output (using temp directory)
- [ ] Error handling for I/O failures (non-fatal, logged)

**Source — Deferred Tasks:**
- Epic 5, Task 5.10: Implement metadata export to file
- Epic 5, Task 5.14: Unit test for metadata export

---

#### US-22.5: Commander Ignored Test Completion

**As a** developer  
**I want** all ignored Commander tests to be enabled and passing  
**So that** Campaign, ChainOfCommand, and error handling are fully validated

**Acceptance Criteria:**
- [ ] `test_execute_campaign` enabled with proper DAG mock setup
- [ ] `test_execute_chain_of_command` enabled with mock delegation
- [ ] Error handling tests enabled: `test_error_handling_fail_fast`, `test_error_handling_continue_on_error`, `test_error_handling_retry_then_continue`, `test_partial_failure_handling`
- [ ] Mock Paladin support for failure scenarios implemented
- [ ] All tests pass in CI

**Source Files:**
- `src/application/use_cases/battalion/commander.rs` — lines 1850, 1875, 2017, 2025, 2033, 2041

---

### Epic 22 Completion Criteria

- [ ] Council and Grove resolve Paladin IDs to instances
- [ ] Grove LLM-based routing implemented
- [ ] Phalanx per-paladin metrics accurate
- [ ] Commander metadata export working
- [ ] All ignored Commander tests enabled and passing
- [ ] All tests pass; `cargo clippy` clean

---

## Epic 23: CLI, Config & Infrastructure Completion

**Theme:** Wire remaining CLI configuration, complete infrastructure stubs  
**Duration:** 1–2 weeks  
**Priority:** Medium  
**Dependencies:** Epics 19–22  
**Origin:** Epics 9, 10, 18 deferred tasks and infrastructure TODOs  
**Status:** ✅ **COMPLETE** (February 14, 2026)

### Completion Summary

Epic 23 successfully completed all deferred CLI configuration and infrastructure work. **All 5 user stories completed**, delivering 84 comprehensive integration tests, production-ready garrison/arsenal/scheduler configuration, and complete mock infrastructure for CI-ready testing.

**Key deliverables:**
- ✅ Garrison (memory) configuration from YAML (in_memory & sqlite)
- ✅ Arsenal (MCP tools) configuration from YAML (STDIO & SSE)
- ✅ MockLlmAdapter and MockArsenalPort for CI-ready testing
- ✅ 84 integration tests passing (zero API keys required)
- ✅ Scheduler integration with tokio-cron-scheduler
- ✅ Comprehensive documentation (CONFIGURATION.md, 500+ lines)
- ✅ Example configurations with extensive comments

**Test coverage:** All CLI workflows covered (Paladin, Formation, Phalanx, tool integration, error handling)
**Documentation:** Complete configuration guide, testing guide updates
**Quality:** All tests passing, zero clippy warnings, zero TODOs

For detailed completion report, see: [`Epic_23/EPIC_23_COMPLETION_SUMMARY.md`](Epic_23/EPIC_23_COMPLETION_SUMMARY.md)

---

### Description

The CLI agent command has TODOs for wiring garrison and arsenal configuration from YAML files. Several CLI integration tests were deferred because they require mock provider support. The API content deliverer has a scheduler stub. This epic completes all infrastructure and CLI wiring.

### User Stories

#### US-23.1: CLI Garrison Configuration

**As a** developer  
**I want** the CLI `muster` command to configure garrison from YAML  
**So that** agents launched from config files have proper memory

**Acceptance Criteria:**
- [x] Parse garrison config from YAML (type: `in_memory` | `sqlite`, path, max_entries)
- [x] Instantiate appropriate garrison adapter based on config
- [x] Pass configured garrison to `PaladinBuilder`
- [x] Unit test with sample config
- [x] Error handling for invalid garrison config

**Source Files:**
- `src/application/cli/commands/agent.rs` — line 293 (Task 5.8)

---

#### US-23.2: CLI Arsenal/MCP Configuration

**As a** developer  
**I want** the CLI `muster` command to configure arsenal from YAML  
**So that** agents launched from config files have tool access

**Acceptance Criteria:**
- [x] Parse MCP server config from YAML (name, type: `stdio` | `sse`, command, args, url)
- [x] Instantiate MCP adapters based on config
- [x] Register tools in arsenal registry
- [x] Pass configured arsenal to `PaladinBuilder`
- [x] Unit test with sample config
- [x] Error handling for invalid arsenal config

**Source Files:**
- `src/application/cli/commands/agent.rs` — line 296 (Task 5.9)

---

#### US-23.3: CLI Integration Tests with Mock Provider

**As a** developer  
**I want** CLI integration tests that use mock LLM providers  
**So that** end-to-end CLI workflows are validated without API keys

**Acceptance Criteria:**
- [x] Implement mock LLM provider support for CLI testing
- [x] Test: run Paladin from config with mock LLM adapter
- [x] Test: run Formation with multiple mock Paladins
- [x] Test: run Phalanx with parallel execution
- [x] Tests run in CI without external dependencies

**Source — Deferred Tasks:**
- Epic 10, Task 13.4–13.6: CLI integration tests requiring mock provider

---

#### US-23.4: CLI End-to-End & Environment Testing

**As a** developer  
**I want** CLI tested across real environments and terminal types  
**So that** the CLI works reliably in production

**Acceptance Criteria:**
- [x] Test full user journey: onboarding → first agent run (with mock provider)
- [x] Test `setup-check` with real services (Redis, Qdrant, MinIO) — Docker-gated
- [x] Test `muster` command with real LLM providers — env-var-gated
- [x] Test `council` command end-to-end — env-var-gated
- [x] Test all commands in non-interactive mode (CI/CD)
- [x] Test CLI with `NO_COLOR` environment variable
- [x] Test CLI with different terminal types

**Source — Deferred Tasks:**
- Epic 18, Task 9.1–9.7: End-to-end CLI testing

---

#### US-23.5: API Content Deliverer Scheduler Integration

**As a** developer  
**I want** scheduled content delivery to use a real scheduler  
**So that** time-based delivery works in production

**Acceptance Criteria:**
- [x] Integrate `tokio-cron-scheduler` or equivalent for scheduled delivery
- [x] `schedule_delivery()` creates real scheduled jobs
- [x] Cancellation support for pending scheduled deliveries
- [x] Unit tests with mock scheduler
- [x] Integration test verifying scheduled execution

**Source Files:**
- `src/infrastructure/adapters/output/api_content_deliverer.rs` — line 297

---

### Epic 23 Completion Criteria

- [x] CLI garrison and arsenal config wired from YAML
- [x] CLI integration tests with mock provider passing
- [x] End-to-end CLI tests documented and gated appropriately
- [x] Scheduler integration completed
- [x] All tests pass; `cargo clippy` clean

---

## Epic 24: Test Hardening, Benchmarks & QA

**Theme:** Enable deferred tests, fix benchmarks, final quality assurance  
**Duration:** 1–2 weeks  
**Priority:** High  
**Dependencies:** All preceding epics  
**Origin:** Deferred tests across all milestones, benchmark API drift

### Description

Across both milestones, numerous tests were deferred or ignored due to missing infrastructure, API changes, or complexity. This final epic enables all deferred tests, fixes benchmark API drift, adds missing integration tests, and performs a final quality sweep.

### User Stories

#### US-24.1: Fix Campaign & ChainOfCommand Benchmarks

**As a** developer  
**I want** all Battalion benchmarks compiling and running  
**So that** performance tracking covers all orchestration patterns

**Acceptance Criteria:**
- [ ] Update `benchmark_campaign` to match current Campaign API (add_node/add_edge)
- [ ] Update `benchmark_chain_of_command` to match current constructor signature
- [ ] Re-enable both benchmarks in criterion group registration
- [ ] All benchmarks compile and produce results
- [ ] Document benchmark results in `docs/BATTALION_BENCHMARKS.md`

**Source Files:**
- `benches/battalion_benchmarks.rs` — lines 297, 390, 950

---

#### US-24.2: Enable Prompt Generation Service Tests

**As a** developer  
**I want** the prompt generation service test module re-enabled  
**So that** prompt generation has proper test coverage

**Acceptance Criteria:**
- [ ] Update mock in `tests/unit/mod.rs` to match current `LlmPort` trait signature
- [ ] Uncomment and fix `prompt_generation_service_test` module
- [ ] All prompt generation tests passing
- [ ] Coverage meets ≥80% for prompt generation service

**Source Files:**
- `tests/unit/mod.rs` — line 22

---

#### US-24.3: Enhance Timeout and Execution Service Tests

**As a** developer  
**I want** timeout behavior properly tested  
**So that** execution timeouts are verified

**Acceptance Criteria:**
- [ ] Enhance `MockLlmPort` to support configurable delays
- [ ] Implement timeout test that verifies 60-second timeout behavior
- [ ] Remove `#[ignore]` from timeout test
- [ ] Test passes reliably in CI

**Source Files:**
- `tests/unit/paladin_execution_service_test.rs` — lines 237, 239

---

#### US-24.4: Qdrant Integration Tests

**As a** developer  
**I want** full integration tests with Qdrant  
**So that** the Sanctum RAG pipeline is validated against a real vector database

**Acceptance Criteria:**
- [ ] Integration tests in `tests/integration/rag_integration_tests.rs` implemented
- [ ] Tests cover: store, search, delete, update with real Qdrant
- [ ] Tests cover end-to-end RAG-enabled Paladin execution
- [ ] Token budget limiting verified
- [ ] Context formatting verified
- [ ] Tests gated behind Docker/Qdrant availability check
- [ ] Unit-level Qdrant tests in `tests/unit/sanctum/qdrant_sanctum_test.rs` expanded

**Source Files:**
- `tests/integration/rag_integration_tests.rs` — line 147
- `tests/unit/sanctum/qdrant_sanctum_test.rs` — line 62

---

#### US-24.5: Deferred Unit Test Coverage Improvement

**As a** developer  
**I want** remaining low-coverage modules addressed  
**So that** overall test coverage meets professional standards

**Acceptance Criteria:**
- [ ] Review coverage for previously deferred modules:
  - User Service (`src/core/platform/manager/user_service.rs`) — was 4.23%
  - Listener Service (`src/core/platform/manager/listener_service.rs`) — was 57.83%
- [ ] Determine if coverage can be improved with reasonable effort
- [ ] Implement tests where ROI justifies the effort
- [ ] Document decisions for any items re-deferred with rationale

**Source — Deferred Tasks:**
- Milestone 1, Unit Test Improvements, Task 6.1–6.2

---

#### US-24.6: CLI Snapshot Testing & Documentation

**As a** developer  
**I want** CLI output validated with snapshot tests  
**So that** UI regressions are caught automatically

**Acceptance Criteria:**
- [ ] Add `insta` crate dependency for snapshot testing
- [ ] Create `tests/cli/snapshots/` directory
- [ ] Snapshot tests for table rendering output
- [ ] Snapshot tests for progress indicators
- [ ] Snapshot tests for error messages
- [ ] Snapshot tests for command help output
- [ ] Add inline rustdoc for all public CLI functions
- [ ] Update `QUICKSTART.md` and `INSTALLATION.md` with CLI information

**Source — Deferred Tasks:**
- Epic 18, Tasks 7.10–7.14: Snapshot testing (requires insta crate)
- Epic 18, Tasks 8.11, 8.13, 8.14: Documentation

---

#### US-24.7: Provider Live API Integration Tests

**As a** developer  
**I want** optional live API integration tests for LLM providers  
**So that** adapter correctness can be validated against real services

**Acceptance Criteria:**
- [ ] Integration tests for OpenAI live API (env-var-gated)
- [ ] Integration tests for DeepSeek live API (env-var-gated)
- [ ] Integration tests for Anthropic live API (env-var-gated)
- [ ] Tests validate: completion, streaming, tool calling, error handling
- [ ] Tests excluded from default `cargo test` (require explicit flag or feature)

**Source — Deferred Tasks:**
- Epic 6, Task 7.0: Live API validation tests

---

#### US-24.8: Final Documentation & Demo Assets

**As a** developer  
**I want** all documentation current and demo assets created  
**So that** the project is presentation-ready

**Acceptance Criteria:**
- [ ] Update `README.md` with Council and Grove patterns
- [ ] Update `docs/QUICKSTART.md` with Council/Grove quickstart
- [ ] Create demo video or GIF showing CLI features
- [ ] Add CI/CD test job for CLI tests
- [ ] `cargo doc` generates clean documentation
- [ ] All public APIs documented with rustdoc

**Source — Deferred Tasks:**
- Epic 16, Tasks 11.23–11.24: README and QUICKSTART updates
- Epic 18, Tasks 9.16, 7.17: Demo assets and CI/CD

---

### Epic 24 Completion Criteria

- [ ] All benchmarks compile and run
- [ ] All previously ignored/disabled tests enabled and passing
- [ ] Qdrant integration tests implemented
- [ ] Snapshot testing infrastructure in place
- [ ] Provider live API tests available (gated)
- [ ] Documentation fully updated
- [ ] `cargo clippy` clean, `cargo fmt` clean
- [ ] `cargo doc` clean

---

## Cross-Cutting Concerns

### Test Coverage Targets

| Layer | Target |
|-------|--------|
| Core domain | ≥ 85% |
| Application services | ≥ 80% |
| Infrastructure adapters | ≥ 70% |
| CLI commands | ≥ 70% |
| Overall | ≥ 75% |

### Quality Gates (Every Epic)

- [ ] `cargo test` — all tests pass
- [ ] `cargo fmt --check` — formatting clean
- [ ] `cargo clippy -- -D warnings` — no warnings
- [ ] `cargo doc --no-deps` — documentation builds cleanly
- [ ] Snyk security scan — no new vulnerabilities

### Commit Convention

All commits follow conventional format:
```
feat: <description>
fix: <description>
refactor: <description>
test: <description>
docs: <description>
```

---

## Appendix A: Complete Deferred Item Traceability

### From Milestone 1 Task Lists (Uncompleted `[ ]` Deferred Items)

| Epic | Task | Description | Target Epic |
|------|------|-------------|-------------|
| 5 | 5.10 | Implement metadata export to file | Epic 22 (US-22.4) |
| 5 | 5.14 | Test: metadata export to file | Epic 22 (US-22.4) |
| 6 | 7.0 | Live API integration tests | Epic 24 (US-24.7) |
| 8 | 7.13 | Integration tests for Battalion with Herald | Epic 22 (US-22.5) |
| 10 | 13.4 | Test: run Paladin from config with mock LLM | Epic 23 (US-23.3) |
| 10 | 13.5 | Test: run Formation with multiple mock Paladins | Epic 23 (US-23.3) |
| 10 | 13.6 | Test: run Phalanx with parallel execution | Epic 23 (US-23.3) |
| Unit Tests | 6.1 | User Service coverage improvement | Epic 24 (US-24.5) |
| Unit Tests | 6.2 | Listener Service coverage improvement | Epic 24 (US-24.5) |

### From Milestone 2 Task Lists (Uncompleted `[ ]` Deferred Items)

| Epic | Task | Description | Target Epic |
|------|------|-------------|-------------|
| 14 | 5.17 | Test for handoff execution | Epic 21 (US-21.1) |
| 14 | 5.18 | Implement HandoffService::execute_handoff() | Epic 21 (US-21.1) |
| 14 | 5.23 | Test for handoff history in PaladinResult | Epic 21 (US-21.3) |
| 14 | 5.24 | Update PaladinResult with handoff_history | Epic 21 (US-21.3) |
| 14 | 5.27 | Integration test for full handoff workflow | Epic 21 (US-21.1) |
| 14 | 5.28 | Create autonomous_handoff_test.rs | Epic 21 (US-21.1) |
| 14 | 6.5 | Test for auto-registration in PaladinBuilder | Epic 21 (US-21.2) |
| 14 | 6.6 | Auto-registration logic in PaladinBuilder | Epic 21 (US-21.2) |
| 14 | 6.9 | Test for tool execution via HandoffService | Epic 21 (US-21.1) |
| 14 | 6.10 | Implement HandoffTool::execute() | Epic 21 (US-21.1) |
| 14 | 6.11 | Test for specialist result return | Epic 21 (US-21.1) |
| 14 | 6.12 | Implement result flow | Epic 21 (US-21.1) |
| 14 | 6.13 | Test for handoff chain tracking | Epic 21 (US-21.1) |
| 14 | 6.14 | Update tool execution for chain tracking | Epic 21 (US-21.1) |
| 14 | 6.17 | Test for circular handoff error | Epic 21 (US-21.1) |
| 14 | 6.18 | Circular handoff validation in tool execution | Epic 21 (US-21.1) |
| 14 | 6.19 | Test for max depth exceeded error | Epic 21 (US-21.1) |
| 14 | 6.20 | Max depth validation | Epic 21 (US-21.1) |
| 14 | 6.21 | Test for tool visibility in trace | Epic 21 (US-21.1) |
| 14 | 6.22 | Update execution trace for handoff tool calls | Epic 21 (US-21.1) |
| 14 | 7.11 | Test for PaladinResult planning metadata | Epic 21 (US-21.3) |
| 14 | 7.12 | Update PaladinResult with plan field | Epic 21 (US-21.3) |
| 14 | 7.13 | Test for PaladinResult handoff metadata | Epic 21 (US-21.3) |
| 14 | 7.14 | PaladinResult handoff_history field | Epic 21 (US-21.3) |
| 14 | 7.15 | Test for orchestration logic | Epic 21 (US-21.4) |
| 14 | 7.16 | Orchestration in PaladinExecutionService | Epic 21 (US-21.4) |
| 14 | 7.17 | Integration test for full autonomous workflow | Epic 21 (US-21.4) |
| 14 | 7.18 | Create autonomous_full_test.rs | Epic 21 (US-21.4) |
| 14 | 7.19 | Test for feature interaction edge cases | Epic 21 (US-21.4) |
| 14 | 7.20 | Tests for complex scenario interactions | Epic 21 (US-21.4) |
| 18 | 9.1 | Test full user journey | Epic 23 (US-23.4) |
| 18 | 9.2 | Test setup-check with real services | Epic 23 (US-23.4) |
| 18 | 9.3 | Test muster with real LLM providers | Epic 23 (US-23.4) |
| 18 | 9.4 | Test council command end-to-end | Epic 23 (US-23.4) |
| 18 | 9.5 | Test all commands in non-interactive mode | Epic 23 (US-23.4) |
| 18 | 9.6 | Test CLI with NO_COLOR | Epic 23 (US-23.4) |
| 18 | 9.7 | Test CLI with different terminal types | Epic 23 (US-23.4) |

### From Inline TODO Comments

| File | Line | Description | Target Epic |
|------|------|-------------|-------------|
| `herald.rs` | 147 | Replace placeholder PaladinResult | Epic 19 (US-19.1) |
| `herald.rs` | 158 | Replace placeholder BattalionResult | Epic 19 (US-19.1) |
| `herald.rs` | 169 | Define complete StreamChunk | Epic 19 (US-19.1) |
| `herald.rs` | 178 | Define complete ExecutionMetadata | Epic 19 (US-19.1) |
| `herald.rs` | 187 | Replace placeholder PaladinError | Epic 19 (US-19.1) |
| `herald_registry.rs` | 186 | Register built-in formatters | Epic 19 (US-19.2) |
| `openai_vision.rs` | 212 | Implement actual API call | Epic 20 (US-20.1) |
| `anthropic_vision.rs` | 220 | Implement actual API call | Epic 20 (US-20.2) |
| `paladin_execution_service.rs` | 371 | Implement full vision execution | Epic 20 (US-20.3) |
| `planning_service.rs` | 128 | Make model configurable | Epic 21 (US-21.5) |
| `planning_service.rs` | 305 | Make model configurable | Epic 21 (US-21.5) |
| `planning_service.rs` | 426 | Ask LLM for expected output | Epic 21 (US-21.5) |
| `planning_service.rs` | 538 | Make model configurable | Epic 21 (US-21.5) |
| `prompt_generation_service.rs` | 146 | Make model configurable | Epic 21 (US-21.5) |
| `council_service.rs` | 160 | Get actual Paladin from registry | Epic 22 (US-22.1) |
| `grove_service.rs` | 475 | Call LLM with prompt | Epic 22 (US-22.2) |
| `phalanx_service.rs` | 270 | Track individual execution times | Epic 22 (US-22.3) |
| `commander.rs` | 562 | Council needs actual Paladins | Epic 22 (US-22.1) |
| `commander.rs` | 617 | Grove needs actual Paladins | Epic 22 (US-22.1) |
| `commander.rs` | 1850 | Ignored test — Campaign DAG | Epic 22 (US-22.5) |
| `commander.rs` | 1875 | Ignored test — ChainOfCommand | Epic 22 (US-22.5) |
| `commander.rs` | 2017–2041 | Ignored error handling tests | Epic 22 (US-22.5) |
| `agent.rs` | 293 | Configure garrison from config | Epic 23 (US-23.1) |
| `agent.rs` | 296 | Configure arsenal/MCP from config | Epic 23 (US-23.2) |
| `api_content_deliverer.rs` | 297 | Scheduler integration | Epic 23 (US-23.5) |
| `battalion_benchmarks.rs` | 297, 390, 950 | Fix Campaign/ChainOfCommand benchmarks | Epic 24 (US-24.1) |
| `mod.rs` (tests/unit) | 22 | Re-enable prompt gen service tests | Epic 24 (US-24.2) |
| `paladin_execution_service_test.rs` | 237, 239 | Enhance MockLlmPort for timeout | Epic 24 (US-24.3) |
| `rag_integration_tests.rs` | 147 | Add full Qdrant integration tests | Epic 24 (US-24.4) |
| `qdrant_sanctum_test.rs` | 62 | Add Qdrant client integration tests | Epic 24 (US-24.4) |

---

## Appendix B: Files Modified Per Epic

| Epic | Key Files |
|------|-----------|
| 19 | `src/core/platform/container/herald.rs`, `src/application/use_cases/herald/herald_registry.rs` |
| 20 | `src/infrastructure/adapters/llm/openai_vision.rs`, `src/infrastructure/adapters/llm/anthropic_vision.rs`, `src/application/use_cases/paladin/paladin_execution_service.rs` |
| 21 | `src/application/use_cases/paladin/paladin_execution_service.rs`, `src/application/use_cases/paladin/planning_service.rs`, `src/application/use_cases/paladin/prompt_generation_service.rs`, `src/core/platform/container/paladin.rs` (PaladinResult) |
| 22 | `src/application/use_cases/battalion/council_service.rs`, `src/application/use_cases/battalion/grove_service.rs`, `src/application/use_cases/battalion/phalanx_service.rs`, `src/application/use_cases/battalion/commander.rs` |
| 23 | `src/application/cli/commands/agent.rs`, `src/infrastructure/adapters/output/api_content_deliverer.rs` |
| 24 | `benches/battalion_benchmarks.rs`, `tests/unit/mod.rs`, `tests/unit/paladin_execution_service_test.rs`, `tests/integration/rag_integration_tests.rs`, `tests/unit/sanctum/qdrant_sanctum_test.rs`, `docs/` |
