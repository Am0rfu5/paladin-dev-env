---
phase: 11-facade-residue-deferred-register-disposition
plan: 01
subsystem: docs
tags: [rustdoc, adr-governance, facade-cleanup, deferred-items, roadmap-amendment]

# Dependency graph
requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "ADR-0031 (leaf-to-leaf invariant), ADR-0028 (M8 reconciliation authoritative), ADR-0018 (no-shim posture), PROMOTION.md next-free-ADR-number bookkeeping"
provides:
  - "Per-occurrence rustdoc-stdout disposition for all 17 D5 println!/eprintln!/dbg! occurrences (FACADE-01)"
  - "Dated Shape-A correction on deferred-items.md's D5 clause, original text retained"
  - "ROADMAP.md §Phase 11 criterion 1 amended in place, falsifiable, original retained"
  - "REQ-m8-deferred-items-register ledger row's D5 half closed, D1-D4 explicitly pending plan 11-05"
  - "ADR allocation gate resolved: option-a (two ADRs, 0034/0035, next free 0036) confirmed for wave 2"
affects: [11-02-PLAN.md, 11-03-PLAN.md, 11-05-PLAN.md]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Shape-A dated correction banner (struck original text + inline 'Corrected (dated ..., ID):' note) applied to a .project/ document that carried no prior banner"
    - "ROADMAP criterion in-place amendment: '**Corrected by Phase N, dated DATE (plan N-NN):** ... Original criterion text retained above.'"
    - "Ledger Evidence-cell amendment in place with an explicit 'pending — plan N-NN' marker so a partial closure cannot be misread as full closure"

key-files:
  created:
    - .planning/registers/facade-01-rustdoc-stdout-disposition.md
  modified:
    - .project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md
    - .planning/ROADMAP.md
    - .planning/ledgers/milestone-07-08.md

key-decisions:
  - "FACADE-01's premise is inverted: all 17 D5 occurrences are rustdoc-example doc-comment lines, not runtime stdout — the register's count was exact, its characterisation was wrong (D-01)"
  - "ADR allocation for wave 2 confirmed as option-a: ADR-0034 = D1-D4 disposition set (plan 11-02), ADR-0035 = paladin-ml leaf-crate placement condition (plan 11-03), PROMOTION.md's next-free advances to 0036 (amended last, by plan 11-05)"

patterns-established:
  - "Pattern: per-occurrence disposition tables grouped per file, in a fixed file order, every file gets its own ### heading even at one occurrence — reusable by 11-03/11-04's register files"

requirements-completed: [FACADE-01]

coverage:
  - id: D1
    description: "Per-occurrence disposition register for all 17 D5 println!/eprintln!/dbg! occurrences across 6 files, naming each deliberate rustdoc-example stdout"
    requirement: "FACADE-01"
    verification:
      - kind: other
        ref: "grep -rn 'println!\\|eprintln!\\|dbg!' src/application/services/ src/infrastructure/ | wc -l (returns 17); same command | grep -v '///' | grep -v '//!' | wc -l (returns 0); loop over all 17 file:line citations against the register (17/17 present)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Dated Shape-A correction on deferred-items.md's D5 clause (Effort/risk rating, Recommendation line, Quick-wins line), original text retained"
    requirement: "FACADE-01"
    verification:
      - kind: other
        ref: "grep -n 'Effort / risk:\\*\\* low / low' deferred-items.md (still matches); grep -c '2026-08-08' deferred-items.md (5); grep -n 'Quick wins:\\*\\* D5' deferred-items.md (still matches, correction within 3 lines); grep -c 'facade-01-rustdoc-stdout-disposition' deferred-items.md (3)"
        status: pass
    human_judgment: false
  - id: D3
    description: "ROADMAP.md §Phase 11 criterion 1 amended in place with a falsifiable, re-runnable claim; original text retained"
    requirement: "FACADE-01"
    verification:
      - kind: other
        ref: "sed -n '726p' ROADMAP.md (original sentence present); grep -c 'Original criterion text retained above' ROADMAP.md (5 -> 6)"
        status: pass
    human_judgment: false
  - id: D4
    description: "REQ-m8-deferred-items-register ledger row's Evidence cell amended in place with the D5 disposition and an explicit D1-D4 pending marker; row count and Verdict cell unchanged"
    requirement: "FACADE-01"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' milestone-07-08.md (86, unchanged); grep -c 'pending — plan 11-05' milestone-07-08.md (1, inside the REQ-m8-deferred-items-register row)"
        status: pass
    human_judgment: false
  - id: D5
    description: "ADR allocation for Phase 11 wave 2 confirmed by checkpoint (option-a: ADR-0034 D1-D4 set / ADR-0035 paladin-ml placement, next free 0036); ADR-0031's unratified ⚠ HUMAN REVIEW status carried forward, not laundered by this resolution"
    verification: []
    human_judgment: true
    rationale: "The checkpoint decision was auto-selected by the orchestrator under AUTO_MODE=true, not independently reasoned by a human reviewer. Whether option-a's coarser supersession trade-off and the still-unratified ADR-0031 dependency are acceptable for the phase remains a human judgment call at phase verification, even though the selection itself is recorded and traceable."

