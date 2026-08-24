---
phase: 17-additional-llm-provider-adapters
plan: 01
subsystem: api
tags: [llm, rust, kimi, moonshot, openai-compatible, mockito, reqwest, tokio]

# Dependency graph
requires: []
provides:
  - "crates/paladin-llm/src/redaction.rs — crate-level, ungated credential-redaction trio (redact_credentials, bounded_excerpt, diagnostic_excerpt) reusable by any future provider, including the bespoke Gemini adapter in plan 17-05"
  - "crates/paladin-llm/src/compat/{mod,types,engine}.rs — CompatEngine, the shared OpenAI-compatible protocol core (D-05) that every remaining named preset (Qwen, Grok, Ollama) and the generic openai-compatible provider sit on"
  - "crates/paladin-llm/src/kimi/{mod,adapter}.rs — KimiAdapter, the first preset proving the engine end-to-end"
  - "crates/paladin-llm/src/provider_factory.rs — ProviderRegistration table-driven registry (D-10) every later plan's provider row registers through"
affects: [17-02, 17-03, 17-04, 17-05, 17-06, 17-07]

# Tech tracking
tech-stack:
  added: [mockito 1.7.0 (dev-dependency, paladin-llm)]
  patterns:
    - "Shared protocol core + thin preset (CompatEngine / KimiAdapter) — the extraction pattern every remaining Phase 17 OpenAI-compatible provider follows"
    - "cfg-gated table-driven registry (ProviderRegistration) — the single lookup mechanism every provider registers a row through"
    - "tokio::sync::OnceCell-memoized live-model-list-with-curated-fallback (D-13/D-14)"

key-files:
  created:
    - crates/paladin-llm/src/redaction.rs
    - crates/paladin-llm/src/compat/mod.rs
    - crates/paladin-llm/src/compat/types.rs
    - crates/paladin-llm/src/compat/engine.rs
    - crates/paladin-llm/src/kimi/mod.rs
    - crates/paladin-llm/src/kimi/adapter.rs
  modified:
    - crates/paladin-llm/src/lib.rs
    - crates/paladin-llm/src/provider_factory.rs
    - crates/paladin-llm/Cargo.toml

key-decisions:
  - "Fixed the SSE stream-assembly loop to process every `data:` line per network chunk (flat_map) instead of returning after the first match, which the copied deepseek/adapter.rs pattern does — see Deviations."
  - "The 20-concurrent-create(\"kimi\")-with-MOONSHOT_API_KEY-set acceptance variant is deferred to 17-07's workspace-level test target per the plan's own explicit instruction (env::set_var is unsafe under Rust 2024; this crate denies unsafe_code); substituted a concurrency test against an unregistered provider name that still proves the OnceLock registry resolves safely under concurrent readers."
  - "Kimi's base URL/default model/fallback IDs could not be live-verified against platform.moonshot.ai in this execution environment (no network egress) — recorded honestly below rather than fabricating a confirmation; RESEARCH.md's own Assumption A1 already carries this exact risk disposition."

patterns-established:
  - "compat/ module: engine.rs owns protocol mechanics, types.rs owns wire shapes, a preset's adapter.rs owns only base_url/env-var/model-defaults/capabilities and delegates every LlmPort method to an owned CompatEngine"
  - "ProviderRegistration { name, env_var, construct } row added to build_provider_registry() is the only way a provider becomes visible to create()/get_default_provider()/list_available_providers()/provider_names()"

requirements-completed: [PROV-02, PROV-03, PROV-04]

