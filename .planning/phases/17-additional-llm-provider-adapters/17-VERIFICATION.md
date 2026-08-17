---
phase: 17-additional-llm-provider-adapters
verified: 2026-08-17T16:56:03Z
status: gaps_found
score: 6/9 must-haves verified
behavior_unverified: 0
overrides_applied: 0
gaps:
  - truth: "New adapter code is free of unresolved Critical-severity security defects (CLAUDE.md's mandatory Snyk/security-review posture for new first-party code; the same bar PROV-02 holds the shipped three adapters to)"
    status: failed
    reason: "Gemini's generate() and generate_stream() interpolate the caller-supplied request.model field unescaped directly into the request URL path, with no percent-encoding and no allow-list validation, before the request is sent carrying the real x-goog-api-key credential. Confirmed present and unpatched in the live tree at verification time (crates/paladin-llm/src/gemini/adapter.rs:564-567 and :622-625). Flagged as CR-01 (Critical) in 17-REVIEW.md; no fix commit exists anywhere in phase-17 git history (checked openai/anthropic/deepseek AND gemini file histories -- no post-review commit touches gemini/adapter.rs). Every other adapter this phase adds carries `model` inside the JSON request body (compat/types.rs::CompatRequest.model, serde-encoded, safe); Gemini is the one adapter that puts it in the URL, and it is the one bespoke-protocol adapter the phase's own context flagged as never verified against a live endpoint."
      artifacts:
        - path: "crates/paladin-llm/src/gemini/adapter.rs"
          issue: "generate() (:564-567) and generate_stream() (:622-625) build the request URL via format!(\"{}/models/{}:generateContent\", base_url, request.model) — a model value containing '/', '?', '#' or ':' can alter the request path or inject/override query parameters (e.g. defeating the mandatory alt=sse parameter on the streaming path)."
      missing:
        - "Percent-encode the model path segment (e.g. percent_encoding::utf8_percent_encode with NON_ALPHANUMERIC) before interpolating it into the URL, per 17-REVIEW.md CR-01's fix guidance"
        - "And/or validate request.model against an allow-listed character set (e.g. [A-Za-z0-9._-]) or against self.available_models(), returning LlmError::InvalidPrompt before the URL is ever constructed"
        - "A regression test proving a model value containing '/', '?', '#' or ':' is rejected or safely encoded rather than reaching the wire"
human_verification:
  - test: "Run `make coverage` (or `cargo llvm-cov --workspace --features integration-tests --fail-under-lines 82`) in an environment with Redis and MinIO reachable via Docker, with all six new provider features compiled in, and confirm the workspace stays at or above the 82% line-coverage floor (ADR-0006)."
    expected: "cargo llvm-cov reports >= 82% workspace line coverage with the six new adapters' code counted, not excluded."
    why_human: "This verification sandbox has no Docker daemon; `make coverage`'s own preflight fails fast on unreachable Redis (6380) and MinIO (9010). The percentage with the new code counted is genuinely UNMEASURED, not failing -- tracked as WINDOWS.md id 13, a human-accepted item of deliberate verification debt (AskUserQuestion checkpoint, 2026-08-17)."
  - test: "Run `tests/integration/ollama_docker_test.rs` against a real `ollama/ollama` container with `qwen2.5:0.5b` pulled (docker compose -f docker/docker-compose.test.yml up ollama-test ollama-test-init), then `cargo test -p paladin-ai --no-default-features --features integration-tests,llm-ollama --test ollama_docker`."
    expected: "All 4 tests (generate round-trip, streaming, get_available_models, validate_model) exercise the real server and pass with real token usage / real model list data, not the SKIP path."
    why_human: "No Docker daemon in this sandbox. Confirmed in this verification pass that the suite gracefully SKIPs (prints a named `SKIP: ollama-test unreachable ...` message and still reports `ok`) rather than failing or silently passing -- so the suite's failure-mode contract is sound -- but the shared compat engine has never actually been exercised end-to-end against a real Ollama implementation of the protocol. Tracked as WINDOWS.md id 12, same 2026-08-17 human-accepted debt."
  - test: "Smoke-test the recorded base URLs and default/fallback model IDs for Kimi, Qwen, Grok and Gemini (README.md / config.example.yml / docs/src/getting-started/configuration.md all carry an explicit 'not verified against a live endpoint' caveat) against each vendor's real API using a live credential."
    expected: "Each vendor's documented base_url resolves, the default model ID exists, and get_available_models()'s live-fetch path (not just the curated fallback) returns a real, well-formed model list."
    why_human: "No network egress / no vendor API keys available in this sandbox. These facts were taken from vendor documentation, never confirmed live -- the phase's own docs are explicit and honest about this; it is not a hidden gap, but it remains unverified and worth a human smoke test before a consumer relies on the defaults."
