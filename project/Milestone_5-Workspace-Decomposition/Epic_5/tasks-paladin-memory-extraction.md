## Relevant Files

- `Cargo.toml` — Workspace root manifest; must add `paladin-memory` to `members` and hoist `sqlx` / `qdrant-client` into `[workspace.dependencies]`.
- `crates/paladin-memory/Cargo.toml` — New crate manifest; defines `sqlite`, `qdrant`, and `content-processing` feature flags; `default = []`.
- `crates/paladin-memory/src/lib.rs` — Crate root; `#![deny(unsafe_code)]`, module declarations, top-level re-exports.
- `crates/paladin-memory/src/prelude.rs` — Curated re-exports of the most commonly used types.
- `crates/paladin-memory/src/garrison/mod.rs` — Feature-gated re-exports for all garrison types.
- `crates/paladin-memory/src/garrison/in_memory_garrison.rs` — `InMemoryGarrison` (always compiled); relocated from `src/infrastructure/adapters/garrison/in_memory_garrison.rs`.
- `crates/paladin-memory/src/garrison/sqlite_garrison.rs` — `SqliteGarrison` (gated: `sqlite`); relocated from `src/infrastructure/adapters/garrison/sqlite_garrison.rs`.
- `crates/paladin-memory/src/garrison/token_counter.rs` — `TokenCounter`, `TiktokenCounter`, `TokenCounterFactory` (gated: `content-processing`); relocated from `src/infrastructure/adapters/garrison/token_counter.rs`.
- `crates/paladin-memory/src/sanctum/mod.rs` — Feature-gated re-exports for all sanctum types.
- `crates/paladin-memory/src/sanctum/in_memory_adapter.rs` — `InMemorySanctum` + `InMemorySanctumConfig` (always compiled); relocated from `src/infrastructure/adapters/sanctum/in_memory_adapter.rs`.
- `crates/paladin-memory/src/sanctum/qdrant_adapter.rs` — `QdrantSanctumAdapter` (gated: `qdrant`); relocated from `src/infrastructure/adapters/sanctum/qdrant_adapter.rs`.
- `crates/paladin-memory/src/services/mod.rs` — Re-exports for all memory service types.
- `crates/paladin-memory/src/services/memory_extraction_service.rs` — `MemoryExtractionService`, `ExtractedMemory`, `MemoryExtractionStrategy`; relocated from `src/application/use_cases/sanctum/memory_extraction_service.rs`.
- `crates/paladin-memory/src/services/rag_retrieval_service.rs` — `RagRetrievalService`, `RagConfig`, `RetrievalTrigger`, `retrieve_context_with_timeout`; relocated from `src/application/use_cases/sanctum/rag_retrieval_service.rs`.
- `src/infrastructure/adapters/garrison/in_memory_garrison.rs` — **Deleted** after extraction.
- `src/infrastructure/adapters/garrison/sqlite_garrison.rs` — **Deleted** after extraction.
- `src/infrastructure/adapters/garrison/token_counter.rs` — **Deleted** after extraction.
- `src/infrastructure/adapters/garrison/mod.rs` — **Deleted** after extraction; replaced by facade re-exports.
- `src/infrastructure/adapters/sanctum/in_memory_adapter.rs` — **Deleted** after extraction.
- `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` — **Deleted** after extraction.
- `src/infrastructure/adapters/sanctum/mod.rs` — **Deleted** after extraction; replaced by facade re-exports.
- `src/application/use_cases/sanctum/memory_extraction_service.rs` — **Deleted** after extraction.
- `src/application/use_cases/sanctum/rag_retrieval_service.rs` — **Deleted** after extraction.
- `src/application/use_cases/sanctum/mod.rs` — **Deleted** after extraction; replaced by facade re-exports.
- `src/infrastructure/adapters/mod.rs` — Remove `pub mod garrison;` and `pub mod sanctum;` declarations after deletion.
- `src/application/use_cases/mod.rs` — Remove `pub mod sanctum;` declaration after deletion.
- `src/lib.rs` — Root `paladin` facade; add `paladin-memory` dependency; add `pub use paladin_memory::...` re-exports for all previously public garrison/sanctum/services paths.
- `tests/unit/sanctum/memory_extraction_service_test.rs` — **Migrated** into inline `#[cfg(test)]` module in `crates/paladin-memory/src/services/memory_extraction_service.rs`.
- `tests/unit/sanctum/rag_retrieval_service_test.rs` — **Migrated** into inline `#[cfg(test)]` module in `crates/paladin-memory/src/services/rag_retrieval_service.rs`.
- `tests/unit/sanctum/qdrant_sanctum_test.rs` — **Migrated** into inline `#[cfg(test)]` module in `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`.
- `tests/unit/sanctum/mod.rs` — Removed after test migration.
- `tests/integration/in_memory_sanctum_tests.rs` — Stays at workspace level; update import paths to use `paladin_memory::` where needed.
- `tests/integration/sqlite_garrison_integration_test.rs` — Stays at workspace level; update import paths.
- `tests/integration/qdrant_sanctum_tests.rs` — Stays at workspace level; update import paths.
- `tests/integration/rag_integration_tests.rs` — Stays at workspace level; update import paths.
- `tests/integration/paladin_garrison_integration_test.rs` — Stays at workspace level; update import paths.

