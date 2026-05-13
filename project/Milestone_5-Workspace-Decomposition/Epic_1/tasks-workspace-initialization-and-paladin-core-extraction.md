## Relevant Files

- `Cargo.toml` - Workspace root manifest (converted from monolithic crate to workspace with `[workspace]` and `[workspace.dependencies]` sections).
- `crates/paladin-core/Cargo.toml` - New manifest for the `paladin-core` crate with minimal dependencies.
- `crates/paladin-core/src/lib.rs` - Library root for `paladin-core`; declares `base` and `platform` modules.
- `crates/paladin-core/src/base/` - Relocated from `src/core/base/`; contains `Node<T>`, `Collection`, `Field`, `Message`, `Action`, `Event`.
- `crates/paladin-core/src/platform/container/` - Relocated from `src/core/platform/container/`; contains `Paladin`, Battalion types, `Garrison`, `Arsenal`, `Citadel`, `Herald`, `Sanctum`.
- `crates/paladin-core/src/platform/container/battalion/mod.rs` - Must no longer import from `application::`; resolution per architectural decision artifact.
- `src/lib.rs` - Root `paladin` facade crate; re-exports `paladin-core` types under existing module paths for backward compatibility.
- `src/core/` - Fully removed or reduced to a thin shim of re-exports after extraction.
- `project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-options.md` - Options-analysis artifact evaluating approaches to resolve the `battalion/mod.rs` → `application::` dependency.
- `project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` - Recorded decision output of the implementer interview.

### Notes

- This is a structural refactor. No behavioral or public-API-shape changes are permitted. All existing tests must continue to pass.
- Follow Rust TDD discipline from the workspace instructions: `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` must pass before marking parent tasks complete.
- Use `cargo build -p paladin-core` and `cargo test -p paladin-core` to validate the extracted crate in isolation.
- Use `cargo tree -p paladin-core` to verify no transitive dependencies on LLM SDKs, DB drivers, HTTP frameworks, or object storage clients.
- The upward dependency resolution (FR-17) is gated behind a decision task — do not write implementation code for that sub-problem until the options analysis and interview are complete.
- Workspace dependency version unification uses `dep = { workspace = true }` in member manifests.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Confirm the current branch is `feature/milestone_5` (the milestone integration branch) via `git status` / `git branch --show-current`
  - [x] 0.2 From `feature/milestone_5`, create and checkout a new feature branch: `git checkout -b feature/milestone_5-epic_1-paladin-core-extraction`
  - [x] 0.3 Push the branch to origin to establish the upstream: `git push -u origin feature/milestone_5-epic_1-paladin-core-extraction`

- [x] 1.0 Initialize Cargo workspace root and shared dependency configuration
  - [x] 1.1 Capture the pre-epic baseline: run `cargo test --all-features` and record the passing test count in `project/Milestone_5-Workspace-Decomposition/Epic_1/baseline-test-count.txt`
  - [x] 1.2 Capture the pre-epic build baseline: run `cargo clean && cargo build --timings` and save the `target/cargo-timings/` HTML report path alongside the baseline
  - [x] 1.3 Edit the root `Cargo.toml`: add a `[workspace]` section with `members = ["crates/*"]` and retain the existing `paladin` package in `.` (member via `members = [".", "crates/*"]` or equivalent depending on facade-crate strategy)
  - [x] 1.4 Add a `[workspace.dependencies]` section declaring shared versions for: `serde` (features `["derive"]`), `uuid` (features `["v4", "serde"]`), `chrono` (features `["serde"]`), `thiserror`, `tokio` (features `["full"]`), `async-trait`, `serde_json`, `reqwest`, and `log` (FR-2)
  - [x] 1.5 Convert the root `paladin` crate's `[dependencies]` entries for the above packages to use `dep = { workspace = true }` syntax so versions are centralized
  - [x] 1.6 Run `cargo build` from the workspace root and confirm it succeeds without source moves (FR-3)
  - [x] 1.7 Run `cargo test --workspace` and confirm the baseline test count still passes

- [x] 2.0 Scaffold the `paladin-core` crate skeleton
  - [x] 2.1 Create the directory `crates/paladin-core/src/`
  - [x] 2.2 Create `crates/paladin-core/Cargo.toml` with `name = "paladin-core"`, `version` matching the workspace, `edition = "2021"`, and `license` consistent with the root crate (FR-5)
  - [x] 2.3 Add the minimal `[dependencies]` block to `crates/paladin-core/Cargo.toml` using workspace references: `serde`, `uuid`, `chrono`, `thiserror`, `async-trait`, `serde_json` — and only these (FR-6)
  - [x] 2.4 Create `crates/paladin-core/src/lib.rs` with crate-level doc comment, `#![deny(...)]` lints consistent with the root crate, and empty `pub mod base;` / `pub mod platform;` placeholders (commented out until Tasks 4.0 / 5.0)
  - [x] 2.5 Run `cargo build -p paladin-core` and confirm the empty crate compiles in isolation (FR-7)
  - [x] 2.6 Run `cargo build --workspace` and confirm both the empty `paladin-core` and the existing `paladin` crate still build together

