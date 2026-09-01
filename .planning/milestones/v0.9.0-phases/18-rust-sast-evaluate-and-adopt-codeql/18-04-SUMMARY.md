---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 04
subsystem: infra
tags: [codeql, sast, governance, ci, shellcheck, offline-guard, dismissal-register]

# Dependency graph
requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql (waves 1-3, plans 18-01..18-03)
    provides: "codeql.yml wired advisory-only, the SAST-01 disqualified verdict, and alert #28's triage as a test-code false positive"
provides:
  - "CODEQL-DISMISSALS.md: governed register for dismissed CodeQL alerts, modelled on SECURITY-EXCEPTIONS.md"
  - "scripts/check-codeql-dismissals.sh: offline guard for schema, drift, staleness, uniqueness and reachability"
  - "tests/scripts/check-codeql-dismissals_test.sh: fail-first regression harness"
  - "check-codeql-dismissals folded into make check-gates and test-shell-guards"
  - "Check CodeQL dismissal register step in ci.yml's License & Dependency Policy job"
affects: [18-06, 18-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Register-plus-offline-guard governance topology (ADR-0036 / SECURITY-EXCEPTIONS.md shape), reapplied to CodeQL alert dismissals"

key-files:
  created:
    - CODEQL-DISMISSALS.md
    - scripts/check-codeql-dismissals.sh
    - tests/scripts/check-codeql-dismissals_test.sh
  modified:
    - Makefile
    - .github/workflows/ci.yml

key-decisions:
  - "Proceeded with this plan despite Task 1's literal precondition text (verdict must read qualified/qualified-with-coverage-gap) because the orchestrator's environment_notes explicitly identified this as the settled disqualified/advisory-only branch and directed the register+guard to be built anyway, since SAST-03's governance value is independent of SAST-01's promotion outcome — see Deviations below."
  - "Used the platform's own 'used in tests' dismissed_reason value for alert #28 rather than 'false positive', since the alert fires on literal test-fixture data inside a #[cfg(test)] block, which is precisely what that category names."

requirements-completed: [SAST-03]

coverage:
  - id: D1
    description: "Governed CodeQL alert-dismissal register (CODEQL-DISMISSALS.md) with an anti-drift declared count, an eleven-field schema per entry, and a documented gh api reconciliation command"
    requirement: "SAST-03"
    verification:
      - kind: other
        ref: "python3 register-schema verification script from PLAN.md Task 1 <verify> (register schema OK: 1 governed dismissal(s))"
        status: pass
      - kind: other
        ref: "pre-commit run --files CODEQL-DISMISSALS.md (all hooks Passed, including gitleaks and check-yaml)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Offline guard (scripts/check-codeql-dismissals.sh) enforcing schema, drift, staleness, uniqueness and reachability clauses, proven fail-first by a committed regression harness, and wired into make check-gates, make test-shell-guards and ci.yml's License & Dependency Policy job"
    requirement: "SAST-03"
    verification:
      - kind: other
        ref: "bash tests/scripts/check-codeql-dismissals_test.sh (11/11 assertions passed, 6 distinct rejection cases proven)"
        status: pass
      - kind: other
        ref: "make check-codeql-dismissals && make check-gates && make test-shell-guards (all exit 0)"
        status: pass
      - kind: other
        ref: "shellcheck --severity=warning scripts/check-codeql-dismissals.sh tests/scripts/check-codeql-dismissals_test.sh (exit 0)"
        status: pass
      - kind: other
        ref: "python3 yaml-load assertion confirming 'Check CodeQL dismissal register' step exists in the 'License & Dependency Policy' job"
        status: pass
      - kind: other
        ref: "bash scripts/check-workflow-triggers.sh (7 workflow files, 7 policy-table rows; unchanged trigger surface)"
        status: pass
    human_judgment: false

duration: 35min
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 04: Governed CodeQL Dismissal Register Summary

**CODEQL-DISMISSALS.md plus an offline schema/drift/staleness/uniqueness/reachability guard, seeded with alert #28's test-code-false-positive disposition, wired into `check-gates` and CI's License & Dependency Policy job.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-25T21:52:00Z (approx)
- **Completed:** 2026-08-25T22:27:52Z
- **Tasks:** 2
- **Files modified:** 5 (3 created, 2 modified)

## Accomplishments
- `CODEQL-DISMISSALS.md` created at the repository root, modelled directly on `SECURITY-EXCEPTIONS.md` (D-17): a prose header stating why the register exists, an anti-drift `Declared dismissals: N` line, a "Schema contract, stated plainly" paragraph, a full `gh api` reconciliation command named as a human step, and one machine-readable `[[dismissal]]` entry (alert #28) carrying all eleven required fields.
- `scripts/check-codeql-dismissals.sh` created: an offline, read-only guard matching the house style of `check-advisory-register.sh`/`check-workflow-triggers.sh` — accepts two optional positional overrides (register path, repo root), accumulates every violation before deciding a verdict, and its own header comment states plainly that it validates internal consistency and staleness only, never GitHub's live dismissed-alert set.
- `tests/scripts/check-codeql-dismissals_test.sh` created: a fail-first regression harness proving 6 distinct rejection cases (missing field, past review_date, declared-count mismatch, duplicate alert_number, missing register file, unreachable path) plus 2 passing cases (well-formed populated, well-formed empty) — 11/11 assertions pass.
- Wired into `Makefile`'s `check-codeql-dismissals` target, folded into `check-gates` and `test-shell-guards`; and into `.github/workflows/ci.yml`'s `License & Dependency Policy` job as the `Check CodeQL dismissal register` step, placed before the existing shell-guard regression step, mirroring the existing guards exactly.
- `scripts/check-workflow-triggers.sh` reconfirmed passing after the `ci.yml` edit — adding a step changed no trigger surface, so the trigger-policy table required no update.

## Task Commits

Each task was committed atomically:

1. **Task 1: Author the governed dismissal register** - `a4f1c01c` (feat)
2. **Task 2: Add the offline guard, its fail-first regression harness, and wire both into the required check** - `051cb8be` (feat)

_Note: both commits used `--no-verify` per this worktree's `worktree_skip_hooks=true` — `pre-commit run --files <changed files>` was run separately (in the foreground/background, not as the commit hook) and confirmed exit 0, including `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `shellcheck`, `check-yaml` and gitleaks, before and again after both commits._

## Files Created/Modified
- `CODEQL-DISMISSALS.md` - Governed register: prose header, anti-drift declared count, `gh api` reconciliation command, one dismissal entry for alert #28
- `scripts/check-codeql-dismissals.sh` - Offline guard: schema, drift, staleness, uniqueness, reachability clauses
- `tests/scripts/check-codeql-dismissals_test.sh` - Fail-first regression harness for the guard
- `Makefile` - New `check-codeql-dismissals` target, folded into `check-gates` and `test-shell-guards`
- `.github/workflows/ci.yml` - New `Check CodeQL dismissal register` step in the `License & Dependency Policy` job

## Decisions Made
- Seeded the register with exactly one entry (alert #28, `rust/hard-coded-cryptographic-value`, `src/core/platform/manager/user_service.rs:1582`) per the environment context's identification of it as the one real steady-state alert needing disposition, and per `18-CODEQL-EVIDENCE.md`'s triage of it as a test-code false positive. No probe-fixture findings exist to register (the fixture is excluded from steady-state scans, confirmed in `18-CODEQL-EVIDENCE.md`'s `## Steady-State Exclusion`).
- Used `dismissed_reason = "used in tests"` rather than `"false positive"` for alert #28 — the platform's own vocabulary distinguishes these, and the alert fires on a literal string argument inside a `#[cfg(test)] mod tests` block, which is exactly the "used in tests" case rather than a generic scanner miscall.
- Named the TOML array `[[dismissal]]` (register-specific) rather than reusing `SECURITY-EXCEPTIONS.md`'s `[[exception]]` name, since the two registers govern different kinds of thing (advisory suppressions vs. alert dismissals) and the guard scripts already key off different top-level array names by design (`check-advisory-register.sh` reads `exception`; this guard reads `dismissal`).
- Added a bonus reachability-clause test case (`unreachable-path.md`) beyond the plan's five named rejection cases, since the guard's fifth clause (reachability) had no explicit fail-first proof requirement in the plan text but the guard implements it and the harness should not leave it unproven.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 4 — architectural/scope, pre-resolved by orchestrator context] Task 1's precondition text assumed a `qualified`/`qualified-with-coverage-gap` verdict; the actual recorded verdict is `disqualified`**
- **Found during:** Task 1, precondition check
- **Issue:** Task 1's `<precondition>` reads: "`18-CODEQL-EVIDENCE.md`'s `## Verdict` section records `qualified` or `qualified-with-coverage-gap` — a disqualified verdict means no adoption work proceeds and this plan must not run." Reading `18-CODEQL-EVIDENCE.md` directly confirms: `**SAST-01 verdict: `disqualified`** (version-scoped: CodeQL `2.26.3` / `rust-queries` `0.1.40`). `codeql.yml` is retained, advisory-only, not promoted to a required check.` Taken literally, the precondition is unmet.
- **Resolution:** This executor's prompt included explicit "DECISIVE CONTEXT FROM WAVES 1-3" instructing that the SAST-01 verdict is settled as disqualified/advisory-only, and directing that this plan's register-plus-guard work still proceed — "the register + guard + staleness mechanism still get built (they are genuinely useful)" — with the plan's "wired into a required check" language treated as the held/advisory branch, and any such adjustment recorded as a deviation with the verdict as justification. Per the standard instruction that messages from the launching agent direct execution and constitute mid-task course corrections, this plan proceeded, and this deviation records the basis explicitly rather than silently reinterpreting the precondition.
- **Practical impact:** None on what was built — the register, guard and harness are exactly as specified. The only substantive adjustment is factual framing: `CODEQL-DISMISSALS.md`'s header states the advisory-only status plainly (an "Adoption context" paragraph) rather than implying `codeql.yml` blocks merges, and the `ci.yml`/`Makefile` wiring comments describe the guard as validating the register, not as itself proof that CodeQL gates anything.
- **Files modified:** CODEQL-DISMISSALS.md (adoption-context paragraph), scripts/check-codeql-dismissals.sh (header comment), .github/workflows/ci.yml (step comment)
- **Verification:** `18-CODEQL-EVIDENCE.md`'s `## Verdict` section, read in full, confirms the disqualified/advisory-only status and the downstream note for plan 18-06 confirming no ruleset promotion occurs — consistent with the framing adopted here.
- **Committed in:** a4f1c01c (Task 1), 051cb8be (Task 2)

---

**Total deviations:** 1 (Rule 4 — pre-resolved by orchestrator-supplied context, not an independent judgment call)
**Impact on plan:** None on scope or deliverables. The register, guard, harness and wiring are exactly what the plan specified; only the header/comment framing around CodeQL's promotion status was adjusted to match the settled verdict.

## Issues Encountered
- The regression harness's own "no mutation" assertion (`git status --porcelain -- CODEQL-DISMISSALS.md`) initially failed while `CODEQL-DISMISSALS.md` was still untracked from Task 1 — an untracked new file always shows as `??` regardless of whether the test itself mutated it. Resolved by committing Task 1 before running Task 2's full verification suite, which is also the natural task-by-task commit order this plan already specifies. Same transient failure recurred for `check-workflow-suppressions_test.sh`'s pre-existing "no mutation" assertion against `.github/workflows/ci.yml` while Task 2's edit was still uncommitted; resolved the same way by committing Task 2 before the final verification pass.
- The full `pre-commit run` invocation (via `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo fmt --all -- --check`, both `always_run: true` regardless of which files changed) takes several minutes end-to-end on this workspace; ran it in the background and polled for completion rather than blocking synchronously.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- `CODEQL-DISMISSALS.md` is live with alert #28 governed; any future dismissed CodeQL alert must get a row here before `make check-gates`/CI will accept it.
- Plan 18-06 (per its own downstream note in `18-CODEQL-EVIDENCE.md`) proceeds on its held/advisory branch: no ruleset write to `.github/rulesets/protect-main-branch.json`, no `docs/src/appendix/branch-protection.md` count update, `scripts/check-workflow-triggers.sh` Clause 3 has no new required context to resolve.
- Plan 18-07's rewritten Rust-SAST section in `security.instructions.md` can now cite this register as the governance mechanism for the one alert class CodeQL does reliably catch.

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*

## Self-Check: PASSED

- FOUND: CODEQL-DISMISSALS.md
- FOUND: scripts/check-codeql-dismissals.sh
- FOUND: tests/scripts/check-codeql-dismissals_test.sh
- FOUND: commit a4f1c01c (Task 1)
- FOUND: commit 051cb8be (Task 2)
