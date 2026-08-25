---
phase: 05-milestone-2-3-ground-truth
plan: 01
subsystem: docs
tags: [ledger, requirements-traceability, sanctum, adr-precursor, verification]

# Dependency graph
requires:
  - phase: 01-ground-truth-decision-records
    provides: "D-00a..D-00g conventions, the ledger shape (milestone-01.md), and the ADR/precedence house style this plan inherits verbatim"
provides:
  - ".planning/ledgers/milestone-02-03.md scaffold: head notes, five-value verdict legend, row-order/amendment convention, all 14 epic sections, 118 keyed REQ-* rows"
  - "Epic 11 (Sanctum Memory Foundation) fully cited end-to-end: 7 satisfied, 1 present-unproven, proving the citation workflow before 11 more plans fan out into it"
  - "REQUIREMENTS.md's Milestone 2-3 as-shipped ledger reduced to a pointer (D-21)"
  - "110 PENDING-VERDICT stub rows keyed to their owning plan (05-05..05-12), the fixed shape every later ledger plan appends into"
affects: [05-05, 05-06, 05-07, 05-08, 05-09, 05-10, 05-11, 05-12, 05-13, "06 (Verified Gap Closure)"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Ledger amend-in-place convention (D-00g) inherited unchanged from Phase 1"
    - "D-01 two-part evidence bar: file:line citation AND a named passing test/example/command actually run — no ledger row satisfied on citation alone"

key-files:
  created:
    - .planning/ledgers/milestone-02-03.md
  modified:
    - .planning/REQUIREMENTS.md

key-decisions:
  - "REQ-qdrant-sanctum-adapter-v1 verdicted present, unproven, not satisfied — the adapter compiles clean under --features qdrant but its own 15-test exerciser (tests/integration/qdrant_sanctum_tests.rs) is #[ignore]d requiring a live Qdrant instance, and this sandbox has no docker binary to start one"
  - "REQ-sanctum-garrison-coexistence verdicted satisfied via structural evidence (independent module wiring at lib.rs:42/:48 and config/mod.rs:3/:5) plus the passing 13/13 Sanctum-config test run, since coexistence is a structural claim rather than a single function under test"

requirements-completed: []  # VERIFY-01 spans plans 05-01..05-13; not completable until 05-13 closes it out

coverage:
  - id: D1
    description: "Ledger scaffold with head notes (supersession, primary key, D-01 evidence bar, D-04 path caveats), five-value verdict legend, and row-order/amendment convention"
    verification:
      - kind: other
        ref: "grep -c '^## Verdict legend' / '^## Row order and amendment convention' .planning/ledgers/milestone-02-03.md — both equal 1"
        status: pass
    human_judgment: false
  - id: D2
    description: "All 14 epic sections present in REQUIREMENTS.md's own run-2 order, 118 REQ-* rows total, no duplicate IDs"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' (118), grep -c '^### Epic ' (14), grep -o '^| REQ-[a-z0-9-]*' | sort | uniq -d (empty) — all run against .planning/ledgers/milestone-02-03.md"
        status: pass
    human_judgment: false
  - id: D3
    description: "Epic 11's 8 rows fully cited: file:line resolved against the shipped tree, satisfied rows backed by a test/command actually executed during this task"
    verification:
      - kind: unit
        ref: "cargo test --offline -p paladin-memory sanctum:: — 13/13 passed"
      - kind: integration
        ref: "cargo test --offline --test in_memory_sanctum_integration — 17/17 passed"
      - kind: integration
        ref: "cargo test --offline --features openai-embeddings --test openai_embedding_integration — 8/8 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ai-core sanctum:: — 3/3 passed"
      - kind: unit
        ref: "cargo test --offline -p paladin-ports sanctum — 3/3 passed"
      - kind: other
        ref: "cargo build --offline -p paladin-memory --features qdrant — exit 0 (compile-only evidence for the present-unproven Qdrant row)"
        status: pass
    human_judgment: true
    rationale: "Plan's own <human-check> requires a human to read three Epic 11 rows end to end and confirm the named exerciser asserts the requirement's behaviour rather than merely importing its symbol (05-VALIDATION.md §Manual-Only Verifications, row 1)."
  - id: D4
    description: "REQUIREMENTS.md's Milestone 2-3 as-shipped ledger section body replaced with a pointer to the new ledger file, heading retained, neighbouring sections intact"
    verification:
      - kind: other
        ref: "sed -n '/^## Milestone 2-3/,/^## Milestone 4-6/p' .planning/REQUIREMENTS.md | grep -c '^| REQ-' equals 0; grep -c 'ledgers/milestone-02-03.md' equals 1; git log -1 --name-only shows both files in one commit"
        status: pass
    human_judgment: false

duration: ~70min
completed: 2026-08-04
status: complete
---

# Phase 5 Plan 01: Milestone 2-3 ledger scaffold + Epic 11 tracer Summary

**Created `.planning/ledgers/milestone-02-03.md` (118 `REQ-*` rows, 14 fixed-order epic sections) and fully cited Epic 11's 8 Sanctum-memory rows against the shipped tree — 7 `satisfied` with a test actually run, 1 `present, unproven` because its Qdrant exerciser needs Docker this sandbox doesn't have.**

## Performance

- **Duration:** ~70 min (dominated by cold, multi-agent-contended workspace compiles — several `cargo test`/`cargo build` invocations took 2-7 minutes each)
- **Tasks:** 2
- **Files modified:** 2 (1 created, 1 edited)

## Accomplishments

- Scaffolded the entire Milestone 2-3 ledger shape — head notes (supersession, primary-key, D-01
  evidence bar, D-04 path caveats), the five-value verdict legend copied verbatim from
  `milestone-01.md`, the row-order/amendment convention, and all 14 epic section headings in
  REQUIREMENTS.md's own run-2 order — so 11 more fan-out plans (05-05..05-12) and the close-out
  plan (05-13) append into a fixed shape rather than inventing their own.
- Fully walked Epic 11 (Sanctum Memory Foundation, 8 requirement IDs) through the citation workflow
  end-to-end: resolved every `file:line` citation against the current ten-crate workspace layout,
  searched for and ran the exercising test/command for each, and assigned one of the five legend
  verdicts per row rather than accepting the ingest ledger's bare `Shipped` claims.
- Found and recorded the one Epic 11 row that does not clear the `satisfied` bar:
  `REQ-qdrant-sanctum-adapter-v1` compiles cleanly (`cargo build --offline -p paladin-memory
  --features qdrant`, exit 0) but its dedicated 15-test exerciser is entirely `#[ignore]`d pending a
  live Qdrant instance this environment cannot start (no `docker` binary) — verdicted `present,
  unproven`, not `satisfied`, and not silently upgraded on citation strength alone (the plan's P1
  prohibition).
- Reduced REQUIREMENTS.md's `## Milestone 2-3 as-shipped ledger` section (246 lines, 118 inline
  component-level rows across 14 epics) to an 8-line pointer paragraph modelled on the existing
  Milestone 1 pointer, per D-21 — REQUIREMENTS.md no longer carries a second, diverging copy of the
  118 verdicts.
