---
phase: 07-workspace-ground-truth-recorded-answers
plan: 11
subsystem: docs
tags: [ledger, requirements-traceability, paladin-memory, ci, adr-0009, adr-0020]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: "Ledger scaffold (plan 07-01) with 115 REQ-* row stubs; ADR-0009 (Phase 4) and ADR-0020 (plan 07-07) already on disk to cite"
provides:
  - "16 verdicted, freshly-cited REQ-* rows under Milestone 5 Epic 5 (paladin-memory extraction) and Epic 6 (workspace finalization) in .planning/ledgers/milestone-04-06.md"
  - "The ledger's canonical citation-drift demonstration: REQ-crate-isolation-ci re-grepped at ci.yml:304, documenting the 76-line drift from the stale ci.yml:228 figure"
affects: [07-13, 08-plan-phase]

# Tech tracking
tech-stack:
  added: []
  patterns: ["scoped offline cargo test/check runs as the exercising artefact for satisfied rows, quoting pass counts inline"]

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-04-06.md

key-decisions:
  - "REQ-memory-originals-deletion verdicted satisfied on the requirement's actual text (monolith implementation deleted, files replaced in place with facade re-exports per the PRD's own FR-6.1 table), not on this plan's own must_haves prose (\"directories… are gone\"), which does not hold literally — both src/infrastructure/adapters/{garrison,sanctum}/ directories still exist as re-export-only shims."
  - "REQ-workspace-ci-upgrade verdicted deferred with reason (not satisfied/genuinely outstanding) since one of three clauses ships and two remain open with named owners (Phase 15/PIPE-04, Phase 8/DEBT-03)."
  - "REQ-build-benchmark-report verdicted satisfied for the deliverable per plan instruction, citing ADR-0020's contested per-scenario verdict in the same cell rather than re-deriving or reconciling it."
  - "REQ-memory-build-gates verdicted present, unproven: 3 of 5 FR-9 build combinations have a direct CI leg (crate-isolation job); sqlite-alone and qdrant-alone are not individually isolated anywhere, matching the ingest-era caveat."

requirements-completed: [ARCH-01, ARCH-03, ARCH-07]

coverage:
  - id: D1
    description: "10 Milestone 5 Epic 5 (paladin-memory extraction) rows verdicted with fresh file:line citations and scoped offline test evidence"
    requirement: "ARCH-01"
    verification:
      - kind: other
        ref: "sed -n '/^### Milestone 5 Epic 5 /,/^### Milestone 5 Epic 6 /p' .planning/ledgers/milestone-04-06.md | grep -c '^| REQ-' == 10, grep -c PENDING-VERDICT == 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "6 Milestone 5 Epic 6 (workspace finalization) rows verdicted, including the canonical crate-isolation citation-drift demonstration and the ADR-0020-cited build-benchmark row"
    requirement: "ARCH-03"
    verification:
      - kind: other
        ref: "sed -n '/^### Milestone 5 Epic 6 /,/^### Milestone 6 Epic 1 /p' .planning/ledgers/milestone-04-06.md | grep -c '^| REQ-' == 6, grep -c 'ci.yml:228' (whole file) == 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "Stub arithmetic assertion: PENDING-VERDICT count drops from 41 to 25, total row count stays 115, no duplicate REQ-* IDs"
    requirement: "ARCH-01"
    verification:
      - kind: other
        ref: "grep -c PENDING-VERDICT .planning/ledgers/milestone-04-06.md == 25; grep -c '^| REQ-' == 115; grep -o '^| REQ-[a-z0-9-]*' | sort | uniq -d prints nothing"
        status: pass
    human_judgment: false
  - id: D4
    description: "REQ-crate-isolation-ci's evidence cell cites the current line the crate-isolation job resolves to (304), not the stale ingest-era 228, and human verification of the job's shape is requested per 07-VALIDATION.md"
    requirement: "ARCH-01"
    verification: []
    human_judgment: true
    rationale: "The plan's own <verify> block explicitly requires a human to open ci.yml:304 and confirm it is the per-crate isolated build matrix the row claims, not a differently-scoped job at a nearby line — this is a designated manual-only verification (07-VALIDATION.md §Manual-Only Verifications, row 1), not something this plan can auto-pass."

