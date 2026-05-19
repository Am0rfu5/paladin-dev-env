## Relevant Files

- `Cargo.toml` — Workspace root manifest; `members` list must include `"crates/paladin-llm"`.
- `crates/paladin-llm/Cargo.toml` — New manifest for the `paladin-llm` crate; defines `openai`, `anthropic`, `deepseek`, `mock`, `vision` feature flags; `default = ["openai", "mock"]`.
- `crates/paladin-llm/src/lib.rs` — Crate root; feature-gated module declarations and top-level re-exports.
- `crates/paladin-llm/src/error.rs` — `LlmProviderError` enum and `From<LlmProviderError> for LlmError` conversion.
- `crates/paladin-llm/src/provider_factory.rs` — `LlmProviderFactory` (relocated from `src/infrastructure/adapters/llm/provider_factory.rs`).
- `crates/paladin-llm/src/openai/mod.rs` — OpenAI module gate (`#[cfg(feature = "openai")]`).
- `crates/paladin-llm/src/openai/adapter.rs` — `OpenAIAdapter` + `OpenAIConfig` (relocated from `src/infrastructure/adapters/llm/openai_adapter.rs`).
- `crates/paladin-llm/src/openai/embedding.rs` — `OpenAIEmbeddingAdapter` (relocated from `openai_embedding_adapter.rs`).
- `crates/paladin-llm/src/openai/vision.rs` — OpenAI vision extension (relocated from `openai_vision.rs`); gated behind `openai` + `vision`.
- `crates/paladin-llm/src/anthropic/mod.rs` — Anthropic module gate (`#[cfg(feature = "anthropic")]`).
- `crates/paladin-llm/src/anthropic/adapter.rs` — `AnthropicAdapter` + `AnthropicConfig` (relocated from `anthropic_adapter.rs`).
- `crates/paladin-llm/src/anthropic/vision.rs` — Anthropic vision extension (relocated from `anthropic_vision.rs`); gated behind `anthropic` + `vision`.
- `crates/paladin-llm/src/deepseek/mod.rs` — DeepSeek module gate (`#[cfg(feature = "deepseek")]`).
- `crates/paladin-llm/src/deepseek/adapter.rs` — `DeepSeekAdapter` + `DeepSeekConfig` (relocated from `deepseek_adapter.rs`).
- `crates/paladin-llm/src/mock.rs` — `MockLlmPort` and `MultiStepMockLlmPort` (relocated from `mock_llm_adapter.rs`); gated behind `mock`.
- `crates/paladin-llm/tests/openai_integration.rs` — OpenAI `LlmPort` integration tests (no live network calls; `#[ignore]` for live-key tests).
- `crates/paladin-llm/tests/anthropic_integration.rs` — Anthropic integration tests.
- `crates/paladin-llm/tests/deepseek_integration.rs` — DeepSeek integration tests.
- `crates/paladin-llm/tests/mock_integration.rs` — Mock adapter integration tests.
- `crates/paladin-llm/tests/provider_factory_test.rs` — `LlmProviderFactory::create()` tests across feature combinations.
- `src/infrastructure/adapters/llm/config_bridge.rs` — New file in the root crate; `From<&LlmProviderConfig> for OpenAIConfig` (and Anthropic/DeepSeek equivalents); also `From<&VisionConfig>` conversions.
- `src/infrastructure/adapters/llm/mod.rs` — Removed after all adapter source is deleted; or replaced with a tombstone comment.
- `src/lib.rs` — Root `paladin` facade; `paladin-llm` added as a dependency; public adapter types added to `paladin::prelude`.
- `src/prelude.rs` — (or equivalent prelude module) Updated to re-export `OpenAIAdapter`, `AnthropicAdapter`, `DeepSeekAdapter`, `MockLlmPort`, `MultiStepMockLlmPort`, `LlmProviderFactory`, `LlmProviderError` from `paladin_llm`.

### Notes

