---
phase: 07-workspace-ground-truth-recorded-answers
plan: 03
subsystem: docs
tags: [adr, project-corpus, milestone-numbering, ingest-conflicts]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: ADR-0014 (created by plan 07-01, this plan amends it in place)
provides:
  - Eight byte-equivalent Milestone 5/6 `.project/` extracts each carrying a one-line dated
    ADR-0014 pointer banner
  - ADR-0014 amended with the eight extract paths under Code Locations and a dated
    nine/eight/seven count reconciliation under Context
affects: [08-workspace-close-out, any-phase-citing-milestone-4-6-project-docs]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "One-line pointer-banner annotation (PATTERNS.md shape (d)) for byte-equivalent corpus
      copies — no inline strikethrough, no rewriting, purely additive diff"

key-files:
  created: []
  modified:
    - .planning/decisions/0014-milestone-4-6-tier-numbering.md
    - .project/Milestone_5-Workspace-Decomposition/Epic_2/Milestone_5-Tier_2-Workspace-Decomposition-Epic_2.md
    - .project/Milestone_5-Workspace-Decomposition/Epic_3/Milestone_5-Tier_2-Epic_3-Battalion_Extraction.md
    - .project/Milestone_5-Workspace-Decomposition/Epic_4/Milestone_5-Epic_4.md
    - .project/Milestone_5-Workspace-Decomposition/Epic_5/Milestone_5-Epic_5.md
    - .project/Milestone_6-Architectural-Refinements/Epic_1/Milestone_6-Epic_1-Decompose_App_Settings.md
    - .project/Milestone_6-Architectural-Refinements/Epic_2/Milestone_6-Epic_2-Relocation_Manager_Layer_Orch_services.md
    - .project/Milestone_6-Architectural-Refinements/Epic_3/Milestone_6-Epic_3-Co-locate_Maneuver_DSL_w_Battalion.md
    - .project/Milestone_6-Architectural-Refinements/Epic_4/Milestone_6-Epic_4-Relocate_CircuitBreaker_Infra_Layer.md

key-decisions:
  - "The enumerated eight extracts (from INGEST-CONFLICTS.md:656's own Note) is the operative
    set, not the headline's nine or CONTEXT.md D-08's pre-amendment seven — recorded in ADR-0014"
  - "Milestone 5 Epic 1 is not a ninth extract: it holds a PRD
    (prd-workspace-initialization-and-paladin-core-extraction.md), a task list, two decision
    records, and two analysis files — confirmed by directory listing, not an overview extract"
  - "All eight extracts start with an H2 (## Epic N: ...), not an H1 — the plan's read_first
    assumed an H1; the banner was placed immediately below each file's actual top heading
    instead, preserving the plan's intent (banner under the title, above all other content)"

patterns-established:
  - "One-line pointer-banner shape (PATTERNS.md (d)) applied verbatim across all eight files:
    names ADR-0014 by relative link, states the source overview the copy came from, and closes
    with the byte-equivalent-copy rationale for why no inline correction was applied"

requirements-completed: [ARCH-02]

coverage:
  - id: D1
    description: "Eight byte-equivalent Milestone 5/6 .project/ extracts each carry a one-line
      dated ADR-0014 pointer banner under their top heading, with zero deletions and zero
      inline strikethrough in any of them"
    requirement: "ARCH-02"
    verification:
      - kind: other
        ref: "grep -c ADR-0014 (8 files, all =1); grep -c '~~' (8 files, all =0); git diff
          --numstat (8 files, all 0 deletions)"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0014 amended in place: eight extract paths appended to Code Locations
      (purely additive), and a dated reconciliation of the nine/eight/seven count discrepancy
      appended to Context, naming the enumerated eight as operative"
    requirement: "ARCH-02"
    verification:
      - kind: other
        ref: "grep -c '^## ' ADR-0014 = 7 (unchanged); sed Code Locations bullet count = 14;
          sed Context block contains nine/eight/seven; git diff --numstat 0 deletions"
        status: pass
    human_judgment: false

duration: 25min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 03: Byte-Equivalent Extract Pointer Banners & Count Reconciliation Summary

**Eight byte-equivalent Milestone 5/6 `.project/` extracts each pointer-banner ADR-0014; ADR-0014 amended with the enumerated eight and the nine/eight/seven count reconciliation.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-08-06T18:11:02Z
- **Tasks:** 2
- **Files modified:** 9 (1 ADR, 8 `.project/` extracts)

## Accomplishments

- Enumerated the byte-equivalent extract set directly from `INGEST-CONFLICTS.md:656`'s own Note
  (not inferred from a directory listing), existence-verified all eight paths on disk, and
  confirmed `.project/Milestone_5-Workspace-Decomposition/Epic_1/` holds a PRD rather than an
  overview extract, closing the "why not a ninth" question with direct evidence.
