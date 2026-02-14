# Product Requirements Document: Test Hardening, Benchmarks & QA

## Document Information

- **Feature Name:** Test Hardening, Benchmarks & QA (Epic 24)
- **Version:** 1.0
- **Created:** February 14, 2026
- **Status:** Draft
- **Epic:** Milestone 3, Epic 24
- **Priority:** High
- **Estimated Duration:** 1-2 weeks (extendable to 3 weeks if needed)
- **Dependencies:** All Epics 19-23 must be complete

---

## 1. Introduction/Overview

Across Milestones 1 and 2 of the Paladin project, numerous tests were intentionally deferred or disabled due to missing infrastructure, API changes, cross-cutting dependencies, or complexity. Additionally, several benchmarks have experienced API drift and no longer compile. This epic represents the final quality assurance phase of Milestone 3, systematically enabling all deferred tests, fixing benchmark issues, adding missing integration test coverage, and performing a comprehensive quality sweep.

**Problem Statement:**
- Deferred and ignored tests reduce confidence in code correctness
- Non-compiling benchmarks prevent performance regression detection
- Missing integration tests leave critical workflows unvalidated
- Incomplete documentation creates barriers to adoption

**Solution:**
This epic completes the testing infrastructure by:
1. Fixing all broken benchmarks (Campaign, ChainOfCommand)
2. Enabling all deferred/ignored test modules
3. Implementing comprehensive integration tests (Qdrant, live APIs)
4. Adding snapshot testing infrastructure for CLI validation
5. Improving test coverage for previously deferred modules
6. Updating all documentation and creating demo assets

---

## 2. Goals

1. **Enable 100% of deferred tests:** All previously ignored or disabled tests must be enabled and passing
2. **Fix all benchmarks:** All benchmark suites must compile and produce valid performance metrics
3. **Achieve integration test completeness:** All critical workflows validated with real external services
4. **Establish snapshot testing:** CLI output regression testing infrastructure in place
5. **Meet coverage targets:** Maintain ≥80% unit test coverage, ≥70% integration test coverage
6. **Production-ready documentation:** All docs current, demo assets created, project presentation-ready
7. **Zero quality debt:** `cargo clippy` clean, `cargo fmt` clean, `cargo doc` clean

---

## 3. User Stories

### US-24.1: Fix Campaign & ChainOfCommand Benchmarks

**As a** performance engineer  
**I want** all Battalion orchestration benchmarks compiling and running  
**So that** I can detect performance regressions across all coordination patterns

**Acceptance Criteria:**
- [ ] Update `benchmark_campaign` to match current Campaign API (add_node/add_edge)
- [ ] Update `benchmark_chain_of_command` to match current constructor signature
- [ ] Re-enable both benchmarks in criterion group registration
- [ ] All benchmarks compile without warnings
- [ ] Benchmarks produce valid performance metrics
- [ ] Document benchmark results in `docs/BATTALION_BENCHMARKS.md`
- [ ] CI can run benchmarks without failure

**Priority:** Critical  
**Source Files:** `benches/battalion_benchmarks.rs` (lines 297, 390, 950)

---

### US-24.2: Enable Prompt Generation Service Tests

**As a** framework developer  
**I want** the prompt generation service test module re-enabled  
**So that** autonomous prompt generation has proper test coverage

**Acceptance Criteria:**
- [ ] Update mock in `tests/unit/mod.rs` to match current `LlmPort` trait signature
- [ ] Uncomment and fix `prompt_generation_service_test` module
- [ ] All prompt generation service tests passing
- [ ] Test coverage for prompt generation meets ≥80%
- [ ] No `#[ignore]` attributes remain on passing tests

**Priority:** High  
**Source Files:** `tests/unit/mod.rs` (line 22)

---

### US-24.3: Enhance Timeout and Execution Service Tests

**As a** reliability engineer  
**I want** timeout behavior properly tested  
**So that** execution timeouts are verified and reliable in production

