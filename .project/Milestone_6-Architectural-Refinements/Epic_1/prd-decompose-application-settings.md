# PRD: Decompose `application_settings.rs` into Per-Domain Configuration Modules

**Feature Name:** decompose-application-settings
**Milestone:** 6 — Architectural Refinements
**Epic:** 1
**Status:** Ready for Implementation
**Created:** 2026-05-23
**Author:** AI-assisted, reviewed by team

---

## 1. Introduction / Overview

`src/config/application_settings.rs` is a 2,636-line monolithic file that contains every configuration type for every subsystem in the Paladin framework. This includes configuration for the LLM providers, memory (Garrison), Arsenal tool registry, notifications, file storage, Redis queue, Sanctum vector store, RAG retrieval, Herald output formatting, Vision, Citadel state persistence, Scheduler, and more — all in a single file.

**The problem this solves:** When a developer needs to change or understand how one subsystem is configured (say, the Garrison memory system), they must navigate a 2,636-line file that contains completely unrelated configuration types. This causes unnecessary cognitive load, creates merge conflicts when multiple developers modify different domains simultaneously, and makes onboarding harder.

**The goal:** Replace `application_settings.rs` with a set of focused, per-domain configuration modules — one file per subsystem. Move each domain's config struct into the workspace sub-crate it belongs to. Use an incremental migration strategy so the application remains functional throughout the refactor.

---

## 2. Goals

1. Replace `application_settings.rs` with a `config/` module directory, each file responsible for exactly one domain.
2. Move each domain's configuration struct into its corresponding workspace sub-crate (e.g., `GarrisonSettings` → `paladin-memory`, `LlmConfig` → `paladin-llm`).
3. Introduce an `EnvOverridable` trait to eliminate the repeated pattern of reading environment variables and overriding struct fields, which currently appears ~30 times.
4. Maintain full backward compatibility: no changes to `config.yml` structure, no breaking changes to the `Settings` public API.
5. Use an incremental migration: add new modules with temporary re-exports from the old file first, then remove the old file in a follow-up step once all consumers are updated.
6. Keep no single config file over 400 lines.
7. Ensure all 128+ existing config-related tests continue to pass throughout the migration.

---

## 3. User Stories

**As a developer working on the Garrison memory system,**
I want to find Garrison's configuration in a single, focused file (`paladin-memory/src/config/garrison.rs`),
so that I don't need to scroll through thousands of lines of unrelated configuration to find what I need.

**As a developer adding a new LLM provider,**
I want to add my provider's config struct to `crates/paladin-llm/src/config/llm.rs`,
so that my change is isolated from all other subsystems and won't cause merge conflicts.

**As a developer implementing env-var override support for a new config field,**
I want to implement the `EnvOverridable` trait on my config struct,
so that I'm guided by a consistent pattern rather than copying and pasting boilerplate.

**As a developer onboarding to the codebase,**
I want each workspace sub-crate to own its own configuration types,
so that I can understand a sub-crate in isolation without needing to read the facade crate's config file.

---

## 4. Functional Requirements

### 4.1 Domain Config File Structure

The system must create the following config modules. Each file must not exceed 400 lines.

#### In the `paladin-memory` crate (`crates/paladin-memory/src/config/`)

| File | Structs to move there |
|------|----------------------|
| `config/mod.rs` | Re-exports all types from sub-modules |
| `config/garrison.rs` | `GarrisonSettings` and its `impl` blocks |
| `config/sanctum.rs` | `QdrantSanctumConfig`, `SanctumConfig` and their `impl` blocks |
| `config/rag.rs` | `MemoryExtractionConfig`. **Note:** `RagConfig` already exists in `paladin-memory/src/services/rag_retrieval_service.rs` — consolidate: use the existing one and remove the duplicate from `application_settings.rs`. |

#### In the `paladin-llm` crate (`crates/paladin-llm/src/config/`)

| File | Structs to move there |
|------|----------------------|
| `config/mod.rs` | Re-exports all types from sub-modules |
| `config/llm.rs` | `LlmProviderConfig`, `LlmConfig` and their `impl` blocks including `get_provider_config()` and `get_default_provider_name()` |
| `config/vision.rs` | `VisionRetryConfig`, `VisionProviderConfig`, `VisionConfig` and their `impl` blocks |

#### In the facade crate (`src/config/`)