### Notes

- This is a **structural refactor with zero behavioral change**. No public API shape, no storage logic, no query semantics may be altered during extraction.
- Follow the workspace Rust TDD discipline: `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` must all pass before marking any parent task complete.
- Build and test the extracted crate in isolation after each major extraction step: `cargo build -p paladin-memory --features <flag>` and `cargo test -p paladin-memory --features <flag>`.
- Verify dependency isolation: after scaffolding, run `cargo tree -p paladin-memory --no-default-features` and confirm no `sqlx`, `qdrant-client`, or `tiktoken-rs` nodes appear.
- The `paladin-llm` crate (Epic 4) is the canonical reference for crate structure, `Cargo.toml` layout, `lib.rs` gating conventions, and rustdoc style — follow it for consistency.
- **Task 1.0 is a prerequisite for all other tasks.** `sqlx` and `qdrant-client` must be hoisted to `[workspace.dependencies]` and the `tiktoken-rs` version must be confirmed before any `Cargo.toml` is written.
- All decisions from PRD Section 9 have been resolved and are reflected in the relevant FRs — no open questions remain.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Run `git branch --show-current` to confirm the current branch; the expected base is `feature/milestone_5-epic_4-paladin-llm-extraction` or the equivalent milestone integration branch with Epics 1–4 merged in.
  - [x] 0.2 Create and checkout the Epic 5 branch: `git checkout -b feature/milestone_5-epic_5-paladin-memory-extraction`
  - [x] 0.3 Push the branch to origin: `git push -u origin feature/milestone_5-epic_5-paladin-memory-extraction`

<!-- BASELINE (captured Task 1.1): cargo test --workspace → 2525 passed, 0 failed, 0 errors (tiktoken-rs v0.6.0 confirmed in step 1.2) -->

- [x] 1.0 Prerequisite: Hoist shared dependencies and confirm versions
  - [x] 1.1 Capture the pre-epic baseline: run `cargo test --workspace --all-features 2>&1 | tail -10` and record the passing/failing test count in a comment at the top of this file for later comparison.
  - [x] 1.2 Confirm the `tiktoken-rs` version: run `cargo tree -p paladin --features content-processing 2>/dev/null | grep tiktoken` and note the version string — this is the value to use in `paladin-memory/Cargo.toml`.
  - [x] 1.3 Open the root `Cargo.toml` and move `sqlx` from `[dependencies]` into `[workspace.dependencies]`. The workspace entry must include only storage-neutral features: `{ version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "json"] }`. Leave the root `[dependencies]` entry as `sqlx = { workspace = true, features = ["mysql"] }` so the `mysql` feature is added only for the root crate.
  - [x] 1.4 Move `qdrant-client` from root `[dependencies]` into `[workspace.dependencies]` as `qdrant-client = { version = "1.14" }`. Update the root `[dependencies]` entry to `qdrant-client = { workspace = true, optional = true }`.
  - [x] 1.5 Run `cargo build --workspace` and confirm no regressions — the hoisting must not change any compiled output.
  - [x] 1.6 Run `cargo test --workspace 2>&1 | tail -10` and confirm the test count matches the baseline recorded in step 1.1.

