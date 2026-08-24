---
phase: 07-workspace-ground-truth-recorded-answers
plan: 07
subsystem: docs
tags: [adr, records, binary-targets, build-benchmark, cli-placement, hexagonal-architecture]

# Dependency graph
requires:
  - phase: 07-workspace-ground-truth-recorded-answers
    provides: 07-CONTEXT.md D-19 through D-25a, 07-RESEARCH.md's verified crate-level facts
provides:
  - ADR-0019 (binary-target architecture and per-binary purpose, ARCH-06)
  - ADR-0020 (build-benchmark target restated per scenario, ARCH-07)
  - ADR-0021 (CLI application-layer placement, PROMOTION.md candidate 2)
affects: [07-12-ledger-fanout, 07-13-summary-bookkeeping, phase-08-verified-defect-closure, phase-16-docs-update]

# Tech tracking
tech-stack:
  added: []
  patterns: [ADR file shape (Status/Context/Decision/Considered Options/Code Locations/Code Conformance/Downstream Consumers, no frontmatter), PROMOTION.md candidate-promotion procedure]

key-files:
  created:
    - .planning/decisions/0019-binary-target-architecture.md
    - .planning/decisions/0020-build-benchmark-per-scenario.md
    - .planning/decisions/0021-cli-application-layer-placement.md
  modified: []

key-decisions:
  - "ADR-0019 ratifies the three shipped [[bin]] targets (paladin, paladin-cli, paladin-server), each with a stated purpose, recording paladin's honest pre-Paladin content-aggregator identity (structopt name smartcontent-aggregator) rather than inventing a tidy purpose"
  - "ADR-0019 records that structopt's only consumer in the tree is the un-gated src/main.rs (3 grep hits, 1 file), so structopt cannot be marked optional without deciding src/main.rs's fate — re-scoping Phase 8's recorded 'three-line fix'"
  - "ADR-0019 records that paladin-herald re-introduces colored and comfy-table unconditionally with no [features] section, independently of the root manifest's own copies of those two crates"
  - "ADR-0020 transcribes build-benchmarks.md's five scenario figures verbatim (two pass, three fail against >=50%), restates SM-7 per scenario, cites the summary table's -6.6% as authoritative over the conclusion's -5% (a transcription error), judges 'Target achieved' as contradicted by the report's own table, and declines the report's own recommended re-measurement with a recorded reason"
  - "ADR-0021 promotes PROMOTION.md Part B candidate 2 (epic17-5.md), ratifying src/application/cli as the CLI's recorded home"
  - "Deviation: ADR-0021 does NOT record 'pub mod cli;' as un-gated as the plan's must_haves instructed — re-grepping the live tree found src/application/mod.rs:57-59 and src/lib.rs:155-156 both already gate the CLI module and its re-export behind #[cfg(feature = \"cli\")]. This corrects 07-CONTEXT.md's D-25a research finding, which was stale; ADR-0021 records the correction explicitly rather than repeating a false premise"

requirements-completed: [ARCH-06, ARCH-07]

coverage:
  - id: D1
    description: "ADR-0019 ratifies three binary targets with a stated purpose each and records the structopt/paladin-herald coupling that re-scopes Phase 8"
    requirement: "ARCH-06"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0019-binary-target-architecture.md == 7; grep -c smartcontent-aggregator/paladin-herald >= 1; grep -rln structopt src/ crates/ | wc -l == 1"
        status: pass
    human_judgment: true
    rationale: "Verification confirms document shape and cited facts mechanically, but whether the recorded purposes and the Phase 8 re-scoping are substantively correct and complete is a judgment call a human reviewer should confirm, per the plan's human-check on ADR-0020's sibling figures."
  - id: D2
    description: "ADR-0020 restates SM-7 per scenario with the report's five transcribed figures and declines re-measurement with a recorded reason"
    requirement: "ARCH-07"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0020-build-benchmark-per-scenario.md == 7; the five improvement figures each appear in build-benchmarks.md; git diff --numstat HEAD -- build-benchmarks.md is empty"
        status: pass
    human_judgment: true
    rationale: "Plan's own <verify> requires a human-check: 'Diff ADR-0020's five figures against build-benchmarks.md's summary table and confirm every number matches the source' (07-VALIDATION.md Manual-Only Verifications row 3)."
  - id: D3
    description: "ADR-0021 promotes PROMOTION.md candidate 2, ratifying src/application/cli placement"
    verification:
      - kind: other
        ref: "grep -c '^## ' .planning/decisions/0021-cli-application-layer-placement.md == 7; test -d src/application/cli && ! test -d src/cli; grep -q epic17-5.md"
        status: pass
    human_judgment: false

duration: 35min
completed: 2026-08-06
status: complete
---

# Phase 7 Plan 07: Binary-Target, Build-Benchmark, and CLI-Placement ADRs Summary

**Three new ADRs (0019-0021) close the never-produced Milestone 4 binary-architecture review, restate the Milestone 5 build-benchmark target per scenario with a declined re-measurement, and promote PROMOTION.md's second Phase-7-owned candidate for CLI placement.**

