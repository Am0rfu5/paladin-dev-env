---
phase: 07-workspace-ground-truth-recorded-answers
plan: 06
subsystem: docs
tags: [ledger, adr, workspace, paladin-core, paladin-ports, evidence-bar]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers (plan 07-01)
    provides: "milestone-04-06.md scaffold with all 115 REQ-* row stubs, plus ADR-0009 citation groundwork"
  - phase: 07-workspace-ground-truth-recorded-answers (plan 07-02)
    provides: "ADR-0016 and the .project/ prd-paladin-ports-extraction.md FR-7/FR-10 annotation this plan cites"
  - phase: 07-workspace-ground-truth-recorded-answers (plan 07-04)
    provides: "ADR-0015 (paladin-core / paladin-ports dependency allowlist) this plan cites"
provides:
  - "20 verdicted, freshly-cited ledger rows for Milestone 5 Epic 1 (Workspace Initialization & paladin-core) and Epic 2 (paladin-ports Extraction)"
  - "Fresh re-grep of the crate-isolation CI job (ci.yml:304, not the stale 228) and the paladin-ports dependency count (11, not the stale 10)"
affects: [07-13-summary-and-bookkeeping, phase-08-verified-defect-closure]

# Tech tracking
tech-stack:
  added: []
  patterns: ["ledger row evidence cells cite file:line plus a scoped cargo test/doc/tree command with pass count, or a manifest line plus its consuming CI job (D-01 manifest carve-out)"]

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-04-06.md

key-decisions:
  - "REQ-workspace-crate-edition-v1, REQ-paladin-core-dependency-allowlist-v1 and REQ-port-value-type-ownership-v1/-v2 recorded superseded by shipped code, citing ADR-0009/ADR-0015/ADR-0016 respectively rather than re-deciding any of the three pairs"
  - "REQ-ports-doctest-compilation recorded genuinely outstanding (Phase 8 / DEBT-03); REQ-ports-tests-and-rustdoc recorded present, unproven for the same underlying doctest-disablement reason, distinguished because its rustdoc text and intra-doc-link integrity are separately proven"
  - "REQ-core-container-extraction's Maneuver-DSL clause recorded satisfied with an inline note pointing at REQ-maneuver-files-moved-from-core's row (filled by plan 07-01) rather than reopened as a nested outstanding item"

requirements-completed: [ARCH-01, ARCH-03]

coverage: []

# Metrics
duration: ~55min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 6: Milestone 5 Epic 1-2 Ledger Rows Summary

**Verdicted all 20 REQ-* rows for Milestone 5 Epic 1 (workspace init, `paladin-core`) and Epic 2 (`paladin-ports` extraction) in `.planning/ledgers/milestone-04-06.md`, re-grepping every citation fresh against the tree and running scoped `cargo test`/`doc`/`tree` commands rather than trusting any earlier document's line numbers.**

## Performance

