---
phase: 02-functional-gap-closure
plan: 09
subsystem: testing
tags: [test-wiring-sweep, adr, ledger-amendment, roadmap, d-02, d-05, d-08, d-12]

# Dependency graph
requires:
  - phase: 02-functional-gap-closure
    provides: "All seven prior Phase 2 plans (02-01 through 02-08) — their SUMMARYs are this
      plan's primary source material: measured verdicts, test names, commands and per-file counts
      every ledger amendment below cites."
provides:
  - "The D-12 sweep record (`02-test-wiring-sweep.md`): every `tests/` subdirectory, `benches/`
    and `examples/` file cross-checked against `[[test]]`/`[[bench]]`/`[[example]]` declarations
    and barrel `mod.rs` files, using `cargo test --workspace -- --list` (35 binaries/doctest-groups)
    as the compiled-target ground truth"
  - "ADR-0007: the battalion-cancellation deferral, recording Phalanx-only cancellation as
    satisfied and the other three patterns as deferred with reason, with a named forward owner
    (the v2 backlog) and a named prerequisite (the mid-run-return contract for Campaign and
    ChainOfCommand)"
  - "`.planning/ledgers/milestone-01.md` amended in place per D-02: 11 rows upgraded, 3 new rows
    added, the Outstanding item reconciliation counts and verdict distribution updated with full
    arithmetic (155 -> 158 rows), and three corrections to this phase's own planning inputs"
  - "`.planning/ROADMAP.md` Phase 2 criteria 1 and 5 corrected at source, with a dated amendment
    note; no other ROADMAP section touched"
affects: [phase-3-qual-work, phase-5-ground-truth, phase-7-ground-truth, phase-10-ground-truth,
  phase-13-ground-truth, phase-15-pipe]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Amend-in-place ledger convention (D-02): every amendment is a scoped edit to an existing
      row, or a new nested row alongside it, never a separate corrections file — the convention
      Phases 5, 7, 10 and 13 inherit for their own sibling ledgers"

key-files:
  created:
    - .planning/phases/02-functional-gap-closure/02-test-wiring-sweep.md
    - .planning/decisions/0007-battalion-cancellation-deferral.md
  modified:
    - .planning/ledgers/milestone-01.md
    - .planning/ROADMAP.md
    - .planning/decisions/PROMOTION.md

key-decisions:
  - "Represented the ADR-0007 cancellation split as the existing REQ-battalion-cancellation row
    (now satisfied, for Phalanx) plus one new nested row (deferred with reason, for
    Formation/Campaign/ChainOfCommand) rather than inventing a sixth verdict class or splitting
    into two REQ-* IDs — keeps the ledger's one-verdict-per-row contract intact for the
    reconciliation arithmetic."
  - "Added two ledger findings from plan 02-08's Garrison PRD review (PaladinError::GarrisonRequired,
    GarrisonSettings::validate()) as new nested rows with a 'v2 backlog (candidate)' forward owner,
    since plan 02-08 explicitly deferred owner-assignment to this plan rather than assigning one
    inside the review itself."
  - "[Rule 2 - missing critical functionality] Updated .planning/decisions/PROMOTION.md's numbering
    index and 'Next free ADR number' line (0007 -> 0008) when minting ADR-0007, even though
    PROMOTION.md is not in the plan's declared files_modified — its own Part A step 5 makes this a
    required part of minting any new ADR number, and leaving it stale would cause a future
    promoting phase (5, 7, 10 or 13) to collide on the same number."
  - "Verified the worktree/shared-checkout boundary mid-execution after several early investigative
    commands (grep, find, cargo test --list) were inadvertently run against /workspace (the shared
    checkout) rather than this worktree, discovered via a tool-guard error on a subsequent git
    command. Confirmed both trees were content-identical at that point (same HEAD commit, no
    concurrent wave-4 sibling plans, zero prior edits from this session) before trusting the
    gathered data, then re-ran the authoritative cargo test --workspace -- --list and the final
    cargo test --workspace verification natively inside this worktree (both corroborated the
    /workspace-derived data exactly: 35 binaries/doctest-groups, 0 failed). No commit, edit or
    write of any kind touched /workspace at any point — only read-only shell commands."
  - "Recomputed the ledger's verdict-class distribution by direct extraction (grep -oE against
    every REQ-* row's Verdict column, plus the final bolded verdict word in every nested-item line)
    rather than by manual delta bookkeeping, after an initial manual tally produced a one-off
    discrepancy — the extracted counts (110/19/5/21/3, total 158) are what the SUMMARY and ledger
    both report."