- Amended `.planning/decisions/0014-milestone-4-6-tier-numbering.md` in place per D-00f: appended
  a dated reconciliation paragraph to `## Context` naming all three counts (nine/eight/seven) and
  declaring the enumerated eight operative, and appended the eight extract paths as bullets under
  `## Code Locations`. No existing section rewritten; heading count stayed at 7.
- Inserted a one-line dated ADR-0014 pointer banner (PATTERNS.md markup shape (d)) into each of
  the eight extracts, immediately below each file's top heading and above all other content.
  Each banner names the source overview the copy came from and states the byte-equivalent-copy
  rationale for why no inline correction was applied.
- Committed all nine files in a single commit (`a16aa71`) per the plan's explicit instruction,
  with the project's pre-commit hooks (fmt + clippy) running to completion rather than skipped.

## Task Commits

Both tasks' file changes were combined into the single commit the plan explicitly required:

1. **Task 1: Enumerate the extract set and record the count reconciliation in ADR-0014** — staged,
   not committed separately (combined per Task 2's explicit instruction)
2. **Task 2: Apply the one-line pointer banner to all eight byte-equivalent extracts** —
   `a16aa71` (docs)

**Commit:** `a16aa71` — `docs(07-03): annotate byte-equivalent Milestone 5/6 extracts with ADR-0014 pointers` (9 files changed, 88 insertions, 0 deletions)

## Files Created/Modified

- `.planning/decisions/0014-milestone-4-6-tier-numbering.md` - amended: 8 Code Locations bullets
  appended, dated Context reconciliation appended
- `.project/Milestone_5-Workspace-Decomposition/Epic_2/Milestone_5-Tier_2-Workspace-Decomposition-Epic_2.md` - ADR-0014 pointer banner added
- `.project/Milestone_5-Workspace-Decomposition/Epic_3/Milestone_5-Tier_2-Epic_3-Battalion_Extraction.md` - ADR-0014 pointer banner added
- `.project/Milestone_5-Workspace-Decomposition/Epic_4/Milestone_5-Epic_4.md` - ADR-0014 pointer banner added
- `.project/Milestone_5-Workspace-Decomposition/Epic_5/Milestone_5-Epic_5.md` - ADR-0014 pointer banner added
- `.project/Milestone_6-Architectural-Refinements/Epic_1/Milestone_6-Epic_1-Decompose_App_Settings.md` - ADR-0014 pointer banner added
- `.project/Milestone_6-Architectural-Refinements/Epic_2/Milestone_6-Epic_2-Relocation_Manager_Layer_Orch_services.md` - ADR-0014 pointer banner added
- `.project/Milestone_6-Architectural-Refinements/Epic_3/Milestone_6-Epic_3-Co-locate_Maneuver_DSL_w_Battalion.md` - ADR-0014 pointer banner added
- `.project/Milestone_6-Architectural-Refinements/Epic_4/Milestone_6-Epic_4-Relocate_CircuitBreaker_Infra_Layer.md` - ADR-0014 pointer banner added

## Enumerated Eight (confirmed on disk)

1. `.project/Milestone_5-Workspace-Decomposition/Epic_2/Milestone_5-Tier_2-Workspace-Decomposition-Epic_2.md`
2. `.project/Milestone_5-Workspace-Decomposition/Epic_3/Milestone_5-Tier_2-Epic_3-Battalion_Extraction.md`
3. `.project/Milestone_5-Workspace-Decomposition/Epic_4/Milestone_5-Epic_4.md`
4. `.project/Milestone_5-Workspace-Decomposition/Epic_5/Milestone_5-Epic_5.md`
5. `.project/Milestone_6-Architectural-Refinements/Epic_1/Milestone_6-Epic_1-Decompose_App_Settings.md`
6. `.project/Milestone_6-Architectural-Refinements/Epic_2/Milestone_6-Epic_2-Relocation_Manager_Layer_Orch_services.md`
7. `.project/Milestone_6-Architectural-Refinements/Epic_3/Milestone_6-Epic_3-Co-locate_Maneuver_DSL_w_Battalion.md`
8. `.project/Milestone_6-Architectural-Refinements/Epic_4/Milestone_6-Epic_4-Relocate_CircuitBreaker_Infra_Layer.md`

All eight `test -f` clean. Disjoint from the five D-08 inline-correction targets (the three
milestone overviews plus `Epic_4/prd-paladin-llm-extraction.md` and
`Epic_2/prd-paladin-ports-extraction.md`) — different filenames, different directories, confirmed
by direct comparison during Task 1.

## What Was Actually Found in Milestone 5 Epic 1

`.project/Milestone_5-Workspace-Decomposition/Epic_1/` contains:
- `prd-workspace-initialization-and-paladin-core-extraction.md` (a PRD, 23,051 bytes)
- `tasks-workspace-initialization-and-paladin-core-extraction.md` (companion task list)
- `decisions/` (directory — the `battalion-result-upward-dependency-decision.md` /
  `-options.md` pair referenced elsewhere in the corpus)
- `baseline-test-count.txt`, `paladin-core-dependency-tree.txt` (analysis artifacts)

No overview-extract-shaped document exists in this directory. This confirms the CONTEXT.md D-08
amendment's claim: Milestone 5 Epic 1 is not a ninth byte-equivalent extract.

## The Reconciliation Appended to ADR-0014 (exact wording)

Appended to `## Context`, immediately before `## Decision`:

> **Reconciliation (dated 2026-08-06, plan 07-03):** `INGEST-CONFLICTS.md`'s own count of its
> byte-equivalent Milestone 5/6 extracts is stated three different ways in this corpus: nine in
> its headline at `:655` ("Nine of the nineteen run-3 DOCs are verbatim extracts"), eight in its
> own enumerated Note at `:656` (M5 Epics 2, 3, 4, 5 and M6 Epics 1, 2, 3, 4), and seven in this
> phase's CONTEXT.md D-08 before its 2026-08-06 amendment. **The enumerated eight is operative.**
> `.project/Milestone_5-Workspace-Decomposition/Epic_1/` was checked directly during this task and
> holds a PRD (`prd-workspace-initialization-and-paladin-core-extraction.md`), its companion task
> list, two decision records, and two analysis text files — not an overview extract — so it is not
> a ninth member of this set. The headline figure at `INGEST-CONFLICTS.md:655` is a bookkeeping
> defect inside that file; it is left in place there, per this phase's annotate-do-not-rewrite
> posture toward the ingest ledger, and corrected here as the operative count for every downstream
> reader of this ADR.

Independently verified against the tree: this agrees with CONTEXT.md D-08's 2026-08-06 amendment.
No disagreement to flag.

## Decisions Made

- Followed the plan's explicit instruction to combine both tasks' file changes into a single
  commit at the end (rather than committing Task 1 and Task 2 separately, which the general
  execute-plan protocol would otherwise call for) — the plan's own verify step for Task 2 requires
  `git log -1 --name-only` to show all nine files in one commit.
- Ran the project's pre-commit hooks to completion (fmt + clippy) rather than passing
  `--no-verify`, per the plan's explicit "Do not pass `--no-verify`" instruction, using a 300s
  Bash timeout to accommodate the cold workspace compile the hook triggers.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Banner anchor point: no H1 exists in any of the eight extract files**
- **Found during:** Task 2, read_first step (reading each file's first 20 lines)
- **Issue:** The plan's `read_first` instructed placing the banner "immediately under the existing
  H1 rather than above it," and the Task 2 acceptance criteria checks for `grep -n '^# '` (a
  literal H1, single `#`). All eight extract files, however, begin directly with an H2 section
  heading (`## Epic N: ...`) — they are excerpts of one section from a larger overview document,
  which itself carries the H1. No H1 exists in any of the eight files to anchor against.
- **Fix:** Inserted the banner immediately below each file's actual top heading (the H2), which
  functions as the document's effective title in this excerpted context — preserving the plan's
  intent (banner under the title, above all other content) without inventing an H1 that isn't
  there. Verified across all eight files before editing (Read tool, first 5 lines each).
- **Files modified:** all eight extract files listed above
- **Verification:** `git diff --numstat` shows 0 deletions and only additive insertions in all
  eight files; each contains `ADR-0014` and `0014-milestone-4-6-tier-numbering.md`; none contains
  `~~`. The plan's Task 2 acceptance-criteria H1-vs-banner line-number check does not apply
  literally (no H1 exists to compare against), but the substantive requirement — banner above all
  other original content — is met in every file.
- **Committed in:** `a16aa71`

---

**Total deviations:** 1 auto-fixed (1 blocking — anchor-point adaptation, no architectural change)
**Impact on plan:** No scope creep; the fix is a literal reading of "immediately under the title"
applied to the actual document structure found, not the assumed one. All eight files still meet
every substantive `must_haves` truth and acceptance criterion (ADR-0014 present, no strikethrough,
purely additive diff, single combined commit).

## Issues Encountered

None beyond the H1/H2 deviation documented above. The first commit attempt hit the Bash tool's
default 2-minute timeout mid-way through `cargo clippy` (a cold workspace compile inside a fresh
worktree); no partial commit was created (`git status` confirmed files remained staged), and the
retry with a 300s timeout completed cleanly with fmt and clippy both passing.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ARCH-02's second half (byte-equivalent extract annotation + count reconciliation) is closed.
  Combined with plan 07-01's Milestone 4 overview title correction, ADR-0014 now names all eight
  extract annotation targets plus the three still-pending inline corrections (plans 07-02, 07-04,
  07-05, 07-09) in its `## Downstream Consumers` section — no changes needed there since those
  plans were already listed before this plan ran.
- No blockers for later phases. `.planning/decisions/PROMOTION.md`'s "Next free ADR number" line
  is unaffected by this plan (that's plan 07-13's job).

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
