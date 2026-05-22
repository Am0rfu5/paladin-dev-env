
## Epic 3: Benchmark Suite Migration and Performance Baseline

**Epic Owner:** TBD
**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (crate structure finalized), Epic 2 (Makefile targets available)

### Objective

Migrate the benchmark suite to the workspace structure, assign benchmarks to their appropriate crates, establish baseline performance measurements for all critical paths, and document the results as the pre-release performance baseline.

### Background & Rationale

The current benchmark suite includes:

- `benches/sanctum_benchmarks.rs` — Vector store performance (search, insert, batch operations).
- Disabled benchmarks: `battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks`, `paladin_benchmarks`, `arsenal_benchmarks` — noted in `Cargo.toml` as needing API fixes.

With the workspace decomposition, benchmarks should live in the crate they measure: sanctum benchmarks in `paladin-memory`, battalion benchmarks in `paladin-battalion`, etc. Additionally, the disabled benchmarks should be evaluated for reactivation now that the API has been refactored.

### Acceptance Criteria

1. Active benchmarks (`sanctum_benchmarks`) migrated to `paladin-memory/benches/`.
2. Disabled benchmarks evaluated; reactivated where possible with the refactored API, or formally deprecated with documented reasons.
3. New benchmarks added for critical paths: battalion orchestration (Formation, Phalanx, Campaign execution), LLM adapter call overhead, garrison read/write, and config loading.
4. `cargo bench --workspace` runs all active benchmarks.
5. Baseline performance document produced with measurements, hardware specification, and methodology.
6. Performance regression CI gate established (optional — flag regressions exceeding a threshold).

### Tasks

#### Task 3.1: Migrate Active Benchmarks to Crate Locations

**Description:** Move `sanctum_benchmarks.rs` to `paladin-memory/benches/`. Update `Cargo.toml` `[[bench]]` entries in the appropriate crate.

**Deliverables:**
- Benchmark files relocated.
- `cargo bench -p paladin-memory` runs sanctum benchmarks.
- Results match pre-migration baseline within noise margin.

**Estimated Effort:** Small

#### Task 3.2: Evaluate and Reactivate Disabled Benchmarks

**Description:** Review each disabled benchmark (`battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks`, `paladin_benchmarks`, `arsenal_benchmarks`) against the current API. Fix or rewrite where the API has changed. Formally deprecate any that are no longer meaningful.

**Deliverables:**
- Assessment document for each disabled benchmark.
- Reactivated benchmarks placed in their appropriate crates.
- Deprecated benchmarks removed with `CHANGELOG.md` entry.

**Estimated Effort:** Medium

#### Task 3.3: Add Critical Path Benchmarks

**Description:** Write new benchmarks for the core performance-sensitive operations:
- Battalion: Formation (3-agent sequential), Phalanx (5-agent parallel), Campaign (DAG with branches). Uses mock `PaladinPort` to isolate orchestration overhead.
- LLM: Adapter request/response serialization overhead (excluding actual HTTP call).
- Garrison: In-memory read/write at various history sizes (100, 1000, 10000 entries).
- Config: `Settings::new()` and per-domain config loading.

**Deliverables:**
- Benchmark files in their respective crates.
- `cargo bench --workspace` exercises all new benchmarks.

**Estimated Effort:** Medium

#### Task 3.4: Document Performance Baseline

**Description:** Run the full benchmark suite on a standardized environment. Produce a performance baseline document with methodology, hardware specs, results, and analysis.

**Deliverables:**
- `docs/PERFORMANCE_BASELINE.md` with all benchmark results.
- Hardware and environment specification.
- Analysis of any performance-sensitive areas.
- Comparison to pre-workspace monolithic build (where comparable benchmarks exist).

**Estimated Effort:** Small

---
