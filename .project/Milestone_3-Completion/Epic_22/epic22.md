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
