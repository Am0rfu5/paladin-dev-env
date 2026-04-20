# Product Requirements Document: CLI Isolation from Library Compilation Path

**Project:** Paladin Framework Refactoring Initiative
**Epic:** Epic 3 - CLI Isolation
**Milestone:** Tier 1 - High-Value, Low-Risk Improvements
**PRD Version:** 1.0
**Status:** Draft
**Author:** AI Assistant
**Created:** 2026-04-20
**Last Updated:** 2026-04-20

---

## 1. Introduction/Overview

The Paladin framework currently includes a comprehensive command-line interface (CLI) totaling 12,000 lines of code in `src/application/cli/` with 193 dedicated tests. While the CLI is architecturally well-separated following hexagonal principles, it remains part of the main library compilation path. This means every downstream consumer who adds `paladin` as a library dependency must compile the entire CLI stack and its 8+ CLI-only dependencies (`clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored`, `console`, `serde_yaml`, and related crates), even if they never use the CLI.

This feature will isolate the CLI from the library compilation path, ensuring that:
- Library consumers only compile core framework functionality
- CLI code and dependencies compile only when building the `paladin-cli` binary target
- Build times and dependency overhead are reduced for the primary library use case (agent orchestration)

**Context:** Epic 1 (Feature Flag Expansion) and Epic 2 (API Hardening) are already complete. This Epic can proceed immediately and will benefit from the feature flag infrastructure established in Epic 1.

**Problem Statement:** Unnecessary compilation overhead for library consumers who don't use the CLI, resulting in longer build times, larger dependency trees, and increased binary sizes.

**Solution:** Gate the CLI module tree and its exclusive dependencies behind a `cli` feature flag, ensuring the CLI compiles only through binary build paths.

---

## 2. Goals

The primary objectives for this feature are:

1. **Reduce Library Build Overhead:** Eliminate CLI compilation from library-only builds to reduce any measurable build time and dependency count.

2. **Maintain CLI Functionality:** Ensure both `paladin` and `paladin-cli` binary targets continue to function correctly with the CLI feature enabled.

3. **Preserve Test Coverage:** Keep all 193 CLI tests passing when the `cli` feature is enabled, with appropriate feature gating to prevent dead-code warnings.

4. **Enable Comprehensive Testing:** Add specific integration tests that verify library-only usage scenarios work correctly without CLI dependencies.

5. **Document Build Impact:** Measure and document the before/after build time improvements and dependency tree reductions to validate the optimization.

6. **Maintain Backward Compatibility:** Ensure existing binary users experience no disruption (CLI remains available in binary builds).

7. **Establish Architecture Guidelines:** Document architectural decisions for handling `src/main.rs` vs `paladin-cli` binary targets for future reference.

---

## 3. User Stories

### User Story 1: Library Consumer (Agent Orchestration Developer)
**As a** developer building an agent orchestration application using Paladin as a library,
**I want** to compile only the core framework functionality without CLI dependencies,
**So that** my build times are faster and my dependency tree is smaller.

**Acceptance Criteria:**
- Running `cargo build` on my project that depends on `paladin` does not compile `clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored`, or `console`.
- My application binary size does not include unused CLI code.
- All framework functionality (Paladin agents, Battalion orchestration, Arsenal tools) works without the `cli` feature.

### User Story 2: CLI User (DevOps/Platform Engineer)
**As a** platform engineer using the `paladin-cli` command-line tool,
**I want** the CLI to continue working exactly as before,
**So that** my workflows and automation scripts are not disrupted.

**Acceptance Criteria:**
- Running `cargo build --bin paladin-cli` compiles successfully with all CLI functionality.
- All CLI commands (`paladin-cli agent run`, `paladin-cli battalion execute`, etc.) work correctly.
- All 193 CLI tests pass when running `cargo test --features cli`.

### User Story 3: Framework Maintainer
**As a** Paladin framework maintainer,
**I want** clear architectural boundaries between library and CLI code,
**So that** I can evolve each independently without cross-contamination.

**Acceptance Criteria:**
- CLI modules are clearly gated with `#[cfg(feature = "cli")]`.
- CI pipeline tests both library-only and CLI-enabled builds.
- Documentation explains when to use each binary target (`paladin` vs `paladin-cli`).

### User Story 4: Integration Test Developer
**As a** developer writing integration tests for Paladin,
**I want** dedicated tests that verify library-only usage scenarios,
**So that** we catch regressions where CLI dependencies leak into library code.

