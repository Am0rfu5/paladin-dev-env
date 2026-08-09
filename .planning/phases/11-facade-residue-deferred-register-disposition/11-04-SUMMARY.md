---
phase: 11-facade-residue-deferred-register-disposition
plan: 04
subsystem: docs
tags: [adr-0028, disposition-record, decision-recording, facade-cleanup, m9-triage]

# Dependency graph
requires:
  - phase: 11-facade-residue-deferred-register-disposition
    provides: "11-01's dated-banner/register annotation pattern; ADR-0028's e5b2011~1..a1e4901 commit-range baseline (Phase 10)"
provides:
  - "A 20-row triage of infrastructure-adapter-disposition.md's Milestone 9 candidate list, each row resolved to done / not a candidate / still open with live-tree evidence"
  - "The resolved tally (14 done / 6 not a candidate / 0 still open) that supersedes 11-RESEARCH.md's two internal, mutually-inconsistent figures"
  - "paladin-arsenal and paladin-sanctum recorded as artefacts of a mis-written table, not future crates, on three cited grounds"
  - "A one-line pointer extending the existing Phase-10 banner on the source document, purely additive"
affects: [phase-13-milestone-9-12-ledger, phase-16-cargo-doc-bar]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR-0028 commit-range triage baseline (D-12) applied to a full disposition table", "purely-additive .project/ banner extension (0 deletions, no second blockquote)"]

key-files:
  created: [.planning/registers/facade-04-m9-candidate-triage.md]
  modified: [.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md]

key-decisions:
  - "Triage baseline is ADR-0028's e5b2011~1..a1e4901 commit range and the live tree, never the disposition record's own 'Stays' claims (D-12)"
  - "Rows 1 (arsenal/) and 19 (sanctum/mod.rs) each record two disagreeing verdicts -- the row's own and the governing PRD's -- and this triage follows the PRD, per D-00b"
  - "Row 14's two-file logs/ inventory is corrected in place (error_log_adapter.rs deleted as an orphan) rather than absorbed silently into a bare 'not a candidate'"
  - "Row 17 carries the do-not-re-delete marker forward: paladin_registry.rs's 418-LOC logic is now paladin-battalion's own registry (commit ca7e4e8), not an orphan"
  - "Zero still-open rows stated as a finding in its own right, not left to read as an incomplete triage"

patterns-established:
  - "Row-identity definition stated before counting, so a tally is provably derived from the table rather than copied from another document"

requirements-completed: [FACADE-04]

coverage:
  - id: D1
    description: "All 20 rows of infrastructure-adapter-disposition.md's M9 candidate table triaged into done / not a candidate / still open, in source order, each with a command-backed live-tree evidence cell"
    requirement: "FACADE-04"
    verification:
      - kind: other
        ref: "awk row-count and disposition-word checks against .planning/registers/facade-04-m9-candidate-triage.md (20 rows, each disposition cell exactly one of the three words, ascending source order, zero still-open)"
        status: pass
    human_judgment: false
  - id: D2
    description: "paladin-arsenal and paladin-sanctum recorded as artefacts of a mis-written table (not future crates), with ls crates/ evidence and PROJECT.md's Out of Scope citation"
    requirement: "FACADE-04"
    verification:
      - kind: other
        ref: "ls crates/ (11 entries, neither name present) and grep -c 'Out of Scope' on the triage register"
        status: pass
    human_judgment: false
  - id: D3
    description: "Phase-10 banner on infrastructure-adapter-disposition.md extended in place with a pointer to the new triage, purely additive, exactly one banner"
    requirement: "FACADE-04"
    verification:
      - kind: other
        ref: "git diff --numstat (1 insertion, 0 deletions) and grep -c 'The original text below is retained unmodified' == 1"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-09
status: complete
---

# Phase 11 Plan 04: FACADE-04 M9 Candidate Triage Summary

**Triaged all 20 rows of `infrastructure-adapter-disposition.md`'s Milestone 9 candidate table against ADR-0028's executed commit range — 14 done, 6 not a candidate, 0 still open — and recorded `paladin-arsenal`/`paladin-sanctum` as artefacts of a mis-written table rather than future crates.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-09T00:13:22Z
- **Tasks:** 2
- **Files modified:** 2 (1 created, 1 modified)