These structs do not have a dedicated sub-crate and remain in the facade crate's `config/` module:

| File | Structs to move there |
|------|----------------------|
| `config/mod.rs` | The root `Settings` struct, `Settings::new()`, `Settings::load_from_file()`, all `get_*_config()` methods, and re-exports of sub-crate config types |
| `config/arsenal.rs` | `MCPServerConfig`, `ArsenalConfig` |
| `config/citadel.rs` | `CitadelConfig` |
| `config/file_storage.rs` | `FileStorageConfig` |
| `config/herald.rs` | `JsonHeraldConfig`, `MarkdownHeraldConfig`, `TableHeraldConfig`, `HeraldConfig` |
| `config/notifications.rs` | `NotificationConfig` |
| `config/queue.rs` | `QueueConfig` |
| `config/scheduler.rs` | `SchedulerConfig` |
| `config/sanctum_app.rs` | *(if any Sanctum config stays in the facade for infrastructure-level settings)* |
| `config/web_server.rs` | `SourceConfig`, `ServerConfig`, `MessageServiceSettings` |

### 4.2 The `EnvOverridable` Trait

The system must introduce an `EnvOverridable` trait. The current code repeats the read-env-var-and-parse pattern about 30 times across `get_queue_config()`, `get_file_storage_config()`, `get_garrison_config()`, etc.

**The trait must be defined in a shared location** accessible by all crates that need it. A good place is `crates/paladin-core/src/config/env_utils.rs` or as part of the facade's `config/env_utils.rs` if `paladin-core` cannot depend on it.

**Trait definition (example):**
```rust
/// Implemented by configuration structs that support environment variable overrides.
pub trait EnvOverridable {
    /// Apply any environment variable overrides to this config.
    /// Each struct reads its specific env vars and overrides its own fields.
    fn apply_env_overrides(&mut self);
}
```

**Helper function for reading env vars (must be provided):**
```rust
/// Read an environment variable and parse it to type T.
/// Returns Some(value) if the variable is set and parseable, None otherwise.
pub fn read_env<T: std::str::FromStr>(var_name: &str) -> Option<T> {
    std::env::var(var_name).ok()?.parse().ok()
}
```

Each config struct that currently has env-var override logic in `Settings::get_*_config()` must instead implement `EnvOverridable` and move that logic into `apply_env_overrides()`.

**Unit tests must cover:** `read_env::<String>`, `read_env::<u16>`, `read_env::<u64>`, `read_env::<bool>`, and `read_env::<Option<_>>` with both set and unset env vars.

### 4.3 Root `Settings` Struct

The `Settings` struct must remain the single entry point for loading configuration. It stays in `src/config/mod.rs` (facade crate) and composes all sub-configs. Its public API must not change:

```rust
pub struct Settings {
    // all existing fields remain, types updated to point to new module paths
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> { ... }
    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> { ... }

    // These methods remain but now delegate to the sub-struct's apply_env_overrides():
    pub fn get_queue_config(&self) -> QueueConfig { ... }
    pub fn get_file_storage_config(&self) -> FileStorageConfig { ... }
    pub fn get_notification_config(&self) -> NotificationConfig { ... }
    pub fn get_garrison_config(&self) -> GarrisonSettings { ... }
    pub fn get_sanctum_config(&self) -> SanctumConfig { ... }
}
```

### 4.4 Migration Strategy: Incremental with Temporary Re-Exports

The migration must be done incrementally to keep the codebase compiling at every step:

**Step A (per domain):** Create the new domain file in the target crate/module. Move the struct and its `impl` blocks there. In `application_settings.rs`, add a `pub use` re-export so existing consumers keep compiling:
```rust
// Temporary re-export — remove in cleanup step
pub use crate::config::queue::QueueConfig;
// OR for cross-crate:
pub use paladin_memory::config::garrison::GarrisonSettings;
```

**Step B (per file):** Update all consumers (files in `src/` that import from `application_settings`) to import from the new path.

**Step C (cleanup):** Once all re-exports in `application_settings.rs` are no longer needed, delete the file and update `config/mod.rs` to no longer reference it.

The 29 known consumer files that import from `application_settings` are:
- `src/application/cli/commands/arsenal.rs`
- `src/application/use_cases/paladin/paladin_builder.rs`
- `src/infrastructure/repositories/sqlite_user_repository.rs`
- `src/infrastructure/adapters/llm/config_bridge.rs`
- `src/main.rs`
- `src/config/setup/mod.rs`
- `src/config/setup/service_runner.rs`
- `src/config/user_config.rs` (most imports — 15+ references)