- [x] 2.0 Scaffold `paladin-memory` crate structure
  - [x] 2.1 Create the directory tree: `mkdir -p crates/paladin-memory/src/garrison crates/paladin-memory/src/sanctum crates/paladin-memory/src/services`
  - [x] 2.2 Create `crates/paladin-memory/Cargo.toml` with: `name = "paladin-memory"`, `edition = "2024"`, `license = "MIT"`, workspace dependency references for `paladin-core`, `paladin-ports`, `async-trait`, `serde`, `serde_json`, `uuid`, `chrono`, `thiserror`, `tokio`, `futures`, and `log`; define `[features]` with `sqlite = ["dep:sqlx"]`, `qdrant = ["dep:qdrant-client"]`, `content-processing = ["dep:tiktoken-rs"]`, and `default = []`; declare `sqlx`, `qdrant-client`, and `tiktoken-rs` as optional dependencies (use workspace = true for the first two; use the version confirmed in step 1.2 for tiktoken-rs); set `[lib] doctest = false` (FR-1.2–FR-1.6).
  - [x] 2.3 Create `crates/paladin-memory/src/lib.rs` with: a crate-level `//!` doc comment describing the crate's purpose, `#![deny(unsafe_code)]`, and stubbed (empty) module declarations: `pub mod garrison;`, `pub mod sanctum;`, `pub mod services;`, `pub mod prelude;` (FR-2.1).
  - [x] 2.4 Create `crates/paladin-memory/src/garrison/mod.rs` with a module-level `//!` doc comment and empty placeholder content — re-exports will be added in Task 3.
  - [x] 2.5 Create `crates/paladin-memory/src/sanctum/mod.rs` with a module-level `//!` doc comment and empty placeholder content — re-exports will be added in Task 4.
  - [x] 2.6 Create `crates/paladin-memory/src/services/mod.rs` with a module-level `//!` doc comment and empty placeholder content — re-exports will be added in Task 5.
  - [x] 2.7 Create `crates/paladin-memory/src/prelude.rs` as a stub with a `//!` doc comment — content will be filled in Task 5.
  - [x] 2.8 Add `"crates/paladin-memory"` to the `members` list in the workspace root `Cargo.toml` (FR-1.7).
  - [x] 2.9 Add `paladin-memory = { path = "crates/paladin-memory" }` to `[workspace.dependencies]` in the root `Cargo.toml` (FR-1.7).
  - [x] 2.10 Run `cargo build -p paladin-memory --no-default-features` and confirm the empty crate compiles cleanly.
  - [x] 2.11 Run `cargo tree -p paladin-memory --no-default-features` and confirm `sqlx`, `qdrant-client`, and `tiktoken-rs` are absent from the output.
  - [x] 2.12 Run `cargo build --workspace` and confirm no regressions in existing workspace members.

