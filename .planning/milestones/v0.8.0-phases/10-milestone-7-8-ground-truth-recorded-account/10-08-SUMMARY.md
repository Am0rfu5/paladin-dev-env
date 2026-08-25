---
phase: 10-milestone-7-8-ground-truth-recorded-account
plan: 08
subsystem: docs
tags: [ledger, requirements-traceability, benchmarks, rustsec-governance, licence-posture, milestone-7-8]

requires:
  - phase: 10-milestone-7-8-ground-truth-recorded-account
    provides: "10-01's ledger scaffold (86 row stubs, seven-class legend, supersession summary table), 10-05's ADR-0032 and D-19's audit.toml correction, 10-06's ADR-0033 and its measured doctest/doc-warning findings, and 10-03's ADR-0029 version trajectory"
provides:
  - "Milestone 7 Epic 3 (10 rows) and Epic 4 (12 rows) of .planning/ledgers/milestone-07-08.md fully derived, replacing scaffold pending stubs in place"
  - "cargo bench --workspace --no-run run to completion this session (10m 41s), closing REQ-workspace-bench-execution as satisfied on the strength of a completed compile rather than bench-file existence"
  - "Six Phase-9-closed rows (REQ-crate-metadata-completion, REQ-per-crate-changelog, REQ-rustsec-risk-acceptance, REQ-rustsec-hardening-actions, REQ-license-policy-signoff, REQ-paladin-ports-publish-verification-closed) recorded closed, each citation re-run against the current tree rather than transcribed from the hand-off block"
  - "REQ-doc-coverage-audit recorded present, unproven citing ADR-0033's seven-crate doctest-flag list and count, replacing the stale Contested -> HARD-07 premise"
affects: ["10-09", "10-10", "10-11"]

tech-stack:
  added: []
  patterns:
    - "Cell-replacement-only ledger fan-out: two per-task commits inside one file's disjoint epic ranges, verified via grep -c row/section counts and git diff --numstat added==deleted before each commit"
    - "Long-running verification command run via run_in_background rather than inline Bash, to avoid the tool's 120s/590s foreground timeouts on a cold LTO release-profile workspace bench compile"

key-files:
  created: []
  modified:
    - .planning/ledgers/milestone-07-08.md

key-decisions:
  - "REQ-workspace-bench-execution resolved to satisfied by actually running cargo bench --workspace --no-run to completion this session (10m 41s, all five critical-path bench executables produced), not by inferring a pass from the bench files' existence — the row would have been present, unproven had the compile failed or timed out"
  - "REQ-disabled-bench-disposition recorded superseded by outcome rather than satisfied: none of the five disabled/legacy root benches was restored; three (herald, paladin, arsenal) were deprecated and removed outright by commit 10f9a4e with no replacement, and two (battalion, garrison) were removed by the same commit then replaced at deliberately narrower scope by commit d346ab9"
  - "REQ-critical-path-bench-scope re-derived independently of benchmark-assessment.md's own self-asserted 'PRD Success Metrics Status' table (all six metrics recorded Satisfied by its own hand) -- each of the four FR-16 categories was checked directly against the shipped bench source this session rather than trusting the document's own completion claim"
  - "REQ-performance-baseline-doc kept as a separate relocated row from QUAL-05: the mdbook chapter closes the 'a baseline document exists' requirement but not the separate runtime-measurement requirement, so the two are never conflated"
  - "The six Phase-9-closed rows (REQ-crate-metadata-completion, REQ-per-crate-changelog, REQ-rustsec-risk-acceptance, REQ-rustsec-hardening-actions, REQ-license-policy-signoff, REQ-paladin-ports-publish-verification-closed) are cited, not re-opened, per D-04 -- but every citation (SECURITY-EXCEPTIONS.md's 10 governed rows, both guard scripts wired into CI, the manifest licence field across 11 manifests, the crate/lib name split) was re-run this session against REQUIREMENTS.md:1320-1355's hand-off block, and none had moved"
  - "REQ-doc-coverage-audit flipped from the stale 'Contested -> HARD-07' premise (paladin-ports doctest=false, which Phase 8 already discharged) to present, unproven citing ADR-0033's fresh finding: seven different crates still set doctest=false, four crates' doctests measurably execute, and the count is written rather than left implicit"
  - "REQ-versioning-policy resolved to relocated per the ledger's own tie-break rule even though no document was actually relocated anywhere -- the class is chosen to preserve the moved-not-missing signal against a later phase planning a document search for a page that was never written, with the superseded-by-outcome half stated explicitly in the same cell"
  - "REQ-release-readiness-audit recorded satisfied as closed history, explicitly stating the tree is four minors past rc.1 today, so no reader treats the v0.1.0-rc.1 artefact as live"
  - "REQ-per-crate-readme and REQ-per-crate-changelog land on different verdicts despite both being ten-of-ten-complete facts: changelog has a CI-wired guard script (scripts/check-changelogs.sh) so it is satisfied; readme has no equivalent enforcer so it is present, unproven"

