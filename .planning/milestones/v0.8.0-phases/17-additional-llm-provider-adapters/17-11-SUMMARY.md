---
phase: 17-additional-llm-provider-adapters
plan: 11
subsystem: llm-providers
tags: [rust, gemini, error-handling, security, credential-handling, tdd]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "17-09's validate_model_identifier guard and 17-10's 300..=399 refused-redirect map_error arm — both must survive this plan's rework of the same file"
provides:
  - "Gemini map_error now classifies every 401/403 as AuthenticationError unconditionally, whatever the RPC status string or whether the envelope parses"
  - "GEMINI_CREDENTIAL_MESSAGE_SIGNATURES + is_credential_failure_message: named, narrow discriminator separating a credential-shaped 400/INVALID_ARGUMENT from a genuine bad-prompt one"
  - "generate() attempts an unrecognised auth failure exactly once instead of retrying it (proven end-to-end against a mock transport)"
affects: [17-additional-llm-provider-adapters]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Named, documented, narrowly-matched provider-message signature list (mirrors crate::anthropic::adapter::ANTHROPIC_USAGE_CAP_SIGNATURE), paired with a mandatory over-trigger control test for any future addition"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/gemini/adapter.rs

key-decisions:
  - "401 | 403 arm made unconditional (dropped the `if rpc_status == Some(\"PERMISSION_DENIED\")` guard) — any 401/403, whatever the RPC status string or an unparseable envelope, now maps to AuthenticationError, matching execute_with_retry's existing non-retryable set."
  - "New 400/INVALID_ARGUMENT credential arm ordered BEFORE the existing plain INVALID_ARGUMENT arm, guarded by is_credential_failure_message(raw_message) — a credential-shaped 400 wins; every other 400/INVALID_ARGUMENT still returns InvalidPrompt."
  - "GEMINI_CREDENTIAL_MESSAGE_SIGNATURES deliberately excludes the hyphenated header name x-goog-api-key — an echoed request body carries that string for reasons unrelated to credential validity; pinned by a dedicated over-trigger control test."
  - "The discriminator reads raw_message (pre-redaction, pre-truncation) so a signature past the excerpt's char budget is not missed, but only the redacted+bounded excerpt is ever emitted in the resulting error — redact-then-bound ordering preserved, no second redaction implementation added."

requirements-completed: [PROV-02, PROV-04]

coverage:
  - id: D1
    description: "Every Gemini 401/403 classifies as AuthenticationError regardless of RPC status string or envelope parseability, closing the retry-on-auth-failure defect (WR-03)"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_401_unauthenticated_maps_to_authentication_error"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_403_with_no_parseable_envelope_maps_to_authentication_error"
        status: pass
      - kind: integration
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure"
        status: pass
    human_judgment: false
  - id: D2
    description: "A 400/INVALID_ARGUMENT naming an invalid API key maps to AuthenticationError; a genuine bad-prompt 400, and a 400 merely echoing the x-goog-api-key header name, both still map to InvalidPrompt"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_400_invalid_argument_naming_an_invalid_api_key_maps_to_authentication_error"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_400_invalid_argument_with_a_prompt_complaint_still_maps_to_invalid_prompt"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_400_echoing_the_api_key_header_name_still_maps_to_invalid_prompt"
        status: pass
    human_judgment: false
  - id: D3
    description: "Rate-limit classification (429, RESOURCE_EXHAUSTED) and 17-10's refused-redirect arm and 17-09's model-identifier guard all survive byte-equivalent behavior"
    requirement: "PROV-04"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_429_with_resource_exhausted_status_maps_to_rate_limit_exceeded"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::map_error_non_429_with_resource_exhausted_status_still_maps_to_rate_limit_exceeded"
        status: pass
      - kind: other
        ref: "grep -c 'validate_model_identifier(&request.model)?' returns 2; grep -c '300..=399' returns 1"
        status: pass
    human_judgment: false

duration: 10min
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 11: Gemini credential-failure classification (WR-03) Summary

**Every Gemini 401/403, and every 400/INVALID_ARGUMENT naming a credential via a named five-entry signature list, now classifies as `LlmError::AuthenticationError` — closing the defect where an unrecognised auth failure was retried three times, each attempt re-transmitting a live `x-goog-api-key` to an endpoint that had already rejected it.**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-08-17T20:05:11Z (base commit `bbfcd75`)
- **Completed:** 2026-08-17T20:15:39Z
- **Tasks:** 2 (RED, GREEN)
- **Files modified:** 1 (`crates/paladin-llm/src/gemini/adapter.rs`)