**Acceptance Criteria:**
- Integration tests exist that build library-only configurations and verify core functionality.
- Tests fail if CLI dependencies are required when `cli` feature is disabled.
- CI runs these tests in the feature matrix.

---

## 4. Functional Requirements

### FR1: CLI Module Feature Gating
**Priority:** Critical
**Description:** The entire `src/application/cli/` module tree must be gated behind a `cli` feature flag using `#[cfg(feature = "cli")]` guards.

**Requirements:**
- FR1.1: Add `#[cfg(feature = "cli")]` to the `application::cli` module declaration in `src/application/mod.rs`.
- FR1.2: Apply `#[cfg(feature = "cli")]` to all sub-modules within `src/application/cli/`.
- FR1.3: Gate all CLI type re-exports in `src/lib.rs` with `#[cfg(feature = "cli")]`.
- FR1.4: Ensure no dead-code warnings when `cli` feature is disabled.

### FR2: CLI Dependency Isolation
**Priority:** Critical
**Description:** CLI-only dependencies must not compile when building the library without the `cli` feature.

**Requirements:**
- FR2.1: Mark CLI-exclusive dependencies as `optional = true` in `Cargo.toml` `[dependencies]` section.
- FR2.2: Create a `cli` feature in `[features]` section that enables all CLI-optional dependencies.
- FR2.3: Identify and classify each dependency as "CLI-only", "shared", or "core".
- FR2.4: Verify `cargo tree --no-default-features` excludes CLI dependencies.
- FR2.5: Verify `cargo tree --features cli` includes CLI dependencies.

**CLI-Only Dependencies (Preliminary List):**
- `clap` (command-line argument parsing)
- `dialoguer` (interactive prompts)
- `indicatif` (progress bars)
- `comfy-table` (table formatting)
- `colored` (terminal colors)
- `console` (terminal utilities)
- Any CLI-specific `serde_yaml` usage (if not shared with core config)

### FR3: Binary Target Configuration
**Priority:** Critical
**Description:** Both binary targets must be updated to enable the `cli` feature and compile successfully.

**Requirements:**
- FR3.1: Update `src/main.rs` to enable `cli` feature via `#[cfg(feature = "cli")]` or crate features.
- FR3.2: Update `src/bin/paladin-cli.rs` to enable `cli` feature.
- FR3.3: Document the relationship and intended use case for each binary target.
- FR3.4: Determine architecture for `src/main.rs` vs `paladin-cli` - requires architecture review (see Open Questions).
- FR3.5: Ensure `cargo build --bin paladin` succeeds.
- FR3.6: Ensure `cargo build --bin paladin-cli` succeeds.

### FR4: CLI Test Isolation
**Priority:** High
**Description:** All 193 CLI tests must be feature-gated to compile only when the `cli` feature is enabled.

**Requirements:**
- FR4.1: Wrap all CLI test modules with `#[cfg(feature = "cli")]`.
- FR4.2: Ensure all CLI tests pass with `cargo test --features cli`.
- FR4.3: Ensure no CLI test failures or warnings with `cargo test --lib --no-default-features`.
- FR4.4: Verify snapshot test files (`.snap`) remain functional.
- FR4.5: Update CI pipeline to run CLI tests with `--features cli`.

### FR5: Library-Only Build Verification
**Priority:** Critical
**Description:** The library must compile and function correctly without CLI code or dependencies.

**Requirements:**
- FR5.1: `cargo build --lib --no-default-features` must succeed.
- FR5.2: `cargo build --lib` (with default features but no `cli`) must succeed.
- FR5.3: All core framework functionality (Paladin, Battalion, Arsenal, Garrison) available without `cli`.
- FR5.4: Zero CLI code or dependencies in `cargo tree --lib --no-default-features` output.

### FR6: Integration Testing for Library-Only Scenarios
**Priority:** High
**Description:** Add comprehensive integration tests that verify library-only usage scenarios.

**Requirements:**
- FR6.1: Create integration test suite in `tests/library_only_integration/` (or similar).
- FR6.2: Tests must verify core agent orchestration functionality without CLI.
- FR6.3: Tests must verify Battalion patterns (Formation, Phalanx, Campaign, Chain of Command) work without CLI.
- FR6.4: Tests must verify Arsenal tool execution works without CLI.
- FR6.5: Tests must verify Garrison (memory) operations work without CLI.
- FR6.6: Tests must explicitly fail if CLI dependencies are detected.
- FR6.7: CI runs these tests with `--no-default-features` or minimal feature set.

