# Product Requirements Document: Expand Feature Flags to Gate the Full Optional Surface

> **Correction (dated 2026-08-06, ARCH-05):** This PRD's FR1 and Design Considerations describe
> three feature-flag behaviours the shipped `Cargo.toml` manifest contradicts — the `vision`
> gating of the encryption dependencies, the MCP transport flags, and the `web-server` gating of
> `actix-web`. Applying the `vision` clause literally would break
> `cargo build --no-default-features` for user auth and Citadel encryption. See the Milestone 4-6
> ledger's `diverged` rows and
> [`.planning/decisions/0011-vision-port-surfaces.md`](../../../.planning/decisions/0011-vision-port-surfaces.md)
> for the prior decision that dispositioned this same encryption code. Original text is retained
> below with inline corrections — nothing is deleted.

**Created:** April 14, 2026
**Epic:** Epic 1 (Paladin Milestone 4)
**Document Version:** 1.1

---

## Executive Summary

The Paladin framework currently compiles approximately 60 direct dependencies unconditionally, including heavy optional systems like Redis, MinIO, web server (Actix), email/notifications (Lettre), content processing (PDF extraction, web scraping), and multiple LLM provider clients. A downstream consumer using Paladin only for multi-agent orchestration should not pay the compile-time and binary-size cost of these tools.

This PRD defines the expansion of Cargo feature flags from the current 5 thin flags to a comprehensive gating surface that allows consumers to compile **only what they use**. The implementation is phased by subsystem to minimize integration risk and allow incremental testing.

**NOTE (2026-04-15):** The originally planned `mcp-arsenal` feature flag (Task 7.0) has been **eliminated from scope**. Arsenal and its MCP transport adapters will remain unconditionally compiled as core framework components. The complexity of gating the Arsenal subsystem was deemed unnecessary given its pervasive use throughout the framework and minimal dependency overhead (pure Rust implementation).

---

## Problem Statement