**The file `config/user_config.rs` is the highest-risk consumer** because it constructs `Settings` inline with many sub-struct defaults. Update it last, after all sub-structs are confirmed stable in their new locations.

### 4.5 `config.yml` Backward Compatibility

The `config.yml` deserialization contract must not change. All `#[serde(default)]` and `#[serde(rename)]` attributes on struct fields must be preserved exactly when moving structs to new files.

### 4.6 RagConfig Deduplication

`RagConfig` exists in two places today:
1. `src/config/application_settings.rs` (as a top-level app-settings struct)
2. `crates/paladin-memory/src/services/rag_retrieval_service.rs` (as a service config)

The system must consolidate to a single `RagConfig`. The canonical location must be `crates/paladin-memory/src/config/rag.rs`. The `application_settings.rs` version must be removed and replaced with a re-export from `paladin-memory`. Update all consumers to use the `paladin-memory` version.

---

## 5. Non-Goals (Out of Scope)

- **No changes to `config.yml` schema.** YAML key names must remain identical.
- **No new configuration options.** This is a structural refactor only — no new fields, no new behaviors.
- **No performance optimization** of the config loading code.
- **No changes to the notification service domain model** (it was rebuilt in a prior Epic).
- **No further workspace crate extractions** beyond those completed in Milestone 5.
- **No changes to `STABLE_API.md`** — the public API surface is frozen.
- **No CLI changes.** The CLI command structure is not modified.

---

## 6. Design Considerations

### File Size Constraint

No file in any `config/` module may exceed 400 lines. If a domain's config struct is large (e.g., `GarrisonSettings` with its eviction strategy methods spans ~100 lines), that is fine. If you find a struct growing beyond ~350 lines, split it further.

### Crate Dependency Direction

Crate dependencies flow inward only:
- `paladin-core` must not import from `paladin-memory` or `paladin-llm`.
- `paladin-memory` and `paladin-llm` may import from `paladin-core` and `paladin-ports`.
- The facade crate (`paladin`) may import from all sub-crates.

This means the `EnvOverridable` trait, if placed in `paladin-core`, can be implemented by all crates. If the trait is only needed by the facade, place it in `src/config/env_utils.rs` instead.

### Serialization Attributes

When moving a struct, copy its `#[derive(...)]` line exactly. Common derives in this file are:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
```
Do not add or remove derives.

Pay special attention to:
- `#[serde(default)]` on fields (preserves YAML backward compatibility when fields are absent)
- `#[serde(skip_serializing_if = "Option::is_none")]`
- `#[cfg(feature = "...")]` feature gates on structs and their fields (e.g., `FileStorageConfig` is gated on `s3-storage`)

---

## 7. Technical Considerations

### Feature Flags

Several structs are conditionally compiled. Preserve all `#[cfg(feature = "...")]` gates:

| Struct | Feature Flag |
|--------|-------------|
| `FileStorageConfig` | `s3-storage` |
| `NotificationConfig` fields | `notifications` |
| Parts of `LlmConfig` | provider-specific flags |

When moving these structs, the `cfg` gate must move with them.

### Cross-Crate Config Re-exports in `paladin-memory`

Add a `config` module to `paladin-memory`:
1. Create `crates/paladin-memory/src/config/mod.rs`
2. Create `crates/paladin-memory/src/config/garrison.rs`
3. Create `crates/paladin-memory/src/config/sanctum.rs`
4. Create `crates/paladin-memory/src/config/rag.rs`
5. Expose via `paladin-memory`'s `lib.rs`: `pub mod config;`
6. Add to `paladin-memory/src/prelude.rs` re-exports as appropriate

Similarly for `paladin-llm`:
1. Create `crates/paladin-llm/src/config/mod.rs`
2. Create `crates/paladin-llm/src/config/llm.rs`
3. Create `crates/paladin-llm/src/config/vision.rs`
4. Expose via `paladin-llm`'s `lib.rs`: `pub mod config;`

### `config_bridge.rs`

`src/infrastructure/adapters/llm/config_bridge.rs` bridges `ApplicationSettings` LLM config types to the adapter's own config types. After moving `LlmProviderConfig` and `VisionConfig` to `paladin-llm`, this bridge's imports must be updated. Verify its tests still pass after the move.