### FR7: Build Time Measurement and Documentation
**Priority:** Medium
**Description:** Measure and document the build time improvements and dependency reductions.

**Requirements:**
- FR7.1: Capture baseline measurements before changes (clean build, incremental build).
- FR7.2: Capture post-isolation measurements for library-only builds.
- FR7.3: Capture post-isolation measurements for CLI-enabled builds.
- FR7.4: Document dependency count before/after using `cargo tree`.
- FR7.5: Document results in milestone completion report.
- FR7.6: Any measurable improvement is acceptable (no specific % target required).

### FR8: CI Pipeline Updates
**Priority:** High
**Description:** Update CI to test both library-only and CLI-enabled configurations.

**Requirements:**
- FR8.1: Add CI matrix entry for `cargo build --lib --no-default-features`.
- FR8.2: Add CI matrix entry for `cargo build --lib` (default features, no CLI).
- FR8.3: Add CI matrix entry for `cargo build --bin paladin`.
- FR8.4: Add CI matrix entry for `cargo build --bin paladin-cli`.
- FR8.5: Add CI matrix entry for `cargo test --features cli` (CLI tests).
- FR8.6: Add CI matrix entry for integration tests in library-only mode.

### FR9: Documentation Updates
**Priority:** Medium
**Description:** Update project documentation to reflect CLI isolation changes.

**Requirements:**
- FR9.1: Update `README.md` with `cli` feature flag documentation.
- FR9.2: Update `CONTRIBUTING.md` with CLI testing requirements.
- FR9.3: Document binary target architecture decisions (after architecture review).
- FR9.4: Update migration guide (part of cross-epic deliverables).
- FR9.5: Add `CHANGELOG.md` entry documenting the CLI isolation change.

---

## 5. Non-Goals (Out of Scope)

The following items are explicitly **out of scope** for this Epic:

1. **CLI Feature Additions:** No new CLI commands, features, or UX improvements. This Epic focuses solely on isolation, not enhancement.

2. **CLI Code Refactoring:** No changes to CLI internal structure, architecture, or test organization beyond feature gating.

3. **Workspace Decomposition:** CLI will not be extracted into a separate crate in this Epic (reserved for Tier 2 workspace refactor).

4. **Binary Distribution Changes:** No changes to how binaries are packaged, distributed, or installed.

5. **Performance Optimization:** Beyond build-time reduction from dependency elimination, no runtime performance work is in scope.

6. **CLI Configuration Migration:** No changes to how CLI configuration files (`config.yml`, CLI-specific settings) are loaded or structured.

7. **Alternative CLI Frameworks:** No migration from `clap` to other CLI frameworks.

8. **CLI Stability Guarantees:** CLI remains an unstable/experimental interface; no stability contract is established in this Epic.

9. **Web UI Integration:** No work on web-based alternatives or GraphQL/REST API wrappers for CLI functionality.

---

## 6. Design Considerations

### 6.1 Feature Flag Strategy

**Decision:** Use a single `cli` feature flag rather than multiple granular CLI feature flags.

**Rationale:** The CLI is a cohesive unit. Users either need the full CLI or don't need it at all. Granular flags (e.g., `cli-agent`, `cli-battalion`, `cli-arsenal`) add complexity without providing meaningful value.

**Implementation:**
```toml
[features]
cli = [
    "clap",
    "dialoguer",
    "indicatif",
    "comfy-table",
    "colored",
    "console",
]
```

### 6.2 Binary Target Architecture

**Open Decision:** Requires architecture review (see Open Questions section).

**Options under consideration:**
- **Option A:** Keep both binaries - `paladin` for basic use, `paladin-cli` for full CLI
- **Option B:** Consolidate to single `paladin-cli` binary, deprecate `paladin`
- **Option C:** Keep `paladin` as lightweight wrapper, move all CLI logic to `paladin-cli`

**Constraints:**
- Both binaries currently exist in the codebase
- Must maintain backward compatibility for existing users
- Decision impacts documentation and user migration

### 6.3 Module Gating Pattern

**Pattern:** Apply `#[cfg(feature = "cli")]` at the highest module boundary possible.

