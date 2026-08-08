---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 03
subsystem: docs
tags: [adr, version-trajectory, milestone-numbering, record-writing, ground-truth]

# Dependency graph
requires:
  - phase: 04-release-coherence
    provides: "ADR-0008 (workspace version converges on 0.7.0), REL-01 closed"
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: "ADR-0010, ADR-0014 (milestone/epic numbering precedents), milestone-04-06.md ledger"
provides:
  - "ADR-0029: v0.1.0-rc.1 version trajectory recorded as closed history with an extensible Trajectory table"
  - "ADR-0030: fourth milestone-numbering collision closed, citing ADR-0010 and ADR-0014"
  - "Dated supersession banner + inline corrections on the Milestone 7 overview's self-title and Prerequisites"
affects: [13-milestone-9-12-deferred-qa-close-out]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Shape B compact blockquote banner for document-level supersession", "Shape A inline strike-and-correct for clause-level fixes, applied here to a section heading's parenthetical rather than a full clause"]

key-files:
  created:
    - .planning/decisions/0029-version-trajectory-history.md
    - .planning/decisions/0030-milestone-7-self-numbering.md
  modified:
    - .project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md

key-decisions:
  - "ADR-0029 records v0.1.0-rc.1 as closed history and confirms REL-01 already converged on 0.7.0 (ADR-0008) without re-opening it"
  - "ADR-0029's Trajectory table carries a labelled placeholder row naming Phase 13 / ORCH-05 as owner of v0.3.0-v0.6.0, rather than leaving a gap"
  - "ADR-0030 cites ADR-0010 before ADR-0014 (chronological precedent order) and names each of the six mis-credited Prerequisites items individually with its own milestone-04-06.md citation"
  - "The M7 overview's self-title is left byte-unchanged as evidence; only the Prerequisites heading's parenthetical is struck, with inline Corrected clauses appended as new lines (not in-place edits) so the original bullet wording survives verbatim"

patterns-established:
  - "Pattern: when a heading-level mis-credit needs strike-and-correct but the original bullet items must survive verbatim, append the correction as a new indented line under each bullet rather than editing the bullet text in place — keeps the diff to insertions only and satisfies both 'inline' and 'retain every original word'"

requirements-completed: [HARD-03, HARD-04]

coverage:
  - id: D1
    description: "ADR-0029 records the v0.1.0-rc.1 trajectory as closed history with a Trajectory table"
    requirement: "HARD-03"
    verification:
      - kind: other
        ref: "grep/diff checks against .planning/decisions/0029-version-trajectory-history.md (heading order, Trajectory table presence, REL-01/ADR-0008/a9530fc citations, conforms verdict) — all passed during task execution"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0030 records the fourth milestone-numbering collision, citing ADR-0010 and ADR-0014 in order"
    requirement: "HARD-04"
    verification:
      - kind: other
        ref: "grep/diff checks against .planning/decisions/0030-milestone-7-self-numbering.md (heading order, ADR-0010 precedes ADR-0014, six milestone-04-06.md citations, fifth-instance closure, conforms verdict) — all passed during task execution"
        status: pass
    human_judgment: false
  - id: D3
    description: "M7 overview annotated with a dated banner and inline Prerequisites corrections, original text retained"
    requirement: "HARD-04"
    verification:
      - kind: other
        ref: "grep checks against the overview file (ADR-0030 references, Corrected (dated 2026-08-08, HARD-04) count, title byte-unchanged, numstat deletions <=2) — all passed; see Deviations for the one criterion that could not literally pass"
        status: pass
    human_judgment: true
    rationale: "The acceptance criterion 'grep -c SUPERSEDED BY returns exactly 1' cannot be literally satisfied because the file already carries an unrelated pre-existing Shape B banner from Phase 9 (ADR-0025, licence AC1). A human should confirm the two banners are correctly independent and neither destroys the other's evidence."