- [x] 3.0 Extract garrison adapters
  - [x] 3.1 Copy `src/infrastructure/adapters/garrison/in_memory_garrison.rs` to `crates/paladin-memory/src/garrison/in_memory_garrison.rs` (do not delete the original yet — deletion happens in Task 7).
  - [x] 3.2 Update all `use crate::core::platform::container::garrison::*` imports in the copied file to `use paladin_core::platform::container::garrison::*`. Verify no other `crate::` references remain.
  - [x] 3.3 Declare `pub mod in_memory_garrison;` in `crates/paladin-memory/src/garrison/mod.rs` and add `pub use in_memory_garrison::InMemoryGarrison;` (FR-2.2, FR-3.1).
  - [x] 3.4 Run `cargo build -p paladin-memory --no-default-features` and confirm `InMemoryGarrison` compiles.
  - [x] 3.5 Copy `src/infrastructure/adapters/garrison/sqlite_garrison.rs` to `crates/paladin-memory/src/garrison/sqlite_garrison.rs`.
  - [x] 3.6 Update all `use crate::core::...` imports to `use paladin_core::...` in the copied file. Verify `sqlx` imports do not use any `crate::` path.
  - [x] 3.7 Remove the `#[doc(hidden)]` attribute from the `SqliteGarrison` struct. Replace it with a proper rustdoc `///` comment explaining its purpose (persistent SQLite-backed conversation history), construction via `SqliteGarrison::connect()`, and when to prefer it over `InMemoryGarrison` (FR-3.2).
  - [x] 3.8 Wrap the module declaration in `garrison/mod.rs` with `#[cfg(feature = "sqlite")]`: add `#[cfg(feature = "sqlite")] pub mod sqlite_garrison;` and `#[cfg(feature = "sqlite")] pub use sqlite_garrison::SqliteGarrison;` (FR-2.2, FR-3.2).
  - [x] 3.9 Run `cargo build -p paladin-memory --features sqlite` and confirm `SqliteGarrison` compiles. Run `cargo build -p paladin-memory --no-default-features` and confirm it still compiles without `sqlx`.
  - [x] 3.10 Copy `src/infrastructure/adapters/garrison/token_counter.rs` to `crates/paladin-memory/src/garrison/token_counter.rs`.
  - [x] 3.11 Update any `use crate::...` imports to absolute crate paths. Confirm `tiktoken-rs` is only referenced via the `tiktoken_rs` crate name, not a `crate::` path.
  - [x] 3.12 Remove the `#[doc(hidden)]` attribute from `TiktokenCounter`. Add proper `///` rustdoc comments to: `TiktokenCounter` struct (purpose, supported models, caching behaviour), `TokenCounterFactory` struct (factory pattern description), and all public methods on both types (FR-3.3).
  - [x] 3.13 Add to `garrison/mod.rs`: `#[cfg(feature = "content-processing")] pub mod token_counter;` and `#[cfg(feature = "content-processing")] pub use token_counter::{TiktokenCounter, TokenCounter, TokenCounterFactory};` (FR-2.2, FR-3.3).
  - [x] 3.14 Run `cargo build -p paladin-memory --features content-processing` and confirm the token counter module compiles.
  - [x] 3.15 Run `cargo clippy -p paladin-memory --all-features -- -D warnings` and resolve any warnings before proceeding.

- [ ] 4.0 Extract sanctum adapters
  - [ ] 4.1 Copy `src/infrastructure/adapters/sanctum/in_memory_adapter.rs` to `crates/paladin-memory/src/sanctum/in_memory_adapter.rs`.
  - [ ] 4.2 Update all `use crate::core::platform::container::sanctum::*` imports to `use paladin_core::platform::container::sanctum::*`. Verify no other `crate::` references remain.
  - [ ] 4.3 Remove the `#[doc(hidden)]` attribute from both `InMemorySanctum` and `InMemorySanctumConfig`. Add `///` rustdoc to `InMemorySanctumConfig` explaining its `max_entries` field and the LRU eviction behaviour that kicks in when the limit is reached. Ensure both types are `pub` (FR-4.1).
  - [ ] 4.4 Declare `pub mod in_memory_adapter;` in `crates/paladin-memory/src/sanctum/mod.rs` and add `pub use in_memory_adapter::{InMemorySanctum, InMemorySanctumConfig};` (FR-2.3, FR-4.1).
  - [ ] 4.5 Run `cargo build -p paladin-memory --no-default-features` and confirm `InMemorySanctum` and `InMemorySanctumConfig` compile.
  - [ ] 4.6 Copy `src/infrastructure/adapters/sanctum/qdrant_adapter.rs` to `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`.
  - [ ] 4.7 Update all `use crate::core::...` imports to `use paladin_core::...`. Confirm no non-`qdrant_client` external references need feature gating.
  - [ ] 4.8 Add to `sanctum/mod.rs`: `#[cfg(feature = "qdrant")] pub mod qdrant_adapter;` and `#[cfg(feature = "qdrant")] pub use qdrant_adapter::QdrantSanctumAdapter;` (FR-2.3, FR-4.2).
  - [ ] 4.9 Run `cargo build -p paladin-memory --features qdrant` and confirm `QdrantSanctumAdapter` compiles. Run `cargo build -p paladin-memory --no-default-features` and confirm it compiles without `qdrant-client`.
  - [ ] 4.10 Run `cargo clippy -p paladin-memory --all-features -- -D warnings` and resolve any warnings.

