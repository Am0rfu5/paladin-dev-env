---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 05
subsystem: infra
tags: [codeql, sast, ci, security-tooling]

# Dependency graph
requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql (plan 03/04)
    provides: "The settled SAST-01 evaluation verdict (`disqualified`, version-scoped to CodeQL 2.26.3 / rust-queries 0.1.40) recorded in 18-CODEQL-EVIDENCE.md's `## Verdict` section, and the governed dismissal register (`CODEQL-DISMISSALS.md`) from 18-04"
provides:
  - "An honest, non-fabricated record that the D-14/D-15 observation-window measurement is not applicable given the disqualified verdict"
  - "Explicit closure of plan 18-05 without performing backfill pushes, live-window recording, or alert triage"
affects: [18-06]

# Tech tracking
tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md

key-decisions:
  - "The plan's Task 1 precondition (`## Verdict` records `qualified` or `qualified-with-coverage-gap`) is unmet by the settled `disqualified` verdict — this was recognized and, by explicit user decision at the 18-03 verdict checkpoint (2026-08-25), recorded as not-applicable rather than executed, fabricated, or escalated as a blocking checkpoint."
  - "No observation-window data (backfill table, live advisory period, D-15 metrics) was produced. Producing plausible-looking numbers for a required check that will never exist (18-06's promotion path is not taken per the Verdict) would be dishonest scope creep, not diligence."

requirements-completed: []  # SAST-03 is NOT satisfied by this plan; the measurement it required is moot given the disqualified verdict, not performed under a substitute method.

coverage: []

# Metrics
duration: 5min
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 05: Observation Window (Not Applicable) Summary

**Recorded plan 18-05 as resolved/not-applicable — the observation-window measurement was never run because SAST-01's settled verdict is `disqualified` and CodeQL is not being promoted, so there is no required check to baseline.**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-08-25T22:30:00Z
- **Completed:** 2026-08-25T22:33:54Z
- **Tasks:** 0 of 2 plan tasks executed (both skipped by design — see Deviations)
- **Files modified:** 1

## Accomplishments
- Verified 18-05's Task 1 precondition (`18-CODEQL-EVIDENCE.md`'s `## Verdict` records `qualified` or `qualified-with-coverage-gap`) against the actual recorded verdict and confirmed it is unmet: the verdict is `disqualified` (version-scoped: CodeQL 2.26.3 / rust-queries 0.1.40), with `codeql.yml` retained advisory-only and not promoted.
- Appended an `## Observation Window` / `### Not Applicable — Promotion Not Pursued` section to `18-CODEQL-EVIDENCE.md` recording, honestly and without fabricated data, why the backfill table, live advisory period, and D-15 metric block this plan would have produced were not performed.
- Did not push any `tmp/codeql-backfill-*` branches, did not dispatch any CodeQL run, did not add rows to `CODEQL-DISMISSALS.md`, and did not touch `STATE.md` or `ROADMAP.md`.

## Task Commits

Neither of the plan's two `<task type="auto">` tasks (Task 1: backfill; Task 2: triage and D-15 metrics) was executed — both require the unmet precondition. This plan was instead handled as a single documentation-only recording task per explicit user authorization (see Deviations below).

1. **Evidence-doc recording** - see commit below (docs)

**Plan metadata:** commit below (docs: record 18-05 not-applicable)

## Files Created/Modified
- `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` - Added `## Observation Window` → `### Not Applicable — Promotion Not Pursued` section explaining why the D-14/D-15 measurement is moot given the disqualified SAST-01 verdict, cross-referencing `## Verdict` and `## Promotion Status`.

## Decisions Made
- The human operator, at the 18-03 verdict checkpoint (2026-08-25), decided to record 18-05 as not-applicable rather than have it halt as a blocking-human checkpoint on the unmet precondition, and rather than proceed with a measurement whose purpose (baselining a required check) no longer applies now that CodeQL is disqualified and not promoted.
- No substitute measurement or scaled-down version of the observation window was attempted — an honest "not applicable, and why" was judged the correct record, not a partial or synthetic substitute for the real thing.

## Deviations from Plan

### Plan Scope Not Executed (user-authorized, not an executor shortcut)

**1. [User decision — plan precondition unmet by design] Observation-window measurement (Tasks 1 and 2) not performed**
- **Found during:** Precondition check before Task 1
- **Issue:** Task 1's `<precondition>` requires `18-CODEQL-EVIDENCE.md`'s `## Verdict` to record `qualified` or `qualified-with-coverage-gap`. The actual recorded verdict (settled in an earlier plan, prior to this execution) is `disqualified`, with `codeql.yml` retained advisory-only and 18-06's promotion path explicitly not taken. This plan's whole purpose — producing noise/latency numbers to pin a required check on — is therefore moot: there is no required check to baseline.
- **Handling:** Per explicit user authorization (not an executor-invented shortcut), this was recorded as RESOLVED — NOT APPLICABLE rather than executed, fabricated, or escalated as a `checkpoint:human-verify`. No backfill branches were pushed, no CodeQL runs were dispatched, no alerts were triaged, and no rows were added to `CODEQL-DISMISSALS.md`.
- **Files modified:** `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` (added `## Observation Window` section documenting the non-applicability, honestly, with no fabricated backfill rows, live-window data, or metrics).
- **Verification:** Manual review of `## Verdict` (lines 520-643) and `## Promotion Status` (lines 1185-1192) of `18-CODEQL-EVIDENCE.md` confirms the disqualified/not-promoted status this determination rests on.
- **Committed in:** see commit hash below.

---

**Total deviations:** 1 (plan-scope skip, user-authorized)
**Impact on plan:** SAST-03 is not satisfied by this plan — the requirement's measurement was never produced, because producing it would have been moot busywork against a scanner that is not being promoted. This is recorded honestly rather than marked complete.

## Issues Encountered
None - the precondition check worked exactly as the plan intended (it exists precisely to prevent this measurement from running against a disqualified/unpromoted scanner); the only judgment call was how to record the resulting non-execution, which was made by the user rather than inferred.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- 18-06 already accounts for this outcome in its "Downstream note" (see `18-CODEQL-EVIDENCE.md`'s `## Verdict`): no ruleset write, no `branch-protection.md` count update, no new required context — 18-06's remaining scope is limited to correcting the ruleset re-application procedure documentation, not promotion.
- No blockers introduced by this plan's non-execution; the evidence document is internally consistent (Verdict, Promotion Status, and the new Observation Window section all agree on `disqualified`/advisory-only/not-promoted).

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*
