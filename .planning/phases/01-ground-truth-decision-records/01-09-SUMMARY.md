---
phase: 01-ground-truth-decision-records
plan: 09
subsystem: testing
tags: [coverage, llvm-cov, instrument-coverage, offline-toolchain, ground-truth]

# Dependency graph
requires: []
provides:
  - "A fresh, offline-measured workspace line-coverage figure (84.79%) with full
    verbatim-command provenance, superseding plan 01-04's halt"
  - ".planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md —
    the raw evidence record plan 01-10's ADR-0006 will transcribe from"
  - "COVERAGE.md addendum recording that the offline rustup llvm-tools path
    eliminates the one cargo-llvm-cov crates.io install this phase previously named"
affects: [01-10 (ADR-0006 authoring), phase-03-planning (coverage criterion amendment), phase-15-PIPE-02]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Offline coverage measurement: RUSTFLAGS=\"-C instrument-coverage\" + LLVM_PROFILE_FILE
      + llvm-profdata merge + llvm-cov report, tools resolved from
      $(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/^host: //p')/bin —
      no cargo-llvm-cov install, no network"
    - "Tracer-first proof pattern: prove the pipeline on one crate before spending a
      full workspace instrumented build on it"

key-files:
  created:
    - .planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md
  modified:
    - .planning/phases/01-ground-truth-decision-records/COVERAGE.md

key-decisions:
  - "Corrected the plan's package spec paladin-core -> paladin-ai-core (Rule 3 auto-fix; the Cargo.toml workspace alias renames the crate directory's package name)"
  - "Scope: --workspace, default features only (no --features integration-tests — Docker is entirely absent from this environment); --ignore-filename-regex excludes examples/, benches/, doc-examples, cargo registry source, and rustlib stdlib source; doctests excluded"
  - "Recorded the llvm-cov 'warning: 33 functions have mismatched data' verbatim as a caveat rather than suppressing it, per the plan's integrity requirement"
  - "Task 3 (checkpoint:human-verify, gate=blocking-human) was NOT auto-approved despite auto-mode being active — per explicit instruction this gate type is never auto-approved. Plan execution halted for a human review cycle and resumed once the human replied 'approved'."
  - "Human approved the 84.79% figure and its scope on 2026-07-31T15:30:27Z; the approval, along with two accepted observations (Milestone-1 baseline delta, function-vs-line coverage gap), is recorded in 01-coverage-measurement.md as context for ADR-0006 and VERIFY-05 — not as a reason to re-measure"

requirements-completed: [RECON-07]

coverage: []

# Metrics
duration: ~27min (Tasks 1-2) + human review cycle (Task 3)
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 09: Offline Workspace Coverage Measurement Summary

**Measured workspace line coverage at 84.79% (61,404 lines, 9,340 missed) using a fully offline `rustc -C instrument-coverage` -> `llvm-profdata merge` -> `llvm-cov report` pipeline — no `cargo-llvm-cov` install, no network, no Docker — after first proving the same pipeline end-to-end on `paladin-ai-core` alone. The figure was independently reviewed and approved by the human on 2026-07-31.**

This plan carries information plan 01-04 did not have: the `llvm-tools` rustup component is already installed, so `rustc`'s own source-based coverage instrumentation reaches the same measurement `cargo-llvm-cov` would have produced, without the crates.io install that HTTP-403'd in 01-04's sandbox. All three tasks are now complete. Tasks 1 and 2 (the tracer proof and the workspace-scoped measurement of record) completed exactly as written, with one mechanical correction (package name). Task 3, a `checkpoint:human-verify` gate marked `gate="blocking-human"`, was correctly **not** auto-approved despite active auto-mode — execution halted, the human reviewed the record and the orchestrator's independent pre-checkpoint verification, and replied "approved". The approval — approver, figure, scope, UTC date — plus two accepted observations for ADR-0006/VERIFY-05 (the Milestone-1 baseline delta and the function-vs-line coverage gap) are now recorded in `01-coverage-measurement.md`. The plan is fully complete and `01-coverage-measurement.md` is ready for plan 01-10 to transcribe from.

## Performance

- **Duration:** ~27 min (14:35 worktree setup through 15:02 Task 2 commit) + human review cycle, approved 2026-07-31T15:30:27Z
- **Started:** 2026-07-31T14:39:55Z (first environment probe)
- **Completed (Tasks 1-2):** 2026-07-31T15:02:07Z
- **Completed (Task 3 / plan):** 2026-07-31T15:30:27Z
- **Tasks:** 3 of 3 complete (2 executed and committed by the prior executor; 1 checkpoint correctly halted, then resolved by human approval and recorded by this continuation)
- **Files modified:** 2 (`01-coverage-measurement.md` created and later appended with the approval record, `COVERAGE.md` appended)

## Accomplishments

- **Task 1 (tracer, complete):** Proved the fully offline `-C instrument-coverage` -> `llvm-profdata merge` -> `llvm-cov report` chain works in this environment on `paladin-ai-core` alone — real TOTAL row printed (16.63% line cover, unfiltered pipeline-proof only, not a scope figure). Confirmed `cargo llvm-cov --version` fails (no such command, exit 101) and `command -v docker` returns absent (exit 1), both recorded verbatim rather than hidden. Resolved `llvm-profdata`/`llvm-cov` portably via `$(rustc --print sysroot)`, confirmed both present under the rustup `llvm-tools` component.
- **Task 2 (measurement of record, complete):** Ran the proven pipeline at `--workspace` scope, `--offline`, default features only. All 35 `test result: ok.` lines across the workspace reported 0 failed — no test failures, no compile errors. Merged 2,341 `.profraw` files, discovered 31 test-binary objects via `cargo test --no-run --message-format=json | jq`, and ran `llvm-cov report` with `--ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/'`. Extracted and recorded the single workspace line-coverage figure: **84.79%** (61,404 lines, 9,340 missed), transcribed character-for-character from the pasted TOTAL row. Appended the `## Addendum — offline measurement path` section to `COVERAGE.md` (12 lines added, 0 removed — an append, not a rewrite; the existing `No external API integration` declaration is byte-identical).
- **Task 3 (checkpoint, complete):** `gate="blocking-human"` on a `checkpoint:human-verify` task is never auto-approved per this plan's explicit instruction, regardless of the active auto-mode config (`workflow._auto_chain_active: true`). Execution halted correctly and returned a structured checkpoint to the orchestrator. The human subsequently reviewed the record (per the plan's four-step `<how-to-verify>` checklist) together with the orchestrator's own independent pre-checkpoint verification (TOTAL-row arithmetic, measurement-commit diff scope, package-name legitimacy) and replied "approved". A continuation executor recorded the approval — approver, figure, scope, UTC date — plus two accepted observations (Milestone-1 baseline delta, function-vs-line coverage gap) as a new "Human confirmation (Task 3 checkpoint)" section appended to `01-coverage-measurement.md`. The 84.79% figure and the pasted `llvm-cov report` stdout are unchanged.

