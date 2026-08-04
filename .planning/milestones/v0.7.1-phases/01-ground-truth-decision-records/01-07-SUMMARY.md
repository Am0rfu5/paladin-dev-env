---
phase: 01-ground-truth-decision-records
plan: 07
subsystem: docs
tags: [ground-truth, decision-records, ledger, provider-expansion, citadel, herald, armory-cli, coverage, requirements-audit]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records
    provides: ".planning/ledgers/milestone-01.md header, D-19 evidence bar, D-20 verdict legend, RECON-08 Epic 10 dispute resolution, and the 60 REQ-* rows across Epics 1-5 (plan 01-06)"
provides:
  - "52 REQ-* rows authored across Epic 6 (Provider Expansion, 9), Epic 7 (Citadel State Persistence, 10), Epic 8 (Herald Output Formatting, 10), Epic 9 (Armory CLI Tools, 12), Epic 10 (Validation & Documentation, 9) and unit-test-improvements (2), completing all 112 REQ-* rows in the ledger"
  - "26 outstanding task items nested under their owning requirement (Epic 6: 19, Epic 8: 2, Epic 9: 3, unit-test-improvements: 2), each with literal checkbox text and a resolving file:line citation, completing all 39 outstanding Milestone-1 task items across the whole ledger"
  - "The closing '## Outstanding item reconciliation' section: 39 outstanding items reconciled against intel/task-completion-state.md's deterministic per-file breakdown (exact agreement, no adjustment needed), plus a programmatically-computed verdict-class distribution (100 satisfied / 23 present-unproven / 11 genuinely-outstanding / 19 superseded / 1 deferred, 154 total rows)"
  - "REQ-herald-battalion-result-fields and REQ-herald-paladin-result-fields upgraded from the 2026-01 Partial/Verify status to satisfied on fresh 2026-07-31 citations"
  - "Two new dead-code findings, both re-verified against the tree rather than inferred: (1) tests/unit/llm/{deepseek,anthropic,provider_factory}_adapter_test.rs (Epic 6's claimed mockito suite) is never compiled -- tests/unit/mod.rs does not declare pub mod llm; (2) unit-test-improvements task 6.3's own claim (16 tests, 86.71% coverage, create_with_config()) does not match the current provider_factory.rs, which has 3 tests and no such method"
  - "Explicit non-fabrication on ADR-0006: the coverage-gate ADR (plan 01-04's RECON-07 output) does not exist in this tree -- 01-04 is non-autonomous, blocked on crates.io network access for cargo-llvm-cov -- so unit-test-improvements rows carry workstream-local figures with a caveat instead of a broken link, and REQ-test-coverage-target-v1/-v2 stays genuinely outstanding pending that ADR"
  - "RECON-01 ticked complete in REQUIREMENTS.md (checkbox + traceability row) -- the full ledger (Epics 1-10 + unit-test-improvements, all 39 outstanding items) now satisfies every truth in the requirement"
affects: [phase-02, phase-03, phase-05, phase-15]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Dead-code detection pattern: before citing a test as an exerciser, confirm it is reachable from a [[test]] binary's own entry-point module tree (tests/unit/mod.rs, tests/integration/mod.rs), not just that the file exists on disk -- this caught two separate cases in this plan where task-list-claimed tests are orphaned"
    - "ADR-not-yet-authored pattern: when a plan's read_first names a sibling plan's not-yet-produced artifact (here, ADR-0006 from non-autonomous plan 01-04), do not fabricate the link -- record the gap explicitly, carry forward the best available local evidence with a caveat, and forward-own the resolution to the plan that actually produces it"

