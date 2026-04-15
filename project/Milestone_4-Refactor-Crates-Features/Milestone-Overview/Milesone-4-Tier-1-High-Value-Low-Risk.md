# Milestone 1: High-Value, Low-Risk Foundations

**Project:** Paladin Framework Refactoring Initiative
**Milestone:** Tier 1 — High-Value, Low-Risk Improvements
**Status:** Planning
**Target Start:** TBD
**Target Completion:** TBD
**Document Version:** 1.0
**Last Updated:** 2026-04-14

---

## Executive Summary

Milestone 1 addresses the highest-impact, lowest-risk improvements identified in the Paladin refactoring analysis. The current codebase is a single Cargo crate of approximately 102,000 lines across 200+ source files. While the internal hexagonal architecture is well-enforced at the module level, the monolithic crate structure creates unnecessary compilation overhead, an unbounded public API surface, and tight coupling between optional subsystems.

This milestone delivers three foundational changes — expanded feature flags, hardened port trait contracts, and CLI isolation — that provide immediate compile-time and API-surface benefits without requiring a workspace refactor. These changes are fully backward-compatible and establish the architectural prerequisites for the Tier 2 workspace decomposition.

### Success Criteria

- All existing tests (1,487+) continue to pass after each Epic is completed.
- A downstream consumer using only agent orchestration no longer compiles Redis, MinIO, notification, web server, or content processing dependencies.
- The `paladin-cli` binary and its 8+ CLI-only dependencies are excluded from library compilation.
- Port traits are documented as the stable public API contract with internal types marked `pub(crate)` where appropriate.
- Zero regressions in `cargo clippy -- -D warnings` and `cargo fmt --check`.
- Incremental build time for infrastructure adapter changes measurably decreases (target: ≥30% reduction in affected-module recompilation).

---

## Milestone Scope & Boundaries

### In Scope

- Expansion of Cargo feature flags across all optional subsystems.
- Visibility hardening of port traits and internal types.
- CLI extraction from the library compilation path.
- Documentation updates for the new feature flag surface.
- Migration guide for downstream consumers.

### Out of Scope

- Cargo workspace decomposition (Tier 2).
- Crate extraction of `paladin-core` or `paladin-battalion` (Tier 2).
- Splitting `application_settings.rs` (Tier 3).
- Moving manager-layer services to the application layer (Tier 3).
- Relocating the Maneuver DSL or `CircuitBreaker` (Tier 3).

### Dependencies & Assumptions

- The existing test suite is green on `main` before work begins.
- CI pipeline supports feature-flag matrix testing (or will be configured as part of Epic 1).
- No concurrent large-scale refactors are in flight on overlapping modules.

### Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Feature flag expansion introduces conditional compilation bugs | Medium | High | Comprehensive CI matrix testing all flag combinations; incremental rollout per subsystem |
| Downstream consumers depend on currently-public internal types | Medium | Medium | Audit `pub use` re-exports before restricting visibility; provide deprecation warnings before breaking changes |
| CLI separation breaks the `paladin-cli` binary entry point | Low | High | Existing `[[bin]]` target and `src/bin/paladin-cli.rs` already separate; test binary builds in CI |
| Feature-gated code introduces dead-code warnings | Low | Low | Use `#[cfg]` consistently; suppress warnings only where architecturally justified |

---

## Epic 1: Expand Feature Flags to Gate the Full Optional Surface

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Large
**Dependencies:** None (first Epic in sequence)

### Objective

