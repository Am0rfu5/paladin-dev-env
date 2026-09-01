---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 07
subsystem: infra
tags: [release, crates-io, recovery, rehearsal, oidc, trusted-publishing, runbook]

# Dependency graph
requires:
  - phase: 20-03
    provides: "scripts/check-release-consistency.sh's status tokens, CI_MISMATCH/CI_LOOKUP_FAILED distinction — the gate the rc.4 rehearsal exercised live and found two bugs in"
  - phase: 20-05
    provides: "scripts/publish-crates.sh's four outcome states and per-crate outcome table — the loop the rc.4 rehearsal's mixed-split recovery run exercised"
  - phase: 20-06
    provides: "docs/src/appendix/release-recovery.md, the runbook whose status line this plan replaces with a tested one"
provides:
  - "20-RECOVERY-EVIDENCE.md — the phase's rehearsal evidence log: two real rehearsals (v0.8.1-rc.3 pre-Phase-20, v0.8.1-rc.4 with Phase 20's own gate and scripts live), three-moment per-crate registry snapshots, both runs' outcome tables, independent OIDC-provenance re-verification proving Assumption A3, and a six-finding ledger"
  - "docs/src/appendix/release-recovery.md status line updated from untested to tested (2026-08-30), with both run URLs and three new procedure lessons (PR-merge-commit release flow, unpublished-tag-may-move/published-never, CI_MISMATCH recovers via re-run not re-tag)"
  - "Two live gate bug fixes (PRs #45, #46) discovered by the rehearsal itself and merged before this document was written"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Recovery evidence documents follow 19-PUBLISH-EVIDENCE.md's shape: dated, sourced, independently re-verified rather than transcribed from operator report, with an explicit assumptions-and-limits section naming what was not proven"
    - "A rehearsal run twice (once pre-fix, once post-fix) generates its own findings ledger when the second run exercises code the first run predates — recorded as a ledger rather than folded silently into the narrative"

key-files:
  created: []
  modified:
    - .planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/20-RECOVERY-EVIDENCE.md
    - docs/src/appendix/release-recovery.md

key-decisions:
  - "Recorded both rehearsals (rc.3 and rc.4) in full rather than only the more complete rc.3 one, because rc.4 is the first live exercise of Phase 20's own gate and create-or-reuse-release.sh, and it found two real bugs (Findings 5, 6) that rc.3 could not have found since that code did not exist yet when rc.3 ran"
  - "Treated the plan's 'two run URLs' framing as describing two events, not two literal URLs — GitHub's 'Re-run failed jobs' mechanic attaches every attempt to the original run ID, so both rehearsals produced one run URL apiece with multiple numbered attempts; recorded this explicitly as an observed mechanic that matches the runbook's own 're-running the same tag's existing workflow run' language, rather than silently reshaping the record to fit the plan's original two-URL phrasing"
  - "Used rc.3's Task-1 pre-collision check (all eleven 404, captured before the tag was pushed) as the authoritative 'before' moment for the three-moment snapshot, since registry state only moves 404 to 200 and never back — rather than requiring a separate redundant before-snapshot immediately before rc.3's tag push"
  - "Stated rc.4's before-state honestly as evidenced by the publish loop's own per-crate pre-checks rather than a standalone snapshot, since none was taken for that rehearsal — the plan's honesty requirement extends to admitting a weaker evidence path exists alongside a stronger one, not just to declining outright"
  - "Added three procedure lessons to the runbook beyond the status-line update alone, per this plan's explicit instructions (superseding PLAN.md Task 3's more general 'change nothing else' framing, written before the rehearsal's actual findings were known) — recorded here as a deviation, see below"

patterns-established: []

requirements-completed: [PUBOPS-03, PUBOPS-05]