- This is a **structural refactor with zero behavioral change**. No public API shape, no retry logic, no streaming behavior may be altered.
- Follow the workspace Rust TDD discipline: `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` must all pass before marking any parent task complete.
- Build and test the extracted crate in isolation after each provider extraction: `cargo build -p paladin-llm --features <provider>` and `cargo test -p paladin-llm --features <provider>`.
- Verify dependency isolation with `cargo tree -p paladin-llm --features openai` — confirm Anthropic and DeepSeek symbols are absent.
- `reqwest` must NOT appear in `cargo tree -p paladin-llm --no-default-features --features mock`.
- Open questions from the PRD (Section 9) must each be answered before the task that touches that area begins — see individual task notes.
- The `MultiStepMockLlmPort` grep (PRD Open Question 3) must be resolved in Task 6.0 before writing the mock extraction.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [x] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Run `git branch --show-current` to confirm the current integration branch (expected: `feature/milestone_5-epic_1-paladin-core-extraction` or equivalent milestone branch with Epics 1–3 merged in)
  - [x] 0.2 Create and checkout the Epic 4 branch: `git checkout -b feature/milestone_5-epic_4-paladin-llm-extraction`
  - [x] 0.3 Push the branch to origin: `git push -u origin feature/milestone_5-epic_4-paladin-llm-extraction`

- [x] 1.0 Scaffold the `paladin-llm` crate
  - [x] 1.1 Capture the pre-epic baseline: run `cargo test --workspace --all-features 2>&1 | tail -5` and record the passing test count in a comment at the top of this file
  - [x] 1.2 Resolve PRD Open Question 1 — check if `rand` is in `[workspace.dependencies]`: run `grep -n "^rand" Cargo.toml`; if absent, add `rand = "0.8"` (or the version already used in `openai_adapter.rs`) to the `[workspace.dependencies]` section of the root `Cargo.toml`
  - [x] 1.3 Create the directory `crates/paladin-llm/src/`
  - [x] 1.4 Create `crates/paladin-llm/Cargo.toml` with `name = "paladin-llm"`, `edition = "2021"`, workspace dependency references for `paladin-core`, `paladin-ports`, `serde`, `serde_json`, `thiserror`, `async-trait`, `tokio`, `futures`, and `log`; define `[features]` with `openai`, `anthropic`, `deepseek`, `mock`, `vision` and `default = ["openai", "mock"]`; declare `reqwest` as an optional dependency activated by `openai`, `anthropic`, and `deepseek`; declare `rand` as an optional dependency activated by `openai` (FR-2, FR-3, FR-4)
  - [x] 1.5 Create `crates/paladin-llm/src/lib.rs` with a crate-level `//!` doc comment, `#![deny(unsafe_code)]` lint, and empty `#[cfg(feature = "...")]` module stubs for `openai`, `anthropic`, `deepseek`, `mock`, and `error` (uncommented as they are created in subsequent tasks)
  - [x] 1.6 Add `"crates/paladin-llm"` to the `members` list in the workspace root `Cargo.toml` (FR-7 prerequisite)
  - [x] 1.7 Run `cargo build -p paladin-llm --no-default-features` and confirm the empty crate compiles (FR-5)
  - [x] 1.8 Run `cargo build --workspace` and confirm no regressions in the existing workspace members

- [x] 2.0 Define `LlmProviderError` and the error conversion boundary
  - [x] 2.1 Create `crates/paladin-llm/src/error.rs`; define the `LlmProviderError` enum with all required variants: `OpenAI(String)`, `Anthropic(String)`, `DeepSeek(String)`, `Configuration(String)`, `Network(String)`, `RateLimit`, `Timeout(u64)`, `TokenLimitExceeded { limit: usize, requested: usize }`, `Authentication(String)`, `Serialization(String)` — derive `thiserror::Error` and `Debug` (FR-8)
  - [x] 2.2 In the same `error.rs`, implement `From<LlmProviderError> for paladin_ports::output::llm_port::LlmError`, mapping each `LlmProviderError` variant to the closest `LlmError` variant (FR-9)
  - [x] 2.3 Uncomment / add `pub mod error;` and `pub use error::LlmProviderError;` in `crates/paladin-llm/src/lib.rs` so the type is accessible at `paladin_llm::LlmProviderError`
  - [x] 2.4 Write unit tests inside a `#[cfg(test)]` block in `error.rs` covering: (a) each `From` conversion variant produces the expected `LlmError` discriminant, and (b) `thiserror` Display formatting is non-empty for each variant (FR-34)
  - [x] 2.5 Run `cargo test -p paladin-llm --no-default-features` and confirm the error module tests pass without any provider feature
  - [x] 2.6 Run `cargo clippy -p paladin-llm --no-default-features -- -D warnings` and resolve any warnings

