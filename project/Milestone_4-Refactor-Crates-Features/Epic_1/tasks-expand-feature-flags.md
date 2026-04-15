# Task List: Expand Feature Flags to Gate the Full Optional Surface

**Epic:** Milestone 4, Epic 1
**PRD:** `prd-expand-feature-flags.md`
**Status:** In Progress

---

## Relevant Files

### Modified Files
- `Cargo.toml` — Add `optional = true` to gated deps; update `[features]` section with all new flags
- `src/infrastructure/adapters/llm/mod.rs` — Add `#[cfg]` guards on OpenAI, Anthropic, DeepSeek, and vision adapter module declarations
- `src/infrastructure/adapters/llm/provider_factory.rs` — Gate each provider import and factory `match` arm behind its feature flag
- `src/infrastructure/adapters/output/openai_llm_adapter.rs` — **Removed** (legacy adapter superseded by `llm/openai_adapter.rs`; `#[cfg]` gate in `output/mod.rs` also removed)
- `src/infrastructure/adapters/mod.rs` — Gate `document`, `notifications`, and `arsenal` module declarations
- `src/infrastructure/adapters/output/mod.rs` — Gate `api_content_deliverer` module (uses `actix-web`) behind `web-server`
- `src/infrastructure/mod.rs` — Gate `web` module declaration behind `web-server`
- `src/infrastructure/adapters/garrison/token_counter.rs` — Gate `tiktoken_rs` import behind `content-processing`
- `src/application/ports/output/mod.rs` — Gate `vision_port` and `vision_llm_port` module declarations behind `vision`
- `src/config/application_settings.rs` — Gate `MinioConfig` import and `to_minio_config()` behind `s3-storage`
- `src/config/setup/service_runner.rs` — Gate `RedisQueueAdapter`/`MinioAdapter` imports, struct fields, init blocks, and accessor methods behind `redis-queue`/`s3-storage`
- `src/infrastructure/adapters/llm/provider_factory.rs` (tests) — Gate provider-specific tests behind their respective LLM feature flags
- `tests/integration/mod.rs` — Gate provider/storage/vision integration test module declarations behind their feature flags
- `src/application/use_cases/paladin/paladin_execution_service.rs` — Change vision doctest from `no_run` to `ignore` (depends on gated `openai_adapter`)
- `src/application/cli/commands/agent.rs` — Gate `DocumentAdapter` import and document processing block behind `content-processing`

### New Files
- `docs/FEATURE_FLAGS.md` — Comprehensive documentation for all feature flags with usage examples
- `docs/MIGRATION.md` — Breaking change migration guide for consumers updating from old defaults
- `.github/workflows/feature-flags.yml` — CI workflow for feature flag matrix testing

### Updated Documentation Files
- `docs/CONFIGURATION.md` — Add feature flags section
- `CHANGELOG.md` — Breaking change notice for default feature set change
- `README.md` — Add feature flags table

### Notes
- Unit tests in Rust go in a `#[cfg(test)]` module inside the same file.
- Use `cargo check --features <flag>` to quickly verify a feature compiles without running tests.
- When a module is gated behind `#[cfg(feature = "...")]`, all code that imports from it must also be guarded.
- `reqwest` stays as a **core** (always-compiled) dependency because all LLM providers share it.
- `chacha20poly1305` and `zeroize` are used in `src/infrastructure/security/encryption.rs` for general encryption, **not** vision — they remain core dependencies.
- The `openai_embedding_adapter.rs` is controlled by the existing `openai-embeddings` flag, not `llm-openai`.
- `vision = []` was added to `[features]` early (pulled forward from Task 6.1) to avoid `unexpected_cfgs` warnings since vision guards were applied in Task 2.
- Integration test modules in `tests/integration/mod.rs` must also be gated when they import from feature-gated adapter modules.
- Vision examples (`vision_analysis`, `vision_battalion`) and test targets that depend on gated adapters need `required-features` in `Cargo.toml`.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update after each **sub-task**, not just parent tasks.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file`

Run the following after every parent task before marking the parent `[x]`:
```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
git add .
git commit -m "..."
```

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/epic-1-feature-flags`