### Test Migration

The 128 existing tests in `application_settings.rs` use `#[cfg(test)] mod tests { use super::*; }`. After each struct moves to its new file, the tests for that struct must move too. Place tests immediately after the struct's `impl` block in the new file, in a `#[cfg(test)] mod tests { use super::*; }` block.

Tests that use `#[serial]` (serial_test crate) to avoid env var collision must keep that attribute.

---

## 8. Success Metrics

1. **All tests pass:** `cargo test --workspace` produces zero failures after each task.
2. **No file exceeds 400 lines:** Verified by: `find crates/*/src/config src/config -name "*.rs" | xargs wc -l`
3. **`application_settings.rs` is deleted** by the end of Task 1.4 (cleanup step).
4. **No new public API surface:** `cargo doc --workspace --no-deps` shows no new public items beyond what existed before.
5. **`cargo clippy --workspace -- -D warnings` is clean** at the end of each task.
6. **`config.yml` loads identically:** A regression test loads the reference `config.test.yml` and asserts field values before and after the migration match.
7. **`RagConfig` is unified:** `grep -r "struct RagConfig" crates/` returns exactly one result.

---

## 9. Task Breakdown

### Task 1.1 — Audit and Map Configuration Domains
**Goal:** Produce a written mapping before touching any code.

Steps:
1. Read `src/config/application_settings.rs` from top to bottom.
2. Create a document (or code comments) listing: every struct name → its target file → which crate it belongs in.
3. Note every place where env var override logic lives in `Settings::get_*_config()` methods — these need to move into `apply_env_overrides()` on each struct.
4. Identify every `use` path in the 29 consumer files and group them by which struct they import.

**Definition of done:** You can answer "where does `GarrisonSettings` go?" without looking it up.

---

### Task 1.2 — Create the `EnvOverridable` Trait and `read_env` Helper
**Goal:** Have the trait and helper function available before any struct moves.

Steps:
1. Decide where to place the trait (recommendation: `src/config/env_utils.rs` in the facade crate, since all current env-override logic lives in the facade).
2. Create the file with the `EnvOverridable` trait and `read_env::<T>` function.
3. Write unit tests covering all primitive types (`String`, `u16`, `u64`, `bool`).
4. Add `pub mod env_utils;` to `src/config/mod.rs`.
5. Run `cargo test` — all tests must pass.

**Definition of done:** `cargo test config::env_utils` passes.

---

### Task 1.3 — Extract Per-Domain Config Modules (one domain at a time)

**For each domain, follow this exact sequence:**

1. Create the target file (e.g., `crates/paladin-memory/src/config/garrison.rs`).
2. Copy the struct and all its `impl` blocks to the new file.
3. Move the env-var override logic from `Settings::get_garrison_config()` into `GarrisonSettings::apply_env_overrides()` implementing the `EnvOverridable` trait.
4. In `application_settings.rs`, replace the struct definition with a `pub use` re-export pointing to the new location.
5. Run `cargo build` — it must compile with zero errors.
6. Move the struct's tests from `application_settings.rs` to the new file.
7. Run `cargo test` — all tests must pass.
8. Repeat for the next domain.

**Recommended order** (least to most risk):
1. `HeraldConfig` group (`JsonHeraldConfig`, `MarkdownHeraldConfig`, `TableHeraldConfig`, `HeraldConfig`) → `src/config/herald.rs`
2. `SchedulerConfig` → `src/config/scheduler.rs`
3. `CitadelConfig` → `src/config/citadel.rs`
4. `ArsenalConfig`, `MCPServerConfig` → `src/config/arsenal.rs`
5. `QueueConfig` → `src/config/queue.rs`
6. `FileStorageConfig` → `src/config/file_storage.rs`
7. `NotificationConfig` → `src/config/notifications.rs`
8. `ServerConfig`, `SourceConfig`, `MessageServiceSettings` → `src/config/web_server.rs`
9. `VisionRetryConfig`, `VisionProviderConfig`, `VisionConfig` → `crates/paladin-llm/src/config/vision.rs`
10. `LlmProviderConfig`, `LlmConfig` → `crates/paladin-llm/src/config/llm.rs`
11. `GarrisonSettings` → `crates/paladin-memory/src/config/garrison.rs`
12. `QdrantSanctumConfig`, `SanctumConfig` → `crates/paladin-memory/src/config/sanctum.rs`
13. `RagConfig` (consolidate with existing), `MemoryExtractionConfig` → `crates/paladin-memory/src/config/rag.rs`