- **Duration:** ~55 min
- **Started:** 2026-08-06T17:45:00Z (approx.)
- **Completed:** 2026-08-06T18:42:00Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- Filled all ten Milestone 5 Epic 1 rows: `REQ-cargo-workspace-root`, `REQ-paladin-core-scaffold`, `REQ-core-base-extraction`, `REQ-core-container-extraction`, `REQ-core-upward-dependency-resolution`, `REQ-facade-core-reexports` and `REQ-core-dependency-validation` verdicted `satisfied`; `REQ-workspace-crate-edition-v1` and `REQ-paladin-core-dependency-allowlist-v1` verdicted `superseded by shipped code` citing ADR-0009 and ADR-0015; `REQ-port-value-type-ownership-v1` verdicted `satisfied` citing ADR-0016.
- Filled all ten Milestone 5 Epic 2 rows: `REQ-output-ports-extraction`, `REQ-input-ports-extraction`, `REQ-ports-facade-wiring`, `REQ-ports-import-migration`, `REQ-ports-docs-markdown-update` and `REQ-ports-layering-validation` verdicted `satisfied`; `REQ-paladin-ports-scaffold` and `REQ-port-value-type-ownership-v2` verdicted `superseded by shipped code` (ADR-0015, ADR-0016); `REQ-ports-doctest-compilation` verdicted `genuinely outstanding` (Phase 8 / DEBT-03); `REQ-ports-tests-and-rustdoc` verdicted `present, unproven` for the same underlying reason.
- Re-grepped the `crate-isolation` CI job fresh — confirmed at `ci.yml:304`, not the stale `:228` `intel/code-verification.md` carries — and measured `paladin-ports` at 11 dependencies (the corrected figure ADR-0015 already recorded; `intel/code-verification.md`'s figure of 10 predates `mime_guess`).
- Ran and recorded pass counts for every scoped command this task executed: `cargo test --offline -p paladin-ai-core --lib base::` (50 passed), `platform::container::` (316 passed), `cargo test --offline -p paladin-ports --lib output::` (86 passed), `input::` (12 passed), `cargo test --offline --test unit maneuver` (21 passed), `cargo check --offline --workspace --lib` (0 errors), `cargo doc --offline -p paladin-ai-core --no-deps` and `-p paladin-ports --no-deps` under `RUSTDOCFLAGS="-D warnings"` (both 0 warnings), and `cargo tree --offline -p paladin-ai-core` / `-p paladin-ports` (both zero matches for forbidden dependency classes).

## Task Commits

Both tasks share one ledger commit, per this plan's explicit instruction ("Commit the ledger once at the end of the plan... Do not commit per row."):

1. **Task 1: Milestone 5 Epic 1 — Workspace Initialization and `paladin-core` (10 rows)** — part of `18dbc34`
2. **Task 2: Milestone 5 Epic 2 — `paladin-ports` Extraction (10 rows)** — part of `18dbc34`

**Commit:** `18dbc34` — `docs(07-06): verdict M5 Epic 1-2 ledger rows (workspace init, paladin-ports)`

## Files Created/Modified

- `.planning/ledgers/milestone-04-06.md` — 20 Verdict/Evidence cells replaced in place under `### Milestone 5 Epic 1` and `### Milestone 5 Epic 2`; no rows inserted, deleted or reordered.

## Decisions Made

- **`REQ-core-container-extraction`'s Maneuver-DSL clause** recorded `satisfied` with an inline note pointing at `REQ-maneuver-files-moved-from-core`'s row (Milestone 6 Epic 3, filled by plan 07-01) for the later partial supersession, rather than reopened as a nested outstanding item — matches this plan's explicit instruction.
- **`REQ-ports-facade-wiring`** recorded `satisfied` rather than `diverged`: the `.project/…prd-paladin-ports-extraction.md` §9 Resolved Design Decision 2 ("Full deletion selected (Option B)... no shim debt") is itself part of the same source document and supersedes FR-17's earlier literal "re-export at old paths" text — shipped code (full deletion of `src/application/ports/`, `paladin_ports::` imported directly at call sites, convenience re-exports centralized in `src/prelude.rs` rather than `src/lib.rs`) matches the document's own later resolution, not a divergence from it.
- **`REQ-ports-tests-and-rustdoc`** recorded `present, unproven` rather than collapsing it into the same `genuinely outstanding` verdict as `REQ-ports-doctest-compilation`: the rustdoc text itself (6683 doc-comment lines) and its structural intra-doc-link integrity (`cargo doc --no-deps` under `-D warnings`, 0 warnings) are both freshly proven this task, so only the doctest-driven example-code correctness is unproven — the two rows share a root cause (doctests disabled) but earn different verdicts under D-01's bar.
- **Historical `.project/` dependency-validation artifacts were found locatable**, contrary to this plan's own cautious framing ("the Epic 1 dependency-validation evidence artefacts are the likely case" for `project/`→`.project/` rename casualties): `.project/Milestone_5-Workspace-Decomposition/Epic_1/paladin-core-dependency-tree.txt` and `baseline-test-count.txt` both exist under `.project/`. They were not used as the row's evidence anyway — they are pre-rename-era snapshots (`v0.1.0`, a since-removed `fasthash` dependency) rather than current evidence, so `REQ-core-dependency-validation`'s verdict rests on this task's fresh `cargo tree`/`cargo doc` commands instead.

## Deviations from Plan

None — plan executed exactly as written. All prescribed dispositions (edition → ADR-0009, allowlist → ADR-0015, port ownership → ADR-0016, Maneuver inline note, `present, unproven` framing for any unlocatable `.project/` artefact) were followed; no row required a Rule 1-4 deviation.

## Issues Encountered

- A background `cargo test --offline --test unit paladin_domain` invocation (an unnecessary extra verification I attempted for `REQ-facade-core-reexports`, targeting a test binary name that does not exist) triggered a full-workspace cold compile and hit the 120s foreground timeout, auto-backgrounding. It was killed once discovered (`pkill`) rather than left running; no output or side effect from it was used as evidence. The row's actual evidence instead reuses the already-passing `cargo test --offline --test unit maneuver` run (21 passed), which exercises the same `src/core/platform/mod.rs` re-export block.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- 20 of the ledger's 115 rows now verdicted by this plan; combined with plan 07-01's 9-row Milestone 6 Epic 3 tracer, 29 rows total are cited. 86 rows remain `PENDING-VERDICT` across the other ten epic sections, to be filled by plans 07-08 (M6 Epic 1/2/4), 07-10 (M5 Epic 3/4), 07-11 (M5 Epic 5/6), and 07-12 (M4 Epic 1-3).
- Row count remains exactly 115; no duplicate `REQ-*` IDs introduced; no `*.rs`, `Cargo.toml` or `.github/` file touched by this plan's commit.
- `REQ-ports-doctest-compilation` and `REQ-ports-tests-and-rustdoc` hand Phase 8 / DEBT-03 a scoped target (re-enable `paladin-ports`'s doctests, remove the `ci.yml:226` `--exclude paladin-ports`) with no code change made here, consistent with this phase's record-only boundary.

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-04-06.md`
- FOUND: `.planning/phases/07-workspace-ground-truth-recorded-answers/07-06-SUMMARY.md`
- FOUND: commit `18dbc34` (ledger rows)
- FOUND: commit `6254182` (this summary)