requirements-completed: [GAP-01, GAP-02, GAP-03, GAP-04, GAP-05, GAP-06, GAP-07]

coverage:
  - id: D1
    description: "D-12 sweep record covering every tests/ subdirectory, benches/ and examples/,
      with the two already-closed instances recorded closed and new findings (four still-commented
      tests/cli/ files, the LlmProviderError dead-conversion path, the stale 22-examples figure)
      reported with forward owners, no source file modified"
    requirement: "GAP-01"
    verification:
      - kind: other
        ref: "test -s 02-test-wiring-sweep.md && wc -l -ge 40 (196 lines) && grep -q 'Findings requiring a decision'"
        status: pass
      - kind: other
        ref: "cargo test --workspace -- --list (35 binaries/doctest-groups, matching plan 02-01's baseline)"
        status: pass
      - kind: other
        ref: "git diff --name-only after Task 1 lists nothing outside .planning/"
        status: pass
    human_judgment: false
  - id: D2
    description: "ADR-0007 records the Phalanx-only cancellation reality, the three-pattern
      deferral with reasoning, a named forward owner and prerequisite, all seven ADR-0004-shaped
      sections, Code Conformance stating conforms with no code change mandated, and parses under
      the project's ADR parser"
    requirement: "GAP-07"
    verification:
      - kind: other
        ref: "grep -cE '^## (Status|Context|Decision|Considered Options|Code Locations|Code Conformance|Downstream Consumers)$' 0007-battalion-cancellation-deferral.md -> 7"
        status: pass
      - kind: other
        ref: "node .claude/gsd-core/bin/lib/adr-parser.cjs --input 0007-battalion-cancellation-deferral.md (exit 0, same as ADR-0004)"
        status: pass
    human_judgment: false
  - id: D3
    description: "The ledger uses only the five legend verdict classes, carries a dated Phase 2
      amendments note, and every named row (Epic 8 task 7.13, Epic 9 tasks 13.4-13.6, the Epic 4
      cancellation row citing 0007, the Epic 2 rows citing ADR-0006 and the review path) is amended
      with the cited evidence"
    requirement: "GAP-03"
    verification:
      - kind: other
        ref: "grep -oE 'satisfied|present, unproven|genuinely outstanding|deferred with reason|superseded by shipped code' milestone-01.md | sort -u (exactly the five classes)"
        status: pass
      - kind: other
        ref: "grep -q 'Phase 2 amendments' milestone-01.md; grep -q '0007' milestone-01.md"
        status: pass
    human_judgment: false
  - id: D4
    description: "ROADMAP Phase 2 criterion 1 no longer asserts the named test fails today,
      criterion 5 states outcomes without the already-true BattalionResult premise, and no other
      ROADMAP phase section changed"
    requirement: "GAP-05"
    verification:
      - kind: other
        ref: "grep -c 'which fails today' ROADMAP.md -> 0"
        status: pass
      - kind: other
        ref: "git diff -U0 ROADMAP.md shows changed hunks only within the Phase 2 criteria block (lines 217, 221)"
        status: pass
    human_judgment: false
  - id: D5
    description: "cargo test --workspace stays green after all three tasks (this plan makes no
      Rust source change), confirming nothing was disturbed"
    verification:
      - kind: other
        ref: "cargo test --workspace, run natively inside this worktree after Task 3's edits: 0 failed across every reported group"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-01
status: complete
---

# Phase 2 Plan 09: Close the Record — Sweep, ADR-0007, Ledger Amendment, ROADMAP Correction Summary

