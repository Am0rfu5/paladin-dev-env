---
phase: 05-milestone-2-3-ground-truth
plan: 05
subsystem: docs
tags: [ledger, requirements-traceability, battalion, commander, grove, council, verification]

# Dependency graph
requires:
  - phase: 05-milestone-2-3-ground-truth
    provides: "05-01's ledger scaffold (head notes, verdict legend, 118 PENDING-VERDICT stub rows) and its D-01 evidence bar"
provides:
  - "Epic 22 block verdict for tasks-epic22-battalion-commander-hardening.md: satisfied by shipped code, with a 15-row parent-task cluster table backing it"
  - "Epic 22's 10 REQ-* ledger rows cited to the D-01 evidence bar (9 satisfied, 1 genuinely outstanding)"
  - "Phase 6 CLOSE-02 scope for this block: no work required (all 15 clusters verify)"
  - "REQ-grove-llm-routing recorded genuinely outstanding, owner Phase 6 / CLOSE-01, defect not fixed"
affects: [05-13, "06 (Verified Gap Closure)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Parent-task cluster verdict backed by a per-cluster capability check against the tree, never checkbox arithmetic (D-05/D-06)"
    - "Checkbox-vs-tree divergence recorded as 'satisfied by shipped code' with the shipping commit cited, when a source task list understates reality"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-02-03.md

key-decisions:
  - "Epic 22 block verdict: satisfied by shipped code. All 15 parent-task clusters (0.0-14.0) verify against the tree, including the three the source task list still marks open (3.0 Council registry, 4.0 Grove registry, 5.0 Grove LLM routing) — each shipped in its own commit (761c49c, 0cdf8dd, 5f05db7) with passing tests; only the task list's own checkboxes were never updated. Per D-06 this makes the block satisfied by shipped code, not partially outstanding, so Phase 6's CLOSE-02 has no work required for this block."
  - "REQ-grove-llm-routing verdicted genuinely outstanding, not present-unproven or satisfied. The LLM-routing capability itself ships and is fully tested, but the requirement's specific claim (routing honours the Paladin's configured provider) is unmet — grove_service.rs:537 hardcodes model: \"gpt-4\". Owner: Phase 6 / CLOSE-01. Not fixed here per this phase's prohibition on editing .rs files."
  - "REQ-commander-config-metadata-dir-v3 and the 8.0/9.0 cluster rows cite BattalionConfig, not the PRD's CommanderConfig — CommanderConfig never existed in the shipped tree, a fact already settled by the Milestone 1 as-shipped ledger's RECON-03 resolution and not re-decided here."
  - "REQ-battalion-metadata-extension cites battalion/mod.rs, not the PRD's battalion/battalion_result.rs — a relocation covered by this ledger's own head-note path caveat, not a fresh divergence."

requirements-completed: []  # VERIFY-01/VERIFY-02 span all of plans 05-01..05-13; not individually completable until 05-13 closes the ledger out

coverage:
  - id: D1
    description: "Epic 22 block verdict (satisfied by shipped code) backed by a 15-row parent-task cluster table, each row verdicted against a real cargo test run or build, not checkbox state"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion in_memory_registry:: -- 9/9 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ports paladin_registry -- 2/2 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core battalion:: -- 93/93 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion council_service:: -- 10/10 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion grove_service:: -- 15/15 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion phalanx_service:: -- 14/14 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-battalion commander:: -- 50/50 passed, 0 ignored"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core grove:: -- 15/15 passed"
      - kind: integration
        ref: "cargo test --offline -p paladin-ai --test lib -- helpers::mock_llm_adapter -- 10/10 passed"
      - kind: integration
        ref: "cargo test --offline -p paladin-ai --test lib -- integration::battalion -- 76/76 passed"
      - kind: other
        ref: "cargo build --offline --example commander_with_metadata_export -p paladin-ai -- exit 0"
        status: pass
    human_judgment: true
    rationale: "Plan's own <human-check> requires a human to confirm every parent task in the source file appears in the table with a verdict and evidence, and that the satisfied-by-shipped-code verdict is properly backed rather than inferred (05-VALIDATION.md §Manual-Only Verifications, row 2)."
  - id: D2
    description: "Epic 22's 10 REQ-* ledger rows filled to the D-01 evidence bar in the same pass"
    verification:
      - kind: other
        ref: "sed -n '/^### Epic 22 /,/^### Epic 23 /p' .planning/ledgers/milestone-02-03.md | grep -c '^| REQ-' equals 10; grep -c 'PENDING-VERDICT' in that range equals 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "REQ-grove-llm-routing records the hardcoded model: \"gpt-4\" at grove_service.rs:537 as verified open, assigned to Phase 6 / CLOSE-01, not fixed"
    verification:
      - kind: other
        ref: "grep 'grove_service.rs:537' in the Epic 22 ledger range also contains 'CLOSE-01' — confirmed by grep, count 1"
        status: pass
    human_judgment: false
  - id: D4
    description: "Ledger integrity preserved: exactly 118 REQ-* rows, 14 epic sections, no row inserted/deleted/reordered, no .rs/Cargo.toml/.github file touched"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-02-03.md equals 118; git diff --stat -- '*.rs' 'Cargo.toml' '.github/' empty; git diff shows only the 10 Epic 22 rows changed"
        status: pass
    human_judgment: false

duration: ~90min
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 05: Epic 22 block verdict + ledger rows Summary

**Epic 22 (Battalion & Commander Hardening) verdicted `satisfied by shipped code` — all 15 parent-task clusters verify, including three the source task list still marks open, because their capabilities shipped in commits the checkboxes were never updated to reflect; Phase 6's CLOSE-02 needs no work for this block.**

## Performance

- **Duration:** ~90 min (dominated by scoped `cargo test`/`cargo build` compiles against a cold-then-warming workspace cache, several minutes each on the first few invocations)
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-02-03.md`)

## Accomplishments

- Read `tasks-epic22-battalion-commander-hardening.md` in full (15 parent tasks, `0.0`–`14.0`) and
  verified every cluster's distinct capability claim directly against the current tree — not against
  its own checkbox state, per D-05.
- Discovered that the three clusters the source file marks open (`3.0` Council registry integration,
  `4.0` Grove registry integration, `5.0` Grove LLM-based routing) actually **shipped**, each in its
  own commit (`761c49c`, `0cdf8dd`, `5f05db7`) with passing tests — the task list's checkboxes were
  simply never updated. This is exactly the corpus's dominant pattern (checkbox state understating
  shipped reality), corroborated here for the largest of the three VERIFY-02 blocks.
- Wrote the Epic 22 block verdict — **`satisfied by shipped code`** — per D-06 (a block verdicts
  satisfied only if every cluster verifies), backed by a 15-row `| Parent task | Verdict | Evidence |`
  table with a real `cargo test`/`cargo build` invocation behind every row.
- Transcribed the 81-open-item count from `.planning/intel/task-completion-state.md` without
  re-deriving it, per the plan's explicit prohibition.
- Filled all 10 Epic 22 `REQ-*` rows to the D-01 evidence bar: 9 `satisfied` (each backed by an
  executed test run), 1 `genuinely outstanding` — `REQ-grove-llm-routing`, recording the hardcoded
  `model: "gpt-4"` at `grove_service.rs:537` as a verified-open defect owned by **Phase 6 / CLOSE-01**,
  without touching the `.rs` file.
- Measured `#[ignore]` attributes in `commander.rs` directly (`grep -c '#\[ignore\]'` → **0**) rather
  than trusting the PRD's recorded count, for `REQ-commander-test-hardening`.
- Found and recorded a real infrastructure fragility: `tests/helpers/mock_llm_adapter.rs` (Task
  10.0's MockLlmAdapter) carries no explicit `[[test]]` entry in `Cargo.toml` — it is only compiled
  and run because `tests/lib.rs` is itself auto-discovered by Cargo's default `tests/*.rs` convention
  as target `lib`, and that file's `pub mod helpers; pub mod integration; pub mod unit;` pulls
  everything else in transitively. The capability and its tests are real and passing; the wiring is
  implicit rather than declared.

## Task Commits

Both tasks were committed together in a single commit at the end, per the plan's explicit Task 2
instruction (this repo's pre-commit hooks recompile the full 12-crate workspace on every commit
including markdown-only ones; this worktree runs with `workflow.worktree_skip_hooks=true` so
`--no-verify` was used for the single commit, per the parallel-execution hook policy):

1. **Task 1: Verify Epic 22's parent-task clusters and write the block verdict** — part of `aa2f5f1`
2. **Task 2: Fill Epic 22's 10 REQ ledger rows to the D-01 evidence bar** — part of `aa2f5f1`

`aa2f5f1` — `docs(05-05): verify Epic 22 block and fill its 10 ledger rows`

_No separate plan-metadata commit — SUMMARY.md is committed by this same worktree per the
parallel-execution instructions; STATE.md/ROADMAP.md updates are owned by the orchestrator after
the wave merges._

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — Epic 22 section: added the block-verdict subsection (one
  sentence, the transcribed 81-open-item count, and the 15-row parent-task cluster table) above the
  existing `| ID | Verdict | Evidence |` table, and replaced all 10 `PENDING-VERDICT` stub rows with
  cited verdicts. No other epic section touched; row count and section count both verified unchanged
  outside Epic 22.

## Decisions Made

- **Epic 22 block verdict: `satisfied by shipped code`, not `partially outstanding`.** Every one of
  the 15 parent-task clusters verifies against the tree — including the three the source file marks
  open — so per D-06 the block is satisfied and Phase 6's CLOSE-02 needs **no work** for this block.
- **`REQ-grove-llm-routing` → `genuinely outstanding`, distinct from its parent cluster's `satisfied
  by shipped code` verdict.** The cluster-level claim ("Grove routes via LLM with a confidence
  threshold and configurable fallback") ships and is fully tested; the requirement-level claim
  ("routing honours the Paladin's configured provider") does not — the model is hardcoded. These are
  different units of verification (D-05's cluster vs. D-01's per-requirement bar) and are allowed to
  diverge; the plan's own read_first section anticipated this exact split.
- **`REQ-commander-config-metadata-dir-v3` and `REQ-battalion-metadata-extension` cite the already-
  settled `BattalionConfig`/`battalion/mod.rs` locations rather than re-litigating the PRD's
  `CommanderConfig`/`battalion_result.rs` naming**, per the ledger's own head-note path caveat and
  the Milestone 1 ledger's prior RECON-03 resolution — not re-decided here.
- **Task 14.0 (documentation and examples) verdicted `satisfied` despite `docs/BATTALION.md` and
  `docs/COMMANDER.md` not existing at those paths.** Milestone 11's docs overhaul relocated Battalion
  documentation into mdbook, and none of the resulting pages mention the Epic 22 additions by name —
  but a working, building example (`examples/commander_with_metadata_export.rs`) and a full
  `CHANGELOG.md` section clear the D-01 bar via a different artefact than the PRD named.

## Deviations from Plan

None — plan executed exactly as written. The one substantive discovery (three "open" clusters having
actually shipped) is exactly the kind of finding D-05/D-06 exist to surface, not a deviation from the
plan's instructions.

**Worktree-mode note (not a deviation, expected behavior):** per this execution's
`<parallel_execution>` instructions, STATE.md and ROADMAP.md are not modified by this plan — the
orchestrator updates them centrally after the wave merges.

## Issues Encountered

- **Initial confusion over whether `tests/helpers/mock_llm_adapter.rs` and
  `tests/integration/battalion/*.rs` were dead code.** Neither is referenced by any `Cargo.toml`
  `[[test]]` entry by name. Investigation found `tests/lib.rs` — itself not listed in `[[test]]` — is
  auto-discovered by Cargo's default `tests/*.rs` convention as a standalone integration-test binary
  named `lib`, and its `pub mod helpers; pub mod integration; pub mod unit;` declarations pull in
  everything transitively. Confirmed by `cargo test --offline -p paladin-ai --test lib -- --list`,
  which listed and then ran all of it successfully. Recorded as a "New finding" on the `10.0` cluster
  row rather than left unexplained, since the wiring is real but fragile (undocumented in
  `Cargo.toml`, so a future rename/deletion of `tests/lib.rs` would silently drop ~700 tests with no
  manifest diff to flag it).

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Epic 22's ledger section is complete: one block verdict, a 15-row cluster table, and 10 cited
  `REQ-*` rows. Phase 6's CLOSE-02 scope for this block is settled: **no work required.**
- `REQ-grove-llm-routing`'s row is the one live pointer into Phase 6 / CLOSE-01 this plan produces —
  the hardcoded `grove_service.rs:537` model is recorded, not fixed.
- Ledger integrity preserved for the remaining fan-out plans (05-06 .. 05-12): row count still 118,
  section count still 14, no row order disturbed outside Epic 22.
- No blockers for the next wave.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-02-03.md` (modified, Epic 22 section)
- FOUND: `.planning/phases/05-milestone-2-3-ground-truth/05-05-SUMMARY.md`
- FOUND: commit `aa2f5f1` (task commit, single file)

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
