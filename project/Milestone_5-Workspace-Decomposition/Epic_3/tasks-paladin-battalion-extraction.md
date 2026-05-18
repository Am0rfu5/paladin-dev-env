## Relevant Files

- `crates/paladin-battalion/Cargo.toml` — New crate manifest; declares workspace dependency references.
- `crates/paladin-battalion/src/lib.rs` — New crate root; crate-level `//!` doc comment and all `pub mod` declarations.
- `crates/paladin-battalion/src/formation_service.rs` — Moved from `src/application/use_cases/battalion/`; sequential execution service.
- `crates/paladin-battalion/src/phalanx_service.rs` — Moved from `src/application/use_cases/battalion/`; parallel execution service.
- `crates/paladin-battalion/src/campaign_service.rs` — Moved from `src/application/use_cases/battalion/`; DAG/graph orchestration service.
- `crates/paladin-battalion/src/chain_of_command_service.rs` — Moved from `src/application/use_cases/battalion/`; hierarchical delegation service.
- `crates/paladin-battalion/src/conclave_execution_service.rs` — Moved from `src/application/use_cases/battalion/`; mixture-of-experts synthesis service.
- `crates/paladin-battalion/src/council_service.rs` — Moved from `src/application/use_cases/battalion/`; discussion orchestration service.
- `crates/paladin-battalion/src/grove_service.rs` — Moved from `src/application/use_cases/battalion/`; intelligent routing service.
- `crates/paladin-battalion/src/maneuver_service.rs` — Moved from `src/application/use_cases/battalion/`; flow DSL execution service.
- `crates/paladin-battalion/src/commander.rs` — Moved from `src/application/use_cases/battalion/`; strategy router with auto-detection heuristics.
- `crates/paladin-battalion/src/error_aggregation.rs` — Moved from `src/application/use_cases/battalion/`; error collection utility for ContinueOnError strategy.
- `crates/paladin-battalion/src/flow_visualizer.rs` — Moved from `src/application/use_cases/battalion/`; battalion execution flow visualizer.
- `crates/paladin-battalion/src/retry.rs` — Moved from `src/application/use_cases/battalion/`; exponential backoff retry utility.
- `Cargo.toml` (root) — Add `paladin-battalion = { path = "crates/paladin-battalion" }` to `[workspace.dependencies]` and to the root `paladin` crate `[dependencies]`.
- `src/lib.rs` — Add `pub use paladin_battalion;` re-export so battalion types are available through the facade crate.
- `src/application/use_cases/battalion/mod.rs` — Converted to a re-export shim (`pub use paladin_battalion::*;`) after originals are moved; the only file that remains in the directory.
- `src/application/mod.rs` — Update `use_cases::battalion` doc comments to reference `paladin_battalion::`.
- `project/Milestone_5-Workspace-Decomposition/Epic_3/baseline-test-count.txt` — Pre-epic test baseline (created in Task 1.1).
- `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-isolation-build.txt` — Output of `cargo build -p paladin-battalion` (created in Task 4.5).
- `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-dependency-tree.txt` — Output of `cargo tree -p paladin-battalion` (created in Task 4.4).
- `tests/unit/battalion/` — Workspace-level unit tests for battalion services; remain in place and must continue to pass.
- `tests/integration/battalion/` — Workspace-level integration tests; remain in place and must continue to pass.

### Notes

- This is a structural refactor. No behavioral or public-API-shape changes are permitted. All existing tests must continue to pass.
- Follow Rust TDD discipline: `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` must pass before marking any parent task complete.
- Use `cargo build -p paladin-battalion` and `cargo test -p paladin-battalion` to validate the extracted crate in isolation.
- Use `cargo tree -p paladin-battalion` to verify no transitive dependencies on LLM SDKs, DB drivers, or storage clients.
- **Do NOT delete `src/application/use_cases/battalion/` source files during Tasks 2.0 or 3.0.** The originals remain in place until all import sites have been migrated in Task 5.0 and `cargo test --workspace` is green.
- The in-module import migration (~169 occurrences) is best handled with the `sed` one-liner in Section 6 above, followed by `cargo build -p paladin-battalion` to catch any misses.
- `PaladinError` (from `crate::application::use_cases::paladin::error`) is used by `phalanx_service.rs`. Confirm whether it is accessible via `paladin_ports::` or needs a separate resolution before extracting that file.
- Workspace dependency versions use `dep = { workspace = true }` syntax in member manifests.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Confirm the current branch is `feature/milestone_5` via `git branch --show-current`
  - [x] 0.2 From `feature/milestone_5`, create and checkout a new feature branch: `git checkout -b feature/milestone_5-epic_3-battalion-extraction`
  - [x] 0.3 Push the branch to origin to establish the upstream: `git push -u origin feature/milestone_5-epic_3-battalion-extraction`

