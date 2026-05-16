## Relevant Files

### New Files (to be created)
- `crates/paladin-ports/Cargo.toml` — Manifest for the new `paladin-ports` crate; declares workspace dependencies.
- `crates/paladin-ports/src/lib.rs` — Crate root; declares `pub mod input;` and `pub mod output;`.
- `crates/paladin-ports/src/output/mod.rs` — Declares all 19 output port modules (all unconditional).
- `crates/paladin-ports/src/output/llm_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/garrison_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/sanctum_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/embedding_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/arsenal_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/citadel_port.rs` — Moved from `src/application/ports/output/`; imports `CitadelError` via `paladin_core::platform::container::citadel_error::CitadelError`.
- `crates/paladin-ports/src/output/queue_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/notification_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/file_storage_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/paladin_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/paladin_executor_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/paladin_registry.rs` — Moved; `RegistryError` re-export updated to `paladin_core::`.
- `crates/paladin-ports/src/output/battalion_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/log_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/scheduler_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/search_engine_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/content_delivery_port.rs` — Moved from `src/application/ports/output/`.
- `crates/paladin-ports/src/output/vision_llm_port.rs` — Moved from `src/application/ports/output/`; `#[cfg(feature)]` guard removed.
- `crates/paladin-ports/src/output/vision_port.rs` — Moved from `src/application/ports/output/`; `#[cfg(feature)]` guard removed.
- `crates/paladin-ports/src/input/mod.rs` — Declares all 6 input port modules.
- `crates/paladin-ports/src/input/content_input_port.rs` — Moved from `src/application/ports/input/`.
- `crates/paladin-ports/src/input/document_port.rs` — Moved from `src/application/ports/input/`.
- `crates/paladin-ports/src/input/listener_port.rs` — Moved from `src/application/ports/input/`.
- `crates/paladin-ports/src/input/ml_port.rs` — Moved from `src/application/ports/input/`.
- `crates/paladin-ports/src/input/nlp_port.rs` — Moved from `src/application/ports/input/`.
- `crates/paladin-ports/src/input/rpc_port.rs` — Moved from `src/application/ports/input/`.
- `project/Milestone_5-Workspace-Decomposition/Epic_2/baseline-test-count.txt` — Pre-epic test baseline.
- `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-isolation-build.txt` — Output of `cargo build -p paladin-ports` (FR-25).
- `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-dependency-tree.txt` — Output of `cargo tree -p paladin-ports` (FR-26).

### Modified Files
- `Cargo.toml` (root) — Add `paladin-ports = { path = "crates/paladin-ports" }` dependency.
- `src/lib.rs` — Update all `pub use application::ports::*` re-exports to `pub use paladin_ports::*`.
- `src/application/mod.rs` — Remove `pub mod ports;` declaration and update doc comments referencing `application::ports::`.
- `src/application/errors/citadel_error.rs` — Becomes a re-export shim: `pub use paladin_core::platform::container::citadel_error::CitadelError;`.
- `crates/paladin-core/src/platform/container/citadel_error.rs` — **New file.** `CitadelError` enum and `impl` blocks moved here from `src/application/errors/citadel_error.rs`, consistent with `garrison_error.rs`, `registry_error.rs`, etc.
- `crates/paladin-core/src/platform/container/mod.rs` — Add `pub mod citadel_error;` declaration.
- `src/infrastructure/**/*.rs` — Import paths updated from `crate::application::ports::` to `paladin_ports::` (bulk migration).
- `src/application/use_cases/**/*.rs` — Import paths updated (bulk migration).
- `docs/ARSENAL.md`, `docs/BATTALION.md`, `docs/CONTRIBUTING_PROVIDERS.md`, `docs/GARRISON.md`, `docs/PROVIDER_EXPANSION.md`, `docs/SANCTUM.md`, `docs/SANCTUM_MIGRATION.md`, `docs/SENTINEL.md`, `docs/architecture/dependency-flow-diagrams.md`, `docs/architecture/hexagonal-design.md`, `docs/contributing/adapter-development.md`, `docs/port-trait-doc-template.md`, `docs/Design/minio-file-repository-setup.md`, `docs/Design/redis-queue-adapter-setup.md` — Update `application::ports::` import path examples.

### Deleted Files
- `src/application/ports/` — Entire directory removed after all ports are extracted and all import sites are migrated (FR-16).

### Notes

