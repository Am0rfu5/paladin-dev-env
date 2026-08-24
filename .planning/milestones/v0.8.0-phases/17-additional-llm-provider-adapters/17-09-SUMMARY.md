---
phase: 17-additional-llm-provider-adapters
plan: 09
subsystem: llm-adapters
tags: [rust, security, gemini, url-injection, allow-list, tdd]

# Dependency graph
requires:
  - phase: 17-additional-llm-provider-adapters
    provides: "GeminiAdapter (plan 17-05) — the bespoke, URL-path-based Gemini LlmPort implementation this plan patches"
provides:
  - "validate_model_identifier — a private ASCII character allow-list guard in gemini/adapter.rs that rejects any request.model value containing a character outside [A-Za-z0-9._-] before generate() or generate_stream() build a request URL"
  - "12 new regression/unit tests proving CR-01 is closed: 6 behavioral rejection tests + 1 positive control (mock-transport level) + 5 pure-logic guard tests"
affects: [17-10, 17-11]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "URL-path-interpolated fields get a character allow-list guard as the first statement of every LlmPort method that builds a request URL from them, reusing crate::redaction::bounded_excerpt for the rejection message rather than a second truncation implementation"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/gemini/adapter.rs

key-decisions:
  - "Guard rejects out-of-allow-list model identifiers rather than percent-encoding them — an operator must never receive a completion from a model they did not name (CR-01's prohibition)"
  - "Guard is a character allow-list, not a membership check against available_models() — avoids forcing a network fetch into generate()'s hot path and avoids rejecting models the provider ships after this release (D-13)"
  - "No new dependency added — percent-encoding was considered and deliberately rejected; std-only implementation keeps make deny / make audit verdicts provably unchanged"

patterns-established:
  - "Trust-boundary module-doc section (## Trust boundary: ...) mirroring openai_compatible/adapter.rs's shape for a caller-supplied field that crosses into a URL"

requirements-completed: [PROV-02, PROV-04]

coverage:
  - id: D1
    description: "validate_model_identifier rejects a model value containing a path separator, query delimiter, colon operation suffix, fragment delimiter, empty string, or non-ASCII homoglyph — on both generate() and generate_stream() — before any request reaches the mock transport"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_rejects_a_model_containing_a_path_separator_without_issuing_a_request"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_stream_rejects_a_model_containing_a_query_delimiter_without_issuing_a_request"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_rejects_a_model_containing_a_colon_operation_suffix"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_rejects_a_model_containing_a_fragment_delimiter"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_rejects_an_empty_model"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_rejects_a_model_containing_a_non_ascii_homoglyph"
        status: pass
    human_judgment: false
  - id: D2
    description: "The guard does not over-reject: a model value using only allowed characters (ASCII alphanumerics, '.', '_', '-') still reaches the mock transport exactly once and returns Ok"
    requirement: "PROV-02"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::generate_accepts_a_model_whose_characters_are_all_in_the_allowed_set"
        status: pass
    human_judgment: false
  - id: D3
    description: "The guard's own logic is correct at the boundary: shipped defaults/fallbacks always pass, every URL metacharacter is rejected, an allowed-character-only value with no alphanumeric is rejected, a 2,000-character multi-byte value is rejected without panicking and with a bounded message, and a value exercising every allowed character class passes"
    requirement: "PROV-04"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::validate_model_identifier_accepts_the_default_and_every_fallback_model"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::validate_model_identifier_rejects_each_url_metacharacter"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::validate_model_identifier_rejects_a_value_with_no_alphanumeric_character"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::validate_model_identifier_rejects_a_long_multibyte_value_without_panicking"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/gemini/adapter.rs#gemini::adapter::tests::validate_model_identifier_accepts_every_character_of_the_allowed_set"
        status: pass
    human_judgment: false
  - id: D4
    description: "Snyk static-analysis scan of the modified file for newly introduced security defects"
    verification: []
    human_judgment: true
    rationale: "Snyk MCP tool (snyk_code_scan) and the Snyk CLI are both unavailable in this executor's runtime (no network egress in this worktree). The scan could not be run and is not recorded as passed — see Deviations."

# Metrics
duration: ~20min
completed: 2026-08-17
status: complete
---

# Phase 17 Plan 09: Close CR-01 — Gemini model-identifier URL injection Summary

**Allow-list guard (`validate_model_identifier`) in `gemini/adapter.rs` rejects any `request.model` value outside `[A-Za-z0-9._-]` before `generate()`/`generate_stream()` build a request URL, closing the one Critical finding blocking `17-VERIFICATION.md`.**

## Performance

- **Duration:** ~20 min
- **Completed:** 2026-08-17T19:32:36Z
- **Tasks:** 2 (RED, GREEN)
- **Files modified:** 1 (`crates/paladin-llm/src/gemini/adapter.rs`)