- Assigned the 110 non-Epic-11 rows their owning plan number (`PENDING-VERDICT` stub verdict, plan
  number in the Evidence cell) per the plan's fixed fan-out table, so every later plan can locate its
  rows by grep without re-deriving the assignment.

## Task Commits

Both tasks were committed together in a single commit, per the plan's explicit instruction (the
repo's pre-commit hooks compile the full 12-crate workspace on every commit including
markdown-only ones, so per-task commits here would double that cost with no benefit):

1. **Task 1: Ledger scaffold plus Epic 11 fully cited** — part of `71b6d7b` (docs)
2. **Task 2: Reduce REQUIREMENTS.md's Milestone 2-3 ledger section to a pointer** — part of `71b6d7b` (docs)

`71b6d7b` — `docs(05-01): create Milestone 2-3 cited status ledger, reduce REQUIREMENTS.md to pointer`

_No plan-metadata commit is separate from the task commit above — SUMMARY.md/STATE.md/ROADMAP.md
updates are governed by worktree mode (see Deviations) and are committed by the orchestrator after
merge, per this plan's parallel-execution instructions._

## Files Created/Modified

- `.planning/ledgers/milestone-02-03.md` — new file, 118 `REQ-*` rows across 14 epic sections, Epic
  11 fully cited
- `.planning/REQUIREMENTS.md` — `## Milestone 2-3 as-shipped ledger` section body replaced with an
  8-line pointer; heading retained; `## Milestone 1 as-shipped ledger` and `## Milestone 4-6
  as-shipped ledger` sections confirmed untouched

