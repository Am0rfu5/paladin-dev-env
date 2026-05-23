## Relevant Files

### New Files (to be created)

- `src/config/env_utils.rs` — `EnvOverridable` trait and `read_env::<T>` generic helper function.
- `src/config/herald.rs` — `JsonHeraldConfig`, `MarkdownHeraldConfig`, `TableHeraldConfig`, `HeraldConfig`.
- `src/config/scheduler.rs` — `SchedulerConfig`.
- `src/config/citadel.rs` — `CitadelConfig`.
- `src/config/arsenal.rs` — `MCPServerConfig`, `ArsenalConfig`.
- `src/config/queue.rs` — `QueueConfig` (implements `EnvOverridable`).
- `src/config/file_storage.rs` — `FileStorageConfig` (implements `EnvOverridable`; gated on `s3-storage` feature).
- `src/config/notifications.rs` — `NotificationConfig` (implements `EnvOverridable`; gated on `notifications` feature).
- `src/config/web_server.rs` — `SourceConfig`, `ServerConfig`, `MessageServiceSettings`.
- `crates/paladin-llm/src/config/mod.rs` — Re-exports all `paladin-llm` config types.
- `crates/paladin-llm/src/config/vision.rs` — `VisionRetryConfig`, `VisionProviderConfig`, `VisionConfig`.
- `crates/paladin-llm/src/config/llm.rs` — `LlmProviderConfig`, `LlmConfig` (implements `EnvOverridable`).
- `crates/paladin-memory/src/config/mod.rs` — Re-exports all `paladin-memory` config types.
- `crates/paladin-memory/src/config/garrison.rs` — `GarrisonSettings` (implements `EnvOverridable`).
- `crates/paladin-memory/src/config/sanctum.rs` — `QdrantSanctumConfig`, `SanctumConfig` (implements `EnvOverridable`).
- `crates/paladin-memory/src/config/rag.rs` — `RagConfig` (canonical), `MemoryExtractionConfig`.

### Existing Files (to be modified)

- `src/config/application_settings.rs` — Monolithic file being decomposed; replaced by `pub use` re-exports during migration, then deleted.
- `src/config/mod.rs` — Gains `pub mod` declarations for all new domain files, `pub use` re-exports, and the `Settings` struct moved here in Task 8.
- `src/main.rs` — Import path update (consumer of `Settings`).
- `src/application/cli/commands/arsenal.rs` — Import path update (consumer of `Settings`).
- `src/application/use_cases/paladin/paladin_builder.rs` — Import path update (consumer of `MCPServerConfig`).
- `src/infrastructure/repositories/sqlite_user_repository.rs` — Import path update (consumer of `Settings`).
- `src/infrastructure/adapters/llm/config_bridge.rs` — Import path update (consumer of `LlmProviderConfig`, `VisionConfig`).
- `src/infrastructure/adapters/input/local_file_fetcher.rs` — Verify/remove commented-out import.
- `src/config/setup/mod.rs` — Import path update (consumer of `Settings`).
- `src/config/setup/service_runner.rs` — Import path update (consumer of `Settings`).
- `src/config/user_config.rs` — Import path update (highest-risk consumer — 15+ inline struct references).
- `crates/paladin-llm/src/lib.rs` — Add `pub mod config;`.
- `crates/paladin-memory/src/lib.rs` — Add `pub mod config;`.
- `crates/paladin-memory/src/prelude.rs` — Add re-exports for new config types.
- `crates/paladin-memory/src/services/rag_retrieval_service.rs` — Update `RagConfig` import to `crate::config::rag::RagConfig`.
- `crates/paladin-memory/Cargo.toml` — Add `serial_test` as a dev-dependency (needed for migrated env-var tests using `#[serial]`).
- `crates/paladin-llm/Cargo.toml` — Add `serial_test` as a dev-dependency if any migrated LLM tests use `#[serial]`.

### Reference Files (read-only)

- `config.test.yml` — Reference config used for regression tests in Task 9.
- `config.yml` — Production config; YAML schema must remain unchanged.

### Notes