- [x] 3.0 Resolve the `battalion/mod.rs` upward dependency on the application layer (decision + implementation)
  - [x] 3.1 Map the full upward coupling: run `grep -rn "application::" src/core/` and record every file and symbol (notably `PaladinResult` from `application::ports::output::paladin_port` and `RegistryError` from `application::ports::output::paladin_registry`)
  - [x] 3.2 Author the options-analysis artifact at `project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-options.md` covering at least the three Milestone-2 Option A/B/C paths, each with: definition, files touched, pros, cons, downstream/breaking-change impact, and test-migration impact
  - [x] 3.3 Conduct the implementer decision interview: walk through each option's trade-offs, answer outstanding questions, and select the chosen approach — **do not write implementation code before this step**
  - [x] 3.4 Record the decision at `project/Milestone_5-Workspace-Decomposition/Epic_1/decisions/battalion-result-upward-dependency-decision.md` including: chosen option, rationale, rejected-option reasoning, and a concrete implementation checklist derived from the decision
  - [x] 3.5 Update this task-list file by appending implementation sub-tasks (3.6a–3.6k) generated from the decision checklist in step 3.4 before executing them
  - [x] 3.6a Create `src/core/platform/container/execution_result.rs` — move `PaladinResult` struct and `StopReason` enum from `src/application/ports/output/paladin_port.rs`; update imports to use `crate::core::platform::container::` paths
  - [x] 3.6b Create `src/core/platform/container/token_usage.rs` — move `TokenUsage` struct from `src/application/ports/output/llm_port.rs`
  - [x] 3.6c Create `src/core/platform/container/registry_error.rs` — move `RegistryError` enum from `src/application/ports/output/paladin_registry.rs`
  - [x] 3.6d Move `HandoffError` to `src/core/platform/container/arsenal/handoff_error.rs` (copy content from `src/application/errors/handoff_error.rs`)
  - [x] 3.6e Register new modules: add `pub mod execution_result; pub mod token_usage; pub mod registry_error;` to `src/core/platform/container/mod.rs` and `pub mod handoff_error;` to `src/core/platform/container/arsenal/mod.rs`
  - [x] 3.6f Update `src/application/ports/output/paladin_port.rs` — remove `PaladinResult`/`StopReason` struct/enum bodies; add `pub use crate::core::platform::container::execution_result::{PaladinResult, StopReason};`
  - [x] 3.6g Update `src/application/ports/output/llm_port.rs` — remove `TokenUsage` struct body; add `pub use crate::core::platform::container::token_usage::TokenUsage;`
  - [x] 3.6h Update `src/application/ports/output/paladin_registry.rs` — remove `RegistryError` enum body; add `pub use crate::core::platform::container::registry_error::RegistryError;`
  - [x] 3.6i Replace `src/application/errors/handoff_error.rs` content with `pub use crate::core::platform::container::arsenal::handoff_error::HandoffError;`
  - [x] 3.6j Update core internal imports: `battalion/mod.rs`, `battalion/conclave.rs`, `herald.rs` (also remove `pub use PaladinError`), `arsenal/handoff_tool.rs` — replace all `application::` import paths with `crate::core::platform::container::` paths
  - [x] 3.7 Run `cargo build --workspace` and `cargo test --workspace` to confirm the refactor preserves behavior with zero regressions

- [ ] 4.0 Extract `src/core/base/` into `paladin-core`
  - [ ] 4.1 Verify `src/core/base/` is free of `application::` / `infrastructure::` imports (run `grep -rn "application::\|infrastructure::" src/core/base/`) — fix any stragglers before moving
  - [ ] 4.2 Move all files from `src/core/base/` to `crates/paladin-core/src/base/` preserving sub-module structure and `mod.rs` declarations (FR-8)
  - [ ] 4.3 Update every `use` statement inside the moved files to use crate-local paths (replace `crate::core::base::...` with `crate::base::...` and `crate::core::...` with the equivalent `paladin-core` path) (FR-10)
  - [ ] 4.4 Enable `pub mod base;` in `crates/paladin-core/src/lib.rs`
  - [ ] 4.5 Confirm no moved file contains `use` statements referencing `application::`, `infrastructure::`, or any path outside `paladin-core` (FR-11)
  - [ ] 4.6 Run `cargo build -p paladin-core` and confirm successful isolated build
  - [ ] 4.7 Run `cargo test -p paladin-core` and confirm all base unit tests (inside `#[cfg(test)]` modules) pass (FR-12)
  - [ ] 4.8 In the root `paladin` crate, add a temporary `pub use paladin_core::base;` re-export under `src/core/mod.rs` (or equivalent shim) so unmoved code that still imports `crate::core::base::...` continues to compile
  - [ ] 4.9 Run `cargo build --workspace` and `cargo test --workspace` to confirm end-to-end compilation and no test regressions