coverage:
  - id: D1
    description: "20-RECOVERY-EVIDENCE.md records, per crate, the registry state at three moments (before, mid-interruption, after) for the rc.3 rehearsal, satisfying the plan's three-moment must-have"
    requirement: "PUBOPS-05"
    verification:
      - kind: other
        ref: "python3 consistency assertion (dry-run disclaimer present, run URLs present, both outcome-table states present, runbook links evidence file) -> 'evidence and status line consistent'"
        status: pass
    human_judgment: false
  - id: D2
    description: "Both rehearsal run URLs and conclusions are recorded, and both per-crate outcome tables (rc.3's log-line record and rc.4's step-summary tables) are present, including a genuinely mixed split (non-zero already-at-this-version and non-zero published-now) on the rc.4 recovery run"
    requirement: "PUBOPS-03"
    verification:
      - kind: other
        ref: "20-RECOVERY-EVIDENCE.md Rehearsal 2 recovery outcome table: 5 already-at-this-version, 6 published-now"
        status: pass
    human_judgment: false
  - id: D3
    description: "Assumption A3 (OIDC token exchange succeeds on a same-tag re-run against the crates-io environment's deployment policy) is settled, independently re-verified against crates.io's trustpub_data.run_id field rather than taken from either workflow run's self-report"
    requirement: "PUBOPS-03"
    verification:
      - kind: other
        ref: "trustpub_data run_id=33210072054 (paladin-llm, paladin-ai) and run_id=33322587044 (paladin-battalion, paladin-memory, paladin-ai), both independently queried"
        status: pass
    human_judgment: false
  - id: D4
    description: "The runbook's status line matches reality: tested, dated, linking both run URLs and the evidence file; cargo publish --dry-run is stated plainly as never used as evidence anywhere in either rehearsal"
    requirement: "PUBOPS-05"
    verification:
      - kind: other
        ref: "docs/src/appendix/release-recovery.md status line + 20-RECOVERY-EVIDENCE.md dry-run disclaimer, checked by the same consistency assertion"
        status: pass
    human_judgment: false
  - id: D5
    description: "Shell-guard test harnesses (check-release-consistency_test.sh, publish-crates_test.sh, make test-shell-guards) still pass after the doc-only changes, confirming no scope creep into the scripts themselves"
    verification:
      - kind: other
        ref: "./tests/scripts/check-release-consistency_test.sh (32/32 pass), ./tests/scripts/publish-crates_test.sh (54/54 pass), make test-shell-guards (54/54 pass, no mutation)"
        status: pass
    human_judgment: false

# Metrics
duration: 35min
completed: 2026-08-30
status: complete
---

# Phase 20 Plan 07: Recovery Rehearsal Evidence Summary

**Two real rehearsals (v0.8.1-rc.3, v0.8.1-rc.4) proved the documented recovery procedure end to end — including the first live test of Phase 20's own pre-publish gate, which found and fixed two real bugs along the way — and the runbook now says tested.**

## Performance

- **Duration:** 35 min (Task 3 only — Tasks 1 and 2 were authorized and executed by the human operator across 2026-08-28 through 2026-08-30, outside this execution window)
- **Started:** 2026-08-30T00:00:00Z (approx, Task 3 start)
- **Completed:** 2026-08-30
- **Tasks:** 3 (Task 1: operator decision, recorded pre-existing; Task 2: operator-executed rehearsal, recorded pre-existing; Task 3: this execution — evidence file + runbook + summary)
- **Files modified:** 2