Transform the current five thin feature flags (`redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `integration-tests`) into a comprehensive feature-flag surface that allows downstream consumers to compile only the subsystems they need. A user who only requires agent orchestration should not pay the compile cost of Redis, MinIO, notification adapters, the web server, content processing, or vision pipelines.

### Background & Rationale

The current `Cargo.toml` includes approximately 60 direct dependencies, but only five are gated behind feature flags. Heavy dependencies like `actix-web`, `lettre`, `pdf-extract`, `scraper`, `tiktoken-rs`, and all three LLM provider HTTP clients (`reqwest` configurations for OpenAI, Anthropic, DeepSeek) compile unconditionally. This inflates build times and binary size for every consumer regardless of their actual usage.

The existing feature flag pattern for `redis` and `rust-s3` is correct and well-implemented. This Epic extends that pattern to cover the remaining optional surface.

### Acceptance Criteria

1. Each of the following subsystems is individually gatable via a Cargo feature flag:
   - **LLM Providers:** `llm-openai`, `llm-anthropic`, `llm-deepseek` (and a convenience `llm-all` flag).
   - **Content Processing:** `content-processing` (gates `pdf-extract`, `scraper`, `tiktoken-rs`, `rss`).
   - **Vision Pipeline:** `vision` (gates vision-related adapters and the Sentinel Vision System).
   - **Web Server:** `web-server` (gates `actix-web` and all web/API infrastructure).
   - **Notification Adapters:** `notifications` (gates `lettre` and notification publisher adapters).
   - **MCP Transports:** `mcp-stdio`, `mcp-sse` (or combined `mcp-transports`) — gates the concrete MCP transport adapters (`MCPStdioAdapter`, `MCPSseAdapter`) and their infrastructure dependencies. The Arsenal domain types (`Armament`, `ArmamentCall`, `ArsenalPort`, `ArsenalRegistry`) and application services (`ArsenalExecutionService`, `ArsenalRegistryService`) remain unconditionally compiled as core framework components.
   - **Existing flags retained:** `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`.
2. A new `full` convenience feature enables all optional features.
3. The `default` feature set is revised to include only the minimal viable surface for agent orchestration (core + battalion + one LLM provider).
4. All `#[cfg(feature = "...")]` guards are applied to module declarations, `use` statements, port implementations, and adapter registrations.
5. `cargo build --no-default-features` compiles successfully with only core domain types and port trait definitions.
6. `cargo build --all-features` compiles successfully and all tests pass.
7. CI is updated to test at minimum: `--no-default-features`, default features, `--all-features`, and each individual flag in isolation.
8. No dead-code warnings are introduced under any tested flag combination.

### Tasks

#### Task 1.1: Audit and Classify All Dependencies

**Description:** Review every entry in `[dependencies]` and classify it as either "core" (always required) or "optional" (gated by a specific feature). Produce a dependency classification matrix mapping each dependency to its feature flag.

**Deliverables:**
- Dependency classification matrix document (Markdown table).
- Draft `Cargo.toml` diff showing all proposed `optional = true` additions and `[features]` entries.

**Estimated Effort:** Small

#### Task 1.2: Implement LLM Provider Feature Flags

**Description:** Gate the OpenAI, Anthropic, and DeepSeek adapter modules behind `llm-openai`, `llm-anthropic`, and `llm-deepseek` feature flags respectively. The `LlmPort` trait itself remains unconditionally compiled as it is part of the port contract. Only the concrete adapter implementations and their provider-specific HTTP dependencies are gated.

**Deliverables:**
- Updated `Cargo.toml` with three new LLM feature flags and an `llm-all` convenience flag.
- `#[cfg(feature = "...")]` guards on adapter module declarations and registration code.
- Updated adapter factory/builder to handle unavailable providers gracefully (compile-time exclusion, not runtime error).
- Tests for each provider compiling independently.

**Estimated Effort:** Medium

#### Task 1.3: Implement Content Processing Feature Flag

**Description:** Gate the content processing pipeline — document parsing, RSS ingestion, web scraping, and token counting — behind a `content-processing` feature flag. This includes `pdf-extract`, `scraper`, `tiktoken-rs`, and `rss` dependencies.

**Deliverables:**
- Updated `Cargo.toml` gating content processing dependencies.
- `#[cfg]` guards on content processing modules, use-case services, and any port implementations that depend on gated types.
- Verification that agent orchestration builds cleanly without content processing.

**Estimated Effort:** Medium

#### Task 1.4: Implement Web Server Feature Flag

**Description:** Gate the `actix-web` dependency and the entire `infrastructure::web` module behind a `web-server` feature flag. The web server is not required for library usage or CLI operation.

**Deliverables:**
- Updated `Cargo.toml` with `web-server` feature flag.
- `#[cfg]` guards on web module, middleware, route handlers, and WebSocket infrastructure.
- Verification that library and CLI builds succeed without `web-server`.

**Estimated Effort:** Medium

#### Task 1.5: Implement Notification, Vision, and MCP Transport Feature Flags

**Description:** Gate notification adapters (`lettre`, email/SMS/push publishers) behind `notifications` and vision pipeline adapters behind `vision`. Gate the concrete MCP transport adapters behind `mcp-transports` (or split as `mcp-stdio` / `mcp-sse`).

**Important distinction:** The Arsenal domain types (`Armament`, `ArmamentCall`, `ArmamentResult`, `ArsenalError`), port traits (`ArsenalPort`, `ArsenalRegistry`), and application services (`ArsenalExecutionService`, `ArsenalRegistryService`) are **not gated** — they are core framework components used pervasively by `PaladinBuilder`, `PaladinExecutionService`, all battalion execution services, and the CLI. The Arsenal is the agent's toolkit abstraction (tools, skills, prompts, instructions, and future extensions beyond MCP), not merely an MCP integration layer. Gating it would require `#[cfg]` guards across every execution path in the framework for negligible dependency savings (the Arsenal core depends only on `serde`, `serde_json`, `uuid`, and `async-trait`). Only the **infrastructure transport adapters** that connect to external MCP servers via subprocess (STDIO) or HTTP (SSE) — and their dependencies (`tokio::process`, protocol serialization, HTTP client configuration) — are gated.

**Deliverables:**
- Three new feature flags: `notifications`, `vision`, `mcp-transports`.
- `#[cfg]` guards on notification adapters, vision adapters, and MCP transport adapter modules.
- Arsenal domain types, port traits, and application services compile unconditionally.
- Dependency isolation verified for each flag independently.

**Estimated Effort:** Medium

#### Task 1.6: Revise Default Feature Set and Add Convenience Flags

**Description:** Redefine the `default` feature set to reflect the minimum viable surface for the primary use case (multi-agent orchestration). Add a `full` convenience flag that enables everything.

**Current default:** `["redis-queue", "s3-storage", "openai-embeddings"]`

**Proposed default:** `["llm-openai"]` (or a curated minimal set determined during Task 1.1 analysis).

**Deliverables:**
- Updated `[features]` section with revised `default` and new `full` flag.
- Migration guide documenting the default change and how existing users should update their `Cargo.toml` dependency declarations.
- `CHANGELOG.md` entry documenting the breaking change in default features.

**Estimated Effort:** Small

#### Task 1.7: CI Feature Matrix Configuration

**Description:** Update the CI pipeline to test feature flag combinations systematically. At minimum: `--no-default-features`, default, `--all-features`, and each new flag in isolation.

**Deliverables:**
- CI configuration file updates (GitHub Actions or equivalent).
- Matrix build definition covering the required flag combinations.
- Verification that all matrix entries pass.

**Estimated Effort:** Small

---

## Epic 2: Harden Port Traits as the Stable Public API Contract

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Can proceed in parallel with Epic 1

### Objective

Establish the ~20 port traits in `src/application/ports/` (6,000 LOC) as the explicit, documented, stable public API of the Paladin framework. Restrict visibility of internal implementation types that are currently exposed through the glob re-export in `src/lib.rs`, protecting downstream consumers from coupling to implementation details.

### Background & Rationale

The current `src/lib.rs` performs a glob re-export of every internal module:

```rust
pub use application::*;
pub use config::*;
pub use core::*;
pub use infrastructure::*;
```

This makes the entire internal module graph publicly accessible. While convenient during early development, it means any internal type rename, module reorganization, or implementation change is a potential breaking change for downstream consumers. The port traits — `LlmPort`, `GarrisonPort`, `SanctumPort`, `EmbeddingPort`, `QueuePort`, `NotificationPort`, `ArsenalPort`, etc. — are the architecturally correct public surface and should be explicitly designated as such.

### Acceptance Criteria

1. `src/lib.rs` replaces glob re-exports with explicit, curated `pub use` statements for port traits, domain entities needed by consumers, and builder/configuration types.
2. Internal implementation types (adapter internals, repository details, CLI modules, manager services) are not re-exported from the crate root.
3. All port traits have complete `rustdoc` documentation including usage examples, error semantics, and implementor guidance.
4. A `STABLE_API.md` document catalogs every publicly exported type and trait with its stability guarantee.
5. All existing tests pass without modification (or with minimal import path adjustments in integration tests).
6. `cargo doc --no-deps` generates clean documentation with no broken intra-doc links.

### Tasks

#### Task 2.1: Audit Current Public API Surface

**Description:** Enumerate every type, trait, function, and module currently accessible through the glob re-exports. Classify each as "public API" (should remain exported), "internal" (should be restricted), or "needs discussion" (ambiguous).

**Deliverables:**
- Public API audit spreadsheet or Markdown document.
- Classification of all ~200+ exported items.
- List of types that downstream consumers or examples currently reference directly.

**Estimated Effort:** Medium

#### Task 2.2: Replace Glob Re-Exports with Curated Exports

**Description:** Rewrite `src/lib.rs` to export only the curated public API surface. Use `pub(crate)` or `pub(super)` on internal modules and types that should not be consumer-facing.

**Deliverables:**
- Refactored `src/lib.rs` with explicit exports.
- Updated visibility modifiers on internal modules.
- All existing examples and integration tests updated to use the new import paths.
- No public API regression for types classified as "public API" in Task 2.1.

**Estimated Effort:** Medium

#### Task 2.3: Document Port Traits as Stable API

**Description:** Add comprehensive `rustdoc` to all port traits. Each trait should document its purpose, expected behavior contract, error semantics, thread-safety guarantees, and provide at least one usage example.

**Deliverables:**
- Complete `rustdoc` on all ~20 port traits.
- `cargo doc --no-deps` builds cleanly with no warnings.
- At least one `/// # Examples` block per port trait.

**Estimated Effort:** Medium

#### Task 2.4: Create STABLE_API.md Reference Document

**Description:** Produce a developer-facing reference document that catalogs the public API surface, versioning policy, and stability guarantees. This serves as the contract for downstream consumers.

**Deliverables:**
- `docs/STABLE_API.md` listing all public types, traits, and functions.
- Stability tier classification (Stable, Unstable/Experimental, Deprecated).
- Versioning policy statement (what constitutes a breaking change).

**Estimated Effort:** Small

---

## Epic 3: Isolate the CLI from the Library Compilation Path

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Benefits from Epic 1 (feature flags) but can proceed independently

### Objective

Ensure that the 12,000 lines of CLI code and its 8+ CLI-only dependencies (`clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored`, `console`, `serde_yaml` for CLI configs, and related crates) are not compiled when Paladin is used as a library dependency. The CLI should compile only when building the `paladin-cli` binary target.

### Background & Rationale

The current codebase embeds the CLI in `src/application/cli/` (12,000 LOC) with 193 dedicated tests. A `[[bin]] paladin-cli` target already exists in `Cargo.toml` pointing to `src/bin/paladin-cli.rs`. However, because the CLI modules are part of the main `lib` target and their dependencies are unconditional in `[dependencies]`, any consumer who adds `paladin` as a library dependency pays the compile cost of the entire CLI stack.

The CLI has already been consolidated into `src/application/cli/` following hexagonal architecture principles (completed in a prior Epic). This clean separation makes isolation straightforward.

### Acceptance Criteria

1. `cargo build --lib` does not compile any CLI modules or their exclusive dependencies.
2. `cargo build --bin paladin-cli` compiles the CLI and links against the library.
3. CLI-only dependencies (`clap`, `dialoguer`, `indicatif`, `comfy-table`, `colored`, `console`) are gated behind a `cli` feature flag or moved to binary-only scope.
4. All 193 CLI tests continue to pass when the `cli` feature is enabled.
5. Library consumers see no change to available types or functionality.
6. The `paladin` binary target (`src/main.rs`) continues to function correctly.
7. Measured reduction in library-only build time and dependency count.

### Tasks

#### Task 3.1: Analyze CLI Dependency Boundaries

**Description:** Map which dependencies are used exclusively by CLI code and which are shared with library functionality. Determine the cleanest boundary for gating.

**Deliverables:**
- CLI dependency analysis document listing each dependency and its usage scope.
- Recommendation on gating strategy: feature flag (`cli`), binary-only dependencies, or workspace crate.
- Impact assessment on `src/main.rs` (the `paladin` binary).

**Estimated Effort:** Small

#### Task 3.2: Gate CLI Module Behind Feature Flag

**Description:** Wrap the `src/application/cli/` module tree behind a `cli` feature flag. Add `#[cfg(feature = "cli")]` to the module declaration and all downstream references. Gate CLI-only dependencies with `optional = true`.

**Deliverables:**
- Updated `Cargo.toml` with `cli` feature flag and optional CLI dependencies.
- `#[cfg(feature = "cli")]` guards on `application::cli` module and its sub-modules.
- Updated `src/lib.rs` exports to conditionally include CLI types.
- Binary targets updated to enable the `cli` feature.

**Estimated Effort:** Medium

#### Task 3.3: Update Binary Entry Points

**Description:** Ensure both `src/main.rs` and `src/bin/paladin-cli.rs` correctly depend on the `cli` feature and compile the CLI stack only through the binary build path.

**Deliverables:**
- Updated binary source files with correct feature-gated imports.
- Verified `cargo build --bin paladin` and `cargo build --bin paladin-cli` both succeed.
- Verified `cargo build --lib --no-default-features` excludes CLI code.

**Estimated Effort:** Small

#### Task 3.4: Migrate CLI Tests to Feature-Gated Compilation

**Description:** Ensure all 193 CLI tests are gated behind `#[cfg(feature = "cli")]` so they don't fail or produce dead-code warnings when the CLI feature is disabled.

**Deliverables:**
- All CLI test modules wrapped with `#[cfg(feature = "cli")]`.
- CI updated to run CLI tests with the `cli` feature enabled.
- Snapshot test files (`.snap`) remain committed and functional.

**Estimated Effort:** Small

#### Task 3.5: Measure and Document Build Time Impact

**Description:** Benchmark library-only build times before and after CLI isolation. Document the dependency count reduction and compile-time improvement.

**Deliverables:**
- Before/after build time measurements (clean build and incremental).
- Before/after dependency tree comparison (`cargo tree` output).
- Results documented in the milestone completion report.

**Estimated Effort:** Small

---

## Cross-Epic Deliverables

### Migration Guide

A single migration guide covering all three Epics, documenting:

- New feature flags and their contents.
- Changes to the default feature set.
- Updated import paths for any types affected by the visibility changes.
- How to enable the CLI feature for binary builds.
- Examples of minimal `Cargo.toml` dependency declarations for common use cases.

### CHANGELOG Updates

Each Epic produces a `CHANGELOG.md` entry following the project's existing Keep a Changelog format with conventional commit references.

### Documentation Updates

- `README.md` updated with the new feature flag table.
- `CONTRIBUTING.md` updated with feature flag testing requirements.
- `cargo doc` output verified clean after all Epics complete.

---

## Milestone Schedule Overview

| Phase | Epic | Estimated Duration | Predecessors |
|-------|------|--------------------|--------------|
| Phase 1A | Epic 1: Feature Flag Expansion | 2–3 sprints | None |
| Phase 1B | Epic 2: Port Trait Hardening | 1–2 sprints | None (parallel with Epic 1) |
| Phase 2 | Epic 3: CLI Isolation | 1–2 sprints | Benefits from Epic 1 |
| Wrap-up | Cross-Epic Deliverables | 1 sprint | Epics 1, 2, 3 |

**Total Estimated Duration:** 3–5 sprints (depending on parallelization and team capacity)

---

## Completion Checklist

- [ ] All feature flags implemented and tested in CI matrix.
- [ ] `cargo build --no-default-features` succeeds.
- [ ] `cargo build --all-features` succeeds with all 1,487+ tests passing.
- [ ] Port traits fully documented with `rustdoc` examples.
- [ ] `src/lib.rs` uses curated exports instead of glob re-exports.
- [ ] `STABLE_API.md` published.
- [ ] CLI excluded from library compilation path.
- [ ] CLI tests pass under `cli` feature flag.
- [ ] Build time improvement measured and documented.
- [ ] Migration guide published.
- [ ] `CHANGELOG.md` updated for all changes.
- [ ] `README.md` updated with feature flag documentation.
- [ ] No clippy warnings under `cargo clippy -- -D warnings`.
- [ ] All code formatted with `cargo fmt`.
- [ ] Milestone retrospective completed and findings documented.

---

## Appendix A: Current Feature Flag State

```toml
[features]
default = ["redis-queue", "s3-storage", "openai-embeddings"]
redis-queue = ["redis"]
s3-storage = ["rust-s3"]
openai-embeddings = []
qdrant = ["qdrant-client"]
integration-tests = []
live-api-tests = []
```

## Appendix B: Proposed Feature Flag State (Post-Milestone)

```toml
[features]
default = ["llm-openai"]

# LLM Providers
llm-openai = []
llm-anthropic = []
llm-deepseek = []
llm-all = ["llm-openai", "llm-anthropic", "llm-deepseek"]

# Infrastructure
redis-queue = ["redis"]
s3-storage = ["rust-s3"]
web-server = ["actix-web"]
notifications = ["lettre"]

# AI & Processing
openai-embeddings = []
qdrant = ["qdrant-client"]
content-processing = ["pdf-extract", "scraper", "tiktoken-rs", "rss"]
vision = []

# MCP Transport Adapters (Arsenal domain types are always compiled)
mcp-transports = ["mcp-stdio", "mcp-sse"]
mcp-stdio = []
mcp-sse = []

# CLI (binary-only)
cli = ["clap", "dialoguer", "indicatif", "comfy-table", "colored", "console"]

# Convenience
full = [
    "llm-all", "redis-queue", "s3-storage", "web-server",
    "notifications", "openai-embeddings", "qdrant",
    "content-processing", "vision", "mcp-transports", "cli"
]

# Testing
integration-tests = []
live-api-tests = []
```

## Appendix C: Port Traits Inventory (Stable API Candidates)

| Port Trait | Module | Direction | Description |
|------------|--------|-----------|-------------|
| `LlmPort` | `ports::output::llm_port` | Output | LLM provider integration |
| `GarrisonPort` | `ports::output::garrison_port` | Output | Conversation memory persistence |
| `SanctumPort` | `ports::output::sanctum_port` | Output | Vector store / semantic memory |
| `EmbeddingPort` | `ports::output::embedding_port` | Output | Text embedding generation |
| `QueuePort` | `ports::output::queue_port` | Output | Distributed task queue |
| `NotificationPort` | `ports::output::notification_port` | Output | Notification publishing |
| `ArsenalPort` | `ports::output::arsenal_port` | Output | Tool execution (core — always compiled) |
| `LogPort` | `ports::output::log_port` | Output | Structured logging |
| `SearchPort` | `ports::output::search_port` | Output | Search engine integration |
| `FileStoragePort` | `ports::output::file_storage_port` | Output | Object/file storage |
| `ContentIngestionPort` | `ports::input::content_ingestion_port` | Input | Content fetching/ingestion |
| `RpcGatewayPort` | `ports::input::rpc_gateway_port` | Input | API gateway (REST/gRPC/GraphQL) |
| `MlPort` | `ports::input::ml_port` | Input | ML model integration |
| `NlpPort` | `ports::input::nlp_port` | Input | NLP service integration |

*Note: Exact trait names and module paths to be confirmed during Task 2.1 audit.*
