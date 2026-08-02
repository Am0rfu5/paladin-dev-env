---
phase: 03-verification-depth
plan: 07
subsystem: testing

tags: [llvm-cov, coverage-gate, integration-tests, adr-0006, qual-03]

# Dependency graph
requires:
  - phase: 03-verification-depth
    provides: "Plan 03-01's entry coverage measurement and ADR-0006's binding pipeline; plans 03-02 through 03-06's new test-writing this run re-measures against"
provides:
  - "Exit coverage measurement (85.92% workspace line coverage) proving ADR-0006's 84% floor still holds at the phase's ending commit"
  - "QUAL-02 closure ledger confirming 4 of the entry run's 5 zero-coverage files are closed, 1 (src/bin/paladin-server.rs) deferred to Phase 5"
  - "QUAL-03 critical-path exerciser evidence at the D-19 bar for all three named paths, with the percentage clause recorded superseded"
affects: [verify-work, gsd-secure-phase, phase-05-verify, phase-15-pipe]

tech-stack:
  added: []
  patterns: ["Two-run coverage evidence pattern: entry measurement (start of phase) + exit measurement (end of phase), appended side by side in one evidence file, never merged"]

key-files:
  created:
    - .planning/phases/03-verification-depth/03-critical-path-exercisers.md
  modified:
    - .planning/phases/03-verification-depth/03-coverage-measurement.md

key-decisions:
  - "Exit measurement re-ran ADR-0006's verbatim pipeline (RUSTFLAGS instrument-coverage, llvm-profdata merge, --object discovery, llvm-cov report) at commit 1ad8be5, unchanged flags/regex/feature-scope from the entry run and from ADR-0006 itself"
  - "Ratchet trigger not fired: exit delta 1.92pp remains below the 2pp milestone-close threshold; ADR-0006 not amended, per the same reasoning the entry run recorded (raising the floor is a milestone-close action, not Phase 3's)"
  - "QUAL-03's percentage clause recorded superseded by ADR-0006 with zero coverage percentages introduced anywhere in the exerciser evidence file"

requirements-completed: [QUAL-01, QUAL-02, QUAL-03]

coverage:
  - id: D1
    description: "Exit coverage measurement re-runs ADR-0006's pipeline verbatim at the phase's ending HEAD, proving the 84% floor still holds (measured 85.92%, PASS by 1.92pp)"
    requirement: "QUAL-01"
    verification:
      - kind: other
        ref: "llvm-cov report TOTAL row pasted in .planning/phases/03-verification-depth/03-coverage-measurement.md ## Exit measurement section"
        status: pass
    human_judgment: false
  - id: D2
    description: "QUAL-02 closure ledger: 4 of 5 entry-run zero-coverage files closed (redis.rs, file_storage_port.rs, paladin-llm/error.rs, arsenal_port.rs), 1 deferred with owner (src/bin/paladin-server.rs, Phase 5/VERIFY-05)"
    requirement: "QUAL-02"
    verification:
      - kind: other
        ref: "03-coverage-measurement.md ### QUAL-02 closure ledger table"
        status: pass
    human_judgment: false
  - id: D3
    description: "QUAL-03's three critical paths (Paladin execution, Battalion orchestration, tool invocation) each carry file:line citations plus verified passing non-#[ignore]d exercisers at the D-19 bar; percentage clause recorded superseded"
    requirement: "QUAL-03"
    verification:
      - kind: integration
        ref: "tests/integration/paladin_integration_test.rs#test_end_to_end_paladin_execution"
        status: pass
      - kind: integration
        ref: "tests/integration/commander_integration_tests.rs#test_commander_executes_formation_end_to_end"
        status: pass
      - kind: integration
        ref: "tests/integration/battalion/load_test.rs#test_load_formation_50_concurrent_battalions"
        status: pass
      - kind: integration
        ref: "tests/integration/arsenal_bridge_regression_test.rs#function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call"
        status: pass
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_round_trip_with_correct_bearer_token_succeeds"
        status: pass
    human_judgment: false

duration: 14min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 07: Exit Coverage Re-measurement & QUAL-03 Critical-Path Evidence Summary

**Exit coverage measures 85.92% (up 0.36pp from the 85.56% entry figure), closes 4 of 5 entry-run zero-coverage files, and establishes named passing exercisers for all three QUAL-03 critical paths with zero invented percentages.**

## Performance

- **Duration:** 14 min
- **Started:** 2026-08-02T17:36:20Z
- **Completed:** 2026-08-02T17:49:45Z
- **Tasks:** 2
- **Files modified:** 2 (1 appended, 1 created)

## Accomplishments