- This is a structural refactor. No behavioral or public-API-shape changes are permitted. All existing tests must continue to pass.
- Follow Rust TDD discipline: `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` must pass before marking any parent task complete.
- Use `cargo build -p paladin-ports` and `cargo test -p paladin-ports` to validate the extracted crate in isolation.
- Use `cargo tree -p paladin-ports` to verify no transitive dependencies on LLM SDKs, DB drivers, or storage clients.
- **`CitadelError` relocation:** `CitadelError` moves to `crates/paladin-core/src/platform/container/citadel_error.rs` — the same layer as `garrison_error.rs`, `herald_error.rs`, `paladin_error.rs`, and `registry_error.rs`. `paladin-core` already carries all required dependencies (`serde_json`, `uuid`, `thiserror`). After the move: `citadel_port.rs` in `paladin-ports` imports via `use paladin_core::platform::container::citadel_error::CitadelError;`, and `src/application/errors/citadel_error.rs` becomes a one-line re-export shim: `pub use paladin_core::platform::container::citadel_error::CitadelError;`.
- **Do NOT delete `src/application/ports/` files during Tasks 2.0 or 3.0.** The originals remain in place until all import sites have been migrated in Task 5.0.
- The mass import migration (Task 5.0) covers ~314 occurrences in 76 `.rs` files — a scripted `sed` pass is recommended, followed by `cargo build --workspace` to catch any misses.
- Workspace dependency versions use `dep = { workspace = true }` syntax in member manifests.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Confirm the current branch is `feature/milestone_5` (the milestone integration branch) via `git status` / `git branch --show-current`
  - [x] 0.2 From `feature/milestone_5`, create and checkout a new feature branch: `git checkout -b feature/milestone_5-epic_2-paladin-ports-extraction`
  - [x] 0.3 Push the branch to origin to establish the upstream: `git push -u origin feature/milestone_5-epic_2-paladin-ports-extraction`

- [x] 1.0 Capture baseline and scaffold `paladin-ports` crate skeleton
  - [x] 1.1 Capture the pre-epic baseline: run `cargo test --workspace --all-features` and record the passing test count in `project/Milestone_5-Workspace-Decomposition/Epic_2/baseline-test-count.txt`
  - [x] 1.2 Create directory structure: `mkdir -p crates/paladin-ports/src/output crates/paladin-ports/src/input`
  - [x] 1.3 Create `crates/paladin-ports/Cargo.toml` with `name = "paladin-ports"`, `edition = "2021"`, `version` matching the workspace root, and a `[dependencies]` block using workspace references: `paladin-core = { path = "../paladin-core" }`, plus `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, and `tokio` — all via `{ workspace = true }` — and **only** these (FR-1, FR-2, FR-3)
  - [x] 1.4 Create `crates/paladin-ports/src/lib.rs` with a crate-level `//!` doc comment describing the crate's role as the port contract layer, `#![deny(...)]` lints consistent with the root crate, and **commented-out** `// pub mod input;` and `// pub mod output;` placeholders (to be uncommented in Tasks 2.0 and 3.0) (FR-5)
  - [x] 1.5 Create stub files `crates/paladin-ports/src/output/mod.rs` and `crates/paladin-ports/src/input/mod.rs` — empty for now
  - [x] 1.6 Run `cargo build -p paladin-ports` and confirm the empty crate compiles in isolation (FR-4)
  - [x] 1.7 Run `cargo build --workspace` and confirm both the empty `paladin-ports` and the existing `paladin` crate still build together