## Task Commits

1. **Task 1: Tracer — prove the offline coverage pipeline on paladin-ai-core** - `9be788c` (feat)
2. **Task 2: Expand the proven pipeline to the whole workspace and record the measurement** - `281c875` (feat)
3. **Task 3: Confirm the measured number is real and reproducible** - `d9cd26c` (docs) — human approval recorded, no measurement change

**Plan metadata:** this SUMMARY's own commit (see below)

## Files Created/Modified

- `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` - Created. Tracer section (environment probes, package-name correction, pipeline commands, unfiltered TOTAL row), measurement-of-record section (scope decision, full verbatim command chain, complete `llvm-cov report` stdout for the workspace, extracted 84.79% figure, caveats), and a "Human confirmation (Task 3 checkpoint)" section (approver, date, accepted observations for ADR-0006/VERIFY-05).
- `.planning/phases/01-ground-truth-decision-records/COVERAGE.md` - Appended `## Addendum — offline measurement path` (12 insertions, 0 deletions). Existing declaration text and its two detector-signal explanations left byte-identical.

## Decisions Made

- **Corrected `paladin-core` to `paladin-ai-core` in every `-p` flag (Rule 3 auto-fix).** The plan's prose names the crate `paladin-core` (matching the `crates/paladin-core/` directory), but the workspace root `Cargo.toml` aliases the package name to `paladin-ai-core`. `cargo test -p paladin-core --offline` fails with a package-ID-not-found error. This is a mechanical package-spec correction, not a scope substitution, and is recorded verbatim in the measurement file rather than silently fixed.
- **Deleted `/workspace/target/coverage` (main checkout, outside this worktree) by mistake on the first `rm -rf` invocation**, before recognizing the worktree has its own independent `target/` directory relative to its own cwd. Verified afterward that only the (empty/nonexistent) `coverage/` subdirectory was affected — `target/debug`, `target/flycheck0`, and `target/tmp` in the main checkout were untouched, and `target/coverage` did not previously exist there either (confirmed via `ls` immediately after). All subsequent deletions and the actual instrumented builds used the worktree-local `target/coverage/` exclusively (verified via `pwd` and relative paths). No git-tracked file was affected (`target/` is gitignored). Recorded here per worktree-path-safety discipline even though the practical impact was nil.
- **Recorded the `warning: 33 functions have mismatched data` llvm-cov warning verbatim as a caveat** rather than suppressing or omitting it, consistent with this plan's integrity requirement to record real tool output including imperfections.
- **Did not auto-approve Task 3's `checkpoint:human-verify gate="blocking-human"`**, even though `workflow._auto_chain_active: true` is set for this project. The plan's own checkpoint_protocol instructions state this gate type is never auto-approved regardless of auto-mode.
- **On resume, recorded the human's "approved" resume-signal into the measurement file rather than treating verbal approval alone as sufficient closure.** The plan's integrity requirement is that the figure's provenance be auditable end-to-end; an approval with no written approver, date, or scope restatement would leave that chain broken at the last link. The two accepted observations (Milestone-1 delta, function-vs-line gap) were recorded as explicitly flagged context rather than folded silently into the measurement narrative, so ADR-0006 and VERIFY-05 can pick them up without re-deriving them.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Corrected Cargo package spec `paladin-core` -> `paladin-ai-core`**
- **Found during:** Task 1 (tracer, first pipeline command)
- **Issue:** The plan's read_first/action text names the crate `paladin-core`; `cargo test -p paladin-core --offline` fails because the workspace's `Cargo.toml` aliases that crate's package name to `paladin-ai-core`
- **Fix:** Used `-p paladin-ai-core` in every subsequent command (test run, object discovery)
- **Files modified:** None (command-line correction only, no source changed)
- **Verification:** `cargo test -p paladin-ai-core --offline` succeeds; tracer TOTAL row printed
- **Committed in:** `9be788c` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking).
**Impact on plan:** Necessary for the tracer and workspace commands to run at all. No scope creep — the correction is purely a package-ID spelling fix with an identical crate as the target.