**Current State:**
- Only 5 feature flags exist: `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `integration-tests`
- Essential infrastructure dependencies (~60 total) compile unconditionally
- Heavy subsystems (web server, notifications, content processing, vision) are mandatory even for library-only usage
- Build times and binary sizes are larger than necessary for pure orchestration use cases

**Impact:**
- A developer writing a Rust CLI that uses Paladin agents spends 30-50% of build time compiling unused subsystems
- Embedded or resource-constrained deployments pay unnecessary binary size overhead
- Users cannot opt-out of security audit burden for unused dependencies

**Solution Direction:**
Implement a comprehensive feature-flag matrix allowing granular opt-in of subsystems. Consumers who need only agent orchestration will compile core + one LLM provider (~40 dependencies, not ~60).

**Key Heavy Dependencies Identified (from Cargo.toml):**

| Dependency | Category | Proposed Flag |
|---|---|---|
| `actix-web` | HTTP server framework | `web-server` |
| `axum` | HTTP server framework (also present) | `web-server` |
| `lettre` | Email/SMTP transport | `notifications` |
| `pdf-extract` | PDF parsing | `content-processing` |
| `scraper` | HTML/web scraping | `content-processing` |
| `tiktoken-rs` | Token counting | `content-processing` |
| `rss` | RSS feed parsing | `content-processing` |
| `reqwest` | HTTP client (shared by all LLM adapters) | Core (stays unconditional; adapter code is gated) |
| `chacha20poly1305` | Encryption (Sentinel Vision) | `vision` |
| `zeroize` | Secure memory zeroing (Sentinel Vision) | `vision` |
| `structopt` | Legacy CLI arg parsing | `cli` (Epic 3) or remove |
| `clap` | CLI arg parsing | `cli` (Epic 3) |
| `dialoguer`/`indicatif`/`colored`/`console`/`comfy-table` | CLI UI | `cli` (Epic 3) |
| `redis` | Redis client | `redis-queue` (existing) |
| `rust-s3` | S3/MinIO client | `s3-storage` (existing) |
| `qdrant-client` | Vector DB client | `qdrant` (existing) |

---

## Goals

1. **Reduce Compile Time:** Enable a 30%+ faster incremental build for agents-only use cases
2. **Modular Compilation:** Eliminate mandatory compilation of Redis, MinIO, web server, and notifications for library consumers
3. **Clear API Contract:** Establish feature-gated subsystems as either "always included" (core, ports) or "optional" (adapters, integrations)
4. **Backward Compatibility Path:** Break the existing default feature set cleanly; provide migration guide for affected users
5. **CI Verification:** Test feature combinations systematically to prevent "works with all features, breaks with subset" bugs

---

## User Stories

### User Story 1: Agent Orchestration Library Consumer
> As a developer building a Rust CLI tool that uses Paladin agents with a single LLM provider (OpenAI), I want to compile only the core agent orchestration and OpenAI adapter, so my build times are fast and my binary is lean.

**Acceptance Criteria:**
- I can add `paladin = { version = "...", features = ["llm-openai"] }` to my `Cargo.toml`
- `cargo build` compiles in <2 minutes (vs. current ~3+ minutes)
- My binary excludes Redis, MinIO, web server, and all other unused subsystems
- Zero compilation errors or warnings

### User Story 2: Full-Featured Web Application
> As a developer building a full-featured web application with Paladin agents, persistent storage, content processing, and an HTTP API, I want a simple way to enable all optional features, so I don't need to enumerate each flag individually.

**Acceptance Criteria:**
- A `full` feature flag exists that enables all optional subsystems
- I can use `paladin = { version = "...", features = ["full"] }` to get the production-ready experience
- All integration tests pass with `--all-features`

### User Story 3: Legacy Code Migration
> As the maintainer of an existing application that was using Paladin with the old default features (`redis-queue`, `s3-storage`, `openai-embeddings`), I want a clear migration guide so my application continues to build without surprises.

**Acceptance Criteria:**
- A migration guide in `CHANGELOG.md` explains the change
- The guide provides the exact feature flags to use to restore old behavior
- Existing examples are updated to use the new defaults or explicitly specify old features

---

## Functional Requirements

### FR1: Feature Flag Structure

The following feature flags **must** be implemented:

#### LLM Providers (Individual + Convenience)
- `llm-openai` — Gates OpenAI adapter and `reqwest`-based HTTP client
- `llm-anthropic` — Gates Anthropic adapter and HTTP client
- `llm-deepseek` — Gates DeepSeek adapter and HTTP client
- `llm-all` — Convenience flag enabling all three LLM providers

**Behavior:** The `LlmPort` trait is always compiled. Only concrete adapter implementations and provider-specific dependencies are gated.

#### Subsystem Flags
- `content-processing` — Gates document parsing, web scraping, token counting (pdf-extract, scraper, tiktoken-rs, rss)
- ~~`web-server` — Gates both `actix-web` and `axum` frameworks and all HTTP/API infrastructure (both are present in Cargo.toml)~~
  **Corrected (dated 2026-08-06, diverged from shipped code):** Shipped `Cargo.toml:276`
  declares `web-server = ["dep:paladin-web", "dep:axum"]` — `actix-web` is not a root dependency,
  confirmed by `grep -rn actix Cargo.toml` returning no output during this task. The `web-server`
  feature gates the `paladin-web` crate and `axum` only.
- `notifications` — Gates Lettre and notification publisher adapters
- ~~`vision` — Gates vision pipeline, vision adapters (`openai_vision.rs`, `anthropic_vision.rs`), Sentinel Vision encryption deps (`chacha20poly1305`, `zeroize`), and `VisionPort`/`VisionCapableLlm` trait implementations~~
  **Corrected (dated 2026-08-06, diverged from shipped code):** Shipped `Cargo.toml:274` declares
  `vision = []` — an empty feature that gates **no dependency**. `chacha20poly1305`
  (`Cargo.toml:134`) and `zeroize` (`Cargo.toml:135`) are unconditional root dependencies,
  confirmed by `grep -rn 'chacha20poly1305\|zeroize' Cargo.toml crates/*/Cargo.toml` during this
  task — they serve `src/infrastructure/security/encryption.rs`'s general encryption for user
  auth and Citadel state, not only the vision pipeline. The Epic 1
  [`dependency-matrix.md`](dependency-matrix.md) audit in this same directory classified both
  correctly at the time; this clause did not. Applying this clause literally would break
  `cargo build --no-default-features` for user auth and Citadel. Cross-reference
  [ADR-0011](../../../.planning/decisions/0011-vision-port-surfaces.md), which dispositioned this
  same encryption code.
- ~~`mcp-arsenal`~~ — **ELIMINATED:** Arsenal remains unconditionally compiled
  **Confirmed (dated 2026-08-06, ARCH-05):** No MCP feature flag of any kind exists in the
  shipped manifest — `grep -n mcp Cargo.toml` returns no output, re-run during this task. This
  document's own 2026-04-15 elimination note above (line 15) is what shipped: Arsenal and its MCP
  transport adapters compile unconditionally as core framework components. This is the citation
  the Milestone 4 overview's MCP correction and the Milestone 4-6 ledger's `diverged` rows point
  at.

#### Existing Flags (Retained)
- `redis-queue` — Existing, unchanged
- `s3-storage` — Existing, unchanged
- `openai-embeddings` — Existing, unchanged
- `qdrant` — Existing, unchanged
- `integration-tests` — Existing, unchanged

#### Convenience Flags
- `full` — Enables all optional features: all LLM providers, content-processing, web-server, notifications, vision, redis-queue, s3-storage, openai-embeddings, qdrant

### FR2: Default Feature Set

**Old Default:** `["redis-queue", "s3-storage", "openai-embeddings"]`

**New Default:** `["llm-openai"]`

**Rationale:** A new user should be able to build a simple multi-agent orchestration tool immediately. Redis, S3, and embeddings are advanced features and should be opt-in.

### FR3: Conditional Compilation Guards

All feature-gated code **must** use `#[cfg(feature = "...")]` guards at:
1. Module declarations (e.g., `#[cfg(feature = "web-server")] pub mod web;`)
2. `use` statements for imports from gated modules
3. Port implementation registrations and adapter instantiations
4. Test modules that depend on gated features

**Example Pattern:**
```rust
#[cfg(feature = "llm-openai")]
pub mod openai_adapter;

#[cfg(feature = "llm-openai")]
use crate::infrastructure::adapters::llm::openai_adapter::OpenaiLlmAdapter;
```

### FR4: Graceful Degradation (Scope Clarification)

Adapters that are compile-time gated should **not** raise runtime errors for missing providers. If a user tries to instantiate an unavailable adapter at runtime:
- The code will not compile if the adapter is gated and the feature is disabled
- No need for dynamic runtime detection (compile-time safety is preferred)

### FR5: Documentation & Migration

**Deliverables:**
- Update `/docs/CONFIGURATION.md` to document all feature flags
- Create `/docs/FEATURE_FLAGS.md` with detailed explanation of each flag, use cases, and examples
- Add `/docs/MIGRATION.md` explaining the breaking change and how to migrate
- Update primary `README.md` with a features table
- Update `CHANGELOG.md` with breaking change notice
- Add feature flag examples to `/examples` directory demonstrating minimal and full builds

### FR6: Testing Matrix

The following build combinations **must** pass all tests:

**Main Combinations:**
1. `cargo build --no-default-features` — Core only, no adapters
2. `cargo build` — Default features only
3. `cargo build --all-features` — All features enabled

**Feature-Specific Test Matrix:**
1. Each LLM provider flag in isolation (llm-openai only, llm-anthropic only, etc.)
2. `web-server` enabled alone
3. `content-processing` enabled alone
4. `notifications` enabled alone
5. `redis-queue` + compatible combinations

Combinations must:
- Compile without errors
- Run `cargo test` successfully or skip tests appropriately with `#[cfg]`
- Produce no `cargo clippy` warnings under `-D warnings`
- Pass `cargo fmt --check`

---

## Non-Goals (Out of Scope)

- **Runtime Feature Detection:** This PRD does not require dynamic/runtime checking of available features. Gate-KeepingError is solved at compile time.
- **Feature Flag Backwards Compatibility:** The breaking change to default features is intentional. Users must update their `Cargo.toml`.
- **Workspace Decomposition:** Extracting separate Cargo crates (e.g., `paladin-core`, `paladin-web`) is out of scope.
- **Deprecation Period:** No multi-version deprecation cycle; breaking change happens immediately.
- **Automatic Feature Selection:** No "smart defaults" that detect user environment and auto-enable flags. Users explicitly choose.

---

## Design Considerations

### Module Structure
- Core domain types remain unconditionally compiled
- All port traits (`LlmPort`, `GarrisonPort`, etc.) remain unconditionally compiled
- Adapter implementations are gated by subsystem
- Routes, handlers, and middleware are gated (e.g., all web routes gated behind `web-server`)

### Dependency Grouping
- **Core dependencies** (serde, tokio, async-trait, reqwest, uuid, chrono, thiserror, sqlx): Always compiled
- **Subsystem-specific dependencies** (actix-web, axum, lettre, pdf-extract, scraper, tiktoken-rs): Gated with their feature
- **Provider-specific dependencies**: `reqwest` stays as a core dep (all three providers use it); only the concrete adapter modules and registration code are gated per provider flag
- **Vision/Sentinel dependencies** (chacha20poly1305, zeroize): Gated under `vision`
- **CLI dependencies** (clap, dialoguer, indicatif, colored, console, comfy-table, structopt): Out of scope for this Epic; handled in Epic 3

### Key File: `provider_factory.rs`

`src/infrastructure/adapters/llm/provider_factory.rs` unconditionally imports all three LLM adapters. This file must be updated with `#[cfg]` guards on each import and on the factory match arm for unavailable providers. This is one of the most impactful single-file changes in this Epic.

### Testing Strategy
Use feature-specific test modules:
```rust
#[cfg(test)]
#[cfg(feature = "web-server")]
mod integration_tests {
    // Tests only compile and run when web-server feature is enabled
}
```

---

## Technical Considerations

### Build System Integration
- Feature flags are defined in `Cargo.toml` under `[features]` section
- No custom build scripts required; standard Cargo feature mechanism used
- CI/CD automatically tests matrix via `cargo build --features ...` matrix

### Compatibility with Existing Code
- Existing examples using full features will continue to work (they'll use `--all-features` or default)
- Library consumers must update `Cargo.toml` to specify features (most will just add `llm-openai`)
- The `paladin-cli` binary (if built as `[[bin]]`) will use `--all-features` in its build

### Dead Code Warnings
- When a feature is disabled, code gated by `#[cfg]` is excluded from compilation (not dead code)
- `#[allow(dead_code)]` should not be used to suppress `cfg`-gated code

---

## Success Metrics

1. **Compilation Success:**
   - `cargo check --no-default-features` passes (core ports only)
   - `cargo check --all-features` passes (all features enabled)
   - `cargo check` with default features passes

2. **Test Coverage:**
   - All existing unit tests continue to pass
   - CI matrix tests all defined combinations with `cargo test`
   - No new clippy warnings or formatter violations introduced

3. **Documentation Completeness:**
   - Feature flags explicitly documented in markdown
   - At least one example per subsystem feature
   - Migration guide published in `CHANGELOG.md`

4. **Usage Verification:**
   - Existing downstream consumers (if any) can complete migration with <5 line changes to `Cargo.toml`
   - New consumer documentation makes it obvious which features to enable for common use cases

---

## Implementation Phases

This work is **phased by subsystem** to allow incremental testing and reduce risk of merge conflicts:

### Phase 1: LLM Provider Flags
**Deliverable:** `llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all` implemented and tested
**Estimated Duration:** 1-2 weeks

### Phase 2: Content Processing Flag
**Deliverable:** `content-processing` feature gates document processing pipeline
**Estimated Duration:** 1 week

### Phase 3: Web Server & Notifications Flags
**Deliverable:** `web-server` and `notifications` flags implemented
**Estimated Duration:** 1-2 weeks

### Phase 4: Vision Flag
**Deliverable:** `vision` flag implemented
**Estimated Duration:** 1 week
**Note:** `mcp-arsenal` flag originally planned for this phase has been eliminated from scope.

### Phase 5: Default Features & CI Integration
**Deliverable:** Default features updated, CI matrix configured, documentation complete
**Estimated Duration:** 1 week

---

## Open Questions

1. **Minimal Build Size:** Should we measure and document binary size reduction for `--no-default-features` builds?
2. **Feature Interdependencies:** Are there features that logically depend on others (e.g., does `vision` require `content-processing`)? Should we define `feature = [...]` dependencies in `Cargo.toml`?
3. **CLI Separation:** `structopt` (legacy), `clap`, and all CLI UI crates (`dialoguer`, `indicatif`, `colored`, etc.) are in main dependencies but should be gated. Epic 3 handles full CLI separation. Should Epic 1 pre-gate CLI deps to remove the duplication, or leave them for Epic 3?
4. **Web Server Scope:** Does the web server feature gate only the HTTP server adapter, or also WebSocket scaffolding that might be used by non-web consumers?
5. **Testing Infrastructure:** Should `integration-tests` feature gate be expanded to only enable integration test targets in CI?

---

## Acceptance Checklist

- [ ] All feature flags defined in `Cargo.toml` with correct dependencies
- [ ] All module-level `#[cfg]` guards applied correctly
- [ ] `cargo build --no-default-features` succeeds
- [ ] `cargo build --all-features` succeeds and all tests pass
- [ ] CI matrix tests all critical flag combinations
- [ ] All existing examples updated or new examples created
- [ ] Documentation updated: CONFIGURATION.md, FEATURE_FLAGS.md, MIGRATION.md, README.md, CHANGELOG.md
- [ ] No clippy warnings or formatting violations
- [ ] Downstream consumer can update their `Cargo.toml` and build successfully

---

## References

- **Existing Feature Pattern:** `src/Cargo.toml` (redis-queue, s3-storage flags)
- **Epic Source:** Milestone 4, Epic 1 definition
- **Related Epic:** Epic 3 (CLI Isolation) — may affect feature flag structure
- **Rust Crate Book:** https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