## Performance

- **Duration:** ~35 min
- **Completed:** 2026-08-06T18:54:34Z
- **Tasks:** 3
- **Files modified:** 3 (all new)

## Accomplishments

- **ADR-0019** ratifies the three shipped `[[bin]]` targets in `Cargo.toml` declaration order —
  `paladin` (`src/main.rs`, no `required-features`), `paladin-cli` (`required-features = ["cli"]`),
  `paladin-server` (`required-features = ["web-server"]`) — and gives each a stated purpose,
  including the honest finding that `paladin` is the pre-Paladin content-aggregation service runner
  (`#[structopt(name = "smartcontent-aggregator")]`, `src/main.rs:8`), recorded rather than tidied
  away. It also records the coupling that re-scopes Phase 8: `structopt`'s only consumer in the
  entire tree is the un-gated `src/main.rs` (`grep -rn structopt src/ crates/` returns three hits,
  all in that one file — `src/main.rs:5,8,10`), so `structopt` cannot be marked `optional = true`
  without first deciding `src/main.rs`'s fate; and `crates/paladin-herald/Cargo.toml:22-23` declares
  `comfy-table`/`colored` unconditionally with no `[features]` section, re-introducing both into any
  build regardless of the root manifest's own `cli` feature, since `paladin-herald` is itself a
  required (non-optional) workspace dependency (`Cargo.toml:22,54`).
- **ADR-0020** transcribes `build-benchmarks.md`'s five scenario figures verbatim (clean build
  −6.6% fail, `paladin-core` incremental −18.9% fail, `paladin-llm` incremental −44.6% fail,
  `paladin-memory` incremental −50.2% pass, `paladin-battalion`-only −90.9% pass), restates
  Milestone 5 SM-7 as a per-scenario target, resolves the report's own internal
  −6.6%/−5% inconsistency by citing the summary table as authoritative and naming the conclusion's
  figure a transcription error, judges the report's "Overall verdict: Target achieved" as
  contradicted by its own table (without reconciling it), and declines the report's own recommended
  mid-tree re-measurement with a recorded reason — the pre-workspace monolith baseline commit
  (`08dc944`) no longer exists in buildable form on this branch, and the environment carries
  documented offline/no-Docker constraints.
- **ADR-0021** promotes `PROMOTION.md` Part B candidate 2 (`epic17-5.md`), ratifying
  `src/application/cli` as the CLI's recorded home per Hexagonal Architecture reasoning (CLI as an
  input adapter, not infrastructure), citing the shipped tree that already implements it
  (`src/application/cli/` exists, `src/cli/` does not).

## Task Commits

All three ADRs were committed in a single combined commit per the plan's explicit instruction
("Commit this plan's three ADR files in a single commit at the end of the plan"):

1. **Task 1: ADR-0019 — binary-target architecture and per-binary purpose**
2. **Task 2: ADR-0021 — CLI placement in the application layer**
3. **Task 3: ADR-0020 — the build-time benchmark restated per scenario**

**Combined commit:** `0ed0620` — `docs(07-07): add ADR-0019, ADR-0020, ADR-0021 — binary targets, build benchmark, CLI placement`

## Files Created/Modified

- `.planning/decisions/0019-binary-target-architecture.md` — new ADR, `must change` (Phase 16
  mdbook page owed; Phase 8 receives the structopt/paladin-herald re-scoping)
- `.planning/decisions/0020-build-benchmark-per-scenario.md` — new ADR, `conforms`
- `.planning/decisions/0021-cli-application-layer-placement.md` — new ADR, `conforms`, closes
  `PROMOTION.md` candidate 2

No `.rs`, `Cargo.toml`, `.github/`, or `.project/` file was modified — verified via
`git diff --stat HEAD~1 -- '*.rs' 'Cargo.toml' '.github/'` (empty) and
`git diff --numstat HEAD -- .project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md`
(empty).

## Decisions Made

- **`structopt` grep result, verbatim:** `grep -rn structopt src/ crates/` returns three hits
  (`src/main.rs:5`, `src/main.rs:8`, `src/main.rs:10`), confined to one file. This confirms
  `07-CONTEXT.md` D-20's "three hits, all in that file" claim exactly, re-verified fresh this task.
- **`paladin-herald` manifest facts, as read:** `crates/paladin-herald/Cargo.toml:22`
  (`comfy-table = "7.1"`) and `:23` (`colored = "2.1"`) are unconditional dependencies; the file
  carries no `[features]` section at all (confirmed by reading the full manifest, 28 lines). The
  root `Cargo.toml` also carries `colored` (`:125`) and `comfy-table` (`:126`) unconditionally
  (neither marked `optional = true`), and `paladin-herald` is itself a required, non-optional
  workspace dependency (`Cargo.toml:22,54`) — so even a fully-gated root manifest could not exclude
  these two crates from a library-only build while `paladin-herald` remains a required dependency.