duration: ~25min (Task 1 + checkpoint round-trip)
completed: 2026-08-09
status: complete
---

# Phase 11 Plan 01: FACADE-01 — D5 Rustdoc-Stdout Disposition & ADR Allocation Gate Summary

**All 17 D5 `println!`/`eprintln!`/`dbg!` occurrences recorded as deliberate rustdoc-example stdout in a new per-occurrence register, with the misleading "quick win / runtime residue" framing corrected at source in `deferred-items.md` and ROADMAP criterion 1, and the wave-2 ADR allocation gated and confirmed as two ADRs (0034/0035).**

## Performance

- **Duration:** ~25 min (Task 1 execution + checkpoint decision round-trip)
- **Completed:** 2026-08-09
- **Tasks:** 2 (1 tracer task, 1 checkpoint:decision gate)
- **Files modified:** 4 (1 created, 3 modified)

## Accomplishments

- Re-measured D5's two evidence-bar commands live against the tree (17 occurrences / 6 files;
  0 non-doc-comment lines) before writing anything, per D-00e — confirming the register's count
  was exact and its characterisation was not (D-01).
- Created `.planning/registers/facade-01-rustdoc-stdout-disposition.md`: a per-file, per-occurrence
  disposition table for all 17 occurrences (never collapsing adjacent-line occurrences inside one
  fenced block), a citation index for direct `file:line` grepping, the arithmetic proof
  (`4+6+3+2+1+1 = 17`), the Phase 15 hand-off for the four `herald_registry.rs` `rust,ignore`
  fences (165/184/197/210, fence lines 163/182/196/208), and the corpus-level finding that
  `deferred-items.md` — described elsewhere as the corpus's highest-fidelity document — was wrong
  about the *kind* of these occurrences while exact about their *count*.
- Corrected `deferred-items.md`'s D5 clause at source (Shape A per D-00c/Phase-8 precedent
  `94814ff`): struck-and-corrected the "Effort / risk: low / low" rating, the "convert genuine
  diagnostics to `log::*`" recommendation, and the "Quick wins: D5" line, each with a dated
  2026-08-08 correction pointing at the new register — original wording retained beneath every
  strike, plus a document-level pointer note.
- Amended `.planning/ROADMAP.md` §Phase 11 criterion 1 in place, in this file's own established
  amendment shape (`**Corrected by Phase 11, dated 2026-08-08 (plan 11-01):** ... Original
  criterion text retained above.`) — the corrected criterion states the two re-runnable commands
  and their 17/0 verdicts rather than restating the work performed, so a later auditor can falsify
  it if the tree changes.
- Amended the `REQ-m8-deferred-items-register` row in `.planning/ledgers/milestone-07-08.md`
  (Evidence cell only, Verdict cell unchanged at `deferred with register`) with the D5 disposition
  and an explicit `D1–D4: pending — plan 11-05` marker so the row cannot be misread as closed for
  the other four deferred items.
- Reached Task 2's `checkpoint:decision` gate (blocking, ADR allocation for wave 2) and — per this
  plan's explicit checkpoint-handling instructions — did not self-select; returned the full
  decision context and options to the orchestrator. The orchestrator auto-selected **option-a**
  under `AUTO_MODE=true` and resumed this agent to record the resolution and close the plan.

## Task Commits

Each task was committed atomically:

1. **Task 1: Record all 17 D5 dispositions and correct the framing at all three of its sources** -
   `ee9b75a` (docs)
2. **Task 2: Confirm the ADR allocation before any number is burnt** - no commit. Task 2 is a pure
   decision gate with no `<action>`/`<files>` of its own; its resolution is recorded here and in
   this SUMMARY's frontmatter, per the plan's own task definition and the continuation agent's
   resume instructions. No file in this plan's `<files_modified>` list changes as a result of the
   decision itself — the allocation takes effect when plans 11-02/11-03 write ADR-0034/0035 in
   wave 2.

**Plan metadata:** this SUMMARY's own commit (below) — `docs(11-01): create plan summary`

_Note: no TDD tasks in this plan (D-13: zero `.rs` files touched)._

## Checkpoint Resolution — Task 2

**Decision:** option-a — **Two ADRs.** `ADR-0034` = the D1–D4 disposition set (authored by plan
11-02), `ADR-0035` = the `paladin-ml` leaf-crate placement condition (authored by plan 11-03).
`PROMOTION.md`'s "Next free ADR number" advances to **0036**, amended last by plan 11-05 per D-14
step 5.

**Selection mechanism:** auto-selected by the orchestrator under `AUTO_MODE=true`. This plan's
`<checkpoint_handling>` instructions explicitly required the executing agent not to self-select at
the checkpoint and to return control to the orchestrator instead — the selection recorded here is
the orchestrator's, not an independent judgment made inline by this agent.

