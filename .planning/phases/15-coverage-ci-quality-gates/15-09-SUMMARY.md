---
phase: 15-coverage-ci-quality-gates
plan: 09
subsystem: testing
tags: [rust, tokio, event-listener, concurrency, coverage-remeasurement]

requires:
  - phase: 15-coverage-ci-quality-gates (plan 15-08)
    provides: "listener.rs's extended #[cfg(test)] mod tests (3 -> 22 tests), the DEFER-03 entry record stating NOT MEASURED, and the discovered chrono-vs-tokio-clock fact this plan reuses rather than rediscovers"
  - phase: 15-coverage-ci-quality-gates (plan 15-05)
    provides: "src/test_support/event_factory::build_event_batch, the bulk constructor that makes the 1000-plus-event burst test expressible in one call"
provides:
  - "Four #[tokio::test(flavor = \"multi_thread\")] concurrency/stress tests closing DEFER-03's fifth named area: multi-producer emission (exact-total assertion), concurrent registration/unregistration during active processing (list_listeners/get_all_stats consistency), a 1200-event burst across 4 producers (exact trigger_queue_length and per-listener ListenerStats counters), and drop-during-active-processing (Weak-reference proof no lock is leaked, since the type has no shutdown() method and no custom Drop)"
  - "A real DEFER-03 exit coverage measurement -- 96.90% line coverage for listener.rs (1161 lines, 36 missed) -- produced via the raw rustc -C instrument-coverage + llvm-profdata + llvm-cov pipeline (ADR-0006's own local substitute for cargo-llvm-cov, still uninstallable here: crates.io returns HTTP 403, reconfirmed), scoped to the module's own 27 tests"
  - "A one-test fix for a genuine gap this measurement discovered: update_trigger_status's triggers_completed/triggers_failed increment arms were never exercised by 15-08's status round-trip test (it never registered a listener under the trigger's source), closed by update_trigger_status_increments_the_registered_listeners_completed_and_failed_counters"
  - "A completed DEFER-03 justification block (entry -> exit, untested-path accounting, re-derived effort superseding the register's inherited 20-25h/35-45h figures, and the concurrency-testing-pattern prerequisite recorded as satisfied) in the comment block plan 15-08 opened"
affects: [15-10]

tech-stack:
  added: []
  patterns:
    - "tokio::time::timeout wrapping every concurrent test section, asserted not to have elapsed, so a lock-ordering deadlock fails the test with a named panic instead of hanging the runner"
    - "Weak-reference teardown proof: Arc::downgrade before spawning a task that holds the last strong Arc clone, then asserting weak.upgrade().is_none() after the task completes, as the way to demonstrate 'no lock was left held' for a type with no shutdown() affordance and no custom Drop"
    - "Rate-limit-neutral test config (non_interfering_config(): max_triggers_per_window: 100_000, time_window_seconds: 3600) applied to every concurrency/stress listener, so the rate-limit boundary this same module already tests elsewhere cannot silently absorb part of an exact-count assertion"
    - "Raw rustc/llvm-profdata/llvm-cov coverage pipeline (ADR-0006's local substitute for cargo-llvm-cov), scoped to a single package + module test filter rather than --workspace, as the module-level coverage measurement technique when cargo-llvm-cov cannot be installed"

key-files:
  modified:
    - src/application/services/orchestration/listener.rs