- Unit tests in Rust live **in the same file as the code they test**, inside a `#[cfg(test)] mod tests { use super::*; }` block.
- Tests using `#[serial]` (from the `serial_test` crate) prevent env-var test collisions — preserve this attribute when migrating tests to new files.
- Run `cargo build` (not just `cargo check`) after each domain extraction to catch linker errors from feature-gated code.
- Run `cargo test` after moving tests to their new home before proceeding to the next domain.
- The incremental migration strategy means the workspace must compile successfully at the end of every sub-task.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Checkout branch feature/milestone_6: `git checkout feature/milestone_6`
  - [x] 0.2 Create and checkout a new branch: `git checkout -b feature/milestone_6-epic_1-decompose-app-settings`

- [x] 1.0 Audit and map all configuration domains
  - [x] 1.1 Read `src/config/application_settings.rs` in full; note the line range of every struct definition.
  - [x] 1.2 For each struct, record its target crate (`paladin-memory`, `paladin-llm`, or facade `paladin`) and target file path (use the mapping table in the PRD Section 4.1 as the authoritative guide).
  - [x] 1.3 List every `Settings::get_*_config()` method and identify which struct's `apply_env_overrides()` should absorb its env-var override logic.
  - [x] 1.4 For each of the 10 consumer files (`src/main.rs`, `src/application/cli/commands/arsenal.rs`, `src/application/use_cases/paladin/paladin_builder.rs`, `src/infrastructure/repositories/sqlite_user_repository.rs`, `src/infrastructure/adapters/llm/config_bridge.rs`, `src/infrastructure/adapters/input/local_file_fetcher.rs`, `src/config/setup/mod.rs`, `src/config/setup/service_runner.rs`, `src/config/user_config.rs`, `src/config/mod.rs`), list which structs each imports from `application_settings`.
  - [x] 1.5 Compare the `RagConfig` struct definition in `application_settings.rs` with the one in `crates/paladin-memory/src/services/rag_retrieval_service.rs` field-by-field; document any differences that must be reconciled before deduplication.

- [x] 2.0 Create `EnvOverridable` trait and `read_env` helper
  - [x] 2.1 Create `src/config/env_utils.rs` with the `EnvOverridable` trait: a single required method `fn apply_env_overrides(&mut self)`.
  - [x] 2.2 Add the `read_env::<T: std::str::FromStr>(var_name: &str) -> Option<T>` free function to the same file; it reads an env var with `std::env::var`, then parses with `.parse::<T>().ok()`.
  - [x] 2.3 Write unit tests for `read_env` covering: `String` (set and unset), `u16` (valid and invalid value), `u64`, `bool` ("true"/"false"), and `Option<String>` (unset returns `None`). Use `#[serial]` from `serial_test` for all env-var tests to prevent inter-test pollution.
  - [x] 2.4 Add `pub mod env_utils;` to `src/config/mod.rs`.
  - [x] 2.5 Run `cargo test config::env_utils` — all new tests must pass.

- [ ] 3.0 Initialize `config` modules in `paladin-memory` and `paladin-llm` workspace crates
  - [ ] 3.1 Create `crates/paladin-memory/src/config/mod.rs` as an empty file with a single doc comment: `//! Per-domain configuration types for the paladin-memory crate.`
  - [ ] 3.2 Add `pub mod config;` to `crates/paladin-memory/src/lib.rs` (after the existing `pub mod garrison;` line).
  - [ ] 3.3 Create `crates/paladin-llm/src/config/mod.rs` as an empty file with a single doc comment: `//! Per-domain configuration types for the paladin-llm crate.`
  - [ ] 3.4 Add `pub mod config;` to `crates/paladin-llm/src/lib.rs` (after the existing `pub mod error;` line).
  - [ ] 3.5 Run `cargo build --workspace` — must compile with zero errors before proceeding.