**Definition of done:** All structs are in their new files, `application_settings.rs` contains only re-exports and the `Settings` struct itself.

---

### Task 1.4 — Migrate Consumers and Delete `application_settings.rs`

Steps:
1. For each of the 29 consumer files, update `use` paths from `crate::config::application_settings::XxxConfig` to the new module paths.
2. Update `src/config/mod.rs` to re-export all config types from their new locations (so `use paladin::config::XxxConfig` still works from outside the crate).
3. Remove all `pub use` re-exports from `application_settings.rs`.
4. Delete `application_settings.rs`.
5. Remove `pub mod application_settings;` from `src/config/mod.rs`.
6. Run `cargo build --workspace` — must compile cleanly.
7. Run `cargo test --workspace` — all tests must pass.
8. Run `cargo clippy --workspace -- -D warnings` — must be clean.
9. Run `cargo fmt --all -- --check` — must be clean.

**Special attention for `src/config/user_config.rs`:** This file has 15+ references to `application_settings` structs constructed inline. Update all references and verify the `UserServiceFactory` still produces correct defaults.

**Definition of done:** `grep -r "application_settings" src/` returns zero results.

---

## 10. Open Questions

1. **`EnvOverridable` trait location:** If any `paladin-memory` or `paladin-llm` config struct needs env-var overrides, the trait must be accessible from those crates. Should it live in `paladin-core`? Currently, env override logic only appears in `Settings::get_*_config()` in the facade crate, so placing the trait in `src/config/env_utils.rs` may be sufficient. Confirm before starting Task 1.2.

2. **`RagConfig` field parity:** The `RagConfig` in `application_settings.rs` and the one in `paladin-memory/src/services/rag_retrieval_service.rs` may have diverged. Before deleting the application_settings version, diff the two structs field-by-field and ensure the `paladin-memory` version covers all fields used in config loading.

3. **`settings` snapshot test:** The acceptance criteria calls for a snapshot test loading `config.test.yml` and asserting field equality. Decide whether to use `insta` (snapshot testing crate) or plain `assert_eq!` assertions. The `insta` approach is more maintainable but requires adding a dev-dependency.

4. **`paladin-memory` crate's `Cargo.toml`:** Adding a `config` module with `serde` derives may require `serde` to be a dependency of `paladin-memory`. Verify it is already a dependency before creating the new files.

---

## Relevant Files

| File | Role |
|------|------|
| `src/config/application_settings.rs` | The monolithic file being decomposed (deleted at end) |
| `src/config/mod.rs` | Becomes the root `Settings` struct + re-exports |
| `src/config/env_utils.rs` | New — `EnvOverridable` trait and `read_env` helper |
| `src/config/herald.rs` | New — Herald formatter configuration |
| `src/config/arsenal.rs` | New — Arsenal/MCP tool configuration |
| `src/config/citadel.rs` | New — Citadel state persistence configuration |
| `src/config/file_storage.rs` | New — MinIO/S3 file storage configuration |
| `src/config/notifications.rs` | New — Notification channel configuration |
| `src/config/queue.rs` | New — Redis queue configuration |
| `src/config/scheduler.rs` | New — Scheduler configuration |
| `src/config/web_server.rs` | New — Web server / API configuration |
| `crates/paladin-llm/src/config/mod.rs` | New — LLM crate config module |
| `crates/paladin-llm/src/config/llm.rs` | New — LLM provider configuration |
| `crates/paladin-llm/src/config/vision.rs` | New — Vision model configuration |
| `crates/paladin-memory/src/config/mod.rs` | New — Memory crate config module |
| `crates/paladin-memory/src/config/garrison.rs` | New — Garrison memory configuration |
| `crates/paladin-memory/src/config/sanctum.rs` | New — Sanctum vector store configuration |
| `crates/paladin-memory/src/config/rag.rs` | New — RAG and memory extraction configuration |
| `src/config/user_config.rs` | Existing — highest-risk consumer, update last |
| `src/infrastructure/adapters/llm/config_bridge.rs` | Existing — imports LLM config types, update in Task 1.4 |
| `config.test.yml` | Reference config used for regression testing |