## Accomplishments
- Closed WR-03: an unrecognised Gemini auth failure (any 401/403 not carrying `PERMISSION_DENIED`, or an unparseable error envelope) no longer falls through to the retryable `ProcessingError` catch-all.
- Added `GEMINI_CREDENTIAL_MESSAGE_SIGNATURES` and `is_credential_failure_message` — a named, narrow, documented discriminator (following the crate's `ANTHROPIC_USAGE_CAP_SIGNATURE` precedent) that separates Google's documented invalid-key `400`/`INVALID_ARGUMENT` shape from a genuine bad-prompt `400`.
- Proved end-to-end, against a mock transport, that an unrecognised auth failure is now attempted exactly once (`generate_does_not_retry_an_unrecognised_authentication_failure`).
- Verified both 17-09's `validate_model_identifier` guard and 17-10's `300..=399` refused-redirect arm survive the rework unchanged.

## Task Commits

Each task was committed atomically:

1. **Task 1: RED — tests proving an unrecognised auth failure is retried and a bad key reads as a bad prompt** - `ee37de6` (test)
2. **Task 2: GREEN — unconditional 401/403 auth mapping and a named credential-signature discriminator on 400** - `a772305` (fix)

_TDD plan: RED commit then GREEN commit, no REFACTOR commit needed — the GREEN implementation was already clean on first pass (clippy, 0 warnings)._

## Files Created/Modified
- `crates/paladin-llm/src/gemini/adapter.rs` - Added `GEMINI_CREDENTIAL_MESSAGE_SIGNATURES` constant, `is_credential_failure_message` free function, rewrote `map_error`'s `401 | 403` arm (unconditional) and added a credential-shaped `400`/`INVALID_ARGUMENT` arm ordered before the existing plain one; updated `map_error`'s doc comment; added 6 regression tests (2 RED-state failing controls already passed, 4 newly-failing-then-fixed).

## Decisions Made
- **401/403 arm made unconditional.** Dropped the `if rpc_status == Some("PERMISSION_DENIED")` guard entirely — the arm now fires on the HTTP status alone. This is what makes `execute_with_retry`'s pre-existing non-retryable set (which already halts on `AuthenticationError`) actually correct for the shapes Google returns in practice (`UNAUTHENTICATED`, or no envelope at all from a proxy/gateway).
- **Credential-shape discriminator arm ordered first, evaluated before the plain `INVALID_ARGUMENT` arm.** Mutually exclusive by construction (Rust match arm ordering) — a `400` either matches the credential signature and returns `AuthenticationError`, or falls through to the unchanged `InvalidPrompt` arm. No `if`/`else` branching duplicated the guard logic.
- **Signature list deliberately narrow (5 entries) and excludes `x-goog-api-key`.** An echoed request body carries the header's literal name for reasons unrelated to credential validity (e.g. a proxy restating the rejected header); matching on it would send an operator to rotate a working key. The over-trigger control test pins this and the doc comment states that any future addition must ship its own control test.
- **Discriminator reads `raw_message`, not `excerpt`.** `bounded_excerpt` truncates at `RESPONSE_EXCERPT_CHAR_BUDGET`; a signature past that boundary would be missed if the discriminator read the truncated form. Only the redacted+bounded `excerpt` is ever emitted in the resulting `LlmError` — the raw/excerpt split is commented at the call site so a future edit does not read it as an oversight.

## Deviations from Plan

None — plan executed exactly as written. One factual note on the plan's own prose (not a deviation, since the acceptance criteria used the correct number): the plan's `<behavior>` section states the RED-state mock "records four" requests; the actual RED-state failure text (recorded below, D-00e) shows **3** received requests, not 4 — `execute_with_retry`'s loop calls `operation()` exactly `max_retries` (3) times before giving up, not `max_retries + 1`. This does not change any acceptance criterion, since the `.expect(1)` assertion and its GREEN-state pass are what the plan actually gates on, and both behave as specified.

## Issues Encountered

**Test module path.** The plan's `<verify>` commands specify test paths as `gemini::tests::<name>`, but this crate's actual module layout (`gemini/mod.rs` declaring `pub mod adapter;`, with the `#[cfg(test)] mod tests` nested inside `adapter.rs`) resolves to `gemini::adapter::tests::<name>`. Ran all verification commands with the corrected path; behavior and pass/fail outcomes are otherwise exactly as the plan specifies. No production code or test names were changed.

**RED-state mockito failure text (D-00e), verbatim:**
```
---- gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure stdout ----

thread 'gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure' panicked at /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mockito-1.7.2/src/mock.rs:633:13:

> Expected 1 request(s) to:

POST (any)?(any)

...but received 3
```
Plus the three RED-state map_error assertion failures:
```
map_error_400_invalid_argument_naming_an_invalid_api_key_maps_to_authentication_error:
  expected AuthenticationError, got: InvalidPrompt("API key not valid. Please pass a valid API key.")

map_error_401_unauthenticated_maps_to_authentication_error:
  expected AuthenticationError, got: ProcessingError("Gemini request failed (HTTP 401, status=UNAUTHENTICATED): Request had invalid authentication credentials.")

map_error_403_with_no_parseable_envelope_maps_to_authentication_error:
  expected AuthenticationError, got: ProcessingError("Gemini request failed (HTTP 403): Forbidden")
```

## Verification Command Output (D-00e)

1. `cargo test -p paladin-llm --no-default-features --features gemini` — **96 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.**
2. `cargo test -p paladin-llm --no-default-features --features kimi,qwen,grok,ollama,openai-compatible,gemini` — **197 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.**
3. `cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings` — clean, `Finished` with no warnings.
4. `cargo fmt --check -p paladin-llm` — clean, no output, exit 0.
5. `cargo test -p paladin-llm` (default features) — **57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.**
6. `cargo doc -p paladin-llm --no-deps --no-default-features --features gemini` — 2 warnings, both `rustdoc::private_intra_doc_links` (pre-existing, `17-VERIFICATION.md` Truth 6: `adapter` module doc linking to private `GeminiResponse` and `GeminiAdapter::map_error`). **Zero `missing_docs` warnings** — the two new items (`GEMINI_CREDENTIAL_MESSAGE_SIGNATURES`, `is_credential_failure_message`) are both private, so `missing_docs` (which only applies to public items) does not gate them; both are documented anyway.
7. **Snyk:** the `snyk_code_scan` MCP tool was **not available** in this executor's runtime (no `snyk` CLI on `PATH`, no Snyk MCP tool registered). The scan was **not run**. Recorded here plainly per `snyk_rules.instructions.md` rather than claimed as passed.

**Acceptance-criteria greps (all confirmed):**
- `GEMINI_CREDENTIAL_MESSAGE_SIGNATURES` outside comments: **2** (definition + single use).
- `fn is_credential_failure_message` outside comments: **1**.
- `401 | 403 =>` (bare, no guard) outside comments: **1**.
- `PERMISSION_DENIED` in library code (lines 1–1129, before `#[cfg(test)]`): **0**.
- `RESOURCE_EXHAUSTED` count: **4** — unchanged from the base commit.
- `300..=399` count: **1** — 17-10's arm survived.
- `validate_model_identifier(&request.model)?` count: **2** — 17-09's guard survived.
- `unwrap()|expect(|panic!` in library code: **0**.
- `[?&]key=` and `crate::compat`: **0** each (pre-existing D2/D6 gates, still holding).
- `git diff --stat` for `Cargo.toml`/`Cargo.lock`/`compat/`/`openai/`/`anthropic/`/`deepseek/`: **empty**.
- `cargo test ... -- gemini::adapter::tests::map_error` ran **12** tests (plan required ≥11), all pass.

## User Setup Required

None - no external service configuration required.

## Closing Note — Gap-Closure Run Complete

This is the last plan of the 17-09/17-10/17-11 gap-closure run. Three findings from `17-REVIEW.md` are now closed:

- **CR-01** (plan 17-09) — Gemini caller-supplied model-identifier path-injection guard.
- **WR-04** (plan 17-10) — Gemini credential replay on a followed redirect.
- **WR-03** (this plan, 17-11) — Gemini credential-failure classification (unrecognised auth failure retried; invalid-key `400` misread as bad prompt).

Seven findings remain deferred, all recorded in `17-09-PLAN.md` §Deferred and repeated in this plan's own `<deferred>` block: **WR-01, WR-02, WR-05, WR-06, WR-07, IN-01, IN-02** — none are touched by any commit in this plan, and no acceptance criterion here depends on them. `/gsd-verify-work` can treat CR-01/WR-03/WR-04 as closed and the remaining seven as accepted, unaddressed debt without re-reading `17-REVIEW.md`.

## Next Phase Readiness
- Phase 17 is ready for re-verification against the updated finding set (three closed, seven deferred).
- No blockers. Zero new dependencies, zero new public symbols, `compat/`, `openai/`, `anthropic/`, `deepseek/` all untouched — `make deny`/`make audit` verdicts are unaffected by this plan.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*

## Self-Check: PASSED

- FOUND: `crates/paladin-llm/src/gemini/adapter.rs`
- FOUND: `.planning/phases/17-additional-llm-provider-adapters/17-11-SUMMARY.md`
- FOUND commit: `ee37de6` (test(17-11): RED)
- FOUND commit: `a772305` (fix(17-11): GREEN)
- FOUND commit: `29fa2e2` (docs(17-11): SUMMARY)
