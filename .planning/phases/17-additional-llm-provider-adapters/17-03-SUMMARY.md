---
phase: 17-additional-llm-provider-adapters
plan: 03
subsystem: api
tags: [llm, rust, qwen, dashscope, grok, xai, ollama, openai-compatible, mockito, reqwest, tokio]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters (plan 01)
    provides: "crates/paladin-llm/src/compat/{mod,types,engine}.rs (CompatEngine), crates/paladin-llm/src/redaction.rs, the ProviderRegistration table-driven registry in provider_factory.rs, and kimi/adapter.rs as the reference preset"
provides:
  - "crates/paladin-llm/src/qwen/{mod,adapter}.rs — QwenAdapter, Alibaba DashScope compatible-mode preset over CompatEngine"
  - "crates/paladin-llm/src/grok/{mod,adapter}.rs — GrokAdapter, xAI preset over CompatEngine"
  - "crates/paladin-llm/src/ollama/{mod,adapter}.rs — OllamaAdapter, self-hosted keyless preset over CompatEngine — the first registry row with env_var: None"
  - "provider_factory.rs registry rows for qwen/grok/ollama, and the widened kimi_only_build/four_new_preset_build test gates that keep the D-10 regression coverage correct under the combined feature matrix"
affects: [17-04, 17-05, 17-06, 17-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Config-only preset over CompatEngine — no preset in this plan touches compat/engine.rs or compat/types.rs; the D-05 architectural bet plan 17-01 set up holds unmodified for all three presets"
    - "Credential-free registry row (env_var: None) placed after every credentialed row in registry declaration order, so get_default_provider()'s table-order scan never lets a keyless local preset pre-empt an explicitly-configured hosted one"

key-files:
  created:
    - crates/paladin-llm/src/qwen/mod.rs
    - crates/paladin-llm/src/qwen/adapter.rs
    - crates/paladin-llm/src/grok/mod.rs
    - crates/paladin-llm/src/grok/adapter.rs
    - crates/paladin-llm/src/ollama/mod.rs
    - crates/paladin-llm/src/ollama/adapter.rs
  modified:
    - crates/paladin-llm/src/lib.rs
    - crates/paladin-llm/src/provider_factory.rs
    - crates/paladin-llm/Cargo.toml

key-decisions:
  - "compat/ was NOT touched by any of the three presets — confirms the plan's own success criterion that D-05 either holds here or does not; it does. Zero lines changed in compat/engine.rs or compat/types.rs across all three tasks."
  - "Widened provider_factory.rs's kimi_only_build test gate incrementally, once per task (adding not(feature = \"qwen\"), then not(feature = \"grok\"), then not(feature = \"ollama\")), because that test module asserts provider_names() == [\"kimi\"] exactly and would otherwise fail to compile-correct under the plan's own combined-feature verification command (cargo test --features kimi,qwen,grok,ollama). This is Rule 1 (auto-fix bug): the pre-existing gate was written in plan 17-01 when kimi was the only additional feature and did not anticipate siblings landing in the same crate."
  - "Added a new four_new_preset_build test module (gated on all four new-plan features, none of the shipped three) directly proving the plan's own acceptance criterion: provider_names() under --features kimi,qwen,grok,ollama returns exactly [\"kimi\", \"qwen\", \"grok\", \"ollama\"] in table order, with the credential-free ollama row last."
  - "OllamaConfig carries no api_key field at all (unlike Kimi/Qwen/Grok's *Config structs) — the placeholder credential OLLAMA_PLACEHOLDER_API_KEY is a compile-time constant applied directly in OllamaAdapter::new, never operator-settable, per the plan's own recorded Ollama-auth decision."

patterns-established:
  - "Registry row with env_var: None — the shape ProviderRegistration already accommodated per plan 17-01's SUMMARY forward-readiness note; no struct change was needed, confirming that note's prediction."

requirements-completed: [PROV-02, PROV-03, PROV-04]

coverage:
  - id: D1
    description: "QwenAdapter (Alibaba DashScope compatible-mode) implements all six LlmPort methods by delegating to CompatEngine, with no protocol logic of its own; get_provider_name() returns \"qwen\"; get_capabilities() reports supports_vision: false and no tool/function calling"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/qwen/adapter.rs#tests (16 tests: from_env defaulting x3, request shaping, streaming assembly, 4 HTTP error mappings, provider name, capabilities)"
        status: pass
      - kind: other
        ref: "cargo build -p paladin-llm --no-default-features --features qwen && cargo clippy -p paladin-llm --no-default-features --features qwen -- -D warnings"
        status: pass
    human_judgment: false
  - id: D2
    description: "GrokAdapter (xAI) implements all six LlmPort methods by delegating to CompatEngine; get_provider_name() returns \"grok\"; get_capabilities() reports no tool/function calling"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/grok/adapter.rs#tests (15 tests: from_env defaulting x3, request shaping, streaming assembly, 4 HTTP error mappings, provider name, capabilities)"
        status: pass
      - kind: other
        ref: "cargo build -p paladin-llm --no-default-features --features grok && cargo clippy -p paladin-llm --no-default-features --features grok -- -D warnings"
        status: pass
    human_judgment: false
  - id: D3
    description: "OllamaAdapter (self-hosted, keyless) implements all six LlmPort methods by delegating to CompatEngine; resolves via LlmProviderFactory::create(\"ollama\") with zero credential env vars set; sends a fixed Authorization placeholder rather than omitting the header; get_capabilities().max_context_tokens is None; live model list falls back to the curated 3-entry list on an empty /models response"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/ollama/adapter.rs#tests (11 tests) + provider_factory.rs#tests::ollama_requires_no_credential (2 tests) + provider_factory.rs#tests::four_new_preset_build (1 test)"
        status: pass
      - kind: other
        ref: "cargo build -p paladin-llm --no-default-features --features ollama && cargo clippy -p paladin-llm --no-default-features --features ollama -- -D warnings"
        status: pass
    human_judgment: false
  - id: D4
    description: "All three new providers resolve through provider_factory.rs's cfg-gated ProviderRegistration table (D-10); the combined build (kimi,qwen,grok,ollama) compiles and tests green with provider_names() in exact declared table order"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama (111 tests passed)"
        status: pass
      - kind: other
        ref: "cargo clippy -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama -- -D warnings"
        status: pass
    human_judgment: false
  - id: D5
    description: "Default paladin-llm feature set (openai, mock) is unchanged; openai/, anthropic/, deepseek/ are byte-unchanged (D-06)"
    requirement: "PROV-03"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm (default features, 52 tests passed)"
        status: pass
      - kind: other
        ref: "git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ (empty diff)"
        status: pass
    human_judgment: false
  - id: D6
    description: "Vendor facts (base URL, default model, fallback model IDs) for Qwen, Grok and Ollama confirmed against cited vendor docs before commit"
    verification: []
    human_judgment: true
    rationale: "This execution environment has no network egress to verify vendor documentation live. Every value used is exactly 17-RESEARCH.md's [CITED] figure, and Assumptions Log A1/A2/A3 already record this exact risk as accepted (low-impact, since D-13's live-fetch-with-curated-fallback design tolerates a stale default). A human (or a future environment with network access) should confirm https://alibabacloud.com/help/en/model-studio/compatibility-of-openai-with-dashscope (Qwen), https://docs.x.ai (Grok), and Ollama's own OpenAI-compatibility documentation before any of these three presets are used against a live endpoint."

# Metrics
duration: ~6min (git commit timestamps: 02:40:58Z Task 1 to 02:46:39Z Task 3; excludes read/planning time before the first commit)
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 03: Qwen, Grok and Ollama Presets over CompatEngine Summary

**Three thin config-only presets (Qwen/DashScope, Grok/xAI, Ollama self-hosted-keyless) added over the shared `CompatEngine` from plan 17-01, with zero lines changed in `compat/engine.rs` or `compat/types.rs` — the plan's own D-05 architectural bet holds.**

## Performance

- **Duration:** ~6 min (commit-to-commit; see frontmatter note — excludes upfront context-reading time)
- **Completed:** 2026-08-17T02:46:39Z (Task 3 commit)
- **Tasks:** 3 completed
- **Files modified:** 9 (6 new across three preset directories, 3 modified: `lib.rs`, `provider_factory.rs`, `Cargo.toml` — each touched once per task, counted once here)

## Accomplishments

- Built `QwenAdapter`/`QwenConfig` (`crates/paladin-llm/src/qwen/`) — Alibaba DashScope compatible-mode preset. `DASHSCOPE_API_KEY` required; base URL defaults to the international compatible-mode endpoint (`https://dashscope-intl.aliyuncs.com/compatible-mode/v1`) per the plan's own recorded region decision, overridable via `DASHSCOPE_BASE_URL`. `supports_vision: false` — the text adapter reports what it implements, not what the `qwen-vl` product family advertises (D-08's vision boundary; T-17-16).
- Built `GrokAdapter`/`GrokConfig` (`crates/paladin-llm/src/grok/`) — xAI preset. `XAI_API_KEY` required; base URL `https://api.x.ai/v1`, default model `grok-4`, fallback list `[grok-4, grok-3]`.
- Built `OllamaAdapter`/`OllamaConfig` (`crates/paladin-llm/src/ollama/`) — self-hosted, keyless preset. **No credential env var** — `from_env()` succeeds with an entirely empty environment (D-12). Speaks the `/v1/*` OpenAI-compatibility layer (standard `data: `-framed SSE), never the native `/api/chat` NDJSON endpoint, so the shared engine needs no fork. Sends a fixed `OLLAMA_PLACEHOLDER_API_KEY` ("ollama") as the `Authorization` header value — the header is present, never omitted — keeping the engine's header-construction path identical across every preset. `max_context_tokens: None` (the window depends on whichever model the operator pulled). Timeout default `120s`, not the hosted-preset `60s` (T-17-17, cold local model starts).
- Registered all three in `provider_factory.rs`'s `cfg`-gated `ProviderRegistration` table (D-10). Ollama's row is the first with `env_var: None` — placed **after** every credentialed row so it never pre-empts an explicitly-configured hosted provider in `get_default_provider()`'s declared-table-order scan.
- Widened the `kimi_only_build` D-10 regression test gate incrementally (once per task) to keep excluding `qwen`/`grok`/`ollama`, and added two new gated test modules — `ollama_requires_no_credential` and `four_new_preset_build` — the latter directly proving Task 3's own acceptance criterion: `provider_names()` under `--features kimi,qwen,grok,ollama` returns exactly `["kimi", "qwen", "grok", "ollama"]` in declared table order.
- `compat/engine.rs` and `compat/types.rs` are **byte-unchanged** across all three tasks (confirmed via `git diff --stat`, empty). `openai/`, `anthropic/`, `deepseek/` are also byte-unchanged (D-06).