duration: ~25min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 03: Version Trajectory and Milestone 7 Self-Numbering Summary

**ADR-0029 records `v0.1.0-rc.1` as closed history with an extensible Trajectory table; ADR-0030 closes the fourth milestone-numbering collision citing ADR-0010 and ADR-0014; the Milestone 7 overview gets a dated supersession banner plus six inline Prerequisites corrections.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-08-08
- **Tasks:** 3
- **Files modified:** 3 (2 created, 1 annotated)

## Accomplishments

- Wrote `.planning/decisions/0029-version-trajectory-history.md`: records the lockstep `0.2.0` target (targeted, not shipped), the real `v0.1.0-rc.1` release at commit `a9530fc` (2026-05-28, GO sign-off, docs.rs verification of all ten crates, external smoke-project compile), and the current `0.7.0`/`v0.7.0`/`v0.7.1` state — all re-measured this session, not transcribed from stale context. Confirms REL-01 (`REQUIREMENTS.md:358`, `:3913`) already converged on `0.7.0` via ADR-0008 and is not re-opened. Carries a `## Trajectory` table with an ascending, no-blank-cell row set including a labelled placeholder row naming Phase 13 / ORCH-05 as owner of `v0.3.0` through `v0.6.0`.
- Wrote `.planning/decisions/0030-milestone-7-self-numbering.md`: records the M7 overview's self-title collision ("Milestone 4" vs. its `Milestone_7-Production-Hardening` path) and the Prerequisites mis-credit as two separately stated defects. Names each of the six mis-credited items individually with its own `milestone-04-06.md` `file:line` citation, cites ADR-0010 before ADR-0014 (chronological precedent order), and records the Roadmap Extension Protocol's predicted fifth numbering-collision instance as discharged by this fourth instance.
- Annotated `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md`: added a Shape B banner beneath the title naming ADR-0030 and its two precedents (title line itself left byte-unchanged as evidence); struck only the Prerequisites heading's parenthetical and appended a dated inline `**Corrected:**` clause to each of the six mapped bullets.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write ADR-0029** - `632160c` (docs)
2. **Task 2: Write ADR-0030** - `3773494` (docs)
3. **Task 3: Annotate the Milestone 7 overview** - `555d7e9` (docs)

_This is a worktree-mode execution; the plan-completion metadata commit (SUMMARY.md) is a separate, subsequent commit per the orchestrator's convention._

## Files Created/Modified