requirements-completed: [HARD-01, HARD-03, HARD-07]

coverage:
  - id: D1
    description: "Milestone 7 Epic 3's ten benchmark rows re-derived from the tree, including a completed cargo bench --workspace --no-run run this session"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "awk '/^### Milestone 7 Epic 3/{p=1;next}/^### /{p=0}p' .planning/ledgers/milestone-07-08.md | grep -c '^| REQ-' == 10; same range grep -c 'pending — plan' == 0; cargo bench --workspace --no-run exit 0 in 10m 41s"
        status: pass
    human_judgment: false
  - id: D2
    description: "Milestone 7 Epic 4's twelve rows re-derived, six Phase-9-closed rows cited with every citation re-run rather than transcribed"
    requirement: "HARD-01"
    verification:
      - kind: other
        ref: "grep -cE 'ADR-002[4-7]' .planning/ledgers/milestone-07-08.md >= 4; awk '/^### Milestone 7 Epic 4/{p=1;next}/^### /{p=0}p' | grep -c '^| REQ-' == 12"
        status: pass
    human_judgment: false
  - id: D3
    description: "REQ-doc-coverage-audit is present, unproven citing ADR-0033 with the seven-crate doctest-flag count written, not satisfied on an unwritten number"
    requirement: "HARD-07"
    verification:
      - kind: other
        ref: "grep -n 'REQ-doc-coverage-audit' .planning/ledgers/milestone-07-08.md shows 'present, unproven' and 'ADR-0033' and the seven crate names"
        status: pass
    human_judgment: false
  - id: D4
    description: "REQ-versioning-policy and REQ-release-readiness-audit both cite ADR-0029, with the release-readiness row recording the rc.1 trajectory as closed history rather than current state"
    requirement: "HARD-03"
    verification:
      - kind: other
        ref: "grep -c 'ADR-0029' .planning/ledgers/milestone-07-08.md >= 2; REQ-release-readiness-audit row contains 'closed history' and 'four minors past'"
        status: pass
    human_judgment: false
  - id: D5
    description: "Ledger row/section inventory unchanged by this plan's cell-replacement-only edits; each task's diff shows added lines equal to deleted lines"
    verification:
      - kind: other
        ref: "grep -c '^| REQ-' .planning/ledgers/milestone-07-08.md == 86; grep -c '^### ' == 12; git diff --numstat for each of the two task commits shows added == deleted (11/11, then 13/13)"
        status: pass
    human_judgment: false
  - id: D6
    description: "No .rs file modified by this plan (D-23 boundary held)"
    verification:
      - kind: other
        ref: "git status --porcelain -- '*.rs' — empty"
        status: pass
    human_judgment: false

duration: ~30min
completed: 2026-08-08
status: complete
---

# Phase 10 Plan 08: Milestone 7 Epic 3-4 Ledger Derivation Summary

**Derived all 22 Milestone 7 Epic 3/4 ledger rows from the tree, running `cargo bench --workspace --no-run` to a real 10m 41s completion for the workspace-execution row and re-running (not transcribing) all six Phase-9-closed citations for the security and metadata rows.**

## Performance