- [x] 2.0 Extract all output port traits into `paladin-ports`
  - [x] 2.1 Copy all 19 output port files from `src/application/ports/output/` into `crates/paladin-ports/src/output/` — **do NOT delete the originals yet** (they are removed in Task 5.0): `llm_port.rs`, `garrison_port.rs`, `sanctum_port.rs`, `embedding_port.rs`, `arsenal_port.rs`, `citadel_port.rs`, `queue_port.rs`, `notification_port.rs`, `file_storage_port.rs`, `paladin_port.rs`, `paladin_executor_port.rs`, `paladin_registry.rs`, `battalion_port.rs`, `log_port.rs`, `scheduler_port.rs`, `search_engine_port.rs`, `content_delivery_port.rs`, `vision_llm_port.rs`, `vision_port.rs`
  - [x] 2.2 In each copied output port file, replace all `use crate::core::platform::container::` import paths with `use paladin_core::platform::container::` (FR-9)
  - [x] 2.3 In each copied output port file, replace all cross-port imports of the form `use crate::application::ports::output::<module>::` with `use crate::output::<module>::` so they reference sibling modules within `paladin-ports` (FR-9)
  - [x] 2.4 **`CitadelError` relocation to `paladin-core`:** Create `crates/paladin-core/src/platform/container/citadel_error.rs` by copying the `CitadelError` enum, `impl` blocks, and tests verbatim from `src/application/errors/citadel_error.rs`. Update the doc example in the new file from `paladin::application::errors::citadel_error::CitadelError` to `paladin_core::platform::container::citadel_error::CitadelError`. Then add `pub mod citadel_error;` to `crates/paladin-core/src/platform/container/mod.rs` (alongside the existing `garrison_error`, `registry_error`, etc.). Run `cargo build -p paladin-core` to confirm the new module compiles in isolation (consistent with FR-9).
  - [x] 2.5 In `crates/paladin-ports/src/output/paladin_registry.rs`, update the `pub use` re-export of `RegistryError` from `crate::core::platform::container::registry_error::RegistryError` to `paladin_core::platform::container::registry_error::RegistryError` (FR-11)
  - [x] 2.6 In `crates/paladin-ports/src/output/vision_llm_port.rs` and `vision_port.rs`, remove any `#[cfg(feature = "vision")]` attribute that was copied over — these modules are exported unconditionally from `paladin-ports` (FR-6)
  - [x] 2.7 Update `crates/paladin-ports/src/output/mod.rs` to declare all 19 modules with `pub mod <module_name>;` — all unconditional, no `#[cfg(feature)]` guards (FR-6)
  - [x] 2.8 Uncomment `pub mod output;` in `crates/paladin-ports/src/lib.rs`
  - [x] 2.9 Run `cargo build -p paladin-ports` and fix all compilation errors in the copied output port files until the crate builds cleanly in isolation
  - [x] 2.10 Run `cargo build --workspace` to confirm zero regressions in the root crate (which still uses `src/application/ports/` — this is expected)

- [ ] 3.0 Extract all input port traits into `paladin-ports`
  - [ ] 3.1 Copy all 6 input port files from `src/application/ports/input/` into `crates/paladin-ports/src/input/` — **do NOT delete the originals yet**: `content_input_port.rs`, `document_port.rs`, `listener_port.rs`, `ml_port.rs`, `nlp_port.rs`, `rpc_port.rs`
  - [ ] 3.2 In each copied input port file, replace all `use crate::core::platform::container::` import paths with `use paladin_core::platform::container::` (FR-13, FR-14)
  - [ ] 3.3 In each copied input port file, remove or update any remaining `crate::application::`, `crate::infrastructure::`, or `crate::core::` references — none should remain after the substitution (FR-13)
  - [ ] 3.4 Update `crates/paladin-ports/src/input/mod.rs` to declare all 6 modules with `pub mod <module_name>;`
  - [ ] 3.5 Uncomment `pub mod input;` in `crates/paladin-ports/src/lib.rs`
  - [ ] 3.6 Run `cargo build -p paladin-ports` and fix any remaining compilation errors until the crate builds cleanly (FR-4)
  - [ ] 3.7 Run `cargo test -p paladin-ports` to confirm all unit tests from the moved port files pass (FR-27)
  - [ ] 3.8 Run `cargo build --workspace` to confirm zero regressions

- [ ] 4.0 Wire `paladin-ports` into the root `paladin` crate
  - [ ] 4.1 Add `paladin-ports = { path = "crates/paladin-ports" }` to the root `Cargo.toml` `[dependencies]` section (FR-15)
  - [ ] 4.2 Run `cargo build --workspace` to confirm the new dependency resolves and the workspace still compiles cleanly before any import changes are made