## Accomplishments
- Closed CR-01: `GeminiAdapter::generate()` and `generate_stream()` no longer splice a caller-supplied `model` value unescaped into the request URL path — `validate_model_identifier` runs as the first statement of both methods and rejects any value containing a character outside the ASCII allow-list, before any `format!` builds the URL and before the live `x-goog-api-key`-bearing request is dispatched.
- Twelve new tests: 6 behavioral rejection tests (path separator, query delimiter, colon operation suffix, fragment delimiter, empty string, non-ASCII homoglyph — each asserting the mock transport records **zero** requests), 1 positive control (an allowed-character-only model still reaches the mock exactly once), and 5 pure-logic tests on the guard itself (shipped defaults/fallbacks, every URL metacharacter, no-alphanumeric values, a 2,000-character multi-byte value without panicking, every allowed character class).
- Zero new dependency, zero new public symbol, zero change to `LlmPort`, and `openai/`, `anthropic/`, `deepseek/`, `compat/` are byte-unchanged.

## Task Commits

Each task was committed atomically:

1. **Task 1: RED — regression tests proving an injected model value reaches the wire today** - `3b0845d` (test)
2. **Task 2: GREEN — allow-list guard on the model identifier, called before any URL is built** - `8087eab` (fix)

_TDD plan: RED (`test(17-09):`) then GREEN (`fix(17-09):`), no REFACTOR commit needed — the implementation passed clippy/fmt clean on first pass._

## Files Created/Modified
- `crates/paladin-llm/src/gemini/adapter.rs` - Added `validate_model_identifier` (private, module-scope free function), wired into `generate()`/`generate_stream()` as their first statement, added a `## Trust boundary` module-doc section, and 12 new tests (7 behavioral over the existing `#[cfg(test)] mod tests` mock-transport harness, 5 pure-logic over the guard).

## Verification Command Output (D-00e)

Per the plan's `<output>` instruction, the exact command output, not a paraphrase:

**RED phase (Task 1), before the guard existed:**
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_rejects gemini::adapter::tests::generate_stream_rejects

running 6 tests
test gemini::adapter::tests::generate_rejects_a_model_containing_a_colon_operation_suffix ... FAILED
test gemini::adapter::tests::generate_rejects_a_model_containing_a_fragment_delimiter ... FAILED
test gemini::adapter::tests::generate_stream_rejects_a_model_containing_a_query_delimiter_without_issuing_a_request ... FAILED
test gemini::adapter::tests::generate_rejects_a_model_containing_a_path_separator_without_issuing_a_request ... FAILED
test gemini::adapter::tests::generate_rejects_an_empty_model ... FAILED
test gemini::adapter::tests::generate_rejects_a_model_containing_a_non_ascii_homoglyph ... FAILED

test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 78 filtered out; finished in 0.34s
```
Representative failure (the assertion that fired — proving a genuine behavioral failure, not a panic or compile error):
```
thread 'gemini::adapter::tests::generate_rejects_a_model_containing_a_path_separator_without_issuing_a_request'
panicked at crates/paladin-llm/src/gemini/adapter.rs:1710:9:
expected LlmError::InvalidPrompt, got Some(EmptyCompletion("Gemini response contained no candidates"))
```
This proves the hostile model value reached the mock transport (which returned `{}`, parsed as zero candidates) rather than being rejected before the request was built.

Positive control passed already at this point:
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_accepts_a_model_whose_characters_are_all_in_the_allowed_set --exact
test gemini::adapter::tests::generate_accepts_a_model_whose_characters_are_all_in_the_allowed_set ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 83 filtered out; finished in 0.13s
```

