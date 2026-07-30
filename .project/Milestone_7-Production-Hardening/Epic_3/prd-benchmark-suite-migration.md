# PRD: Benchmark Suite Migration and Performance Baseline

**Feature Name:** benchmark-suite-migration
**Milestone:** 7 — Production Hardening (Tier 4)
**Epic:** 3 — Benchmark Suite Migration and Performance Baseline
**Status:** Ready for Implementation
**Created:** 2026-05-27
**Author:** AI-assisted, reviewed by team

---

## 1. Introduction / Overview

Paladin's benchmark suite still reflects the pre-workspace layout. Some benchmarks remain in the workspace-root `benches/` directory, some are disabled because the API changed during the refactor, and there is not yet a single documented baseline for the current multi-crate architecture.

This creates three problems:

- Benchmark ownership is unclear because benchmark files are not located with the crates they measure.
- Performance-sensitive paths introduced or stabilized during the refactor do not yet have consistent benchmark coverage.
- The team does not have a documented, repeatable pre-release baseline for comparing future regressions.

**Goal:** migrate benchmarks into the crates they measure, ensure all benchmark-critical paths listed in Epic 3 are covered, formally remove disabled benchmarks that cannot be meaningfully restored, and publish a single baseline document at `docs/PERFORMANCE_BASELINE.md`.

This PRD assumes Epic 1 and Epic 2 are complete, so the crate layout is stable and workspace-level benchmark commands are available.

---

## 2. Goals

1. Ensure all benchmark-critical paths called out in Epic 3 are measured and baseline-documented.
2. Move active benchmarks into the crate directories that own the measured functionality.
3. Evaluate every currently disabled benchmark and either reactivate it against the refactored API or remove it with documented rationale.
4. Make `cargo bench --workspace` the authoritative way to run all active benchmarks.
5. Produce a single baseline report in `docs/PERFORMANCE_BASELINE.md` with methodology, hardware details, and recorded results.
6. Add an optional CI regression check that flags benchmark regressions without blocking merges.

---

## 3. User Stories

**As a framework maintainer,** I want benchmarks to live inside the crates they measure, so ownership and maintenance responsibility are obvious.

**As a developer changing battalion orchestration code,** I want battalion benchmarks to run from `paladin-battalion`, so I can measure orchestration overhead without relying on unrelated workspace benchmarks.

**As a developer working on LLM adapters,** I want a benchmark for serialization overhead that excludes network calls, so I can isolate adapter cost from provider latency.

**As a release engineer,** I want a single baseline document with hardware specs and benchmark methodology, so I can compare future releases against a stable reference.

**As a reviewer,** I want disabled benchmarks either restored or explicitly removed with reasons, so the benchmark suite does not accumulate abandoned placeholders.

**As a CI maintainer,** I want regression checks to surface benchmark drift without blocking routine development, so the team gets signal early without turning noisy performance variance into merge friction.

---

## 4. Functional Requirements

### 4.1 Benchmark Migration: Active Benchmarks

**FR-01** The existing active benchmark `benches/sanctum_benchmarks.rs` must be moved to `crates/paladin-memory/benches/sanctum_benchmarks.rs`.

**FR-02** `crates/paladin-memory/Cargo.toml` must be updated so the migrated benchmark is registered and runnable from that crate.

**FR-03** The workspace root benchmark configuration must be updated so it no longer treats `sanctum_benchmarks` as a root-owned benchmark after migration.

**FR-04** `cargo bench -p paladin-memory` must execute the migrated sanctum benchmark successfully.

**FR-05** The post-migration sanctum benchmark results must be compared against the pre-migration results and documented as within an acceptable noise margin. The PRD does not require a specific percentage threshold, but the implementation must record the comparison method used.

### 4.2 Disabled Benchmark Evaluation and Disposition

**FR-06** The following disabled benchmarks must each be reviewed against the current API surface:

- `battalion_benchmarks`
- `herald_benchmarks`
- `garrison_benchmarks`
- `paladin_benchmarks`
- `arsenal_benchmarks`

**FR-07** For each disabled benchmark, the team must produce one of two outcomes only:

- Reactivate it in the crate that owns the measured functionality.
- Remove it and document the deprecation reason.

**FR-08** Disabled benchmarks that cannot be meaningfully restored against the refactored API must be removed rather than left disabled for later.

**FR-09** Each removed benchmark must have a short written rationale recorded in the benchmark assessment output, and the removal must be reflected in the relevant `CHANGELOG.md` entry or equivalent milestone documentation.

**FR-10** After this Epic is complete, there must be no benchmark files still marked as disabled in workspace manifests or carried forward as commented-out placeholders without documented disposition.

### 4.3 New Critical Path Benchmarks

**FR-11** New battalion benchmarks must be added under the crate that owns battalion execution and must cover exactly these orchestration scenarios:

- Formation execution with 3 agents in sequence.
- Phalanx execution with 5 agents in parallel.
- Campaign execution using a branching DAG.

**FR-12** Battalion benchmarks must use mock `PaladinPort` implementations or an equivalent mock execution boundary so they measure orchestration overhead rather than external model latency.