- [x] 3.0 Extract the OpenAI adapter
  - [x] 3.1 Resolve PRD Open Question 4 — run `grep -rn "openai-embeddings" --include="*.rs" --include="*.toml" .` to check whether the `openai-embeddings` feature flag is referenced anywhere outside `src/`; document findings and confirm it is safe to consolidate under the `openai` feature in `paladin-llm`
  - [x] 3.2 Resolve PRD Open Question 2 — read `src/config/application_settings.rs` `VisionConfig` struct and compare its fields against the config requirements in `src/infrastructure/adapters/llm/openai_vision.rs`; note any gaps to address in Task 8.0
  - [x] 3.3 Create `crates/paladin-llm/src/openai/mod.rs`; declare `pub mod adapter;`, `pub mod embedding;`, and `#[cfg(feature = "vision")] pub mod vision;`; re-export `OpenAIAdapter` and `OpenAIEmbeddingAdapter` at the module level
  - [x] 3.4 Copy `src/infrastructure/adapters/llm/openai_adapter.rs` to `crates/paladin-llm/src/openai/adapter.rs`; update every `use crate::core::...` import to `use paladin_core::...`; update every `use paladin_ports::` import to remove any old full paths; replace internal error propagation with `LlmProviderError` variants using `map_err`; convert `impl LlmPort` method return-site errors with `.map_err(LlmError::from)` (FR-11, FR-13, FR-14)
  - [x] 3.5 Copy `src/infrastructure/adapters/llm/openai_embedding_adapter.rs` to `crates/paladin-llm/src/openai/embedding.rs`; apply the same import-path updates (FR-11, FR-14)
  - [x] 3.6 Copy `src/infrastructure/adapters/llm/openai_vision.rs` to `crates/paladin-llm/src/openai/vision.rs`; wrap file contents with `#[cfg(all(feature = "openai", feature = "vision"))]`; remove `use crate::config::application_settings::VisionConfig` import (the config bridge in Task 8.0 will handle this); apply remaining import-path updates (FR-12, FR-14)
  - [x] 3.7 Add `#[cfg(feature = "openai")] pub mod openai;` to `crates/paladin-llm/src/lib.rs` and re-export `openai::OpenAIAdapter` and `openai::OpenAIEmbeddingAdapter` at the crate root under `#[cfg(feature = "openai")]`
  - [x] 3.8 Run `cargo build -p paladin-llm --features openai` and fix any remaining import or type errors (FR-15)
  - [x] 3.9 Run `cargo build -p paladin-llm --no-default-features` and confirm `OpenAIAdapter` does not appear in `cargo tree` output (FR-15)
  - [x] 3.10 Run `cargo test -p paladin-llm --features openai` and confirm all co-located unit tests pass (FR-16)
  - [x] 3.11 Create `crates/paladin-llm/tests/openai_integration.rs`; gate the file with `#![cfg(feature = "openai")]`; write at least two integration tests: (a) `MockLlmPort` satisfies `LlmPort` compile-time check and (b) an `#[ignore]` live-key test documented with the required env var (FR-35, FR-36)
  - [x] 3.12 Run `cargo tree -p paladin-llm --features openai` and confirm no Anthropic or DeepSeek crates appear in the dependency graph