- [ ] 5.0 Extract memory services
  - [ ] 5.1 Copy `src/application/use_cases/sanctum/memory_extraction_service.rs` to `crates/paladin-memory/src/services/memory_extraction_service.rs`.
  - [ ] 5.2 Update all `use crate::core::platform::container::*` imports to `use paladin_core::platform::container::*`. Verify the file contains zero `crate::infrastructure::` or `crate::application::` references (FR-5.3, FR-5.4).
  - [ ] 5.3 Confirm that all dependencies are port traits only (`LlmPort`, `SanctumPort`, `EmbeddingPort`) with no references to `InMemorySanctum`, `QdrantSanctumAdapter`, or any concrete adapter type — if any exist, remove them (FR-5.3).
  - [ ] 5.4 Declare `pub mod memory_extraction_service;` in `crates/paladin-memory/src/services/mod.rs` and add `pub use memory_extraction_service::{ExtractedMemory, MemoryExtractionService, MemoryExtractionStrategy};` (FR-2.4).
  - [ ] 5.5 Copy `src/application/use_cases/sanctum/rag_retrieval_service.rs` to `crates/paladin-memory/src/services/rag_retrieval_service.rs`.
  - [ ] 5.6 Update all `use crate::core::...` imports to `use paladin_core::...`. Verify zero concrete adapter references (FR-5.2, FR-5.3, FR-5.4).
  - [ ] 5.7 Add to `services/mod.rs`: `pub mod rag_retrieval_service;` and `pub use rag_retrieval_service::{RagConfig, RagRetrievalService, RetrievalTrigger, retrieve_context_with_timeout};` (FR-2.4).
  - [ ] 5.8 Run `cargo build -p paladin-memory --no-default-features` and confirm both services compile without any optional features.
  - [ ] 5.9 Populate `crates/paladin-memory/src/prelude.rs` with unconditional re-exports: `pub use crate::garrison::InMemoryGarrison;`, `pub use crate::sanctum::{InMemorySanctum, InMemorySanctumConfig};`, `pub use crate::services::{MemoryExtractionService, RagRetrievalService, RagConfig};` (FR-2.5).
  - [ ] 5.10 Run `cargo clippy -p paladin-memory --no-default-features -- -D warnings` and resolve any warnings.