**FR-13** A new LLM benchmark must measure request/response serialization overhead only and must explicitly exclude live HTTP calls and remote provider latency.

**FR-14** New garrison benchmarks must measure in-memory read and write operations at history sizes of 100, 1000, and 10000 entries.

**FR-15** A new configuration benchmark must measure `Settings::new()` and the current per-domain configuration loading path.

**FR-16** Critical path benchmarks for this Epic are limited to the set listed in Epic 3 and confirmed by the user:

- Battalion orchestration
- LLM adapter serialization overhead
- Garrison read/write performance
- Config loading

No additional critical-path categories are required by this PRD.

### 4.4 Workspace Benchmark Execution

**FR-17** `cargo bench --workspace` must run all active benchmarks after migration is complete.

**FR-18** Benchmark crates and manifests must be configured so workspace execution does not require manual per-benchmark file selection.

**FR-19** If any benchmark requires feature flags, those requirements must be documented in the crate-level benchmark setup or the baseline methodology section.

### 4.5 Baseline Documentation

**FR-20** A single baseline report must be created at `docs/PERFORMANCE_BASELINE.md`.

**FR-21** The baseline report must include:

- Benchmark execution date
- Hardware specification
- Operating system and Rust toolchain information
- Benchmark methodology
- Raw or summarized results for every active benchmark
- Notes about variance, caveats, or unstable measurements

**FR-22** Where a comparable pre-workspace or pre-migration measurement exists, the baseline report must include a comparison note. Where no comparable historical result exists, the report must explicitly state that the current run is the first baseline.

**FR-23** The baseline report must clearly separate measured results from interpretation so future runs can compare against the recorded numbers without ambiguity.

### 4.6 CI Regression Signaling

**FR-24** A CI-based performance regression check may be added for benchmark reporting, but it must be non-blocking for merges.

**FR-25** If the CI regression check is implemented, it must flag regressions above a documented threshold and surface the result in CI output, a PR comment, or another team-visible report.

**FR-26** Failure or variance in the optional regression check must not fail the required CI pipeline for this Epic.

---

## 5. Non-Goals (Out of Scope)

- Adding benchmark coverage for systems not listed in Epic 3.
- Performing production performance optimization work beyond identifying notable results.
- Benchmarking live provider network latency or end-to-end HTTP performance for external services.
- Creating a blocking performance gate in CI.
- Preserving disabled benchmarks in place for possible later investigation.
- Reworking unrelated crate APIs solely to save an obsolete benchmark.

---

## 6. Design Considerations

### 6.1 Benchmark Ownership

Benchmark files should live in the crate that owns the measured implementation. At minimum, the target layout should follow this pattern:

- `crates/paladin-memory/benches/` for sanctum and garrison benchmarks
- `crates/paladin-battalion/benches/` for formation, phalanx, and campaign benchmarks
- `crates/paladin-llm/benches/` for adapter serialization benchmarks
- The crate that owns application configuration loading for config benchmarks

### 6.2 Benchmark Isolation

New benchmark scenarios should isolate framework overhead from external system latency wherever possible. This is especially important for battalion and LLM benchmarks.

### 6.3 Deprecation Standard

If an old benchmark no longer maps cleanly to the refactored architecture, removal is preferred over keeping a broken or misleading benchmark. The removal must explain why the benchmark is obsolete, what replaced it if anything, and why restoration is not the right outcome.

### 6.4 Baseline Repeatability

The baseline document should make it possible for another developer to rerun the suite under similar conditions. Methodology notes should be explicit enough for a junior developer to repeat the process without guessing.

---

## 7. Technical Considerations

- Epic 1 is complete, so benchmark files should now target the extracted crate boundaries rather than the old monolithic layout.
- Epic 2 is complete, so workspace-level benchmark commands and build tooling are assumed to be available.
- Existing benchmark code may require API adaptation because the public interfaces changed during Milestones 5 through 7.
- Battalion benchmark implementations should avoid real LLM or network dependencies and use mock ports.
- LLM benchmarks must measure serialization and adapter overhead only; using live provider calls would make results noisy and unsuitable as a framework baseline.
- Config benchmarks should target the current settings-loading path used by the refactored workspace, not legacy configuration entry points that no longer control runtime behavior.

---

## 8. Success Metrics

1. `cargo bench --workspace` completes successfully and runs all active benchmarks.
2. `sanctum_benchmarks` runs from `paladin-memory` after migration.
3. Every currently disabled benchmark is either reactivated in the proper crate or removed with documented rationale.
4. New benchmarks exist for all four required critical-path categories confirmed by the user.
5. `docs/PERFORMANCE_BASELINE.md` is published and includes methodology, hardware details, and recorded results.
6. If a CI regression check is added, it reports regressions without blocking merges.

---

## 9. Open Questions

1. Which existing crate should own the configuration-loading benchmark if configuration code is still shared across multiple crates rather than clearly isolated in one crate?
2. What threshold should the team adopt for the optional CI regression signal so it is useful without becoming noisy?
3. Which historical measurements, if any, are trustworthy enough to treat as pre-migration comparison points for the baseline report?
