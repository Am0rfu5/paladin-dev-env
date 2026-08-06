
## Epic 1: Decompose `application_settings.rs` into Per-Domain Configuration Modules

> **See [ADR-0014](../../../.planning/decisions/0014-milestone-4-6-tier-numbering.md)** (dated
> 2026-08-06) for the corrected Milestone/Tier numbering this document's Milestone-numbering
> references predate. This document is a byte-equivalent copy of
> `.project/Milestone_6-Architectural-Refinements/overview/Milestone_6-Tier_3-Architectural-Refinements.md`,
> carrying no independent content beyond that source, which is corrected there. Not corrected
> inline here.

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** None (can begin immediately after Milestone 5)

### Objective

Replace the monolithic 3,172-line `src/config/application_settings.rs` file with a set of focused, per-domain configuration modules. Each subsystem — agent execution, garrison memory, arsenal tools, notifications, queue infrastructure, file storage, web server, LLM providers, and battalion orchestration — gets its own configuration struct in its own file. A root `Settings` struct composes them, preserving the existing deserialization contract with `config.yml` and environment variable overrides.

### Background & Rationale

A single file carrying all configuration types for every subsystem is a well-known Rust anti-pattern that compounds over time. The current `application_settings.rs` contains:

- `Settings` (root aggregate with 15+ optional fields)
- `QueueConfig` (Redis queue configuration with 10+ fields and env var override logic)
- `FileStorageConfig` (MinIO/S3 configuration with env var overrides)
- `NotificationConfig` (notification channel and template settings)
- `MCPServerConfig` (MCP Arsenal server definitions)
- `LlmProviderConfig` / `OpenAIConfig` / `AnthropicConfig` / `DeepSeekConfig`
- `GarrisonConfig` references and defaults
- `WebServerConfig`
- `LoggingConfig`
- Various helper methods for environment variable override logic

Every developer who needs to understand or modify any single subsystem's configuration must navigate a 3,172-line file. Configuration changes for unrelated subsystems create merge conflicts. The env var override methods are duplicated per-config-struct with identical patterns.

### Acceptance Criteria

1. `application_settings.rs` is replaced by a `config/` module directory with individual files:
   - `config/mod.rs` — Re-exports and the root `Settings` struct.
   - `config/agent.rs` — Paladin agent execution configuration.
   - `config/garrison.rs` — Garrison memory system configuration.
   - `config/arsenal.rs` — Arsenal and MCP server configuration.
   - `config/notifications.rs` — Notification channel and template configuration.
   - `config/queue.rs` — Redis queue configuration.
   - `config/file_storage.rs` — MinIO/S3 file storage configuration.
   - `config/web_server.rs` — Web server and API configuration.
   - `config/llm.rs` — LLM provider configurations (OpenAI, Anthropic, DeepSeek).
   - `config/battalion.rs` — Battalion orchestration defaults.
   - `config/logging.rs` — Logging and tracing configuration.
2. The root `Settings` struct remains the single entry point for config loading, composing all sub-configs via `#[serde(default)]` fields.
3. `Settings::new()`, `Settings::load_from_file()`, and all environment variable override methods produce identical results to the pre-refactor implementation.
4. A shared `env_override` utility function or trait eliminates the duplicated env var override pattern across config structs.
5. All existing config-related tests pass without behavioral changes.
6. `config.yml` deserialization is fully backward-compatible — no changes to the YAML schema.
7. No file in the `config/` module exceeds 400 lines.

### Tasks

#### Task 1.1: Audit and Map Configuration Domains

**Description:** Analyze `application_settings.rs` and produce a detailed map of every struct, field, method, and env var override. Classify each into its target domain module. Identify shared patterns (env var override logic) that should become utilities.

**Deliverables:**
- Configuration domain mapping document (struct → target file).
- Inventory of duplicated patterns to extract into shared utilities.
- Dependency graph showing which config structs reference others.

**Estimated Effort:** Small

#### Task 1.2: Create Shared Environment Override Utility

**Description:** Extract the repeated pattern of "read env var, parse, override field" into a reusable utility. The current code repeats this pattern approximately 30 times across `get_queue_config()`, `get_file_storage_config()`, and similar methods.

**Deliverables:**
- `config/env_utils.rs` (or similar) with generic env var override helpers.
- Helper functions or a trait for typed env var reading with fallback: `env_override::<T>(var_name, &mut field)`.
- Unit tests for the utility with various types (String, u16, u64, bool, Option).

**Estimated Effort:** Small

#### Task 1.3: Extract Per-Domain Configuration Modules

**Description:** Move each configuration struct and its associated methods into its designated file. Update the root `Settings` struct to compose sub-configs. Ensure `#[serde(default)]` and `#[serde(flatten)]` attributes preserve YAML compatibility.

**Deliverables:**
- Individual config files as listed in the acceptance criteria.
- Root `Settings` struct in `config/mod.rs` composing all sub-configs.
- All `use` paths updated throughout the codebase.
- `config.yml` deserialization verified identical via snapshot testing.

**Estimated Effort:** Medium

#### Task 1.4: Migrate Config Tests and Verify Backward Compatibility

**Description:** Migrate all existing configuration tests to the new module structure. Add regression tests that load a known `config.yml` and assert field-by-field equality against expected values. Verify all environment variable overrides work correctly.

**Deliverables:**
- Migrated config tests passing in new locations.
- New regression test loading a reference `config.yml` and asserting all fields.
- Env var override integration tests for each config domain.
- No behavioral changes detected.

**Estimated Effort:** Small

---