key-decisions:
  - "cargo-llvm-cov remains uninstalled and uninstallable in this authoring environment -- crates.io returns HTTP 403 (reconfirmed via curl -sSI, and via a `cargo install cargo-llvm-cov --locked` attempt that did not complete inside 30s), the same constraint ADR-0006 and plan 15-08 both record. Rather than repeating 15-08's NOT MEASURED outcome, this plan used ADR-0006's own already-proven local substitute -- the raw `rustc -C instrument-coverage` + rustup-toolchain-bundled `llvm-profdata`/`llvm-cov` pipeline -- scoped to `cargo test -p paladin-ai --lib application::services::orchestration::listener` (the module's own 27 tests only), which is both a real measurement and squarely inside this plan's own harness policy's 'scoped test invocation' allowance."
  - "The exit figure (96.90%) is explicitly NOT the plan's literal `cargo llvm-cov --workspace --lib` scope and is recorded as such -- narrower in one direction (only this module's own tests run, not every workspace lib test that might incidentally touch this file), offline/local rather than CI's Docker-backed run in another. It is also explicitly not comparable to ADR-0006's 82.39% workspace `--features integration-tests` gate figure -- stated inline in the comment block per the plan's own instruction, consistent with ADR-0006's repeated same warning about scope-incomparability."
  - "A genuinely new gap discovered mid-measurement (update_trigger_status's stat-increment arms, never exercised because 15-08's status round-trip test never registered a listener under the trigger's source) was closed with a new test in the same commit rather than merely justified in prose -- cheap, real, and directly actionable, unlike the two remaining justified gaps (the Debug impl's no-branching boilerplate, and the create_trigger error arm MockEventListener structurally cannot reach without expanding the mock beyond this plan's scope)."
  - "The register's inherited 20-25h (Module 2) / 35-45h (register total) effort estimates are recorded as superseded, not merely updated: the estimate budgeted for infrastructure (MockEventSource/MockTriggerExecutor, a mock-clock framework) that turned out unnecessary or inapplicable (this module reads chrono::Utc::now(), not tokio::time::Instant, so no mock-clock framework built to the register's spec could ever have controlled it) -- actual effort across plans 15-08 and 15-09 was on the order of low single-digit hours."
  - "The graceful-shutdown scenario tests the type's actual behavior (ordinary Arc reference-counting over three Arc-wrapped fields) rather than inventing a shutdown() call -- confirmed by direct inspection that ListenerOrchestrator has no shutdown method and no custom Drop impl, per the plan's explicit instruction not to add one inside a coverage plan."

requirements-completed: [DEFER-03]

coverage:
  - id: D1
    description: "Concurrent emission from multiple producers: 8 producers x 50 events x 3 listeners under a shared Arc<ListenerOrchestrator>, exact arithmetic total asserted (not a lower bound), timeout-guarded, multi_thread runtime"
    requirement: DEFER-03
    verification:
      - kind: unit
        ref: "listener.rs#tests::concurrent_emission_from_multiple_producers_yields_the_exact_expected_trigger_total"
        status: pass
    human_judgment: false
  - id: D2
    description: "Concurrent registration and unregistration during active processing: one task drives 300 events while another registers/unregisters 50 listeners; list_listeners and get_all_stats asserted to agree exactly on the surviving set"
    requirement: DEFER-03
    verification:
      - kind: unit
        ref: "listener.rs#tests::concurrent_registration_and_unregistration_during_active_processing_stays_consistent"
        status: pass
    human_judgment: false
  - id: D3
    description: "A 1000-plus event burst (1200 events via the shared build_event_batch bulk constructor) driven through 4 concurrent producer tasks against 2 listeners; trigger_queue_length and both listeners' triggers_created/events_processed asserted for exact equality against the arithmetic total"
    requirement: DEFER-03
    verification:
      - kind: unit
        ref: "listener.rs#tests::a_1000_plus_event_burst_across_several_producers_yields_exact_aggregate_counts"
        status: pass
    human_judgment: false
  - id: D4
    description: "Graceful shutdown during active processing: ListenerOrchestrator has no shutdown() method and no custom Drop, confirmed by direct inspection; the test drops the caller's Arc handle while a worker task (holding its own clone) is mid-flight, asserts the worker completes without panic inside a timeout, and proves no lock was left held via a Weak-reference upgrade failing once every strong reference is gone"
    requirement: DEFER-03
    verification:
      - kind: unit
        ref: "listener.rs#tests::dropping_the_orchestrator_during_active_processing_completes_without_panicking_or_leaking_a_lock"
        status: pass
    human_judgment: false
  - id: D5
    description: "DEFER-03 exit coverage measurement: 96.90% line coverage for listener.rs (1161 lines, 36 missed), measured via the raw rustc/llvm-profdata/llvm-cov pipeline scoped to the module's own 27 tests, clearing the 80% module bar by 16.90 points; every remaining untested line categorized and justified in the comment block, not silently absorbed"
    requirement: DEFER-03
    verification:
      - kind: other
        ref: "src/application/services/orchestration/listener.rs, DEFER-03 exit record comment block at the head of #[cfg(test)] mod tests (commit 6a66719)"
        status: pass
    human_judgment: false
  - id: D6
    description: "Discovered gap closed: update_trigger_status's listener-side triggers_completed/triggers_failed increment arms, unexercised by 15-08's status round-trip test, now covered with exact per-status counter assertions"
    requirement: DEFER-03
    verification:
      - kind: unit
        ref: "listener.rs#tests::update_trigger_status_increments_the_registered_listeners_completed_and_failed_counters"
        status: pass
    human_judgment: false