coverage:
  - id: D1
    description: "Shared OpenAI-compatible protocol engine (CompatEngine) extracted from deepseek/adapter.rs: request shaping, HTTP transport, retry-with-backoff, status-to-LlmError mapping, credential redaction, memoized model-list resolution"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/compat/engine.rs#tests (12 tests: map_finish_reason x4, detect_empty_completion x3, map_error x2, capabilities, retry x2, construction)"
        status: pass
    human_judgment: false
  - id: D2
    description: "KimiAdapter implements all six LlmPort methods with no stub, delegating to CompatEngine; get_provider_name() returns \"kimi\"; get_capabilities() reports no tool/function calling"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/kimi/adapter.rs#tests (mockito-backed: request shaping, response parsing, finish-reason mapping, null-content tolerance, empty-choices, error-code mapping x5, redaction-under-error, streaming assembly x2, capabilities, provider name, retry x2 — 32 tests total)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Credential redaction (redact-before-truncate ordering, char-not-byte budget) shared crate-level for reuse by the bespoke Gemini adapter (plan 17-05)"
    requirement: "PROV-04"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/redaction.rs#tests (9 tests, including a key-straddling-the-budget-boundary regression test)"
        status: pass
    human_judgment: false
  - id: D4
    description: "Table-driven provider registry (ProviderRegistration) replaces provider_factory.rs's hardcoded match; get_default_provider()/list_available_providers() can no longer report a compiled-out provider as available"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/provider_factory.rs#tests (10 tests under --features kimi, 6 under default features: no-duplicate-names, list-only-contains-registry-names, kimi-only-build exact-list/case-insensitivity/message-content, concurrent-create)"
        status: pass
    human_judgment: false
  - id: D5
    description: "Kimi vendor facts (base URL, default model, fallback model IDs) confirmed against platform.moonshot.ai before commit"
    verification: []
    human_judgment: true
    rationale: "This execution environment has no network egress to verify vendor documentation. RESEARCH.md's Assumption A1 already records this exact risk as accepted (low-impact, since D-13's live-fetch-with-fallback design tolerates a stale curated list). A human (or a future environment with network access) should confirm https://platform.moonshot.ai/docs before this preset is used against a live key."

# Metrics
duration: ~50min (session start not explicitly timestamped; Task 2 commit landed 6min after Task 1 commit, 2026-08-17T02:22:42Z to 02:28:27Z)
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 01: Shared OpenAI-Compatible Engine + Kimi Preset Summary

**Shared `CompatEngine` OpenAI-compatible protocol core (extracted from `deepseek/adapter.rs`) proven end-to-end by a Kimi (Moonshot AI) preset, plus a `cfg`-gated table-driven provider registry replacing `provider_factory.rs`'s hardcoded match.**

## Performance

- **Duration:** ~50 min (approximate — see frontmatter note)
- **Completed:** 2026-08-17T02:28:27Z (Task 2 commit)
- **Tasks:** 2 completed
- **Files modified:** 9 (7 new, 2 modified in Task 1; 1 modified in Task 2 — `provider_factory.rs` is counted once)

## Accomplishments

- Built `CompatEngine`/`CompatEngineConfig`/`CompatCapabilities` (`crates/paladin-llm/src/compat/`) — the shared OpenAI-compatible protocol core every remaining Phase 17 provider (Qwen, Grok, Ollama, the generic `openai-compatible` provider) will sit on. Request shaping, HTTP transport (body-read-then-parse, never `Response::json()`), SSE stream assembly, exponential-backoff retry, status-to-`LlmError` mapping with a per-preset override hook, and `tokio::sync::OnceCell`-memoized live-model-list-with-curated-fallback (D-13/D-14).
- Built `KimiAdapter`/`KimiConfig` (`crates/paladin-llm/src/kimi/`) — the first preset on the engine, implementing all six `LlmPort` methods by delegation, proving the engine's `(base_url, api_key, model, capabilities)` parameterisation end-to-end against a `mockito` mock transport (81 tests green under `--features kimi`, zero live API key required).
- Extracted `crates/paladin-llm/src/redaction.rs` — the crate-level, ungated credential-redaction trio (`redact_credentials`, `bounded_excerpt`, `diagnostic_excerpt`) lifted from `deepseek/adapter.rs`, reusable by the compatible core and by the bespoke Gemini adapter (plan 17-05) alike.
- Replaced `provider_factory.rs`'s four hand-maintained lookup sites (the `create()` match, the un-`cfg`-gated `get_default_provider()`, the un-`cfg`-gated `list_available_providers()`, the hardcoded `UnknownProvider` prose) with one `cfg`-gated `ProviderRegistration` table (D-10) — structurally removing the defect where a compiled-out provider could be reported as available.
- `openai/`, `anthropic/`, `deepseek/` are byte-unchanged (D-06 — verified via empty `git diff --stat`).

## Task Commits

1. **Task 1: End-to-end "Kimi generates text" — one path through the compatible core** - `3cd31a4` (feat, tracer)
2. **Task 2: Replace the factory's hardcoded match with one cfg-gated registry table (D-10)** - `6f68a11` (refactor)

_Note: this plan's tasks are `type="tracer"` (Task 1) and `type="auto"` (Task 2), neither TDD-gated at the plan level, so each landed as a single commit rather than a RED/GREEN/REFACTOR sequence. Tests were written alongside each task's implementation per the plan's `tdd="true"` task attribute._

## Files Created/Modified