- `.planning/decisions/0029-version-trajectory-history.md` - New ADR: version trajectory as closed history, with a `## Trajectory` table Phase 13/ORCH-05 extends
- `.planning/decisions/0030-milestone-7-self-numbering.md` - New ADR: fourth milestone-numbering collision, citing ADR-0010 and ADR-0014
- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md` - Annotated: dated Shape B banner beneath title; Prerequisites heading parenthetical struck; six bullets get inline `**Corrected:**` clauses

## Decisions Made

- **Correction placement for the Prerequisites bullets:** the plan calls for an "inline" `**Corrected:**` clause per mis-credited bullet while also requiring every original word retained and (per the overall `<acceptance_criteria>`) at most 2 deleted lines in `git diff --numstat`. Editing each bullet's own line in place would have produced one deletion per bullet (6+), exceeding that bound. Resolved by appending each correction as a new indented line directly beneath its bullet rather than rewriting the bullet's own line — this keeps the original bullet text byte-identical, produces only 1 deletion total (the heading's parenthetical), and still reads as "inline" (immediately following the item it corrects, inside the same list entry).
- **Bullet inventory found under Prerequisites — 8 bullets, not 6.** The section holds: (1) feature flags/CI matrix, (2) core workspace crates, (3) facade crate with backward-compatible re-exports, (4) `application_settings.rs` decomposition, (5) manager-service relocation, (6) Maneuver DSL co-location, (7) `CircuitBreaker` relocation, (8) architecture compliance report. Six of these eight (items 1, 2, 4, 5, 6, 7) map one-to-one onto the six items ADR-0030/HARD-04 name; items 3 (facade re-exports) and 8 (architecture compliance report) do **not** appear in HARD-04's enumerated list and were left unannotated per the plan's explicit instruction not to invent a mapping to reach six.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 — acceptance-criterion assumption did not hold against the current file state] The overview already carried an unrelated pre-existing supersession banner**
- **Found during:** Task 3 (annotating the M7 overview)
- **Issue:** The task's acceptance criteria state `grep -c 'SUPERSEDED BY' <file>` must return exactly `1`. The overview file already contains an unrelated Shape B banner from Phase 9 plan `09-05` (commit `74a05fe`) — `> **AC 1 SUPERSEDED BY [ADR-0025](...)** — 2026-08-08.` — for a completely different concern (the licence Acceptance Criterion, ADR-0025). Adding this task's own banner (for the title/Prerequisites concern, ADR-0030) necessarily brings the file's total `SUPERSEDED BY` count to 2, not 1. Deleting or altering the pre-existing ADR-0025 banner to force the count to 1 would violate D-00c (`.project/` annotation is additive, never destructive) and would destroy Phase 9's legitimate evidence.
- **Fix:** Kept the pre-existing ADR-0025 banner untouched and added this task's ADR-0030 banner as a second, independent blockquote. The substantive requirement — "one dated banner naming ADR-0030 and its two precedents, immediately beneath the title" — is met exactly once; the literal `grep -c 'SUPERSEDED BY'` count is 2 because it also matches the unrelated pre-existing banner, which is a fact about the corpus's annotation history, not a defect in this task's execution.
- **Files modified:** `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md`
- **Verification:** `grep -n 'SUPERSEDED BY' <file>` shows exactly two lines: line 3 (this task's new ADR-0030 banner) and line 413 (Phase 9's pre-existing ADR-0025 banner, confirmed via `git log --oneline -- <file>` → `74a05fe docs(09-05): write ADR-0025 and annotate the licence source documents`). `grep -c 'ADR-0030'` returns 3 (banner + Prerequisites paragraph + one further mention), satisfying the ">=2" bound the acceptance criteria actually need for this ADR's own coverage.
- **Committed in:** `555d7e9` (Task 3 commit)

---

**Total deviations:** 1 auto-documented (acceptance-criterion assumption conflicting with pre-existing corpus state)
**Impact on plan:** No scope creep, no code change. The banner-count criterion's literal number cannot be met without violating D-00c; the substantive per-ADR requirement (this task's own banner appears once, naming ADR-0030 and its precedents) is met and independently verified.

## Issues Encountered

None beyond the deviation above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 13 / ORCH-05 has a named, labelled placeholder row in ADR-0029's `## Trajectory` table to append `v0.3.0` through `v0.6.0` into, and inherits the note that REL-01 is already converged.
- Phase 10 / HARD-01 (the ledger, written by sibling plans in this wave/other waves) can cite ADR-0030 for the M7 epic sections that use directory numbering, and ADR-0029 for the `REQ-versioning-policy` / `REQ-release-readiness-audit` / `REQ-changelog-v020-cut` rows.
- No blockers. `git status --porcelain -- '*.rs'` is empty — no `.rs` file was touched by this plan, matching D-23's boundary.

## Self-Check: PASSED

All claimed files and commits verified present:
- `.planning/decisions/0029-version-trajectory-history.md` — FOUND
- `.planning/decisions/0030-milestone-7-self-numbering.md` — FOUND
- `.project/Milestone_7-Production-Hardening/overview/Milestone_7-Tier_4-Production_Hardening.md` — FOUND
- `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-03-SUMMARY.md` — FOUND
- Commit `632160c` — FOUND in git log
- Commit `3773494` — FOUND in git log
- Commit `555d7e9` — FOUND in git log

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