# Metrics
duration: ~50min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 11: Milestone 5 Epic 5-6 Ledger Verdicts Summary

**Verdicted 16 `.planning/ledgers/milestone-04-06.md` rows for `paladin-memory` extraction and workspace finalization, re-grepping the drifted `crate-isolation` CI citation (228 → 304) and citing ADR-0009/ADR-0020 rather than re-deriving their answers.**

## Performance

- **Duration:** ~50 min
- **Completed:** 2026-08-06T19:48:11Z
- **Tasks:** 2 (Task 1: Epic 5, 10 rows; Task 2: Epic 6, 6 rows)
- **Files modified:** 1 (`.planning/ledgers/milestone-04-06.md`)

## Accomplishments

- Filled all 10 Milestone 5 Epic 5 (`paladin-memory` extraction) rows with fresh `file:line` citations, each `satisfied` row backed by a named `cargo test --offline` run with a quoted pass count (70 unit tests in `paladin-memory --lib`, plus 38 combined facade-path integration-test passes across `paladin_garrison_integration`, `in_memory_sanctum_integration` and `rag_integration --features qdrant`).
- Filled all 6 Milestone 5 Epic 6 (workspace finalization) rows, including this ledger's canonical citation-drift demonstration: `REQ-crate-isolation-ci` now cites `ci.yml:304`, re-grepped fresh this task, replacing the stale `ci.yml:228` figure REQUIREMENTS.md and `intel/code-verification.md` both carried forward — a documented 76-line drift.
- `REQ-workspace-crate-edition-v2` confirms all **12** workspace manifests (root + 11 crates) declare `edition = "2024"`, citing ADR-0009 rather than re-adjudicating.
- `REQ-build-benchmark-report` cites ADR-0020's per-scenario restatement (two of five scenarios pass the ≥ 50% target, three fail; the report's own "Target achieved" conclusion is recorded as contradicted by its own table) instead of re-deriving the answer.
- `REQ-workspace-ci-upgrade` addresses all three clauses in one cell — workspace scoping shipped, the toolchain-action sweep deferred to Phase 15/PIPE-04 (with its own smaller citation-drift found: 147→148, 317→393, 507→792), and the `paladin-ports` doctest exclusion deferred to Phase 8/DEBT-03.
- Stub arithmetic confirmed: `PENDING-VERDICT` count dropped from 41 to exactly 25; total row count held at 115; no duplicate `REQ-*` IDs.

## Task Commits

Both tasks landed in a single commit per the plan's explicit instruction ("Commit the ledger once at the end of the plan"):

1. **Task 1 + Task 2: Verdict M5 Epic 5-6 ledger rows** - `5fb1578` (docs)

_No separate plan-metadata commit was made — this plan modifies only the ledger file; STATE.md/ROADMAP.md updates are owned by the orchestrator per this plan's instructions._

## Files Created/Modified

- `.planning/ledgers/milestone-04-06.md` - 16 `REQ-*` rows under `### Milestone 5 Epic 5` and `### Milestone 5 Epic 6` replaced in place (Verdict + Evidence cells only; no rows inserted, deleted, or reordered); two epic-level notes added.

## Decisions Made

