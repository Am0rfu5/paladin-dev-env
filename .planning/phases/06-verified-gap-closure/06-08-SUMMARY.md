---
phase: 06-verified-gap-closure
plan: 08
subsystem: battalion
tags: [rust, grove, routing, error-handling, tdd]

# Dependency graph
requires:
  - phase: 06-verified-gap-closure
    provides: "06-01's route_by_llm missing-routing_model guard (BattalionError::RoutingError) and 06-VERIFICATION.md's truth-3 gap finding"
provides:
  - "The D-02 no-fallback guarantee is reachable from GroveExecutionService::execute(), the only public entry point — not just from the crate-private route_by_llm"
  - "Single shared resolver (resolve_routing_model) called by both route_task and route_by_llm so the two checks cannot drift apart"
  - "Inverted the standing green counter-example test_grove_llm_routing to assert the D-02 error"
  - "execute()-level edge-case witnesses: blank/whitespace routing_model, a configured-and-resolvable fallback_tree that must not be consulted, and the scope negative control (absent llm_port keeps its pre-existing fallback)"
affects: ["06-09", "06-10"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Shared private resolver function called from both the dispatch layer (route_task) and the strategy layer (route_by_llm) to prevent a config-check guarantee from being orphaned by an unrelated catch-all"
    - "Pre-dispatch early return: resolve a configuration-error case before entering a `match strategy { .. }` dispatch whose Err arm has broader fallback semantics, so `?` propagates the configuration error before the fallback arm can see it"

key-files:
  created: []
  modified:
    - crates/paladin-battalion/src/grove_service.rs
    - crates/paladin-core/src/platform/container/battalion/grove.rs
    - tests/integration/battalion/grove_integration_test.rs

key-decisions:
  - "No new BattalionError variant added (per plan's explicit non-goal) — resolving routing_model before dispatch in route_task keeps the error type exactly as ADR-0013/CHANGELOG already document, adds no public surface, and is reversible rather than costly"
  - "Fix scoped strictly to the missing-routing_model case: an absent llm_port under LlmRouting keeps its pre-existing fallback behaviour, proven by a dedicated scope negative control test (test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set)"

patterns-established:
  - "When a guard's guarantee must hold at a public entry point, prove it with a test that drives the entry point (execute()), not just the crate-private function that implements the guard (route_by_llm) — this is exactly the class of gap 06-VERIFICATION.md found"

requirements-completed: [CLOSE-01]

coverage:
  - id: D1
    description: "A Grove using RoutingStrategy::LlmRouting with a configured llm_port and no routing_model returns Err(BattalionError::RoutingError(..)) naming routing_model from GroveExecutionService::execute(), with zero LLM calls made"
    requirement: "CLOSE-01"
    verification:
      - kind: integration
        ref: "tests/integration/battalion/grove_integration_test.rs#test_grove_llm_routing_errors_when_routing_model_absent_through_execute"
        status: pass
      - kind: integration
        ref: "tests/integration/battalion/grove_integration_test.rs#test_grove_llm_routing"
        status: pass
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#grove_service::tests::test_execute_errors_when_routing_model_absent"
        status: pass
    human_judgment: false
  - id: D2
    description: "Blank/whitespace-only routing_model and a configured-and-resolvable fallback_tree are both declined for this configuration error at the execute() level"
    requirement: "CLOSE-01"
    verification:
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#grove_service::tests::test_execute_errors_when_routing_model_blank"
        status: pass
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#grove_service::tests::test_execute_errors_despite_fallback_tree_when_routing_model_absent"
        status: pass
    human_judgment: false
  - id: D3
    description: "The fix is scoped to the missing-routing_model case only: an absent llm_port under LlmRouting with routing_model set still falls back successfully through execute()"
    requirement: "CLOSE-01"
    verification:
      - kind: integration
        ref: "tests/integration/battalion/grove_integration_test.rs#test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set"
        status: pass
    human_judgment: false
  - id: D4
    description: "All four pre-existing route_by_llm-level guard tests still pass unmodified"
    requirement: "CLOSE-01"
    verification:
      - kind: unit
        ref: "crates/paladin-battalion/src/grove_service.rs#grove_service::tests (test_llm_routing_errors_when_routing_model_absent, test_llm_routing_errors_when_routing_model_empty, test_llm_routing_missing_model_error_precedes_keyword_fallback, test_llm_routing_uses_configured_routing_model)"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-05
status: complete
---

# Phase 6 Plan 08: Wire D-02's no-fallback guarantee into route_task's dispatch Summary

**`route_task`'s blanket `Err` fallback arm no longer swallows the deliberate missing-`routing_model` `RoutingError`; a shared `resolve_routing_model` resolver now runs pre-dispatch, so `GroveExecutionService::execute()` — the only public entry point — returns the hard error instead of silently substituting a fallback agent.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-05T21:07:56Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Closed 06-VERIFICATION.md truth 3 (score 8/10 → the CLOSE-01 half that was "half-closed"): the D-02 hard-error/no-fallback guarantee is now reachable from `GroveExecutionService::execute()`, not just from the crate-private `route_by_llm`.
- Added a single shared resolver, `GroveExecutionService::resolve_routing_model`, backed by one `MISSING_ROUTING_MODEL_ERROR` constant, called from both `route_task`'s new pre-dispatch early return and `route_by_llm`'s in-strategy guard — the two checks structurally cannot drift apart.
- Inverted `tests/integration/battalion/grove_integration_test.rs::test_grove_llm_routing`, the standing green counter-example `06-VERIFICATION.md` cited by name, to assert the D-02 error instead of `Ok`.
- Added five new tests proving the guarantee at the `execute()` level: the primary tracer proof (with a recording mock proving zero LLM calls), blank/whitespace-only `routing_model`, a configured-and-resolvable `fallback_tree` that must still be declined, the plain `None` case, and a scope negative control proving an absent `llm_port` with a configured `routing_model` still falls back successfully.
- Amended three rustdoc blocks (`GroveConfig.routing_model`, `execute`'s `# Errors`, `route_task`'s doc comment) to describe the shipped, `execute()`-reachable control flow rather than only the internal guard.

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end "a misconfigured Grove errors through execute()" — one path only** - `41b4075` (fix, tdd)
2. **Task 2: Prove the edges at the entry point — blank values, fallback_tree, and the scope negative control** - `ab16dc3` (test, tdd)
3. **Task 3: Document the reachable guarantee and run the full workspace gate** - `d83cd36` (docs)

_Note: Task 1's commit also includes the Task 2 integration-file negative-control test — it was written in the same file edit that inverted `test_grove_llm_routing`; see Deviations below._

## Files Created/Modified

- `crates/paladin-battalion/src/grove_service.rs` - `MISSING_ROUTING_MODEL_ERROR` const, `resolve_routing_model` shared resolver, `route_task` pre-dispatch early return, `route_by_llm` guard rewritten to delegate, five new `#[cfg(test)]` tests, three amended rustdoc blocks
- `crates/paladin-core/src/platform/container/battalion/grove.rs` - `GroveConfig.routing_model` rustdoc amended to name `GroveExecutionService::execute()` as the observable entry point
- `tests/integration/battalion/grove_integration_test.rs` - `test_grove_llm_routing` inverted to assert the error; three new tests added (`test_grove_llm_routing_errors_when_routing_model_absent_through_execute`, `RecordingRoutingLlmMock`, `test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set`)

## Decisions Made

- No new `BattalionError` variant — matches the plan's explicit non-goal (a new variant would be a semver break for `BattalionError`, which is not `#[non_exhaustive]`, and would falsify ADR-0013/CHANGELOG's `RoutingError` claim). Resolving `routing_model` before dispatch keeps the error type as already documented.
- Fix scoped strictly to the missing-`routing_model` case per the plan's `<assumption_delta_decision>` (`no-change`): an absent `llm_port` under `LlmRouting` deliberately keeps its pre-existing fallback behaviour and is proven unaffected by a dedicated negative-control test.

