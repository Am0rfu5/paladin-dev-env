---
phase: 17-additional-llm-provider-adapters
plan: 10
subsystem: llm-provider-adapters
tags: [reqwest, redirect-policy, security, kimi, qwen, grok, ollama, gemini, openai-compatible, ssrf, wr-04]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "openai_compatible/adapter.rs's Policy::none() mitigation (T-17-18), the reference implementation this plan copies verbatim; plan 17-09's validate_model_identifier guard on Gemini, which this plan does not touch"
provides:
  - "reqwest::redirect::Policy::none() on Kimi, Qwen, Grok, Ollama and Gemini HTTP clients — a 3xx from the configured *_BASE_URL can no longer replay a credential header to a redirect target"
  - "A 300..=399 arm in CompatEngine::map_error and GeminiAdapter::map_error producing an actionable, operator-diagnosable refused-redirect error"
  - "Corrected in-code rationale on all four compat presets (kimi/qwen/grok/ollama), replacing the false 'fixed vendor host, not operator-supplied' claim"
  - "A Gemini module-doc 'Trust boundary: the operator-supplied base URL' section, sibling to plan 17-09's model-identifier section"
affects: [17-review, 17-verification, future-llm-provider-adapters]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Per-preset redirect_policy: Some(reqwest::redirect::Policy::none()) rather than inverting the engine's own None-means-default field semantics"
    - "Refused-redirect errors map into the existing LlmError::ProcessingError variant (retryable), never a new variant — PROV-02's 'no new parallel error type' rule"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/compat/engine.rs
    - crates/paladin-llm/src/kimi/adapter.rs
    - crates/paladin-llm/src/qwen/adapter.rs
    - crates/paladin-llm/src/grok/adapter.rs
    - crates/paladin-llm/src/ollama/adapter.rs
    - crates/paladin-llm/src/gemini/adapter.rs

key-decisions:
  - "Fix placed per-adapter (each preset sets its own redirect_policy), not by inverting the engine's None-means-default field semantics — reuses openai_compatible's reviewed pattern and keeps the field's documented meaning honest for a future preset that legitimately needs to follow a redirect"
  - "A 302 response to a POST request is downgraded to a bodyless GET by tower-http's follow_redirect layer (RFC 7231 6.4.2/6.4.3) — confirmed by reading reqwest 0.12.28's redirect.rs and tower-http 0.6.11's follow_redirect/mod.rs directly, not assumed; every redirect-target mock in both RED tests matches 'GET', not 'POST'"
  - "Refused-redirect errors map to the existing LlmError::ProcessingError (retryable up to max_retries) rather than a new LlmError variant, accepting a bounded retry cost against a redirecting host rather than breaching PROV-02"

patterns-established:
  - "WR-04 mitigation pattern: Policy::none() at client construction + a 300..=399 map_error arm naming the status and the base-URL setting to correct + a module-doc trust-boundary section, applied identically across five adapters"

requirements-completed: [PROV-02, PROV-04]

coverage:
  - id: D1
    description: "Kimi and Gemini both refuse to follow a 302 redirect from their configured base_url, proven behaviorally with two independent mockito servers — the redirect target records zero requests and the returned error names the refused redirect"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/kimi/adapter.rs#kimi_does_not_replay_the_authorization_header_to_a_redirect_target"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini_does_not_replay_the_api_key_header_to_a_redirect_target"
        status: pass
    human_judgment: false
  - id: D2
    description: "Qwen, Grok and Ollama carry the identical Policy::none() literal as Kimi/Gemini/openai_compatible (source-asserted, not independently exercised with a live redirect — the plan's own scope limits behavioral proof to the two construction paths)"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible,gemini (191 passed)"
        status: pass
    human_judgment: false
  - id: D3
    description: "A refused redirect surfaces as an actionable LlmError::ProcessingError naming the HTTP status and the base-URL setting to correct, not an opaque failure"
    requirement: "PROV-04"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/compat/engine.rs#map_error_maps_a_redirect_status_to_an_actionable_processing_error"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 10: Redirect-following credential replay (WR-04) Summary

**`Policy::none()` on Kimi, Qwen, Grok, Ollama and Gemini's HTTP clients, closing the redirect-following credential-replay gap WR-04 found across every adapter except the already-mitigated `openai_compatible`.**

## Performance

- **Duration:** ~55 min
- **Completed:** 2026-08-17
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments

- All five previously-unmitigated adapters (Kimi, Qwen, Grok, Ollama, Gemini) now build their HTTP client with `reqwest::redirect::Policy::none()`, matching `openai_compatible`'s already-reviewed T-17-18 mitigation — a `3xx` from the configured `*_BASE_URL` can no longer cause the credential header (`Authorization` for the four compat presets, `x-goog-api-key` for Gemini) to be replayed to a different, attacker-influenced host.
- Behavioral proof on **both** client-construction paths in the crate: `kimi_does_not_replay_the_authorization_header_to_a_redirect_target` exercises the shared `CompatEngine` client; `gemini_does_not_replay_the_api_key_header_to_a_redirect_target` exercises Gemini's bespoke client. Both assert the redirect target receives **zero** requests.
- A `300..=399` arm added to both `CompatEngine::map_error` and `GeminiAdapter::map_error` turns a refused redirect into an actionable error naming the HTTP status and the specific base-URL environment variable to correct — the plan's one prohibition (an opaque failure) is satisfied.
- The four preset comments that falsely asserted each endpoint was "a fixed vendor host, not operator-supplied" are replaced with accurate rationale citing WR-04 and naming the real, documented, operator-settable variable (`MOONSHOT_BASE_URL`, `DASHSCOPE_BASE_URL`, `XAI_BASE_URL`, `OLLAMA_BASE_URL`).
- `CompatEngineConfig.redirect_policy`'s public semantics are unchanged — `None` still preserves `reqwest`'s default, so a future preset that legitimately needs to follow a redirect still can.

## Task Commits

Each task was committed atomically:

1. **Task 1: RED — prove the credential header follows a redirect to a second host today** - `28e23d4` (test)
2. **Task 2: GREEN (compat presets) — Policy::none() for Kimi, Qwen, Grok and Ollama** - `15c29d6` (fix)
3. **Task 3: GREEN (Gemini) — Policy::none() on the bespoke client** - `9cdcb9e` (fix)

_Task 1 is a pure test-addition commit (no production-code change); the tree was genuinely red between it and Task 2._

## Files Created/Modified

- `crates/paladin-llm/src/compat/engine.rs` — `300..=399` arm in `map_error`; `redirect_policy` field doc updated; new test `map_error_maps_a_redirect_status_to_an_actionable_processing_error`
- `crates/paladin-llm/src/kimi/adapter.rs` — `redirect_policy: Some(Policy::none())`; corrected WR-04 comment; new RED test
- `crates/paladin-llm/src/qwen/adapter.rs` — `redirect_policy: Some(Policy::none())`; corrected WR-04 comment
- `crates/paladin-llm/src/grok/adapter.rs` — `redirect_policy: Some(Policy::none())`; corrected WR-04 comment
- `crates/paladin-llm/src/ollama/adapter.rs` — `redirect_policy: Some(Policy::none())`; corrected WR-04 comment (with its own placeholder-credential nuance)
- `crates/paladin-llm/src/gemini/adapter.rs` — `.redirect(Policy::none())` on the bespoke `Client::builder()`; `300..=399` arm in `map_error`; "Trust boundary: the operator-supplied base URL" module-doc section; new RED test

## Decisions Made

- **Fix placement (recorded deliberately in the plan itself):** per-adapter, not by inverting the engine's `None`-means-default `redirect_policy` field semantics. Reuses `openai_compatible`'s reviewed pattern; an engine-side inversion would be an invisible semantic change to a `pub` field and would still need a second, per-adapter fix for Gemini (bespoke client, D-08).
- **302→GET method downgrade, confirmed by direct source inspection (D-00e):** before writing the RED tests I read `reqwest` 0.12.28's `redirect.rs` and `tower-http` 0.6.11's `follow_redirect/mod.rs` (both vendored under `/usr/local/cargo/registry/src/`). Confirmed that `tower-http`'s follow_redirect layer downgrades a POST to a bodyless GET on a 301/302 response per RFC 7231 §6.4.2/6.4.3 — so both RED tests register their redirect-target mock as `"GET"`, not `"POST"`. Registering it as `"POST"` would have made the `.expect(0)` assertion trivially pass regardless of whether the vulnerability existed, since mockito's per-mock call counter only increments on an exact method+path+query match.
- **`reqwest`'s own cross-host header stripping does not make this a non-issue:** `reqwest` 0.12.28 already strips `Authorization`/`Cookie`/`Proxy-Authorization`/`WWW-Authenticate` on a cross-host redirect (`redirect.rs::remove_sensitive_headers`), but (a) `x-goog-api-key` — Gemini's credential header — is **not** in that stripped set, so Gemini's exposure is real and unmitigated by `reqwest` itself; and (b) even where `reqwest` strips the credential header, the request is still forwarded to an unconfigured host, which is the broader threat this plan's fix (refuse the redirect entirely) addresses regardless of which specific header would or wouldn't have survived.
- **Load-bearing assertion ordered first in both RED tests:** each test calls `redirect_target_mock.assert_async().await` before checking `result.is_err()`. In the RED state, the redirect is followed transparently and the redirect target answers with a well-formed response, so `result` is actually `Ok` today — the `.expect(0)` mock assertion is what needs to fail first, matching the plan's stated behavior contract ("not a panic" from an unrelated assertion).
- **`.expect_at_least(1)` used for both primary mocks**, per the plan's own guidance: `LlmError::ProcessingError` (what the fixed `map_error` arm returns) is in each engine's retryable set, so the fixed adapter may hit `primary` up to `max_retries`(+1) times, while the RED-state adapter hits it exactly once (the redirect follow succeeds on the first attempt). An exact `.expect(1)` would have been brittle across the RED→GREEN transition.

