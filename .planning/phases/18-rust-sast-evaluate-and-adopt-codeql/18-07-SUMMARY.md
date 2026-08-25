---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 07
subsystem: security
tags: [codeql, sast, rust, security-documentation, ci]

# Dependency graph
requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql (plans 01-06)
    provides: "The CodeQL probe evaluation itself — 18-CODEQL-EVIDENCE.md's Verdict, Promotion Status and Open Item sections, settled by the user at four checkpoints on 2026-08-25"
provides:
  - "A rewritten Rust-SAST section in .github/instructions/security.instructions.md stating the measured CodeQL outcome (disqualified, version-scoped, advisory-retained) instead of the prior 'open work' framing"
  - "CLAUDE.md and .github/copilot-instructions.md brought into agreement with the security instructions on the same claim"
  - "MILESTONES.md's v0.8.0 'No Rust SAST' Known Gap entry updated in place to the settled verdict"
  - "codeql.yml and its branching-model.md register row rewritten from 'not yet promoted' to 'measured and deliberately retained advisory' — with the file itself kept, per user override of the plan's disqualified-branch removal instruction"
  - "18-CODEQL-EVIDENCE.md's Verdict section closed with a pointer to the rewritten security section and an explicit record of the Task-3 deviation"
affects: [phase-19-crates-io-trusted-publishing, phase-20-release-pipeline-recovery, any-future-codeql-rust-queries-upgrade-evaluation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Verdict-closing pattern: an evaluation's raw evidence document stays the source of record; downstream instruction/state files carry only its conclusions, each citing the evidence doc by path so a claim can always be checked against a number"

key-files:
  created: []
  modified:
    - .github/instructions/security.instructions.md
    - CLAUDE.md
    - .github/copilot-instructions.md
    - .planning/MILESTONES.md
    - .github/workflows/codeql.yml
    - docs/src/contributing/branching-model.md
    - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md

key-decisions:
  - "STATE.md was NOT updated by this executor, deviating from the plan's Task 2 file list and the plan's own files_modified frontmatter — the orchestrator's spawn instructions for this worktree-mode run explicitly reserved STATE.md/ROADMAP.md for itself ('execute-plan auto-skips STATE.md in worktree mode')."
  - "codeql.yml was NOT removed on the disqualified verdict, deviating from Task 3's literal disqualified-branch instruction — the user explicitly chose advisory-retention over removal at the checkpoint that settled the verdict, and 18-CODEQL-EVIDENCE.md's own Verdict/Promotion Status sections already recorded that choice before this plan ran. Task 3 executed the disposition the evidence actually records, not the plan text's binary branch."

requirements-completed: [SAST-04]

coverage:
  - id: D1
    description: "Rust-SAST section in security.instructions.md rewritten to the measured CodeQL outcome, Snyk section left untouched, manual credential-handling checks preserved in order"
    requirement: "SAST-04"
    verification:
      - kind: other
        ref: "python3 heading-count/number-overlap/credential-check assertion (see Task 1 commit body) — exit 0"
        status: pass
    human_judgment: false
  - id: D2
    description: "CLAUDE.md, copilot-instructions.md and MILESTONES.md propagated to agree with the rewritten security section (gating claim consistent, Snyk prohibition intact, MILESTONES.md points at the evidence doc)"
    requirement: "SAST-04"
    verification:
      - kind: other
        ref: "python3 gating-consistency + Snyk-prohibition + evidence-pointer assertion (see Task 2 commit body) — exit 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "codeql.yml retained and its header/register row rewritten to state the measured disqualification plainly instead of a pending-promotion framing"
    requirement: "SAST-04"
    verification:
      - kind: other
        ref: "bash scripts/check-workflow-triggers.sh && make check-gates — both exit 0; exactly one codeql.yml register row"
        status: pass
    human_judgment: false

duration: ~35min
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 07: Close the Phase — Rust-SAST Record Rewritten to the Measured CodeQL Verdict Summary

**Rewrote every record of the Rust-SAST gap (security.instructions.md, CLAUDE.md, copilot-instructions.md, MILESTONES.md, codeql.yml, branching-model.md) to say CodeQL was measured and disqualified as a required-check-grade Rust SAST at CodeQL 2.26.3, and that the resulting `codeql.yml` scan is deliberately retained advisory-only rather than removed or pending promotion.**

## Performance

- **Duration:** ~35 min
- **Tasks:** 3
- **Files modified:** 7 (`security.instructions.md`, `CLAUDE.md`, `copilot-instructions.md`, `MILESTONES.md`, `codeql.yml`, `branching-model.md`, `18-CODEQL-EVIDENCE.md`)

## Accomplishments

- `.github/instructions/security.instructions.md`'s "Known gap: no Rust SAST" section rewritten to a dated, evidence-cited verdict (CodeQL `2.26.3` / `rust-queries` `0.1.40`, evaluated 2026-08-25): 3 of 4 rule-aligned classes never fired across four independent measurements, `385/385` file coverage held on every run, the one working class (hardcoded-credential) carries a 100% FP rate on its one real-code sample. The Snyk verdict section is untouched — verified byte-for-byte via `git diff` inspection before commit.
- `CLAUDE.md` and `.github/copilot-instructions.md`'s Security sections brought into agreement with the rewritten instructions — same claim, same "does not gate a merge" framing, standing Snyk prohibition intact in both.
- `.planning/MILESTONES.md`'s v0.8.0 "No Rust SAST" Known Gap entry updated in place (not deleted) to the settled verdict, with the owner/revisit date carried from the evidence document's Open Item.
- `.github/workflows/codeql.yml`'s header comment rewritten from "not yet pinned, promotion happens later" to "measured and disqualified, retained deliberately" — and, per the user's checkpoint decision, the workflow file itself is **kept**, not removed.
- `docs/src/contributing/branching-model.md`'s `codeql.yml` trigger-policy row updated to match; still exactly one row.
- `18-CODEQL-EVIDENCE.md`'s `## Verdict` section closed with a pointer back to the rewritten security section, plus an explicit Task-3 disposition note explaining why neither of the plan's two anticipated branches (remove-on-disqualified / no-op-on-qualified) matched what actually happened.

## Task Commits

1. **Task 1: Rewrite the Rust-SAST section to match the measured outcome** - `73defe93` (docs)
2. **Task 2: Propagate the settled verdict to every place that asserts the gap** - `c977151b` (docs)
3. **Task 3: Settle the workflow's own fate to match the verdict** - `cc748d97` (docs)

_No plan-metadata commit — this SUMMARY.md and its own state-file conventions follow immediately below; `.planning/STATE.md`/`.planning/ROADMAP.md` are intentionally excluded from any commit in this plan per the worktree-mode spawn instructions._

## Files Created/Modified

- `.github/instructions/security.instructions.md` - Rust-SAST section rewritten to the measured verdict; Snyk section and manual-review checklist preserved
- `CLAUDE.md` - Security working-agreement bullet updated to name the CodeQL disposition
- `.github/copilot-instructions.md` - Security section updated to match CLAUDE.md
- `.planning/MILESTONES.md` - v0.8.0 "No Rust SAST" Known Gap entry updated in place to the settled verdict
- `.github/workflows/codeql.yml` - header comment rewritten from pending-promotion framing to measured-and-retained framing; workflow itself unchanged/kept
- `docs/src/contributing/branching-model.md` - `codeql.yml` trigger-policy row updated to match
- `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` - `## Verdict` closed with a pointer to the rewritten section and a Task-3 disposition note

## Decisions Made

- **STATE.md excluded from this plan's scope.** The plan's own frontmatter lists `.planning/STATE.md` under `files_modified` and Task 2's text instructs updating it, but this executor's spawn-time objective explicitly overrode that: "Do NOT update STATE.md or ROADMAP.md — the orchestrator owns those (execute-plan auto-skips STATE.md in worktree mode; MILESTONES.md you MAY edit)." An initial edit to STATE.md's known-limitations row was made, then reverted (`git checkout -- .planning/STATE.md`) once this was caught, before any commit. STATE.md is unchanged by this plan's execution; the orchestrator is expected to apply the equivalent update.
- **codeql.yml retained rather than removed, per user override of the plan's literal disqualified-branch text.** Task 3's `<action>` describes only two outcomes keyed on the verdict: remove the workflow on `disqualified`, leave it untouched on `qualified`/`qualified-with-coverage-gap`. The actual verdict is `disqualified`, but `18-CODEQL-EVIDENCE.md`'s own `## Verdict` and `## Promotion Status` sections — settled by the user at the checkpoint that closed the evaluation, before this plan ran — already recorded a third disposition: retain the workflow, advisory-only. This plan's Task 3 executed that recorded disposition (rewriting the workflow's comments and the register row to state the measured disqualification plainly) rather than the plan text's binary branch, and added an explicit note to the evidence document explaining the mismatch.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 4 — architectural/scope, user-directed] STATE.md excluded from Task 2's scope**
- **Found during:** Task 2 (Propagate the settled verdict)
- **Issue:** Plan's Task 2 instructs rewriting `.planning/STATE.md`'s known-limitations row; the executor's own spawn instructions (not the plan) explicitly forbid touching STATE.md/ROADMAP.md in this worktree-mode run.
- **Resolution:** Followed the spawn instructions (higher precedence than the plan text for this specific constraint). Made and then reverted a STATE.md edit before any commit, once the conflict was noticed. STATE.md remains at its pre-plan state; the orchestrator is responsible for applying the equivalent update after merge.
- **Files affected:** `.planning/STATE.md` (reverted, no net change)
- **Committed in:** N/A — no commit touches STATE.md

**2. [Rule 4 — architectural, user-directed at an earlier checkpoint] codeql.yml retained instead of removed**
- **Found during:** Task 3 (Settle the workflow's own fate)
- **Issue:** Plan's Task 3 `<action>` names only two branches (remove on disqualified, no-op on qualified); the actual settled disposition — retain the workflow advisory-only despite disqualification — is a third option the plan text didn't anticipate, already decided by the user and recorded in `18-CODEQL-EVIDENCE.md` before this plan executed.
- **Resolution:** Executed the disposition the evidence document already records: kept `.github/workflows/codeql.yml`, `.github/codeql/codeql-config.yml` and `.github/codeql/codeql-config-probe.yml`; rewrote the workflow's header comment and the `branching-model.md` register row to state the measured disqualification plainly (not a pending promotion); added an explicit disposition note to `18-CODEQL-EVIDENCE.md` explaining the mismatch with the plan's literal branches.
- **Files affected:** `.github/workflows/codeql.yml`, `docs/src/contributing/branching-model.md`, `18-CODEQL-EVIDENCE.md`
- **Verification:** `bash scripts/check-workflow-triggers.sh` and `make check-gates` both exit 0; `fixtures/codeql-probe/Cargo.toml` and `scripts/codeql-analysed-files.sh` both still present; exactly one `codeql.yml` register row.
- **Committed in:** `cc748d97`

---

**Total deviations:** 2, both directed by explicit user/orchestrator instruction rather than executor judgment. No scope creep — one is a scope *reduction* (STATE.md excluded), the other is executing a disposition the evidence document already recorded rather than the plan text's stale binary framing.
**Impact on plan:** Neither deviation weakens the honest-record bar this plan exists to enforce. Both are documented here and in `18-CODEQL-EVIDENCE.md` so a future reader sees the full reasoning rather than an unexplained divergence from the plan text.

## Issues Encountered

Full-workspace `pre-commit run` (cargo fmt + `cargo clippy --workspace --all-targets --all-features`) takes ~4.5 minutes on first invocation in this worktree due to a cold `target/` cache. Ran it once in the background against the Task 1 files (exit 0, all hooks including gitleaks/shellcheck/yaml/toml passed) to confirm the toolchain is clean, then committed subsequent tasks with `--no-verify` per this run's `worktree_skip_hooks=true` environment note rather than re-paying the multi-minute clippy cost for markdown-only changes on every commit. `bash scripts/check-workflow-triggers.sh` and `make check-gates` (which includes the CodeQL-dismissal-register, advisory-register and workflow-trigger guards) were run explicitly for Task 3 and both exited 0.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 18 (`rust-sast-evaluate-and-adopt-codeql`) is closed: SAST-01 through SAST-04 all have their evidence and documentation in final form. `18-CODEQL-EVIDENCE.md` is the durable record for any future CodeQL/`rust-queries` upgrade evaluation — the fixture (`fixtures/codeql-probe/`) and the analysed-file-count script are both retained for exactly that reuse. No blockers for Phase 19 (crates.io Trusted Publishing) or Phase 20 (Release Pipeline Recovery); neither depends on this phase's SAST tooling decision.

The orchestrator still needs to apply the STATE.md update this plan intentionally skipped (known-limitations row, current-position block, phase-completion bookkeeping) since worktree-mode execution reserves that file for it.

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*

## Self-Check: PASSED

All 7 modified/created files confirmed present on disk (`ls -la` against each path). All 3 task
commits (`73defe93`, `c977151b`, `cc748d97`) confirmed present in `git log --oneline -5`. No
missing items.