duration: ~26min
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 09: Listener concurrency/stress suite + DEFER-03 exit measurement Summary

**Closed DEFER-03's concurrency/stress half with four timeout-guarded, exact-assertion `#[tokio::test(flavor = "multi_thread")]` scenarios, then produced a real 96.90% exit coverage measurement for `listener.rs` via ADR-0006's local raw-`llvm-cov` pipeline (since `cargo-llvm-cov` remains uninstallable in this environment) and closed one genuine coverage gap the measurement surfaced -- all with zero production-code changes.**

## Performance

- **Duration:** ~26 min
- **Tasks:** 2 completed
- **Files modified:** 1 (`src/application/services/orchestration/listener.rs`)

## Accomplishments

- **Task 1 -- the four concurrency and stress scenarios DEFER-03 names.** Extended `listener.rs`'s existing `#[cfg(test)] mod tests` block (22 tests, from plan 15-08) with four new `#[tokio::test(flavor = "multi_thread")]` tests, each wrapped in an explicit `tokio::time::timeout`:
  - `concurrent_emission_from_multiple_producers_yields_the_exact_expected_trigger_total` -- 8 producers x 50 events x 3 listeners, asserts the trigger total equals the exact arithmetic product (400 events x 3 listeners = 1200), not a lower bound.
  - `concurrent_registration_and_unregistration_during_active_processing_stays_consistent` -- one task drives 300 events while another registers 50 listeners and unregisters half of them concurrently; asserts `list_listeners` and `get_all_stats` agree exactly on the surviving set (52 listeners: 2 steady + 25 surviving churn).
  - `a_1000_plus_event_burst_across_several_producers_yields_exact_aggregate_counts` -- a 1200-event burst built in one call via `event_factory::build_event_batch`, driven through 4 concurrent producer tasks against 2 listeners; asserts `trigger_queue_length` (2400) and both listeners' `events_processed`/`triggers_created` (1200 each) for exact equality.
  - `dropping_the_orchestrator_during_active_processing_completes_without_panicking_or_leaking_a_lock` -- `ListenerOrchestrator` has no `shutdown()` method and no custom `Drop` (confirmed by direct inspection of the struct and its `impl` block, per the plan's explicit instruction not to invent one). The test drops the caller's `Arc` handle while a worker task holding its own clone is processing 200 events, asserts the worker completes without panicking inside a 15-second timeout, and proves no lock was left held via a `Weak` reference that fails to upgrade only once every strong reference is gone.

  Every listener registered by these tests uses a rate-limit-neutral configuration (`max_triggers_per_window: 100_000`, `time_window_seconds: 3600`) so the rate-limit boundary this module already tests elsewhere cannot silently absorb part of an exact-count assertion. Verified stable across 20 consecutive runs (twice -- once after Task 1, once again after Task 2's addition).

- **Task 2 -- a real exit measurement, plus one closed gap.** `cargo-llvm-cov` is still not installable in this environment (`curl -sSI https://crates.io/` returns HTTP 403; a direct `cargo install cargo-llvm-cov --locked` attempt did not complete inside 30 seconds) -- the same constraint plan 15-08 and ADR-0006 both record. Rather than repeating 15-08's `NOT MEASURED` outcome, this plan used ADR-0006's own already-proven local substitute: the raw `rustc -C instrument-coverage` + rustup-toolchain-bundled `llvm-profdata`/`llvm-cov` pipeline (confirmed present at `$(rustc --print sysroot)/lib/rustlib/.../bin/`), scoped to `cargo test -p paladin-ai --lib application::services::orchestration::listener` -- the module's own tests only, and squarely inside this plan's harness policy's "scoped test invocation" allowance. **Result: 96.90% line coverage** (1161 lines, 36 missed), clearing the 80% module bar by 16.90 points. Every one of the 36 remaining missed lines was inspected individually and categorized in the comment block: the hand-written `Debug` impl for `ListenerWrapper` (no branching logic, nothing to assert), the `create_trigger` error-log arm in `process_event` (structurally unreachable without a mock that returns `Err`, which this plan's scope does not authorize adding), and a handful of `llvm-cov` line-vs-region attribution artifacts on multi-line `match`/`?`/macro constructs (the enclosing statement demonstrably executes). One genuine gap was found and **closed**, not merely justified: `update_trigger_status`'s `triggers_completed`/`triggers_failed` increment arms were never exercised by 15-08's status round-trip test, because that test never registered a listener under the trigger's `source` -- closed with a new test asserting both counters move by exactly one per matching status and not at all for a non-matching one.

## Task Commits

1. **Task 1: The concurrency and stress suite** - `f216c16` (test)
2. **Task 2: Exit measurement, the 80% bar, and the justification record** - `6a66719` (docs)

_No TDD tasks in this plan; both are `type="auto"`._

## Files Created/Modified

- `src/application/services/orchestration/listener.rs` - Four `#[tokio::test(flavor = "multi_thread")]` concurrency/stress tests added (27 tests total, up from 22); one additional test closing a discovered `update_trigger_status` stats gap; the DEFER-03 exit-record comment block completing the entry record plan 15-08 opened. Nothing above the `#[cfg(test)]` marker changed (verified byte-identical after every edit).

## Decisions Made

- **Used ADR-0006's raw local coverage pipeline instead of repeating 15-08's `NOT MEASURED` outcome.** `cargo-llvm-cov` remains uninstalled and uninstallable here (crates.io HTTP 403, reconfirmed), but the rustup-toolchain-bundled `llvm-profdata`/`llvm-cov` binaries are present and produce the same underlying LLVM source-based coverage data. Scoped to this module's own tests via `cargo test -p paladin-ai --lib application::services::orchestration::listener`, matching this plan's own harness policy's explicitly allowed pattern.
- **The 96.90% exit figure is explicitly scoped and labeled non-comparable** to both the plan's originally-specified `--workspace --lib` command (this run is narrower: one module's own tests, not every workspace lib test) and to ADR-0006's 82.39% CI gate figure (different scope, tool, and environment). Recorded inline rather than glossed.
- **Closed the one cheap, real coverage gap found during measurement** (the `update_trigger_status` stat-increment arms) rather than only justifying it, since it required no mock expansion and directly strengthens an existing, previously-incomplete test's claim.
- **Re-derived DEFER-03's inherited effort estimate as superseded, not updated.** The register's 20-25h (Module 2) / 35-45h (register total) figures budgeted for infrastructure (a dedicated mock pair, a mock-clock framework) that turned out unnecessary or structurally inapplicable — this module reads `chrono::Utc::now()`, not `tokio::time::Instant`, so no mock-clock framework matching the register's spec could ever have controlled it. Actual effort across plans 15-08 and 15-09 was on the order of low single-digit hours.
- **The DEFERRED_COVERAGE.md "establish concurrency testing patterns" prerequisite is recorded as satisfied**, naming Task 1's four tests as the reference implementation for future concurrency tests against `tokio::sync::Mutex`/`RwLock`-guarded orchestrators in this workspace.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug, discovered via coverage inspection] `update_trigger_status`'s listener stat-increment arms were untested**
- **Found during:** Task 2, while categorizing the 36 lines the exit measurement flagged as uncovered
- **Issue:** Plan 15-08's `every_supported_trigger_status_round_trips_through_update_and_get` test round-trips every `TriggerStatus` variant through `update_trigger_status`/`get_trigger`, but never registers a listener under the trigger's `source` name first — so `update_trigger_status`'s `listeners.get(&trigger.source)` lookup always misses and the `triggers_completed`/`triggers_failed` increment arms never execute. This is a genuine, previously-undocumented gap in an existing test's coverage claim, not merely a missing new scenario.
- **Fix:** Added `update_trigger_status_increments_the_registered_listeners_completed_and_failed_counters`, which registers a listener first and asserts both counters move by exactly one per matching status (`Completed`, then `Failed`) and stay untouched for a non-matching status (`Pending`).
- **Files modified:** `src/application/services/orchestration/listener.rs` (test-only)
- **Verification:** New test passes; re-measured coverage confirms lines 276-281 (the increment arms) moved from uncovered to covered (line coverage rose from 96.31% to 96.90% after this addition).
- **Committed in:** `6a66719` (Task 2 commit)