- [ ] 4.0 Extract facade-crate config modules (Herald, Scheduler, Citadel, Arsenal, Queue, FileStorage, Notifications, WebServer)
  - [ ] 4.1 **Herald:** Create `src/config/herald.rs`; move `JsonHeraldConfig`, `MarkdownHeraldConfig`, `TableHeraldConfig`, and `HeraldConfig` (with all `impl Default` blocks) from `application_settings.rs`. Replace the four struct definitions in `application_settings.rs` with `pub use crate::config::herald::{HeraldConfig, JsonHeraldConfig, MarkdownHeraldConfig, TableHeraldConfig};`. Run `cargo build`.
  - [ ] 4.2 **Scheduler:** Create `src/config/scheduler.rs`; move `SchedulerConfig` with its `impl Default` and `impl` blocks. Add re-export in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.3 **Citadel:** Create `src/config/citadel.rs`; move `CitadelConfig` with its `impl Default` and `validate()` blocks. Add re-export in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.4 **Arsenal:** Create `src/config/arsenal.rs`; move `MCPServerConfig` and `ArsenalConfig` with all `impl` blocks. Add re-exports in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.5 **Queue:** Create `src/config/queue.rs`; move `QueueConfig` with its `impl Default` block. Implement `EnvOverridable` for `QueueConfig` by moving the env-var override logic currently in `Settings::get_queue_config()` into `QueueConfig::apply_env_overrides()`. Add re-export in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.6 **FileStorage:** Create `src/config/file_storage.rs`; move `FileStorageConfig` and its `impl` blocks, preserving the `#[cfg(feature = "s3-storage")]` gate on the entire file. Implement `EnvOverridable` by moving the logic from `Settings::get_file_storage_config()`. Add re-export in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.7 **Notifications:** Create `src/config/notifications.rs`; move `NotificationConfig` and its `impl` blocks, preserving any `#[cfg(feature = "notifications")]` gates. Implement `EnvOverridable` by moving the logic from `Settings::get_notification_config()`. Add re-export in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.8 **WebServer:** Create `src/config/web_server.rs`; move `SourceConfig`, `ServerConfig`, and `MessageServiceSettings` with all `impl` blocks. Add re-exports in `application_settings.rs`. Run `cargo build`.
  - [ ] 4.9 Add `pub mod herald;`, `pub mod scheduler;`, `pub mod citadel;`, `pub mod arsenal;`, `pub mod queue;`, `pub mod file_storage;`, `pub mod notifications;`, and `pub mod web_server;` declarations to `src/config/mod.rs`.
  - [ ] 4.10 Move each struct's existing tests from `application_settings.rs` to its new file (inside a `#[cfg(test)] mod tests { use super::*; }` block). Add `serial_test` as a workspace dev-dependency in sub-crates only if needed — these tests stay in the facade crate where `serial_test` is already available via the workspace `Cargo.toml`.
  - [ ] 4.11 Run `cargo test` — all tests must pass before proceeding.

- [ ] 5.0 Extract `paladin-llm` config modules (Vision, LLM providers)
  - [ ] 5.1 **Vision:** Create `crates/paladin-llm/src/config/vision.rs`; move `VisionRetryConfig`, `VisionProviderConfig`, and `VisionConfig` with all `impl Default` and `validate()` blocks. Add `pub mod vision;` to `crates/paladin-llm/src/config/mod.rs`. Add `pub use crate::config::vision::*;` re-exports to `crates/paladin-llm/src/config/mod.rs`.
  - [ ] 5.2 In `application_settings.rs`, replace the three Vision struct definitions with cross-crate re-exports: `pub use paladin_llm::config::vision::{VisionRetryConfig, VisionProviderConfig, VisionConfig};`. Run `cargo build`.
  - [ ] 5.3 **LLM:** Create `crates/paladin-llm/src/config/llm.rs`; move `LlmProviderConfig` and `LlmConfig` with all `impl` blocks including `get_provider_config()`, `get_default_provider_name()`, and `validate()`. Implement `EnvOverridable` for `LlmConfig` if it has env-var override logic. Add `pub mod llm;` to `crates/paladin-llm/src/config/mod.rs` and re-export types.
  - [ ] 5.4 In `application_settings.rs`, replace the two LLM struct definitions with cross-crate re-exports: `pub use paladin_llm::config::llm::{LlmProviderConfig, LlmConfig};`. Run `cargo build`.
  - [ ] 5.5 Add `pub mod config;` re-export visibility: ensure `paladin-llm`'s `lib.rs` exposes config types publicly so the facade can use `paladin_llm::config::llm::LlmConfig`.
  - [ ] 5.6 Add `serial_test` as a dev-dependency to `crates/paladin-llm/Cargo.toml` if any migrated tests use `#[serial]`. Then move Vision and LLM tests from `application_settings.rs` to the new files.
  - [ ] 5.7 Update `src/infrastructure/adapters/llm/config_bridge.rs` to import `LlmProviderConfig` and `VisionConfig` from their new locations (`paladin_llm::config::llm::LlmProviderConfig`, `paladin_llm::config::vision::VisionConfig`).
  - [ ] 5.8 Run `cargo test --workspace` — all tests must pass.

