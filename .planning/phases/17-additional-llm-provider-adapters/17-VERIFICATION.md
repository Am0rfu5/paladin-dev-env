---
phase: 17-additional-llm-provider-adapters
verified: 2026-08-17T21:15:00Z
status: gaps_found
score: 9/12 must-haves verified
behavior_unverified: 0
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 6/9
  gaps_closed:
    - "CR-01 (original): Gemini generate()/generate_stream() spliced caller-supplied request.model unescaped into the request URL path — now closed by a validate_model_identifier allow-list guard, 12 passing regression tests"
    - "WR-04 (from 17-REVIEW.md, gap-closure scope): Kimi/Qwen/Grok/Ollama/Gemini followed redirects while holding a credential header — now closed by reqwest::redirect::Policy::none() on all five clients plus openai_compatible, a 300..=399 map_error arm, and two behavioral zero-request regression tests"
    - "WR-03 (from 17-REVIEW.md, gap-closure scope): Gemini misclassified unrecognised 401/403 shapes and Google's documented 400/INVALID_ARGUMENT invalid-key shape as retryable/InvalidPrompt — now closed by an unconditional 401|403 arm plus a named GEMINI_CREDENTIAL_MESSAGE_SIGNATURES discriminator, proven end-to-end (1 request instead of 3 on an unrecognised auth failure)"
  gaps_remaining: []
  regressions: []
  new_findings:
    - "A fresh code review (17-REVIEW.md, dated after gap-closure) found a new Critical-severity defect in crates/paladin-llm/src/provider_factory.rs: get_default_provider()/list_available_providers() treat an empty-but-set credential env var as configured (std::env::var(var).is_ok(), not is_ok_and(|v| !v.trim().is_empty())). Independently reproduced in this verification pass (see gaps below) — not addressed by any of the three gap-closure plans, which were explicitly scoped to CR-01/WR-04/WR-03 only."
gaps:
  - truth: "provider_factory.rs's get_default_provider() and list_available_providers() correctly distinguish an unset credential env var from one that is set to an empty (or whitespace-only) string when deciding a provider is 'available'"
    status: failed
    reason: "Both functions use std::env::var(var).is_ok(), which is true for an env var set to the empty string. This directly contradicts list_available_providers()'s own doc comment ('compiled in and have their credential configured') and is inconsistent with the crate's own *Config::validate() methods (e.g. GrokConfig::validate() at grok/adapter.rs:117-118), which correctly reject an empty api_key — so factory.create(name) then fails with ConfigurationMissing for a provider get_default_provider()/list_available_providers() just reported as available. The crate's own provider_name_round_trip test module (provider_factory.rs:714-773) already uses the correct is_ok_and(|v| !v.trim().is_empty()) check and documents why; that fix was applied to a test but never to the two production call sites it was guarding against. Flagged as this review's one Critical finding (17-REVIEW.md, dated after the CR-01/WR-04/WR-03 gap-closure run) and independently reproduced in this verification pass: this sandbox's own ambient environment carries GEMINI_API_KEY=, XAI_API_KEY=, OPENAI_API_KEY=, ANTHROPIC_API_KEY=, DEEPSEEK_API_KEY= (all present but empty), and `cargo test --test unit --features llm-all -- provider_factory --test-threads=1` deterministically fails test_get_default_provider (expected None, got Some(\"grok\")) and test_list_available_providers (expected 0, got 3) against the live tree at verification time. Not addressed by plans 17-09/17-10/17-11 — the developer scoped that gap-closure run to CR-01 (Gemini URL injection), WR-04 (redirect credential replay) and WR-03 (Gemini auth-failure classification) only; this is new debt surfaced by a subsequent review pass, not part of that scope."
      artifacts:
        - path: "crates/paladin-llm/src/provider_factory.rs"
          issue: "get_default_provider() (:365-376) and list_available_providers() (:378-390) both use `std::env::var(var).is_ok()` to decide a provider's credential is configured, matching \"set\" rather than \"set to a non-empty, non-whitespace value\". A common deployment pattern (.env templates, Docker/K8s environments passing through unset host vars as empty strings — this repository's own .env is itself an example) makes an unconfigured provider read back as available, and can select an unusable provider as the reported \"default\"."
      missing:
        - "Apply the same is_ok_and(|v| !v.trim().is_empty()) check the crate's own provider_name_round_trip test already uses, in get_default_provider() and list_available_providers()"
        - "Either merge test_get_default_provider's/test_list_available_providers's CleanProviderEnv guard with CleanNewProviderEnv (or acquire both) so these two tests clear all nine provider credential vars, not just the original three — both functions scan the whole registry"
        - "A regression test proving an env var set to the empty string is treated as absent by both functions (the crate already has the pattern to copy: provider_name_round_trip's is_ok_and usage)"