- **Duration:** ~30 min
- **Completed:** 2026-08-08
- **Tasks:** 2
- **Files modified:** 1 (`.planning/ledgers/milestone-07-08.md`)

## Accomplishments

- Re-derived all ten Milestone 7 Epic 3 rows from the tree rather than from `benchmark-assessment.md`'s own self-asserted "PRD Success Metrics Status" table: ran `cargo bench --workspace --no-run` to completion this session (10m 41s, all five critical-path bench executables produced), closing `REQ-workspace-bench-execution` `satisfied` on the strength of the completed compile; recorded `REQ-disabled-bench-disposition` `superseded by outcome` naming which three of the five legacy benches were deprecated-and-removed and which two were removed-then-replaced at narrower scope, by commit; independently re-checked all four FR-16 critical-path categories against the shipped bench source for `REQ-critical-path-bench-scope` rather than trusting the assessment document's own Satisfied table.
- Re-derived all twelve Milestone 7 Epic 4 rows, including the six rows Phase 9 already closed (`REQ-crate-metadata-completion`, `REQ-per-crate-changelog`, `REQ-rustsec-risk-acceptance`, `REQ-rustsec-hardening-actions`, `REQ-license-policy-signoff`, `REQ-paladin-ports-publish-verification-closed`): every one of the six citations (the `SECURITY-EXCEPTIONS.md` register, both CI-wired guard scripts, the licence field across all eleven manifests, the crate/lib name split) was re-run against the current tree this session, per D-04, and none had moved from where `REQUIREMENTS.md:1320-1355`'s hand-off block names them.
- Flipped `REQ-doc-coverage-audit` from its stale "Contested → HARD-07" premise (which rested on a `paladin-ports` doctest exclusion Phase 8 already discharged) to `present, unproven`, citing ADR-0033's fresh finding of seven different crates still setting `doctest = false`, with the exact count and crate list written into the row rather than left implicit.
- Resolved `REQ-versioning-policy` to `relocated` per the ledger's own tie-break rule even though no document was actually relocated anywhere (none exists at any path), preserving the moved-not-missing signal; recorded `REQ-release-readiness-audit` as `satisfied` closed history, stating explicitly that the tree is four minors past the `v0.1.0-rc.1` tag today.
- Confirmed the ledger's row/section inventory is unchanged: `grep -c '^| REQ-'` still reads `86`, `grep -c '^### '` still reads `12`, and each of the two task commits shows equal added/deleted line counts (11/11, then 13/13) — cell replacement only, no row inserted, deleted, or reordered.

## Task Commits

Each task was committed atomically:

1. **Task 1: Derive Milestone 7 Epic 3's ten benchmark rows** — `c4cc768` (feat)
2. **Task 2: Derive Milestone 7 Epic 4's twelve rows, six closed by Phase 9** — `ad5820c` (feat)

_No plan-metadata commit: this executor ran in worktree mode. STATE.md and ROADMAP.md are owned by the orchestrator after all wave-3 agents complete; this SUMMARY.md is committed separately per the worktree execution contract._

## Files Created/Modified

- `.planning/ledgers/milestone-07-08.md` — Milestone 7 Epic 3 (10 rows) and Epic 4 (12 rows) Verdict/Evidence cells replaced in place; epic notes filled for both sections. No other section touched (confirmed via `git diff HEAD~2 HEAD`, a single hunk spanning only lines 211-249).

## Decisions Made