**D-12 test-wiring sweep across `tests/`/`benches/`/`examples/`, ADR-0007 recording the battalion-cancellation deferral (Phalanx satisfied, the other three patterns deferred to the v2 backlog), 14 ledger rows amended or added in `milestone-01.md` per D-02, and ROADMAP Phase 2's two stale-premise criteria corrected at source — closing Phase 2's own record-keeping half**

## Performance

- **Duration:** ~35 min of active edit/commit work (plus upfront reading of ~15 input documents:
  the plan, PROJECT.md, STATE.md, all 8 prior plan SUMMARYs, the milestone-01 ledger, ADR-0004,
  PROMOTION.md, 02-CONTEXT.md, ROADMAP.md's Phase 2/3 sections, and direct tree verification via
  `grep`/`cargo test -- --list`)
- **Started:** ~2026-08-01T14:50Z (worktree spawn)
- **Completed:** 2026-08-01T15:18:09Z (Task 3 commit `0dd3ae9`)
- **Tasks:** 3 (all `type="auto"`)
- **Files modified:** 5 (2 created, 3 modified — including `PROMOTION.md`, a Rule 2 deviation
  outside the plan's declared `files_modified`)

## Accomplishments

- **Task 1 — D-12 sweep.** Enumerated every `.rs` file under `tests/`, `benches/` and `examples/`
  and cross-checked each against `Cargo.toml`'s `[[test]]`/`[[bench]]`/`[[example]]` declarations
  and every barrel `mod.rs`, using `cargo test --workspace -- --list` (35 binaries/doctest-groups,
  matching plan 02-01's baseline exactly) and `cargo test --features cli --test cli -- --list` (99
  tests) as the compiled-target ground truth. Recorded the two already-closed instances (the LLM
  unit module by plan 02-06, the five CLI suites by plan 02-07) as closed. Reported three new
  findings without fixing them: four still-commented `tests/cli/` files (66 never-compiled test
  functions across `arsenal_config_test.rs`, `environment_tests.rs`, `garrison_config_test.rs`,
  `integration_tests.rs`), the `LlmProviderError` dead-conversion path
  (`crates/paladin-llm/src/error.rs:16,54`, zero constructors outside that file, verified live), and
  the stale "22 examples" figure in ROADMAP Phase 4 criterion 5 (46 examples exist today; the CI
  script already enumerates dynamically). Confirmed every `benches/` file (5 across the workspace)
  is declared and none is orphaned. No Rust source file was touched.
- **Task 2 — ADR-0007.** Wrote `.planning/decisions/0007-battalion-cancellation-deferral.md`
  following ADR-0004's exact section shape. Records: Phalanx's `execute_with_cancellation`
  (`phalanx_service.rs:151`, tested at `:758`) as the requirement's satisfied scope; Formation,
  Campaign and ChainOfCommand as deferred with reason, verified live to have zero
  `execute_with_cancellation`/`CancellationToken` sites; the forward owner (the v2 backlog, gated on
  a cancellation-contract decision) and the named prerequisite (the mid-run-return contract for
  Campaign and ChainOfCommand); Phase 3 considered and rejected as owner, citing its own five
  success criteria directly. `Code Conformance: conforms`, no code change mandated. Parses under
  `.claude/gsd-core/bin/lib/adr-parser.cjs` with the same exit status (0) as ADR-0004.
- **Task 3 — ledger amendment and ROADMAP correction.** Amended `.planning/ledgers/milestone-01.md`
  in place per D-02: 11 existing rows changed verdict class or evidence, 3 new nested rows added
  (the ADR-0007 cancellation split, and two gaps plan 02-08's review surfaced), the Outstanding item
  reconciliation section's counts and verdict distribution updated with full shown arithmetic
  (155 → 158 rows; the 39/39 task-list-checkbox reconciliation stays intact since no task-list-sourced
  row was removed), and all three planning-input corrections recorded (D-15's incomplete
  construction-site list, ADR-0001's incomplete Code Locations, the false doctest-workspace-wide
  research claim). Corrected `.planning/ROADMAP.md` Phase 2 criterion 1 (dropped the stale "fails
  today" premise) and criterion 5 (dropped the already-true "one `BattalionResult`" clause), with a
  dated amendment note; confirmed via `git diff -U0` that no other ROADMAP section changed.
  `cargo test --workspace` stayed green (0 failed) after this doc-only task, run natively inside
  this worktree as the final verification.

## Task Commits

Each task was committed atomically:

1. **Task 1: Sweep every `tests/` directory against the compiled test targets (D-12)** - `0f8b58d` (docs)
2. **Task 2: Mint ADR-0007 recording the battalion-cancellation deferral (D-05/D-08)** - `96e9be6` (docs)
3. **Task 3: Amend the ledger in place and correct the ROADMAP's stale premises** - `0dd3ae9` (docs)

**Plan metadata:** this SUMMARY's own commit, made immediately after this file.

## Files Created/Modified

- `.planning/phases/02-functional-gap-closure/02-test-wiring-sweep.md` (created) - The D-12 sweep
  record: raw `cargo test --workspace -- --list` evidence summarised by target, a full sweep table
  covering `tests/unit/`, `tests/integration/`, `tests/cli/`, `tests/functional/`, `tests/helpers/`,
  every other direct child of `tests/`, `benches/` and `examples/`, the `LlmProviderError`
  dead-conversion finding, and a "Findings requiring a decision" section.
- `.planning/decisions/0007-battalion-cancellation-deferral.md` (created) - ADR-0007, the
  battalion-cancellation deferral decision, following ADR-0004's exact section shape.
- `.planning/decisions/PROMOTION.md` (modified, Rule 2 deviation) - Numbering index gained a row
  for ADR-0007; "Next free ADR number" advanced from 0007 to 0008.
- `.planning/ledgers/milestone-01.md` (modified) - See "Ledger rows amended" below for the full
  before/after table.
- `.planning/ROADMAP.md` (modified) - Phase 2 criterion 1 and criterion 5 corrected at source, with
  a dated amendment note; no other section touched.

## Ledger rows amended (for Phases 5, 7, 10, 13 to inherit as the D-02 convention worked example)

| Row | Old verdict | New verdict | Evidence cited |
|---|---|---|---|
| Epic 8, `REQ-herald-battalion-result-fields` | satisfied (on false evidence) | satisfied (on real evidence) | Plan 02-04's direct-code-reading contradiction (Table Herald ignored its argument) + plan 02-04's fix (Formation aggregation, all three Heralds rendering real data) + plan 02-05's `test_formation_result_through_json_markdown_table_heralds`/`test_formation_partial_results_through_all_three_heralds` |
| Epic 8, nested task 7.13 | present, unproven | satisfied | Plan 02-05's two end-to-end tests, `cargo test --test lib -- integration::battalion_herald_end_to_end_test` (2 passed) |
| Epic 9, nested task 13.4 | genuinely outstanding | satisfied | Plan 02-07's `test_paladin_basic_execution`; correction to the "never written" finding — the test existed, only the helper shim was missing |
| Epic 9, nested task 13.5 | genuinely outstanding | satisfied | Plan 02-07's `test_formation_basic_sequential_execution`, same correction |
| Epic 9, nested task 13.6 | genuinely outstanding | satisfied | Plan 02-07's `test_phalanx_basic_parallel_execution`, same correction |
| Epic 6, `REQ-provider-testing` | present, unproven | satisfied | Plan 02-06: `tests/unit/mod.rs`'s missing `pub mod llm;` added, 25/25 tests passing, 0 removed, `cargo test --test unit -- llm` (41 passed) |
| Epic 6, nested task 7.10 | genuinely outstanding | satisfied | Plan 02-06's `tests/integration/provider_switching_test.rs`, `cargo test --test lib -- integration::provider_switching_test` (2 passed) |
| Epic 6, nested task 7.14 | genuinely outstanding | deferred with reason | D-09: CI-workflow change, out of Phase 2 scope; owner Phase 15 (PIPE), blocked on Phase 5's VERIFY-06 |
| Epic 4, `REQ-battalion-cancellation` | present, unproven | satisfied (Phalanx) | ADR-0007; `phalanx_service.rs:151`, tested at `:758` |
| Epic 4, new nested row (cancellation split) | *(new row)* | deferred with reason | ADR-0007 as deferring authority; forward owner the v2 backlog |
| Epic 2, `REQ-garrison-testing` | present, unproven | satisfied | Plan 02-08's `02-garrison-prd-review.md` (50 rows), the citable artifact the row previously lacked |
| Epic 2, nested task 11.5 | genuinely outstanding | superseded by shipped code | ADR-0006's workspace-wide 84% floor, per D-04; forward owner QUAL-01 |
| Epic 2, nested task 11.6 | genuinely outstanding | satisfied | `02-garrison-prd-review.md`'s existence and D-19-bar contents |
| Epic 2, new nested row (`PaladinError::GarrisonRequired`) | *(new row)* | genuinely outstanding | Plan 02-08's finding: defined, never constructed; forward owner the v2 backlog (candidate) |
| Epic 2, new nested row (`GarrisonSettings::validate()`) | *(new row)* | present, unproven | Plan 02-08's finding: unit-tested but disconnected from the config-loading path; forward owner the v2 backlog (candidate) |
| Epic 5, `REQ-commander-auto-selection` | satisfied | satisfied (evidence tightened) | Plan 02-01's exact re-run command, `cargo test -p paladin-battalion test_auto_selects` (7 passed), replacing inference with a command that ran |

**Rows re-proved with zero contradiction (stated explicitly per the plan's own instruction, not
left to inference):** plan 02-01's baseline re-proof of GAP-01, GAP-02, GAP-04 and GAP-05 found all
four agreeing with the ledger rows they checked; no amendment above is driven by a GAP-01/02/04/05
contradiction.

**Three planning-input corrections recorded:** (1) CONTEXT.md D-15's `ProviderCapabilities`
construction-site list omitted the OpenAI/Anthropic adapters — plan 02-02's SUMMARY carries the
full compiler-verified list; (2) ADR-0001's `Code Locations` omitted `citadel_port.rs` and
`citadel_integration_test.rs` — plan 02-03's SUMMARY carries the corrected six-file count; (3) the
research document's claim that a missed doc example fails `cargo test` workspace-wide is false for
`paladin-ports` (`doctest = false`), making DEBT-03 load-bearing for future renames in that crate.

## Decisions Made

- Represented ADR-0007's cancellation split as one amended `REQ-*` row (Phalanx, now satisfied) plus
  one new nested row (the other three patterns, deferred with reason) rather than inventing a sixth
  verdict class or a second `REQ-*` ID, preserving the ledger's one-verdict-per-row contract.
- Assigned forward owners for plan 02-08's two newly-surfaced Garrison gaps
  (`PaladinError::GarrisonRequired`, `GarrisonSettings::validate()`) as "the v2 backlog (candidate)",
  since no existing GAP-*/QUAL-* requirement names either construction path and plan 02-08's own
  SUMMARY explicitly deferred this assignment to this plan.
- Updated `.planning/decisions/PROMOTION.md`'s numbering index and "Next free ADR number" line
  (Rule 2 deviation, see below) so a future promoting phase does not collide on ADR number 0007.
- Recomputed the ledger's final verdict-class distribution by direct grep extraction against every
  row's own Verdict token (both the `REQ-*` table column and the final bolded word in each nested
  item), after an initial manual delta-tracking approach produced a one-off discrepancy between two
  adjacent classes — the extracted, file-verified counts are what both the ledger and this SUMMARY
  report (satisfied 110, present-unproven 19, genuinely-outstanding 5, superseded 21, deferred 3,
  total 158).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - missing critical functionality] Updated `PROMOTION.md`'s ADR numbering index**
- **Found during:** Task 2 (minting ADR-0007)
- **Issue:** `PROMOTION.md`'s "Next free ADR number: 0007" line, and its numbering-index table,
  would go stale the moment ADR-0007 was minted — leaving it unchanged would cause a future phase
  (5, 7, 10 or 13) reading that line to reuse number 0007 for a different ADR, per the file's own
  documented purpose ("Phases 5, 7, 10 and 13 take the next free number from this line ... they do
  not need to `ls` the directory to find it").
- **Fix:** Added a numbering-index row for `0007-battalion-cancellation-deferral` and advanced the
  "Next free ADR number" line to `0008`.
- **Files modified:** `.planning/decisions/PROMOTION.md` (not in the plan's declared
  `files_modified`).
- **Verification:** `PROMOTION.md`'s own Part A step 5 ("Updating the `Next free ADR number` line in
  this file") documents this as a required part of minting any new ADR number; the ADR parser
  itself does not depend on this line, so no functional check applies beyond the file's own
  documented contract.
- **Committed in:** `96e9be6` (Task 2 commit).

---

**Total deviations:** 1 auto-fixed (Rule 2, missing critical functionality).
**Impact on plan:** Necessary to keep the shared ADR numbering index correct for sibling
ground-truth phases; no scope creep — the edit is confined to the one line and one table row
`PROMOTION.md`'s own procedure requires when a new ADR number is consumed.

## Issues Encountered

- **Worktree/shared-checkout boundary.** Several early investigative commands (file reads via
  `grep`/`find`, and an initial `cargo test --workspace -- --list` run) were run with `cd /workspace`
  before a subsequent `git` command triggered the harness's worktree-isolation guard, revealing that
  `/workspace` is the shared checkout, distinct from this plan's isolated worktree
  (`/workspace/.claude/worktrees/agent-ab775cf0e0adad5c0`). Verified both trees were content-identical
  at that point — same HEAD commit (`b9b0abe`), no wave-4 sibling plans running concurrently (this
  wave has only plan 02-09), and no edits made from this session yet — before trusting any
  already-gathered data. From that point on, every command ran against the default (worktree) cwd
  only; no further `cd /workspace` was used, and no write of any kind (edit, commit) ever touched
  `/workspace`. The authoritative `cargo test --workspace -- --list` (35 binaries/doctest-groups) and
  the final `cargo test --workspace` verification were both re-run natively inside this worktree and
  corroborated the earlier, shared-checkout-derived data exactly.
- The repository's pre-commit hook runs `cargo clippy --workspace -D warnings`, which takes several
  minutes on a cold or partially-warm `target/` directory; this worktree's `target/` was empty at
  session start (each worktree has its own), so the first background build (`cargo test --workspace
  -- --list`) took several minutes before any commit could proceed quickly. No hook failure occurred
  at any commit; this was purely a timing/warm-cache consideration handled via a background task.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Phase 2 is now closed on its own record-keeping terms**: every Milestone-1 functional
  requirement this phase touched is either recorded working-and-tested with a named exerciser, or
  recorded deferred with a reason, an authority (ADR-0007 or D-09) and a named owner (the v2
  backlog, or Phase 15/PIPE).
- **Phases 5, 7, 10 and 13** inherit the amend-in-place convention (D-02) worked exactly as shown in
  the "Ledger rows amended" table above — each phase's own sibling ledger
  (`milestone-02-03.md`/`milestone-04-06.md`/`milestone-07-08.md`/`milestone-09-12.md`) should follow
  the same shape: dated amendment note, per-row old/new verdict with cited evidence, and an updated
  reconciliation section with shown arithmetic.
- **Phase 3** inherits three named forward-owner candidates from this plan's findings: the four
  still-commented `tests/cli/` files (66 test functions), the `LlmProviderError` dead-conversion
  path, and (already carried from plan 02-01) the GAP-04 edge-probe classification gap — none
  formally claimed, all named for Phase 3 to pick up or reassign.
- **The v2 backlog** inherits the battalion-wide cancellation contract (ADR-0007's own prerequisite)
  and the two Garrison findings (`PaladinError::GarrisonRequired`, `GarrisonSettings::validate()`).
- **Phase 4** should correct ROADMAP's own stale "22 examples" figure at source (this plan's Task 1
  found 46) when it next touches its own success-criteria wording — not amended here, since Task 3's
  ROADMAP edit scope was confined to the Phase 2 section only.
- No blockers: this plan touched zero Rust source files, and `cargo test --workspace` stayed green
  (0 failed) at the final commit, run natively inside this worktree.

---
*Phase: 02-functional-gap-closure*
*Completed: 2026-08-01*