**Example:**
```rust
// src/application/mod.rs
#[cfg(feature = "cli")]
pub mod cli;
```

Rather than gating individual types within the CLI module. This minimizes conditional compilation complexity.

### 6.4 Test Organization

**Pattern:** Keep CLI tests in their current location with feature guards.

**Rationale:**
- Maintains existing test organization
- No need to move 193 test files
- Feature guards are sufficient for isolation
- Snapshot files remain in place

### 6.5 Shared Dependencies

**Decision:** Dependencies used by both library and CLI remain unconditional.

**Examples of shared dependencies:**
- `serde`, `serde_json` (used everywhere)
- `tokio` (async runtime)
- `anyhow`, `thiserror` (error handling)
- Core domain dependencies

Only mark dependencies as optional if they are **exclusively** used by CLI code.

---

## 7. Technical Considerations

### 7.1 Dependency Analysis Scope

**Task 3.1** must produce a comprehensive dependency classification matrix identifying:
- Which dependencies are CLI-only
- Which are shared between library and CLI
- Which have transitive dependencies that might be CLI-only
- Any version conflicts or feature conflicts

### 7.2 Feature Flag Testing Matrix

The CI pipeline must test the following configurations at minimum:

| Configuration | Command | Purpose |
|---------------|---------|---------|
| Library minimal | `cargo build --lib --no-default-features` | Verify core compiles without any optional features |
| Library default | `cargo build --lib` | Verify default library build excludes CLI |
| CLI binary | `cargo build --bin paladin-cli` | Verify CLI binary compiles |
| Main binary | `cargo build --bin paladin` | Verify main binary compiles |
| CLI tests | `cargo test --features cli` | Verify all 193 CLI tests pass |
| Library-only integration | `cargo test --test library_only_*` | Verify library-only integration tests |

### 7.3 Breaking Change Classification

**Assessment:** This is a **breaking change** for downstream consumers who:
- Explicitly import CLI types from the library
- Depend on CLI functionality being available by default

**Mitigation:**
- This is acceptable per requirement 2C (breaking changes acceptable, major version bump)
- Migration guide will document required changes
- `CHANGELOG.md` will clearly mark this as a breaking change

### 7.4 Integration with Epic 1 Feature Flags

**Benefit:** Epic 1 already established feature flag infrastructure and CI matrix testing.

**Leveraging Epic 1:**
- CI matrix configuration patterns can be reused
- Feature flag documentation format is established
- Conditional compilation patterns are proven

### 7.5 Build System Considerations

**Cargo Feature Unification:** When building binaries, Cargo unifies features across all dependencies. This means:
- Binary builds automatically enable required library features
- No need for complex feature forwarding
- Binary `Cargo.toml` sections or build scripts can specify required features

### 7.6 Documentation Generation

**Consideration:** `cargo doc` must handle feature-gated modules gracefully.

**Requirements:**
- CLI modules should be documented when `--features cli` is used
- Library documentation should not show broken CLI links when `cli` feature is disabled
- Public API documentation (from Epic 2) should be clean regardless of feature flags

---

## 8. Success Metrics

### 8.1 Build Time Metrics

**Baseline (Before):**
- Clean library build time: [To be measured in Task 3.5]
- Incremental library build time: [To be measured in Task 3.5]
- Dependency count (library): [To be measured in Task 3.5]

**Target (After):**
- Any measurable reduction in clean build time is acceptable
- Any measurable reduction in incremental build time is acceptable
- Measurable reduction in dependency count

**Success Criteria:** Document before/after measurements showing improvement.

### 8.2 Test Coverage Metrics

**Baseline:**
- 193 CLI tests currently exist
- All tests must continue passing

**Target:**
- 193 CLI tests pass with `--features cli`
- Zero CLI test failures with library-only builds
- New integration tests added for library-only scenarios (minimum 5 tests covering core functionality)

**Success Criteria:** All tests green in both configurations.

### 8.3 Dependency Reduction Metrics

**Measurable via `cargo tree`:**
- Count of direct dependencies in library-only build
- Count of transitive dependencies in library-only build
- Absence of CLI-only dependencies (`clap`, `dialoguer`, `indicatif`, etc.) in library tree

**Success Criteria:** CLI dependencies absent from `cargo tree --lib --no-default-features` output.

### 8.4 CI Pipeline Metrics

**Baseline:**
- Current CI test matrix coverage: [Document current state]