- [ ] 6.0 Extract `paladin-memory` config modules (Garrison, Sanctum, RAG) and deduplicate `RagConfig`
  - [ ] 6.1 **Garrison:** Create `crates/paladin-memory/src/config/garrison.rs`; move `GarrisonSettings` with all `impl` blocks including `get_eviction_strategy()` and `validate()`. Implement `EnvOverridable` by moving the env-var override logic from `Settings::get_garrison_config()`. Add `pub mod garrison;` to `crates/paladin-memory/src/config/mod.rs` with re-exports.
  - [ ] 6.2 In `application_settings.rs`, replace `GarrisonSettings` with a cross-crate re-export: `pub use paladin_memory::config::garrison::GarrisonSettings;`. Run `cargo build`.
  - [ ] 6.3 **Sanctum:** Create `crates/paladin-memory/src/config/sanctum.rs`; move `QdrantSanctumConfig` and `SanctumConfig` with all `impl` blocks including `validate()` and `adapter_type_str()`. Implement `EnvOverridable` by moving the logic from `Settings::get_sanctum_config()`. Add `pub mod sanctum;` to `crates/paladin-memory/src/config/mod.rs` with re-exports.
  - [ ] 6.4 In `application_settings.rs`, replace the two Sanctum struct definitions with cross-crate re-exports. Run `cargo build`.
  - [ ] 6.5 **RagConfig deduplication:** Create `crates/paladin-memory/src/config/rag.rs`. Move `MemoryExtractionConfig` from `application_settings.rs` to this file. Reconcile the two `RagConfig` definitions (using the diff from Task 1.5); define the unified `RagConfig` in this file with all fields from both sources.
  - [ ] 6.6 Update `crates/paladin-memory/src/services/rag_retrieval_service.rs` to import `RagConfig` from `crate::config::rag::RagConfig` instead of defining it locally. Remove the old struct definition from `rag_retrieval_service.rs`.
  - [ ] 6.7 Add `pub mod rag;` to `crates/paladin-memory/src/config/mod.rs` with re-exports for `RagConfig` and `MemoryExtractionConfig`.
  - [ ] 6.8 In `application_settings.rs`, replace the RAG struct definitions with cross-crate re-exports: `pub use paladin_memory::config::rag::{RagConfig, MemoryExtractionConfig};`. Run `cargo build`.
  - [ ] 6.9 Update `crates/paladin-memory/src/prelude.rs` to re-export the new config types: add `pub use crate::config::{garrison::GarrisonSettings, sanctum::SanctumConfig, rag::{RagConfig, MemoryExtractionConfig}};`.
  - [ ] 6.10 Add `serial_test` as a dev-dependency to `crates/paladin-memory/Cargo.toml` (needed for migrated env-var tests that use `#[serial]`). Move Garrison, Sanctum, and RAG tests from `application_settings.rs` to the new files.
  - [ ] 6.11 Run `cargo test --workspace` — all tests must pass.

- [ ] 7.0 Migrate all consumer files to new import paths
  - [ ] 7.1 Update `src/main.rs`: change `use paladin::config::application_settings::Settings;` to `use paladin::config::Settings;`.
  - [ ] 7.2 Update `src/application/cli/commands/arsenal.rs`: update the `Settings` import to `use crate::config::Settings;`.
  - [ ] 7.3 Update `src/application/use_cases/paladin/paladin_builder.rs`: update `MCPServerConfig` import to `use crate::config::arsenal::MCPServerConfig;`.
  - [ ] 7.4 Update `src/infrastructure/repositories/sqlite_user_repository.rs`: update `Settings` import to `use crate::config::Settings;`.
  - [ ] 7.5 Update `src/infrastructure/adapters/llm/config_bridge.rs`: confirm imports updated in Task 5.7; if not done, update now.
  - [ ] 7.6 Update `src/infrastructure/adapters/input/local_file_fetcher.rs`: remove or update the commented-out import; if no longer needed, delete it.
  - [ ] 7.7 Update `src/config/setup/mod.rs`: update `Settings` import to `use crate::config::Settings;`.
  - [ ] 7.8 Update `src/config/setup/service_runner.rs`: update `Settings` import to `use crate::config::Settings;`.
  - [ ] 7.9 Update `src/config/user_config.rs` (highest-risk): replace all 15+ occurrences of `crate::config::application_settings::XxxConfig` with the new module paths (e.g., `crate::config::queue::QueueConfig`, `crate::config::garrison::GarrisonSettings`). Verify the `UserServiceFactory` struct still compiles and its inline `Settings` construction still produces correct defaults.
  - [ ] 7.10 Update `src/config/mod.rs`: add `pub use` re-exports so that external consumers using the path `paladin::config::XxxConfig` continue to work (e.g., `pub use crate::config::queue::QueueConfig;`). Remove the `pub use` re-exports from `application_settings.rs` now that consumers import directly.
  - [ ] 7.11 Run `cargo build --workspace` — must compile cleanly with zero errors.