**Acceptance Criteria:**
- [ ] Enhance `MockLlmPort` to support configurable delays
- [ ] Implement timeout test that verifies 60-second timeout behavior
- [ ] Remove `#[ignore]` from timeout test
- [ ] Test passes reliably in CI (no flakiness)
- [ ] Test covers both hard timeout and graceful shutdown scenarios
- [ ] Edge cases tested (timeout at 0s, timeout > max_duration, etc.)

**Priority:** High  
**Source Files:** `tests/unit/paladin_execution_service_test.rs` (lines 237, 239)

---

### US-24.4: Qdrant Integration Tests

**As a** RAG system developer  
**I want** full integration tests with Qdrant vector database  
**So that** the Sanctum RAG pipeline is validated against real infrastructure

**Acceptance Criteria:**
- [ ] Integration tests in `tests/integration/rag_integration_tests.rs` implemented
- [ ] Tests cover: store, search, delete, update with real Qdrant instance
- [ ] Tests cover end-to-end RAG-enabled Paladin execution workflow
- [ ] Token budget limiting verified in integration context
- [ ] Context formatting verified with real vector search results
- [ ] Tests gated behind Docker/Qdrant availability check (skip if unavailable)
- [ ] Unit-level Qdrant tests in `tests/unit/sanctum/qdrant_sanctum_test.rs` expanded
- [ ] Test setup/teardown cleans up test collections
- [ ] Integration test coverage meets ≥70%

**Priority:** Critical  
**Source Files:**
- `tests/integration/rag_integration_tests.rs` (line 147)
- `tests/unit/sanctum/qdrant_sanctum_test.rs` (line 62)

---

### US-24.5: Deferred Unit Test Coverage Improvement

**As a** quality assurance engineer  
**I want** remaining low-coverage modules addressed  
**So that** overall test coverage meets professional standards

**Acceptance Criteria:**
- [ ] Review coverage report for previously deferred modules:
  - User Service (`src/core/platform/manager/user_service.rs`) — was 4.23%
  - Listener Service (`src/core/platform/manager/listener_service.rs`) — was 57.83%
- [ ] Determine if coverage can be improved with reasonable effort (cost/benefit analysis)
- [ ] Implement tests where ROI justifies the effort
- [ ] Document decisions for any items re-deferred with clear rationale
- [ ] Overall project coverage maintains ≥80% unit, ≥70% integration
- [ ] Coverage report generated and reviewed in PR

**Priority:** Medium  
**Source:** Milestone 1, Unit Test Improvements, Task 6.1–6.2 (deferred)

---

### US-24.6: CLI Snapshot Testing & Documentation

**As a** CLI developer  
**I want** CLI output validated with snapshot tests  
**So that** UI regressions are caught automatically and user experience remains consistent

**Acceptance Criteria:**
- [ ] Add `insta` crate dependency for snapshot testing (latest stable version)
- [ ] Create `tests/cli/snapshots/` directory structure
- [ ] Snapshot tests for table rendering output (all table formats)
- [ ] Snapshot tests for progress indicators and spinners
- [ ] Snapshot tests for error messages (formatted and colored)
- [ ] Snapshot tests for command help output (all subcommands)
- [ ] Add inline rustdoc for all public CLI functions and types
- [ ] Update `QUICKSTART.md` with CLI usage examples
- [ ] Update `INSTALLATION.md` with CLI installation and setup
- [ ] Update `docs/cli/README.md` with comprehensive CLI documentation

**Priority:** High  
**Source:** Epic 18, Tasks 7.10–7.14, 8.11, 8.13, 8.14 (deferred)

---

### US-24.7: Provider Live API Integration Tests

**As a** LLM integration developer  
**I want** optional live API integration tests for all LLM providers  
**So that** adapter correctness can be validated against real provider APIs