- `crates/paladin-llm/src/redaction.rs` - Credential redaction trio + null-tolerant deserializer, ungated
- `crates/paladin-llm/src/compat/mod.rs` - Module root re-exporting `CompatEngine`/`CompatEngineConfig`/`CompatCapabilities`
- `crates/paladin-llm/src/compat/types.rs` - Generalized OpenAI-compatible wire types (`CompatRequest`, `CompatMessage`, `CompatResponse`, streaming/model-list shapes)
- `crates/paladin-llm/src/compat/engine.rs` - `CompatEngine`: request build, retry, error mapping, streaming, memoized model list
- `crates/paladin-llm/src/kimi/mod.rs` - Module re-export shape mirroring `deepseek/mod.rs`
- `crates/paladin-llm/src/kimi/adapter.rs` - `KimiConfig`/`KimiAdapter`, 32 mockito-backed tests
- `crates/paladin-llm/src/lib.rs` - Declares `redaction` (ungated) and `compat`/`kimi` (feature-gated) modules; adds kimi doc-table row
- `crates/paladin-llm/src/provider_factory.rs` - `ProviderRegistration` table replaces the hardcoded match (D-10)
- `crates/paladin-llm/Cargo.toml` - Adds `kimi` feature (`["dep:reqwest", "dep:rand"]`) and `mockito = "1.7.0"` dev-dependency; default feature set unchanged (PROV-03)

## Decisions Made

- **Engine error-override hook, not a hardcoded per-status arm:** `CompatEngineConfig.error_override: Option<fn(u16, &str) -> Option<LlmError>>` lets a future preset (e.g. one with its own 402 semantics like DeepSeek) add a status-code arm without editing the shared engine. Consulted before the engine's default mapping, tested via `map_error_consults_override_before_default_mapping`.
- **`KimiConfig::from_parts` extracted as pure defaulting logic**, separate from `from_env()`, so `MOONSHOT_API_KEY`-absent and defaults-when-only-key-set behavior is testable without `std::env::set_var` (unsafe under Rust 2024; this crate denies `unsafe_code`).
- **`ProviderRegistration` kept private** (not `pub`) — only `provider_names()` is public surface; the row shape is an internal implementation detail no external consumer needs to construct directly.
- **`total_tokens` computed as `prompt_tokens + completion_tokens` when the provider's `usage` object omits it** (PROV-02 precision edge), rather than reporting zero — tested via `generate_computes_total_tokens_when_provider_omits_it`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] SSE stream assembly only processed the first `data:` line per network chunk**
- **Found during:** Task 1, writing `generate_stream_assembles_deltas_in_wire_order_with_terminal_stop`
- **Issue:** The plan's `<action>` directs copying `deepseek/adapter.rs:736-785`'s SSE assembly loop verbatim. That loop iterates `text.lines()` inside a `.map()` closure and `return`s on the FIRST matching `data:` line, discarding every subsequent event in the same `bytes_stream()` chunk. This is a latent bug in the copied source: it happens not to manifest for DeepSeek's existing tests (which don't exercise a multi-event single chunk), but the plan's own required behavior — "an SSE body of three `data: {...}` chunks... yields deltas in wire order whose concatenation equals the full text" — fails against a mock transport that serves the whole body as one `bytes_stream()` item (and is plausible in production depending on TCP framing).
- **Fix:** Restructured `CompatEngine::generate_stream()` to use `.flat_map()` instead of `.map()`: every `data:` line found in a chunk is now pushed onto a `Vec<Result<StreamingResponse, LlmError>>` and flattened via `futures::stream::iter`, so no event is silently dropped regardless of how many SSE frames land in one network read.
- **Files modified:** `crates/paladin-llm/src/compat/engine.rs`
- **Verification:** `generate_stream_assembles_deltas_in_wire_order_with_terminal_stop` and `generate_stream_with_only_done_terminates_with_stop_and_no_error` both pass; full 81-test suite green under `--features kimi`.
- **Committed in:** `3cd31a4` (Task 1 commit)

**2. [Rule 3 - Blocking] `Box<dyn Stream<...> + Send>` is not directly consumable via `StreamExt::next()`**
- **Found during:** Task 1, writing the streaming tests
- **Issue:** `futures::Stream` is implemented for `Box<S>` only when the inner `S: Unpin`; a `dyn Stream + Send` trait object is not `Unpin`, so `stream.next().await` on the boxed value the port trait returns fails to compile with `the trait Unpin is not implemented for dyn Stream<...>`.
- **Fix:** `Box::into_pin(stream)` before consuming — sound unconditionally (Box owns its heap allocation), and does not require the inner type to be `Unpin`.
- **Files modified:** `crates/paladin-llm/src/kimi/adapter.rs` (test code only)
- **Verification:** Streaming tests compile and pass.
- **Committed in:** `3cd31a4` (Task 1 commit)