## Deviations from Plan

**1. [No rule — sequencing only] Task 2's scope negative control test landed in the Task 1 commit**
- **Found during:** Task 1 (writing the integration test file edit)
- **Issue:** None — this is not a defect. While writing Task 1's integration-test edits (which touch the same file and the same logical region as Task 2's integration-file addition), the plan's Task 2 negative-control test (`test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set`) was authored and committed as part of the same file edit rather than deferred to a separate Task 2 edit.
- **Fix:** No fix needed; the test's content and behavior exactly match the plan's Task 2 `<behavior>` and `<action>` specification. Task 2's commit instead covers the three `grove_service.rs`-level tests the plan assigned to it.
- **Files modified:** `tests/integration/battalion/grove_integration_test.rs` (in the Task 1 commit, `41b4075`)
- **Verification:** `cargo test -p paladin-ai --test lib grove_integration_test` names and passes `test_grove_llm_routing_falls_back_when_llm_port_absent_but_routing_model_set` after both commits; acceptance-criteria greps for Task 2 (`grep -c 'fn test_execute_errors'` → 3) were verified against the `grove_service.rs`-only tests, matching the plan's literal criterion.
- **Committed in:** `41b4075` (Task 1 commit)

**2. [Rule 1 - Bug] cargo fmt reformatted the Task 1 integration-test addition before the Task 3 gate**
- **Found during:** Task 3 (`cargo fmt --check`)
- **Issue:** `RecordingRoutingLlmMock::generate`'s return-type line, as I originally wrote it in Task 1, exceeded rustfmt's preferred wrapping and was reformatted by `cargo fmt`.
- **Fix:** Ran `cargo fmt` (auto-fixes formatting only, no behavior change), then verified `cargo fmt --check` was clean.
- **Files modified:** `tests/integration/battalion/grove_integration_test.rs`
- **Verification:** `cargo fmt --check` exits 0; `cargo test -p paladin-ai --test lib grove_integration_test` still green after the reformat.
- **Committed in:** `d83cd36` (Task 3 commit)