**Acceptance Criteria:**
- [ ] Integration tests for OpenAI live API (requires `OPENAI_API_KEY` env var or .env file)
- [ ] Integration tests for DeepSeek live API (requires `DEEPSEEK_API_KEY` env var or .env file)
- [ ] Integration tests for Anthropic live API (requires `ANTHROPIC_API_KEY` env var or .env file)
- [ ] Tests validate: completion, streaming, tool calling, error handling, rate limits
- [ ] Tests gated behind feature flag `live-api-tests` (excluded from default `cargo test`)
- [ ] Tests skip gracefully if API keys not available (no failures, just warnings)
- [ ] README section documents how to run live API tests
- [ ] Tests respect rate limits and implement appropriate delays/retries

**Priority:** Medium  
**Source:** Epic 6, Task 7.0 (deferred)

---

### US-24.8: Final Documentation & Demo Assets

**As a** project maintainer  
**I want** all documentation current and demo assets created  
**So that** the project is presentation-ready for production use

**Acceptance Criteria:**
- [ ] Update `README.md` with Council and Grove patterns (comprehensive examples)
- [ ] Update `docs/QUICKSTART.md` with Council/Grove quickstart guides
- [ ] Create demo video or animated GIF showing CLI features (terminal recording)
- [ ] Add CI/CD test job specifically for CLI tests
- [ ] `cargo doc --open` generates clean documentation with no warnings
- [ ] All public APIs documented with rustdoc (functions, types, modules)
- [ ] Architecture documentation (`docs/Design/Design_and_Architecture.md`) reviewed and updated
- [ ] `CONTRIBUTING.md` updated with testing guidelines
- [ ] Release notes drafted for Milestone 3 completion

**Priority:** High  
**Source:** Epic 16, Tasks 11.23–11.24; Epic 18, Tasks 9.16, 7.17 (deferred)

---

## 4. Functional Requirements

### FR-4.1: Benchmark System
1. All benchmarks in `benches/` directory must compile without errors or warnings
2. Benchmarks must use current API signatures (no deprecated or removed APIs)
3. Benchmark results must be reproducible (same code produces similar metrics)
4. Benchmarks must be registered in criterion groups for execution
5. Benchmark documentation must explain what each benchmark measures

### FR-4.2: Test Infrastructure
6. `MockLlmPort` must support configurable response delays for timeout testing
7. Snapshot testing infrastructure must use `insta` crate
8. Integration tests must check for service availability before running
9. Live API tests must be opt-in via feature flag and environment variables
10. All tests must clean up after themselves (no persistent state between runs)

### FR-4.3: Test Coverage
11. Unit test coverage must be ≥80% for all modules (measured with `cargo tarpaulin` or equivalent)
12. Integration test coverage must be ≥70% for critical workflows
13. Previously deferred modules must be evaluated for coverage improvement
14. Coverage reports must be generated in CI/CD pipeline

### FR-4.4: Qdrant Integration
15. Qdrant integration tests must run against local Qdrant instance (Docker)
16. Tests must create and destroy test collections to avoid pollution
17. Tests must validate all CRUD operations (Create, Read, Update, Delete)
18. Tests must validate vector search with different similarity metrics
19. Tests must validate RAG workflow end-to-end with real Paladin execution

### FR-4.5: CLI Testing
20. Snapshot tests must capture terminal output including ANSI codes
21. Snapshot tests must be reviewable with `cargo insta review`
22. CLI tests must validate both success and error output formatting
23. CLI help output must be stable and properly formatted

### FR-4.6: Documentation
24. All public functions must have rustdoc comments with examples
25. `cargo doc` must produce no warnings
26. Documentation must include practical examples for all features
27. Demo assets must be stored in appropriate location (e.g., `docs/assets/`)

