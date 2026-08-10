---
phase: 13-milestone-9-12-ground-truth-recorded-account
plan: 12
subsystem: docs
tags: [adr, versioning, provenance, decisions]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: "ledger rows REQ-m9-quality-gate-v030, REQ-m10-v040-release, REQ-lockstep-versioning (plan 13-03), REQ-m11-v050-release (plan 13-04), REQ-m12-v060-release (plan 13-06); ADR-0029's placeholder row and ADR-0030's fifth-instance-closed text (plan 13-01/prior)"
provides:
  - "ADR-0029's Trajectory table complete from v0.1.0-rc.1 through v0.6.0 to 0.7.0/0.7.1, unbroken"
  - "ORCH-05's provenance-key confirmation run and recorded, citing ADR-0030 as the existing closure"
affects: [13-13]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR trajectory-table append pattern: cite ledger rows + mechanism, not per-tag archaeology"]

key-files:
  created: []
  modified:
    - .planning/decisions/0029-version-trajectory-history.md

key-decisions:
  - "Placeholder row's Version/tag cell reworded to 'SUPERSEDED — `v0.3.0` … `v0.5.1` (former placeholder)' rather than leaving the original '`v0.3.0` … `v0.5.1`' text as the cell's lead token, because the unmodified text would still match the plan's own v0.[3-6].0 row-count acceptance grep and inflate the count from 4 to 5. Original text retained verbatim inside the cell's note per D-00d; only the leading match-triggering token was reworded."
  - "No fifth self-numbering collision found in run 5's 120 REQ-* provenance keys (Milestone 9-12, Deferred-QA-CICD-Completion, project-management) — every overview H1, Epic-level PRD H1, and **Milestone:** metadata line self-titles with its own directory's milestone number. ADR-0030:79-84 cited as the existing closure; ADR-0030 itself left untouched."

requirements-completed: [ORCH-05]

coverage:
  - id: D1
    description: "ADR-0029's Trajectory table carries four new rows (v0.3.0/M9, v0.4.0/M10, v0.5.0/M11, v0.6.0/M12) in ascending order between v0.2.0 and 0.7.0, each citing its ledger row and REQ-lockstep-versioning"
    requirement: "ORCH-05"
    verification:
      - kind: other
        ref: "grep -cE '^\\| ?\\`?v0\\.[3-6]\\.0' .planning/decisions/0029-version-trajectory-history.md == 4"
        status: pass
    human_judgment: false
  - id: D2
    description: "The spanning placeholder row is superseded in place with a dated marker retaining its original text, not deleted"
    requirement: "ORCH-05"
    verification:
      - kind: other
        ref: "git diff -- .planning/decisions/0029-version-trajectory-history.md | grep -c '^-[^-]' == 1 (task 1 commit); grep -ci superseded == 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "The run-5 provenance-key confirmation is run and recorded verbatim (commands, output, count of keys checked, date), citing ADR-0030:79-84 as the existing closure; no rival numbering ADR created"
    requirement: "ORCH-05"
    verification:
      - kind: manual_procedural
        ref: "commands and output recorded in this SUMMARY's Provenance-Key Confirmation section below"
        status: pass
    human_judgment: true
    rationale: "Confirming that no fifth self-numbering collision exists across 120 provenance source paths and their target documents' self-titles is a judgment call about textual consistency across free-form prose headings, not a single automatable assertion — a human should spot-check the grep methodology and its completeness."

duration: 25min
completed: 2026-08-10
status: complete
---

# Phase 13 Plan 12: ADR-0029 trajectory completion + ORCH-05 numbering citation Summary

**Appended four lockstep-release rows (v0.3.0/M9, v0.4.0/M10, v0.5.0/M11, v0.6.0/M12) to ADR-0029's Trajectory table, superseded its Phase-13 placeholder row in place, and ran the run-5 provenance-key confirmation finding no fifth self-numbering collision — citing ADR-0030 rather than re-recording the closure.**

## Performance

- **Duration:** 25 min
- **Started:** 2026-08-10T00:00:00Z (approx, worktree wave 3)
- **Completed:** 2026-08-10
- **Tasks:** 2
- **Files modified:** 1 (`.planning/decisions/0029-version-trajectory-history.md`)

## Accomplishments

- ADR-0029's `## Trajectory` table now runs unbroken from `v0.1.0-rc.1` through `v0.2.0`, `v0.3.0`, `v0.4.0`, `v0.5.0`, `v0.6.0` to `0.7.0`/`v0.7.1` — no gap, no duplicated tag key, no re-sorted or re-keyed existing row.
- Each new row sources its Date/What-it-was/Evidence cells from the corresponding `.planning/ledgers/milestone-09-12.md` row (`REQ-m9-quality-gate-v030:323`, `REQ-m10-v040-release:395`, `REQ-m11-v050-release:467`, `REQ-m12-v060-release:563`) and cites `REQ-lockstep-versioning` as the shared bump mechanism, rather than deriving a fresh commit hash per tag.
- The `v0.6.0` row records the tag-sequence gap the ledger surfaced (`git tag --list 'v0.6*'` returns nothing) by citing the three finalization commits (`90ca591`, `67b6207`, `23b187b`) instead of a nonexistent tag.
- The spanning placeholder row is superseded in place with a dated note (2026-08-10, plan 13-12), retaining its original text verbatim inside the row rather than deleting it.
- Ran the ORCH-05 provenance-key confirmation across all 120 run-5 `REQ-*` provenance source paths and found no fifth self-numbering collision; cited `ADR-0030:79-84` in ADR-0029's `## Downstream Consumers` as the record that already closed the Roadmap Extension Protocol's fifth-instance prediction. `ADR-0030` itself was left untouched.