## Accomplishments

- Re-verified every one of the 20 source rows against the live tree this session (`ls`/`cat`/`test -e` on every named adapter path, plus `ls crates/` for the two disputed crate names) and wrote `.planning/registers/facade-04-m9-candidate-triage.md`: 20 rows in source order, each disposition cell exactly one of `done` / `not a candidate` / `still open`, each backed by the command that produced its evidence.
- Resolved the tally — **14 `done`, 6 `not a candidate`, 0 `still open`, summing to 20** — and explicitly named both of `11-RESEARCH.md`'s superseded internal figures ("11 of the 20" in its Summary paragraph, "13 rows" in its own tally paragraph, which omits row 12 despite row 12's own cell reading `done`).
- Recorded rows 1 (`arsenal/`) and 19 (`sanctum/mod.rs`) with **both** disagreeing verdicts stated explicitly — the row's own M9-candidate verdict and the governing PRD's contrary "Stays… No" / folds-into-`paladin-memory` verdict — and followed the PRD per D-00b, rather than silently picking one.
- Corrected row 14's two-file `logs/` inventory in place: `error_log_adapter.rs` was deleted as an orphan inside ADR-0028's commit range; only `system_log_adapter.rs` survives.
- Carried forward the do-not-re-delete marker on row 17 (`paladin_registry.rs`): its 418-LOC logic was consolidated into `paladin-battalion` by commit `ca7e4e8`, not deleted as an orphan.
- Stated the zero-still-open result as a finding in its own right (`## Zero rows are still open — a finding, not an omission`), and recorded the two crate-name artefacts on three cited grounds (`## The two crate names are artefacts`).
- Extended the existing Phase-10 banner on `infrastructure-adapter-disposition.md` with one dated 2026-08-08 pointer line to the new triage — purely additive, `git diff --numstat` confirms 1 insertion / 0 deletions, still exactly one banner (no second blockquote).

## Task Commits

Each task was committed atomically:

1. **Task 1: Re-measure all 20 candidate paths and write the triage table** - `31c9ade` (docs)
2. **Task 2: Write the triage's findings and extend the source document's existing banner** - `9949b27` (docs)

_Note: Task 1's commit already carried the full register file content, including the Tally and
findings sections Task 2's `<action>` describes — the file was authored in a single Write call
before either commit was made. See Deviations below._

## Files Created/Modified