- [ ] 6.0 Migrate and write unit tests
  - [ ] 6.1 Check for existing `TokenCounterFactory` tests: run `grep -rn "TokenCounterFactory" tests/` — if no tests are found, proceed to write them in step 6.5.
  - [ ] 6.2 Read `tests/unit/sanctum/memory_extraction_service_test.rs` in full. At the bottom of `crates/paladin-memory/src/services/memory_extraction_service.rs`, add a `#[cfg(test)] mod tests { use super::*; ... }` block and paste in the test content. Update any import paths that reference `paladin::` to use `paladin_core::` or `paladin_ports::` directly (FR-8.1).
  - [ ] 6.3 Read `tests/unit/sanctum/rag_retrieval_service_test.rs` in full. Add an inline `#[cfg(test)] mod tests { ... }` block at the bottom of `crates/paladin-memory/src/services/rag_retrieval_service.rs` with the same import-path update treatment (FR-8.1).
  - [ ] 6.4 Read `tests/unit/sanctum/qdrant_sanctum_test.rs` in full. Add an inline `#[cfg(test)] mod tests { ... }` block at the bottom of `crates/paladin-memory/src/sanctum/qdrant_adapter.rs`, wrapped in `#[cfg(all(test, feature = "qdrant"))]` (FR-8.1).
  - [ ] 6.5 If no `TokenCounterFactory` tests existed (confirmed in step 6.1), add a `#[cfg(test)] mod tests { ... }` block at the bottom of `crates/paladin-memory/src/garrison/token_counter.rs` (gated on `#[cfg(all(test, feature = "content-processing"))]`) with at minimum these three tests: (a) `test_factory_creates_counter_for_known_model` — construct a factory and call `create("gpt-4")`; assert `Ok`; (b) `test_factory_returns_error_for_unknown_model` — call `create("not-a-real-model-xyz")`; assert `Err`; (c) `test_counter_counts_tokens` — create a counter for a known model and call `count_tokens("Hello, world!")`; assert the result is `Ok` and `> 0` (FR-8.1 addendum).
  - [ ] 6.6 Run `cargo test -p paladin-memory --all-features` and confirm all migrated and newly written tests pass.
  - [ ] 6.7 Delete the now-migrated workspace unit test files: `tests/unit/sanctum/memory_extraction_service_test.rs`, `tests/unit/sanctum/rag_retrieval_service_test.rs`, `tests/unit/sanctum/qdrant_sanctum_test.rs`, and `tests/unit/sanctum/mod.rs`. Do **not** delete `tests/unit/sanctum_domain_tests.rs` or `tests/unit/sanctum_port_tests.rs` — those stay at workspace level.
  - [ ] 6.8 Run `cargo test --workspace --all-features 2>&1 | tail -10` and confirm no test regressions versus the baseline recorded in step 1.1.

- [ ] 7.0 Delete originals and configure facade re-exports
  - [ ] 7.1 Add `paladin-memory` as a direct dependency in the root `Cargo.toml` `[dependencies]` section: `paladin-memory = { workspace = true }`. If specific features need to be forwarded (e.g., for `qdrant` or `sqlite` integration tests), add them as feature-forwarding entries in `[features]` (e.g., `qdrant = ["paladin-memory/qdrant", ...]`) (FR-1.7).
  - [ ] 7.2 Run `cargo build --workspace` before any deletions to confirm the facade compiles with the new dependency in place.
  - [ ] 7.3 Delete the original garrison source files: `src/infrastructure/adapters/garrison/in_memory_garrison.rs`, `src/infrastructure/adapters/garrison/sqlite_garrison.rs`, `src/infrastructure/adapters/garrison/token_counter.rs`.
  - [ ] 7.4 Update `src/infrastructure/adapters/garrison/mod.rs`: remove all `pub mod` and `pub use` declarations for the deleted files; either delete the file if empty or replace its contents with a tombstone comment pointing to `paladin_memory::garrison`.
  - [ ] 7.5 Delete the original sanctum adapter source files: `src/infrastructure/adapters/sanctum/in_memory_adapter.rs`, `src/infrastructure/adapters/sanctum/qdrant_adapter.rs`.
  - [ ] 7.6 Update `src/infrastructure/adapters/sanctum/mod.rs`: remove all `pub mod` and `pub use` declarations for the deleted files; replace with a tombstone comment or delete the file if it is now empty.
  - [ ] 7.7 Delete the original sanctum use case source files: `src/application/use_cases/sanctum/memory_extraction_service.rs`, `src/application/use_cases/sanctum/rag_retrieval_service.rs`.
  - [ ] 7.8 Update `src/application/use_cases/sanctum/mod.rs`: remove `pub mod memory_extraction_service;`, `pub mod rag_retrieval_service;`, and the corresponding `pub use` lines; replace with a tombstone comment or delete if empty.
  - [ ] 7.9 Remove `pub mod garrison;` from `src/infrastructure/adapters/mod.rs` and `pub mod sanctum;` from both `src/infrastructure/adapters/mod.rs` and `src/application/use_cases/mod.rs` if those entries now point to empty/deleted modules (FR-6.2).
  - [ ] 7.10 Audit existing `use paladin::...` paths for all garrison, sanctum, and services types: run `grep -rn "infrastructure::adapters::garrison\|infrastructure::adapters::sanctum\|use_cases::sanctum" --include="*.rs" examples/ tests/ src/` and list every match.
  - [ ] 7.11 Add `pub use paladin_memory::...` re-export statements to `src/lib.rs` covering every path found in step 7.10 per the mapping table in PRD FR-7. Group them under a clearly labelled comment block (e.g., `// Memory Adapters (re-exported from paladin-memory)`).
  - [ ] 7.12 Run `cargo build --workspace` and confirm everything compiles after the deletions and re-exports.
  - [ ] 7.13 Run `cargo test --workspace --all-features 2>&1 | tail -10` and confirm zero regressions.