- [x] 1.0 Capture baseline and scaffold `paladin-battalion` crate skeleton
  - [x] 1.1 Capture the pre-epic baseline: run `cargo test --workspace` and record the passing test count in `project/Milestone_5-Workspace-Decomposition/Epic_3/baseline-test-count.txt` (include branch name and date)
  - [x] 1.2 Audit feature-flag guards: run `grep -rn "#\[cfg(feature" src/application/use_cases/battalion/` and record any feature gates that must be carried into `paladin-battalion/Cargo.toml`
  - [x] 1.3 Audit `PaladinError` usage: run `grep -rn "use_cases::paladin::error\|PaladinError" src/application/use_cases/battalion/` and confirm `PaladinError` is accessible via `paladin_ports::` or a path resolvable in `paladin-battalion`
  - [x] 1.4 Create the directory `crates/paladin-battalion/src/`
  - [x] 1.5 Create `crates/paladin-battalion/Cargo.toml` with `name = "paladin-battalion"`, `version` matching the workspace `[workspace.package]` version, `edition = "2021"`, and `license` consistent with the root crate
  - [x] 1.6 Add `[dependencies]` to `crates/paladin-battalion/Cargo.toml` using workspace references for all deps listed in FR-2: `paladin-core`, `paladin-ports`, `tokio`, `async-trait`, `serde`, `serde_json`, `uuid`, `log`, `futures`, `chrono`, `rand`, `tokio-util`, `petgraph`, `regex` — and only these
  - [x] 1.7 Create `crates/paladin-battalion/src/lib.rs` with crate-level `//!` doc comment, `#![deny(unused_imports, dead_code)]` lint attributes consistent with the root crate, and empty `pub mod` placeholders (commented out) for all 13 modules
  - [x] 1.8 Add `paladin-battalion = { path = "crates/paladin-battalion" }` to the `[workspace.dependencies]` section of the root `Cargo.toml`
  - [x] 1.9 Run `cargo build -p paladin-battalion` and confirm the empty skeleton crate compiles in isolation
  - [x] 1.10 Run `cargo build --workspace` and confirm both the empty `paladin-battalion` and the existing `paladin` crate still build together with zero regressions

- [ ] 2.0 Copy execution service files into `paladin-battalion`
  - [ ] 2.1 Copy all 9 execution service files from `src/application/use_cases/battalion/` into `crates/paladin-battalion/src/` — **do NOT delete the originals yet** (they are removed in Task 5.0): `formation_service.rs`, `phalanx_service.rs`, `campaign_service.rs`, `chain_of_command_service.rs`, `conclave_execution_service.rs`, `council_service.rs`, `grove_service.rs`, `maneuver_service.rs`, `commander.rs`
  - [ ] 2.2 Enable the 9 corresponding `pub mod` declarations in `crates/paladin-battalion/src/lib.rs`
  - [ ] 2.3 Run the import migration `sed` pass on the copied execution service files (see Technical Considerations §6):
    - Replace `crate::application::ports::` → `paladin_ports::`
    - Replace `crate::application::use_cases::battalion::` → `crate::`
    - Replace `crate::core::` → `paladin_core::`
  - [ ] 2.4 Add `use paladin_ports;` and `use paladin_core;` extern crate references if needed (verify compiler error messages guide this)
  - [ ] 2.5 Run `cargo build -p paladin-battalion` — fix any remaining unresolved path errors introduced by the migration; document each fix with a comment noting the original path for traceability
  - [ ] 2.6 Run `cargo build --workspace` to confirm the root crate (still using `src/application/use_cases/battalion/`) compiles with zero regressions — this is expected at this stage

- [ ] 3.0 Copy utility/support files into `paladin-battalion`
  - [ ] 3.1 Copy the 3 utility files from `src/application/use_cases/battalion/` into `crates/paladin-battalion/src/`: `error_aggregation.rs`, `flow_visualizer.rs`, `retry.rs`
  - [ ] 3.2 Enable the 3 corresponding `pub mod` declarations in `crates/paladin-battalion/src/lib.rs`
  - [ ] 3.3 Run the import migration `sed` pass on the copied utility files (same substitutions as Task 2.3)
  - [ ] 3.4 Run `cargo build -p paladin-battalion` — fix any path errors; all 13 modules must compile cleanly in isolation
  - [ ] 3.5 Run `cargo test -p paladin-battalion` — confirm inline `#[cfg(test)]` unit tests in the extracted files pass (there are 12 test modules spread across the service files)
  - [ ] 3.6 Run `cargo build --workspace` to confirm zero regressions in the root crate

- [ ] 4.0 Verify dependency isolation of `paladin-battalion`
  - [ ] 4.1 Run `cargo tree -p paladin-battalion` and inspect the full dependency tree
  - [ ] 4.2 Confirm none of the following crates appear in the tree: `reqwest`, `hyper`, `actix-web`, `actix-http`, `sqlx`, `redis`, `qdrant-client`, `lettre`, `aws-sdk`, `minio`
  - [ ] 4.3 If any forbidden crate appears, identify the path in the dependency chain, resolve it (by removing the dep or replacing it with a pure-Rust alternative), and re-run `cargo tree -p paladin-battalion`
  - [ ] 4.4 Save the clean `cargo tree -p paladin-battalion` output to `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-dependency-tree.txt`
  - [ ] 4.5 Save the `cargo build -p paladin-battalion` stdout + stderr to `project/Milestone_5-Workspace-Decomposition/Epic_3/paladin-battalion-isolation-build.txt`