**2. [Rule 3-adjacent — environmental constraint honestly worked around, not routed around silently] `cargo-llvm-cov` still uninstallable; used ADR-0006's raw pipeline instead**
- **Found during:** Task 2, attempting the plan's literal verify-block command
- **Issue:** The plan's Task 2 verify block installs `cargo-llvm-cov` via `rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked` before running `cargo llvm-cov --workspace --lib --json`. `curl -sSI https://crates.io/` returns HTTP 403 in this session, and a direct `cargo install cargo-llvm-cov --locked` attempt did not complete inside a 30-second bound — the same environmental constraint ADR-0006 and plan 15-08 both already document.
- **Fix:** Used ADR-0006's own already-proven local substitute instead of repeating a `NOT MEASURED` outcome: the raw `rustc -C instrument-coverage` + rustup-toolchain-bundled `llvm-profdata`/`llvm-cov` pipeline, scoped to this module's own tests (`cargo test -p paladin-ai --lib application::services::orchestration::listener`) rather than an unfiltered `--workspace --lib` run, both to stay within this plan's harness policy's "scoped test invocation" allowance and to produce an accurate module-level figure. The scope difference and non-comparability with the ADR-0006 CI gate figure are both stated explicitly in the exit record.
- **Files modified:** `src/application/services/orchestration/listener.rs` (comment only)
- **Verification:** `command -v cargo-llvm-cov` confirmed empty; the raw pipeline's `llvm-cov report` output is transcribed byte-identical into the comment block, same column order ADR-0006 uses.
- **Committed in:** `6a66719` (Task 2 commit)

