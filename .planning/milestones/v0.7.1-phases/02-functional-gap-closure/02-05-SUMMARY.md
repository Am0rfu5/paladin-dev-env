---
phase: 02-functional-gap-closure
plan: 05
subsystem: testing
tags: [rust, integration-test, formation, herald, tokio-test]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "plan 02-04's FormationExecutionService aggregation (per_paladin_times, per_paladin_tokens, total_tokens, node_errors) and Herald battalion-result formatting"
provides:
  - "tests/integration/battalion_herald_end_to_end_test.rs — the Formation-driven, three-Herald end-to-end proof closing Epic 8 task 7.13 and ROADMAP success criterion 3"
  - "test_formation_result_through_json_markdown_table_heralds and test_formation_partial_results_through_all_three_heralds, both exercised via `cargo test --test lib -- integration::battalion_herald_end_to_end_test`"
affects: ["02-09 (ledger row upgrade for Epic 8 task 7.13)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "FormationMockPaladinPort: a name-keyed (output, token_count, execution_time_ms) response map with an IntegrationMockPaladinPort-style with_failures() configurable-failure mechanism, reused across both tests in the file"

key-files:
  created:
    - tests/integration/battalion_herald_end_to_end_test.rs
  modified:
    - tests/integration/mod.rs

key-decisions:
  - "Split the plan's two tasks into two atomic commits against the same file: task 1 committed the file with only the happy-path test and a non-failing mock; task 2's diff added the with_failures mechanism and the second test on top"
  - "Used node_errors array length (not a paladin_failure_count JSON field, which JsonHerald does not emit) as the structured JSON signal for failure count, matching the repo's own existing JsonHerald test convention"

requirements-completed: [GAP-03]

coverage:
  - id: D1
    description: "A real FormationExecutionService run over three mock Paladins, formatted through JsonHerald, MarkdownHerald and TableHerald, shows Battalion name/id/strategy, positional per-Paladin execution order, and an aggregate token total derived from the mocks' own counts"
    requirement: "GAP-03"
    verification:
      - kind: integration
        ref: "tests/integration/battalion_herald_end_to_end_test.rs#test_formation_result_through_json_markdown_table_heralds"
        status: pass
    human_judgment: false
  - id: D2
    description: "A Formation with one deliberately-failed Paladin under ContinueOnError renders a partial result — two successes, one named failure — through all three Heralds"
    requirement: "GAP-03"
    verification:
      - kind: integration
        ref: "tests/integration/battalion_herald_end_to_end_test.rs#test_formation_partial_results_through_all_three_heralds"
        status: pass
    human_judgment: false

# Metrics
duration: ~50min
completed: 2026-08-01
status: complete
---

# Phase 02 Plan 05: Formation-to-Herald End-to-End Proof Summary

**A real `FormationExecutionService` run over three named mock Paladins, rendered through JsonHerald, MarkdownHerald and TableHerald, proves ROADMAP success criterion 3 and closes Epic 8 task 7.13 — no hand-built `BattalionResult` literal anywhere in the file.**

## Performance

- **Duration:** ~50 min (dominated by a cold `cargo test`/`cargo clippy --workspace` dependency compile)
- **Tasks:** 2
- **Files modified:** 2 (1 created, 1 modified)

## Accomplishments
- `tests/integration/battalion_herald_end_to_end_test.rs` drives a real `FormationExecutionService::execute` over three mock Paladins with distinct, non-round token counts and execution times, then formats the *same* `BattalionResult` through `JsonHerald`, `MarkdownHerald` and `TableHerald` in one test, asserting Battalion name/id/strategy, positional (not containment) per-Paladin execution order in the JSON array, and an aggregate token total computed in the test from the mocks' own counts.
- A second test configures a `ContinueOnError` Formation with a deliberately-failed second Paladin and asserts `paladin_success_count == 2`, `paladin_failure_count == 1`, and a single `node_errors` entry naming the failed Paladin — then renders that partial result through all three Heralds, checking the JSON `node_errors` array and `paladin_results` length as structured fields (not substring search).
- `tests/integration/mod.rs` declares the new module in its existing alphabetical order, no feature guard.

## Task Commits

1. **Task 1: Formation-driven happy path through JSON, Markdown and Table Heralds** - `26e999f` (test)
2. **Task 2: Partial results — one deliberately-failed Paladin, rendered by all three Heralds** - `9143924` (test)

_Note: Both tasks touch the same new file. Task 1's commit contains the file with only the happy-path test and a `FormationMockPaladinPort` that has no failure-injection capability; task 2's commit is the diff that adds the `with_failures()` mechanism (following `IntegrationMockPaladinPort`'s pattern) and the second test on top. Each commit was independently built, tested (`cargo test --test lib -- integration::battalion_herald_end_to_end_test`), `cargo fmt --check`'d and `cargo clippy --workspace --all-targets -- -D warnings`'d before being made._

## Files Created/Modified
- `tests/integration/battalion_herald_end_to_end_test.rs` - The Formation-driven, three-Herald end-to-end test (two `#[tokio::test]` functions, a `MockLlmPort`, and a `FormationMockPaladinPort` with configurable per-Paladin responses and failures)
- `tests/integration/mod.rs` - Added `pub mod battalion_herald_end_to_end_test;` in alphabetical order

## Decisions Made
- Derived the expected token total inside each test from the mocks' own configured counts (`responses.values().map(...).sum()`) rather than hardcoding it, so changing a mock's count changes the expected value — satisfying the plan's anti-tautology requirement (T-02-18 in the plan's threat model).
- Used the JSON `node_errors` array's length as the structured signal for "failure count" in the JSON assertion, since `JsonHerald::battalion_result_to_json` does not emit a `paladin_failure_count` field (only `MarkdownHerald` and `TableHerald` surface success/failure counts as human-readable fields). This mirrors the existing `test_json_herald_battalion_includes_strategy_and_total_tokens` test already in `paladin-herald`'s own test suite.
- Reused `herald_integration_test.rs`'s `MockLlmPort`/`PaladinBuilder` setup shape and `commander_integration_tests.rs`'s `IntegrationMockPaladinPort` failure-injection pattern rather than inventing new mock shapes, per the plan's `<read_first>` guidance.

## Deviations from Plan

None - plan executed exactly as written. Both tasks' `<behavior>`, `<action>` and `<acceptance_criteria>` sections were followed directly; no Rule 1-4 auto-fixes were needed because plan 02-04's `FormationExecutionService` aggregation fields (`per_paladin_times`, `per_paladin_tokens`, `total_tokens`, `node_errors`) were already present and correct, as its own unit tests (`test_formation_aggregates_per_paladin_times_and_tokens`, `test_formation_records_node_errors_on_continue_on_error`) confirmed before writing this plan's tests.

## Issues Encountered
None. The cold dependency build (`cargo test`/`cargo clippy --workspace --all-targets`) took several minutes each on first run in this worktree, but no compile or test failures occurred at any point — both new tests passed on the first attempt.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- `tests/integration/battalion_herald_end_to_end_test.rs` and the exact command `cargo test --test lib -- integration::battalion_herald_end_to_end_test` are ready for plan 02-09 to cite when it upgrades Epic 8 task 7.13's ledger row from `present, unproven` to `satisfied` at the D-19 evidence bar.
- `cargo test --workspace` (all crates, unit + integration + doctests) and `cargo clippy --workspace --all-targets -- -D warnings` both exit 0 with the new tests included, confirming no regression to the rest of the workspace.
- No blockers for downstream plans.

## Self-Check: PASSED

- FOUND: `tests/integration/battalion_herald_end_to_end_test.rs`
- FOUND: `tests/integration/mod.rs` (modified, contains `pub mod battalion_herald_end_to_end_test;`)
- FOUND commit `26e999f` (task 1)
- FOUND commit `9143924` (task 2)
- Verified: `cargo test --test lib -- integration::battalion_herald_end_to_end_test` → 2 passed, 0 failed
- Verified: `cargo test --workspace` → 0 failed across every `test result:` line in the full run
- Verified: `cargo fmt --check` → clean
- Verified: `cargo clippy --workspace --all-targets -- -D warnings` → clean
- Verified: `grep -c 'BattalionResult {' tests/integration/battalion_herald_end_to_end_test.rs` → 0

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*
