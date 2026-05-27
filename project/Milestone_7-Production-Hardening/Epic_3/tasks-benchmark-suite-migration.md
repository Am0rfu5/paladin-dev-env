## Relevant Files

### Files to Modify

- `Cargo.toml` - Workspace benchmark configuration may need updates so root-owned benchmark entries are removed or adjusted after per-crate migration.
- `benches/sanctum_benchmarks.rs` - Existing active benchmark to migrate out of the workspace root.
- `benches/battalion_benchmarks.rs` - Existing battalion benchmark candidate to restore, relocate, or replace.
- `benches/garrison_benchmarks.rs` - Existing garrison benchmark candidate to restore, relocate, or replace.
- `benches/herald_benchmarks.rs` - Existing herald benchmark candidate to assess for deprecation or rewrite.
- `benches/paladin_benchmarks.rs.disabled` - Existing disabled benchmark candidate to assess for deprecation or rewrite.
- `benches/arsenal_benchmarks.rs.disabled` - Existing disabled benchmark candidate to assess for deprecation or rewrite.
- `crates/paladin-memory/Cargo.toml` - Register migrated sanctum benchmark and any new garrison-related benchmarks.
- `crates/paladin-battalion/Cargo.toml` - Register battalion orchestration benchmarks.
- `crates/paladin-llm/Cargo.toml` - Register adapter serialization benchmarks.
- `src/config/settings.rs` - Current `Settings::new()` implementation and config-loading path to benchmark.
- `CHANGELOG.md` - Record formal benchmark deprecations if disabled benchmarks are removed at the root level.

### Files to Create

- `crates/paladin-memory/benches/sanctum_benchmarks.rs` - Migrated sanctum benchmark owned by `paladin-memory`.
- `crates/paladin-memory/benches/garrison_benchmarks.rs` - In-memory read/write benchmark coverage for garrison history sizes.
- `crates/paladin-battalion/benches/battalion_benchmarks.rs` - Formation, Phalanx, and Campaign orchestration benchmarks.
- `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` - Request/response serialization overhead benchmarks excluding network calls.
- `docs/PERFORMANCE_BASELINE.md` - Single performance baseline report with methodology, environment, and results.
- `.github/workflows/ci.yml` - Optional non-blocking benchmark regression reporting if current CI is extended for this Epic.
- `project/Milestone_7-Production-Hardening/Epic_3/benchmark-assessment.md` - Assessment log for disabled benchmark reactivation or deprecation decisions.

### Reference Files

- `project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md` - Source of truth for Epic 3 requirements.
- `project/Milestone_7-Production-Hardening/Epic_3/Milestone_7-Epic_3-Benchmark_Suite_Migration.md` - Epic overview and acceptance criteria.
- `benches/battalion_benchmarks.rs` - Existing disabled battalion benchmark candidate for evaluation.
- `benches/garrison_benchmarks.rs` - Existing disabled garrison benchmark candidate for evaluation.
- `benches/herald_benchmarks.rs` - Existing disabled herald benchmark candidate for evaluation.
- `benches/paladin_benchmarks.rs.disabled` - Existing disabled paladin benchmark candidate for evaluation.
- `benches/arsenal_benchmarks.rs.disabled` - Existing disabled arsenal benchmark candidate for evaluation.

### Notes

