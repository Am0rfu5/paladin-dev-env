---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 01
subsystem: infra
tags: [codeql, github-actions, sast, rust, ci, code-scanning]

requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql
    provides: SAST-01..04 requirements, D-01..D-22 locked decisions, RESEARCH.md CodeQL mechanics
provides:
  - .github/workflows/codeql.yml — advanced-setup CodeQL Rust analysis workflow (advisory, build-mode none)
  - docs/src/contributing/branching-model.md trigger-policy row for codeql.yml
  - scripts/codeql-analysed-files.sh — D-13 analysed-file-count evidence extractor
  - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md — evidence log with dated promotion criteria and the first real run
  - A proven, real, end-to-end CodeQL Rust run against this repository (run 32868842656, success)
  - Empirical answer to D-12's cargo-feature-coverage open question (buildless extraction indexes by file extension, not cargo features — full coverage confirmed)
  - Correction to RESEARCH.md's unverified Assumption A5 (src.zip's true nested location inside db-<language>.zip)
affects: [18-02, 18-03, 18-04, 18-05, 18-06, 18-07]

tech-stack:
  added: [github/codeql-action@v3, dtolnay/rust-toolchain (already in use), Swatinem/rust-cache (already in use)]
  patterns:
    - "Advisory-first CI job: context deliberately not pinned in any ruleset, no error-suppressing step flag anywhere — visible failure, not masked green"
    - "debug:true → nested src.zip inside db-<language>.zip as first-class file-analysis evidence, parsed by a dedicated offline guard-style script"

key-files:
  created:
    - .github/workflows/codeql.yml
    - scripts/codeql-analysed-files.sh
    - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md
  modified:
    - docs/src/contributing/branching-model.md

key-decisions:
  - "Fine-grained PAT lacked git-push (Contents write) permission despite reporting collaborator push:true — this is a token-scope gap, not a collaborator-role gap; documented as an authentication gate, resolved by the user issuing a full-access temporary token"
  - "analysed_rs_files in codeql-analysed-files.sh is scoped to exactly the two globs (crates/**/*.rs, root src/**/*.rs) that define the 385 denominator, not src.zip's raw total entry count, because src.zip also archives the full Rust stdlib source and a few CodeQL-bundled builtins — reporting the raw total as 'analysed' would itself be confusing evidence"
  - "Promotion Criteria numeric thresholds (FP rate ≤20%, wall-clock ≤600s, analysed-file floor ≥95% of 385) set now in 18-CODEQL-EVIDENCE.md, dated before the D-11 probe or D-14/D-15 window produce any number, per D-18's 'trigger condition written down, never retrofitted' requirement"
  - "Tracer branch (codeql-tracer-18-01) deleted from origin immediately after its run's evidence was captured (run conclusion, wall-clock, SARIF, analysed-file count, alert data) — per D-09's no-standing-scan-surface posture; code-scanning alert #28 remains queryable by commit SHA after branch deletion"

patterns-established:
  - "D-06 advisory posture: a scanner job that can genuinely fail (no continue-on-error anywhere) while remaining non-blocking purely because its context isn't in any ruleset yet — the first positive local example of this pattern (osv-scanner's continue-on-error stacking was the anti-pattern to avoid)"
  - "Guard-script house style extended to a one-shot evidence-extraction script: offline apart from a single gh run download call, mktemp -d cleanup on exit, named non-zero failure for every missing-input case, no default substituted for a missing artifact"

requirements-completed: []