- [x] 4.0 Extract the Anthropic adapter
  - [x] 4.1 Create `crates/paladin-llm/src/anthropic/mod.rs`; declare `pub mod adapter;` and `#[cfg(feature = "vision")] pub mod vision;`; re-export `AnthropicAdapter`
  - [x] 4.2 Copy `src/infrastructure/adapters/llm/anthropic_adapter.rs` to `crates/paladin-llm/src/anthropic/adapter.rs`; update `use crate::core::...` → `paladin_core::`; update `use paladin_ports::` paths; replace internal error propagation with `LlmProviderError::Anthropic(...)` variants; apply `.map_err(LlmError::from)` at `impl LlmPort` return sites (FR-17, FR-18)
  - [x] 4.3 Copy `src/infrastructure/adapters/llm/anthropic_vision.rs` to `crates/paladin-llm/src/anthropic/vision.rs`; wrap with `#[cfg(all(feature = "anthropic", feature = "vision"))]`; remove `VisionConfig` import; apply remaining path updates (FR-17)
  - [x] 4.4 Add `#[cfg(feature = "anthropic")] pub mod anthropic;` to `crates/paladin-llm/src/lib.rs` and re-export `AnthropicAdapter` under `#[cfg(feature = "anthropic")]`
  - [x] 4.5 Run `cargo build -p paladin-llm --features anthropic` and fix any errors (FR-19)
  - [x] 4.6 Run `cargo build -p paladin-llm --features anthropic --no-default-features` and confirm OpenAI symbols are absent from `cargo tree`
  - [x] 4.7 Run `cargo test -p paladin-llm --features anthropic` and confirm all unit tests pass (FR-20)
  - [x] 4.8 Create `crates/paladin-llm/tests/anthropic_integration.rs`; gate with `#![cfg(feature = "anthropic")]`; write at least one compile-time trait-satisfaction test and one `#[ignore]` live-key test (FR-35, FR-36)

- [x] 5.0 Extract the DeepSeek adapter
  - [x] 5.1 Inspect `src/infrastructure/adapters/llm/deepseek_adapter.rs` and determine if it warrants sub-modules; if it is a single file with no sub-modules, use `crates/paladin-llm/src/deepseek.rs`; otherwise create `crates/paladin-llm/src/deepseek/mod.rs` and `adapter.rs` (FR-21)
  - [x] 5.2 Copy `deepseek_adapter.rs` to the target path determined in 5.1; update `use crate::core::...` → `paladin_core::`; update `use paladin_ports::` paths; replace internal error propagation with `LlmProviderError::DeepSeek(...)` variants; apply `.map_err(LlmError::from)` at `impl LlmPort` return sites (FR-21, FR-22)
  - [x] 5.3 Add `#[cfg(feature = "deepseek")] pub mod deepseek;` (or `mod deepseek;` for the flat file form) to `crates/paladin-llm/src/lib.rs` and re-export `DeepSeekAdapter` under `#[cfg(feature = "deepseek")]`
  - [x] 5.4 Run `cargo build -p paladin-llm --features deepseek` and fix any errors (FR-23)
  - [x] 5.5 Run `cargo build -p paladin-llm --features deepseek --no-default-features` and confirm OpenAI and Anthropic symbols are absent
  - [x] 5.6 Run `cargo test -p paladin-llm --features deepseek` and confirm all unit tests pass (FR-24)
  - [x] 5.7 Create `crates/paladin-llm/tests/deepseek_integration.rs`; gate with `#![cfg(feature = "deepseek")]`; write at least one compile-time trait-satisfaction test and one `#[ignore]` live-key test (FR-35, FR-36)