## Decisions Made

- **REQ-qdrant-sanctum-adapter-v1 → `present, unproven`, not `satisfied`.** The adapter struct, its
  `SanctumPort` impl, and the `qdrant` feature wiring all exist and compile
  (`crates/paladin-memory/src/sanctum/qdrant_adapter.rs:59,377`); its dedicated integration suite
  (`tests/integration/qdrant_sanctum_tests.rs`, the `qdrant_sanctum_integration` `[[test]]` target
  at `Cargo.toml:197-199`) carries `#[ignore = "Requires Qdrant running on localhost:6334"]` on all
  15 test functions. This sandbox has no `docker` binary (`command -v docker` returns nothing), so
  nothing exercises the adapter here — the D-01 evidence bar's "a citation with nothing exercising
  it is `present, unproven`" applies exactly as written, even though the code plainly exists and
  even though the Epic 11 ingest record's "DEFERRED" claim is separately confirmed stale.
- **REQ-sanctum-garrison-coexistence → `satisfied` via structural evidence, not a single function
  test.** The requirement is a claim about module independence, not behaviour, so its "test... that
  exercises it" is the combination of (a) direct listing of the two independent module trees and
  their separate `pub mod` wiring at named `lib.rs`/`config/mod.rs` line numbers, and (b) the
  passing 13/13 Sanctum-scoped test run, which exercises the Sanctum config path with zero
  dependency on any Garrison test passing.
- **Both plan tasks committed together, not per-task.** The plan's own Task 2 action explicitly
  instructs a single end-of-plan commit because the repo's pre-commit hooks
  (`cargo fmt --check` + `cargo clippy -- -D warnings`) recompile the full workspace on every commit
  regardless of whether the change is markdown-only, and this machine's compiles run several
  minutes each under multi-agent contention. This overrides the standard per-task atomic-commit
  protocol for this plan only, as the plan itself directs.

## Deviations from Plan

None — plan executed exactly as written, including its own explicit override of the default
per-task commit protocol (see "Decisions Made" above, third bullet).

**Worktree-mode note (not a deviation, expected behavior):** per this execution's
`<parallel_execution>` instructions, STATE.md and ROADMAP.md are not modified by this plan — the
orchestrator updates them centrally after the wave merges. This differs from the standalone
`execute-plan.md` workflow's `<state_updates>`/`<final_commit>` steps, which are skipped here by
design.

## Issues Encountered

- **Heavy build contention from concurrent sibling worktree agents.** `ps aux` during this task
  showed multiple other `cargo build`/`cargo check`/`cargo clippy` processes running in parallel
  worktrees (`agent-f4c8`, `agent-a61a39b23b2797569`, `agent-a756a2936463d9a93`), which pushed
  several `cargo test`/`cargo build` invocations past the default 120s/300s tool timeouts. Resolved
  by re-running the same commands with longer explicit timeouts (up to 590000ms) and, once,
  polling a background job until its output file was non-empty. No code or test changes were
  needed — purely a wall-clock/scheduling issue in this sandbox.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- The ledger shape (head notes, legend, row-order convention, all 14 section headings, all 118 row
  stubs) is locked in and proven by Epic 11's full walkthrough. Plans 05-05..05-12 can each fill
  their assigned rows by locating their `PENDING-VERDICT` / plan-number stub and replacing only the
  Verdict and Evidence cells in place, per the Row order and amendment convention.
- REQUIREMENTS.md carries no second copy of the 118 verdicts to drift out of sync with the ledger.
- One open item for 05-13 (the close-out plan): the ledger's "Nested outstanding-item count is
  finalised by plan 05-13" placeholder sentence in REQUIREMENTS.md still needs its real figure once
  every epic section is filled.
- No blockers for the next wave. This plan (05-01, wave 1) has no `depends_on`, and its output is a
  pure prerequisite for the fan-out plans — nothing here is itself blocked on their completion.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-02-03.md`
- FOUND: `.planning/phases/05-milestone-2-3-ground-truth/05-01-SUMMARY.md`
- FOUND: commit `71b6d7b` (task commit, both plan files)
- FOUND: commit `7ffb2b7` (SUMMARY.md commit)

---
*Phase: 05-milestone-2-3-ground-truth*
*Completed: 2026-08-04*