- [ ] 5.0 Extract `src/core/platform/container/` into `paladin-core`
  - [ ] 5.1 Verify `src/core/platform/container/` has zero `application::` / `infrastructure::` imports (Task 3.0 must be complete); re-run `grep -rn "application::\|infrastructure::" src/core/platform/`
  - [ ] 5.2 Move all files from `src/core/platform/container/` to `crates/paladin-core/src/platform/container/` preserving sub-module structure (including `battalion/`, `battalion/maneuver/` with its lexer/AST/parser, and all entity files) (FR-13)
  - [ ] 5.3 Create `crates/paladin-core/src/platform/mod.rs` declaring `pub mod container;`
  - [ ] 5.4 Update every `use` statement in the moved files to use crate-local paths; replace references to `crate::core::base::...` with `crate::base::...` (FR-15)
  - [ ] 5.5 Enable `pub mod platform;` in `crates/paladin-core/src/lib.rs`
  - [ ] 5.6 Confirm no moved file contains `use` statements referencing `application::` or `infrastructure::` (FR-16)
  - [ ] 5.7 Run `cargo build -p paladin-core` and confirm successful isolated build
  - [ ] 5.8 Run `cargo test -p paladin-core` and confirm all domain entity unit tests pass (FR-18)
  - [ ] 5.9 Update/extend the temporary re-export shim in the root `paladin` crate so callers using `crate::core::platform::container::...` still compile
  - [ ] 5.10 Run `cargo build --workspace` and `cargo test --workspace`; fix any remaining path references in the root crate

- [ ] 6.0 Wire the root `paladin` facade crate to re-export `paladin-core` types
  - [ ] 6.1 Add `paladin-core = { path = "crates/paladin-core" }` to the root `paladin` crate's `[dependencies]` (FR-19)
  - [ ] 6.2 In `src/lib.rs`, replace the temporary shim with the final re-export strategy so that `paladin::core::base::...` and `paladin::core::platform::container::...` resolve to `paladin_core` types (FR-20)
  - [ ] 6.3 Remove the now-empty `src/core/base/` and `src/core/platform/container/` directories (FR-21) — retain only any thin re-export shim that may still be needed
  - [ ] 6.4 Sweep the workspace for any lingering references that still import from the old `src/core/...` paths and update them (application + infrastructure layers, examples, benches, integration tests)
  - [ ] 6.5 Run `cargo build --workspace` and confirm clean build (FR-22)
  - [ ] 6.6 Run `cargo test --workspace` and confirm the test count matches or exceeds the baseline captured in 1.1 (FR-23)
  - [ ] 6.7 Spot-check at least three existing examples under `examples/` (e.g., `basic_paladin.rs`, `formation_sequential.rs`, `garrison_in_memory.rs`) with `cargo check --example <name>` to confirm no public import path broke

- [ ] 7.0 Validate dependency layering and documentation
  - [ ] 7.1 Run `cargo tree -p paladin-core` and save the output to `project/Milestone_5-Workspace-Decomposition/Epic_1/paladin-core-dependency-tree.txt` (FR-24)
  - [ ] 7.2 Inspect the dependency tree and confirm no entries for: `openai`, `anthropic`, `deepseek`, `sqlx`, `redis`, `mysql`, `axum`, `actix`, `minio`, `s3`, `reqwest`, `tokio` (FR-25)
  - [ ] 7.3 Run `cargo doc -p paladin-core --no-deps` and confirm zero broken intra-doc links in the output (FR-26)
  - [ ] 7.4 Run `cargo doc --workspace --no-deps` and confirm the facade crate produces clean documentation
  - [ ] 7.5 Confirm `paladin-core/Cargo.toml` `[dependencies]` section still contains only the FR-6 approved set (serde, uuid, chrono, thiserror, async-trait, serde_json)

- [ ] 8.0 Run full workspace test and quality gates, then commit and open PR
  - [ ] 8.1 Run `cargo fmt --all --check` — if it fails, run `cargo fmt --all` and re-check
  - [ ] 8.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and resolve every warning
  - [ ] 8.3 Run `cargo test --workspace --all-features` and confirm the passing test count equals or exceeds the baseline in 1.1
  - [ ] 8.4 Run `make clean-code` (format + lint + check) and `make audit` to confirm no new security advisories
  - [ ] 8.5 Remove any temporary debug output (`dbg!`, `println!`, stray shim comments) introduced during extraction
  - [ ] 8.6 Stage changes: `git add .`
  - [ ] 8.7 Commit with a conventional-commit message using multiple `-m` flags, e.g. `git commit -m "refactor: extract paladin-core as first workspace crate" -m "- Initialize Cargo workspace with crates/paladin-core member" -m "- Move src/core/base and src/core/platform/container into paladin-core" -m "- Resolve battalion upward dependency per Epic_1 decision artifact" -m "- Root paladin crate re-exports paladin-core for backward compatibility" -m "Implements Milestone 5 Epic 1"`
  - [ ] 8.8 Push the branch: `git push`
  - [ ] 8.9 Open a pull request targeting `feature/milestone_5` titled `refactor(milestone-5/epic-1): extract paladin-core as first workspace crate`, linking this task list and the PRD in the description
