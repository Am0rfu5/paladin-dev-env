# ADR-0012: Live-API-test missing-key behaviour

## Status

Accepted

**Date:** 2026-08-04

## Context

Epic 23 FR-23.4.4 and Epic 24 US-24.7 both require the live-API test suite to skip gracefully
with a clear message when API keys are missing. The post-Epic-24 cleanup deliberately reversed
this: `require_api_key()` was changed from returning a `Result` that let a test skip, to panicking
outright, on the stated rationale that tests previously "printed SKIPPED and returned early,
counting as PASS" and should instead "properly FAIL when keys are missing." Both positions are
defensible, and the reversal was a conscious choice, not an oversight — so precedence between the
two PRDs cannot settle which one wins.

Neither PRD's author traced one fact that changes the question: the suite is **double-gated**.
`tests/integration/mod.rs:34-35` puts the entire `llm_live_api_tests` module behind
`#[cfg(feature = "live-api-tests")]`, and every one of its 13 tests additionally carries
`#[ignore]`. Re-deriving both figures directly for this task: `grep -c '#\[ignore\]'
tests/integration/llm_live_api_tests.rs` returns `13`, and `grep -n 'cfg(feature =
"live-api-tests")' tests/integration/mod.rs` confirms the gate at line 34. A default `cargo test
--workspace` run — no feature flag, no `--ignored` — never compiles this module at all, because
`tests/lib.rs:61`'s `pub mod integration;` is the only thing that pulls `tests/integration/mod.rs`
into a test target, and that module's `#[cfg]` excludes `llm_live_api_tests` from the build
without the feature. `Cargo.toml:265` declares `live-api-tests = []`, an empty, opt-in-only
feature.

## Decision

The shipped panic stands. `require_api_key` is not changed to skip.

The justification: the graceful skip both PRDs require is supplied by the double gate, not by the
helper itself. A default `cargo test --workspace` run with no API keys set already skips the
entire suite cleanly — it never reaches `require_api_key` because the module does not even
compile in that configuration. The panic only fires when a developer has explicitly opted into
both `--features live-api-tests` and `--ignored`. At that point, a silent skip would be a false
pass: the developer asked to run these specific tests, and a key that is missing or empty means
the test cannot exercise anything, which is exactly the failure the post-Epic-24 cleanup reversed
the old skip-based behaviour to catch. Recording the panic as standing preserves both PRDs' intent
— graceful skip in the default, ungated path — without overruling the post-Epic-24 position that a
deliberately-opted-into run with a bad key should fail loudly rather than silently pass.

The missing-key semantics, read from the shipped body rather than assumed: `require_api_key`
matches on `env::var(env_var)`. `Ok(key) if !key.is_empty()` returns the key — the only
non-panicking arm. `Ok(_)` (the variable is present but its value is the empty string) panics with
an "API key is empty" message. `Err(_)` (the variable is absent entirely) panics with an "API key
not found" message. The helper therefore treats exactly two conditions as missing — the
environment variable being absent, and being present with an empty-string value — using
`str::is_empty()`, a byte-length check with no trimming. A whitespace-only value (e.g. `" "`) is
non-empty by that check and is treated as present: it takes the `Ok(key) if !key.is_empty()` arm
and is returned as-is, without panicking and without any validation that the value is a
plausible-looking key. This ADR records that behaviour; it does not propose changing it.

The concurrency consequence: because the double gate keeps `llm_live_api_tests` out of a default
`cargo test --workspace` run entirely, the panic inside `require_api_key` cannot abort or corrupt
an unrelated test running concurrently in that default parallel run — the panicking code is not
present in the compiled test binary unless the feature is explicitly enabled.

The one real defect: the doc comment on `require_api_key`
(`tests/integration/llm_live_api_tests.rs:61-64`) opens with *"Skip test if API key is not present
or empty, otherwise return the key"* at line 61, while both matched arms panic rather than skip.
Note this ADR's own line-63 re-derivation: CONTEXT.md's D-18 and this phase's own plan cite the
doc comment at `tests/integration/llm_live_api_tests.rs:63`; direct re-reading for this task shows
the specific sentence that lies is the summary line at :61, with :63-64 continuing the same
four-line doc block with an accurate description of the panic behaviour. The panic messages
themselves are already correct and helpful — they tell the reader *"To skip this test, don't run
with --ignored flag"*. The doc comment's opening line is the only thing that misinforms about this
harness.

## Considered Options

- Changing `require_api_key` to skip, per Epic 23 FR-23.4.4 / Epic 24 US-24.7 literally — rejected.
  It would turn an opted-in live-API run (feature enabled, `--ignored` passed) into a false pass
  when a key is missing or empty, which is the exact failure mode the post-Epic-24 cleanup
  introduced the panic to prevent.
- Keeping the panic and leaving the doc comment as-is — rejected. The comment actively misinforms
  a reader about what the function does; "the panic messages already tell you how to skip" does
  not excuse a doc comment that says the opposite of the function's actual behaviour.
- Recording no answer and leaving both PRD positions open — rejected. VERIFY-06 requires exactly
  one recorded answer, and the shipped harness already embodies one; declining to record it would
  leave Phase 6 without a scoped instruction.

## Code Locations

- `tests/integration/llm_live_api_tests.rs:61-64` — the four-line doc comment on
  `require_api_key`; line 61 carries the sentence that lies ("Skip test if API key is not present
  or empty"), and line 63 (the line this phase's plan and CONTEXT.md D-18 cite) continues the same
  comment block
- `tests/integration/llm_live_api_tests.rs:71-78` — the `Ok(_)` arm, the empty-string-value case,
  which panics with an "API key is empty" message
- `tests/integration/llm_live_api_tests.rs:79-87` — the `Err(_)` arm, the absent-variable case,
  which panics with an "API key not found" message
- `tests/integration/mod.rs:34-35` — `#[cfg(feature = "live-api-tests")] pub mod
  llm_live_api_tests;`, the feature gate that keeps the module out of a default build
- `tests/lib.rs:61` — `pub mod integration;`, the sole autodiscovered target that compiles
  `tests/integration/mod.rs` (and therefore the gate above) into any test binary at all
- `Cargo.toml:265` — `live-api-tests = []`, the empty, opt-in-only feature the gate depends on
- The measured `#[ignore]` count on `tests/integration/llm_live_api_tests.rs` is **13**, matching
  `grep -c '#\[ignore\]' tests/integration/llm_live_api_tests.rs` re-run for this task

## Code Conformance

must change

**CLOSE-03 in Phase 6** is the requirement that executes the consequence of this ADR. The change
is documentation only: correct the doc comment's opening line to describe the shipped panic
behaviour, and state the double gate (`tests/integration/mod.rs:34-35` plus the 13 `#[ignore]`
attributes) as the mechanism that actually supplies the graceful skip, in the module header. No
behavioural change. Phase 5 does not make this edit — nothing in this phase touches
`tests/integration/llm_live_api_tests.rs` or any other `.rs` file.

## Downstream Consumers

- Phase 6 CLOSE-03 — corrects the doc comment and documents the double gate as the skip mechanism
- Phase 5 ledger plan 05-07 — cites this ADR on the `REQ-provider-live-api-tests` and
  `REQ-cli-tiered-environment-testing` rows for the contested skip semantics
- Any developer running the live-API suite with `--features live-api-tests -- --ignored` and a
  missing or empty key