coverage:
  - id: D1
    description: "codeql.yml wired end-to-end: workflow + trigger-policy register row land in one commit, structural acceptance criteria (job name literal, build-mode none, no PR path filter, no continue-on-error, job-level permissions, single register row) verified, and a real push-triggered run completes with conclusion success"
    requirement: "SAST-02"
    verification:
      - kind: automated_ui
        ref: "bash scripts/check-workflow-triggers.sh"
        status: pass
      - kind: e2e
        ref: "gh run watch 32868842656 --exit-status (CodeQL Analysis (Rust), conclusion success, 212s)"
        status: pass
    human_judgment: false
  - id: D2
    description: "scripts/codeql-analysed-files.sh extracts the analysed-.rs-file count from a real run's debug artifact, scoped to the 385 denominator, with named failures for a missing run id, nonexistent run, or absent artifact/src.zip"
    requirement: "SAST-03"
    verification:
      - kind: unit
        ref: "shellcheck --severity=warning scripts/codeql-analysed-files.sh"
        status: pass
      - kind: integration
        ref: "bash scripts/codeql-analysed-files.sh 32868842656 → analysed_rs_files=385, denominator=385, difference=0"
        status: pass
    human_judgment: false
  - id: D3
    description: "18-CODEQL-EVIDENCE.md exists with all six required sections, promotion criteria dated before any probe/window measurement, and the tracer run's real conclusion/wall-clock/analysed-file-count recorded"
    requirement: "SAST-03"
    verification:
      - kind: manual_procedural
        ref: "grep-based schema check against 18-CODEQL-EVIDENCE.md (all six headings, 385, 3x feature_gated_present, verdict text)"
        status: pass
    human_judgment: false

duration: ~40min active work (spread across a session interrupted by a human-action checkpoint for a token-permission fix; total elapsed wall-clock across both sessions ~1h33m)
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 01: Wire CodeQL Rust Analysis — Tracer Slice Summary

**CodeQL Rust static analysis wired end-to-end and proven against this repository's real tree: a live `CodeQL Analysis (Rust)` run completed successfully, analysed exactly 385 of the 385 first-party `.rs` files (crates/ + root src/), reached all three tested non-default-feature-gated paths, and surfaced one genuine first-party finding (`rust/hard-coded-cryptographic-value` in `user_service.rs`) on its very first execution.**

## Performance

- **Duration:** ~40min active work; ~1h33m total elapsed (paused mid-plan for a `checkpoint:human-action` — the environment's `GH_TOKEN` had read-only repository scope and could not `git push`, despite `gh api` reads succeeding; the user issued a temporary full-access fine-grained PAT and the plan resumed from the same commit)
- **Started:** 2026-08-25T14:42:01Z (Task 1 commit)
- **Completed:** 2026-08-25T16:15:06Z (Task 3 commit)
- **Tasks:** 3/3
- **Files modified:** 4 (2 created workflow/register-row in one commit, 1 script, 1 evidence doc)

## Accomplishments

- `.github/workflows/codeql.yml` — advanced-setup CodeQL Rust analysis (`build-mode: none`, `security-extended` queries, `debug: true`), triggers on `push:['**']` + `pull_request` (no path filter) + weekly schedule + `workflow_dispatch`, job-level-only permissions, zero error-suppressing step flags anywhere — visibly-advisory, not silently-masked.
- Trigger-policy register row for `codeql.yml` landed in the same commit as the workflow (D-05); `scripts/check-workflow-triggers.sh` passes with all 7 workflow files and their rows.
- A real, live run (`32868842656`, pushed to a disposable `codeql-tracer-18-01` branch, watched to completion, then the branch deleted) completed with `conclusion: success` in 212s (cold cache), and its SARIF is confirmed readable in the code-scanning store.
- `scripts/codeql-analysed-files.sh` — downloads a run's `debug-artifacts` artifact, correctly locates the nested `src.zip` inside `db-<language>.zip` (not at the artifact's top level, correcting RESEARCH.md's unverified Assumption A5), and reports the analysed-file count scoped to the 385 denominator, plus raw-total/toolchain/vendored breakdowns so neither number is mistaken for the other.
- **D-12's open question is answered empirically, not assumed**: all three feature-gated probe paths (`web-server`-gated, `cli`-gated, and `paladin-web`'s own root) were present in the analysed set, and the run's own log shows the mechanism — CodeQL's Rust extractor indexes every `.rs` file in the checkout by extension (`codeql database index-files --include-extension=.rs`), never invoking `cargo build`/`cargo check` to resolve a feature set. Feature gating does not narrow buildless-Rust-extraction coverage.
- `18-CODEQL-EVIDENCE.md` seeded with dated Promotion Criteria (FP-rate ≤20%, wall-clock ≤600s, analysed-file floor ≥95% of 385, both D-11/D-13 disqualifying conditions named), the Run Log's first row, Analysis Coverage detail, and the two placeholder sections (`Verdict`, `Promotion Status`) that later plans replace.

## Task Commits

Each task was committed atomically:

1. **Task 1: Wire CodeQL end-to-end — workflow plus its trigger-policy register row, one commit** - `4b74cae7` (feat)
2. **Task 2: Extract the analysed-`.rs`-file count from a run's own debug artifact** - `0e05957b` (feat)
3. **Task 3: Seed the evidence document — schema, promotion criteria, first measured run** - `40d29f26` (docs)

_No TDD tasks in this plan — Task 1 is `type="tracer"` (real implementation + real end-to-end proof, not a throwaway slice); Tasks 2-3 are `type="auto"`._

## Files Created/Modified

- `.github/workflows/codeql.yml` - Advanced-setup CodeQL Rust analysis workflow, advisory posture
- `docs/src/contributing/branching-model.md` - New trigger-policy register row for `codeql.yml`
- `scripts/codeql-analysed-files.sh` - D-13 analysed-file-count evidence extractor (executable, offline apart from one `gh run download` call)
- `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` - Phase evidence log: method, dated promotion criteria, run log, analysis coverage, verdict, promotion status

## Decisions Made

- **Fine-grained PAT scope gap treated as an authentication gate, not a bug to work around.** The environment's `GH_TOKEN` reported `push: true` via the repository-collaborator-permissions API (a role-level fact) but was actually scoped to `metadata: read` only at the token-grant level (`X-Accepted-Github-Permissions: metadata=read` on a live response header) — `git push` failed with a genuine 403 from GitHub, not a local credential-helper misconfiguration (verified by pinning `gh`'s own credential helper explicitly and still failing identically). Per `<authentication_gates>`, this stopped the plan with a `checkpoint:human-action` rather than attempting a workaround; the user replaced the token with a temporary full-access one and the plan resumed from the exact commit where it paused.
- **`analysed_rs_files` is denominator-scoped, not raw-total.** `src.zip` (nested inside `db-<language>.zip`, not top-level as RESEARCH.md's Pattern 2 sketched) archives 3,434 `.rs` entries on this run, of which 2,874 are the Rust toolchain's own vendored standard-library source and only 557 are this checkout's own files. Reporting the raw 3,434 as "analysed_rs_files" against a 385 denominator would itself be exactly the kind of confusing, uncomparable evidence D-13 exists to prevent — the script reports the denominator-scoped count (385, exact match) as the headline field and the raw/toolchain/vendored breakdown as separate, clearly labelled diagnostic fields.
- **Promotion Criteria numbers set now, not deferred.** No PLAN.md decision specified exact FP-rate/wall-clock/coverage-floor numbers (Claude's Discretion per CONTEXT.md); chose FP rate ≤20%, wall-clock ≤600s, analysed-file floor ≥95% of 385 — conservative, defensible bars set and dated before the D-11 probe or D-14/D-15 backfill window produce a single real number, satisfying D-18's "never retrofitted" requirement.
- **Tracer branch deleted immediately after evidence capture**, per D-09's "no standing scan surface" posture — the code-scanning analysis and the one alert it raised remain queryable by commit SHA (`4b74cae7`) even after the branch is gone.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected `src.zip` location assumption inherited from RESEARCH.md**
- **Found during:** Task 2 (writing `scripts/codeql-analysed-files.sh`)
- **Issue:** RESEARCH.md's Pattern 2 (and its own Assumptions Log A5) sketched `src.zip` as living at the debug artifact's top level. A real run's artifact was inspected directly: the top level instead contains `db-rust.zip` (the full CodeQL database archive), and `src.zip` lives nested inside that, at `db-rust/src.zip`. A script written against the sketched (wrong) location would have found nothing and, absent careful handling, could have silently reported 0 analysed files — a false negative in exactly the mechanism this evidence chain exists to protect.
- **Fix:** `scripts/codeql-analysed-files.sh` locates `db-<language>.zip` at the artifact's top level, opens it, and reads the nested `src.zip` entry from inside it, with named failures at every step if the expected shape isn't found.
- **Files modified:** `scripts/codeql-analysed-files.sh`
- **Verification:** Ran successfully against the real run (`32868842656`), producing `analysed_rs_files=385` — matches the independently-computed 385 denominator exactly.
- **Committed in:** `0e05957b` (Task 2 commit)

**2. [Rule 2 - Missing Critical] Scoped `analysed_rs_files` to the denominator's own globs rather than reporting `src.zip`'s raw entry count**
- **Found during:** Task 2, after inspecting the real `src.zip`'s contents
- **Issue:** The plan's literal task text defines `analysed_rs_files` as "count of entries ending in `.rs`" without further qualification. Applied literally against the real archive, that count is 3,434 — dominated by 2,874 vendored Rust-toolchain standard-library files that have nothing to do with this repository's own 385-file denominator. Reporting that raw number as "analysed_rs_files" next to `denominator=385` would produce a nonsensical `difference` (385 − 3,434 = −3,049) and would itself be the kind of misleading evidence this whole phase exists to prevent.
- **Fix:** `analysed_rs_files` is scoped to entries whose checkout-relative path starts with `crates/` or root `src/` — the exact two globs D-13 itself uses to define the 385 denominator — making `difference` a directly meaningful number. The raw archive totals (total `.rs` entries, toolchain-stdlib entries, other-vendored entries, and the broader checkout-wide `.rs` count including tests/examples/benches) are reported as separate, explicitly-labelled fields so no reader mistakes one number for the other.
- **Files modified:** `scripts/codeql-analysed-files.sh`, `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`
- **Verification:** `difference=0` on the real run — the denominator-scoped count matches exactly.
- **Committed in:** `0e05957b` (Task 2), `40d29f26` (Task 3, documented in Analysis Coverage)

---

**Total deviations:** 2 auto-fixed (1 bug correction, 1 missing-critical-clarity addition)
**Impact on plan:** Both corrections were necessary for the evidence to mean what D-13 requires it to mean; neither changed the plan's scope or deliverables. No scope creep.

## Issues Encountered

- **Authentication gate: `git push` denied despite passing `gh api` reads (the task's own stated precondition).** Root-caused to a fine-grained PAT scoped to `metadata: read` only, not the `Contents: Read and write` permission `git push` requires — a token-grant-level fact invisible to the repository-collaborator-permissions API, which reported `push: true` based on the account's role rather than the token's actual grant. Documented in full in a `checkpoint:human-action`, resolved by the user issuing a temporary full-access token; the plan resumed from the exact commit (`4b74cae7`) where it paused, redoing no committed work.
- **`gh run download` requires being invoked from inside a recognized git checkout** (or an explicit `--repo` flag) — failed with "fatal: not a git repository" when first tested from a bare `/tmp` inspection directory. Not a plan blocker; `scripts/codeql-analysed-files.sh` derives the repo slug from the checkout's own `origin` remote (falling back to a hardcoded slug) so this doesn't recur for any future caller.

## User Setup Required

None - no external service configuration required beyond the temporary token the user already provided to resolve the authentication gate above (a permanent, correctly-scoped `GH_TOKEN` is expected to already be the user's ongoing responsibility outside this plan's scope).

## Next Phase Readiness

- `codeql.yml` and its register row are live and proven against a real run — plan 18-02 (the probe fixture, D-07..D-11) can build directly on this wiring, including the `scan_probe_fixture` dispatch input it plans to add.
- `scripts/codeql-analysed-files.sh` is ready for reuse in every subsequent run this phase records (the D-14/D-15 backfill window, the probe run itself) — no further mechanism work needed for the D-13 evidence chain.
- `18-CODEQL-EVIDENCE.md`'s `## Verdict` and `## Promotion Status` sections are explicit placeholders (`pending — probe not yet run`, `advisory — context not pinned in any ruleset`) for plan 18-03 and plan 18-06 respectively to replace — do not treat their current text as a real verdict.
- **One genuine first-party finding is now open in the code-scanning alert store** (`rust/hard-coded-cryptographic-value`, critical, `src/core/platform/manager/user_service.rs`, alert #28) — deliberately not remediated here per this phase's explicit out-of-scope boundary on fixing real defects the scanner finds. Flagged here so a later triage pass (this phase's D-17 alert-triage register, or separate remediation work) does not lose track of it.

## Threat Flags

None found — this plan's new surface (a CI workflow, a read-only evidence-extraction script, a documentation register row and an evidence log) matches the phase's own `<threat_model>` register (T-18-01 through T-18-06, T-18-SC) exactly; no new trust boundary or security-relevant surface outside what was already threat-modelled in PLAN.md was introduced.

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*