**Target:**
- At least 6 new CI matrix entries covering library/CLI combinations (per FR8)
- All matrix entries green

**Success Criteria:** Updated CI configuration committed and passing.

### 8.5 Code Quality Metrics

**Continuous Requirements:**
- Zero warnings from `cargo clippy -- -D warnings`
- All code passes `cargo fmt --check`
- No dead-code warnings in any feature configuration
- `cargo doc --no-deps` builds cleanly

**Success Criteria:** Quality gates pass for all feature combinations.

---

## 9. Open Questions

### Q1: Binary Target Architecture Decision (Critical)

**Question:** What is the intended relationship between `src/main.rs` (the `paladin` binary) and `src/bin/paladin-cli.rs`?

**Options:**
- A. Keep both binaries with distinct purposes (need to document what each does)
- B. Consolidate to single `paladin-cli` binary
- C. Make `paladin` a lightweight wrapper
- D. Defer decision to architecture review meeting

**Impact:** High - affects user documentation, migration guide, and binary distribution strategy.

**Decision Maker:** Project architect / Epic owner

**Deadline:** Before Task 3.3 (Update Binary Entry Points)

**Status:** User selected Option D - requires architecture review

---

### Q2: CLI Feature in Default Feature Set

**Question:** Should the `cli` feature be included in the `default` feature set?

**Context:**
- If `cli` is in default, library consumers must explicitly disable it with `default-features = false`
- If `cli` is not in default, binary builders must explicitly enable it with `features = ["cli"]`

**Recommendation:** **Do not** include `cli` in default features. Rationale:
- Library use case is the primary framework use case
- Binary builds can easily add `features = ["cli"]`
- Aligns with principle of opt-in for optional functionality

**Decision Maker:** Epic owner

**Deadline:** Before Task 3.2 (Gate CLI Module Behind Feature Flag)

---

### Q3: Shared `serde_yaml` Usage

**Question:** Is `serde_yaml` used exclusively by CLI config loading, or is it also used by library configuration?

**Context:**
- If CLI-only, mark it as optional and gate behind `cli` feature
- If shared with library config, keep it as core dependency

**Action Required:** Analyze during Task 3.1 (Analyze CLI Dependency Boundaries)

**Impact:** Medium - affects dependency classification

---

### Q4: Integration Test Placement

**Question:** Where should library-only integration tests be located?

**Options:**
- A. New directory: `tests/library_only/`
- B. Existing integration test directory with feature guards
- C. New top-level directory: `integration_tests/`
- D. Within `tests/` with clear naming convention: `tests/library_only_*.rs`

**Recommendation:** Option D - clear naming convention in existing `tests/` directory.

**Decision Maker:** Epic owner

**Deadline:** Before Task implementation begins

---

### Q5: CLI Stability Contract

**Question:** Should this Epic establish any stability guarantees for the CLI interface?

**Context:**
- CLI is currently experimental/unstable
- Epic 2 established stability contracts for port traits
- CLI isolation makes CLI more clearly a separate concern

**Recommendation:** Defer to future Epic. This Epic focuses on isolation, not stabilization.

**Decision Maker:** Project lead

---

### Q6: Breaking Change Version Bump

**Question:** Will this Epic trigger a major version bump (e.g., 0.x.0 → 0.y.0 or 1.0.0)?

**Context:**
- User selected 2C: Breaking changes acceptable
- Need to coordinate with Epic 1 and Epic 2 version changes
- All three Epics may be released together as single major version

**Recommendation:** Coordinate version bump across all three Tier 1 Epics in milestone wrap-up phase.

**Decision Maker:** Project lead

**Deadline:** Before milestone release

---

## 10. Dependencies and Sequencing

### Prerequisites

**Completed:**
- ✅ Epic 1: Feature Flag Expansion (provides feature flag infrastructure)
- ✅ Epic 2: API Hardening (provides stable public API definition)

**Required Before Starting:**
- CI infrastructure supports feature matrix testing
- Test suite is green on current branch

### Task Sequencing

**Phase 1: Analysis (Sprint 1)**
1. Task 3.1: Analyze CLI Dependency Boundaries
2. Resolve Q2 (CLI in default features?)
3. Resolve Q3 (serde_yaml classification)
4. Resolve Q4 (integration test placement)

