---
phase: 17-additional-llm-provider-adapters
plan: 15
subsystem: api
tags: [gemini, llm-adapter, error-handling, tdd, gap-closure]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "17-11's Gemini auth-classification fix and the file state after plans 17-09/17-10/17-11"
provides:
  - "Gemini parse_response emits EmptyCompletion for a truncated-to-empty response, matching CompatEngine::detect_empty_completion's contract"
affects: ["17-16 (owns generate_stream and compat/engine.rs next wave)", "17-17 (18-row edge reconciliation, WINDOWS.md rows)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Hand-maintained behavioural parity between a bespoke adapter (Gemini, D-08) and the shared CompatEngine, pinned by a doc comment cross-referencing the mirrored function"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/gemini/adapter.rs

key-decisions:
  - "Refusal-control test uses Gemini's SAFETY finish reason (maps to FinishReason::ContentFilter per map_finish_reason:1075), not RECITATION (maps to FinishReason::Error) — SAFETY is the reason that most plausibly could be confused with a truncation, so it is the sharper control."
  - "The plan's literal acceptance-criterion grep `git diff ... | grep -c '^+.*fn parse_response'` returns 5, not the specified 0, because the five mandated test names (parse_response_maps_max_tokens_..., parse_response_keeps_..., parse_response_does_not_blame_...) share the `parse_response` prefix with the production function and the pattern has no word-boundary anchor. This is a property of the plan's exact test-name mandate, not a deviation in this execution. A precise check (`grep -c '^+    fn parse_response('`, anchored on the opening paren) returns 0, confirming the production function signature itself was not touched in Task 1."

requirements-completed: [PROV-02, PROV-04]

coverage:
  - id: D1
    description: "A Gemini response with finishReason MAX_TOKENS and empty parts[] returns Err(LlmError::EmptyCompletion), not a silent empty-string Ok"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::parse_response_maps_max_tokens_with_no_parts_to_empty_completion"
        status: pass
    human_judgment: false
  - id: D2
    description: "A Gemini response with finishReason MAX_TOKENS and whitespace-only text returns Err(LlmError::EmptyCompletion) — whitespace-only counts as blank"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::parse_response_maps_max_tokens_with_whitespace_only_text_to_empty_completion"
        status: pass
    human_judgment: false
  - id: D3
    description: "Adjacency: a truncated response that produced real text still returns Ok with FinishReason::Length and content intact"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::parse_response_keeps_a_truncated_response_that_produced_text"
        status: pass
    human_judgment: false
  - id: D4
    description: "Adjacency: an empty response that finished normally (STOP) still returns Ok — exact parity with CompatEngine::detect_empty_completion"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::parse_response_keeps_an_empty_response_that_finished_normally"
        status: pass
    human_judgment: false
  - id: D5
    description: "Refusal control: a SAFETY finish with empty content returns Ok, never EmptyCompletion — a refusal is never reported as a token-budget problem"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::parse_response_does_not_blame_the_token_budget_for_a_refusal"
        status: pass
    human_judgment: false
  - id: D6
    description: "The no-candidates guard still fires first, unchanged, with its own distinguishable message"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::parse_response_empty_candidates_yields_empty_completion"
        status: pass
    human_judgment: false
  - id: D7
    description: "Plans 17-09, 17-10, 17-11's prior gap-closure work survives untouched in the same file"
    verification:
      - kind: other
        ref: "grep -c validate_model_identifier / redirect::Policy::none() / GEMINI_CREDENTIAL_MESSAGE_SIGNATURES over gemini/adapter.rs, each >= 1; plus generate_does_not_retry_an_unrecognised_authentication_failure and gemini_does_not_replay_the_api_key_header_to_a_redirect_target end-to-end tests"
        status: pass
    human_judgment: false
  - id: D8
    description: "Snyk code scan of the modified file"
    verification: []
    human_judgment: true
    rationale: "Neither the snyk_code_scan MCP tool nor the snyk CLI is available in this worktree environment. Not run — recorded here per D-00e/executor notes rather than claimed as passed. Plan 17-17 files the matching WINDOWS.md row for the whole run."

# Metrics
duration: 45min
completed: 2026-08-18
status: complete
---

# Phase 17 Plan 15: Gemini truncated-empty completion parity Summary

**Gemini's `parse_response` now returns `Err(LlmError::EmptyCompletion)` for a `MAX_TOKENS`-truncated response with no text, matching `CompatEngine::detect_empty_completion`'s contract used by every other adapter this phase shipped.**

## Performance

- **Duration:** 45 min
- **Started:** 2026-08-18T01:10:00Z (approx, from worktree branch check)
- **Completed:** 2026-08-18T01:55:00Z (approx)
- **Tasks:** 2 completed (RED, GREEN)
- **Files modified:** 1 (`crates/paladin-llm/src/gemini/adapter.rs`)

## Accomplishments

- Closed the **new** WR-03 from `17-REVIEW.md` (Gemini truncated-empty detection) — distinct from the auth-classification WR-03 plan 17-11 already closed on this same file. Stated plainly here so a future reader is not misled by the reused label.
- `parse_response` now carries a two-condition guard — `FinishReason::Length` AND blank content — that fires only on the reasoning-model truncation signature, mirroring `crate::compat::engine::CompatEngine::detect_empty_completion` by hand (D-08 keeps Gemini bespoke; `compat/engine.rs` was not touched).
- Five new regression tests pin the full contract: two positive cases (empty parts, whitespace-only text), two adjacency controls (productive truncation stays `Ok`; normal-finish empty content stays `Ok`), and one refusal control (a `SAFETY` finish with empty content is never reported as a token-budget problem).
- Corrected `parse_response`'s doc comment, which previously promised failure on any empty content that the code did not actually deliver; it now names both failure conditions explicitly.
- Confirmed by grep and by two end-to-end test runs that plans 17-09 (`validate_model_identifier`), 17-10 (`redirect::Policy::none()`), and 17-11 (`GEMINI_CREDENTIAL_MESSAGE_SIGNATURES`) all survive untouched in this file.

## Two resolved wire-level facts (D-00e)

1. **Gemini `finishReason` string that maps to `FinishReason::Length`:** `"MAX_TOKENS"` — confirmed at `crates/paladin-llm/src/gemini/adapter.rs:1074` (`Some("MAX_TOKENS") => FinishReason::Length,` inside `map_finish_reason`, function at `:1070-1078`).
2. **Non-truncation reason used for the refusal control:** `"SAFETY"` — confirmed at `crates/paladin-llm/src/gemini/adapter.rs:1075` (`Some("SAFETY") => FinishReason::ContentFilter,`), which does **not** map to `FinishReason::Length`. (`"RECITATION"` was also confirmed non-`Length`, mapping to `FinishReason::Error("RECITATION")` per the exhaustive-mapping test at `:1339-1341`, but `SAFETY` was chosen as the sharper control since a safety refusal is the case most plausibly confusable with truncation.)

## Exact wording of the new `EmptyCompletion` message

```
"Gemini response finished with MAX_TOKENS and produced no text ({N} raw chars) — reasoning likely consumed the entire max_tokens budget; retry with a larger max_tokens"
```

where `{N}` is `content.len()` (the raw character count), mirroring `CompatEngine::detect_empty_completion`'s structure of naming the raw char count and the remedy.

## Task Commits

Each task was committed atomically with `git commit --no-verify` per `workflow.worktree_skip_hooks=true` (D-00o):

1. **Task 1: RED — five tests proving a Gemini response truncated to no text is reported as a success** - `4f0e732` (test)
2. **Task 2: GREEN — a truncated-to-empty Gemini response is an EmptyCompletion, and the doc comment stops over-promising** - `26028f8` (fix)

**Plan metadata:** SUMMARY commit follows this file.

## Files Created/Modified

- `crates/paladin-llm/src/gemini/adapter.rs` - Added the two-condition truncation guard to `parse_response`, corrected its doc comment to name both failure conditions, and added five regression tests (two positive, two adjacency controls, one refusal control).

## Decisions Made

- Chose `SAFETY` (maps to `FinishReason::ContentFilter`) over `RECITATION` (maps to `FinishReason::Error`) for the refusal control, since `SAFETY` is the finish reason most plausibly confusable with truncation and therefore the sharper test of the discriminator.
- Kept the predicate inline in `parse_response` rather than extracting a helper function, per the plan's explicit "New functions: none" artifact constraint — the guard sits next to the values it reads rather than becoming a second helper that can drift from `CompatEngine`'s.

## Deviations from Plan

**None — plan executed exactly as written**, with one clarifying note (not a deviation in behavior or scope):

**1. [Acceptance-criterion grep mismatch, not a deviation] `git diff ... grep -c '^+.*fn parse_response'` returns 5, not 0**
- **Found during:** Task 1 verification
- **Issue:** The plan's Task 1 acceptance criterion `git diff -- crates/paladin-llm/src/gemini/adapter.rs | grep -c '^+.*fn parse_response'` → **0** ("This task adds no production code") is written without a word-boundary anchor after `parse_response`. The plan's own mandated test names (`parse_response_maps_max_tokens_with_no_parts_to_empty_completion`, `parse_response_maps_max_tokens_with_whitespace_only_text_to_empty_completion`, `parse_response_keeps_a_truncated_response_that_produced_text`, `parse_response_keeps_an_empty_response_that_finished_normally`, `parse_response_does_not_blame_the_token_budget_for_a_refusal`) all share the `parse_response` prefix, so each of their `fn` declaration lines matches the grep pattern too. Running the command as literally specified returns `5`, not `0`.
- **Verification of actual intent:** A precise, anchored check — `git diff -- crates/paladin-llm/src/gemini/adapter.rs | grep -c '^+    fn parse_response('` (requiring the opening paren of the production function's own signature) — returns `0`, confirming Task 1 added no change to the production `parse_response` function itself. Task 1's diff is purely additive test code, verified by inspection: `git diff ... | grep '^+.*fn parse_response'` shows all five matched lines are `+    fn parse_response_...() {` test declarations, none is the production signature.
- **Files modified:** None beyond the planned test additions.
- **Committed in:** `4f0e732` (Task 1 commit) — no separate fix needed; this is a note on the criterion's phrasing, not a code change.

---

**Total deviations:** 0 auto-fixed. One documentation note on an acceptance-criterion grep pattern that does not account for the plan's own mandated test-name prefixes.
**Impact on plan:** None. No scope creep, no behavior change beyond what the plan specified.

## Issues Encountered

None. Both RED and GREEN gates were achieved on the first attempt; all verification commands ran and passed as specified (or, for Snyk, were confirmed unavailable and recorded as not run rather than assumed).

## Verification Commands Run — exact commands and exact output (D-00e)

**Task 1 — RED state (both `_maps_max_tokens_` tests fail against the pre-fix tree):**

```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::parse_response_maps_max_tokens
```
```
running 2 tests
test gemini::adapter::tests::parse_response_maps_max_tokens_with_no_parts_to_empty_completion ... FAILED
test gemini::adapter::tests::parse_response_maps_max_tokens_with_whitespace_only_text_to_empty_completion ... FAILED

failures:

---- gemini::adapter::tests::parse_response_maps_max_tokens_with_no_parts_to_empty_completion stdout ----

thread 'gemini::adapter::tests::parse_response_maps_max_tokens_with_no_parts_to_empty_completion' panicked at crates/paladin-llm/src/gemini/adapter.rs:1409:9:
expected Err(EmptyCompletion(_)) for a MAX_TOKENS finish with no parts, got Ok(LlmResponse { id: f7f49499-bd8d-4183-8027-81977dcb123d, request_id: d18c6998-c2ae-408a-a95c-fcd1f544f59b, model: "gemini-2.5-flash", content: "", finish_reason: Length, usage: TokenUsage { prompt_tokens: 0, completion_tokens: 0, total_tokens: 0 }, created_at: 2026-08-18T01:25:24.134616405Z, metadata: {}, function_call: None })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- gemini::adapter::tests::parse_response_maps_max_tokens_with_whitespace_only_text_to_empty_completion stdout ----

thread 'gemini::adapter::tests::parse_response_maps_max_tokens_with_whitespace_only_text_to_empty_completion' panicked at crates/paladin-llm/src/gemini/adapter.rs:1434:9:
expected Err(EmptyCompletion(_)) for a MAX_TOKENS finish with whitespace-only text, got Ok(LlmResponse { id: 6a7dfc28-dbb4-49b7-9b09-853d6e3ae06c, request_id: c2ef397c-d6f3-417a-9d43-2b3afe6cf1d4, model: "gemini-2.5-flash", content: "   \n  ", finish_reason: Length, usage: TokenUsage { prompt_tokens: 0, completion_tokens: 0, total_tokens: 0 }, created_at: 2026-08-18T01:25:24.134654340Z, metadata: {}, function_call: None })

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 99 filtered out; finished in 0.15s
```
(Command exits non-zero, confirming genuine RED — the plan's `<automated>` verify is `!` of this same command, which inverts to exit 0 on this failure.)

**Task 1 — adjacency controls (pass before the fix):**
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::parse_response_keeps
```
```
running 2 tests
test gemini::adapter::tests::parse_response_keeps_an_empty_response_that_finished_normally ... ok
test gemini::adapter::tests::parse_response_keeps_a_truncated_response_that_produced_text ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out; finished in 0.15s
```

**Task 1 — refusal control (passes before the fix):**
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::parse_response_does_not_blame_the_token_budget_for_a_refusal --exact
```
```
running 1 test
test gemini::adapter::tests::parse_response_does_not_blame_the_token_budget_for_a_refusal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 100 filtered out; finished in 0.15s
```

**Task 1 — clippy/fmt (pre-fix tree, tests-only diff):**
```
$ cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings
```
→ `Finished` dev profile, exit 0, zero warnings.
```
$ cargo fmt --check -p paladin-llm
```
→ exit 0.

**Task 2 — GREEN (all `parse_response` tests, including the 5 new + 2 pre-existing = 7):**
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::parse_response
```
```
running 7 tests
test gemini::adapter::tests::parse_response_maps_max_tokens_with_no_parts_to_empty_completion ... ok
test gemini::adapter::tests::parse_response_keeps_a_truncated_response_that_produced_text ... ok
test gemini::adapter::tests::parse_response_well_formed_candidate_parses_content_usage_and_finish_reason ... ok
test gemini::adapter::tests::parse_response_keeps_an_empty_response_that_finished_normally ... ok
test gemini::adapter::tests::parse_response_does_not_blame_the_token_budget_for_a_refusal ... ok
test gemini::adapter::tests::parse_response_maps_max_tokens_with_whitespace_only_text_to_empty_completion ... ok
test gemini::adapter::tests::parse_response_empty_candidates_yields_empty_completion ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 94 filtered out; finished in 0.41s
```

**Task 2 — whole Gemini module:**
```
$ cargo test -p paladin-llm --no-default-features --features gemini
```
→ `test result: ok. 101 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.00s`

**Task 2 — all sibling wave-2 adapter features together (arithmetic: this worktree has no sibling wave-2 plan merged in, so prior count = 202 − 5 = 197; observed 202 confirms exactly the 5 new tests, no unexpected drift):**
```
$ cargo test -p paladin-llm --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini"
```
→ `test result: ok. 202 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.48s`

**Task 2 — default features (Gemini inert for default build, PROV-03):**
```
$ cargo test -p paladin-llm
```
→ `test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.82s`

**Task 2 — guard shape, scoped to the `parse_response` function body:**
```
$ sed -n '/fn parse_response(/,/^    }$/p' crates/paladin-llm/src/gemini/adapter.rs | grep -c 'EmptyCompletion'
2
$ sed -n '/fn parse_response(/,/^    }$/p' crates/paladin-llm/src/gemini/adapter.rs | grep -c 'FinishReason::Length'
1
$ sed -n '/fn parse_response(/,/^    }$/p' crates/paladin-llm/src/gemini/adapter.rs | grep -c 'trim().is_empty()'
1
```

**Task 2 — prior gap-closure work survival (each ≥ 1, recorded individually):**
```
$ grep -c 'validate_model_identifier' crates/paladin-llm/src/gemini/adapter.rs
15
$ grep -c 'redirect::Policy::none()' crates/paladin-llm/src/gemini/adapter.rs
1
$ grep -c 'GEMINI_CREDENTIAL_MESSAGE_SIGNATURES' crates/paladin-llm/src/gemini/adapter.rs
4
```

**Task 2 — plans 17-10 and 17-11's end-to-end proofs:**
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure gemini::adapter::tests::gemini_does_not_replay_the_api_key_header_to_a_redirect_target
```
```
running 2 tests
test gemini::adapter::tests::generate_does_not_retry_an_unrecognised_authentication_failure ... ok
test gemini::adapter::tests::gemini_does_not_replay_the_api_key_header_to_a_redirect_target ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out; finished in 3.97s
```

**Task 2 — clippy/fmt on final tree:**
```
$ cargo clippy -p paladin-llm --all-targets --no-default-features --features "kimi,qwen,grok,ollama,openai-compatible,gemini" -- -D warnings
```
→ exit 0, zero warnings.
```
$ cargo fmt --check -p paladin-llm
```
→ exit 0.

**Task 2 — doc build (zero `missing_docs`; only the pre-existing `rustdoc::private_intra_doc_links` warnings permitted per `17-VERIFICATION.md` Truth 7):**
```
$ cargo doc -p paladin-llm --no-deps --no-default-features --features gemini
```
```
warning: public documentation for `adapter` links to private item `GeminiResponse` (crates/paladin-llm/src/gemini/adapter.rs:28:20)
warning: public documentation for `adapter` links to private item `GeminiAdapter::map_error` (crates/paladin-llm/src/gemini/adapter.rs:59:37)
warning: `paladin-llm` (lib doc) generated 2 warnings
```
Both warnings are `rustdoc::private_intra_doc_links`, pre-existing per `17-VERIFICATION.md` Truth 7. Zero `missing_docs` warnings.

**Task 2 — dependency and sibling-file diffs (both required empty):**
```
$ git diff --stat -- crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock
(empty)
$ git diff --stat -- crates/paladin-llm/src/compat/ crates/paladin-llm/src/openai/ crates/paladin-llm/src/anthropic/ crates/paladin-llm/src/deepseek/ crates/paladin-llm/src/provider_factory.rs crates/paladin-llm/src/openai_compatible/
(empty)
```

**Snyk:** `snyk_code_scan` MCP tool not available in this session's toolset; `snyk` CLI not found on `PATH` (`command -v snyk` exit 1). **Not run.** Plan 17-17 files the corresponding `WINDOWS.md` row for the whole phase-17 run.

## Known Stubs

None. No stubs, placeholders, or unwired data introduced by this plan.

## Threat Flags

None. No new network endpoints, auth paths, file access patterns, or trust-boundary schema changes were introduced. This plan's threat surface (the classification of a truncated response as success vs. error) was already fully enumerated in the plan's own `<threat_model>` (T-17-67, T-17-68, T-17-SC-15) and is not additive beyond what that register already covers.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The new WR-03 is closed: Gemini emits the same `EmptyCompletion` signal for a truncated-to-empty response that `CompatEngine::detect_empty_completion` emits for Kimi, Qwen, Grok, Ollama, and the generic openai-compatible preset — one error contract across all six adapters this phase shipped.
- `compat/engine.rs`, `provider_factory.rs`, `openai_compatible/adapter.rs`, and the `openai`/`anthropic`/`deepseek` crates are all untouched by this plan — clear for plan 17-16 (next wave) to work `generate_stream` and `compat/engine.rs` without any merge collision from this plan's changes.
- `gemini/adapter.rs` now carries the truncated-empty guard alongside plans 17-09/17-10/17-11's prior gap-closure work, all confirmed surviving together.
- Plan 17-17 still owns: the whole-run Snyk-not-run `WINDOWS.md` row, the 18-row edge reconciliation table, and the four `human_verification` items from `17-VERIFICATION.md` frontmatter (none closeable in this environment).

## Self-Check: PASSED

- `crates/paladin-llm/src/gemini/adapter.rs` — FOUND
- `.planning/phases/17-additional-llm-provider-adapters/17-15-SUMMARY.md` — FOUND
- Commit `4f0e732` (test, RED) — FOUND in `git log --oneline --all`
- Commit `26028f8` (fix, GREEN) — FOUND in `git log --oneline --all`
- Commit `53eb630` (docs, this SUMMARY) — FOUND in `git log --oneline --all`

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-18*