### FR-4.7: Quality Gates
28. `cargo fmt --check` must pass (all code formatted)
29. `cargo clippy -- -D warnings` must pass (no lint warnings)
30. `cargo test` must pass (all non-live-api tests)
31. `cargo test --features live-api-tests` must pass when API keys provided
32. `cargo bench --no-run` must compile all benchmarks successfully

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **NOT** part of this epic:

1. **Pre-existing TODO items outside multi-agent scope:**
   - Content management system TODOs in `src/application/use_cases/content/`
   - Legacy SQL store TODOs in `src/application/storage/sql_store.rs`
   - Trigger system TODOs in `src/core/platform/container/trigger.rs`
   - Repository TODOs in `src/infrastructure/repositories/`
   - These predate the multi-agent feature work and are out of scope

2. **Performance optimization:** This epic focuses on correctness, not performance improvements

3. **New feature development:** No new features should be added; focus is on validating existing features

4. **Load testing:** This epic covers functional testing, not load/stress testing

5. **Security testing:** Security scanning is covered in separate Epic (Snyk integration)

6. **UI/UX redesign:** CLI output validation only, no design changes

7. **Breaking API changes:** All tests must work with current APIs; no refactoring allowed

---

## 6. Technical Considerations

### 6.1 Testing Stack
- **Unit/Integration Testing:** `cargo test` (built-in Rust testing)
- **Snapshot Testing:** `insta` crate (version ^1.34.0 or latest stable)
- **Coverage Reporting:** `cargo tarpaulin` or `llvm-cov`
- **Benchmarking:** `criterion` crate (existing)
- **Mocking:** `mockall` crate (existing for trait mocking)

### 6.2 External Service Dependencies
- **Qdrant:** Vector database for RAG integration tests
  - Requires Docker or local Qdrant instance
  - Connection: `http://localhost:6333`
  - Tests must skip if unavailable
- **LLM Provider APIs:** For live API tests
  - OpenAI: `https://api.openai.com/v1`
  - DeepSeek: `https://api.deepseek.com/v1`
  - Anthropic: `https://api.anthropic.com/v1`
  - Requires valid API keys in environment

### 6.3 CI/CD Integration
- Tests must run in GitHub Actions or equivalent CI
- Qdrant tests: Use Docker Compose service in CI
- Live API tests: Run only if secrets configured (not on forks/PRs)
- Benchmark compilation: Run `cargo bench --no-run` to catch API drift early

### 6.4 Architecture Boundaries
- Follow hexagonal architecture (no core → infrastructure imports)
- Test at appropriate boundaries (unit tests for core, integration for infrastructure)
- Mock external dependencies appropriately

### 6.5 Rust Best Practices
- Follow TDD: Write tests before fixing bugs
- Use `#[should_panic]` for expected failure tests
- Use `#[ignore]` only temporarily during development
- Document test intent with rustdoc on test functions
- Use meaningful assertion messages

---

## 7. Success Metrics

### 7.1 Quantitative Metrics
- **Test Count:** Number of enabled tests increases by at least 50
- **Test Coverage:** Unit ≥80%, Integration ≥70% (measured with coverage tool)
- **Benchmark Health:** 100% of benchmarks compile and run
- **Documentation Coverage:** 100% of public APIs have rustdoc
- **Snapshot Tests:** At least 10 snapshot tests for CLI output

### 7.2 Qualitative Metrics
- **Developer Confidence:** Team reports increased confidence in code correctness
- **CI Reliability:** CI pipeline runs without failures on main branch
- **Onboarding Time:** New developers can understand and run tests within 1 hour
- **Code Quality:** Zero clippy warnings, zero format violations

### 7.3 Acceptance Criteria (Epic-Level)
✅ **All User Stories Complete:** US-24.1 through US-24.8 have all acceptance criteria met  
✅ **All Benchmarks Pass:** `cargo bench --no-run` compiles successfully  
✅ **All Tests Pass:** `cargo test` passes without ignored tests  
✅ **Coverage Targets Met:** ≥80% unit, ≥70% integration coverage  
✅ **Quality Gates Met:** `make clean-code` passes (fmt, clippy, check)  
✅ **Documentation Complete:** `cargo doc` clean, README/QUICKSTART updated, demo assets created  

