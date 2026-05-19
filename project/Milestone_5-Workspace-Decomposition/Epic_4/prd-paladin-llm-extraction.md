# PRD: Extract `paladin-llm` Crate

**Epic:** Epic 4 — Extract `paladin-llm` Crate
**Milestone:** Milestone 5 (Tier 2) — Cargo Workspace Split
**Project:** Paladin Framework Refactoring Initiative
**Status:** Draft
**Author:** TBD
**Reviewers:** TBD
**Document Version:** 1.0
**Created:** 2026-05-18
**Last Updated:** 2026-05-18
**Target Audience:** Junior Developer

---

## 1. Introduction / Overview

### What Is This Feature?

This epic extracts all LLM (Large Language Model) provider adapter implementations out of the monolithic `paladin` crate and into a new, independent `paladin-llm` crate. The new crate consolidates four provider adapters — OpenAI, Anthropic, DeepSeek, and a Mock adapter — each guarded behind its own Cargo feature flag, so downstream consumers only compile the providers they actually use.

### What Problem Does It Solve?

Today, every adapter — OpenAI, Anthropic, DeepSeek, and the mock — lives in `src/infrastructure/adapters/llm/` inside the monolithic `paladin` crate. This means:

1. **Heavy, always-on dependencies.** Even a project that only uses Anthropic must compile the OpenAI and DeepSeek adapters (and their `reqwest` client setup) unless the Cargo feature flags are manually managed. With the current monolith structure, forgetting a feature flag is easy.
2. **No isolation of provider concerns.** A bug in the DeepSeek adapter requires rebuilding the entire framework to test.
3. **Testing is expensive.** Mock adapters live next to production adapters, and changing one test double requires rebuilding everything.

By extracting into `paladin-llm`, the Rust compiler enforces these boundaries: a crate that lists `paladin-llm` without enabling `--features anthropic` literally cannot reference `AnthropicAdapter` — the linker will refuse, not just a linter warning.

### Why Now (After Epics 1–3)?

`paladin-llm` depends on:
- `paladin-core` — for domain types used in request/response structs (e.g. `ContentItem`, `PromptItem`). ✅ Extracted in Epic 1.
- `paladin-ports` — for the `LlmPort`, `VisionPort`, and `EmbeddingPort` trait definitions. ✅ Extracted in Epic 2.

There is no dependency on `paladin-battalion`, so this epic can proceed directly after Epic 2 is complete (Epic 3 runs in parallel or beforehand — both are fine).

### Spec-Driven Development (SDD) Note

This PRD is a **specification**, not an implementation plan. It defines *what* must be true when Epic 4 is complete. A separate **task list** (in `tasks-paladin-llm-extraction.md`) will be derived from this PRD and will contain the concrete, step-by-step implementation work with checkboxes.

---

## 2. Goals

1. **Create `crates/paladin-llm/`** as an independent Cargo crate with its own `Cargo.toml` and `src/lib.rs`.
2. **Extract OpenAI adapter** (`openai_adapter.rs`, `openai_embedding_adapter.rs`, `openai_vision.rs`) into `paladin-llm` behind the `openai` feature flag.
3. **Extract Anthropic adapter** (`anthropic_adapter.rs`, `anthropic_vision.rs`) into `paladin-llm` behind the `anthropic` feature flag.
4. **Extract DeepSeek adapter** (`deepseek_adapter.rs`) into `paladin-llm` behind the `deepseek` feature flag.
5. **Extract mock adapters** (`mock_llm_adapter.rs`, `MultiStepMockLlmPort`) into `paladin-llm` behind the `mock` feature flag.
6. **Extract the provider factory** (`provider_factory.rs`) into `paladin-llm` so adapter creation logic is co-located with the adapters.
7. **Define a `LlmProviderError` enum** in `paladin-llm` that captures per-provider error detail and converts to the shared `LlmError` type (from `paladin-ports`) at the public API boundary.
8. **Integrate configuration** so `paladin-llm` adapters own their own config structs (`OpenAIConfig`, `AnthropicConfig`, `DeepSeekConfig`), and the root `paladin` crate provides the wiring that populates those structs from `ApplicationSettings`.
9. **Set default features to `["openai", "mock"]`** — production-ready out of the box, with the mock always available for testing.
10. **Expose extracted types through `paladin::prelude`** — add all public adapter types to the facade prelude module; do not preserve old deep import paths.
11. **All existing tests continue to pass** — zero regressions.