- [ ] 5.0 Wire `paladin-battalion` into the root `paladin` facade crate
  - [ ] 5.1 Add `paladin-battalion = { workspace = true }` to the `[dependencies]` section of the root `paladin` crate's `Cargo.toml`
  - [ ] 5.2 Add `pub use paladin_battalion;` to `src/lib.rs` so the crate is re-exported at the crate root
  - [ ] 5.3 Convert `src/application/use_cases/battalion/mod.rs` to a re-export shim: replace its contents with `pub use paladin_battalion::*;` sub-module re-exports so existing `use paladin::application::use_cases::battalion::formation_service::FormationExecutionService` paths continue to resolve
  - [ ] 5.4 Run `cargo build --workspace` — fix any path resolution errors; the CLI commands and any other external consumers of battalion paths must compile cleanly
  - [ ] 5.5 Remove the original source files from `src/application/use_cases/battalion/` (all except `mod.rs` which is now the re-export shim): `formation_service.rs`, `phalanx_service.rs`, `campaign_service.rs`, `chain_of_command_service.rs`, `conclave_execution_service.rs`, `council_service.rs`, `grove_service.rs`, `maneuver_service.rs`, `commander.rs`, `error_aggregation.rs`, `flow_visualizer.rs`, `retry.rs`
  - [ ] 5.6 Run `cargo build --workspace` and confirm zero compilation errors after deletion of originals
  - [ ] 5.7 Run `cargo test --workspace` and confirm the full test suite passes at or above the pre-epic baseline count recorded in Task 1.1

- [ ] 6.0 Verify existing examples and integration tests compile
  - [ ] 6.1 Spot-check at least four battalion-related examples: `cargo check --example formation_sequential`, `cargo check --example campaign_workflow`, `cargo check --example chain_of_command_delegation`, `cargo check --example commander_basic`
  - [ ] 6.2 Spot-check at least two examples using Commander auto-detection: `cargo check --example commander_auto`, `cargo check --example commander_full_config`
  - [ ] 6.3 Run `cargo test --test integration` to confirm all workspace-level integration tests pass (covers `tests/integration/battalion/`)
  - [ ] 6.4 Run `cargo test --test unit` to confirm all workspace-level unit tests pass (covers `tests/unit/battalion/`)
  - [ ] 6.5 Confirm the two top-level battalion integration test files compile: `cargo check --test battalion_campaign_integration_test`, `cargo check --test battalion_chain_of_command_integration_test`

- [ ] 7.0 Run full workspace quality gates
  - [ ] 7.1 Run `cargo fmt --all --check` — if it fails, run `cargo fmt --all` and re-check; commit the formatting diff separately
  - [ ] 7.2 Run `cargo clippy --workspace -- -D warnings` — fix all warnings before proceeding
  - [ ] 7.3 Run `cargo doc -p paladin-battalion --no-deps` — fix any broken intra-doc links or missing doc warnings
  - [ ] 7.4 Run `cargo test --workspace` one final time and confirm all tests pass; record final count
  - [ ] 7.5 Confirm `crates/paladin-battalion/Cargo.toml` `[dependencies]` contains no forbidden infrastructure crates (manual review)

- [ ] 8.0 Commit and open pull request
  - [ ] 8.1 Stage all changes: `git add .`
  - [ ] 8.2 Commit using conventional commit format:
    ```bash
    git commit \
      -m "feat(milestone-5/epic-3): extract paladin-battalion as third workspace crate" \
      -m "- Move all 13 files from src/application/use_cases/battalion/ to crates/paladin-battalion/src/" \
      -m "- Migrate ~169 internal import paths from crate::application::ports:: to paladin_ports::" \
      -m "- Migrate crate::core:: references to paladin_core::" \
      -m "- Convert src/application/use_cases/battalion/mod.rs to re-export shim" \
      -m "- Verify zero transitive infrastructure deps via cargo tree -p paladin-battalion" \
      -m "- All 2610+ workspace tests pass; clippy and fmt clean" \
      -m "Closes Epic 3 of Milestone 5 Workspace Decomposition"
    ```
  - [ ] 8.3 Push the branch: `git push origin feature/milestone_5-epic_3-battalion-extraction`
  - [ ] 8.4 Open a pull request targeting `feature/milestone_5` titled `feat(milestone-5/epic-3): extract paladin-battalion as third workspace crate`, linking this task list and the Epic 3 outline (`project/Milestone_5-Workspace-Decomposition/Epic_3/Milestone_5-Tier_2-Epic_3-Battalion_Extraction.md`) in the description