key-files:
  created:
    - .planning/phases/01-ground-truth-decision-records/01-07-SUMMARY.md
  modified:
    - .planning/ledgers/milestone-01.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "REQ-provider-testing stays present, unproven rather than satisfied: the richer mockito-based test suite the task list's own task 6.0 claims (tests/unit/llm/*.rs) is dead code, never compiled into any test binary -- confirmed by absence from tests/unit/mod.rs and by cargo test --test unit -- --list showing zero matches for deepseek/anthropic/provider_factory. The tests that do run (crate-level adapter tests) are cited as the satisfied basis for the adapter-existence rows instead."
  - "REQ-herald-battalion-result-fields upgraded from the 2026-01 'Partial -> GAP-03 (depends on RECON-03)' note to satisfied: RECON-03 already closed in a prior ingest run (BattalionResult resolved to a merged superset), so the Herald's battalion-field formatting is no longer blocked on anything, and this is confirmed by a real passing test asserting metadata fields."
  - "Epic 6's task 7.0 (live-API integration tests) is recorded present, unproven at the parent level, not deferred with reason -- the task's own inline annotation calls it deferred, but STATE.md's Deferred Items table records it as un-deferred by ingest run 2 (the suite ships behind live-api-tests), with only the skip-vs-fail semantics open. require_api_key()'s panic-on-missing-key was independently re-confirmed by reading llm_live_api_tests.rs directly, matching STATE.md's own 'two contradictions are live in shipped code' note."
  - "unit-test-improvements' two open parents (2.0, 6.0) are both stale-parent-over-complete-subtasks except 6.0's own 6.3 child, whose claim (16 new tests, 86.71% coverage) is contradicted by the current provider_factory.rs (3 tests, no create_with_config()). Recorded as a finding, not smoothed over -- neither speculating that the claim was inflated nor that Milestone 5's workspace decomposition definitely rewrote it, since neither can be confirmed from this tree alone."
  - "REQ-test-coverage-target-v1/-v2 (the coverage-gate variant group) is recorded genuinely outstanding rather than picking a side, because ADR-0006 -- the artifact RECON-07/plan 01-04 was supposed to produce to settle it with one number -- does not exist in this tree. Recording a winner without that ADR would repeat exactly the 'seventh unverified coverage figure' 01-04-PLAN.md's own prohibitions forbid."
  - "Fixed a pre-existing double-citation in Epic 5's REQ-commander-auto-selection row (authored by plan 01-06) that inflated the whole-ledger tasks-*.md:NNN citation count from 39 to 40 -- this plan's own Task 2 acceptance criteria gates on the whole-file count, so the fix was in scope even though the row itself predates this plan."
  - "RECON-01 ticked complete: all five must_haves (39 outstanding items enumerated with citations, 26 nested under owning requirements by this plan, bare D-20 verdicts throughout, D-19 evidence bar honestly applied, Epic 10 consistent with plan 01-05's Task 7.0 verdict) are genuinely satisfied by the finished ledger."

requirements-completed: [RECON-01]

coverage:
  - id: D1
    description: "Epic 6 (Provider Expansion) -- 9 REQ-* rows re-verdicted with re-checked citations, all 19 outstanding task items (task 7.0 and its 18 subtasks) nested by literal checkbox text"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "grep -q '### Epic 6 ' .planning/ledgers/milestone-01.md && tasks-provider-expansion.md:NNN citation count == 19 && task file's own open-checkbox count == 19 && Epic-6 REQ- row count >= 9 -- all pass, EPIC6_OK"
        status: pass
    human_judgment: false
  - id: D2
    description: "Epic 7-10 and unit-test-improvements -- 43 REQ-* rows re-verdicted, 7 outstanding task items nested (Epic 8: 2, Epic 9: 3, unit-test-improvements: 2), closing reconciliation section with exact 39-item agreement and a programmatic verdict-class distribution"
    requirement: "RECON-01"
    verification:
      - kind: other
        ref: "grep -q for all 5 headings && herald/armory/unit-test citation counts == 2/3/2 && '## Outstanding item reconciliation' present && task-completion-state.md referenced && whole-file tasks-*.md:NNN citation count == 39 && decisions/0005-herald-trait.md linked -- all pass, LEDGER_COMPLETE"
        status: pass
    human_judgment: false