---

**Total deviations:** 1 sequencing note (no code impact), 1 auto-fixed formatting issue (Rule 1).
**Impact on plan:** None on scope or behavior. Both are cosmetic/organizational; every acceptance criterion and the plan's `<verification>` block was independently re-checked after both commits.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- 06-VERIFICATION.md truth 3 is now closeable: the D-02 hard-error/no-fallback guarantee is proven reachable from `GroveExecutionService::execute()` by a named passing test (`test_grove_llm_routing_errors_when_routing_model_absent_through_execute`) that drives `execute()` with a configured `llm_port` and an absent `routing_model`, and the previously-green counter-example (`test_grove_llm_routing`) now asserts the error.
- Plan 06-09 and 06-10 (reconciling ADR-0013, `CHANGELOG.md`, and `.planning/REQUIREMENTS.md`'s CLOSE-01 closure text with the now-actually-shipped `execute()`-reachable behaviour — 06-VERIFICATION.md `missing:` item (c)) can now proceed: the code they need to describe is landed and green.
- Full workspace gate confirmed green at HEAD `d83cd36`: `cargo test --workspace` (0 failed), `cargo fmt --check` (clean), `cargo clippy --workspace --all-targets -- -D warnings` (0 warnings).

## Self-Check: PASSED

- Files: `crates/paladin-battalion/src/grove_service.rs`, `crates/paladin-core/src/platform/container/battalion/grove.rs`, `tests/integration/battalion/grove_integration_test.rs` — all FOUND.
- Commits: `41b4075`, `ab16dc3`, `d83cd36` — all FOUND in `git log`.

---
*Phase: 06-verified-gap-closure*
*Completed: 2026-08-05*
