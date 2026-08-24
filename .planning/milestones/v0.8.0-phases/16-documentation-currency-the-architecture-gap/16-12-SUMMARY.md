---
phase: 16-documentation-currency-the-architecture-gap
plan: 12
subsystem: docs
tags: [rustdoc, doctests, heading-convention, DOCS-03, gate-evidence]

# Dependency graph
requires:
  - phase: 16-07
    provides: "Workspace-wide cargo doc gate held at zero warnings"
  - phase: 16-08
    provides: "16-DOCS-03-ENTRY-POINTS.md (D-05 enumeration, 76 items) and
       scripts/check-public-api-examples.sh (the gate); recorded the D-06 plural
       heading rule in .planning/codebase/CONVENTIONS.md"
  - phase: 16-11
    provides: "Zero MISSING D-05 entry points tree-wide (76/76 have an example
       block); left exactly 6 SINGULAR rows in src/ for this plan's D-06 sweep"
provides:
  - "76/76 D-05 entry points carry the plural `# Examples` heading — the last 6
     SINGULAR rows normalised (PaladinBuilder, ArsenalRegistryService,
     ArsenalExecutionService, HandoffService, PaladinExecutionService,
     EncryptionService)"
  - "bash scripts/check-public-api-examples.sh (default gating mode) exits 0
     for the first time this phase — 76 OK, 0 MISSING, 0 SINGULAR"
  - "Closing gate evidence recorded verbatim in 16-DOCS-03-GATE-EVIDENCE.md:
     cargo doc --workspace --no-deps (ci.yml:63's exact command, 0 warnings),
     the examples gate (0 exit), and cargo test --workspace --doc (318 passed,
     0 failed)"
  - "16-DOCS-03-ENTRY-POINTS.md updated in place with end-of-phase closing
     totals, opening figures kept visible above them"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Heading normalisation targets the item's own resolved doc block only —
       the same block scripts/check-public-api-examples.sh's own_doc_block/
       module_doc_block functions resolve against, not every '# Example'
       occurrence in the file. Two of the six sites (PaladinExecutionService,
       EncryptionService) have no heading in their own preceding /// block, so
       the gate falls back to the file's leading //! module doc — the fix
       targeted that module-doc heading line, not the struct's own doc block."

key-files:
  created: []
  modified:
    - src/application/services/paladin/paladin_builder.rs
    - src/application/services/arsenal/arsenal_registry_service.rs
    - src/application/services/arsenal/arsenal_execution_service.rs
    - src/application/services/paladin/handoff_service.rs
    - src/application/services/paladin/paladin_execution_service.rs
    - src/infrastructure/security/encryption.rs
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-GATE-EVIDENCE.md
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-ENTRY-POINTS.md

key-decisions:
  - "Scoped every edit strictly to the 6 rows scripts/check-public-api-examples.sh
     --list reported as SINGULAR, confirmed by running the script before touching
     any file (baseline: 76 OK -> 70 OK/0 MISSING/6 SINGULAR matched the plan's
     prior-wave 6-row list exactly). Each of the 6 touched files carries many
     other '# Example' headings on individual methods (paladin_builder.rs alone
     has 36 occurrences) — those are out of scope by design (D-06) and were left
     untouched; only the heading the gate script's own_doc_block/module_doc_block
     logic resolves against for the pub struct/module was changed."
  - "For PaladinExecutionService and EncryptionService, the struct's own preceding
     doc block has no '# Example'/'# Examples' heading at all (PaladinExecutionService's
     own block has '# Features'/'# Thread Safety'; EncryptionService's own block has
     '# Framework usage') — the gate's own_doc_block check finds nothing, falls back
     to the file's leading //! module doc, and that module doc's heading (line 16 and
     line 13 respectively) is what the gate actually classifies as SINGULAR. The fix
     targeted those module-doc lines, not the struct doc blocks, matching the gate's
     own resolution order exactly."
  - "Left the 76-vs-79 D-05 arithmetic delta (2 unattributed *Service items, recorded
     in 16-08's enumeration) unresolved rather than closing it by adjustment. No new
     evidence surfaced during this plan's normalisation work that attributes either
     item to a specific excluded declaration."

requirements-completed: [DOCS-03]

coverage:
  - id: D1
    description: "The 6 remaining SINGULAR D-05 entry points normalised to the
       plural '# Examples' heading, scoped to exactly those 6 sites"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh (gate mode): exit 0, '76 D-05 public API entry points carry a plural # Examples heading'"
        status: pass
      - kind: other
        ref: "git diff --numstat -- crates/ src/: 6 files, all appear in 16-DOCS-03-ENTRY-POINTS.md"
        status: pass
    human_judgment: false
  - id: D2
    description: "Non-enumerated heading sites left untouched — tree-wide singular
       count fell by exactly 6 (the enumerated rows changed), not more"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "grep -rnE singular-heading count: 225 -> 219 (-6); plural: 224 -> 230 (+6)"
        status: pass
      - kind: other
        ref: "git diff --exit-code scripts/check-public-api-examples.sh: exit 0 (gate script untouched)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Both closing gates green with verbatim output recorded, examples
       proven executable, no workflow file modified, no suppression used"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo doc --workspace --no-deps (ci.yml:63 exact command): exit 0, 0 warnings"
        status: pass
      - kind: unit
        ref: "cargo test --workspace --doc: 318 passed, 0 failed, 205 ignored"
        status: pass
      - kind: other
        ref: "git diff --exit-code .github/workflows/: exit 0; git diff -U0 -- '*.rs' | grep -c '^+.*allow(rustdoc': 0"
        status: pass
    human_judgment: false

duration: 30min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 12: D-06 Heading Normalisation and Closing Gate Evidence for DOCS-03 Summary

**Normalised the last 6 SINGULAR `# Example` headings to the plural `# Examples` form on their D-05-enumerated entry points, taking `scripts/check-public-api-examples.sh` to 76/76 OK for the first time this phase, and recorded both gates' closing output verbatim, closing DOCS-03.**

## Performance

- **Duration:** ~30 min
- **Completed:** 2026-08-24
- **Tasks:** 2 (Task 1: heading normalisation; Task 2: closing gate evidence)
- **Files modified:** 8 (6 source, 2 phase-tracking docs)

## Accomplishments

- **6 D-05 entry points' heading spelling normalised to plural `# Examples`**, scoped strictly to
  the enumerated `file:line` rows 16-11 handed forward — no tree-wide sweep:
  - `PaladinBuilder` (`src/application/services/paladin/paladin_builder.rs:61`, own doc block)
  - `ArsenalRegistryService` (`src/application/services/arsenal/arsenal_registry_service.rs:18`, own doc block)
  - `ArsenalExecutionService` (`src/application/services/arsenal/arsenal_execution_service.rs:51`, own doc block)
  - `HandoffService` (`src/application/services/paladin/handoff_service.rs:25`, own doc block)
  - `PaladinExecutionService` (`src/application/services/paladin/paladin_execution_service.rs:16`, file `//!` module doc — its own struct doc block has no Example heading at all, so the gate's own resolution falls back to the module doc)
  - `EncryptionService` (`src/infrastructure/security/encryption.rs:13`, file `//!` module doc — same fallback shape)
- **`bash scripts/check-public-api-examples.sh` (default gating mode) exits 0** — `All 76 D-05
  public API entry points carry a plural '# Examples' heading.` First time this exact command has
  passed anywhere in the phase.
- **`bash scripts/check-public-api-examples.sh --list` closing totals: `TOTAL: 76 entry points --
  76 OK, 0 MISSING, 0 SINGULAR`** — both halves of the D-05/D-06 gate (MISSING, closed by
  16-09→16-11; SINGULAR, closed by this plan) are now fully closed.
- **Boundary proven to have held exactly:** tree-wide count of the singular `# Example` heading
  form across `crates/` and `src/` fell from **225 to 219** (−6), and the plural count rose from
  **224 to 230** (+6) — precisely the 6 enumerated rows this plan changed, confirming the roughly
  219 remaining non-enumerated singular-heading sites were left untouched by design.
- **`cargo doc --workspace --no-deps` (the exact `ci.yml:63` command): 0 warnings**, holding the
  bar 16-07 established. `git diff --exit-code .github/workflows/` exits 0 — no workflow file
  touched anywhere in this plan.
- **`cargo test --workspace --doc`: 318 passed, 0 failed, 205 ignored** across 12 doctest-bearing
  crates — identical to the pre-existing figure at this plan's spawn, confirming the heading-only
  edits added zero new doctests and broke none.
- **`cargo test -p paladin-web openapi`: 6 passed**, including `openapi_matches_committed_baseline`
  — run as a sanity check even though none of the 6 touched files carry a `#[utoipa::path]`
  attribute (confirmed by grep), so the OpenAPI-coupling risk this plan's prior-wave context
  flagged did not apply here.
- **`cargo fmt --check`: clean** after both tasks.
- **Closing evidence appended verbatim to `16-DOCS-03-GATE-EVIDENCE.md`** under a dated "Closing"
  section: the three closing checks' commands, output, and exit statuses, plus the two
  inherited-not-delivered clauses (D-00u — the CI gate mechanism pre-existed at `ci.yml:63`; M-02
  — `missing_docs` was already clean workspace-wide before phase 16 began).
- **`16-DOCS-03-ENTRY-POINTS.md` updated in place** with an end-of-phase "Closing totals — end of
  phase" section below the original D-05 baseline figures (kept visible, not overwritten): 76/76
  have an example, 76/76 carry the plural heading, and the compile-and-run vs. `no_run` split
  aggregated across plans 16-09→16-11 (38/38 new examples compile-and-run, 0 non-running fences
  introduced across the whole examples wave).
- **The 76-vs-79 D-05 arithmetic delta is left recorded and unresolved**, exactly as instructed —
  no new evidence surfaced during this plan's work to attribute either of the 2 remaining
  unaccounted `*Service` items to a specific excluded declaration.

## Task Commits

Each task was committed atomically with `--no-verify` (D-00o):

1. **Task 1: Normalise the heading spelling on the enumerated entry points, and only there** -
   `ca5ee92d` (docs) — 6 files, one heading line changed per file (12 removed / 12 added lines in
   `git diff -U0`, counting file-header pairs: a one-for-one swap with no documentation removed).
2. **Task 2: Prove both gates green at the phase's closing state and record the evidence
   verbatim** - `cfe3863e` (docs) — 2 files, 160 insertions (closing evidence section + closing
   totals section), 0 deletions.

**Plan metadata:** this SUMMARY is handled by the orchestrator after wave merge (worktree mode,
per execute-plan.md) — STATE.md/ROADMAP.md are NOT touched by this executor.

## Files Created/Modified

### Task 1 (6 files)
- `src/application/services/paladin/paladin_builder.rs` — `PaladinBuilder`'s own doc block
  heading, line 61.
- `src/application/services/arsenal/arsenal_registry_service.rs` — `ArsenalRegistryService`'s own
  doc block heading, line 18.
- `src/application/services/arsenal/arsenal_execution_service.rs` — `ArsenalExecutionService`'s
  own doc block heading, line 51.
- `src/application/services/paladin/handoff_service.rs` — `HandoffService`'s own doc block
  heading, line 25.
- `src/application/services/paladin/paladin_execution_service.rs` — file `//!` module doc heading,
  line 16 (the gate resolves against this since `PaladinExecutionService`'s own doc block has no
  Example heading).
- `src/infrastructure/security/encryption.rs` — file `//!` module doc heading, line 13 (same
  fallback shape as above, for `EncryptionService`).

### Task 2 (2 files)
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-GATE-EVIDENCE.md` —
  appended the phase's closing evidence (three checks, verbatim commands/output/exit statuses,
  boundary-held arithmetic, two inherited-not-delivered clauses).
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-ENTRY-POINTS.md` —
  appended end-of-phase closing totals below the original D-05 baseline figures.

## Decisions Made

See `key-decisions` in frontmatter. Summarized:

- Edits scoped exclusively to the 6 rows `scripts/check-public-api-examples.sh --list` reported as
  SINGULAR at plan start — confirmed by running the script before touching any file. Every
  unrelated `# Example` heading in these same 6 files (individual method docs, e.g. 35 other
  occurrences in `paladin_builder.rs` alone) was left untouched by design.
- For 2 of the 6 sites (`PaladinExecutionService`, `EncryptionService`), the item's own preceding
  doc block carries no Example heading at all — the gate's `own_doc_block`/`module_doc_block`
  resolution order falls back to the file's leading `//!` module doc, so the fix targeted that
  module-doc line, matching the gate's own logic exactly rather than editing the struct's own doc
  block (which would have had no effect on the gate's classification).
- The 76-vs-79 D-05 delta was left unresolved rather than closed by adjusting the expected total —
  no attributing evidence surfaced during this plan.

## Deviations from Plan

None — plan executed exactly as written. Both tasks' acceptance criteria and `<verify>` blocks
passed on the first attempt; no auto-fix, no architectural question, no auth gate.

## Issues Encountered

None.

## Non-running fence audit

Zero fences were touched in either task. Task 1 changed only heading text (`# Example` →
`# Examples`); the code fence type on each edited block (```rust```, ```rust,no_run```,
```rust,ignore```) is unchanged. Confirmed: `git diff -U0 -- crates/ src/ | grep -c
'rust,ignore\|rust,text'` returns 0 new occurrences (the two pre-existing `no_run`/`ignore` fences
among the 6 touched files — `paladin_builder.rs`'s file-level module doc and
`arsenal_execution_service.rs`'s own doc block — were already that way before this plan; neither
fence type was added or removed).

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- **DOCS-03 is closed.** All three of its constituent halves are satisfied: `missing_docs` was
  already clean (M-02, inherited), the CI gate already existed at `ci.yml:63` (D-00u, inherited),
  and the `# Examples` requirement (D-05's 76-item enumeration, D-06's heading rule) is now fully
  closed at 76/76 OK, 0 MISSING, 0 SINGULAR.
- Both gates — `cargo doc --workspace --no-deps` and `scripts/check-public-api-examples.sh` — are
  green with verbatim closing evidence recorded in `16-DOCS-03-GATE-EVIDENCE.md`.
- `cargo test --workspace --doc` (318 passed) proves every example this phase's wave (16-09→16-12)
  added is executable, not decorative.
- The 76-vs-79 D-05 arithmetic delta remains an open, explicitly-recorded item — not a blocker for
  DOCS-03's closure (the definition applied literally and exhaustively produces 76; the 79 figure
  in FR-26.3 predates any independent grep-based re-derivation), but worth a future ADR amendment
  if the 2 unaccounted items are ever identified.
- No blockers for phase closure.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*