**GREEN phase (Task 2), after the guard was wired in:**
```
$ cargo test -p paladin-llm --no-default-features --features gemini
running 89 tests
... (all 89 listed as ok) ...
test result: ok. 89 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.57s
```
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::generate_rejects gemini::adapter::tests::generate_stream_rejects
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 83 filtered out; finished in 0.34s
```
```
$ cargo test -p paladin-llm --no-default-features --features gemini -- gemini::adapter::tests::validate_model_identifier
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 84 filtered out; finished in 0.00s
```
```
$ cargo clippy -p paladin-llm --no-default-features --features gemini -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 30.50s     (no warnings emitted, exit 0)
```
```
$ cargo fmt --check -p paladin-llm
(no output, exit 0)
```
```
$ cargo test -p paladin-llm       # default features (openai, mock) — PROV-03
test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```
```
$ git diff --stat -- crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock
(empty — no dependency added)
```

**Snyk scan:** not run. The `snyk_code_scan` MCP tool and the `snyk` CLI are both unavailable in this executor's runtime (this worktree has no network egress and no Snyk MCP server connected). Recorded here as not-run per the plan's explicit instruction — never recorded as passed.

## Decisions Made
- **Reject, never rewrite.** The guard returns `LlmError::InvalidPrompt` for any out-of-allow-list value rather than percent-encoding it. Every character in the allow-list (`[A-Za-z0-9._-]`) is already URL-unreserved, so encoding a valid value is a no-op, and encoding an invalid one would silently substitute a different model than the caller named — the plan's stated prohibition.
- **Character allow-list, not a live-catalog membership check.** `validate_model_identifier` does not call `available_models()`. Gating on the memoized model list would force a network fetch into `generate()`'s hot path and would reject any model the provider ships after this release, which is exactly the failure D-13 was written to prevent.
- **No new dependency.** `percent-encoding` was considered and explicitly not added; the guard uses only `std` plus the crate's existing `crate::redaction::bounded_excerpt`, keeping `git diff --stat -- crates/paladin-llm/Cargo.toml Cargo.toml Cargo.lock` empty and the `make deny`/`make audit` posture provably unchanged.

## Deviations from Plan

### Auto-fixed Issues

None — no bugs, missing critical functionality, or blocking issues were found beyond what the plan itself specified fixing (CR-01 is the target defect, not a deviation).

### Acceptance-criteria arithmetic that could not be satisfied as literally stated

The plan specifies two source-grep counts that are internally inconsistent with the plan's own mandated test names, discovered while verifying Task 1/Task 2 acceptance criteria. Documented per D-00e rather than silently worked around:

**1. `assert_async` count.** Task 1's acceptance criteria state: `grep -v '^\s*//' ... | grep -c 'assert_async'` "returns at least `12` — six new zero-request assertions on top of the file's existing ones." The file held **4** pre-existing `assert_async` calls (not enough for "4 + 6 = 12"; verified via `git show HEAD:crates/paladin-llm/src/gemini/adapter.rs | grep -c assert_async` before this plan's first commit). This plan added exactly 7 new `mock.assert_async().await;` calls (the 6 rejection tests plus the positive control, per the plan's own `<action>` step 6 and the positive-control instruction) — the maximum consistent with the plan's own enumerated seven test names — giving a post-plan total of **11**, one short of the stated threshold of 12. No extra, unrelated assertion was added purely to hit the number; doing so would have been gaming the metric rather than fixing anything real. All behavioral requirements in `<behavior>` and `<action>` are met.

**2. `fn validate_model_identifier` count.** Task 2's acceptance criteria state this grep "returns `1`". The plan's own mandated test names (`validate_model_identifier_accepts_the_default_and_every_fallback_model`, etc. — five of them) all begin with the substring `validate_model_identifier`, so `fn validate_model_identifier` matches each test's `fn` declaration line too, along with the one real guard definition, yielding **6** rather than 1. The real function is defined exactly once (`grep -c 'fn validate_model_identifier(model: &str)'` returns 1); the grep pattern in the acceptance criterion does not anchor on the parameter list, so it cannot distinguish the guard from tests whose names it also mandated.

Both are read as plan-authoring arithmetic/pattern imprecision, not implementation defects — every literally-instructed test, behavior, and code shape from `<action>`/`<behavior>` is present and passing.

---

**Total deviations:** 0 auto-fixed; 2 documented acceptance-criterion inconsistencies (not code issues).
**Impact on plan:** None on functionality or security posture — CR-01 is closed, all `<behavior>`/`<success_criteria>` requirements are met, and both flagged criteria are grep-pattern precision issues in the plan text itself.

## Issues Encountered
- The plan's `read_first` module-path hint (`gemini::tests::...`) does not match the crate's actual module structure — the test module lives at `gemini::adapter::tests::...` (`gemini/mod.rs` declares `pub mod adapter;`, and the `#[cfg(test)] mod tests` block is nested inside `adapter.rs`). All `cargo test` filter invocations in this SUMMARY and in both commit messages use the corrected `gemini::adapter::tests::` path; the plan's verification commands were run with this correction applied and behave identically otherwise.
- The plan's line-number anchors (e.g. `:161-175`, `:908`) drifted slightly by the time of execution — the plan itself flags these as "offsets, not anchors" read on 2026-08-17. `#[cfg(test)] mod tests` was found to start at line 908 in the pre-plan file, matching the plan's stated offset; no discrepancy required correction beyond the module-path issue above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- CR-01 is closed; `17-VERIFICATION.md`'s single failed truth is now satisfied by code with a named test per missing item.
- Plans 17-10 and 17-11 (WR-04, WR-03) can proceed against the same file — this plan's Task 2 commit is the tracer proving the file's mock-transport test harness and `bounded_excerpt`/`LlmError::InvalidPrompt` conventions work end-to-end before those plans expand into it.
- Human verification debt is unchanged and explicitly out of scope here (per `<deferred>`): coverage-floor measurement, Ollama live-server behavior, and vendor base-URL/model-ID smoke test remain `WINDOWS.md` ids 12/13. The Snyk scan not-run status (D4 above) is new debt this plan surfaces — worth a follow-up scan whenever network egress / Snyk MCP is available.

---
*Phase: 17-additional-llm-provider-adapters*
*Completed: 2026-08-17*