**Rationale carried into the phase record:** option-a matches D-14's own framing of "the D1–D4
disposition set" as one contested bundle, and RESEARCH.md's Open Question 2 recommendation to fold
D2's `content_service.rs`/`event_manager.rs` verdicts into the same ADR rather than minting a
fifth. It keeps plans 11-02 and 11-03 parallel in wave 2 (two subjects, two ADRs — "exactly one
live ADR answers each question" per `PROMOTION.md`).

**Accepted consequence, recorded rather than hidden:** a future phase that wants to revisit only
D3's verdict must supersede an ADR that also carries D1, D2 and D4 — a coarser unit of supersession
than one-per-item would have given. This trade-off was explicit in option-a's own `<cons>` and is
recorded here as an accepted cost, not a hidden one.

**⚠ The unratified-ADR-0031 dependency survives this resolution, not laundered by it.** ADR-0031 —
the leaf-to-leaf invariant that legalises D3/D4's relocation targets, and the authority
`ADR-0035` (under option-a) and `ADR-0034` both carry forward — was authored under Phase 10
`--auto` and is one of two decisions Phase 10 explicitly flagged `⚠ HUMAN REVIEW` (D-00m). Nothing
in this checkpoint's resolution ratifies it. It remains unconfirmed by a human, and any future
phase that *executes* a D3/D4 relocation on ADR-0031's authority should confirm it first. Plan
11-02 (which drafts `ADR-0034` under D-04, `defer` for D1/D3/D4) inherits a prohibition against
treating this checkpoint as ADR-0031 ratification, and this note exists so the dependency is
visible at phase verification rather than requiring re-derivation from Phase 10's records.

## Files Created/Modified

- `.planning/registers/facade-01-rustdoc-stdout-disposition.md` - New per-occurrence D5 disposition
  register; per-file tables, citation index, Phase 15 hand-off, corpus-level finding
- `.project/Milestone_8-Facade-Cleanup-Shim-Resolution/deferred-items.md` - Dated Shape-A
  correction on the D5 clause (rating, recommendation, quick-wins line), original text retained
- `.planning/ROADMAP.md` - §Phase 11 criterion 1 amended in place, original retained
- `.planning/ledgers/milestone-07-08.md` - `REQ-m8-deferred-items-register` row's Evidence cell
  amended in place with the D5 disposition and a `D1–D4: pending — plan 11-05` marker

## Decisions Made

- **ADR allocation for wave 2: option-a** (two ADRs, 0034/0035, next free → 0036) — see
  `## Checkpoint Resolution — Task 2` above for full rationale, accepted trade-off, and the
  carried-forward ADR-0031 caveat.
- No decision was made to convert any of the 17 D5 occurrences to `log::*` — D-01 established the
  exception branch fires for all 17, so the only decision executed was to annotate, not convert.

## Deviations from Plan

**None — plan executed exactly as written**, including its checkpoint. Task 1's verification loop
and all listed acceptance criteria were re-run and passed without needing a Rule 1/2/3 auto-fix.
The register's initial draft required one self-correction before commit: the plan's own `<verify>`
script checks for literal `file:line` strings (e.g. `herald_registry.rs:165`) rather than bare line
numbers under a file heading, so a citation index and full `file:line` fence references were added
to the register to satisfy the automated verify loop — this is a formatting adjustment to meet the
plan's own stated verification command, not a deviation in scope or content.

## Issues Encountered

None. Both D5 evidence-bar commands, all 17 citations, the fence-line off-by-two guard (Pitfall 3:
163/182/195/208 vs. the correct 165/184/197/210, this register uses only the correct set and cites
163/182/196/208 solely as fence-line references, never as occurrence lines), and the ledger's
86-row invariant all held on first re-measurement.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Wave 2 unblocked.** Plans 11-02 (FACADE-02, `ADR-0034`) and 11-03 (FACADE-03, `ADR-0035`) can
  now execute with a confirmed ADR allocation; both should re-read `PROMOTION.md`'s "Next free ADR
  number" line at execution time per the plan's own resume-signal instruction, and record the
  number they actually took in their own SUMMARY.
- **Plan 11-04** (FACADE-04) is unaffected by this plan's outputs and can run in parallel with
  11-02/11-03 in wave 2, as already scheduled.
- **Plan 11-05** (wave 3 close-out) inherits: the `D1–D4: pending` marker on
  `REQ-m8-deferred-items-register` to resolve once 11-02 lands; `PROMOTION.md`'s "Next free ADR
  number" update as its own last act (0036), per D-14; and a `## Key Decisions` row for the ADR
  allocation.
- **No blocker.** FACADE-01 is fully closed by this plan; nothing further is owed to it.

---
*Phase: 11-facade-residue-deferred-register-disposition*
*Completed: 2026-08-09*