human_verification:
  - test: "Run `snyk_code_scan` (or the Snyk CLI) over every file plans 17-09/17-10/17-11 modified: crates/paladin-llm/src/{gemini/adapter.rs,kimi/adapter.rs,qwen/adapter.rs,grok/adapter.rs,ollama/adapter.rs,compat/engine.rs}. Fix and rescan until clean, per .github/instructions/snyk_rules.instructions.md (imported into CLAUDE.md as a mandatory scan for new/modified first-party code)."
    expected: "Snyk reports no unresolved issues on the modified adapter/engine files, or any findings are fixed and a clean rescan is recorded."
    why_human: "The snyk_code_scan MCP tool and the Snyk CLI were unavailable in every one of the three gap-closure executors' worktrees (no network egress) and are equally unavailable to this verifier's own runtime — confirmed by the absence of a snyk_code_scan tool in this session's tool list. All three SUMMARYs (17-09, 17-10, 17-11) record the scan as explicitly not-run, never as passed, per the plans' own instructions. WINDOWS.md records this as open debt for 17-09 (id 15) and 17-10 (id 16); no matching id exists yet for 17-11's identical not-run scan, a minor tracking gap worth closing alongside the code fix."
  - test: "Run `make coverage` (or `cargo llvm-cov --workspace --features integration-tests --fail-under-lines 82`) in an environment with Redis and MinIO reachable via Docker, with all nine provider features compiled in, and confirm the workspace stays at or above the 82% line-coverage floor (ADR-0006)."
    expected: "cargo llvm-cov reports >= 82% workspace line coverage with the nine adapters' code (including the three plans' new regression tests) counted, not excluded."
    why_human: "This verification sandbox has no Docker daemon; make coverage's own preflight fails fast on unreachable Redis (6380) and MinIO (9010). Genuinely UNMEASURED, not failing — unchanged since the prior verification pass. Tracked as WINDOWS.md id 13, human-accepted debt (AskUserQuestion checkpoint, 2026-08-17)."
  - test: "Run tests/integration/ollama_docker_test.rs against a real ollama/ollama container with qwen2.5:0.5b pulled (docker compose -f docker/docker-compose.test.yml up ollama-test ollama-test-init), then cargo test -p paladin-ai --no-default-features --features integration-tests,llm-ollama --test ollama_docker."
    expected: "All 4 tests (generate round-trip, streaming, get_available_models, validate_model) exercise the real server and pass with real token usage / real model list data, not the SKIP path."
    why_human: "No Docker daemon in this sandbox. Unchanged since the prior verification pass — confirmed again that the suite gracefully SKIPs with a named SKIP: message rather than failing or silently passing. Tracked as WINDOWS.md id 12, same human-accepted debt."
  - test: "Smoke-test the recorded base URLs and default/fallback model IDs for Kimi, Qwen, Grok and Gemini (README.md / config.example.yml / docs/src/getting-started/configuration.md all carry an explicit 'not verified against a live endpoint' caveat) against each vendor's real API using a live credential."
    expected: "Each vendor's documented base_url resolves, the default model ID exists, and get_available_models()'s live-fetch path (not just the curated fallback) returns a real, well-formed model list."
    why_human: "No network egress / no vendor API keys available in this sandbox. Unchanged since the prior verification pass — these facts remain taken from vendor documentation, never confirmed live."
---

# Phase 17: Additional LLM Provider Adapters Verification Report