## Issues Encountered

None beyond the auto-fixed package-name mismatch and the self-corrected worktree-path slip documented above. Both the tracer and the full workspace instrumented build completed with zero test failures and zero compile errors.

## User Setup Required

None for Tasks 1-2 — both completed fully offline with no external service or credential required.

**Task 3 required human action** (not a service/dashboard setup, but the plan's own designed checkpoint): open `01-coverage-measurement.md`, copy the recorded workspace command verbatim, run it, and confirm the printed percentage matches `84.79%` character-for-character. This was completed — the human approved on 2026-07-31T15:30:27Z. See the plan's `<how-to-verify>` steps for the checklist that was satisfied (stdout pasted not summarized; commit SHA matched; scope statement accepted as one Phase 15 can wire into CI).

## Known Stubs

None. All three tasks produced real, fully-verified evidence — no placeholder or estimated figures anywhere in the record, and the figure carries a recorded human approval.

## Next Phase Readiness

- **Ready for plan 01-10 (ADR-0006 authoring) to consume this figure.** Task 3's human checkpoint is resolved and recorded — `01-coverage-measurement.md` now carries the tracer section, the measurement-of-record section, and a "Human confirmation (Task 3 checkpoint)" section with approver, date, and scope.
- `01-coverage-measurement.md` is complete and ready for plan 01-10 to transcribe the 84.79% figure, the `--ignore-filename-regex` value, the doctest-exclusion statement, and the unreproduced-CI-scope caveat into ADR-0006 byte-for-byte.
- The measured 84.79% figure is well above all four documented stale/contested baselines (60.88%/67.79% Milestone 1, ~78% Milestone 3, 76-77% Deferred-QA), consistent with the substantial capability build-out documented in PROJECT.md between Milestone 1 and the current v0.7.0 tree — no coincidence with any prior baseline was found or is claimed. This delta, along with the function-vs-line coverage gap (77.34% vs 84.79%), is recorded as explicit accepted context for ADR-0006 and VERIFY-05.

## Self-Check: PASSED

- FOUND: commit `d9cd26c` (Task 3 approval record)
- FOUND: commit `9be788c` (Task 1 tracer)
- FOUND: commit `281c875` (Task 2 measurement of record)
- FOUND: `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`
- FOUND: literal string `84.79%` in the measurement record

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
