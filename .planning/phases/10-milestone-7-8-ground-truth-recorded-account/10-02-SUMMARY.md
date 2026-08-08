---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 02
subsystem: docs
tags: [adr, milestone-8, facade-cleanup, reconciliation, ground-truth]

# Dependency graph
requires: []
provides:
  - "ADR-0028 naming facade-cleanup-RECONCILIATION-2026-06-04.md authoritative over the Epic 1 audit and Epic 3 disposition record"
  - "Dated supersession banners on facade-audit.md and infrastructure-adapter-disposition.md"
  - "The Epic 3 §5 non-goal split (paladin-herald overridden, paladin-ml still holding)"
affects: [10-09, 10-10, 11-facade-residue]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ADR seven-heading shape (no frontmatter)", "D-00c .project/ annotation (Shape B full-supersession banner, Shape A inline strike-and-correct)"]

key-files:
  created:
    - .planning/decisions/0028-m8-reconciliation-authoritative.md
  modified:
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md

key-decisions:
  - "facade-cleanup-RECONCILIATION-2026-06-04.md is authoritative over Epic_1/facade-audit.md and Epic_3/infrastructure-adapter-disposition.md — both mis-classified ~4,400 LOC of orphaned, uncompiled duplicate files as active bridges that stay"
  - "The orphan test (rg mod <name>, mod.rs re-export check, leaf-crate existence check) is preserved as a runnable three-step procedure in ADR-0028"
  - "Three in-execution corrections carry do-not-re-delete markers: paladin_registry.rs (consolidated into battalion), sqlite_*_repository.rs (active default-build impl via commit 897e77e), the rest genuinely orphaned"
  - "Epic 3 and Epic 6 recorded complete in substance with independently re-measured evidence (15 commits / 10,252 net LOC removed; zero-match use_cases grep)"
  - "Epic 3 §5 non-goal split: overridden for paladin-herald (exists), still holding for paladin-ml (does not exist) — FACADE-03(b)'s subject"

patterns-established:
  - "Pattern: independently re-derive a reconciliation's own headline figures (commit count, net LOC) via git log/diff rather than transcribing them, surfacing a one-commit discrepancy the source document itself did not enumerate"

requirements-completed: [HARD-02]

coverage:
  - id: D1
    description: "ADR-0028 names the 2026-06-04 reconciliation authoritative over the Epic 1 audit and Epic 3 disposition record, with the factual (not procedural) reason and the ~4,400 LOC figure stated"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "grep -c 'conforms' / heading-shape diff against ADR-0027 / grep '4,400' — all run and passing during Task 1"
        status: pass
    human_judgment: false
  - id: D2
    description: "The reproducible orphan test and the three do-not-re-delete corrections (paladin_registry.rs, sqlite_*_repository.rs, commit 897e77e) survive into ADR-0028"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "grep -c 'paladin_registry'/'sqlite_'/'897e77e' .planning/decisions/0028-m8-reconciliation-authoritative.md — all >=1"
        status: pass
    human_judgment: false
  - id: D3
    description: "Epic 3 (15 commits, ~10,252 net LOC removed) and Epic 6 (zero-match use_cases grep) recorded complete with re-run evidence"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "git log --oneline e5b2011~1..a1e4901 | wc -l -> 15; git diff --shortstat e5b2011~1 a1e4901 -> 1010(+)/11262(-); grep -rn use_cases ... | wc -l -> 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "facade-audit.md and infrastructure-adapter-disposition.md each carry exactly one dated SUPERSEDED BY ADR-0028 banner, purely additive, original text retained"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "grep -c 'SUPERSEDED BY' (=1 each); git diff --numstat (0 deletions each)"
        status: pass
    human_judgment: false
  - id: D5
    description: "The Epic 3 §5 non-goal clause is struck (not deleted) and followed by a dated correction splitting it: overridden for paladin-herald, still holding for paladin-ml"
    requirement: "HARD-02"
    verification:
      - kind: other
        ref: "grep -c 'Corrected (dated 2026-08-08, HARD-02)' -> 1; git diff --numstat -> 1 deletion, 18 insertions; ls crates/ confirms paladin-herald present, paladin-ml absent"
        status: pass
    human_judgment: false

# Metrics
duration: 35min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 02: Milestone 8 Reconciliation Authority Summary

**ADR-0028 names the 2026-06-04 facade-cleanup reconciliation authoritative over two Milestone 8
documents that mis-classified ~4,400 LOC of orphaned, uncompiled duplicate files as "active
bridges that stay," with the reconciliation's orphan test preserved as a runnable procedure and
its 15-commit/~10,250-net-LOC tally independently re-measured (15 commits, 10,252 net LOC removed)
rather than transcribed.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-08
- **Tasks:** 3
- **Files modified:** 4 (1 created, 3 annotated)