---

## 8. Implementation Phases

### Phase 1: Critical Path (Week 1, Days 1-3)
- US-24.1: Fix benchmarks
- US-24.2: Enable prompt generation tests
- US-24.3: Enhance timeout tests

**Deliverable:** All previously broken tests/benchmarks now passing

### Phase 2: Integration Testing (Week 1, Days 4-5)
- US-24.4: Implement Qdrant integration tests

**Deliverable:** RAG pipeline validated end-to-end

### Phase 3: Coverage & Tooling (Week 2, Days 1-2)
- US-24.5: Improve deferred module coverage
- US-24.6: Add CLI snapshot testing

**Deliverable:** Coverage targets met, CLI regression testing in place

### Phase 4: API Validation (Week 2, Days 3-4)
- US-24.7: Implement live API tests (gated)

**Deliverable:** Provider adapters validated against real APIs

### Phase 5: Documentation & Polish (Week 2, Day 5 or extending to Week 3)
- US-24.8: Update all documentation and create demo assets

**Deliverable:** Project is presentation-ready

---

## 9. Open Questions

1. **Coverage Tool Selection:**
   - Should we use `cargo tarpaulin`, `cargo llvm-cov`, or another tool for coverage reporting?
   - **Decision needed by:** Start of implementation
   - **Recommendation:** `cargo llvm-cov` (better maintained, faster)

2. **Live API Test Budget:**
   - What is the acceptable API cost for running live tests in CI?
   - Should we implement request caching to reduce API calls?
   - **Decision needed by:** Before implementing US-24.7
   - **Recommendation:** Cache responses for deterministic tests, limit to 10 API calls per provider per CI run

3. **Snapshot Test Format:**
   - Should CLI snapshots use inline or file-based snapshots?
   - **Decision needed by:** Before implementing US-24.6
   - **Recommendation:** Inline snapshots (easier review in PRs)

4. **Demo Asset Format:**
   - What format for demo assets: GIF, MP4, or terminal recording (asciinema)?
   - **Decision needed by:** Before implementing US-24.8
   - **Recommendation:** Asciinema (text-based, searchable, smaller file size)

5. **Deferred Module Coverage:**
   - If User Service (4.23% coverage) or Listener Service (57.83%) cannot reach 80% within reasonable effort, what is the minimum acceptable threshold?
   - **Decision needed by:** During US-24.5 implementation
   - **Recommendation:** Document reason for lower coverage and create follow-up technical debt ticket

---

## 10. Dependencies

### Upstream Dependencies (Must be complete before starting)
- ✅ Epic 19: Herald & Domain Type Consolidation
- ✅ Epic 20: Vision Pipeline Completion
- ✅ Epic 21: Autonomous Agent Completion
- ✅ Epic 22: Battalion & Commander Hardening
- ✅ Epic 23: CLI, Config & Infrastructure Completion

### External Dependencies
- `insta` crate (for snapshot testing)
- Docker (for Qdrant integration tests)
- Qdrant server (Docker image)
- LLM provider API keys (for live tests - optional)

### Downstream Dependencies
- None (final epic in Milestone 3)

---

## 11. Risks & Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Live API tests fail due to provider outages | Medium | Medium | Tests must be skipped (not fail) when APIs unavailable; use cached responses where possible |
| Qdrant not available in CI | High | Low | Use Docker Compose in CI; provide clear skip logic with helpful error messages |
| Coverage targets unachievable in timeframe | Medium | Medium | Prioritize critical path coverage; document deferred modules with rationale |
| Benchmark API drift more extensive than expected | Medium | Low | Allocate extra time for fixing benchmarks; consider API stability in future design |
| `insta` crate conflicts with existing dependencies | Low | Low | Verify dependency compatibility before starting; have fallback plan for manual assertion-based tests |
| Timeline extends beyond 2 weeks | Medium | Medium | As confirmed by user, extend timeline rather than defer user stories; communicate delays early |