- Re-ran ADR-0006's verbatim coverage pipeline at commit `1ad8be53d1ffd383e5ed45b35b04c9a7ab4abde1` (phase's ending HEAD): instrumented `cargo test --workspace --offline` (35 suites, 0 failed), `llvm-profdata merge`, `--object` discovery (31 test binaries — byte-identical count and hashes to the entry run), `llvm-cov report`.
- Measured workspace line coverage **85.92%** (63,821 lines counted, 8,984 missed) — PASS against the 84.00% floor by 1.92 percentage points, appended to `03-coverage-measurement.md` as a second `## Exit measurement` section standing side by side with the untouched entry section.
- Post-phase zero-coverage set drops from 5 files to 1: `redis.rs` (34.69%), `file_storage_port.rs` (79.11%), `paladin-llm/error.rs` (83.50%), and `arsenal_port.rs` (95.00%) are all closed by plans 03-05/03-06's new unit tests; `src/bin/paladin-server.rs` remains at exactly 0.00%, deferred to Phase 5 / VERIFY-05 exactly as plan 03-06 recorded.
- QUAL-02 closure ledger recorded with a verdict and owner for every entry-run zero-coverage file plus the two out-of-scope rows (`minio.rs`, the generated `target/` file).
- Ratchet readiness re-evaluated at exit: delta 1.92pp remains below the 2pp milestone-close trigger; ADR-0006 not amended.
- New `03-critical-path-exercisers.md` establishes QUAL-03's D-19-bar evidence: `test_end_to_end_paladin_execution` for Paladin execution; `test_commander_executes_formation_end_to_end` and `test_load_formation_50_concurrent_battalions` for Battalion orchestration; `function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call` and `streamable_http_round_trip_with_correct_bearer_token_succeeds` for tool invocation — all five verified passing under the default parallel `cargo test --offline` harness, no test-thread-limiting flag used, no coverage percentage recorded.

## Task Commits

Each task was committed atomically:

1. **Task 1: Re-run the pipeline at the phase's ending HEAD and record the exit measurement** - `d98a4d6` (docs)
2. **Task 2: Establish QUAL-03's critical-path exerciser evidence at the D-19 bar** - `e133cde` (docs)

**Plan metadata:** committed separately after this SUMMARY (see final commit below).

## Files Created/Modified

- `.planning/phases/03-verification-depth/03-coverage-measurement.md` - appended `## Exit measurement` section (environment probes, pipeline commands, full `llvm-cov report` stdout, zero-coverage set, QUAL-02 closure ledger, object-count delta, ratchet readiness) without touching the entry section
- `.planning/phases/03-verification-depth/03-critical-path-exercisers.md` - new file recording QUAL-03's D-19-bar evidence for all three critical paths plus the superseded-percentage-clause disposition

## Decisions Made

- Reused ADR-0006's command exactly (no flag, regex, env var, or feature-set change) — required by D-02, and necessary so the exit figure is comparable to both the entry figure and ADR-0006's original 84.79% baseline.
- Did not fire the ratchet trigger despite the delta narrowing from 1.56pp (entry) to 1.92pp (exit) — still short of the 2pp threshold, and per ADR-0006's own text the raise is scoped to milestone close, not phase close.
- Selected `test_end_to_end_paladin_execution` as the sole Paladin-execution exerciser after searching the full `tests/integration/` tree (per the plan's read_first instruction) rather than crediting a test whose primary assertions are about a different concern (e.g. `function_call_dispatch_still_invokes_arsenal_exactly_once_with_matching_call`, which constructs a `Paladin` as scaffolding but asserts Arsenal dispatch behavior — kept under tool invocation only, with no shared-citation claim).
- Recorded QUAL-03's percentage clause in paraphrase (no literal digit-percent tokens) to satisfy the record's own zero-percentage constraint while still conveying its substance and disposition.

## Deviations from Plan

None - plan executed exactly as written. Both tasks' automated `<verify>` checks passed on the first attempt with no fix-up commits needed.

## Issues Encountered

None. The instrumented coverage run and all five exerciser verification runs completed cleanly; no test failures, no compile errors, no ENOSPC (disk stayed at 19GB free throughout).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Both coverage runs (entry and exit) now stand recorded side by side in `03-coverage-measurement.md`, giving plan 03-08 (and any later audit) a clean before/after comparison at the phase boundary.
- QUAL-03's evidence file gives plan 03-08 everything it needs to amend the requirement's percentage clause at source in `REQUIREMENTS.md` without re-deriving any test evidence.
- `src/bin/paladin-server.rs` remains the one open QUAL-02 item, already owned by Phase 5 / VERIFY-05 — no action needed from this plan or from 03-08.
- No blockers for plan 03-08.

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

All created/modified files verified present on disk; both task commits (`d98a4d6`, `e133cde`) verified present in `git log --oneline --all`.