**Phase Goal:** Paladin talks to the providers its users actually deploy — the candidate field is
narrowed to a shortlist against recorded criteria, and every provider that survives ships as a
feature-gated adapter meeting the same `LlmPort` contract the existing three do.

**Verified:** 2026-08-17T21:15:00Z
**Status:** gaps_found
**Re-verification:** Yes — after gap-closure plans 17-09 (CR-01), 17-10 (WR-04) and 17-11 (WR-03)

## Re-verification Summary

The three gap-closure plans **did what they claimed**. This is not trust in the SUMMARYs — every
closure was independently re-derived from the live tree and re-run in this pass:

- `validate_model_identifier` exists in `crates/paladin-llm/src/gemini/adapter.rs` (grep-confirmed
  at both call sites, `generate()`:701 and `generate_stream()`:760), and all 12 of plan 17-09's
  named regression tests pass when run directly:
  `cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_rejects gemini::adapter::tests::generate_stream_rejects gemini::adapter::tests::validate_model_identifier gemini::adapter::tests::generate_accepts_a_model_whose_characters_are_all_in_the_allowed_set`
  → **12 passed; 0 failed**.
- `redirect_policy: Some(reqwest::redirect::Policy::none())` is present in all five adapters plan
  17-10 modified (`kimi`, `qwen`, `grok`, `ollama` via grep; `gemini` via
  `.redirect(reqwest::redirect::Policy::none())` at `gemini/adapter.rs:277`), plus the pre-existing
  `openai_compatible`. Both end-to-end behavioral proofs pass when run directly:
  `kimi::adapter::tests::kimi_does_not_replay_the_authorization_header_to_a_redirect_target` and
  `gemini::adapter::tests::gemini_does_not_replay_the_api_key_header_to_a_redirect_target` →
  **1 passed each; 0 failed**.
- Gemini's `map_error` now has an unconditional `401 | 403 =>` arm (grep-confirmed, no `if` guard)
  and a `GEMINI_CREDENTIAL_MESSAGE_SIGNATURES` discriminator (grep-confirmed, definition + one use).
  All 6 of plan 17-11's named tests pass when run directly, including the end-to-end proof that an
  unrecognised auth failure is attempted **exactly once**:
  `gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure` →
  **1 passed; 0 failed**.
- Full crate-scoped six-preset test run: `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` → **197 passed; 0 failed** (up from 170 at the original 17-05 baseline, 191 after 17-10, 197 after 17-11 per the SUMMARYs — matches).
- `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` → exit 0, no warnings.
- `cargo fmt --check -p paladin-llm` → exit 0.
- `cargo test -p paladin-llm` (default features) → **57 passed; 0 failed** — the default build (`openai`, `mock`) is unaffected by any of the three gap-closure plans.
- `cargo check -p paladin-ai --no-default-features --features llm-all` and `cargo check -p paladin-ai` (default) → both exit 0.