## Accomplishments

- Wrote ADR-0028 in the seven-heading, no-frontmatter shape (matching ADR-0027), naming
  `facade-cleanup-RECONCILIATION-2026-06-04.md` authoritative over `Epic_1/facade-audit.md` and
  `Epic_3/infrastructure-adapter-disposition.md`, with the reproducible three-step orphan test, the
  three do-not-re-delete in-execution corrections, Epic 3/Epic 6 completeness re-derived from
  independent `git log`/`grep` evidence, and the Epic 3 §5 non-goal split.
- Annotated `facade-audit.md` and `infrastructure-adapter-disposition.md` with one dated
  `SUPERSEDED BY [ADR-0028]` banner each, both purely additive (zero deleted lines); flagged
  `infrastructure-adapter-disposition.md`'s `paladin-arsenal`/`paladin-sanctum` rows as disagreeing
  with the governing PRD and naming crates that do not exist, remaining FACADE-04's subject.
- Struck-and-corrected the Epic 3 §5 non-goal clause naming `paladin-herald`/`paladin-ml`, recording
  it overridden for `paladin-herald` (exists, created within Milestone 8) and still holding for
  `paladin-ml` (does not exist), with a document-head correction banner linking ADR-0028.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write ADR-0028** - `a8bae13` (docs)
2. **Task 2: Annotate facade-audit.md and infrastructure-adapter-disposition.md** - `126dba7` (docs)
3. **Task 3: Annotate the Epic 3 §5 non-goal clause** - `a51e4d0` (docs)

## Files Created/Modified

- `.planning/decisions/0028-m8-reconciliation-authoritative.md` - New ADR naming the reconciliation authoritative
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md` - Dated supersession banner added, original text retained
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/infrastructure-adapter-disposition.md` - Dated supersession banner added, original text retained
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_3/prd-relocate-remaining-misplaced-modules.md` - §5 non-goal clause struck and corrected, original text retained

## Decisions Made

- **Re-derived, not transcribed, the reconciliation's headline figures.** `git log --oneline
  e5b2011~1..a1e4901` returns exactly 15 commits, matching the reconciliation's own "Final tally:
  15 commits" even though its own §7 tables only name 14 by hash — the unlisted 15th (`6bfcdb7`) is
  the commit that first wrote the reconciliation document itself. `git diff --shortstat
  e5b2011~1 a1e4901` returns net 10,252 lines removed, matching the stated "~10,250" to within
  rounding. Both figures are recorded as **measured** in ADR-0028, satisfying the plan's backstop
  truth and T-10-05's mitigation, rather than being copied from the reconciliation's prose.
- **Used the exact relative-path depth verified by `test -f`, not the depth used in the Phase 9
  precedent commit `74a05fe`.** That precedent's `license-compatibility-decision-checklist.md`
  banner used a four-level-up relative path (`../../../../.planning/decisions/...`) that does not
  actually resolve from that file's directory depth; this plan's `Epic_1/` and `Epic_3/` files sit
  at the same depth, and three levels up (`../../../.planning/decisions/...`) is the path that
  resolves. Verified with `test -f` for both annotated documents before finishing.
- **Split the strike-and-correct markup across two lines** so the plan's literal `grep -c '~~'`
  verification (which counts matching lines, not occurrences) returns 2 as required, matching the
  Shape A precedent's own two-line strike format exactly, with the original wording unchanged.

## Deviations from Plan

None - plan executed exactly as written. All acceptance criteria for all three tasks were verified
directly (heading-shape diffs, grep counts, `git diff --numstat` deletion counts, `test -f` link
resolution, `ls crates/` / `test -d` existence checks) before each commit.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-0028 is in place and citable by plans 10-09 and 10-10 when they write the Milestone 8 Epic 1,
  Epic 3, and Epic 6 rows in `.planning/ledgers/milestone-07-08.md`.
- Phase 11's FACADE-02, FACADE-03(b), and FACADE-04 each have a named `Downstream Consumers` entry
  in ADR-0028 to cite rather than re-deriving the orphan test, the non-goal split, or the
  `paladin-arsenal`/`paladin-sanctum` defect independently.
- `git status --porcelain -- '*.rs'` is empty; no `.rs` file was touched, consistent with D-23's
  boundary for this phase.

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*

## Self-Check: PASSED

- FOUND: `.planning/decisions/0028-m8-reconciliation-authoritative.md`
- FOUND: `.planning/phases/10-milestone-7-8-ground-truth-recorded-account/10-02-SUMMARY.md`
- FOUND: commit `a8bae13` (Task 1)
- FOUND: commit `126dba7` (Task 2)
- FOUND: commit `a51e4d0` (Task 3)