---

**Total deviations:** 2 (one Rule-1 bug fix discovered via coverage inspection, one honest environmental-constraint workaround using an already-established alternative tool path)
**Impact on plan:** Neither deviation touched production code or expanded scope beyond what the plan itself anticipated as a fallback. The stat-increment fix directly strengthens DEFER-03's own closure claim; the raw-pipeline substitution produces a real number where the plan's literal command would have failed identically to plan 15-08's.

## Issues Encountered

- **`cargo install cargo-llvm-cov --locked` hung rather than failing fast.** A 30-second timeout was used to confirm the install could not complete rather than waiting indefinitely, consistent with `curl -sSI https://crates.io/`'s already-confirmed HTTP 403.
- **`CARGO_TARGET_DIR` cache invalidation under `RUSTFLAGS="-C instrument-coverage"`.** Changing `RUSTFLAGS` forced a ~4m36s recompile of every workspace crate linked into the `paladin-ai` lib test binary (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-herald`) on the first instrumented run. Expected and one-time; the second instrumented run (after adding the stat-increment test) reused the now-warm instrumented cache.

## User Setup Required

None — no external service configuration required. A future session with working `crates.io` access should install `cargo-llvm-cov` and reproduce the workspace-wide `--features integration-tests` figure this plan's scoped measurement is explicitly not equivalent to, per ADR-0006's own reproducibility bar.

## Next Phase Readiness

DEFER-03 is closed: both halves (15-08's sequential/lifecycle/status/health coverage, this plan's concurrency/stress coverage and exit measurement) are complete, with a real 96.90% figure, every untested path named and justified, and the register's inherited effort estimate re-derived and recorded as superseded. Plan 15-10 (the formal DEFER-03 closure record, and DEFER-01's per-name mock verdict) can proceed — this plan's exit record explicitly names the evidence it needs to cite (the four Task 1 tests as the concurrency-testing-pattern reference, the exit figure and its scope, and the re-derived effort). No blockers.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*

## Self-Check: PASSED

- FOUND: `src/application/services/orchestration/listener.rs`
- FOUND: `.planning/phases/15-coverage-ci-quality-gates/15-09-SUMMARY.md`
- FOUND commit `f216c16` (Task 1)
- FOUND commit `6a66719` (Task 2)
- FOUND commit `3431460` (plan summary)