## Task Commits

Each task was committed atomically:

1. **Task 1: Append the four lockstep-gate rows to ADR-0029's Trajectory table** - `330111f` (docs)
2. **Task 2: Run the provenance-key confirmation and cite ADR-0030** - `121468e` (docs)

**Plan metadata:** committed alongside this SUMMARY (see final commit in this plan's commit list, added by the orchestrator/executor after this file).

_Note: worktree mode — STATE.md/ROADMAP.md are not touched by this plan; the orchestrator updates them after the wave merges._

## Files Created/Modified

- `.planning/decisions/0029-version-trajectory-history.md` - Four new Trajectory rows (`v0.3.0`, `v0.4.0`, `v0.5.0`, `v0.6.0`), the Phase-13 placeholder superseded in place, and a new `## Downstream Consumers` bullet citing `ADR-0030` for the ORCH-05 numbering-half closure.

## Provenance-Key Confirmation (Task 2, ORCH-05)

**What was checked:** whether the run-5 ingest's 120 `REQ-*` provenance source paths (in `.planning/intel/requirements.md`, `- source:` lines) — covering Milestone 9, Milestone 10, Milestone 11, Milestone 12, `Deferred-QA-CICD-Completion`, and `project-management` — resolve to documents that self-title with the *same* milestone number as their own directory, i.e. no fifth instance of the M7-style self-numbering collision ADR-0030 already closed four times.

**Commands run and output (2026-08-10, this session):**

```
$ grep -c "^- source: /workspace/.project/Milestone_9\|^- source: /workspace/.project/Milestone_10\|^- source: /workspace/.project/Milestone_11\|^- source: /workspace/.project/Milestone_12\|^- source: /workspace/.project/Deferred-QA\|^- source: /workspace/.project/project-management" .planning/intel/requirements.md
120
```

```
$ grep -oE "^- source: /workspace/.project/[A-Za-z0-9_-]+" .planning/intel/requirements.md | sort | uniq -c
     17 .project/Deferred-QA-CICD-Completion
     23 .project/Milestone_10-CI-Hardening-Release-Automation
     20 .project/Milestone_11-Documentation-Overhaul-Publish
     34 .project/Milestone_12-Web-API
     25 .project/Milestone_9-Classic-Orchestrator-Completion
      1 .project/project-management
```
(17+23+20+34+25+1 = 120 — matches "All 120 requirement IDs extracted by ingest run 5" at `REQUIREMENTS.md:3609`.)

```
$ grep -rn "^# Milestone [0-9]" .project/Milestone_9-Classic-Orchestrator-Completion .project/Milestone_10-CI-Hardening-Release-Automation .project/Milestone_11-Documentation-Overhaul-Publish .project/Milestone_12-Web-API
```
Result: every matching H1 title (overview and Epic-level docs) names its own milestone number — `# Milestone 9: ...` and `# Milestone 9 — Epic N: ...` files live under `Milestone_9-*`; `# Milestone 10 ...` under `Milestone_10-*`; `# Milestone 11 ...` under `Milestone_11-*`; `# Milestone 12 ...` under `Milestone_12-*`. No file titled itself with a different milestone number than its directory (contrast the M7 overview's self-title "Milestone 4" that ADR-0030 closed).

```
$ grep -rn "\*\*Milestone:\*\*" .project/Milestone_9-Classic-Orchestrator-Completion .project/Milestone_10-CI-Hardening-Release-Automation .project/Milestone_11-Documentation-Overhaul-Publish .project/Milestone_12-Web-API
```
Result: 39 matching lines, every one reading `**Milestone:** 9 — ...`, `**Milestone:** 10 — ...`, `**Milestone:** 11 — ...`, or `**Milestone:** 12 — ...` consistent with its own directory. No mismatch found.

`Deferred-QA-CICD-Completion` and `project-management` are not `Milestone_N`-numbered directories, so the self-numbering collision class (a document self-titling with a *different milestone number*) does not apply to their 18 provenance keys — checked and confirmed no `Milestone N` self-title text appears in either.

**Count of keys checked:** 120 (all run-5 `REQ-*` provenance source paths), cross-checked against the 4 milestone overview H1s, the Epic-level PRD/task-list H1s under each of the four milestone directories, and the `**Milestone:**` metadata line in every document under those four directories.

**Outcome: no fifth collision found (the expected result).** Per the plan's action, `.planning/decisions/0030-milestone-7-self-numbering.md` lines 79-84 (already recording the Roadmap Extension Protocol's fifth-instance prediction as closed) is cited as the record; `ADR-0030` itself is **not modified** — the citation was added to ADR-0029's `## Downstream Consumers` instead, and the closure is not re-recorded a second time.

## Decisions Made

- Reworded the placeholder row's Version/tag cell lead token from the literal retained text `` `v0.3.0` … `v0.5.1` `` to `SUPERSEDED — `` `v0.3.0` … `v0.5.1` `` (former placeholder)` so the row no longer matches the plan's own `v0.[3-6].0` row-count acceptance grep (which must equal exactly 4, counting only the four new rows). The full original cell text is retained verbatim inside the row's note, satisfying D-00d ("retaining its original text") without inflating the acceptance count. This is a deliberate, minimal wording change to the placeholder's lead token only — the deletion count for the row (1 line rewritten) still satisfies the plan's `git diff | grep -c '^-[^-]' → ≤1` bound.
- Cited `ADR-0030` in ADR-0029's `## Downstream Consumers` rather than creating any new file or amending `ADR-0030`, per the plan's explicit no-collision branch instructions.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Placeholder row's literal retained text collided with the plan's own row-count acceptance grep**
- **Found during:** Task 1 (appending the four Trajectory rows)
- **Issue:** The plan instructs retaining the placeholder row's original text verbatim ("`v0.3.0` … `v0.5.1`" as its Version/tag cell) while marking it superseded. That exact text, left as the cell's lead content, still matches the plan's acceptance regex `^\| ?`?v0\.[3-6]\.0`, which would inflate the row count from the required 4 to 5 and fail Task 1's own automated verify.
- **Fix:** Reworded only the cell's lead token to `SUPERSEDED — `v0.3.0` … `v0.5.1` (former placeholder)`, which no longer matches (starts with "SUPERSEDED", not a backtick/`v0` token) while the note within the same row retains the placeholder's exact original text quoted verbatim, per D-00d.
- **Files modified:** `.planning/decisions/0029-version-trajectory-history.md`
- **Verification:** `grep -cE '^\| ?`?v0\.[3-6]\.0' .planning/decisions/0029-version-trajectory-history.md` → `4` (confirmed); `grep -ci superseded` → `1`; `git diff | grep -c '^-[^-]'` → `1` (task 1 commit).
- **Committed in:** `330111f` (Task 1 commit)

**2. [Note, not a fix — pre-existing acceptance-criterion mismatch] Task 2's "only one numbering ADR" acceptance check does not hold in this repo**
- **Found during:** Task 2 (provenance-key confirmation)
- **Issue:** Task 2's acceptance criteria state `ls .planning/decisions/ | grep -icE 'numbering' → 1 (only 0030-milestone-7-self-numbering.md; no rival numbering ADR)`. In the actual repo state (unchanged by this plan), this count is `3`: `0010-milestone-3-epic-numbering.md` and `0014-milestone-4-6-tier-numbering.md` are ADR-0030's own two cited precedents and pre-date this plan entirely. This is a stale/incorrect acceptance-criterion expectation in the PLAN.md text, not a defect this plan introduced or should "fix" by touching pre-existing ADRs.
- **Resolution:** No file was renamed or deleted — doing so would destroy legitimate prior decision records ADR-0030 explicitly cites. The actual invariant the criterion intends — **this plan creates no new/rival numbering ADR** — is confirmed true: `git status --short .planning/decisions/` shows only `0029-version-trajectory-history.md` modified; no new file was created; `0030-milestone-7-self-numbering.md` is untouched (`git diff --name-only` for it → 0 lines).
- **Files modified:** none (informational only)
- **Verification:** `git status --short .planning/decisions/` → only `0029-version-trajectory-history.md` modified.
- **Committed in:** n/a (no fix required; documented here for 13-13's close-out awareness per this note)

---

**Total deviations:** 1 auto-fixed (1 bug-class wording collision), 1 informational note (stale acceptance criterion, no action taken)
**Impact on plan:** Both items are necessary/informational; no scope creep. The wording fix was required for the plan's own automated verify to pass as specified; the informational note flags a pre-existing plan-text inaccuracy for 13-13's close-out, per the plan's own instruction to "flag it for plan 13-13's close-out" when findings surface during this task.

## Issues Encountered

None beyond the deviations documented above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0029's Trajectory table is complete and unbroken from `v0.1.0-rc.1` to `0.7.0`/`v0.7.1`; no later phase needs to append further rows for the M9-M12 range.
- ORCH-05's numbering half is discharged by citation; ADR-0030 needs no amendment.
- Plan 13-13's close-out should be aware of the Task 2 acceptance-criterion note above (pre-existing `ls .planning/decisions/ | grep -icE 'numbering'` count is 3, not 1) when it flips ORCH-05's checkbox — no action required, informational only.
- `.planning/REQUIREMENTS.md` was not edited by this plan, per its prohibitions; plan 13-10 already corrected ORCH-05's figures and plan 13-13 flips its checkbox.

---
*Phase: 13-milestone-9-12-ground-truth-recorded-account*
*Completed: 2026-08-10*
