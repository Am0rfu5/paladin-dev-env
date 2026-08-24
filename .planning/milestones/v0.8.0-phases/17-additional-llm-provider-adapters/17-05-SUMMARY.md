---
phase: 17-additional-llm-provider-adapters
plan: 05
subsystem: api
tags: [llm, rust, gemini, google, bespoke-protocol, mockito, reqwest, tokio, sse]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters (plan 01)
    provides: "crates/paladin-llm/src/redaction.rs (redact_credentials/bounded_excerpt/diagnostic_excerpt trio), the ProviderRegistration table-driven registry in provider_factory.rs"
  - phase: 17-additional-llm-provider-adapters (plan 04)
    provides: "the completed five-preset compat-engine build (kimi/qwen/grok/ollama/openai-compatible), against which this plan's combined verification command and six_preset_build registry test run"
provides:
  - "crates/paladin-llm/src/gemini/{mod,adapter}.rs — GeminiAdapter/GeminiConfig, the one build-list provider (D-01) that implements LlmPort directly against Gemini's own generateContent protocol rather than delegating to compat::CompatEngine (D-08)"
  - "provider_factory.rs registry row for gemini / GEMINI_API_KEY, placed alongside the other curated (named-vendor) presets, before the generic openai-compatible row and Ollama's credential-free row"
  - "provider_factory.rs::tests::six_preset_build — registry-wide six-row declared-order regression test for the plan's own combined verification command"
  - "the gemini Cargo feature (crates/paladin-llm/Cargo.toml), default feature set (openai, mock) unchanged (PROV-03)"
