---
phase: 03-verification-depth
plan: 02
subsystem: testing
tags: [rust, tokio, async-trait, commander, battalion, formation, mocking, error-strategy]

# Dependency graph
requires:
  - phase: 03-verification-depth (03-01)
    provides: coverage baseline confirming the four ignored Commander stubs as a named gap
provides:
  - "FaultyPaladinPort — configurable failing PaladinPort mock (fail-always, fail-a-named-Paladin, fail-until-Nth-attempt, controllable delay) in tests/helpers/mock_paladin_port.rs"
  - "Four real, passing Commander error-path integration tests in tests/integration/commander_error_paths_test.rs"
  - "commander.rs with zero skip attributes — the four #[ignore]d empty stubs are gone"
affects: [03-03, 03-07, phase-04-testing]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "FaultyPaladinPort: Arc<Mutex<_>> interior mutability (never Rc/RefCell) for a Send + Sync fault-injection mock, union of four existing in-workspace mock idioms"
    - "Global (not per-Paladin) invocation counter drives fail_until_attempt — tests must reason about the shared counter across every Paladin executed through one port instance"

key-files:
  created:
    - tests/integration/commander_error_paths_test.rs
  modified:
    - tests/helpers/mock_paladin_port.rs
    - tests/helpers/mod.rs
    - tests/integration/mod.rs
    - crates/paladin-battalion/src/commander.rs

key-decisions:
  - "FaultyPaladinPort's fail_until_attempt operates on a single counter shared across every Paladin passed through the port (not a per-Paladin counter) — the retry test was designed around this exact global-counter semantics rather than fighting it"
  - "fail_paladin(name) is a chainable, additive builder method (pushes onto Arc<Mutex<Vec<String>>>) so a single mock instance can be configured to fail more than one named Paladin, needed for the two-of-three-failing ContinueOnError test"

requirements-completed: [QUAL-04]

coverage:
  - id: D1
    description: "FaultyPaladinPort exists, is Send + Sync, and supports all four fault modes (fail-always, fail-a-named-Paladin, fail-until-Nth-attempt, controllable delay)"
    requirement: "QUAL-04"
    verification:
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_is_send_and_sync"
        status: pass
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_new_succeeds_and_logs_execution"
        status: pass
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_fail_always_fails_every_execution"
        status: pass
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_fail_paladin_fails_only_named_paladin"
        status: pass
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_fail_until_attempt_then_succeeds"
        status: pass
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_with_delay_ms_sleeps_before_deciding"
        status: pass
      - kind: unit
        ref: "tests/helpers/mock_paladin_port.rs#faulty_paladin_port_execution_log_records_invocation_order"
        status: pass
    human_judgment: false
  - id: D2
    description: "The four relocated Commander error-path tests run and pass: fail-fast stops on first error, continue-on-error collects distinct errors, retry-then-continue retries an exact number of times, partial results preserved alongside a collected failure"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/commander_error_paths_test.rs#test_fail_fast_stops_on_first_error"
        status: pass
      - kind: integration
        ref: "tests/integration/commander_error_paths_test.rs#test_continue_on_error_collects_all_errors"
        status: pass
      - kind: integration
        ref: "tests/integration/commander_error_paths_test.rs#test_retry_then_continue_retries_failed_paladins"
        status: pass
      - kind: integration
        ref: "tests/integration/commander_error_paths_test.rs#test_partial_results_returned_with_errors"
        status: pass
    human_judgment: false
  - id: D3
    description: "commander.rs carries zero skip attributes and no stub bodies; the in-crate MockPaladinPort and all its passing tests are untouched"
    requirement: "QUAL-04"
    verification:
      - kind: other
        ref: "grep -c '#\\[ignore\\]' crates/paladin-battalion/src/commander.rs == 0"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-battalion --offline (206 passed, 0 failed)"
        status: pass
    human_judgment: false

# Metrics
duration: ~25min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 2: Commander Error-Path Tests via FaultyPaladinPort Summary

**FaultyPaladinPort (fail-always/fail-named/fail-until-Nth-attempt/delay) built in `tests/helpers/`, driving four newly-real Commander error-path integration tests that replace four `#[ignore]`d empty stubs**

## Performance

- **Duration:** ~25 min
- **Tasks:** 2
- **Files modified:** 4 modified, 1 created

## Accomplishments