- Unit benchmarks in Rust typically live in each crate's `benches/` directory and are run with `cargo bench -p <crate>` or `cargo bench --workspace`.
- Keep benchmark ownership aligned with crate ownership so future maintenance is local to the measured subsystem.
- If a disabled benchmark is removed instead of restored, document the reason clearly in milestone artifacts and changelog material.
- This file now includes parent tasks and implementation-ready subtasks derived from the Epic 3 PRD.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.0 Review benchmark inventory` -> `- [x] 1.0 Review benchmark inventory` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
	- [x] 0.1 Create and checkout a new branch for this Epic 3 work (for example `git checkout -b feature/milestone_7-epic_3-benchmarks`).
- [ ] 1.0 Audit the current benchmark suite and benchmark ownership
	- [ ] 1.1 Read `project/Milestone_7-Production-Hardening/Epic_3/prd-benchmark-suite-migration.md` in full and map each functional requirement to an implementation area.
	- [ ] 1.2 Inventory all files in `benches/` and classify them as active, disabled, or unclear based on the manifest and file naming.
	- [ ] 1.3 Read the current root `Cargo.toml` benchmark-related sections and record how benchmarks are currently enabled, disabled, or commented out.
	- [ ] 1.4 Identify the owning crate for each benchmark area: sanctum and garrison -> `paladin-memory`, battalion -> `paladin-battalion`, LLM serialization -> `paladin-llm`, config loading -> current settings owner.
	- [ ] 1.5 Read `src/config/settings.rs` and confirm whether the config-loading benchmark should live in the root crate or another crate based on the actual `Settings::new()` ownership.
	- [ ] 1.6 Create `project/Milestone_7-Production-Hardening/Epic_3/benchmark-assessment.md` and record the initial inventory, target ownership, and open questions.
- [ ] 2.0 Migrate active workspace benchmarks into their owning crates
	- [ ] 2.1 Create `crates/paladin-memory/benches/` if it does not already exist.
	- [ ] 2.2 Move `benches/sanctum_benchmarks.rs` to `crates/paladin-memory/benches/sanctum_benchmarks.rs`.
	- [ ] 2.3 Update imports, helper paths, and crate references inside the migrated sanctum benchmark so it compiles against `paladin-memory` from its new location.
	- [ ] 2.4 Update `crates/paladin-memory/Cargo.toml` to register the migrated sanctum benchmark with `harness = false` if Criterion is used.
	- [ ] 2.5 Remove or disable the root benchmark entry for sanctum in `Cargo.toml` so the workspace no longer treats it as a root-owned benchmark.
	- [ ] 2.6 Run `cargo bench -p paladin-memory --bench sanctum_benchmarks --no-run` to confirm the migrated benchmark builds.
	- [ ] 2.7 Record the migration result and any API adjustments in `benchmark-assessment.md`.
- [ ] 3.0 Evaluate disabled benchmarks and decide reactivation or deprecation
	- [ ] 3.1 Review `benches/battalion_benchmarks.rs` against the current battalion APIs and decide whether it can be restored directly, requires rewrite, or should be superseded by a new benchmark file.
	- [ ] 3.2 Review `benches/garrison_benchmarks.rs` against the current garrison APIs and decide whether it can be restored directly, requires rewrite, or should be superseded by a new benchmark file.
	- [ ] 3.3 Review `benches/herald_benchmarks.rs` against the current herald APIs and determine whether it still represents a meaningful benchmark target.
	- [ ] 3.4 Review `benches/paladin_benchmarks.rs.disabled` and determine whether the benchmark still maps to a valid architectural boundary after the workspace split.
	- [ ] 3.5 Review `benches/arsenal_benchmarks.rs.disabled` and determine whether the benchmark should be rewritten around current armament types or formally deprecated.
	- [ ] 3.6 For each reviewed benchmark, document one final disposition in `benchmark-assessment.md`: reactivate in owning crate or remove with rationale.
	- [ ] 3.7 Remove any benchmark files that are formally deprecated and update `CHANGELOG.md` or milestone documentation with the removal reason.
	- [ ] 3.8 Relocate any benchmarks that are being restored into the `benches/` directory of their owning crate and update the owning crate manifest accordingly.
- [ ] 4.0 Implement new critical-path benchmarks for battalion, LLM, garrison, and config loading
	- [ ] 4.1 Create `crates/paladin-battalion/benches/` and add `battalion_benchmarks.rs` covering Formation with 3 agents, Phalanx with 5 agents, and Campaign with a branching DAG.
	- [ ] 4.2 Implement or reuse mock `PaladinPort` support inside the battalion benchmark so the measurement isolates orchestration overhead rather than model latency.
	- [ ] 4.3 Create `crates/paladin-memory/benches/garrison_benchmarks.rs` covering in-memory read and write performance at 100, 1000, and 10000 history entries.
	- [ ] 4.4 Create `crates/paladin-llm/benches/` and add `llm_serialization_benchmarks.rs` covering request/response serialization overhead without live HTTP calls.
	- [ ] 4.5 Implement a config benchmark that measures `Settings::new()` and the current per-domain config loading path at the crate that owns that code.
	- [ ] 4.6 Update each owning crate `Cargo.toml` so all new benchmark files are registered and buildable.
	- [ ] 4.7 Run `cargo bench -p paladin-battalion --no-run`, `cargo bench -p paladin-memory --no-run`, and `cargo bench -p paladin-llm --no-run` to verify the new benchmark targets compile.
- [ ] 5.0 Update workspace benchmark manifests and execution flow
	- [ ] 5.1 Review the root `Cargo.toml` and remove obsolete `[[bench]]` entries or commented placeholders that no longer belong at workspace root.
	- [ ] 5.2 Add any missing Criterion dev-dependencies to owning crate manifests if the benchmarks are no longer inherited adequately from the root.
	- [ ] 5.3 Ensure each owning crate manifest lists its benchmark targets consistently and does not rely on root-only benchmark registration.
	- [ ] 5.4 Run `cargo bench --workspace --no-run` and fix manifest or feature-flag issues until the full workspace benchmark suite builds from the workspace entry point.
	- [ ] 5.5 Document any required feature flags or benchmark preconditions in crate-level notes or in the baseline methodology plan.
- [ ] 6.0 Produce the baseline performance report
	- [ ] 6.1 Decide on the benchmark execution environment and record hardware, OS, Rust toolchain, and commit SHA to use in the baseline report.
	- [ ] 6.2 Run the full active benchmark suite and capture results for all migrated and newly added benchmarks.
	- [ ] 6.3 Compare post-migration sanctum results with pre-migration results and record the comparison method and observed variance.
	- [ ] 6.4 Identify any trustworthy historical benchmark data for comparison; explicitly mark areas with no prior comparable baseline.
	- [ ] 6.5 Create `docs/PERFORMANCE_BASELINE.md` with methodology, environment, measured results, comparison notes, and interpretation separated clearly from raw results.
	- [ ] 6.6 Cross-check that every active benchmark is represented in the baseline document.
- [ ] 7.0 Add optional CI regression signaling and complete end-to-end validation
	- [ ] 7.1 Review `.github/workflows/ci.yml` and determine whether benchmark reporting can be added without blocking merges.
	- [ ] 7.2 If implemented, add a non-blocking benchmark regression step or job that surfaces regressions above a documented threshold without failing the required pipeline.
	- [ ] 7.3 Run `cargo bench --workspace --no-run` as the final structural validation of the benchmark suite.
	- [ ] 7.4 Run the narrow benchmark commands for each owning crate to confirm benchmark discovery works from crate scope as well as workspace scope.
	- [ ] 7.5 Review the Epic 3 PRD success metrics and confirm each required benchmark, documentation, and disposition outcome is satisfied.
	- [ ] 7.6 Update this task file as work completes, marking parent tasks done only after all child subtasks are complete and validation has passed.