affects: [17-06, 17-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Bespoke non-compat-engine adapter reusing only the crate-level redaction trio — the one build-list provider (Gemini) that owns its own wire types, request builder, streaming parse loop and error mapping, modeled structurally on anthropic/adapter.rs rather than the openai-compatible core"
    - "Header-only credential placement, never a URL query parameter (x-goog-api-key) — set once via reqwest::Client::default_headers at construction rather than rebuilt per request"
    - "Dual HTTP-status + RPC-status-string error mapping — Google's error envelope carries an RPC-style status string (RESOURCE_EXHAUSTED, INVALID_ARGUMENT, etc.) alongside the HTTP code, so map_error switches on both rather than the HTTP-status-only pattern the compat engine's four presets use"

key-files:
  created:
    - crates/paladin-llm/src/gemini/mod.rs
    - crates/paladin-llm/src/gemini/adapter.rs
  modified:
    - crates/paladin-llm/src/lib.rs
    - crates/paladin-llm/src/provider_factory.rs
    - crates/paladin-llm/Cargo.toml

key-decisions:
  - "RESOURCE_EXHAUSTED maps conservatively to LlmError::RateLimitExceeded (retryable), per the decision RESEARCH.md's Open Question 2 already resolved and this plan's own body restated: retrying a true quota exhaustion merely burns the bounded retry budget, whereas mapping a transient rate limit to the non-retryable UsageLimitExceeded would fail a request that would have succeeded. Documented in map_error's own doc comment, following the ANTHROPIC_USAGE_CAP_SIGNATURE precedent's pattern of a named, documented, narrowly-matched assumption at the point of use. Verification path: the live-api-tests feature with a real GEMINI_API_KEY (D-15, deliberately unused this phase)."
  - "Split Task 1 and Task 2 into two atomic commits after writing and testing the complete adapter in one pass, by temporarily reducing generate_stream()/get_available_models()/validate_model() to a curated-fallback-only / not-yet-implemented surface for the Task 1 commit, then restoring the full SSE-streaming and OnceCell-memoized-model-list implementation for the Task 2 commit — each commit's tree independently builds, tests, and clippy-passes under `--features gemini`, matching the plan's own task boundaries rather than collapsing both into one commit."
  - "Registered the gemini row in provider_factory.rs's table alongside the other curated (named-vendor) presets — after grok, before the generic openai-compatible row and Ollama's credential-free row — for the same reason those two are already ordered last: a credential-free or catch-all row placed earlier would pre-empt an explicitly-configured named provider in get_default_provider()'s declared-table-order scan."
  - "build_request_serializes_only_the_known_gemini_request_fields asserts the closed key set of the serialized request as a JSON object's key set rather than a substring search for the literal strings 'tools'/'toolConfig' — written this way so the test itself does not trip this crate's own source-wide acceptance-criteria grep (`grep -v '^\\s*//' ... | grep -cE 'toolConfig|\"tools\"'` must return 0), which forbids those substrings anywhere in non-comment source in this file, test code included."

patterns-established:
  - "The bespoke-adapter template (own wire types, own streaming loop, own error mapping, crate-level redaction trio reused) is now proven twice in this crate (anthropic/, gemini/) — a future non-OpenAI-compatible provider has two worked examples to follow, not one."

requirements-completed: [PROV-02, PROV-03, PROV-04]

coverage:
  - id: D1
    description: "GeminiAdapter implements all six LlmPort methods directly against Gemini's own generateContent protocol, with zero dependency on compat::CompatEngine (D-08); get_provider_name() returns the fixed literal \"gemini\""
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#tests (39 tests: config defaulting x3, request shaping x5, finishReason mapping x1, response parsing x2, error mapping x7, capabilities/identity x2, retry semantics x2, generate() over mock transport x3, streaming x3, model-list x5, plus provider_factory's six_preset_build and provider_name_round_trip coverage)"
        status: pass
      - kind: other
        ref: "cargo build -p paladin-llm --no-default-features --features gemini && cargo test -p paladin-llm --no-default-features --features gemini (73 tests) && cargo clippy -p paladin-llm --no-default-features --features gemini -- -D warnings"
        status: pass
    human_judgment: false
  - id: D2
    description: "The system prompt is sent as the top-level systemInstruction field, never as a contents[] entry with a system role; the request body omits tools/toolConfig entirely; auth uses the x-goog-api-key header exclusively, never the ?key= query parameter"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#tests::build_request_places_system_prompt_in_system_instruction_never_in_contents, ::build_request_serializes_only_the_known_gemini_request_fields, ::generate_posts_to_generate_content_with_x_goog_api_key_header"
        status: pass
      - kind: other
        ref: "grep -v '^\\s*//' crates/paladin-llm/src/gemini/adapter.rs | grep -cE '[?&]key=' returns 0; grep -v '^\\s*//' ... | grep -cE 'toolConfig|\"tools\"' returns 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "finishReason mapping is exhaustive: STOP to Stop, MAX_TOKENS to Length, SAFETY to ContentFilter, OTHER/RECITATION to Error carrying the raw reason — never silently coerced to Stop; RESOURCE_EXHAUSTED maps to LlmError::RateLimitExceeded with the disposition documented as an assumption in map_error's own doc comment"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#tests::map_finish_reason_covers_every_documented_value, ::generate_recitation_finish_reason_produces_error_not_stop, ::map_error_429_with_resource_exhausted_status_maps_to_rate_limit_exceeded, ::map_error_non_429_with_resource_exhausted_status_still_maps_to_rate_limit_exceeded, ::generate_429_with_resource_exhausted_body_maps_to_rate_limit_exceeded"
        status: pass
    human_judgment: false
  - id: D4
    description: "get_available_models() queries GET {base_url}/models once per adapter instance (memoized via tokio::sync::OnceCell, concurrency-safe) and falls back to the curated GEMINI_FALLBACK_MODELS list on failure or an empty response (D-13/D-14); validate_model() accepts a model present only in the live list"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#tests::get_available_models_returns_two_live_entries_without_models_prefix, ::get_available_models_second_call_does_not_hit_the_mock_again, ::get_available_models_two_concurrent_first_calls_hit_the_mock_exactly_once, ::get_available_models_falls_back_to_curated_list_on_failure, ::validate_model_accepts_a_model_present_only_in_the_live_list"
        status: pass
    human_judgment: false
  - id: D5
    description: "Credential redaction reuses crate::redaction's redact_credentials/bounded_excerpt/diagnostic_excerpt trio — no second redaction implementation in this file; a 400 body echoing the configured x-goog-api-key value produces a diagnostic containing neither the key nor a recoverable fragment"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#tests::map_error_echoing_400_never_leaks_the_configured_api_key"
        status: pass
      - kind: other
        ref: "grep -c 'redact_credentials' crates/paladin-llm/src/gemini/adapter.rs returns 2"
        status: pass
    human_judgment: false
  - id: D6
    description: "gemini resolves through provider_factory.rs's cfg-gated registry, registered alongside the other curated presets; default paladin-llm feature set (openai, mock) is unchanged; openai/, anthropic/, deepseek/ remain byte-unchanged (D-06)"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible,gemini (170 tests) — provider_factory.rs#tests::six_preset_build::provider_names_returns_exactly_kimi_qwen_grok_gemini_openai_compatible_ollama_in_table_order, ::provider_name_round_trip::provider_name_round_trips_for_every_registry_row"
        status: pass
      - kind: other
        ref: "grep -c 'default = [\"openai\", \"mock\"]' crates/paladin-llm/Cargo.toml returns 1; git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ (empty diff); cargo test -p paladin-llm (default features, 53 tests) passes unchanged"
        status: pass
    human_judgment: false
  - id: D7
    description: "Wire-level facts (base URL, v1beta API version segment, default model gemini-2.5-flash, finishReason value set, streamGenerateContent?alt=sse requirement, error envelope shape) are sourced from 17-RESEARCH.md's citations against ai.google.dev, not independently re-verified against a live endpoint in this environment (no network egress in this sandbox)"
    verification: []
    human_judgment: true
    rationale: "This sandbox has no network egress, so the plan's own instruction to 'confirm the base URL, API version segment, default model ID and finishReason value set against ai.google.dev/api/generate-content before committing' could not be executed live. Every value used is the one 17-RESEARCH.md already cited against that same doc URL — no value was invented or guessed — but a human (or a future live-key run) should re-confirm none of it drifted since 17-RESEARCH.md was authored, given Gemini's catalog is the fastest-moving of the five build-list providers (Assumptions Log A1)."

# Metrics
duration: ~15min (session wall-clock from first file read to final commit; excludes plan-reading overhead shared with prior context)
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 05: Gemini Bespoke Adapter Summary

**`GeminiAdapter` — the one build-list provider (D-01/D-08) that speaks Google's `generateContent` protocol directly (own wire types, own SSE streaming loop, own dual HTTP+RPC-status error mapping) instead of sitting on the shared OpenAI-compatible engine, with header-only credential placement and a documented `RESOURCE_EXHAUSTED` retry disposition.**

## Performance

- **Duration:** ~15 min (session wall-clock)
- **Completed:** 2026-08-17 (Task 2 commit `0c159e5`)
- **Tasks:** 2 (both `type="auto" tdd="true"`, no checkpoints)
- **Files modified:** 5 (2 new under `gemini/`, 3 modified: `lib.rs`, `provider_factory.rs`, `Cargo.toml`)

## Accomplishments

- Built `GeminiAdapter`/`GeminiConfig` (`crates/paladin-llm/src/gemini/`) — the one bespoke, non-OpenAI-compatible provider in this crate (D-08). Modeled structurally on `anthropic/adapter.rs`: own private wire types (`GeminiRequest`/`GeminiContent`/`GeminiPart`/`GeminiSystemInstruction`/`GeminiGenerationConfig`/`GeminiResponse`/`GeminiCandidate`/`GeminiResponseContent`/`GeminiUsageMetadata`/`GeminiErrorEnvelope`/`GeminiErrorBody`/`GeminiModelsResponse`/`GeminiModelEntry`), own `build_request`/`parse_response`/`map_error`/`execute_with_retry`, own SSE streaming parse loop (`parse_sse_chunk`) — zero dependency on `crate::compat::CompatEngine`, confirmed by grep (`crate::compat` count 0 in non-comment source).
- **Request shaping:** `PromptType::System` (and `PromptType::Text` with `PromptRole::System`) map to the top-level `systemInstruction` field, never to a `contents[]` entry — Gemini's API has no `system` role inside `contents[]`. `PromptType::Function` and `PromptRole::Function` return `LlmError::InvalidPrompt` naming the unsupported prompt type rather than silently dropping it (Gemini's function surface is out of scope, D-08). The request body omits `tools`/`toolConfig` entirely — `GeminiRequest` has no such field at all, and `build_request_serializes_only_the_known_gemini_request_fields` asserts the serialized JSON's key set is exactly `{contents, systemInstruction, generationConfig}`.
- **Auth:** the `x-goog-api-key` header is set once via `reqwest::Client::default_headers` at construction — never the documented `?key=` query-parameter alternative, so the credential never appears in a URL or a request log (T-17-24). Confirmed by grep: `[?&]key=` count 0 in non-comment source.
- **`finishReason` mapping** is exhaustive: `STOP`→`Stop`, `MAX_TOKENS`→`Length`, `SAFETY`→`ContentFilter`, `OTHER`/`RECITATION`→`Error` carrying the raw reason string, an absent value→`Stop`. Never coerced to `Stop` for an unrecognised or blocked reason — proven by `generate_recitation_finish_reason_produces_error_not_stop`.
- **`map_error`** switches on both the HTTP status and the JSON `error.status` RPC string (Google's error envelope is ambiguous on HTTP 429 alone). `RESOURCE_EXHAUSTED` maps conservatively to `LlmError::RateLimitExceeded` (retryable) — documented as an assumption in `map_error`'s own doc comment, following the `ANTHROPIC_USAGE_CAP_SIGNATURE` precedent. Every extracted error message is redacted (`redact_credentials`) before it is bounded (`bounded_excerpt`) — redact-then-bound, never the reverse — proven by `map_error_echoing_400_never_leaks_the_configured_api_key`.
- **Streaming (Task 2):** `generate_stream()` POSTs to `{base_url}/models/{model}:streamGenerateContent` with the mandatory `alt=sse` query parameter (proven sent via a `mockito::Matcher::UrlEncoded` assertion with `.expect(1)`/`.assert_async()`). Parses `data: `-framed partial `GenerateContentResponse` objects — the same `GeminiResponse` type the non-streaming path parses — via `parse_sse_chunk`, `flat_map`ping a network chunk into zero or more `StreamingResponse` items so a chunk carrying multiple SSE frames never silently drops any past the first. No `[DONE]` sentinel; the stream ends when the body ends. A safety-blocked frame (no text parts) terminates without error, with `finish_reason` reflecting the block.
- **Model list (Task 2, D-13/D-14):** `get_available_models()` fetches `GET {base_url}/models` once per adapter instance via `tokio::sync::OnceCell` (memoized, concurrency-safe — proven by both a sequential-second-call and a `tokio::join!` concurrent-first-call test each hitting the mock exactly once), strips the `models/` prefix Google's list response applies, and falls back to `GEMINI_FALLBACK_MODELS` (`gemini-2.5-flash`, `gemini-2.5-pro`) on any failure or empty response, logged at `debug` (offline is a supported state). `validate_model()` checks the resolved list — proven to accept a model present only in the live catalog, unlike the shipped DeepSeek preset's hardcoded-`vec!` pattern (D-13).
- **Capabilities:** `supports_streaming: true`, `supports_system_messages: true`, `supports_vision: false` (text-only, D-08 — a truthful report, not an omission), `supports_tool_calling`/`supports_function_calling: false` (`LlmRequest` has no tool-definition field for this adapter to carry), `max_context_tokens: Some(1_048_576)`, `temperature_range: Some((0.0, 2.0))`.
- Registered the `gemini` row in `provider_factory.rs`'s table-driven registry (D-10), placed alongside the other curated (named-vendor) presets — after `grok`, before the generic `openai-compatible` row and Ollama's credential-free row. Added `provider_factory.rs::tests::six_preset_build` (widening the existing `five_new_preset_build` gate with `not(feature = "gemini")` first, mirroring plan 17-04's own precedent for this exact gate) proving the new six-row declared order under the plan's own combined verification command.
- `openai/`, `anthropic/`, `deepseek/`, `compat/` are **byte-unchanged** (D-06, confirmed via `git diff --stat`). Default `paladin-llm` feature set (`openai`, `mock`) is unchanged (PROV-03).

## Task Commits

1. **Task 1: Gemini config, wire types and the generateContent path** - `9c4e8a1` (feat)
2. **Task 2: Gemini streaming and the memoized model list** - `0c159e5` (feat)

_Note: both tasks are `type="auto" tdd="true"`, not TDD-gated at the plan level; each landed as a single commit with tests written alongside implementation, following plans 17-01/17-03/17-04's precedent. See "Deviations from Plan" for how the two commits were separated after the adapter was written and tested as one continuous unit._

## Files Created/Modified

- `crates/paladin-llm/src/gemini/mod.rs` - Module re-export shape mirroring `openai_compatible/mod.rs`
- `crates/paladin-llm/src/gemini/adapter.rs` - `GeminiConfig`/`GeminiAdapter`, all thirteen private wire types, module-level rustdoc recording Gemini's protocol divergences, 39 mockito-backed and pure-logic tests
- `crates/paladin-llm/src/lib.rs` - Declares `gemini` module (feature-gated), adds the doc-table row
- `crates/paladin-llm/src/provider_factory.rs` - `construct_gemini` + registry row (Task 1); widened `five_new_preset_build`'s gate to `not(feature = "gemini")` and added `six_preset_build` (Task 1, alongside the row addition, since both must land together for the plan's own combined verification command to pass)
- `crates/paladin-llm/Cargo.toml` - Adds `gemini = ["dep:reqwest", "dep:rand"]`; `default = ["openai", "mock"]` unchanged (PROV-03)

## Decisions Made

- **`RESOURCE_EXHAUSTED` → `LlmError::RateLimitExceeded` (retryable).** Resolves the plan's own "Decision resolved in this plan" section (RESEARCH.md Open Question 2): Gemini's `RESOURCE_EXHAUSTED` RPC status covers both a transient per-minute rate limit and a hard billing-quota exhaustion, and this adapter cannot distinguish the two without a live key. Retrying a true quota exhaustion merely burns the bounded retry budget; mapping a transient rate limit to the non-retryable `UsageLimitExceeded` would fail a request that would have succeeded — the asymmetry decides it. Recorded in `map_error`'s own doc comment, naming the verification path (`live-api-tests` feature with a real `GEMINI_API_KEY`, D-15).
- **Split Task 1 and Task 2 into two independently-buildable commits** after writing and testing the complete adapter in one continuous pass. Rather than committing the whole file at once, Task 1's commit ships `generate()`/`get_provider_name()`/`get_capabilities()` fully, with `generate_stream()` returning `LlmError::ProcessingError` ("implemented in Task 2") and `get_available_models()`/`validate_model()` checking the curated fallback list directly (no live fetch, no `OnceCell`) — a state that itself builds, passes `cargo test`/`cargo clippy -D warnings` under `--features gemini`, and satisfies every one of Task 1's own acceptance criteria (none of which test streaming or live model-list behavior). Task 2's commit then replaces those three methods with the full SSE-streaming and memoized-live-fetch implementation. This keeps each commit atomic and independently verifiable per the executor's per-task commit protocol, rather than collapsing genuinely separate task-scoped work into a single commit purely because both tasks were authored in one session.
- **`build_request_serializes_only_the_known_gemini_request_fields`** asserts the serialized request's JSON object key set (`{contents, systemInstruction, generationConfig}`) rather than searching for the absence of the literal substrings `"tools"`/`"toolConfig"`. This crate's own acceptance-criteria grep (`grep -v '^\s*//' ... | grep -cE 'toolConfig|"tools"'` must return `0`) is a blunt, source-wide check with no comment-vs-test-code distinction beyond leading-`//` filtering — a negative-assertion test that spells out the forbidden field names would itself trip the very check it's trying to prove passes. The closed-key-set assertion is a stronger behavioral proof (any future stray field is caught, not just these two names) and never needs to spell "tools" or "toolConfig" as a literal.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `five_new_preset_build`'s exact-match assertion would fail under this plan's own combined verification command**
- **Found during:** Task 1, immediately after registering the `gemini` row — plan 17-04's `five_new_preset_build` test module asserts `provider_names() == ["kimi", "qwen", "grok", "openai-compatible", "ollama"]` exactly, and this plan's own `<verification>` block runs `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible,gemini`, under which the table now has six rows.
- **Fix:** Widened the gate to `not(feature = "gemini")` (mirroring the identical pattern plan 17-04 established for this exact gate against its own `openai-compatible` addition) and added `six_preset_build`, gated the opposite way, asserting the new six-row order directly (`["kimi", "qwen", "grok", "gemini", "openai-compatible", "ollama"]`).
- **Files modified:** `crates/paladin-llm/src/provider_factory.rs`
- **Verification:** `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible,gemini` — 170 tests passed, including `six_preset_build::provider_names_returns_exactly_kimi_qwen_grok_gemini_openai_compatible_ollama_in_table_order`.
- **Committed in:** `9c4e8a1` (Task 1 commit)

**2. [Rule 1 - Bug] Two streaming tests failed with a mockito 501 because `.match_query()` was absent while the request carried `?alt=sse`**
- **Found during:** Task 2, first test run — `mockito`'s `Unified` path matcher (used when `.match_query()` is never called) compares the FULL request `path?query` string against the mock's `Exact(path)` matcher, so a request carrying `?alt=sse` never matched a mock registered without an explicit query matcher, returning mockito's default 501.
- **Fix:** Added `.match_query(Matcher::Any)` to `generate_stream_assembles_three_frames_in_wire_order` and `generate_stream_safety_blocked_frame_terminates_without_error` (neither test asserts the query string itself — that's `generate_stream_posts_with_alt_sse_query_parameter`'s job, which already used `Matcher::UrlEncoded`).
- **Files modified:** `crates/paladin-llm/src/gemini/adapter.rs`
- **Verification:** `cargo test -p paladin-llm --no-default-features --features gemini` — 73/73 passed after the fix.
- **Committed in:** `0c159e5` (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1 bugs, both mechanical test-infrastructure fixes with no behavioral consequence to the shipped adapter)
**Impact on plan:** Both fixes were necessary for the plan's own stated verification commands to pass. No scope creep.

## Issues Encountered

- **Vendor facts not live-verified.** This sandbox has no network egress, so the plan's own instruction to confirm the base URL, API version segment, default model ID and `finishReason` value set against `ai.google.dev/api/generate-content` before committing could not be executed against the live doc. Every value used (`https://generativelanguage.googleapis.com/v1beta`, `gemini-2.5-flash`, the `STOP`/`MAX_TOKENS`/`SAFETY`/`OTHER`/`RECITATION` set, the `x-goog-api-key` header, `:generateContent`/`:streamGenerateContent?alt=sse`) is exactly what 17-RESEARCH.md already cited against that doc URL — nothing was invented — but this is recorded as `human_judgment: true` (coverage D7) since Gemini's catalog is the fastest-moving of the five build-list providers and a live re-check was not possible here.
- **Task 1 / Task 2 commit split required post-hoc surgery** — see "Decisions Made" above. Both the strip-down and the restore were separately built, tested, `clippy -D warnings`-checked and `cargo fmt --check`-clean before each commit.

## User Setup Required

None - no external service configuration required. A `GEMINI_API_KEY` would be required to actually call the live endpoint, but every test in this plan runs offline against `mockito` — none requires it.

## Next Phase Readiness

- All five named D-01 presets (Kimi, Qwen, Grok, Ollama), the generic `openai-compatible` provider, and now Gemini — the one bespoke, non-OpenAI-compatible adapter — all ship. The entire PROV-01 build list is complete.
- `GeminiAdapter` establishes the second worked example (after `anthropic/adapter.rs`) of the "own wire types, own streaming loop, own error mapping, crate-level redaction trio reused" bespoke-adapter pattern in this crate — a template for any future non-OpenAI-compatible provider.
- `provider_factory.rs`'s registry now has six feature-gated rows plus the three shipped defaults (openai/anthropic/deepseek); `provider_name_round_trip` and `six_preset_build` both exercise the full table.
- No blockers. `live-api-tests`-gated tests for Gemini (and Kimi/Qwen/Grok) remain deferred per D-15, triggerable once the four API keys are acquired as project secrets.

## Self-Check: PASSED

All created/modified files verified present on disk; both task commit hashes (`9c4e8a1`, `0c159e5`) verified present in `git log`.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*