- **`REQ-workspace-bench-execution`** resolved to `satisfied` by actually running `cargo bench --workspace --no-run` to completion this session (10m 41s, run via `run_in_background` to avoid the tool's foreground timeout on a cold LTO release-profile workspace compile), not by inferring a pass from the bench files' existence.
- **`REQ-disabled-bench-disposition`** is `superseded by outcome`, not `satisfied`: none of the five legacy root benches was restored — `herald_benchmarks`, `paladin_benchmarks.rs.disabled` and `arsenal_benchmarks.rs.disabled` were deprecated and removed outright by commit `10f9a4e` with no replacement anywhere in the tree; `battalion_benchmarks.rs` and `garrison_benchmarks.rs` were removed by the same commit and replaced at deliberately narrower scope by commit `d346ab9` (232 lines vs. the old 982, 88 lines vs. the old 299).
- **`REQ-critical-path-bench-scope`** was re-derived independently of `benchmark-assessment.md`'s own self-asserted "PRD Success Metrics Status" table — each of FR-16's four categories was checked directly against the shipped bench source this session rather than trusting the document's own completion claim.
- **`REQ-performance-baseline-doc`** is kept as a separate `relocated` row from `QUAL-05`: the mdbook chapter closes the "a baseline document exists" half but not the separate runtime-measurement requirement, so the two are never conflated.
- The six Phase-9-closed rows are cited, not re-opened, per D-04 — but every citation was re-run this session against `REQUIREMENTS.md:1320-1355`'s hand-off block, and none had moved.
- **`REQ-doc-coverage-audit`** flipped from the stale "Contested → HARD-07" premise to `present, unproven` citing ADR-0033's fresh finding of seven different crates still setting `doctest = false`.
- **`REQ-versioning-policy`** resolved to `relocated` per the tie-break rule even though no document was actually found at any path — the class preserves the moved-not-missing signal, with the `superseded by outcome` half stated in the same cell.
- **`REQ-release-readiness-audit`** is recorded `satisfied` as closed history, stating explicitly that the tree is four minors past `v0.1.0-rc.1` today.
- **`REQ-per-crate-readme`** and **`REQ-per-crate-changelog`** land on different verdicts despite both being ten-of-ten-complete facts: changelog has a CI-wired guard script so it is `satisfied`; readme has no equivalent enforcer so it is `present, unproven`.

## Deviations from Plan

None — plan executed exactly as written.

One environment-handling note, not a deviation: the first `cargo bench --workspace --no-run` attempt was launched as a plain foreground Bash call and was auto-backgrounded by the tool at its 590s timeout, terminating with `Terminated` and no usable output. The command was re-run via explicit `run_in_background: true` with output piped through `tee` to a scratchpad log; it completed successfully in 10m 41s on the second attempt, and that completed result (not the first, inconclusive attempt) is what the `REQ-workspace-bench-execution` row records.

## Issues Encountered

- Both `Read` and the first `Edit` attempt against `/workspace/.planning/ledgers/milestone-07-08.md` (the absolute path implied by the harness's shared-checkout context) were rejected by the tool's worktree-isolation guard, which correctly identified that path as the main checkout rather than this agent's worktree. Resolved by using the worktree-relative absolute path (`/workspace/.claude/worktrees/agent-ae4423155b23a056e/.planning/...`) for all subsequent Read/Edit calls; the two copies were confirmed byte-identical via `diff` before the switch, so no research already gathered from the shared-checkout path needed to be redone.
- To keep the two tasks' commits independently balanced (added == deleted per task, per the plan's own acceptance criteria), both epic edits were first drafted together, then `git checkout --` reverted the file and each edit was reapplied and committed one at a time, rather than committing both edits from a single combined working-tree state.

## User Setup Required

None — no external service configuration required.

## Known Stubs

None introduced by this plan.

## Next Phase Readiness

- Milestone 7 Epic 3 and Epic 4 are fully derived; plans 10-09 and 10-10 can proceed independently over their own disjoint epic ranges (no file-content dependency between fan-out plans beyond the shared row/section-count invariant, which this plan leaves unchanged at 86/12).
- `REQ-doc-coverage-audit`'s `present, unproven` verdict and ADR-0033's seven-crate doctest-flag list are now cited from the ledger as well as the ADR — Phase 15 / the coverage-and-CI quality gates can proceed against either.
- No blockers. No `.rs` file was touched; `git status --porcelain -- '*.rs'` is empty.

## Self-Check: PASSED

- FOUND: `.planning/ledgers/milestone-07-08.md`
- FOUND: commit `c4cc768` (Task 1)
- FOUND: commit `ad5820c` (Task 2)

---
*Phase: 10-milestone-7-8-ground-truth-recorded-account*
*Completed: 2026-08-08*