---

# Phase 17: Additional LLM Provider Adapters Verification Report

**Phase Goal:** Paladin talks to the providers its users actually deploy — the candidate field is
narrowed to a shortlist against recorded criteria, and every provider that survives ships as a
feature-gated adapter meeting the same `LlmPort` contract the existing three do.

**Verified:** 2026-08-17T16:56:03Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A recorded provider-selection study evaluates candidates against explicit written criteria, with every candidate carrying one build/defer/reject verdict; Kimi, Gemini, Qwen and Meta/Llama each explicitly dispositioned; Llama names a host (Ollama), not a model family | ✓ VERIFIED | `.planning/decisions/0045-additional-llm-provider-selection.md` states 9 scoring criteria before any verdict (lines 45-65), then a single scored table (lines 71-83) covering Kimi/Qwen/Grok/Ollama/Gemini (build), Meta-Llama (rejected as a row, dispositioned via Ollama), and Groq/Together/Mistral/Fireworks/Bedrock (reject — already covered). `REQUIREMENTS.md` PROV-01 amended in place (lines ~3428-3442) with a dated verdict summary citing ADR-0045; `PROMOTION.md` next-free-ADR line advanced 0045→0047 across the phase's two authored ADRs. |
| 2 | Every build-list adapter (Kimi, Qwen, Grok, Ollama, Gemini, generic openai-compatible) implements all six `LlmPort` methods with no stubbed body and no optimistic capability response | ✓ VERIFIED | `grep -n "async fn generate\b\|generate_stream\|validate_model\|get_available_models\|fn get_provider_name\|fn get_capabilities"` against all six adapter files shows all six methods present in each of `kimi/`, `qwen/`, `grok/`, `ollama/`, `gemini/`, `openai_compatible/adapter.rs`. Capability truthfulness spot-checked: Gemini reports `supports_vision: false` / no tool calling (matches D-08's "text-only" claim); the five compat-engine presets all report `supports_tool_calling: false` / `supports_function_calling: false` (`LlmRequest` carries no tool definition); `openai_compatible`'s capabilities are 100% operator-declared with pessimistic defaults (D-04), confirmed by its own test `get_capabilities_reports_exactly_what_was_declared`. `cargo clippy -p paladin-llm --all-targets --no-default-features --features "openai,anthropic,deepseek,kimi,qwen,grok,ollama,gemini,openai-compatible,mock" -- -D warnings` — exit 0, no warnings. |
| 3 | New adapter code is free of unresolved Critical-severity security defects | ✗ FAILED | Gemini's `generate()`/`generate_stream()` splice `request.model` unescaped into the request URL path (`crates/paladin-llm/src/gemini/adapter.rs:564-567`, `:622-625`), confirmed present and unpatched by direct read at verification time. Flagged CR-01 (Critical) in `17-REVIEW.md`; `git log --oneline -- crates/paladin-llm/src/gemini/adapter.rs` shows no commit after the three plan-17-05 commits (`9c4e8a1`, `0c159e5`, `146b30f`) — no fix landed. See Gaps below. |
| 4 | Each new provider is feature-gated: `cargo build -p paladin-llm --no-default-features --features <provider>` succeeds for every provider, individually and combined | ✓ VERIFIED | `cargo check -p paladin-llm --no-default-features --features kimi` → exit 0 (20.75s cold). `cargo check -p paladin-llm --no-default-features --features qwen,grok,ollama,openai-compatible,gemini` → exit 0 (3.29s incremental). `cargo check -p paladin-ai --no-default-features --features llm-all` → exit 0. `cargo check -p paladin-ai` (default features) → exit 0. |
| 5 | The effective default provider set is unchanged (openai+anthropic+deepseek); `provider_factory` resolves new providers exactly as the existing three; adding a provider changes no existing provider's behaviour | ✓ VERIFIED | Root `Cargo.toml:268` `default = ["llm-openai", "llm-anthropic", "llm-deepseek"]`; `:279-288` all nine `llm-*` flags forward `["paladin-llm/<provider>"]`, `llm-all`/`full` enumerate all nine. `provider_factory.rs` is a single `cfg`-gated table (`build_provider_registry`, lines 216-300) that `create()`/`get_default_provider()`/`list_available_providers()`/`UnknownProvider` all derive from (D-10) — one row shape for old and new providers alike. Regression test `default_features_still_resolve_openai_anthropic_and_deepseek` run singly (`cargo test -p paladin-ai --test unit default_features_still_resolve_openai_anthropic_and_deepseek`) → **1 passed**. `git log --oneline -- crates/paladin-llm/src/openai/adapter.rs crates/paladin-llm/src/anthropic/adapter.rs crates/paladin-llm/src/deepseek/adapter.rs` shows **zero** phase-17 commits touching any of the three shipped adapters (D-06 honored). |
| 6 | Mock-transport unit tests cover request shaping, response parsing, streaming chunk assembly and error mapping for every new adapter; every public item carries rustdoc | ✓ VERIFIED | Per-adapter `#[test]`/`#[tokio::test]` counts: kimi 26, qwen 11, grok 11, ollama 9, gemini 31, openai_compatible 24 — all runnable offline, no credential. `LlmConfig` gained one `Option<LlmProviderConfig>` field per new provider (`config/llm.rs:49-65`) plus `config/bridge.rs` `From` impls. `cargo doc -p paladin-llm --no-deps` with all provider features → 1 rustdoc warning total, and it is `private_intra_doc_links` (a doc-comment cross-reference to a private type), **not** a `missing_docs` warning — matches 17-07-SUMMARY's own "0 missing-docs warnings" claim exactly. |
| 7 | The workspace stays at or above the 82% line-coverage floor with the new provider code counted (ADR-0006) | ? UNCERTAIN | `make coverage` requires Redis (6380) and MinIO (9010) via Docker; no Docker daemon exists in this environment, confirmed by attempting the same build/test commands above with no service dependency — those succeed, but coverage's own preflight would fail fast. **Genuinely UNMEASURED, not failing.** Matches `WINDOWS.md` id 13 exactly, an explicitly human-accepted item of verification debt (AskUserQuestion checkpoint, 2026-08-17, during `/gsd-execute-phase 17`). Routed to Human Verification below. |
| 8 | Live-API / real-endpoint behaviour is exercised for the credential-gated and Docker-gated tests (Ollama Tier 2, hosted-four `live-api-tests`) | ? UNCERTAIN | Ran `tests/integration/ollama_docker_test.rs` directly in this sandbox (`cargo test -p paladin-ai --no-default-features --features integration-tests,llm-ollama --test ollama_docker`): compiles, all 4 tests report `ok`, but each is a named, printed `SKIP: ollama-test unreachable ...` early-return (confirmed by reading `skip_if_unreachable`, `ollama_docker_test.rs:56-92`) rather than a real exercise against a server — no Docker daemon here either. Vendor base URLs/default model IDs for Kimi/Qwen/Grok/Gemini carry an explicit "not verified against a live endpoint" caveat in README/docs/config.example.yml already. Matches `WINDOWS.md` id 12, same human-accepted debt. Routed to Human Verification below. |
| 9 | The advertised surface (`paladin-llm`'s Cargo.toml description/keywords, crate README, configuration docs) names exactly the providers that exist | ✓ VERIFIED | `crates/paladin-llm/Cargo.toml` description names "major hosted providers plus a generic OpenAI-compatible adapter"; `keywords = ["ai", "llm", "openai-compatible", "gemini", "agents"]`. README's provider table (lines 12-25) lists all 9 named providers + generic + mock, matching the 9 Cargo features exactly. `docs/src/api-reference/feature-flags.md` and `docs/src/getting-started/configuration.md` name the identical 9-provider set. `config.example.yml` carries a block per provider, `${ENV_VAR}` indirection throughout, zero literal keys (`grep -n "api_key"` shows only `${...}` forms). |

**Score:** 6/9 truths verified (1 failed — CR-01; 2 uncertain — coverage floor and live-endpoint behaviour, both pre-existing, human-accepted verification debt routed to human_verification, not silently passed)

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `crates/paladin-llm/src/compat/{mod,types,engine}.rs` | Shared OpenAI-compatible engine (D-05) | ✓ VERIFIED | Present; kimi/qwen/grok/ollama/openai_compatible all delegate to it, confirmed by `LlmPort` impls calling into `CompatEngine`. |
| `crates/paladin-llm/src/{kimi,qwen,grok,ollama,gemini,openai_compatible}/adapter.rs` | Six new adapters, full `LlmPort` | ✓ VERIFIED | All present, all 6 methods each, all compile + clippy-clean. |
| `.planning/decisions/0045-additional-llm-provider-selection.md` | Recorded selection study | ✓ VERIFIED | Present, criteria-before-verdicts, full candidate table. |
| `.planning/decisions/0046-facade-llm-feature-flag-wiring.md` | Flag-wiring ADR, default preserved | ✓ VERIFIED | Present; records option-b amendment and its provenance. |
| `Cargo.toml` (root) | 9 real `llm-*` flags, default preserved | ✓ VERIFIED | Confirmed by direct read — see Truth 5. |
| `crates/paladin-llm/Cargo.toml` | 9 per-provider features | ✓ VERIFIED | `default = ["openai", "mock"]` plus 9 provider flags, confirmed by direct read. |
| `crates/paladin-llm/src/provider_factory.rs` | Table-driven registry (D-10) | ✓ VERIFIED | `build_provider_registry` / `provider_registry()` single source, confirmed by direct read. |
| `tests/integration/ollama_docker_test.rs` | Docker-gated Tier 2 suite | ✓ VERIFIED (present, wired, gracefully skips) — behaviour against a real server UNCERTAIN | See Truth 8. |
| `docker/docker-compose.test.yml` | `ollama-test` service | ✓ VERIFIED | `ollama-test` / `ollama-test-init` blocks present (lines 69-110). |
| `.project/current-exports.txt` | Regenerated API-surface baseline | ⚠️ PARTIAL | Regenerated (contains `paladin::AnthropicAdapter`/`DeepSeekAdapter`/`OpenAIAdapter` newly), but the six new adapter types are **not** re-exported from `src/lib.rs` at all (WR-05, confirmed: `grep -n "pub use paladin_llm" src/lib.rs` shows only the original 3 providers + mock + embeddings). Not a failure of any of the 5 roadmap criteria as literally scoped (criterion 5 names `paladin-llm`'s own Cargo.toml/README/config docs, not the facade crate's re-exports), but a real, independently-reproduced surface inconsistency worth a maintainer decision — see Anti-Patterns. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `crates/paladin-llm/src/{kimi,qwen,grok,ollama,openai_compatible}/adapter.rs` | `crates/paladin-llm/src/compat/engine.rs` | delegates all 6 `LlmPort` methods to `CompatEngine` | ✓ WIRED | Confirmed by presence of `CompatEngine` construction/calls in each adapter; no adapter re-implements request shaping, SSE assembly, retry or error mapping. |
| `crates/paladin-llm/src/provider_factory.rs` | each of the 6 new adapter modules | one `#[cfg(feature = "...")]`-gated `construct_*` fn + registry row per provider | ✓ WIRED | Confirmed by direct read of `provider_factory.rs:107-297` — kimi, qwen, grok, gemini, openai-compatible, ollama each have a construct fn and a registry row. |
| `Cargo.toml` (root) `[features]` | `crates/paladin-llm/Cargo.toml` `[features]` | `llm-<provider> = ["paladin-llm/<provider>"]` | ✓ WIRED | Confirmed by direct read of both manifests; also independently confirmed via `cargo check -p paladin-ai --no-default-features --features llm-all` (exit 0). |
| `Cargo.toml` (root) `default` | provider_factory's default-resolvable set | `default = ["llm-openai","llm-anthropic","llm-deepseek"]` → `create("anthropic")`/`create("deepseek")` still resolve | ✓ WIRED | `default_features_still_resolve_openai_anthropic_and_deepseek` run singly, passed. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| `paladin-llm` builds with a single new provider feature | `cargo check -p paladin-llm --no-default-features --features kimi` | exit 0, 20.75s | ✓ PASS |
| `paladin-llm` builds with all 5 remaining new provider features combined | `cargo check -p paladin-llm --no-default-features --features qwen,grok,ollama,openai-compatible,gemini` | exit 0, 3.29s | ✓ PASS |
| Facade crate builds with every new `llm-*` flag | `cargo check -p paladin-ai --no-default-features --features llm-all` | exit 0, 11.78s | ✓ PASS |
| Facade crate builds under default features (behaviour-preservation check) | `cargo check -p paladin-ai` | exit 0, 9.02s | ✓ PASS |
| D-11/ADR-0046 regression test, run singly | `cargo test -p paladin-ai --test unit default_features_still_resolve_openai_anthropic_and_deepseek` | 1 passed; 0 failed | ✓ PASS |
| Full crate + all new features clippy, deny warnings | `cargo clippy -p paladin-llm --all-targets --no-default-features --features "openai,anthropic,deepseek,kimi,qwen,grok,ollama,gemini,openai-compatible,mock" -- -D warnings` | exit 0, 0 warnings | ✓ PASS |
| Facade crate + llm-all + integration-tests clippy, deny warnings | `cargo clippy -p paladin-ai --all-targets --no-default-features --features "llm-all,integration-tests" -- -D warnings` | exit 0, 0 warnings | ✓ PASS |
| `cargo doc` produces no `missing_docs` warning | `cargo doc -p paladin-llm --no-deps --no-default-features --features "openai,anthropic,deepseek,kimi,qwen,grok,ollama,gemini,openai-compatible,mock"` | 1 warning, category `private_intra_doc_links`, zero `missing_docs` | ✓ PASS (matches SUMMARY's own claim) |
| Ollama Docker-gated suite gracefully skips with no Docker present | `cargo test -p paladin-ai --no-default-features --features integration-tests,llm-ollama --test ollama_docker` | 4 passed via named `SKIP:` early-return, 0 failed | ✓ PASS (skip-contract only — see Truth 8 for the unexercised real-server behaviour) |

### Probe Execution

No `scripts/*/tests/probe-*.sh` convention exists in this Rust workspace, and no plan/SUMMARY for this phase declares a probe script. SKIPPED (no probe scripts declared for this phase).

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| PROV-01 | 17-02 | Narrow the candidate field to a recorded decision | ✓ SATISFIED | ADR-0045 + REQUIREMENTS.md amendment; see Truth 1. |
| PROV-02 | 17-01, 17-03, 17-04, 17-05, 17-07 | Full `LlmPort` contract, truthful capabilities | ⚠️ PARTIALLY SATISFIED | Contract completeness verified (Truth 2); undermined by the unresolved CR-01 security defect in the Gemini adapter's `generate`/`generate_stream` (Truth 3). |
| PROV-03 | 17-01, 17-03, 17-04, 17-05, 17-06, 17-08 | Feature-gated, additive, default unchanged | ✓ SATISFIED | See Truths 4-5. |
| PROV-04 | 17-01, 17-03, 17-04, 17-05, 17-07, 17-08 | Tested/documented to standard, advertised surface accurate | ⚠️ PARTIALLY SATISFIED | Mock-transport tests + rustdoc verified (Truth 6); coverage floor and live-endpoint behaviour genuinely unmeasured, not failing (Truths 7-8, human-accepted debt); advertised surface accurate (Truth 9). |

No orphaned requirements — REQUIREMENTS.md's Phase 17 section names exactly PROV-01..04, and all four are cited in at least one plan's `requirements:` frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| `crates/paladin-llm/src/gemini/adapter.rs` | 564-567, 622-625 | Unescaped, unvalidated `request.model` spliced into request URL (CR-01) | 🛑 Blocker | Path/query injection risk on a request carrying a live credential; see Gaps. |
| `crates/paladin-llm/src/ollama/adapter.rs` | 55-60 | Placeholder "credential" (`"ollama"`) is a common English word, causing the shared exact-match redactor to over-redact benign diagnostic text containing the substring "ollama" (WR-01) | ⚠️ Warning | Diagnostics/robustness only — over-redaction is the safe direction, not a leak. Not blocking. |
| `crates/paladin-llm/src/provider_factory.rs` | 282-297, 371-395 | `openai-compatible`'s availability check verifies only 1 of its 3 required env vars, so `list_available_providers()`/`get_default_provider()` can report it "available" when `create()` will actually fail with `ConfigurationMissing` (WR-02) | ⚠️ Warning | Caller-facing surprise on partial config; not a contract violation. Not blocking. |
| `crates/paladin-llm/src/gemini/adapter.rs` | 389-417 | `map_error` misclassifies some real-world Gemini auth-failure shapes (400/INVALID_ARGUMENT for bad keys; unrecognised 401/403 falls to a retryable arm) (WR-03) | ⚠️ Warning | Never-live-tested protocol mapping; plausible per the phase's own risk framing. Not blocking. |
| `src/lib.rs`, `Cargo.toml` (root) | 174-188, 279-288 | Six new providers have real facade feature flags but no `pub use paladin_llm::<provider>::{...}` re-export, unlike the original three (WR-05) | ⚠️ Warning | A `paladin` consumer must depend on `paladin-llm` directly to reach the six new adapter types by import; factory-mediated (string-keyed) access works fine. Out of literal scope of criterion 5 as written, but a real surface inconsistency. |
| `crates/paladin-llm/src/compat/engine.rs` | 317-357 | Retry loop's post-loop error path is structurally unreachable (WR-07) | ℹ️ Info | Dead code, not presently incorrect; a future off-by-one change could silently start exercising it. |

No `TBD`/`FIXME`/`XXX` debt markers found in any phase-17-touched file under `crates/paladin-llm/src/{kimi,qwen,grok,ollama,gemini,openai_compatible,compat,redaction.rs,provider_factory.rs,config}`.

### Human Verification Required

See `human_verification` in the frontmatter for the three items (coverage floor measurement, Ollama live-server behaviour, vendor-fact live confirmation). All three are pre-existing, explicitly human-accepted verification debt (`WINDOWS.md` ids 12/13, decided 2026-08-17 via an `AskUserQuestion` checkpoint during `/gsd-execute-phase 17`) — reported here honestly rather than claimed as met, per this verification's instructions. None of the three blocks the phase on its own; CR-01 does.

### Gaps Summary

One blocking gap: **CR-01**, an unresolved Critical-severity security defect in the Gemini adapter (`crates/paladin-llm/src/gemini/adapter.rs`). `request.model` — a caller-supplied field — is spliced unescaped into the request URL path in both `generate()` and `generate_stream()`, with no percent-encoding and no allow-list validation, on a request that carries the real `x-goog-api-key` credential. This was flagged by the phase's own post-execution code review (17-REVIEW.md, CR-01) and independently re-confirmed by direct code read during this verification pass — it is real, not a false positive, and it has not been fixed (no commit after the three plan-17-05 commits touches `gemini/adapter.rs`). This repo's own CLAUDE.md mandates a security scan of newly generated/modified first-party code; an open Critical finding of this shape in shipped adapter code is not something a "passed" verification can wave through.

Everything else checked out. The provider-selection study (ADR-0045) is a genuine recorded decision, not a wish list. All six new adapters implement the full `LlmPort` contract with no stubbed methods and truthful capabilities. The facade's feature-flag wiring is real and the default build's effective provider set is unchanged, matching D-11's amended (option-b) decision exactly, with a passing regression test proving it at runtime. The advertised surface (Cargo.toml, README, config docs) names exactly the shipped provider set. Coverage-floor measurement and Ollama's live-server behaviour remain genuinely unmeasured due to this environment's lack of Docker — that is pre-existing, human-accepted debt (WINDOWS.md ids 12/13), reported honestly here rather than claimed as passing, and it is not what is blocking this verification.

**This looks like a straightforward, isolated fix** (percent-encode or allow-list-validate `request.model` before building the URL in two call sites), not a design-level problem — the shared `compat` engine every other new adapter uses already does this correctly by carrying `model` in the JSON body instead of the URL.

---

_Verified: 2026-08-17T16:56:03Z_
_Verifier: Claude (gsd-verifier)_