## Deviations from Plan

None — plan executed exactly as written. The two judgment calls above (mock method = GET; assertion ordering) were left open by the plan's own text ("Note the actual error text" / "say which you used in the SUMMARY") and are recorded here per D-00e rather than being deviations from an explicit instruction.

## Issues Encountered

None. The RED tests failed on the first run in exactly the way the plan predicted (`.expect(0)` mock assertion, mockito panic text "Expected 0 request(s)... but received 1"), and turned green without modification after Tasks 2 and 3 — confirmed via `git diff` showing zero deletions inside either test body between the Task 1 commit and the final state.

### RED-state mockito failure text (recorded per D-00e)

```
thread 'kimi::adapter::tests::kimi_does_not_replay_the_authorization_header_to_a_redirect_target' panicked at mockito-1.7.2/src/mock.rs:633:13:

> Expected 0 request(s) to:

GET (any)?(any)

...but received 1
```

The Gemini RED test failed with the identical mockito panic text (same mock definition shape).

## User Setup Required

None - no external service configuration required.

## Verification Evidence

- `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible,gemini` — **191 passed; 0 failed** (up from 170 at 17-05).
- `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` — clean, zero warnings.
- `cargo fmt --check -p paladin-llm` — clean.
- `cargo test -p paladin-llm` (default features) — **57 passed; 0 failed**, confirming PROV-03 (default build unaffected).
- `cargo check -p paladin-ai --no-default-features --features llm-all` — compiles clean, confirming the facade still builds with every provider.
- `git diff --stat -- crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock` — empty across all three commits (no new dependency; `reqwest::redirect::Policy` was already reachable through the crate's existing optional `reqwest` dependency).
- `git diff --stat -- crates/paladin-llm/src/openai_compatible/ crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/` — empty (D-06; reference implementation read, not edited; three shipped adapters untouched).
- **Snyk code scan: NOT RUN.** The `snyk_code_scan` MCP tool was not available in this executor's runtime. Recorded here per the plan's explicit instruction to state this plainly rather than report it as passed. A human or a runtime with the Snyk MCP tool available should scan `crates/paladin-llm/src/{compat/engine,kimi/adapter,qwen/adapter,grok/adapter,ollama/adapter,gemini/adapter}.rs` before this change ships.

## Next Phase Readiness

- WR-04 is closed across all five adapters that carry an operator-settable base URL plus a credential header, plus the already-mitigated `openai_compatible` — six of six adapters in `paladin-llm` now refuse to follow a redirect while holding a credential.
- WR-07 (the engine's unreachable post-loop error path) and IN-02 (the five presets' loose `starts_with("http")` base-URL validation) remain deliberately deferred, as instructed — neither was touched despite editing the same files.
- No blockers for phase close-out from this plan's scope.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*

## Self-Check: PASSED

- FOUND: `crates/paladin-llm/src/compat/engine.rs`
- FOUND: `crates/paladin-llm/src/kimi/adapter.rs`
- FOUND: `crates/paladin-llm/src/qwen/adapter.rs`
- FOUND: `crates/paladin-llm/src/grok/adapter.rs`
- FOUND: `crates/paladin-llm/src/ollama/adapter.rs`
- FOUND: `crates/paladin-llm/src/gemini/adapter.rs`
- FOUND commit: `28e23d4` (test(17-10): RED)
- FOUND commit: `15c29d6` (fix(17-10): GREEN compat presets)
- FOUND commit: `9cdcb9e` (fix(17-10): GREEN Gemini)