---

## 3. User Stories

### Story 1 — Downstream Consumer (OpenAI Only)

> As a **downstream consumer** who only uses OpenAI models, I want to depend on `paladin-llm` with only the `openai` feature enabled so that my project does not compile the Anthropic SDK or DeepSeek client code.

**Acceptance:** `cargo build` for a downstream crate that specifies `paladin-llm = { version = "...", features = ["openai"] }` does not pull in any Anthropic or DeepSeek dependencies. `cargo tree` confirms this.

---

### Story 2 — Test Author

> As a **test author**, I want `MockLlmPort` and `MultiStepMockLlmPort` to be available whenever I add `paladin-llm` as a `dev-dependency` with its default features so that I can write LLM-backed tests without any real API calls or extra feature flag ceremony.

**Acceptance:** The `mock` feature is enabled by default. A test file that does `use paladin_llm::mock::MockLlmPort;` compiles without specifying any additional feature flags.

---

### Story 3 — Provider Maintainer

> As a **provider maintainer** responsible for the DeepSeek adapter, I want to modify, test, and rebuild only `paladin-llm --features deepseek` without waiting for the OpenAI, Anthropic, or battalion compilation units to recompile.

**Acceptance:** `cargo test -p paladin-llm --features deepseek` runs in isolation. Changes to DeepSeek-only code do not trigger recompilation of `paladin-battalion`, `paladin-core`, or the root facade crate.

---

### Story 4 — Application Developer Using Config System

> As an **application developer**, I want adapter creation to work automatically from the existing YAML configuration system (`config.yml`) so that I do not need to manually construct `OpenAIConfig` structs in my application bootstrap code.

**Acceptance:** The root `paladin` crate's wiring layer reads `ApplicationSettings.llm.openai` (etc.) and passes the data into `OpenAIConfig`. The application developer using the full `paladin` facade does not interact with `paladin-llm` config structs directly.

---

### Story 5 — Junior Developer Working from the Task List

> As the **implementer working from the task list**, I want every functional requirement and technical constraint spelled out clearly so that I can implement each task without guessing the intent or making architecture decisions that belong to the spec.

**Acceptance:** The task list derived from this PRD contains no ambiguous steps. Every task maps to at least one numbered functional requirement in Section 4.

---

## 4. Functional Requirements

> **Note for implementer:** Requirements are numbered FR-N. Every requirement must be satisfied for the epic to be considered complete. Where a requirement references a specific file, that file path is relative to the workspace root.

### 4.1 Crate Scaffold

- **FR-1:** The directory `crates/paladin-llm/` must exist with a valid `Cargo.toml` and `src/lib.rs`.
- **FR-2:** `crates/paladin-llm/Cargo.toml` must set `name = "paladin-llm"`, `edition = "2021"`, and use workspace dependency syntax (`dep = { workspace = true }`) for all shared dependencies.
- **FR-3:** The `[features]` table in `crates/paladin-llm/Cargo.toml` must define: `openai`, `anthropic`, `deepseek`, `mock`, and `vision`. The `default` feature must be `["openai", "mock"]`.
- **FR-4:** `reqwest` (with TLS features) must be an optional dependency, activated only when at least one of `openai`, `anthropic`, or `deepseek` is enabled. It must not be compiled when building with `--no-default-features`.
- **FR-5:** `cargo build -p paladin-llm --no-default-features` must succeed and produce a crate with no provider code (an empty public surface is acceptable).
- **FR-6:** `paladin-llm` must depend on `paladin-core` and `paladin-ports` as non-optional workspace dependencies.
- **FR-7:** `paladin-llm` must NOT depend on the root `paladin` crate, `paladin-battalion`, `paladin-memory`, or any infrastructure crate other than the adapters being moved into it.