## Task Commits

1. **Task 1: Qwen preset (Alibaba DashScope compatible-mode)** - `2aaa2e1` (feat)
2. **Task 2: Grok preset (xAI)** - `1326744` (feat)
3. **Task 3: Ollama preset (self-hosted, keyless) — settles the Meta/Llama row** - `cef9852` (feat)

_Note: this plan's tasks are `type="auto" tdd="true"`, not TDD-gated at the plan level; each landed as a single commit with tests written alongside the implementation, following plan 17-01's precedent for its own `tdd="true"` tasks._

## Files Created/Modified

- `crates/paladin-llm/src/qwen/mod.rs` - Module re-export shape mirroring `kimi/mod.rs`
- `crates/paladin-llm/src/qwen/adapter.rs` - `QwenConfig`/`QwenAdapter`, 16 mockito-backed tests
- `crates/paladin-llm/src/grok/mod.rs` - Module re-export shape
- `crates/paladin-llm/src/grok/adapter.rs` - `GrokConfig`/`GrokAdapter`, 15 mockito-backed tests
- `crates/paladin-llm/src/ollama/mod.rs` - Module re-export shape
- `crates/paladin-llm/src/ollama/adapter.rs` - `OllamaConfig`/`OllamaAdapter` (no `api_key` field — placeholder is a compile-time constant), 11 mockito-backed tests
- `crates/paladin-llm/src/lib.rs` - Declares `qwen`/`grok`/`ollama` modules (feature-gated), widens the `compat` module's `cfg(any(...))` gate to include all three, adds three doc-table rows
- `crates/paladin-llm/src/provider_factory.rs` - Three new `construct_*` functions + registry rows; widened `kimi_only_build` gate (three additions, one per task); added `ollama_requires_no_credential` and `four_new_preset_build` test modules
- `crates/paladin-llm/Cargo.toml` - Adds `qwen`, `grok`, `ollama` features (`["dep:reqwest", "dep:rand"]` each); `default = ["openai", "mock"]` unchanged (PROV-03)