- [ ] 8.0 Delete `application_settings.rs` and finalize `config/mod.rs`
  - [ ] 8.1 Verify `application_settings.rs` now contains **only** the `Settings` struct and its `impl` blocks (`new()`, `load_from_file()`, and `get_*_config()` methods). Run `grep "^pub struct" src/config/application_settings.rs` — it must return only `pub struct Settings`.
  - [ ] 8.2 Move the `Settings` struct definition and all its `impl` blocks from `application_settings.rs` into `src/config/mod.rs`.
  - [ ] 8.3 Update each `Settings::get_*_config()` method body to call `apply_env_overrides()` on the cloned sub-struct, replacing the manual field-override logic. Example: `pub fn get_queue_config(&self) -> QueueConfig { let mut cfg = self.queue.clone().unwrap_or_default(); cfg.apply_env_overrides(); cfg }`.
  - [ ] 8.4 Remove `pub mod application_settings;` from `src/config/mod.rs`.
  - [ ] 8.5 Delete `src/config/application_settings.rs`.
  - [ ] 8.6 Run `cargo build --workspace` — must compile cleanly.
  - [ ] 8.7 Verify deletion: `grep -r "application_settings" src/` must return zero results.
  - [ ] 8.8 Run `cargo test --workspace` — all tests must pass.
  - [ ] 8.9 Run `cargo clippy --workspace -- -D warnings` — must be clean with zero warnings.
  - [ ] 8.10 Run `cargo fmt --all -- --check` — must be clean; run `cargo fmt --all` to fix any formatting issues, then re-check.

- [ ] 9.0 Add regression tests and verify success metrics
  - [ ] 9.1 In `src/config/mod.rs`, add a `#[cfg(test)] mod tests` block with a regression test that calls `Settings::load_from_file("config.test.yml")` and asserts at least one field per domain (e.g., assert `settings.queue.unwrap().host` equals the value in `config.test.yml`).
  - [ ] 9.2 For each domain that has env-var override logic, add an integration test that: sets the relevant env var, calls `settings.get_*_config()`, asserts the field is overridden, then unsets the env var. Use `#[serial]` on each test.
  - [ ] 9.3 Verify no config file exceeds 400 lines: run `find crates/*/src/config src/config -name "*.rs" | xargs wc -l | sort -rn | head -20`. If any file is over 400 lines, split it further.
  - [ ] 9.4 Verify `RagConfig` is unified: run `grep -r "struct RagConfig" crates/` — must return exactly one result.
  - [ ] 9.5 Run the full test suite one final time: `cargo test --workspace` — zero failures.
  - [ ] 9.6 Run `cargo clippy --workspace -- -D warnings` and `cargo fmt --all -- --check` — both must be clean.
  - [ ] 9.7 Stage all changes and commit: `git add .` then `git commit -m "feat: decompose application_settings.rs into per-domain config modules" -m "- Extract 13 domain config modules across facade, paladin-llm, paladin-memory" -m "- Introduce EnvOverridable trait with read_env helper to eliminate ~30 duplicated env-var override patterns" -m "- Deduplicate RagConfig; canonical location is paladin-memory/src/config/rag.rs" -m "- Migrate 128+ tests to co-located test modules in new files" -m "- Delete application_settings.rs; Settings struct moved to config/mod.rs" -m "Closes Epic 1 of Milestone 6"`.