---

## 12. Testing Strategy

### Unit Testing
- Test individual functions and methods in isolation
- Mock external dependencies using `mockall` or manual mocks
- Aim for ≥80% line coverage

### Integration Testing
- Test complete workflows across module boundaries
- Use real external services (Qdrant, Redis) where possible, mock when necessary
- Validate hexagonal architecture boundaries

### Snapshot Testing (NEW)
- Capture CLI output as snapshots using `insta`
- Review snapshots in PRs to catch regressions
- Update snapshots when intentional changes are made

### Live API Testing (NEW)
- Opt-in with feature flag and environment variables
- Test against real LLM provider APIs
- Implement rate limiting and retry logic
- Cache responses where appropriate

### Benchmark Testing
- Compile all benchmarks in CI to catch API drift
- Run benchmarks periodically to detect performance regressions
- Document baseline performance metrics

---

## 13. Rollout Plan

### Phase 1: Local Development
1. Developer implements user story
2. Runs `cargo test` locally
3. Runs `cargo bench --no-run` to verify benchmarks compile
4. Runs `make clean-code` to verify quality gates

### Phase 2: Pull Request
1. PR includes new tests and updated documentation
2. CI runs all tests (except live API tests)
3. Coverage report generated and reviewed
4. Snapshot tests reviewed for intentional changes

### Phase 3: Main Branch
1. PR merged after approval
2. Full benchmark suite run (optional)
3. Live API tests run if secrets configured
4. Coverage metrics tracked over time

### Phase 4: Release
1. Milestone 3 tagged when Epic 24 complete
2. Release notes published
3. Demo assets shared with community
4. Documentation published to project website

---

## 14. Appendix

### A. File Locations

**Benchmark Files:**
- `benches/battalion_benchmarks.rs` (lines 297, 390, 950 need fixes)

**Test Files Needing Attention:**
- `tests/unit/mod.rs` (line 22 - prompt generation tests commented out)
- `tests/unit/paladin_execution_service_test.rs` (lines 237, 239 - ignored timeout test)
- `tests/integration/rag_integration_tests.rs` (line 147 - placeholder integration tests)
- `tests/unit/sanctum/qdrant_sanctum_test.rs` (line 62 - incomplete unit tests)

**Low Coverage Modules:**
- `src/core/platform/manager/user_service.rs` (4.23% coverage)
- `src/core/platform/manager/listener_service.rs` (57.83% coverage)

**Documentation Files:**
- `README.md`
- `docs/QUICKSTART.md`
- `docs/INSTALLATION.md`
- `docs/cli/README.md`
- `docs/BATTALION_BENCHMARKS.md`
- `CONTRIBUTING.md`

### B. Related Epics

- **Epic 6:** Multi-Provider LLM Support (originally deferred live API tests)
- **Epic 8:** Herald Output Formatting (domain type consolidation dependency)
- **Epic 13:** Sentinel Vision System (vision integration tests)
- **Epic 14:** Autonomous Agent Features (prompt generation tests)
- **Epic 18:** CLI & Arsenal Tools (snapshot testing and documentation)

### C. References

- **Project Standards:** `.github/copilot-instructions.md`
- **Rust Guidelines:** `.github/instructions/rust.instructions.md`
- **Testing Standards:** `CONTRIBUTING.md` (to be updated in US-24.8)
- **Hexagonal Architecture:** `notes/hexagonal-arch.md`
- **Task Management:** `.github/prompts/create-prd.prompt.md`

---

## Document Approval

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Product Owner | TBD | | |
| Tech Lead | TBD | | |
| QA Lead | TBD | | |

---

**END OF DOCUMENT**