**However**, a fresh code review dated after the gap-closure run (`17-REVIEW.md`, committed) surfaced
**one new, unrelated Critical-severity finding** — a defect in `provider_factory.rs`'s availability
checks, not in any of the three closed findings' files — and this verification pass **independently
reproduced it live**, not merely trusted the review's claim (see Truth 11 / Gaps below). That is
what keeps this re-verification at `gaps_found` rather than `passed`.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A recorded provider-selection study evaluates candidates against explicit written criteria, with every candidate carrying one build/defer/reject verdict; Kimi, Gemini, Qwen and Meta/Llama each explicitly dispositioned | ✓ VERIFIED | `.planning/decisions/0045-additional-llm-provider-selection.md` present and unchanged since the prior verification pass; `REQUIREMENTS.md`'s PROV-01 amendment (lines ~3428-3442) still records the verdict summary. Not touched by any gap-closure plan. |
| 2 | Every build-list adapter (Kimi, Qwen, Grok, Ollama, Gemini, generic openai-compatible) implements all six `LlmPort` methods with no stubbed body and no optimistic capability response | ✓ VERIFIED | Re-confirmed by direct read and by the full crate-scoped test run (197 passed). No gap-closure plan removed or stubbed a method; `git diff --stat -- crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/` remains empty across all three plans (D-06 honored). |
| 3 | **CR-01 (original, blocking):** Gemini `generate()`/`generate_stream()` no longer splice a caller-supplied `request.model` unescaped into the request URL path | ✓ VERIFIED — CLOSED | `validate_model_identifier` (private, ASCII allow-list `[A-Za-z0-9._-]`) is called as the first statement of both methods (`gemini/adapter.rs:701`, `:760`), confirmed by direct read. 12 regression tests (6 behavioral zero-request rejections + 1 positive control + 5 pure-logic guard tests) run independently in this pass: **12 passed; 0 failed**. No dependency added (`git diff --stat -- crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock` empty across the 17-09 commits). |
| 4 | **WR-04:** Kimi, Qwen, Grok, Ollama and Gemini all refuse to follow a redirect that could replay a credential header to a different host | ✓ VERIFIED — CLOSED | `redirect_policy: Some(reqwest::redirect::Policy::none())` present in all four compat presets plus `openai_compatible` (grep-confirmed, 5 hits); Gemini's bespoke client carries `.redirect(reqwest::redirect::Policy::none())` (`gemini/adapter.rs:277`). Both end-to-end proofs (`kimi_does_not_replay_the_authorization_header_to_a_redirect_target`, `gemini_does_not_replay_the_api_key_header_to_a_redirect_target`) pass independently in this pass. A `300..=399` arm exists in both `CompatEngine::map_error` and `GeminiAdapter::map_error`, each naming the refused redirect and the base-URL variable to correct (per-plan acceptance criteria, re-confirmed by grep). |
| 5 | **WR-03:** Gemini classifies every 401/403 (any RPC status, or an unparseable envelope) as `AuthenticationError`, and Google's documented 400/INVALID_ARGUMENT invalid-key shape is distinguished from a genuine bad-prompt 400 via a narrow named signature list; an unrecognised auth failure is attempted exactly once | ✓ VERIFIED — CLOSED | `401 | 403 =>` is unconditional (no `if` guard, grep-confirmed); `GEMINI_CREDENTIAL_MESSAGE_SIGNATURES` + `is_credential_failure_message` present (grep-confirmed: 2 uses of the constant, 1 definition of the function). `generate_does_not_retry_an_unrecognised_authentication_failure` passes independently in this pass: the mock records **exactly 1** request, not the pre-fix 3. The two control tests (`map_error_400_invalid_argument_with_a_prompt_complaint_still_maps_to_invalid_prompt`, `map_error_400_echoing_the_api_key_header_name_still_maps_to_invalid_prompt`) both still pass, proving the discriminator did not become a catch-all. `RESOURCE_EXHAUSTED`, the `300..=399` arm (WR-04) and `validate_model_identifier` (CR-01) all survive the rework, grep-confirmed at their expected counts. |
| 6 | Each new provider is feature-gated: `cargo build -p paladin-llm --no-default-features --features <provider>` succeeds for every provider, individually and combined; effective default provider set unchanged | ✓ VERIFIED | `cargo check -p paladin-ai --no-default-features --features llm-all` → exit 0; `cargo check -p paladin-ai` (default) → exit 0; `cargo test -p paladin-llm` (default) → 57 passed. `Cargo.toml:268` `default = ["llm-openai", "llm-anthropic", "llm-deepseek"]`, unchanged. |
| 7 | Mock-transport unit tests cover request shaping, response parsing, streaming chunk assembly and error mapping for every new adapter; every public item carries rustdoc | ✓ VERIFIED | `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` → 197 passed (up from 170 pre-gap-closure). `cargo doc -p paladin-llm --no-deps` with all provider features → 3 warnings, all `rustdoc::private_intra_doc_links` (two new ones from the gap-closure trust-boundary doc sections linking to private `map_error`/`GeminiResponse`), **zero `missing_docs`** warnings. |
| 8 | The workspace stays at or above the 82% line-coverage floor with the new provider code counted (ADR-0006) | ? UNCERTAIN | Unchanged since the prior pass — no Docker daemon in this sandbox; `make coverage`'s preflight fails fast on unreachable Redis/MinIO. Routed to Human Verification. |
| 9 | Live-API / real-endpoint behaviour is exercised for the credential-gated and Docker-gated tests | ? UNCERTAIN | Unchanged since the prior pass — Ollama Docker suite still gracefully SKIPs (no Docker daemon); vendor base URLs/model IDs still carry an explicit "not verified against a live endpoint" caveat. Routed to Human Verification. |
| 10 | The advertised surface (`paladin-llm`'s Cargo.toml description/keywords, crate README, configuration docs) names exactly the providers that exist | ✓ VERIFIED | Re-confirmed by direct read: `Cargo.toml` description/keywords and README's provider table are unchanged and still list exactly the 9 named providers + generic + mock; none of the gap-closure plans touched these files. |
| 11 | **New (this pass):** `provider_factory.rs`'s `get_default_provider()`/`list_available_providers()` correctly treat an unset credential env var, and only an unset one, as "not configured" | ✗ FAILED | `std::env::var(var).is_ok()` is `true` for an env var set to the empty string. Independently reproduced in this sandbox: `cargo test --test unit --features llm-all -- provider_factory --test-threads=1` → `test_get_default_provider` FAILED (expected `None`, got `Some("grok")`), `test_list_available_providers` FAILED (expected `0`, got `3`) — because this environment's own ambient `.env` carries `GEMINI_API_KEY=`, `XAI_API_KEY=`, `OPENAI_API_KEY=`, `ANTHROPIC_API_KEY=`, `DEEPSEEK_API_KEY=` (all present but empty). Flagged Critical by `17-REVIEW.md`, dated after the gap-closure run; not in scope of plans 17-09/17-10/17-11 (developer-scoped to CR-01/WR-04/WR-03 only). See Gaps below. |
| 12 | New/modified first-party code from this phase (including the three gap-closure plans) has been scanned for security defects per CLAUDE.md's mandatory Snyk posture | ? UNCERTAIN | The `snyk_code_scan` tool is unavailable to this verifier's own runtime (not present in this session's tool list), exactly as it was to all three gap-closure executors. All three SUMMARYs record the scan as explicitly **not run**, never as passed. Routed to Human Verification. |

**Score:** 9/12 truths verified (1 failed — new provider_factory.rs availability defect; 2 uncertain — coverage floor and live-endpoint behaviour, unchanged pre-existing human-accepted debt; 1 uncertain — Snyk scan not run for any of the three gap-closure plans, a CLAUDE.md compliance gap)

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `crates/paladin-llm/src/gemini/adapter.rs` — `validate_model_identifier` | Private allow-list guard, called first in both URL-building methods | ✓ VERIFIED | Confirmed present, wired at both call sites, 12/12 regression tests pass. |
| `crates/paladin-llm/src/{kimi,qwen,grok,ollama,gemini}/adapter.rs` — redirect policy | `Policy::none()` on every client that carries a credential header + operator-settable base URL | ✓ VERIFIED | Confirmed present in all five plus `openai_compatible`; both behavioral proofs pass. |
| `crates/paladin-llm/src/gemini/adapter.rs` — `GEMINI_CREDENTIAL_MESSAGE_SIGNATURES` / `is_credential_failure_message` | Named, narrow credential-shape discriminator | ✓ VERIFIED | Confirmed present, correctly ordered before the plain `INVALID_ARGUMENT` arm, both control tests pass. |
| `crates/paladin-llm/src/provider_factory.rs` | Table-driven registry with correct availability semantics | ⚠️ DEFECTIVE | Registry structure is sound (D-10), but `get_default_provider()`/`list_available_providers()` misreport availability for an empty-but-set env var — reproduced live in this pass. Not a gap-closure-plan regression; a pre-existing defect a fresh review surfaced. |
| `.planning/decisions/0045-...md`, `0046-...md` | Recorded selection study, flag-wiring ADR | ✓ VERIFIED | Both present, unchanged since the prior pass. |
| `Cargo.toml` (root), `crates/paladin-llm/Cargo.toml` | 9 real `llm-*` / provider flags, default preserved | ✓ VERIFIED | Confirmed by direct read. |
| `tests/integration/ollama_docker_test.rs`, `docker/docker-compose.test.yml` | Docker-gated Tier 2 suite | ✓ VERIFIED (present, wired, gracefully skips) — behaviour against a real server still UNCERTAIN | Unchanged since the prior pass. |
| `17-REVIEW.md` | Fresh code review post-gap-closure | ✓ VERIFIED | Present, committed (`1d27648`), 1 Critical + 4 Warning + 1 Info; the Critical is independently reproduced in this pass (see Truth 11). |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `gemini/adapter.rs generate()`/`generate_stream()` | `gemini/adapter.rs validate_model_identifier` | called as the first statement of each method | ✓ WIRED | Confirmed by direct read and 12 passing tests. |
| `{kimi,qwen,grok,ollama}/adapter.rs` | `compat/engine.rs CompatEngine::new` | `redirect_policy: Some(Policy::none())` passed into `CompatEngineConfig` | ✓ WIRED | Confirmed by grep across all four presets + `openai_compatible`. |
| `gemini/adapter.rs map_error` | `gemini/adapter.rs execute_with_retry` | `AuthenticationError` is in the existing non-retryable set, made correct by the now-unconditional `401 | 403` arm | ✓ WIRED | Confirmed: `generate_does_not_retry_an_unrecognised_authentication_failure` passes, mock records exactly 1 request. |
| `provider_factory.rs get_default_provider()`/`list_available_providers()` | `std::env::var` | credential-presence check | ⚠️ WIRED BUT INCORRECT | Wired and reachable, but the check's semantics (`is_ok()` vs. the needed `is_ok_and(|v| !v.trim().is_empty())`) are wrong — reproduced live. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Gemini rejects a hostile model identifier before any URL is built (CR-01 closed) | `cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_rejects gemini::adapter::tests::generate_stream_rejects gemini::adapter::tests::validate_model_identifier gemini::adapter::tests::generate_accepts_a_model_whose_characters_are_all_in_the_allowed_set` | 12 passed; 0 failed | ✓ PASS |
| Kimi does not replay `Authorization` to a redirect target (WR-04 closed) | `cargo test -p paladin-llm --no-default-features --features kimi -- kimi::adapter::tests::kimi_does_not_replay_the_authorization_header_to_a_redirect_target --exact` | 1 passed; 0 failed | ✓ PASS |
| Gemini does not replay `x-goog-api-key` to a redirect target (WR-04 closed) | `cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::gemini_does_not_replay_the_api_key_header_to_a_redirect_target --exact` | 1 passed; 0 failed | ✓ PASS |
| An unrecognised Gemini auth failure is attempted exactly once, not retried (WR-03 closed) | `cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure --exact` | 1 passed; 0 failed | ✓ PASS |
| Gemini auth-classification controls (credential vs. genuine bad-prompt 400) | `cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::map_error_401_unauthenticated_maps_to_authentication_error gemini::adapter::tests::map_error_403_with_no_parseable_envelope_maps_to_authentication_error gemini::adapter::tests::map_error_400_invalid_argument_naming_an_invalid_api_key_maps_to_authentication_error gemini::adapter::tests::map_error_400_invalid_argument_with_a_prompt_complaint_still_maps_to_invalid_prompt gemini::adapter::tests::map_error_400_echoing_the_api_key_header_name_still_maps_to_invalid_prompt` | 5 passed; 0 failed | ✓ PASS |
| Full six-preset crate-scoped suite | `cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"` | 197 passed; 0 failed | ✓ PASS |
| Crate-scoped clippy, deny warnings | `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` | exit 0, 0 warnings | ✓ PASS |
| `cargo fmt --check -p paladin-llm` | — | exit 0 | ✓ PASS |
| Default-feature crate/facade builds unaffected | `cargo test -p paladin-llm`; `cargo check -p paladin-ai`; `cargo check -p paladin-ai --no-default-features --features llm-all` | 57 passed; exit 0; exit 0 | ✓ PASS |
| **New Critical reproduced:** empty-but-set credential env vars misread as configured | `cargo test --test unit --features llm-all -- provider_factory --test-threads=1` | 9 passed; **2 failed** (`test_get_default_provider`, `test_list_available_providers`) | ✗ FAIL |
| `cargo doc` produces no `missing_docs` warning | `cargo doc -p paladin-llm --no-deps --no-default-features --features "openai,anthropic,deepseek,kimi,qwen,grok,ollama,gemini,openai-compatible,mock"` | 3 warnings, all `private_intra_doc_links`, zero `missing_docs` | ✓ PASS |

### Probe Execution

No `scripts/*/tests/probe-*.sh` convention exists in this Rust workspace, and no plan/SUMMARY for
this phase declares a probe script. SKIPPED (no probe scripts declared for this phase).

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| PROV-01 | 17-02 | Narrow the candidate field to a recorded decision | ✓ SATISFIED | Unchanged since the prior pass. |
| PROV-02 | 17-01, 17-03, 17-04, 17-05, 17-07, 17-09, 17-11 | Full `LlmPort` contract, truthful capabilities, distinguishable rate-limit/auth failures | ✓ SATISFIED | CR-01 and WR-03 are both closed with passing regression tests; auth/rate-limit distinguishability is now correct for Gemini specifically, which was the requirement's own named clause. |
| PROV-03 | 17-01, 17-03, 17-04, 17-05, 17-06, 17-08, 17-10 | Feature-gated, additive, default unchanged, provider_factory resolves consistently | ⚠️ PARTIALLY SATISFIED | Feature-gating and default-preservation hold (Truth 6). But `provider_factory.rs`'s availability-reporting functions are demonstrably incorrect for both old and new providers alike under a common deployment pattern (empty-but-set credential vars) — reproduced live. This undermines the requirement's operational promise even though it does not single out new providers for worse treatment than old ones. |
| PROV-04 | 17-01, 17-03, 17-04, 17-05, 17-07, 17-08, 17-09, 17-10, 17-11 | Tested/documented to standard, advertised surface accurate | ⚠️ PARTIALLY SATISFIED | Mock-transport tests + rustdoc verified; advertised surface accurate. Coverage floor and live-endpoint behaviour remain genuinely unmeasured (unchanged, human-accepted debt). Snyk scan — mandated by CLAUDE.md for new/modified first-party code — was not run for any of the three gap-closure plans' changes. |

No orphaned requirements — `REQUIREMENTS.md`'s Phase 17 section names exactly PROV-01..04, all four cited in plan frontmatter across the 11 plans.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| `crates/paladin-llm/src/provider_factory.rs` | 365-390 | `std::env::var(var).is_ok()` treats an empty-but-set credential env var as configured | 🛑 Blocker (new, this pass) | Misreports provider availability; independently reproduced live. See Gaps. |
| `crates/paladin-llm/src/provider_factory.rs` | 352-363 | `LlmProviderFactory::create()` does not accept the `openai_compatible` (underscore) alias that `LlmConfig` does (WR-01, 17-REVIEW.md) | ⚠️ Warning | Config-layer/factory-layer inconsistency for one specific alias spelling. Developer-accepted debt, out of gap-closure scope; not independently re-verified in this pass beyond confirming the code is unchanged. |
| `crates/paladin-llm/src/openai_compatible/adapter.rs` | 222-248 | `parse_temperature_range_env` accepts an inverted min>max range (WR-02, 17-REVIEW.md) | ⚠️ Warning | Same — developer-accepted debt, out of gap-closure scope. |
| `crates/paladin-llm/src/gemini/adapter.rs` | 400-429 | `parse_response` lacks the truncated/empty-completion detection every `CompatEngine` preset has (WR-03 in the *new* review, distinct from the closed WR-03 from the original review) | ⚠️ Warning | Note the ID collision: `17-REVIEW.md`'s WR-03 is a **different finding** from the originally-closed WR-03 (auth-failure classification). Developer-accepted debt, out of gap-closure scope. |
| `crates/paladin-llm/src/gemini/adapter.rs`, `compat/engine.rs` | 756-809, 486-582 | `generate_stream()` never retries a transient stream-open failure the way `generate()` does (WR-04 in the new review, distinct from the closed WR-04 credential-replay finding) | ⚠️ Warning | Same ID-collision note. Developer-accepted debt, out of gap-closure scope. |
| `.project/current-exports.txt` | — | Regenerated under default features only; the six new adapter types are not re-exported from `src/lib.rs` (carried forward from the prior verification, restated as IN-01 in the new review) | ⚠️ Warning | Unchanged since the prior pass; not blocking. |

**Note on finding-ID collisions:** `17-REVIEW.md`'s post-gap-closure review reused the labels
CR-01/WR-03/WR-04 for entirely new findings, distinct from the original CR-01/WR-03/WR-04 that
plans 17-09/17-10/17-11 closed. This report disambiguates by description, not by label alone, to
avoid a false "still open" or false "already closed" read.

No `TBD`/`FIXME`/`XXX` debt markers found in any of the files touched by plans 17-09/17-10/17-11
(`gemini/adapter.rs`, `kimi/adapter.rs`, `qwen/adapter.rs`, `grok/adapter.rs`, `ollama/adapter.rs`,
`compat/engine.rs`) or in `provider_factory.rs`.

### Human Verification Required

See `human_verification` in the frontmatter for four items: the Snyk scan (never run for any of
the three gap-closure plans, and unavailable to this verifier too — a CLAUDE.md compliance gap
worth escalating rather than silently carrying forward), the workspace coverage-floor measurement,
Ollama's live-server behaviour, and the vendor base-URL/model-ID live smoke test. The latter three
are unchanged, pre-existing, human-accepted debt (`WINDOWS.md` ids 12/13). None of the four blocks
the phase on its own; the new `provider_factory.rs` defect does.

### Gaps Summary

**The three gaps this re-verification run was scoped to close are genuinely closed.** CR-01
(Gemini model-identifier URL injection), WR-04 (redirect-following credential replay across five
adapters) and WR-03 (Gemini auth-failure misclassification and doomed-request retry) are each
verified in the live tree with independently-run, passing regression tests — not merely claimed by
the SUMMARYs. All 197 crate-scoped tests pass, clippy and fmt are clean, and the default build
(`openai`, `anthropic`, `deepseek`) is unaffected.

**One new gap blocks a clean pass.** A fresh code review dated after the gap-closure run
(`17-REVIEW.md`) found a Critical-severity defect in `crates/paladin-llm/src/provider_factory.rs`:
`get_default_provider()` and `list_available_providers()` decide a provider's credential is
"configured" using `std::env::var(var).is_ok()`, which is `true` for an env var set to the empty
string. This is not a claim taken on trust — this verification pass independently reproduced it in
this very sandbox: the ambient environment carries five provider credential env vars set to empty
strings, and `cargo test --test unit --features llm-all -- provider_factory --test-threads=1`
deterministically fails both `test_get_default_provider` (expects `None`, gets `Some("grok")`) and
`test_list_available_providers` (expects `0`, gets `3`). This was not addressed by any of the three
gap-closure plans, which were explicitly and correctly scoped by the developer to CR-01/WR-04/WR-03
only — it is new information this verification pass is obligated to report rather than silently
absorb into a passing score because the three originally-blocking items closed.

**This looks like a small, well-scoped fix**, matching the reviewer's own suggested patch: swap
`std::env::var(var).is_ok()` for `std::env::var(var).is_ok_and(|v| !v.trim().is_empty())` at both
call sites, and widen the two affected tests' environment guard to cover all nine provider
credential vars (not just the original three). It does not touch any of the three closed findings'
files.

Everything else from the prior verification pass holds unchanged: the provider-selection study
(ADR-0045), the full `LlmPort` contract across all six adapters, feature-gating and default-set
preservation, mock-transport test coverage, rustdoc completeness (zero `missing_docs`), and the
advertised-surface accuracy. Coverage-floor measurement and Ollama's live-server behaviour remain
the same pre-existing, human-accepted debt as before. The Snyk scan mandated by CLAUDE.md for
new/modified first-party code was not run for any of the three gap-closure plans' changes — flagged
here plainly, as instructed, rather than waved through.

---

_Verified: 2026-08-17T21:15:00Z_
_Verifier: Claude (gsd-verifier)_