- [x] 1.0 Audit and classify all Cargo.toml dependencies
  - [x] 1.1 Create `project/Milestone_4-Refactor-Crates-Features/Epic_1/dependency-matrix.md` with a Markdown table classifying every `[dependencies]` entry as "core" (always compiled) or "optional" (gated), and map each optional dep to its proposed feature flag
  - [x] 1.2 Run `cargo tree --edges normal 2>/dev/null | head -80` to inspect the full dependency graph and verify no surprise transitive deps are pulled in unconditionally
  - [x] 1.3 Confirm the following deps will be marked `optional = true` in Cargo.toml: `actix-web`, `axum`, `lettre`, `pdf-extract`, `scraper`, `tiktoken-rs`, `rss`
  - [x] 1.4 Note that `reqwest`, `chacha20poly1305`, and `zeroize` remain core (unconditional) and document the rationale in the matrix

- [x] 2.0 Implement LLM Provider feature flags (`llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all`)
  - [x] 2.1 In `Cargo.toml` `[features]`, add: `llm-openai = []`, `llm-anthropic = []`, `llm-deepseek = []`, `llm-all = ["llm-openai", "llm-anthropic", "llm-deepseek"]`
  - [x] 2.2 In `src/infrastructure/adapters/llm/mod.rs`, wrap the `openai_adapter`, `openai_vision`, and `openai_embedding_adapter` module declarations with `#[cfg(feature = "llm-openai")]`
  - [x] 2.3 In `src/infrastructure/adapters/llm/mod.rs`, wrap the `anthropic_adapter` and `anthropic_vision` module declarations with `#[cfg(feature = "llm-anthropic")]`
  - [x] 2.4 In `src/infrastructure/adapters/llm/mod.rs`, wrap the `deepseek_adapter` module declaration with `#[cfg(feature = "llm-deepseek")]`
  - [x] 2.5 In `src/infrastructure/adapters/llm/provider_factory.rs`, add `#[cfg(feature = "llm-openai")]` guards around the `use super::openai_adapter::...` import and the `"openai"` match arm; repeat for anthropic and deepseek
  - [x] 2.6 Add a fallback `_ => Err(ProviderFactoryError::UnknownProvider(...))` in `provider_factory.rs` so the match is exhaustive when one or more providers are compiled out
  - [x] 2.7 ~~In `src/infrastructure/adapters/output/mod.rs`, wrap the `openai_llm_adapter` module declaration with `#[cfg(feature = "llm-openai")]`~~ — **N/A:** `openai_llm_adapter.rs` was removed (superseded by `llm/openai_adapter.rs`); `#[cfg]` gate in `output/mod.rs` also removed
  - [x] 2.8 Run `cargo check --no-default-features --features llm-openai` and fix any errors
  - [x] 2.9 Run `cargo check --no-default-features --features llm-anthropic` and fix any errors
  - [x] 2.10 Run `cargo check --no-default-features --features llm-deepseek` and fix any errors
  - [x] 2.11 Run `cargo check --no-default-features` (no LLM provider) and confirm it passes
  - [x] 2.12 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit

- [x] 3.0 Implement Content Processing feature flag (`content-processing`)
  - [x] 3.1 In `Cargo.toml` `[dependencies]`, mark `pdf-extract`, `scraper`, `tiktoken-rs`, and `rss` as `optional = true`
  - [x] 3.2 In `Cargo.toml` `[features]`, add: `content-processing = ["pdf-extract", "scraper", "tiktoken-rs", "rss"]`
  - [x] 3.3 In `src/infrastructure/adapters/mod.rs`, wrap the `document` module declaration with `#[cfg(feature = "content-processing")]`
  - [x] 3.4 In `src/infrastructure/adapters/garrison/token_counter.rs`, wrap the `use tiktoken_rs::...` import and the `TokenCounter` implementation in `#[cfg(feature = "content-processing")]`
  - [x] 3.5 In `src/infrastructure/adapters/garrison/mod.rs`, gate the `token_counter` module declaration with `#[cfg(feature = "content-processing")]`
  - [x] 3.6 Search for any other files referencing `pdf_extract::`, `scraper::`, or `rss::` with `grep -r "pdf_extract\|scraper::\|rss::" src/` and add appropriate `#[cfg]` guards
  - [x] 3.7 Run `cargo check --no-default-features` and confirm content processing deps are not included
  - [x] 3.8 Run `cargo check --no-default-features --features content-processing` and confirm it compiles
  - [x] 3.9 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit

- [x] 4.0 Implement Web Server feature flag (`web-server`)
  - [x] 4.1 In `Cargo.toml` `[dependencies]`, mark `actix-web` and `axum` as `optional = true`
  - [x] 4.2 In `Cargo.toml` `[features]`, add: `web-server = ["actix-web", "axum"]`
  - [x] 4.3 In `src/infrastructure/mod.rs`, wrap the `pub mod web;` declaration with `#[cfg(feature = "web-server")]`
  - [x] 4.4 In `src/infrastructure/adapters/output/mod.rs`, wrap `pub mod api_content_deliverer;` with `#[cfg(feature = "web-server")]`
  - [x] 4.5 Run `cargo check --no-default-features` to confirm actix-web and axum are excluded
  - [x] 4.6 Run `cargo check --no-default-features --features web-server` to confirm the module compiles with the flag
  - [x] 4.7 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit

- [x] 5.0 Implement Notifications feature flag (`notifications`)
  - [x] 5.1 In `Cargo.toml` `[dependencies]`, mark `lettre` as `optional = true`; assess whether `handlebars` is also exclusively used for notifications (if so, mark it optional too)
  - [x] 5.2 In `Cargo.toml` `[features]`, add: `notifications = ["lettre"]` (add `"handlebars"` if applicable)
  - [x] 5.3 In `src/infrastructure/adapters/mod.rs`, wrap `pub mod notifications;` with `#[cfg(feature = "notifications")]`
  - [x] 5.4 Run `cargo check --no-default-features` to confirm lettre is excluded
  - [x] 5.5 Run `cargo check --no-default-features --features notifications` to confirm notifications compile
  - [x] 5.6 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit

- [x] 6.0 Implement Vision feature flag (`vision`)
  - [x] 6.1 In `Cargo.toml` `[features]`, add: `vision = []` (no additional deps needed; vision adapters use reqwest which is core)
  - [x] 6.2 In `src/infrastructure/adapters/llm/mod.rs`, wrap `pub mod openai_vision;` with `#[cfg(all(feature = "vision", feature = "llm-openai"))]` (requires both flags)
  - [x] 6.3 In `src/infrastructure/adapters/llm/mod.rs`, wrap `pub mod anthropic_vision;` with `#[cfg(all(feature = "vision", feature = "llm-anthropic"))]`
  - [x] 6.4 In `src/application/ports/output/mod.rs`, wrap `pub mod vision_llm_port;` and `pub mod vision_port;` with `#[cfg(feature = "vision")]`
  - [x] 6.5 In `Cargo.toml` `[[test]]` for `vision_integration`, add `required-features = ["vision", "llm-openai"]`
  - [x] 6.6 Run `cargo check --no-default-features` to confirm vision is excluded
  - [x] 6.7 Run `cargo check --no-default-features --features "vision,llm-openai"` to confirm vision+openai compiles
  - [x] 6.8 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit

- ~~[ ] 7.0 Implement MCP Arsenal feature flag (`mcp-arsenal`)~~ **ELIMINATED** — Deemed unnecessary complexity; Arsenal remains unconditionally compiled as a core framework component
  - ~~[ ] 7.1 In `Cargo.toml` `[features]`, add: `mcp-arsenal = []` (all MCP deps are pure Rust; no external crate to mark optional)~~
  - ~~[ ] 7.2 In `src/infrastructure/adapters/arsenal/mod.rs`, wrap `pub mod mcp_protocol;`, `pub mod mcp_sse_adapter;`, `pub mod mcp_stdio_adapter;`, `pub mod resource_controls;`, and `pub mod tool_result_formatter;` with `#[cfg(feature = "mcp-arsenal")]`~~
  - ~~[ ] 7.3 In `src/infrastructure/adapters/mod.rs`, wrap `pub mod arsenal;` with `#[cfg(feature = "mcp-arsenal")]`~~
  - ~~[ ] 7.4 Run `cargo check --no-default-features` to confirm arsenal is excluded~~
  - ~~[ ] 7.5 Run `cargo check --no-default-features --features mcp-arsenal` to confirm arsenal compiles~~
  - ~~[ ] 7.6 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit~~

- [ ] 8.0 Revise default feature set and add `full` convenience flag
  - [ ] 8.1 In `Cargo.toml` `[features]`, change `default = ["redis-queue", "s3-storage", "openai-embeddings"]` to `default = ["llm-openai"]`
  - [ ] 8.2 Add `full = ["llm-all", "content-processing", "web-server", "notifications", "vision", "redis-queue", "s3-storage", "openai-embeddings", "qdrant"]`
  - [ ] 8.3 Run `cargo build` (default features) and confirm it compiles cleanly with only `llm-openai`
  - [ ] 8.4 Run `cargo build --no-default-features` and confirm the core-only build succeeds
  - [ ] 8.5 Run `cargo build --all-features` and confirm the full build succeeds
  - [ ] 8.6 Run `cargo build --features full` and confirm the `full` convenience flag also works
  - [ ] 8.7 Run `cargo test` under each of the three main combinations (default, no-default, all-features)
  - [ ] 8.8 Run `cargo fmt --check`, `cargo clippy -- -D warnings`; commit

- [ ] 9.0 Configure CI feature flag matrix
  - [ ] 9.1 Create `.github/workflows/feature-flags.yml`
  - [ ] 9.2 Define a `strategy.matrix` with entries for: `--no-default-features`, default (no flags arg), `--all-features`, `--features llm-openai`, `--features llm-anthropic`, `--features llm-deepseek`, `--features web-server`, `--features content-processing`
  - [ ] 9.3 Each matrix entry should run: `cargo check`, `cargo build`, and `cargo test` (use `cargo test --no-run` for expensive test builds if needed)
  - [ ] 9.4 Verify the YAML file is syntactically valid (use `yamllint` or the GitHub Actions schema)
  - [ ] 9.5 Commit and confirm the workflow appears in the `.github/workflows/` directory

- [ ] 10.0 Update documentation and examples
  - [ ] 10.1 Create `docs/FEATURE_FLAGS.md` with: a table of all flags, their gated dependencies, their gated modules, example `Cargo.toml` snippets, and a "minimal build" vs "full build" comparison
  - [ ] 10.2 Create `docs/MIGRATION.md` explaining the breaking change: old default was `["redis-queue", "s3-storage", "openai-embeddings"]`, new default is `["llm-openai"]`; provide the exact line to add to `Cargo.toml` to restore old behavior
  - [ ] 10.3 Update `docs/CONFIGURATION.md` — add a "Feature Flags" section linking to `FEATURE_FLAGS.md` and summarizing the available flags
  - [ ] 10.4 Update `CHANGELOG.md` at the top with a `## [Unreleased]` section noting the breaking change in default features
  - [ ] 10.5 Update `README.md` — add a "Feature Flags" table after the installation section listing each flag and its purpose
  - [ ] 10.6 Review `/examples/` — update any examples that relied on `redis-queue` or `s3-storage` being default by adding explicit feature annotations in their doc comments
  - [ ] 10.7 Run `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`; commit