- **The five benchmark figures, with source line numbers:** all read from
  `.project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md`'s Summary Table
  (`:65-71`) — clean build −6.6% (`:67`), `paladin-core` incremental −18.9% (`:68`), `paladin-llm`
  incremental −44.6% (`:69`), `paladin-memory` incremental −50.2% (`:70`), `paladin-battalion`-only
  −90.9% (`:71`). The Conclusion's differing clean-build figure (−5%, `:109`) is recorded as the
  transcription error, not a competing measurement.
- **Deviation, flagged for the phase verifier:** the plan's `must_haves` and Task 2 `<action>`
  instructed ADR-0021 to record `src/application/mod.rs:59`'s `pub mod cli;` as **un-gated**, per
  `07-CONTEXT.md` D-25a's research finding. Re-grepping the live tree this task found that claim
  stale: `src/application/mod.rs:57-59` gates the module declaration behind
  `#[cfg(feature = "cli")]`, and `src/lib.rs:155-156` gates the corresponding re-export the same way.
  Per the plan's own phase-character instruction ("Re-grep every `file:line` citation against the
  live tree before writing it. Prior documents have drifted"), ADR-0021 records the corrected fact
  rather than repeating the stale premise, and states this correction explicitly in its own text so
  a reader does not need to cross-reference this SUMMARY to find it. This does not weaken the
  ADR-0019/Phase-8 findings — those (the `structopt`/`src/main.rs` coupling and the `paladin-herald`
  dependency-declaration hole) were independently re-verified this task and stand unchanged; the
  correction narrows the *reason* the CLI surface remains a library-only-build concern to the
  dependency-declaration layer (FR2/FR5), not the module-gating layer (FR1), which is already
  satisfied.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug/stale premise] Corrected ADR-0021's "un-gated `pub mod cli;`" claim to match the live tree**
- **Found during:** Task 2 (ADR-0021)
- **Issue:** The plan's `must_haves` and Task 2 `<action>` (sourced from `07-CONTEXT.md` D-25a's
  research) asserted `src/application/mod.rs:59`'s `pub mod cli;` declaration is un-gated and
  compiles into the library regardless of the `cli` feature. Re-grepping during this task found
  both the module declaration (`src/application/mod.rs:57-59`) and its `src/lib.rs:155-156`
  re-export already correctly gated behind `#[cfg(feature = "cli")]`.
- **Fix:** ADR-0021's `## Decision` records the corrected finding explicitly — including naming
  that the earlier research description does not hold against the tree at authoring time — instead
  of repeating the stale claim. ADR-0019's own text was also written to state the same corrected
  fact plainly, so both new ADRs are internally consistent.
- **Files modified:** `.planning/decisions/0021-cli-application-layer-placement.md`,
  `.planning/decisions/0019-binary-target-architecture.md`
- **Verification:** `sed -n '57,59p' src/application/mod.rs` and `sed -n '155,156p' src/lib.rs`
  both show `#[cfg(feature = "cli")]` immediately preceding the module/re-export line; confirmed via
  `git log -p --follow -- src/application/mod.rs`, which shows the gate predates this phase (not a
  same-session artifact).
- **Committed in:** `0ed0620` (the plan's single combined commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 — corrected a stale premise inherited from research)
**Impact on plan:** The correction is additive precision, not scope creep — it keeps the phase's
ground-truth mandate intact (do not repeat a claim the live tree contradicts) without weakening any
`must_haves` truth: the `structopt` and `paladin-herald` findings the phase actually cares about for
Phase 8's re-scoping are unaffected and independently re-verified.

## Issues Encountered

None beyond the deviation above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **Phase 16** has its mdbook binary-architecture page deliverable named by ADR-0019's
  `Code Conformance: must change` verdict.
- **Phase 8's CLI-isolation requirement** receives ADR-0019's two-part finding (the `structopt`
  precondition and the `paladin-herald` hole) plus ADR-0021's corrected module-gating finding — its
  recorded "three-line fix" needs re-scoping to the dependency-declaration layer specifically.
- **Plan 07-12** (ledger fan-out) can cite ADR-0019 for `REQ-binary-target-config`, `REQ-cli-docs`,
  `REQ-cli-dependency-isolation`, `REQ-library-only-build`; ADR-0020 for `REQ-build-benchmark-report`,
  `REQ-llm-build-validation`; ADR-0021 for the Milestone 4 Epic 3 CLI-placement rows.
- **Plan 07-13** (summary/bookkeeping) must add all three ADRs' rows to
  `.planning/decisions/PROMOTION.md` and advance the "Next free ADR number" line to **0022**
  (0021 consumed by ADR-0021, not the originally-planned 0021→0021-stops-here reading) — this plan
  did not touch `PROMOTION.md` per its `files_modified` scope.
- No blockers for downstream plans in this wave; `.planning/ledgers/milestone-04-06.md` was not
  touched, preserving the concurrent plan 07-08's exclusive ownership of that file.

## Known Stubs

None.

---
*Phase: 07-workspace-ground-truth-recorded-answers*
*Completed: 2026-08-06*
