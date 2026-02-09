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