- [x] 6.0 Extract mock adapters
  - [x] 6.1 Resolve PRD Open Question 3 — run `grep -rn "MultiStepMockLlmPort" --include="*.rs" .` across the workspace; if found, record the source location; if not found, note that it must be created as a new type in `crates/paladin-llm/src/mock.rs`
  - [x] 6.2 Copy `src/infrastructure/adapters/llm/mock_llm_adapter.rs` to `crates/paladin-llm/src/mock.rs`; update `use paladin_ports::` imports; confirm zero `reqwest` imports exist in the file (FR-25, FR-26)
  - [x] 6.3 If `MultiStepMockLlmPort` does not yet exist: implement it in `crates/paladin-llm/src/mock.rs` as a struct that accepts a `Vec<String>` of pre-configured responses and returns them in order on each `generate()` call, cycling through the queue (FR-25)
  - [x] 6.4 Add `#[cfg(feature = "mock")] pub mod mock;` to `crates/paladin-llm/src/lib.rs`; re-export `mock::MockLlmPort` and `mock::MultiStepMockLlmPort` at the crate root under `#[cfg(feature = "mock")]` (FR-25)
  - [x] 6.5 Run `cargo build -p paladin-llm --no-default-features --features mock` and confirm success (FR-26)
  - [x] 6.6 Run `cargo tree -p paladin-llm --no-default-features --features mock` and confirm `reqwest` is absent (FR-26)
  - [x] 6.7 Run `cargo test -p paladin-llm --features mock` and confirm all mock unit tests pass (FR-27)
  - [x] 6.8 Create `crates/paladin-llm/tests/mock_integration.rs`; gate with `#![cfg(feature = "mock")]`; write tests covering: (a) `MockLlmPort` queues and dequeues responses in order, (b) `MultiStepMockLlmPort` steps through its configured response sequence, (c) `MockLlmPort` returns a configured `LlmError` when set up to do so (FR-35)

- [x] 7.0 Extract the provider factory
  - [x] 7.1 Resolve PRD Open Question 5 — run `grep -n "application_settings\|ApplicationSettings\|LlmConfig\|LlmSettings" src/infrastructure/adapters/llm/provider_factory.rs`; if any import from the root crate's config system is found, remove it and replace with direct `*Config::from_env()` calls before copying the file (FR-31)
  - [x] 7.2 Copy `src/infrastructure/adapters/llm/provider_factory.rs` to `crates/paladin-llm/src/provider_factory.rs`; update all `use super::...` imports to `use crate::...` paths within `paladin-llm`; gate `#[cfg(feature = "openai")]`, `#[cfg(feature = "anthropic")]`, and `#[cfg(feature = "deepseek")]` arms of `create()` appropriately (FR-28, FR-29)
  - [x] 7.3 Implement `From<ProviderFactoryError> for LlmProviderError` in `provider_factory.rs` (FR-30)
  - [x] 7.4 Ensure `LlmProviderFactory::create()` returns `Err(LlmProviderError::Configuration(...))` for any provider name whose feature flag is not compiled in, rather than relying on `#[cfg]` to simply omit the match arm (which would leave an unreachable match and cause a clippy warning) (FR-29)
  - [x] 7.5 Add `pub mod provider_factory;` to `crates/paladin-llm/src/lib.rs`; re-export `LlmProviderFactory` and `ProviderFactoryError` at the crate root
  - [x] 7.6 Run `cargo build -p paladin-llm --all-features` and fix any errors
  - [x] 7.7 Run `cargo build -p paladin-llm --features mock` (factory alone + mock only) and confirm no compile error
  - [x] 7.8 Create `crates/paladin-llm/tests/provider_factory_test.rs`; write tests: (a) `create("unknown_provider")` returns `Err(LlmProviderError::Configuration(...))`, (b) under `#[cfg(feature = "openai")]`, `create("openai")` returns `Ok(...)` when `OPENAI_API_KEY` is set in the environment (use `#[ignore]` if env is not guaranteed), (c) requesting a disabled provider at runtime returns `Err` not a compile failure (FR-35)
  - [x] 7.9 Run `cargo test -p paladin-llm --all-features` and confirm all provider factory tests pass