**3. [Rule 3 - Blocking] Two clippy lints under `-D warnings`**
- **Found during:** Task 1/2, running `cargo clippy -p paladin-llm --no-default-features --features kimi -- -D warnings`
- **Issue:** (a) `#[cfg(any(feature = "kimi"))]` in `lib.rs` trips `clippy::non_minimal_cfg` (single-condition `any()`); (b) a nested `if let Some(...) { if let Some(...) { ... } }` in `CompatEngine::map_error` trips `clippy::collapsible_if`; (c) the `cfg`-gated `Vec::push` sequence building the registry table in `provider_factory.rs` trips `clippy::vec_init_then_push` (clippy cannot see that the push count varies per build).
- **Fix:** (a) simplified to a plain `feature = "kimi"` cfg with a doc comment explaining when to widen to `any(...)`; (b) rewritten using an `if let ... && let ...` chain; (c) extracted into its own `#[allow(clippy::vec_init_then_push)]`-annotated `build_provider_registry()` function with a doc comment explaining why the lint doesn't apply.
- **Files modified:** `crates/paladin-llm/src/lib.rs`, `crates/paladin-llm/src/compat/engine.rs`, `crates/paladin-llm/src/provider_factory.rs`
- **Verification:** `cargo clippy -p paladin-llm --no-default-features --features kimi -- -D warnings` and `cargo clippy -p paladin-llm -- -D warnings` both exit 0.
- **Committed in:** `3cd31a4` and `6f68a11`

---

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** The stream-assembly fix (#1) is a genuine correctness improvement over the copied source pattern — it is the reason the plan's own required streaming-order test exists. The other two are mechanical compile/lint fixes with no behavioral consequence. No scope creep.

## Issues Encountered

- **Kimi vendor facts not live-verified.** Per D-00e and the plan's `<action>` instruction, the Kimi base URL (`https://api.moonshot.ai/v1`), default model (`moonshot-v1-8k`) and fallback model IDs were to be "confirmed against the cited vendor doc before committing." This execution environment has no network egress, so no live confirmation was possible. The values used are exactly RESEARCH.md's `[CITED: platform.moonshot.ai]` figures, and RESEARCH.md's own Assumption A1 already records this as an accepted, low-impact risk (D-13's live-fetch-with-fallback design is specifically built to tolerate a stale curated list). Recorded as `human_judgment: true` coverage item D5 above rather than silently asserted as verified.
- **Task 2's "20 concurrent `create("kimi")` calls ... with `MOONSHOT_API_KEY` present in the test process" acceptance variant was not implemented in-crate**, per the plan's own explicit text in Task 2's `<action>`: this crate denies `unsafe_code` and `std::env::set_var` is `unsafe` under Rust 2024, so env-var-mutating tests are deferred to the workspace-level test target plan 17-07 owns. Substituted a concurrency test (`create_is_safe_to_call_concurrently`) that spawns 20 concurrent `create()` calls against an unregistered provider name, which still proves the `OnceLock`-memoized registry resolves safely under concurrent readers — the structural property D-10 depends on — without touching process environment.

## User Setup Required

None - no external service configuration required. (A `MOONSHOT_API_KEY` would be required to actually call Kimi's live API, but no test in this plan requires one — all 32 Kimi-adapter tests run offline against `mockito`.)

## Next Phase Readiness

- The `compat::CompatEngine`/`CompatCapabilities`/`CompatEngineConfig` API surface is now proven and ready for plans 17-03/17-04 (Qwen, Grok, Ollama, the generic `openai-compatible` provider) to build thin presets against with no further engine changes expected.
- The `ProviderRegistration` table shape (with `env_var: Option<&'static str>`) already accommodates Ollama's no-credential-required row (D-12), so plan 17-03 does not need to rework the struct.
- `crate::redaction` is ready for plan 17-05's bespoke Gemini adapter to reuse directly (ungated, no `compat::` dependency required).
- No blockers. The one open item (Kimi vendor-fact live verification) is low-risk and explicitly flagged for human/future-environment follow-up rather than blocking.

## Self-Check: PASSED

All created files verified present on disk; both task commit hashes (`3cd31a4`, `6f68a11`) verified present in `git log`.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*