- [ ] 5.0 Migrate all import sites and delete `src/application/ports/`
  - [ ] 5.1 Confirm migration scope: run `grep -rn "application::ports::" src/ --include="*.rs" | wc -l` and record the count (expected ~314)
  - [ ] 5.2 Run a scripted bulk substitution for output port imports across all non-ports source files: `find src -name "*.rs" ! -path "*/application/ports/*" | xargs sed -i 's|crate::application::ports::output::|paladin_ports::output::|g'`
  - [ ] 5.3 Run a scripted bulk substitution for input port imports: `find src -name "*.rs" ! -path "*/application/ports/*" | xargs sed -i 's|crate::application::ports::input::|paladin_ports::input::|g'`
  - [ ] 5.4 Update `src/lib.rs` output port re-exports: replace each `pub use application::ports::output::<module>::{...}` with `pub use paladin_ports::output::<module>::{...}` for all output ports (FR-17)
  - [ ] 5.5 Update `src/lib.rs` input port re-exports: replace each `pub use application::ports::input::<module>::{...}` with `pub use paladin_ports::input::<module>::{...}` (FR-17)
  - [ ] 5.6 Update the `#[cfg(feature = "vision")]` conditional re-exports in `src/lib.rs` to reference `paladin_ports::output::vision_llm_port` and `paladin_ports::output::vision_port` (FR-6)
  - [ ] 5.7 Replace the body of `src/application/errors/citadel_error.rs` with a single re-export line: `pub use paladin_core::platform::container::citadel_error::CitadelError;` so that all existing callers of `application::errors::citadel_error::CitadelError` continue to compile without changes
  - [ ] 5.8 Delete the entire `src/application/ports/` directory: `rm -rf src/application/ports/` (FR-16)
  - [ ] 5.9 Remove `pub mod ports;` from `src/application/mod.rs` and update the module-level doc comment in that file to remove the `ports` section description (FR-18)
  - [ ] 5.10 Run `cargo build --workspace` to surface any remaining import errors missed by the bulk substitution; manually fix every reported error
  - [ ] 5.11 Verify migration completeness: run `grep -rn "application::ports::" src/ --include="*.rs"` and confirm zero remaining occurrences (FR-19 complete)
  - [ ] 5.12 Run `cargo test --workspace` to confirm zero regressions — the passing test count must match the baseline recorded in Task 1.1 (FR-28)
  - [ ] 5.13 Identify and update all `application::ports::` references in docs Markdown files: run `grep -rn "application::ports::" docs/` to locate them across the 14 affected files, then update each reference to reflect the new `paladin_ports::` import paths (FR-22)

- [ ] 6.0 Validate dependency layering and save artifacts
  - [ ] 6.1 Run `cargo build -p paladin-ports` in isolation and confirm it succeeds; save the full terminal output to `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-isolation-build.txt` (FR-25)
  - [ ] 6.2 Run `cargo tree -p paladin-ports` and save the full output to `project/Milestone_5-Workspace-Decomposition/Epic_2/paladin-ports-dependency-tree.txt` (FR-26)
  - [ ] 6.3 Inspect `paladin-ports-dependency-tree.txt`: confirm `paladin-core` appears as the only workspace-internal direct dependency, and that there are zero entries for `redis`, `sqlx`, `aws-sdk-s3`, `reqwest` (storage client context), `openai`, `anthropic`, or any LLM provider SDK (FR-23, FR-24)
  - [ ] 6.4 Run `cargo doc -p paladin-ports --no-deps` and confirm zero broken intra-doc link errors (FR-30)
  - [ ] 6.5 Spot-check three existing examples with `cargo check --example basic_paladin`, `cargo check --example formation_sequential`, and `cargo check --example garrison_in_memory` to confirm no public import path broke

- [ ] 7.0 Run full quality gates, commit, and open PR
  - [ ] 7.1 Run `cargo fmt --all` to format all code, then `cargo fmt --all --check` to confirm zero formatting issues
  - [ ] 7.2 Run `cargo clippy --workspace -- -D warnings` and fix all warnings
  - [ ] 7.3 Run `cargo test --workspace` one final time and confirm the passing count matches the Task 1.1 baseline (FR-28)
  - [ ] 7.4 Remove any temporary debug prints or scratch files; confirm `git status` shows only intentional changes
  - [ ] 7.5 Stage all changes: `git add .`
  - [ ] 7.6 Commit using conventional commit format: `git commit -m "refactor(milestone-5/epic-2): extract paladin-ports as dedicated workspace crate" -m "- Scaffold crates/paladin-ports with Cargo.toml and src/lib.rs" -m "- Move all 19 output port traits and 6 input port traits to paladin-ports" -m "- Move CitadelError to paladin-core/platform/container/citadel_error.rs (consistent with garrison_error, registry_error)" -m "- Shim src/application/errors/citadel_error.rs to re-export from paladin_core::" -m "- Migrate ~314 import-path occurrences across 76 .rs files to paladin_ports::" -m "- Delete src/application/ports/ entirely (FR-16)" -m "- Update src/lib.rs re-exports and 14 docs/ Markdown files" -m "- Verified: cargo build -p paladin-ports isolates cleanly; no LLM/DB/storage transitive deps" -m "Resolves Epic 2 of Milestone 5 PRD"`
  - [ ] 7.7 Push the branch: `git push`
  - [ ] 7.8 Open a pull request targeting `feature/milestone_5` titled `refactor(milestone-5/epic-2): extract paladin-ports as dedicated workspace crate`, linking this task list and `project/Milestone_5-Workspace-Decomposition/Epic_2/prd-paladin-ports-extraction.md` in the description