## Accomplishments
- Extended `20-RECOVERY-EVIDENCE.md` with the complete rehearsal record for both `v0.8.1-rc.3` (pre-Phase-20 pipeline, complete three-moment registry snapshot) and `v0.8.1-rc.4` (Phase 20's own gate, `create-or-reuse-release.sh`, and rewritten `publish-crates.sh` loop, live for the first time)
- Independently re-verified Assumption A3 (OIDC token exchange survives a same-tag re-run) against crates.io's `trustpub_data.run_id` field on both rehearsals rather than transcribing either workflow's self-report
- Recorded a genuinely mixed-split recovery outcome table (5 `already-at-this-version`, 6 `published-now`) on the `rc.4` recovery run, satisfying PUBOPS-04's proof that recovery is real rather than a from-scratch republish
- Updated `docs/src/appendix/release-recovery.md`'s status line from `untested` to `tested (2026-08-30)`, linking both run URLs and the evidence file, and added three procedure lessons the rehearsals proved that were not previously documented

## Task Commits

Only Task 3 required a commit in this execution window (Tasks 1 and 2 are decision/human-action checkpoints with no code or doc artifacts of their own — their outcomes are recorded as data inside the evidence file this task wrote):

1. **Task 3: Record the evidence and make the runbook's status line true** - `5bb22125` (docs)

**Plan metadata:** commit pending (this SUMMARY + STATE/ROADMAP/REQUIREMENTS updates)

## Files Created/Modified
- `.planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/20-RECOVERY-EVIDENCE.md` - Extended from Task-1-only scaffold to the full two-rehearsal evidence record: three-moment registry snapshots, both run URLs/conclusions, both outcome tables, OIDC-provenance re-verification, six-finding ledger, assumptions-and-limits
- `docs/src/appendix/release-recovery.md` - Status line changed to tested with run URLs and evidence-file reference; three procedure lessons added

## Decisions Made
See `key-decisions` in frontmatter above — most notably: recording both rehearsals in full (not just the more complete `rc.3` one) because `rc.4` is the only live proof of Phase 20's own gate and recovery scripts, and treating "two run URLs" as two rehearsal events rather than two literal distinct URLs, since GitHub's re-run mechanic keeps one run ID across attempts.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - explicit human instruction] Added three procedure lessons to the runbook beyond the status-line change alone**
- **Found during:** Task 3
- **Issue:** `20-07-PLAN.md`'s own Task 3 acceptance criteria states "Change nothing else in the runbook" and "`git diff docs/src/appendix/release-recovery.md` touches only the status line" — written before the rehearsal ran, when the plan author could not yet know what the rehearsal would prove. The orchestrator's explicit Task 3 instructions for this execution (informed by the actual rehearsal outcome) directed adding three specific procedure lessons the rehearsals proved: (a) release commits travel via PR merge commit with the tag pushed after, (b) an unpublished tag may be re-tagged but a published version never may, (c) a tag pushed before its commit's CI completes is refused (`CI_MISMATCH`) and recovers via re-run, not re-tag.
- **Fix:** Added the three lessons as a short numbered list immediately under the updated status-line paragraph, before the pre-existing `## 1. Establishing what actually reached crates.io` section; changed nothing else in the file.
- **Files modified:** `docs/src/appendix/release-recovery.md`
- **Verification:** The plan's own automated `<verify>` script (which checks status-line/evidence-file consistency, not the "touches only the status line" prose criterion) still passes; all shell-guard test harnesses still pass with no mutation to `scripts/` or `.github/workflows/`.
- **Committed in:** `5bb22125` (Task 3 commit)

---

**Total deviations:** 1 auto-fixed (Rule 2, per explicit instruction superseding a stale plan-authored acceptance-criteria line)
**Impact on plan:** The runbook is more useful for exactly the failure modes the rehearsal exercised; no scope creep beyond the three named lessons, and the file's mechanically-checked `<verify>` assertion still passes.

## Issues Encountered
The plan's own automated consistency check (`re.search(r'\buntested\b', r, re.I)` over the whole runbook body) is fragile: the word "untested" legitimately appears elsewhere in the runbook (describing `workflow_dispatch` eligibility, Assumption A1, which remains untested regardless of this rehearsal). This did not cause a false failure in practice — the script's `else` branch happens to also pass when "untested" appears anywhere, and the actual status-line and run-URL assertions were satisfied — but it means the script's `tested`/`untested` branch selection is not a reliable signal on its own. Not fixed (out of this plan's file scope — `check-release-consistency.sh` and `publish-crates.sh` are the only shell scripts this plan's `<verify>` covers, and the fragile assertion lives inline in the plan file itself, not in a script). Recorded here for visibility.

## User Setup Required
None - no external service configuration required. The rehearsal's registry and credential setup was completed in Phase 19 and re-used, not reconfigured.

## Next Phase Readiness
- PUBOPS-03 and PUBOPS-05 are both proven with recorded, independently re-verified evidence — no longer resting on documentation alone.
- The recovery procedure is proven end-to-end against a real partial-publish failure, twice, including against Phase 20's own gate and recovery scripts.
- Two live gate bugs (Findings 5, 6) were found and fixed as a byproduct of the rehearsal, already merged (PRs #45, #46) before this document was written — no follow-up action needed.
- Remaining known gaps (stated in the evidence file's Assumptions and limits section): `workflow_dispatch` eligibility under Trusted Publishing (Assumption A1) remains untested; failure modes other than operator cancellation were not exercised; publish-to-index-visible timing against the 180-second poll bound was not measured in either rehearsal. None of these block phase completion — they are residual, named risks, not open requirements.

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-30*