duration: ~95min
completed: 2026-07-31
status: complete
---

# Phase 1 Plan 07: Epic 6-10 ledger rows and outstanding-item reconciliation Summary

**Completed the Milestone-1 ledger: 52 more citation-backed `REQ-*` rows (Epics 6-10 plus unit-test-improvements) and all 26 remaining outstanding task items, closing RECON-01 with an exact 39-item reconciliation against `intel/task-completion-state.md` and two new dead-code findings that the 2026-01 task list's own completion claims don't survive re-verification.**

## Performance

- **Duration:** ~95 min
- **Completed:** 2026-07-31
- **Tasks:** 2/2
- **Files modified:** 2 (`.planning/ledgers/milestone-01.md`, `.planning/REQUIREMENTS.md`)

## Accomplishments

- Epic 6 (Provider Expansion): 9 rows, all 19 of its outstanding task items (task 7.0 and 18 subtasks) nested individually by literal checkbox text -- the single largest concentration of outstanding work in Milestone 1, now fully accounted for
- Epic 7 (Citadel State Persistence): 10 rows, all `satisfied` with fresh citations against `citadel.rs`, `citadel_port.rs`, `file_citadel.rs`, `paladin_builder.rs`; no open items
- Epic 8 (Herald Output Formatting): 10 rows plus 2 nested items; `REQ-herald-battalion-result-fields` and `REQ-herald-paladin-result-fields` both upgraded from the 2026-01 Partial/Verify note to `satisfied` on re-verified evidence
- Epic 9 (Armory CLI Tools): 12 rows plus 3 nested items (13.4-13.6, recorded `genuinely outstanding` -- the blocking mock-provider dependency shipped, but no CLI-level test was ever written against it); `REQ-cli-interactive-mode` points at the existing Divergences-table row rather than repeating it
- Epic 10 (Validation & Documentation): 9 rows, no open items; Task 7.0's dispute is not re-opened, consistent with plan 01-05's resolution
- unit-test-improvements workstream: 2 rows plus 2 nested items; explicitly does not fabricate a link to ADR-0006 (which does not exist -- plan 01-04 is blocked on crates.io network access)
- Closing `## Outstanding item reconciliation` section: 39 outstanding items agree exactly with the deterministic source across all 8 task files; verdict-class distribution computed programmatically (100 satisfied / 23 present-unproven / 11 genuinely-outstanding / 19 superseded-by-shipped-code / 1 deferred, 154 total rows) rather than via a loose text grep that would double-count verdict words inside evidence prose
- Two dead-code findings, both confirmed by direct inspection rather than assumed: `tests/unit/llm/*.rs` (Epic 6's claimed 27-test mockito suite) is never compiled into any test binary; unit-test-improvements task 6.3's claim about `provider_factory.rs` (16 tests, `create_with_config()`) does not match the 3 tests and no such method actually in that file today
- RECON-01 ticked complete in `REQUIREMENTS.md` (checkbox + traceability row) -- the ledger this requirement asks for is now fully authored across all 10 Milestone-1 epics plus the unit-test-improvements workstream

## Task Commits

Each task was committed atomically:

1. **Task 1: Ledger rows for Epic 6-7** - `5c7f425` (feat)
2. **Task 2: Ledger rows for Epic 8-10, unit-test-improvements, and reconciliation** - `65ec0a8` (feat)

## Files Created/Modified

- `.planning/ledgers/milestone-01.md` -- appended `### Epic 6` through `### Epic 10` and `### unit-test-improvements workstream` sections (52 `REQ-*` rows, 26 nested outstanding-item bullets) plus the closing `## Outstanding item reconciliation` section; fixed one pre-existing double-citation in the Epic 5 section
- `.planning/REQUIREMENTS.md` -- ticked RECON-01's checkbox and flipped its traceability-table row from `Pending` to `Complete`

## Decisions Made

See `key-decisions` in frontmatter above for the full list. In short: `REQ-provider-testing` and `REQ-herald-battalion-result-fields`/`REQ-herald-paladin-result-fields` each got re-verified rather than carried forward (one holding at `present, unproven` on a genuine dead-code finding, two upgrading to `satisfied` on closed dependencies); the coverage-gate variant group stays unresolved because its resolving ADR doesn't exist yet, rather than this plan inventing a seventh unverified number; and RECON-01 is ticked complete because all five of its `must_haves` are genuinely met by the finished ledger.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Pre-existing double-citation in Epic 5's REQ-commander-auto-selection row inflated the whole-ledger citation count**
- **Found during:** Task 2, while verifying the whole-file `tasks-[a-z-]+\.md:[0-9]+` citation count against the required total of 39
- **Issue:** Plan 01-06's Epic 5 section cited `tasks-commander-strategy-router.md:99` twice -- once in `REQ-commander-auto-selection`'s evidence prose and again in the nested item bullet immediately below it -- which is correct in substance (both references are to the same real line) but inflated the total citation count to 40 against this plan's own Task 2 acceptance criterion (`-eq 39`).
- **Fix:** Removed the redundant `file:line`-patterned citation from the evidence prose (kept the plain "line 99" reference without the `tasks-commander-strategy-router.md:99` pattern), leaving the nested item's own citation as the sole formal one.
- **Files modified:** `.planning/ledgers/milestone-01.md`
- **Verification:** `grep -cE 'tasks-[a-z-]+\.md:[0-9]+' .planning/ledgers/milestone-01.md` returns exactly 39
- **Committed in:** `65ec0a8` (Task 2 commit, since the fix was needed to satisfy Task 2's own gate)

**2. [Rule 1 - Bug] Two of my own new evidence-prose citations initially double-counted against their nested items' formal citations**
- **Found during:** Task 1 (Epic 6) and Task 2 (unit-test-improvements), while running each task's own citation-count verify commands
- **Issue:** In drafting `REQ-provider-testing` (Epic 6) and `REQ-unit-test-gap-closure` (unit-test-improvements), I cited specific task-file line numbers in the evidence prose using the same `file:line` pattern the nested-item bullets use, which inflated the per-file citation counts past the exact totals the acceptance criteria require (19 for Epic 6, 2 for unit-test-improvements).
- **Fix:** Rephrased the prose citations to reference the line number in plain text (e.g. "at line 131") without the `path:line` backtick pattern, since the substantive citation (a real, resolving line reference) is preserved either way -- only the double-counted formal pattern was removed.
- **Files modified:** `.planning/ledgers/milestone-01.md`
- **Verification:** `grep -cE 'tasks-provider-expansion\.md:[0-9]+'` returns 19; `grep -cE 'tasks-improve-unit-test-coverage\.md:[0-9]+'` returns 2
- **Committed in:** `5c7f425` (Task 1) and `65ec0a8` (Task 2) respectively -- caught and fixed before each task's commit, not as a follow-up

---

**Total deviations:** 2 auto-fixed (both Rule 1 -- correctness of the acceptance-gated citation counts)
**Impact on plan:** No scope creep; both are self-caught precision fixes on this plan's own verification gates, one of which touched a prior plan's section because the whole-ledger gate spans the entire file.

## Issues Encountered

- **ADR-0006 does not exist.** The plan's `read_first` for Task 2 names `.planning/decisions/0006-coverage-gate.md` as the source the unit-test-improvements rows should link to. That file is plan 01-04's output (RECON-07), and 01-04 is `autonomous: false`, blocked on a `user_setup` step requiring network access to crates.io for `cargo-llvm-cov` (confirmed: `cargo llvm-cov --version` fails with "no such command" in this sandbox, and no `.planning/decisions/0006-coverage-gate.md` file exists anywhere in this worktree). This is a genuine cross-plan dependency gap: 01-07's frontmatter lists only `depends_on: ["01-06"]`, but its own Task 2 action assumes 01-04's artifact exists. Rather than fabricating a link to a nonexistent file (which would violate the "every cited file:line resolves" acceptance criterion) or blocking the whole plan on a checkpoint, I recorded the gap explicitly in the `unit-test-improvements` section's own scope note, carried forward the task file's workstream-local self-reported coverage figures with an explicit caveat, and left `REQ-test-coverage-target-v1/-v2` `genuinely outstanding` pending that ADR. This plan's own Task 2 `<verify>` command does not check for the ADR-0006 link (only for `decisions/0005-herald-trait.md`), so this did not block the automated gate.
- **`cargo-llvm-cov` unavailable.** Same underlying blocker as above -- no coverage percentages in this plan's new sections were freshly measured; every coverage figure cited (DeepSeek 15.02%, Anthropic 28.19%, the unit-test-improvements workstream's self-reported 70.56%/68.29%) is carried forward from a prior source with an explicit "not re-measured" caveat, never presented as fresh.
- **Two genuine dead-code findings**, both re-verified rather than assumed: (1) `tests/unit/llm/{deepseek_adapter_test,anthropic_adapter_test,provider_factory_test}.rs` (Epic 6 task 6.0's claimed 27-test mockito suite) is never compiled into any test binary -- `tests/unit/mod.rs` (the `[[test]] name = "unit"` binary's entry point) does not declare `pub mod llm;`, confirmed independently by `cargo test --test unit -- --list` showing zero matches for `deepseek`/`anthropic`/`provider_factory`. (2) unit-test-improvements task 6.3's own claim ("16 comprehensive unit tests covering `create_with_config()`", "49.73% -> 86.71%") does not match the current `crates/paladin-llm/src/provider_factory.rs`, which has exactly 3 test functions and no `create_with_config()` method at all. Neither finding speculates about *why* the mismatch exists (workspace decomposition rewrite vs. inflated original claim) -- both are recorded as the mismatch itself.
- No blockers for the ledger's own completion; RECON-01 is genuinely satisfied.

## User Setup Required

None -- no external service configuration required by this plan. (Plan 01-04's own `user_setup` for crates.io access remains outstanding and is that plan's responsibility, not this one's.)

## Next Phase Readiness

- `.planning/ledgers/milestone-01.md` now holds the complete Milestone-1 ledger: header, D-19/D-20 legend, 3 Divergences rows, RECON-08 Epic 10 dispute resolution, ingest bookkeeping corrections, all 112 `REQ-*` rows across Epics 1-10 and unit-test-improvements, all 39 nested outstanding task items, and the closing reconciliation section. RECON-01 is complete.
- **RECON-07 (plan 01-04) remains genuinely open** -- this plan did not and could not close it (no network access to crates.io in this sandbox for `cargo-llvm-cov`). The unit-test-improvements section explicitly forward-owns the single coverage number to that plan rather than resolving it here. Any later phase or plan that needs a single workspace-wide coverage figure should look for `.planning/decisions/0006-coverage-gate.md` and, if still absent, re-surface plan 01-04's blocker rather than re-deriving a number independently.
- Two dead-code findings (Epic 6's orphaned mockito test suite; unit-test-improvements' stale `provider_factory.rs` claim) are recorded in the ledger with forward owners (**VERIFY-06**, **QUAL-01/QUAL-02**) but not fixed -- fixing dead test code or writing the missing CLI mock-provider tests (Epic 9's 13.4-13.6) is out of scope for a ground-truth ledger phase and belongs to whichever forward phase owns those requirement IDs.
- No blockers for downstream phases reading this ledger.

---
*Phase: 01-ground-truth-decision-records*
*Completed: 2026-07-31*