- [ ] 8.0 Final quality gates and workspace verification
  - [ ] 8.1 Run `cargo build -p paladin-memory --no-default-features` and then `cargo tree -p paladin-memory --no-default-features | grep -E "sqlx|qdrant|tiktoken"` — confirm no matches (FR-9, Story 1).
  - [ ] 8.2 Run `cargo build -p paladin-memory --features sqlite` — confirm it succeeds and `SqliteGarrison` is compiled (FR-9).
  - [ ] 8.3 Run `cargo build -p paladin-memory --features qdrant` — confirm it succeeds and `QdrantSanctumAdapter` is compiled (FR-9).
  - [ ] 8.4 Run `cargo build -p paladin-memory --features content-processing` — confirm it succeeds and `TiktokenCounter` is compiled (FR-9).
  - [ ] 8.5 Run `cargo build -p paladin-memory --all-features` — confirm all components compile together (FR-9).
  - [ ] 8.6 Run `cargo build --workspace` — confirm full workspace build succeeds (FR-9).
  - [ ] 8.7 Run `cargo test -p paladin-memory --all-features` — confirm all crate-level unit tests pass (FR-8.3).
  - [ ] 8.8 Run `cargo test --workspace --all-features` — confirm all workspace tests pass with zero regressions versus the baseline from step 1.1 (FR-8.4, FR-9).
  - [ ] 8.9 Run `cargo clippy -p paladin-memory --all-features -- -D warnings` — confirm zero warnings (FR-9).
  - [ ] 8.10 Run `cargo clippy --workspace -- -D warnings` — confirm zero warnings across all crates.
  - [ ] 8.11 Run `cargo fmt --all -- --check` — confirm all code is properly formatted; run `cargo fmt --all` to fix any issues, then re-check.
  - [ ] 8.12 Run `cargo doc -p paladin-memory --no-deps 2>&1 | grep -i "warn\|error"` — confirm no broken intra-doc links or missing documentation warnings (FR-9).
  - [ ] 8.13 Smoke-run the garrison and sanctum examples to confirm they compile without import path changes: `cargo build --example garrison_in_memory`, `cargo build --example garrison_persistent`, `cargo build --example garrison_semantic_search` (add `--features` as needed).
  - [ ] 8.14 Compare the final test count from step 8.8 against the baseline from step 1.1 and confirm the count is equal or higher (migrated unit tests now run via `cargo test -p paladin-memory` in addition to the workspace run).
  - [ ] 8.15 Stage all changes and commit: `git add .` then `git commit -m "feat: extract paladin-memory crate from monolith" -m "- Scaffold crates/paladin-memory with sqlite, qdrant, content-processing feature flags" -m "- Extract InMemoryGarrison, SqliteGarrison, TokenCounter/TiktokenCounter" -m "- Extract InMemorySanctum, QdrantSanctumAdapter" -m "- Extract MemoryExtractionService, RagRetrievalService" -m "- Migrate unit tests to inline #[cfg(test)] modules" -m "- Delete original source files; add facade re-exports in src/lib.rs" -m "- Hoist sqlx and qdrant-client to [workspace.dependencies]" -m "Closes Epic 5 — Milestone 5 Workspace Decomposition"`