- `.planning/registers/facade-04-m9-candidate-triage.md` - New: 20-row triage table, tally, zero-still-open finding, two-crate-artefacts finding, what-this-triage-does-not-do finding
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md` - Modified: one dated pointer line added inside the existing Phase-10 blockquote banner; the 20-row table, target-crate cells and `Date: 2025-01` header are all untouched

## Decisions Made

- **Triage baseline is ADR-0028's `e5b2011~1..a1e4901` range and the live tree, never the disposition record's own claims (D-12).** Any row whose relocation falls inside that range is `done` by outcome regardless of what the source record says — the mechanism that stops this triage re-planning an already-executed relocation.
- **Rows 1 and 19 record both disagreeing verdicts and follow the PRD (D-00b).** `arsenal/`'s row targets a future `paladin-arsenal`; the governing PRD (`prd-relocate-remaining-misplaced-modules.md:154`) says "Stays… No". `sanctum/mod.rs`'s row targets a future `paladin-sanctum`; the PRD's §8 resolved decision 2 (`:310-316`) folds sanctum into `paladin-memory`, which already exists. Both rows are `not a candidate`, following the PRD.
- **The list is not blanket-marked superseded (D-11).** Rows 1, 5, 13, 14, 19 and 20 are live, deliberate decisions; rows 1 and 19 in particular were never acted on either way inside the ADR-0028 range and are decided on the PRD's authority, not written off as obsolete.
- **Row 14's inventory is corrected on the row, not absorbed silently.** The source row describes two log adapters; only one survives.
- **Row 12 (`tensorflow_adapter.rs`) is triaged as `done` but pointed at FACADE-03(b)'s register**, since its subject (an outright deletion, not the feature-gating the source row describes) belongs to `.planning/registers/facade-03-removed-features.md`, not to a Milestone 9 relocation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed a markdown table-parsing break caused by literal `\|` characters inside a cell**
- **Found during:** Task 1 verification (running the plan's own `awk -F'\|'` acceptance-criteria commands against the freshly-written register)
- **Issue:** Row 1's reason cell initially quoted the governing PRD's own table row verbatim, using escaped pipes (`` \| ``) to render literal pipe characters inside the cell's prose. `awk -F'|'` splits on every literal `|` byte regardless of a preceding backslash, so those three extra pipes shifted every subsequent field in that row (and broke the `$5`-is-disposition assumption the plan's acceptance criteria and Task 2's own tally-recount rely on).
- **Fix:** Replaced the verbatim-piped quotation with prose description of the same PRD table row (target crate, rationale, verdict stated as three dashed clauses instead of pipe-separated cells).
- **Files modified:** `.planning/registers/facade-04-m9-candidate-triage.md`
- **Verification:** Re-ran `awk -F'\|' '/^\| *[0-9]+ /{print $5}' "$T"` — all 20 rows now parse cleanly to exactly one of the three disposition words; re-ran every other acceptance-criteria command (20-row count, ascending-order sort, evidence-cell presence, PRD-citation count, row-14/row-17 markers, `e5b2011` citation count) — all pass.
- **Committed in:** `31c9ade` (Task 1 commit — fixed before the first commit, so no separate fix commit was needed)

**2. [Rule 1 - Bug] Fixed a banner edit that initially violated the "0 deletions" acceptance criterion**
- **Found during:** Task 2 verification (`git diff --numstat` after the first banner-edit attempt)
- **Issue:** The first attempt to extend the Phase-10 banner rewrote the existing closing sentence ("for those two names. The original text below is retained unmodified.") by splitting it across two lines to insert the new pointer paragraph between them. `git diff --numstat` registered this as a modified (not purely-added) line, producing 1 deletion where the plan's acceptance criteria require exactly 0 (`--numstat` deletions of `0`, the 20-row table and header untouched).
- **Fix:** Reworked the edit to insert the entire new pointer paragraph as new lines placed *before* the original closing sentence, leaving that sentence byte-identical and on its original single line.
- **Files modified:** `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md`
- **Verification:** `git diff --numstat` now reports `1 0` (1 insertion, 0 deletions); `grep -c 'The original text below is retained unmodified'` returns `1`; `grep -c 'facade-04-m9-candidate-triage'` returns `1`.
- **Committed in:** `9949b27` (Task 2 commit — fixed before the commit, so no separate fix commit was needed)

---

**Total deviations:** 2 auto-fixed (both Rule 1 — markdown/diff mechanics discovered while running the plan's own acceptance-criteria commands, not content errors). **No content deviation from the plan's specified dispositions, tally, or findings.**
**Impact on plan:** Both fixes were required for the plan's own automated `<verify>` and acceptance criteria to pass; neither changed a single disposition, evidence claim, or finding. No scope creep.

## Issues Encountered

None beyond the two auto-fixed mechanical issues above.

## Next Phase Readiness

- FACADE-04 is closed by disposition: every row of `infrastructure-adapter-disposition.md`'s 20-row table now carries a `done` / `not a candidate` verdict findable from `.planning/` without reading `.project/`, satisfying ROADMAP criterion 5.
- The `paladin-arsenal` / `paladin-sanctum` non-existence is now doubly recorded — PROJECT.md's `### Out of Scope` (prior phase) and this triage register (this plan) — so a future reader of either lands on the same answer.
- No blockers for 11-05 (`PROMOTION.md` final amendment) or for Phase 13's Milestone 9-12 ledger, which can cite this triage directly for any row touching the M8/M9 boundary.

---
*Phase: 11-facade-residue-deferred-register-disposition*
*Completed: 2026-08-09*