- **REQ-memory-originals-deletion verdicted `satisfied` on the requirement's actual text, not this plan's own must_haves phrasing.** The plan's `must_haves.truths` and `acceptance_criteria` both assert `test -d src/infrastructure/adapters/{garrison,sanctum}` fails (directories gone). Direct inspection this task found both directories still exist — but as re-export-only shims (`git log` shows the exact deleting commit, `d44ed67 feat: delete originals and add facade re-exports (Task 7.0)`), and the Epic 5 PRD's own FR-6.1 table specifies `mod.rs` files are "replaced by facade re-exports" at the **same path**, never directory removal. The row is verdicted `satisfied` against the PRD's real requirement; the directory-absence framing in this plan's own text is flagged as a plan-authoring imprecision, not a code defect.
- **REQ-workspace-ci-upgrade verdicted `deferred with reason`**, not `satisfied` or `genuinely outstanding` — one of three clauses (workspace scoping) ships, two (toolchain-action sweep, doctest exclusion) remain open with two different named owning phases, and the D-02 vocabulary's `deferred with reason` is the closest fit for a partially-shipped, partially-owned-elsewhere row.
- **REQ-build-benchmark-report verdicted `satisfied` for the deliverable**, per the plan's explicit instruction, with the contested verdict (report's own table vs. its "Target achieved" conclusion) recorded in the same Evidence cell rather than reconciled.
- **REQ-memory-build-gates verdicted `present, unproven`**: the `crate-isolation` job's `paladin-memory` leg directly exercises 3 of 5 named FR-9 build combinations (default, no-default-features, all-features); `content-processing` has its own `feature-flags.yml` leg; `sqlite` rides along unconditionally via the root manifest's hard dependency feature but has no dedicated isolated leg; `qdrant` alone is never isolated outside `--all-features`. Both unconfirmed legs are named explicitly in the row.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Plan text vs. ground truth] REQ-memory-originals-deletion's must_haves/acceptance_criteria assert a directory-absence check that does not hold**
- **Found during:** Task 1 (`REQ-memory-originals-deletion` row)
- **Issue:** This plan's `must_haves.truths` and `acceptance_criteria` both specify `test -d src/infrastructure/adapters/garrison` and `sanctum` must fail (directories gone). Both directories exist — they were never removed, only their contents replaced in place with facade re-export shims, exactly as the Epic 5 PRD's FR-6.1 table specifies ("replaced by facade re-exports").
- **Fix:** Verdicted the row `satisfied` against the requirement's actual text (monolith implementation deleted, confirmed via `git log --follow` showing the deleting commit and via reading both `mod.rs` files' current re-export-only content), with the plan-text mismatch documented explicitly inside the Evidence cell itself so a later reader does not draw the wrong conclusion from the row's own `must_haves` prose.
- **Files modified:** `.planning/ledgers/milestone-04-06.md` (row content only; no code changed — this is a records phase)
- **Verification:** `git log --follow --oneline -- src/infrastructure/adapters/garrison/mod.rs` (commit `d44ed67`), direct read of both `mod.rs` files, and `prd-paladin-memory-extraction.md:250,253,256`'s FR-6.1 table, all cited inline in the row.
- **Committed in:** `5fb1578` (part of the single plan commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 — plan text corrected against verified ground truth, not a code defect)
**Impact on plan:** No scope creep; the ledger row records the accurate, verified state of the tree rather than forcing a false claim to match an imprecise acceptance criterion written before this task's own directory inspection.

## Issues Encountered

None beyond the deviation documented above. Two `cargo test`/`cargo check` compiles were required to produce exercising artefacts for facade re-export rows (the root `paladin-ai` crate and its `--features qdrant` variant); both completed within the plan's Bash timeout budget on a cold worktree (no pre-existing `target/` directory), using the pre-warmed `~/.cargo/registry` cache — no network access was required.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `.planning/ledgers/milestone-04-06.md` now has 90 of 115 rows verdicted (25 `PENDING-VERDICT` remain, owned by other waves in this phase).
- `REQ-crate-isolation-ci`'s corrected `ci.yml:304` citation is available for any later row in this ledger (or a future phase) that also needs to cite the `crate-isolation` job — no other row should re-derive the line number; cite this row's citation or re-grep independently.
- `REQ-workspace-ci-upgrade`'s two named forward-owners (Phase 15/PIPE-04 for the toolchain-action sweep, Phase 8/DEBT-03 for the `paladin-ports` doctest exclusion) are ready inputs for those phases' own planning.
- `REQ-crate-isolation-ci`'s row carries a `human-check` per the plan's `<verify>` block — a human should open `ci.yml:304` and confirm it is the per-crate isolated build matrix before this row is treated as fully closed (07-VALIDATION.md §Manual-Only Verifications, row 1).

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-04-06.md`
- FOUND: `.planning/phases/07-workspace-ground-truth-recorded-answers/07-11-SUMMARY.md`
- FOUND: commit `5fb1578` (ledger row content)
- FOUND: commit `40fc623` (this summary)

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