## Decisions Made

- **Region default for Qwen's base URL** was already resolved in the plan's own frontmatter ("Decisions resolved in this plan") — the international DashScope compatible-mode endpoint, documented in the adapter's module rustdoc per the plan's instruction. No new decision made here; implemented as specified.
- **Ollama's placeholder-auth approach** was likewise pre-resolved in the plan. Implemented as specified: `OLLAMA_PLACEHOLDER_API_KEY` is a compile-time constant passed directly into `CompatEngineConfig.api_key`, never exposed as an `OllamaConfig` field an operator could set.
- **Test-gate widening (kimi_only_build)** — see key-decisions in frontmatter. This is the one place execution diverged from a literal reading of the plan text (which only says "add the registry row" and doesn't call out the existing test's exact-match assertion), but it is required for the plan's own stated verification command (`cargo test --features kimi,qwen,grok,ollama`) to pass, so it is Rule 1 (auto-fix bug), not scope creep.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `kimi_only_build`'s exact-match assertion would fail to compile-correct under the plan's own combined-feature test command**
- **Found during:** Task 1, before writing any qwen code — read `provider_factory.rs`'s existing test module structure first
- **Issue:** Plan 17-01's `kimi_only_build` test module is gated `#[cfg(all(feature = "kimi", not(openai), not(anthropic), not(deepseek)))]` and asserts `provider_names() == vec!["kimi"]` exactly. That gate does not exclude `qwen`/`grok`/`ollama`, none of which existed when it was written. Task 3's own acceptance criterion runs `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama` — under that invocation, `kimi_only_build` would compile (its `kimi` feature is on) and its `provider_names_returns_exactly_kimi` assertion would fail, since the table now has four rows.
- **Fix:** Widened the gate once per task as each new feature landed — `not(feature = "qwen")` in Task 1, `not(feature = "grok")` in Task 2, `not(feature = "ollama")` in Task 3 — so the module only compiles under exactly the single-provider build it was written to test. Added a parallel `four_new_preset_build` module (gated the opposite way: all four new features on, the shipped three off) asserting the exact `["kimi", "qwen", "grok", "ollama"]` order, which is Task 3's own acceptance-criterion text turned into a real test.
- **Files modified:** `crates/paladin-llm/src/provider_factory.rs`
- **Verification:** `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama` — 111 tests passed, including `four_new_preset_build::provider_names_returns_exactly_kimi_qwen_grok_ollama_in_table_order`.
- **Committed in:** `2aaa2e1`, `1326744`, `cef9852` (one gate widening per task's commit)

---

**Total deviations:** 1 auto-fixed (1 bug), applied incrementally across all three task commits
**Impact on plan:** The fix is mechanical (a `cfg` gate widening) with no behavioral consequence to any shipped adapter. It is necessary for the plan's own stated combined-feature verification command to pass. No scope creep.

## Issues Encountered

- **Vendor facts not live-verified** for Qwen, Grok or Ollama. Per D-00e and each task's `<action>` instruction, these should be "confirmed against the vendor doc before committing." This execution environment has no network egress, so no live confirmation was possible for any of the three. The values used are exactly 17-RESEARCH.md's `[CITED]` figures (DashScope international endpoint + `qwen-plus`; `api.x.ai/v1` + `grok-4`; Ollama's documented `/v1/*` compat layer + "required but ignored" auth note), matching plan 17-01's own precedent for Kimi. Recorded as `human_judgment: true` coverage item D6 above rather than silently asserted as verified.
- **Confirmed (read, not asserted) that the engine's request builder sends none of Ollama's documented-unsupported fields**, per Task 3's instruction to verify rather than add defensive code: `crates/paladin-llm/src/compat/types.rs`'s `CompatRequest` struct (lines 13-28) has exactly seven fields — `model`, `messages`, `temperature`, `max_tokens`, `top_p`, `frequency_penalty`, `presence_penalty`, `stream`. No `tool_choice`, `logit_bias`, `user`, `n`, or logprobs field exists anywhere in `compat/types.rs`, so no divergence-handling code was added for Ollama's documented-unsupported-field list — there is no request path through which any of them could be sent.

## User Setup Required

None - no external service configuration required. (A `DASHSCOPE_API_KEY` or `XAI_API_KEY` would be required to actually call the live Qwen or Grok APIs, and a running local Ollama server to call Ollama's, but no test in this plan requires any of them — all 42 new-preset tests run offline against `mockito`.)

## Next Phase Readiness

- All five named D-01 presets that sit on the shared compatible core (Kimi, Qwen, Grok, Ollama) are now shipped; only Gemini (bespoke protocol, plan 17-05) and the generic `openai-compatible` provider (plan 17-04) remain.
- `provider_factory.rs`'s registry now demonstrably supports a mixed credentialed/credential-free table under concurrent and combined-feature builds — plan 17-04's generic provider (also credentialed, config-driven) has a proven pattern to follow.
- The `ProviderRegistration { env_var: Option<&'static str> }` shape needed no struct change to accommodate Ollama's `None` row, confirming plan 17-01's SUMMARY forward-readiness prediction.
- No blockers. The one open item (Qwen/Grok/Ollama vendor-fact live verification) is low-risk and explicitly flagged for human/future-environment follow-up, matching the disposition already accepted for Kimi in plan 17-01.

## Self-Check: PASSED

All created files verified present on disk; all three task commit hashes (`2aaa2e1`, `1326744`, `cef9852`) verified present in `git log`.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*
