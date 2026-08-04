---
phase: 01-ground-truth-decision-records
plan: 04
subsystem: testing
tags: [coverage, llvm-cov, adr, superseded, halted-precondition]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records (plan 01-01)
    provides: "The decision-record substrate (decisions/ + PROMOTION.md numbering index) this plan's ADR-0006 would have been authored into"
provides:
  - "Nothing. This plan halted at its own Task 1 precondition and shipped no artifact. Its entire scope was delivered by gap-closure plans 01-09, 01-10 and 01-12."
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified: []

key-decisions:
  - "Halted rather than substituted: Task 1's precondition required a runnable `cargo llvm-cov`, and explicitly instructed HALT-and-report if the install could not complete — 'do not substitute an estimated or previously-documented figure'. The environment could not install it (crates.io HTTP 403, no Docker), so the plan stopped instead of recording a seventh unverified coverage number, which is precisely the failure mode D-07 and RECON-07 exist to prevent."
  - "Superseded, not abandoned: the same requirement (RECON-07) was re-planned as gap-closure plans 01-09 and 01-10, which reached the number via an offline measurement path this plan did not have available."
  - "Closed out as superseded rather than re-executed: re-running this plan today would measure a different tree and overwrite ADR-0006 and 01-coverage-measurement.md, which ROADMAP.md Phase 3, REQUIREMENTS.md (both RECON-07 surfaces) and PROJECT.md now cite as authoritative."

requirements-completed: []  # RECON-07 was NOT satisfied by this plan. It was satisfied by 01-09 (measurement), 01-10 (ADR-0006) and 01-12 (REQUIREMENTS.md flip).

coverage: []  # No deliverables — this plan shipped no artifact.

# Metrics
duration: n/a (halted at Task 1 precondition)
completed: 2026-08-03
status: superseded
superseded_by: ["01-09", "01-10"]
---

# Phase 01 Plan 04: Coverage Measurement & Gate ADR — Superseded

**Halted at its own Task 1 precondition (`cargo-llvm-cov` uninstallable — crates.io HTTP 403, no Docker) and superseded by gap-closure plans 01-09 and 01-10, which reached the same result through an offline measurement path.**

## Performance

- **Duration:** n/a — no task executed to completion
- **Tasks:** 0 of 3 completed
- **Files modified:** 0
- **Commits:** 0 (no `01-04`-scoped commit exists in history)

## Accomplishments

None. This plan produced no artifact and no commit.

Its scope — one measured coverage number, one recorded scope, one recorded gate — was delivered
in full by the gap-closure plans listed under **Supersession** below.

## Task Commits

None. `git log --all --grep="(01-04)"` returns zero commits.

| Task | Type | Outcome |
|------|------|---------|
| Task 1: Measure workspace coverage and record raw evidence | auto | ✗ Halted at precondition |
| Task 2: ADR-0006 — one number, one scope, one gate, one target, one ratchet | auto | ○ Never reached (depends on Task 1's figure) |
| Task 3: Confirm the number is reproducible and the gate acceptable | checkpoint:human-verify (blocking) | ○ Never reached |

## Why it halted

Task 1 carried an explicit precondition:

> `cargo llvm-cov` is runnable in this environment — either already installed, or
> `cargo install cargo-llvm-cov` can reach crates.io. … **If the install cannot complete, HALT and
> report the blocker — do not substitute an estimated or previously-documented figure.**

The plan's `user_setup` block recorded the same constraint up front, asking for "an execution
environment that can reach crates.io, OR … the existing `.github/workflows/integration-tests.yml`
run and supply its coverage output."

Neither was available: `cargo-llvm-cov` was absent, crates.io returned HTTP 403, and no Docker was
available to run the CI-equivalent scope. The plan halted as instructed.

**This was the correct outcome, not a failure.** The plan's own prohibitions forbid recording a
figure not produced by running the recorded command against the current tree — "no estimate, no
interpolation, and no figure copied from a stale or contested prior baseline. If the measurement
cannot be run, the task halts and reports the blocker rather than producing a seventh unverified
number." Six stale or contested baselines already existed; the phase exists to end that, not extend it.

## Supersession

RECON-07 was re-planned as gap-closure plans that did not depend on registry access:

| Plan | Delivered | Commits |
|------|-----------|---------|
| **01-09** | `01-coverage-measurement.md` — 84.79% workspace line coverage measured offline, with full command, flag set, toolchain versions (`rustc 1.97.1`, LLVM 22.1.6), commit `9be788c`, date, and raw pasted `llvm-cov report` stdout; human-approved at a `blocking-human` checkpoint | `9be788c`, `281c875`, `d9cd26c`, `e6bda8c`, `21f690b`, `799c53f` |
| **01-10** | `.planning/decisions/0006-coverage-gate.md` — ADR-0006, the single recorded gate: 84% hard-fail floor (84.79% truncated down), workspace default-feature scope, named ratchet trigger, Herald ≥ 95% / autonomous ≥ 90% preserved and handed to VERIFY-05 | `2635601`, `480b553` |
| **01-12** | Downstream wiring — PROJECT.md, ROADMAP.md Phase 3 criterion 1, REQUIREMENTS.md's two RECON-07 surfaces, and the ledger scope note all pointed at ADR-0006 | (see 01-12-SUMMARY.md) |

Both files this plan declared in `files_modified` therefore exist, authored by 01-09 and 01-10:

- `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`
- `.planning/decisions/0006-coverage-gate.md`

## Verification standing

[01-VERIFICATION.md](./01-VERIFICATION.md) (`status: passed`, 5/5 success criteria, re-verified
2026-07-31T16:46:51Z after gap closure) records this supersession explicitly:

> Plan 01-04 remains formally unexecuted (no SUMMARY) but is honestly annotated at
> `ROADMAP.md:177` as superseded by plans 01-09/01-10, which used an offline measurement path
> 01-04 did not have available. **This is a disclosed supersession, not a silently dropped plan,
> and does not constitute a gap.**

Its requirements-coverage table reads: `RECON-07 | 01-09, 01-10, 01-12 (supersedes halted 01-04) |
Coverage gate ADR | ✓ SATISFIED`.

## Decisions Made

See `key-decisions` frontmatter. In short: halt over fabricate; re-plan over force; and close out
as superseded over re-execute, because re-execution today would measure a different tree and
overwrite the verified artifacts that Phase 3, Phase 5 and Phase 15 are authored against.

## Deviations from Plan

The plan was not executed, so there are no execution deviations. The deviation is the plan's
disposition itself: superseded rather than completed, recorded here and at `ROADMAP.md:177`.

## Issues Encountered

One, and it is the reason for this document: `cargo-llvm-cov` could not be installed in the
execution environment (crates.io HTTP 403, no Docker fallback). Resolved by re-planning the
requirement onto an offline measurement path in plan 01-09.

## User Setup Required

None — the `user_setup` request this plan carried (crates.io access, or CI coverage output) is
moot. Plan 01-09 reached the figure without either.

## Next Phase Readiness

No impact. RECON-07 is satisfied, ADR-0006 is the single binding coverage record, and Phase 1
passed verification on 2026-07-31. Phases 2, 3 and 4 have since executed against ADR-0006's 84%
floor.

---
*Phase: 01-ground-truth-decision-records*
*Closed out as superseded: 2026-08-03*