**Phase 2: Implementation (Sprint 1-2)**
5. Task 3.2: Gate CLI Module Behind Feature Flag
6. Task 3.3: Update Binary Entry Points (blocked by Q1 resolution)
7. Task 3.4: Migrate CLI Tests to Feature-Gated Compilation

**Phase 3: Verification (Sprint 2)**
8. Task 3.5: Measure and Document Build Time Impact
9. FR6: Add Library-Only Integration Tests
10. FR8: Update CI Pipeline

**Phase 4: Documentation (Sprint 2)**
11. FR9: Documentation Updates
12. Contribution to cross-epic migration guide

---

## 11. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Binary target architecture decision delayed | Medium | High | Escalate Q1 to project architect immediately; provide recommendation with pros/cons for quick decision |
| CLI dependencies have unexpected shared usage | Low | Medium | Comprehensive analysis in Task 3.1; conservative classification (when in doubt, keep unconditional) |
| Integration tests reveal CLI leakage into library | Low | High | This is actually a success - tests are working as intended; fix any leakage discovered |
| Build time improvement is negligible | Low | Low | Any measurable improvement is acceptable per success criteria; even small wins compound |
| Snapshot tests break with feature guards | Low | Medium | Test CLI with feature enabled; snapshot files remain committed; CI runs correct configuration |
| Transitive dependency conflicts | Low | Medium | Use `cargo tree` analysis to detect conflicts early; may need to adjust feature flag granularity |

---

## 12. Acceptance Checklist

The Epic is considered complete when:

- [ ] **FR1-FR9:** All functional requirements implemented and verified
- [ ] **Q1-Q6:** All critical open questions resolved and documented
- [ ] **Build Verification:**
  - [ ] `cargo build --lib --no-default-features` succeeds
  - [ ] `cargo build --lib` succeeds without CLI
  - [ ] `cargo build --bin paladin` succeeds
  - [ ] `cargo build --bin paladin-cli` succeeds
- [ ] **Test Verification:**
  - [ ] All 193 CLI tests pass with `--features cli`
  - [ ] No CLI test failures with library-only builds
  - [ ] New library-only integration tests added and passing
- [ ] **Dependency Verification:**
  - [ ] `cargo tree --lib --no-default-features` excludes CLI dependencies
  - [ ] CLI dependencies present only in binary builds
- [ ] **Quality Gates:**
  - [ ] `cargo clippy -- -D warnings` passes for all feature combinations
  - [ ] `cargo fmt --check` passes
  - [ ] `cargo doc --no-deps` builds cleanly
- [ ] **CI Pipeline:**
  - [ ] All 6+ matrix entries configured and passing
- [ ] **Metrics:**
  - [ ] Build time measurements documented
  - [ ] Dependency reduction documented
- [ ] **Documentation:**
  - [ ] `README.md` updated
  - [ ] `CONTRIBUTING.md` updated
  - [ ] `CHANGELOG.md` entry added
  - [ ] Migration guide contribution prepared
  - [ ] Binary architecture decision documented
- [ ] **Code Review:**
  - [ ] All changes peer reviewed
  - [ ] Architecture review completed for binary targets

---

## 13. Appendix: Task Breakdown from Milestone Document

The following tasks are defined in the Milestone Overview document:

### Task 3.1: Analyze CLI Dependency Boundaries
- **Effort:** Small
- **Deliverables:** CLI dependency analysis document, gating strategy recommendation, impact assessment

### Task 3.2: Gate CLI Module Behind Feature Flag
- **Effort:** Medium
- **Deliverables:** Updated Cargo.toml, #[cfg] guards, updated lib.rs exports, binary targets updated

### Task 3.3: Update Binary Entry Points
- **Effort:** Small
- **Deliverables:** Updated binary source files, build verification
- **Blocker:** Requires Q1 (binary architecture) resolution

### Task 3.4: Migrate CLI Tests to Feature-Gated Compilation
- **Effort:** Small
- **Deliverables:** Feature-gated test modules, CI updates, snapshot verification

### Task 3.5: Measure and Document Build Time Impact
- **Effort:** Small
- **Deliverables:** Before/after measurements, dependency tree comparison, results documentation

---

## Document Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-20 | AI Assistant | Initial PRD creation based on Epic 3 specification and user clarifications |

---

**Next Steps:**
1. Review and approve this PRD
2. Resolve critical Open Question Q1 (binary architecture)
3. Begin Phase 1: Analysis (Task 3.1)
4. Create task tracking file following project task list conventions