- [ ] 8.0 Implement the configuration bridge in the root `paladin` crate
  - [ ] 8.1 Create `src/infrastructure/adapters/llm/config_bridge.rs` in the root `paladin` crate
  - [ ] 8.2 Implement `From<&crate::config::application_settings::LlmProviderConfig> for paladin_llm::openai::adapter::OpenAIConfig` — map `api_key`, `base_url`, `timeout_seconds`, `max_retries`; default `organization` to `None` (FR-32)
  - [ ] 8.3 Implement the equivalent `From<&LlmProviderConfig>` conversions for `paladin_llm::anthropic::adapter::AnthropicConfig` and `paladin_llm::deepseek::*::DeepSeekConfig` (FR-32)
  - [ ] 8.4 Implement vision config conversions — `From<&crate::config::application_settings::VisionConfig> for` the vision adapter config types (one per provider), using the field mapping gaps identified in Task 3.2 (PRD Section 6.5)
  - [ ] 8.5 Add `pub mod config_bridge;` to `src/infrastructure/adapters/llm/mod.rs` (or equivalent location that makes the bridge visible to the root crate's bootstrap code)
  - [ ] 8.6 Update any bootstrap / factory wiring code in the root `paladin` crate that previously constructed LLM adapters directly to use the `config_bridge` conversions (this includes any code in `src/infrastructure/` or `src/application/` that reads `ApplicationSettings.llm.openai` etc. and creates adapters)
  - [ ] 8.7 Write unit tests in `config_bridge.rs` inside a `#[cfg(test)]` block: (a) a typical `LlmProviderConfig` converts to `OpenAIConfig` with all fields correctly mapped, (b) a `LlmProviderConfig` with a non-standard `base_url` correctly overrides the default (FR-34)
  - [ ] 8.8 Run `cargo build --workspace` and confirm no circular dependency error (FR-31); the build must not reference `crate::config::application_settings` from within `paladin-llm`
  - [ ] 8.9 Run `cargo test --workspace` and confirm no regressions

- [ ] 9.0 Wire `paladin-llm` into the root facade crate and `paladin::prelude`
  - [ ] 9.1 Add `paladin-llm` to the root `paladin` crate's `Cargo.toml` `[dependencies]` with `default-features = false` and `features` matching the full set of LLM feature flags the root crate exposes (e.g., `["openai", "anthropic", "deepseek", "mock", "vision"]`) (FR-37)
  - [ ] 9.2 Locate the root crate's `prelude` module (typically `src/prelude.rs` or inline in `src/lib.rs`) and add feature-gated re-exports for: `paladin_llm::OpenAIAdapter`, `paladin_llm::openai::embedding::OpenAIEmbeddingAdapter`, `paladin_llm::AnthropicAdapter`, `paladin_llm::DeepSeekAdapter`, `paladin_llm::mock::MockLlmPort`, `paladin_llm::mock::MultiStepMockLlmPort`, `paladin_llm::LlmProviderFactory`, `paladin_llm::LlmProviderError` (FR-38)
  - [ ] 9.3 Delete the original source files that have now been fully replaced: `src/infrastructure/adapters/llm/openai_adapter.rs`, `src/infrastructure/adapters/llm/openai_embedding_adapter.rs`, `src/infrastructure/adapters/llm/openai_vision.rs`, `src/infrastructure/adapters/llm/anthropic_adapter.rs`, `src/infrastructure/adapters/llm/anthropic_vision.rs`, `src/infrastructure/adapters/llm/deepseek_adapter.rs`, `src/infrastructure/adapters/llm/mock_llm_adapter.rs`, `src/infrastructure/adapters/llm/provider_factory.rs` (FR-40)
  - [ ] 9.4 Update `src/infrastructure/adapters/llm/mod.rs` — remove all `pub mod` declarations for deleted files; if `config_bridge` is the only remaining module, leave only `pub mod config_bridge;`; add a tombstone comment explaining that adapter implementations moved to `paladin-llm` (FR-40)
  - [ ] 9.5 Run `cargo build --workspace` and fix any broken imports in the root crate that still referenced the deleted files (FR-46)
  - [ ] 9.6 Run the isolated feature-flag build matrix to confirm all six FR-41–FR-46 builds pass:
        `cargo build -p paladin-llm --no-default-features`
        `cargo build -p paladin-llm --features openai`
        `cargo build -p paladin-llm --features anthropic`
        `cargo build -p paladin-llm --features deepseek`
        `cargo build -p paladin-llm --features mock`
        `cargo build -p paladin-llm --all-features`
  - [ ] 9.7 Run `cargo test --workspace` and confirm the test count matches or exceeds the pre-epic baseline captured in Task 1.1 (FR-47)

- [ ] 10.0 Import path sweep — update all workspace examples, tests, and benchmarks
  - [ ] 10.1 Run `grep -rn "infrastructure::adapters::llm\|adapters::llm::\|openai_adapter\|anthropic_adapter\|deepseek_adapter\|mock_llm_adapter\|provider_factory" --include="*.rs" examples/ tests/ benches/` and record every hit
  - [ ] 10.2 For each file found in 10.1: replace old deep import paths with `use paladin::prelude::*;` or the direct `use paladin_llm::...;` path as appropriate; ensure every file compiles (FR-39)
  - [ ] 10.3 Run `grep -rn "infrastructure::adapters::llm\|adapters::llm::" --include="*.rs" src/` to catch any remaining references inside the root crate itself (e.g., in integration glue code, web handlers, or CLI); update each
  - [ ] 10.4 Run `cargo check --workspace --all-targets` and confirm zero compile errors across all targets (examples, tests, benchmarks, bins)
  - [ ] 10.5 Run `cargo test --workspace` and confirm zero test regressions (FR-47)

- [ ] 11.0 Workspace build validation and quality gates
  - [ ] 11.1 Run `cargo clippy -p paladin-llm --all-features -- -D warnings` and fix every warning to zero (FR-48)
  - [ ] 11.2 Run `cargo fmt --check -p paladin-llm`; if it fails run `cargo fmt -p paladin-llm` then re-check until clean (FR-49)
  - [ ] 11.3 Run `cargo doc -p paladin-llm --all-features --no-deps 2>&1 | grep -i "warning\|error"` and resolve all broken intra-doc links and missing doc warnings
  - [ ] 11.4 Run `cargo test --workspace --all-features` and confirm all tests pass; record the final test count
  - [ ] 11.5 Run `cargo tree -p paladin-llm --features openai 2>&1 | grep -i "anthropic\|deepseek"` — the output must be empty, confirming provider isolation
  - [ ] 11.6 Run `cargo tree -p paladin-llm --no-default-features --features mock 2>&1 | grep "reqwest"` — the output must be empty, confirming `reqwest` is not pulled in for mock-only builds (FR-26)
  - [ ] 11.7 Run `make clean-code` (format + lint + check) and `make audit` to confirm no new security advisories were introduced by the new dependencies
  - [ ] 11.8 Remove any temporary debug output (`dbg!`, `println!`, stray `// TODO` or `// TEMP` comments) from all files touched in this epic
  - [ ] 11.9 Stage all changes: `git add .`
  - [ ] 11.10 Commit with a conventional-commit message: `git commit -m "refactor: extract paladin-llm as workspace crate with per-provider feature flags" -m "- Add crates/paladin-llm with openai/anthropic/deepseek/mock/vision features" -m "- Define LlmProviderError with From<LlmProviderError> for LlmError conversion" -m "- Relocate all LLM adapter source from src/infrastructure/adapters/llm/" -m "- Extract provider factory; add config bridge in root paladin crate" -m "- Wire adapters into paladin::prelude; remove old deep import paths" -m "Implements Milestone 5 Epic 4 (FR-1 through FR-49)"`
  - [ ] 11.11 Push the branch: `git push`
  - [ ] 11.12 Open a pull request targeting the milestone integration branch titled `refactor(milestone-5/epic-4): extract paladin-llm crate`, linking this task list and the PRD in the description