### 4.2 Error Handling

- **FR-8:** A `LlmProviderError` enum must be defined in `crates/paladin-llm/src/error.rs` and re-exported at the crate root as `paladin_llm::LlmProviderError`. This enum must have at minimum the following variants — one general variant plus per-provider variants for detail capture:

  ```
  LlmProviderError::OpenAI(String)
  LlmProviderError::Anthropic(String)
  LlmProviderError::DeepSeek(String)
  LlmProviderError::Configuration(String)
  LlmProviderError::Network(String)
  LlmProviderError::RateLimit
  LlmProviderError::Timeout(u64)
  LlmProviderError::TokenLimitExceeded { limit: usize, requested: usize }
  LlmProviderError::Authentication(String)
  LlmProviderError::Serialization(String)
  ```

  The enum must derive `thiserror::Error` and `Debug`.

- **FR-9:** `LlmProviderError` must implement `From<LlmProviderError> for LlmError` (where `LlmError` is `paladin_ports::output::llm_port::LlmError`). This conversion is the boundary-crossing point. Each adapter's internal `?` operator propagates `LlmProviderError`; the `LlmPort` trait method signatures return `LlmError`, so the conversion must be applied at the `impl LlmPort` method bodies.

- **FR-10:** Provider-specific HTTP error structs (e.g., OpenAI's API error JSON body) may remain as private types within each provider module. Only `LlmProviderError` is part of the public API.

### 4.3 OpenAI Provider (feature: `openai`)

- **FR-11:** The following files must be relocated from `src/infrastructure/adapters/llm/` into `crates/paladin-llm/src/openai/`:
  - `openai_adapter.rs` → `crates/paladin-llm/src/openai/adapter.rs`
  - `openai_embedding_adapter.rs` → `crates/paladin-llm/src/openai/embedding.rs`

- **FR-12:** The vision extension `openai_vision.rs` must be relocated to `crates/paladin-llm/src/openai/vision.rs` and gated behind `#[cfg(all(feature = "openai", feature = "vision"))]`.

- **FR-13:** `OpenAIConfig` must remain a public struct in `crates/paladin-llm/src/openai/adapter.rs` (or a shared `config.rs` within `openai/`). Its fields must be as currently defined: `api_key: String`, `base_url: String`, `organization: Option<String>`, `timeout_seconds: u64`, `max_retries: u32`. The `from_env()` constructor must be preserved.

- **FR-14:** All `use crate::...` statements in the moved OpenAI files that previously pointed into the monolithic crate must be updated to reference `paladin_core::` or `paladin_ports::` as appropriate.

- **FR-15:** `cargo build -p paladin-llm --features openai` must succeed. `cargo build -p paladin-llm --no-default-features` must NOT include `OpenAIAdapter` in the compiled output.

- **FR-16:** All unit tests from the OpenAI adapter files must be moved with the source and must pass under `cargo test -p paladin-llm --features openai`.

### 4.4 Anthropic Provider (feature: `anthropic`)

- **FR-17:** The following files must be relocated:
  - `anthropic_adapter.rs` → `crates/paladin-llm/src/anthropic/adapter.rs`
  - `anthropic_vision.rs` → `crates/paladin-llm/src/anthropic/vision.rs`, gated behind `#[cfg(all(feature = "anthropic", feature = "vision"))]`.

- **FR-18:** `AnthropicConfig` must be a public struct with at minimum: `api_key: String`, `base_url: String`, `timeout_seconds: u64`, `max_retries: u32`. The `from_env()` constructor must be preserved.

- **FR-19:** `cargo build -p paladin-llm --features anthropic` must succeed independently of the `openai` and `deepseek` flags.

- **FR-20:** All unit tests from the Anthropic adapter files must be moved and pass under `cargo test -p paladin-llm --features anthropic`.

### 4.5 DeepSeek Provider (feature: `deepseek`)

- **FR-21:** `deepseek_adapter.rs` must be relocated to `crates/paladin-llm/src/deepseek/adapter.rs` (or `crates/paladin-llm/src/deepseek.rs` if it does not have sub-modules).

- **FR-22:** `DeepSeekConfig` must be a public struct with at minimum: `api_key: String`, `base_url: String`, `timeout_seconds: u64`, `max_retries: u32`. The `from_env()` constructor must be preserved.

- **FR-23:** `cargo build -p paladin-llm --features deepseek` must succeed independently of the `openai` and `anthropic` flags.

- **FR-24:** All unit tests from the DeepSeek adapter file must be moved and pass under `cargo test -p paladin-llm --features deepseek`.

### 4.6 Mock Adapters (feature: `mock`)

- **FR-25:** `mock_llm_adapter.rs` must be relocated to `crates/paladin-llm/src/mock.rs`. Both `MockLlmPort` and `MultiStepMockLlmPort` must be re-exported at `paladin_llm::mock::MockLlmPort` and `paladin_llm::mock::MultiStepMockLlmPort`.

- **FR-26:** The `mock` feature must be enabled by default (part of `default = ["openai", "mock"]`). It must compile without any network dependencies — `reqwest` must NOT be required when only `mock` is enabled.

- **FR-27:** Mock adapter tests must compile and pass under `cargo test -p paladin-llm --features mock` (no other feature required).

### 4.7 Provider Factory

- **FR-28:** `provider_factory.rs` must be relocated to `crates/paladin-llm/src/provider_factory.rs`. The `LlmProviderFactory` struct and `ProviderFactoryError` enum must be re-exported at the crate root.

- **FR-29:** `LlmProviderFactory::create()` must remain feature-gated internally: it returns an error for providers whose feature flag is not enabled, rather than failing to compile. This allows the factory to be used even in a partial-feature build.

- **FR-30:** `ProviderFactoryError` is a private concern within `paladin-llm`. It must implement `From<ProviderFactoryError> for LlmProviderError`.

### 4.8 Configuration Integration

- **FR-31:** `paladin-llm` must NOT import from `crate::config::application_settings` or any equivalent path in the root `paladin` crate. Doing so would create a circular dependency.

- **FR-32:** Each provider adapter's `*Config` struct (e.g. `OpenAIConfig`) is the configuration boundary. The root `paladin` crate is solely responsible for reading `ApplicationSettings.llm.*` fields and converting them into the appropriate `paladin-llm` `*Config` struct. This conversion code lives in the main crate (e.g., in a new `src/infrastructure/adapters/llm/config_bridge.rs`), not in `paladin-llm`.

- **FR-33:** Each `*Config` struct must retain its `from_env()` constructor so that `paladin-llm` remains usable without the full `ApplicationSettings` system (e.g., in standalone examples, CLIs, and integration tests that set environment variables directly).

### 4.9 Test Architecture

- **FR-34:** Unit tests (testing individual methods and error conversions) must live co-located with their source in `#[cfg(test)]` modules inside `crates/paladin-llm/src/**/*.rs`.

- **FR-35:** Integration tests (testing full `LlmPort` roundtrips, factory creation, feature-flag matrix) must be placed in `crates/paladin-llm/tests/`. Each integration test file must be gated with `#[cfg(feature = "...")]` appropriate to the provider(s) it tests.

- **FR-36:** No integration test in `crates/paladin-llm/tests/` may make real network calls. All provider integration tests must use the mock adapter or a test double. Tests that require a live API key must be annotated with `#[ignore]` and documented accordingly.

### 4.10 Facade and Prelude

- **FR-37:** The root `paladin` crate's `Cargo.toml` must add `paladin-llm` as a dependency with `default-features = false` and the features list mirroring what the root crate currently enables for its own LLM feature flags (e.g., `features = ["openai", "anthropic", "deepseek", "mock"]`).

- **FR-38:** The root `paladin` crate must add all public `paladin-llm` adapter types to `paladin::prelude`. At minimum: `OpenAIAdapter`, `AnthropicAdapter`, `DeepSeekAdapter`, `MockLlmPort`, `MultiStepMockLlmPort`, `LlmProviderFactory`, `LlmProviderError`.

- **FR-39:** Old deep import paths (e.g., `paladin::infrastructure::adapters::llm::openai_adapter::OpenAIAdapter`) are **not** required to be preserved. New consumers must use `paladin::prelude::OpenAIAdapter`. The task list must include a grep-and-update sweep over all workspace examples and integration tests to update any broken import paths.

- **FR-40:** After the facade is wired, the original source files in `src/infrastructure/adapters/llm/` must be removed (not left as dead code or re-export shims).

### 4.11 Workspace Build Validation

- **FR-41:** `cargo build -p paladin-llm --no-default-features` must succeed (empty crate).
- **FR-42:** `cargo build -p paladin-llm --features openai` must succeed.
- **FR-43:** `cargo build -p paladin-llm --features anthropic` must succeed.
- **FR-44:** `cargo build -p paladin-llm --features deepseek` must succeed.
- **FR-45:** `cargo build -p paladin-llm --features mock` must succeed.
- **FR-46:** `cargo build -p paladin-llm --all-features` must succeed.
- **FR-47:** `cargo test --workspace` must pass with all tests green (no regressions from earlier epics).
- **FR-48:** `cargo clippy -p paladin-llm --all-features -- -D warnings` must produce zero warnings.
- **FR-49:** `cargo fmt --check -p paladin-llm` must pass.

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** part of this epic:

1. **Adding new LLM providers.** This epic is a refactoring extraction only. No new providers (e.g., Google Gemini, Mistral) are to be added.
2. **Changing the `LlmPort` trait interface.** The port trait lives in `paladin-ports` and was hardened in Milestone 1 / Epic 2. Its signature must not be changed here.
3. **Splitting `paladin-llm` into one crate per provider.** A single crate with feature flags is the chosen design. Per-provider crates are a future consideration only.
4. **Introducing a `paladin-config` crate.** Configuration extraction is scoped to Milestone 5 Tier 3. This epic only enforces the bridge pattern described in FR-32.
5. **Streaming / SSE architecture changes.** The existing streaming implementation in each adapter is moved as-is. Refactoring stream handling is out of scope.
6. **Changing retry or backoff logic.** Retry configuration per adapter is moved as-is.
7. **Memory or storage adapters.** Those belong to Epic 5 (`paladin-memory`).
8. **CI pipeline updates.** Workspace-level CI configuration is covered in Epic 6.

---

## 6. Design Considerations

### 6.1 Module Structure

The target directory layout for `paladin-llm`:

```
crates/paladin-llm/
├── Cargo.toml
└── src/
    ├── lib.rs                    # Feature-gated module declarations; crate-level re-exports
    ├── error.rs                  # LlmProviderError enum + From<LlmProviderError> for LlmError
    ├── provider_factory.rs       # LlmProviderFactory (feature-gated internally)
    ├── openai/                   # Gated: #[cfg(feature = "openai")]
    │   ├── mod.rs
    │   ├── adapter.rs            # OpenAIAdapter + OpenAIConfig
    │   ├── embedding.rs          # OpenAIEmbeddingAdapter
    │   └── vision.rs             # Gated: #[cfg(all(feature = "openai", feature = "vision"))]
    ├── anthropic/                # Gated: #[cfg(feature = "anthropic")]
    │   ├── mod.rs
    │   ├── adapter.rs            # AnthropicAdapter + AnthropicConfig
    │   └── vision.rs             # Gated: #[cfg(all(feature = "anthropic", feature = "vision"))]
    ├── deepseek/                 # Gated: #[cfg(feature = "deepseek")]
    │   ├── mod.rs
    │   └── adapter.rs            # DeepSeekAdapter + DeepSeekConfig
    └── mock.rs                   # Gated: #[cfg(feature = "mock")] — MockLlmPort, MultiStepMockLlmPort
```

And integration tests:

```
crates/paladin-llm/
└── tests/
    ├── openai_integration.rs     # #[cfg(feature = "openai")] — uses mock or env-var gated live tests
    ├── anthropic_integration.rs  # #[cfg(feature = "anthropic")]
    ├── deepseek_integration.rs   # #[cfg(feature = "deepseek")]
    ├── mock_integration.rs       # #[cfg(feature = "mock")]
    └── provider_factory_test.rs  # tests LlmProviderFactory::create() across feature combos
```

### 6.2 Feature Flag Dependency Mapping

| Feature | Required Crate Dependencies (beyond paladin-core, paladin-ports) |
|---------|-------------------------------------------------------------------|
| `openai` | `reqwest` (TLS), `serde`, `serde_json`, `futures` |
| `anthropic` | `reqwest` (TLS), `serde`, `serde_json`, `futures` |
| `deepseek` | `reqwest` (TLS), `serde`, `serde_json`, `futures` |
| `mock` | `tokio` (sync primitives), `futures` |
| `vision` | Requires at least one of `openai` or `anthropic` to be meaningful |

`reqwest` must be declared as an optional dependency in `Cargo.toml`:

```toml
[dependencies]
reqwest = { workspace = true, optional = true, features = ["json", "rustls-tls"] }

[features]
openai    = ["reqwest"]
anthropic = ["reqwest"]
deepseek  = ["reqwest"]
mock      = []
vision    = []
default   = ["openai", "mock"]
```

### 6.3 Error Conversion Flow

The error conversion chain is:
```
Provider HTTP error (private)
    ↓  (via match/map_err inside adapter)
LlmProviderError  (paladin-llm public type)
    ↓  (via From<LlmProviderError> for LlmError, called with `?` or `.into()`)
LlmError          (paladin-ports public type, returned by LlmPort methods)
```

The `impl LlmPort for OpenAIAdapter` methods internally return `Result<T, LlmProviderError>` and convert at the return point:

```rust
async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
    self.call_api(request)
        .await
        .map_err(LlmError::from)   // LlmProviderError → LlmError via From impl
}
```

### 6.4 Configuration Bridge Pattern

To avoid a circular dependency between `paladin-llm` and the root `paladin` crate's config system, this epic introduces a **bridge module** in the root crate:

```
src/infrastructure/adapters/llm/config_bridge.rs
```

This module implements `From<&LlmProviderConfig> for OpenAIConfig` (and equivalent for other providers), where `LlmProviderConfig` is the existing type from `application_settings.rs`. This conversion code lives entirely in the root crate and is invisible to `paladin-llm`.

Callers that do not use the `ApplicationSettings` system (standalone tests, examples, CLI) continue to call `OpenAIConfig::from_env()` directly.

### 6.5 Vision Adapters

Vision adapters (`openai_vision.rs`, `anthropic_vision.rs`) implement `VisionPort` from `paladin-ports`. They depend on `VisionConfig` from `application_settings.rs` — which presents the same circular dependency risk as the main config (see Section 6.4). The same bridge pattern applies: move the vision adapters into `paladin-llm` behind the `vision` feature, remove the direct `application_settings` import, and add a corresponding `From<&VisionConfig> for OpenAIVisionConfig` in the root crate's config bridge module.

---

## 7. Technical Considerations

### 7.1 Prerequisites

- Epic 1 (`paladin-core`) must be complete: domain types must be accessible at `paladin_core::`.
- Epic 2 (`paladin-ports`) must be complete: `LlmPort`, `LlmError`, `EmbeddingPort`, `VisionPort`, and `VisionCapableLlm` must be accessible at `paladin_ports::output::`.
- Epic 3 (`paladin-battalion`) does not need to be complete — this epic has no dependency on battalion.

### 7.2 Dependency on `crate::core::platform::container::content`

The current `openai_adapter.rs` imports:

```rust
use crate::core::platform::container::content::{ContentItem, ContentType};
use crate::core::platform::container::prompt::{PromptItem, PromptType};
```

After extraction these must become:

```rust
use paladin_core::platform::container::content::{ContentItem, ContentType};
use paladin_core::platform::container::prompt::{PromptItem, PromptType};
```

All similar `crate::core::...` references in the moved files must be updated. The implementer must grep for `crate::core::` in each moved file and replace with `paladin_core::`.

### 7.3 `rand` Dependency

`openai_adapter.rs` uses `rand::Rng` (for jitter in retry backoff). `rand` must be added as an optional dependency under the `openai` feature, or extracted into a shared internal utility. Check `Cargo.toml` workspace dependencies — if `rand` is not already declared, it must be added.

### 7.4 No `pub use` Forwarding in `src/infrastructure/adapters/llm/mod.rs`

After the adapter source is deleted from `src/infrastructure/adapters/llm/`, that module should be removed entirely (or reduced to a comment explaining the move). Do not leave empty `pub use` re-exports pointing into `paladin_llm` from the old location — old paths are intentionally deprecated per FR-39.

### 7.5 Workspace `Cargo.toml` — Adding `paladin-llm`

The workspace root `Cargo.toml`'s `members` list must include `"crates/paladin-llm"`. This is separate from the root `paladin` crate depending on it — the workspace membership makes `cargo build --workspace` include it in all-crate builds.

### 7.6 Import Path Sweep

After facade wiring is complete, the implementer must run:

```bash
grep -r "infrastructure::adapters::llm" --include="*.rs" .
```

in the workspace root and update all matches. Examples, integration tests under `tests/`, and benchmarks under `benches/` are the most likely locations of old paths. Each must be updated to use `paladin::prelude::*` or the direct `paladin_llm::` path.

---

## 8. Success Metrics

| Metric | Target |
|--------|--------|
| `cargo build -p paladin-llm --no-default-features` | Succeeds in < 5 seconds |
| `cargo build -p paladin-llm --features openai` | Succeeds; `cargo tree -p paladin-llm --features openai` shows no Anthropic or DeepSeek deps |
| `cargo test -p paladin-llm --all-features` | All tests pass, zero failures |
| `cargo test --workspace` | All 1,487+ existing tests continue to pass |
| `cargo clippy -p paladin-llm --all-features -- -D warnings` | Zero warnings |
| Incremental rebuild time for OpenAI-only change | Measurably less than full workspace rebuild (target: ≥ 50% reduction) |
| `cargo doc -p paladin-llm --all-features --no-deps` | Zero broken intra-doc links, zero warnings |
| Dead provider code compiled for single-provider consumer | Zero bytes (confirmed by `cargo tree`) |

---

## 9. Open Questions

1. **`rand` version alignment.** Is `rand` declared in `[workspace.dependencies]`? If not, should it be added there or declared only in `paladin-llm`'s `Cargo.toml`? The implementer should check before starting Task 4.1.

2. **`VisionConfig` bridge scope.** The bridge pattern for `VisionConfig` → `OpenAIVisionConfig` (and Anthropic equivalent) is described in Section 6.5. However, the full scope of `VisionConfig` fields has not been audited. The implementer should read `src/config/application_settings.rs`'s `VisionConfig` struct in full and confirm all fields have a corresponding slot in the vision adapter config structs before proceeding with Task 4.2 (OpenAI extraction).

3. **`MultiStepMockLlmPort` location.** The Epic 4 outline references `MultiStepMockLlmPort` but this type was not found in `mock_llm_adapter.rs` during spec authoring (the file contained only `MockLlmPort`). The implementer must grep for `MultiStepMockLlmPort` across the workspace to locate it, or confirm it does not yet exist and note that it is a new type to be created in FR-25.

4. **`openai_embedding_adapter.rs` feature flag naming.** The monolith uses `openai-embeddings` as a separate feature flag (distinct from `llm-openai`). In `paladin-llm`, the embedding adapter is consolidated under the `openai` feature. The implementer must confirm that no downstream code depends on `openai-embeddings` as a distinct flag before merging the two.

5. **`provider_factory.rs` and `ApplicationSettings` coupling.** A quick audit is needed: does the current `LlmProviderFactory` in `provider_factory.rs` read from `ApplicationSettings` directly, or does it only read from environment variables and config structs passed to it? If it imports from `application_settings`, that import must be removed and replaced with the bridge pattern before the file is moved (see FR-31 and Section 6.4).