- `FaultyPaladinPort` added to `tests/helpers/mock_paladin_port.rs` alongside the existing
  `MockPaladinPort`, combining the retry-counter idiom (`formation_service.rs`), the
  `fail_paladin_names` + `delay_ms` idiom (`phalanx_service.rs`), and the `execution_log`
  idiom (`commander_integration_tests.rs`'s `IntegrationMockPaladinPort`) into one shared
  mock, all built on `Arc<Mutex<_>>` interior mutability and proven `Send + Sync` by a
  witness test.
- Four real, passing Commander error-path tests now live in
  `tests/integration/commander_error_paths_test.rs`: `test_fail_fast_stops_on_first_error`,
  `test_continue_on_error_collects_all_errors`, `test_retry_then_continue_retries_failed_paladins`,
  `test_partial_results_returned_with_errors` — wired into the `lib` test binary with a
  single `pub mod` line, no `Cargo.toml` change.
- `crates/paladin-battalion/src/commander.rs` now carries zero skip attributes — the four
  `#[ignore]`d, three-comment-line stub tests (never actually implemented) and their
  explanatory comment block are gone; the in-crate `MockPaladinPort` and its 206 passing
  tests are untouched.

## Task Commits

Each task was committed atomically:

1. **Task 1: Build FaultyPaladinPort in the shared helpers home and export it from the barrel** - `fa33e0c` (feat)
2. **Task 2: Relocate the four Commander error-path tests into a real integration module and retire the stubs** - `cb77e65` (test)

_Note: no separate plan-metadata commit for this run; STATE.md/ROADMAP.md/REQUIREMENTS.md updates land in the phase-level final commit._

## Files Created/Modified

- `tests/helpers/mock_paladin_port.rs` - Added `FaultyPaladinPort` struct, builder methods (`fail_always`, `fail_paladin`, `fail_until_attempt`, `with_delay_ms`), `PaladinPort` impl, and a `#[cfg(test)] mod tests` block proving all six configured behaviours plus a `Send + Sync` witness
- `tests/helpers/mod.rs` - Extended the barrel re-export to `pub use mock_paladin_port::{FaultyPaladinPort, MockPaladinPort};`
- `tests/integration/commander_error_paths_test.rs` - New file with the four real Commander error-path tests, built against `FaultyPaladinPort` and `CommanderBuilder`
- `tests/integration/mod.rs` - Added `pub mod commander_error_paths_test;` at its alphabetical position, before `commander_integration_tests`
- `crates/paladin-battalion/src/commander.rs` - Deleted the four `#[ignore]`d empty stub tests and their explanatory comment block; replaced with a one-line pointer comment to the new location

## Decisions Made

- **`fail_until_attempt`'s counter is global, not per-Paladin.** `FaultyPaladinPort` shares one `Arc<Mutex<usize>>` invocation counter across every Paladin executed through a given port instance, matching the pattern's origin in `formation_service.rs`'s in-crate mock. The retry test (`test_retry_then_continue_retries_failed_paladins`) was designed around this: with three Paladins and `fail_until_attempt(2)`, Paladin-1 needs 3 global calls to succeed (retries occurred), then Paladin-2 and Paladin-3 each succeed on their first (4th and 5th global) call — an exact, deterministic `call_count() == 5`, not a range.
- **`fail_paladin(name)` is additive and chainable**, pushing onto an `Arc<Mutex<Vec<String>>>` rather than replacing a single name. This let `test_continue_on_error_collects_all_errors` configure two-of-three Paladins to fail (`.fail_paladin("Paladin-1").fail_paladin("Paladin-3")`) without a separate multi-name constructor.
- **Log-entry assertions use `starts_with("{name}:")`, not `contains(name)`, where a chained Formation input could produce a false match.** Formation carries each Paladin's output into the next Paladin's input, so a later Paladin's log entry can contain an earlier Paladin's name as substring text inside the carried-over input. The retry test's "how many times did Paladin-1 execute" assertion had to be corrected from a `contains` check (which over-counted, catching 5 instead of 3) to a `starts_with("Paladin-1:")` check that only matches the log entry's leading executor-name field. Caught and fixed during Task 2 verification, before commit.

## Deviations from Plan

None - plan executed exactly as written. The log-entry substring fix above was caught during the plan's own `<verify>` loop (a test failure, corrected before committing), not a deviation from what was planned — the plan's own acceptance criteria (`cargo test --offline --test lib -- commander_error_paths` reporting `4 passed; 0 failed; 0 ignored`) is what surfaced it.

## Issues Encountered

- Formation's sequential chaining (each Paladin's output becomes the next Paladin's input) meant the naive `execution_log.contains("Paladin-1")` substring assertion in the retry test matched more entries than intended, since later Paladins' inputs carried the string "Paladin-1" forward from an earlier successful output. Resolved by matching on the log entry's leading `"{name}:"` prefix instead of a full-string substring search. See Decisions Made above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `tests/helpers/mock_paladin_port.rs` now has a real external consumer (`tests/integration/commander_error_paths_test.rs`), closing the "0% coverage, imported only by its own barrel" gap D-09/D-10 named.
- `FaultyPaladinPort` is available to any future Commander/Battalion test needing configurable fault injection, without inventing a sixth parallel mock.
- No blockers for 03-03 (MCP streamable-HTTP error paths) or 03-07 (coverage exit measurement) — this plan's wiring keeps the coverage object count stable (single `pub mod` line, no new `[[test]]` target) as 03-07 depends on.

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

All created/modified files confirmed present on disk; both task commits (`fa33e0c`, `cb77e65`) confirmed in `git log`.
