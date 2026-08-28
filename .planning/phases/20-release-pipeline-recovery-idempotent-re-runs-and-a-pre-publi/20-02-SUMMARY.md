---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 02
subsystem: infra
tags: [github-actions, release, cargo-metadata, gh-cli, bash, python3, shellcheck, tdd]

# Dependency graph
requires:
  - phase: 20-01
    provides: "scripts/check-release-consistency.sh clause 1 (manifest agreement), the collect-then-report failures list shape, the --sha/--ci-runs-json reserved-flag CLI surface, and tests/scripts/check-release-consistency_test.sh's fixture-lifecycle harness"
provides:
  - "scripts/check-release-consistency.sh clause 2 -- changelog-section agreement: every publishable package's own CHANGELOG.md (derived from cargo metadata's manifest_path, never a hardcoded crates/*/CHANGELOG.md glob) must carry a '## [<exact tag version>]' heading"
  - "scripts/check-release-consistency.sh clause 3 -- CI-conclusion agreement (PUBOPS-02/D-10): the tagged SHA's most recent completed ci.yml run must have concluded success, resolved via gh api with a 404-then-numeric-id fallback (Assumption A2 removed) and a documented whole-run granularity decision"
  - "MISSING_SHA / CI_LOOKUP_FAILED / CI_MISMATCH / CHANGELOG_MISMATCH status tokens, plus MISMATCH_AND_CHANGELOG / MISMATCH_AND_CI / CHANGELOG_AND_CI / MISMATCH_AND_CHANGELOG_AND_CI combinations, all accumulating into the same collect-then-report failures list clause 1 established"
  - "_crc_fetch_ci_runs helper (paginated gh api call + 404-then-workflow-id-resolution fallback, no -L/--location on any call)"
  - "write_changelog_fixture and write_ci_runs_fixture harness helpers, plus a manifest_path-aware write_metadata_fixture, reusable by any later plan's clause fixtures"
affects: [20-03, 20-04, 20-05, 20-06, 20-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Clause accumulation: each new D-08 clause appends (name, order, message) tuples into the same combined failures list clause 1 established, sorted by (package name, clause order) so multi-clause runs stay deterministic"
    - "Fixture seam parity: --ci-runs-json feeds the exact same python parsing code path a live gh api response takes (defensive multi-JSON-document merge, so gh's exact --paginate output shape never has to be assumed)"
    - "Fail-closed on infrastructure-level lookup failure: CI_LOOKUP_FAILED and MISSING_SHA are terminal, bash-level short-circuits (like MISSING_TAG/ZERO_PACKAGES), never conflated with a clause verdict"

key-files:
  created: []
  modified:
    - scripts/check-release-consistency.sh
    - tests/scripts/check-release-consistency_test.sh

key-decisions:
  - "CI-conclusion granularity: the whole-run ci.yml conclusion, not a named job subset. Re-derived from ci.yml at authoring time (not carried on trust from RESEARCH.md): benchmark-regression-signal is the only job with a job-level continue-on-error, and its if: restricts it to pull_request/workflow_dispatch, so it never runs on the push this clause reads. osv-scanner's tolerance is step-level, so that job still gates. No job is simultaneously known-flaky, release-irrelevant and able to turn the run red -- the condition that would make whole-run granularity unusably strict"
  - "Assumption A2 removed rather than carried: the script always tries the ci.yml-filename path first, and on a clean 404 (detected via gh's 'HTTP 404' stderr convention) resolves the numeric workflow id from the actions/workflows list endpoint and retries -- never assumes the filename path is universally accepted"
  - "CI_LOOKUP_FAILED and MISSING_SHA are bash-level terminal short-circuits, not python-reported clause failures -- kept them out of the python parser entirely so they can never be conflated with 'no successful run exists' (a clause 3 verdict) or with any clause 1/2 result"
  - "The changelog-section heading regex anchors '\\]' immediately after the escaped tag version (`^##\\s*\\[<version>\\](\\s|$)`), so a longer version sharing a prefix (1.2.30 vs 1.2.3) or a prerelease variant (1.2.3-rc.1 vs 1.2.3) never satisfies a stable tag -- no prefix matching anywhere in the pattern"
  - "release.yml is deliberately untouched by this plan (0 lines under .github/workflows/ modified) -- the script reads --sha from the CLI only, and the workflow-side --sha wiring is explicitly deferred to plan 20-05's release.yml pass, per the plan's own scope boundary"

patterns-established:
  - "Status-token combination table: a fixed dict from a sorted tuple of active clause labels (MISMATCH/CHANGELOG/CI) to a status string, rather than nested if/elif chains -- keeps all 7 non-empty combinations for 3 clauses explicit and auditable at a glance, and is the shape a 4th clause (SHA agreement) would extend"

requirements-completed: [PUBOPS-01, PUBOPS-02]

coverage:
  - id: D1
    description: "Changelog-section clause (D-08 clause 2): every publishable package's own CHANGELOG.md must carry a heading for the exact tag version, with distinct messages for 'file missing' vs 'section missing', accumulating into the same report clause 1 uses"
    requirement: "PUBOPS-01"
    verification:
      - kind: unit
        ref: "tests/scripts/check-release-consistency_test.sh (23 assertions after Task 1, all pass)"
        status: pass
      - kind: manual_procedural
        ref: "./scripts/check-release-consistency.sh --tag v0.8.1-rc.2 2>&1 | grep -c 'CHANGELOG.md' -> 10; names exactly the ten crate changelogs under crates/, excludes root CHANGELOG.md and paladin-doc-examples; no manifest-version mismatch reported in the same run"
        status: pass
    human_judgment: false
  - id: D2
    description: "CI-conclusion clause (D-08 clause 3 / PUBOPS-02): the tagged SHA's most recent completed ci.yml run must have concluded success, resolved deterministically via created_at+id sort, with MISSING_SHA failing closed on the CI path and CI_LOOKUP_FAILED never conflated with 'no successful run'"
    requirement: "PUBOPS-02"
    verification:
      - kind: unit
        ref: "tests/scripts/check-release-consistency_test.sh (32 assertions after Task 2, all pass, covering all 7 <behavior> cases: single success, single failure, most-recent-decides, id tie-break both orderings, empty-runs, MISSING_SHA, offline not-checked note)"
        status: pass
      - kind: manual_procedural
        ref: "GITHUB_ACTIONS=true ./scripts/check-release-consistency.sh --tag v0.8.1-rc.2 (exits non-zero, output contains MISSING_SHA); ./scripts/check-release-consistency.sh --tag v0.8.1-rc.2 with no GITHUB_ACTIONS (still runs offline clauses, output contains the 'was not checked' NOTE line); grep -nE '(-L|--location)' scripts/check-release-consistency.sh -> no match; grep -c 'actions/workflows' -> 7 (>= 2, proving the 404-fallback path exists); git status --short shows no .github/ file touched"
        status: pass
    human_judgment: false

# Metrics
duration: ~35min
completed: 2026-08-28
status: complete
---

# Phase 20 Plan 02: Changelog-Section and CI-Conclusion Clauses Summary

**Extended the pre-publish consistency gate with D-08's changelog-section clause (every publishable package's own CHANGELOG.md must carry a heading for the tag version) and D-10's CI-conclusion clause (the tagged SHA's most recent completed ci.yml run must have concluded success), both accumulating into the same collect-then-report list plan 20-01's tracer established.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-28T14:58:00Z (approx; first file reads)
- **Completed:** 2026-08-28T15:24:36Z
- **Tasks:** 2 (both `tdd="true"`)
- **Files modified:** 2 (`scripts/check-release-consistency.sh`, `tests/scripts/check-release-consistency_test.sh`)

## Accomplishments

- **Clause 2 (changelog-section, D-08):** a publishable package's changelog is `CHANGELOG.md` in `dirname(manifest_path)` -- no hardcoded `crates/*/CHANGELOG.md` glob, so the root package (`paladin-ai`) correctly resolves to the root `CHANGELOG.md` and each crate to its own file. The heading pattern `^##\s*\[<exact tag version>\](\s|$)` anchors immediately after the bracketed version, so `## [1.2.3-rc.1]` and `## [1.2.30]` never satisfy `--tag v1.2.3`. A missing file and a present-but-sectionless file produce two distinct messages, both merged into clause 1's failures list and sorted by package name.
- **Real-tree proof:** `./scripts/check-release-consistency.sh --tag v0.8.1-rc.2` now fails, naming exactly the ten crate changelogs under `crates/` (all ten currently carry only `## [Unreleased]`) and correctly excluding the root `CHANGELOG.md` (already carries `## [0.8.1-rc.2] - 2026-08-27`) and `paladin-doc-examples` (`publish = false`). No manifest-version mismatch fires in the same run, proving clauses 1 and 2 are independent.
- **Clause 3 (CI-conclusion, D-10/PUBOPS-02):** `--ci-runs-json` is the fixture seam (identical parsing code path a live `gh api` response takes); otherwise `_crc_fetch_ci_runs` calls `gh api repos/{owner}/{repo}/actions/workflows/ci.yml/runs` with `head_sha`+`status=completed`, paginated. On a clean 404, it resolves `ci.yml`'s numeric workflow id from the `actions/workflows` list endpoint and retries -- Assumption A2 is *removed*, not carried on trust. Any other lookup failure is `CI_LOOKUP_FAILED`, distinct from "no successful run". The deciding run is chosen by sorting `created_at` ascending with `id` as the tiebreak and taking the last element -- never indexing unsorted.
- **Granularity decision recorded in the script header**, re-derived from `ci.yml` rather than assumed: `benchmark-regression-signal` is the only job with a job-level `continue-on-error`, and its `if:` restricts it to `pull_request`/`workflow_dispatch`, so it never runs on the `push` this clause inspects. No job in `ci.yml` is simultaneously known-flaky, release-irrelevant and able to turn the run red -- whole-run granularity is safe at this shape.
- **MISSING_SHA** fails the whole gate closed, before any clause runs, when `GITHUB_ACTIONS=true` and `--sha` is absent. Outside CI, an absent `--sha` still runs clauses 1-2 and the report states explicitly ("NOTE: the CI-conclusion clause (D-10) was not checked...") that clause 3 was skipped, so a local pass is never misread as a full-gate pass.
- **No `-L`/`--location` on any `curl` or `gh` call** (verified mechanically in Task 2's `<verify>`), per the credential-header control in `security.instructions.md`.
- **`.github/workflows/release.yml` deliberately untouched** by this plan (confirmed via `git status --short`): the script reads `--sha` from the CLI only; the workflow-side wiring is plan 20-05's job.

## Task Commits

Both tasks carried `tdd="true"` and followed RED/GREEN:

1. **Task 1 (RED): failing changelog-section fixtures** - `f1f27b46` (test) -- 9 new assertions in `tests/scripts/check-release-consistency_test.sh`, confirmed 9/23 failing against the clause-1-only script
2. **Task 1 (GREEN): changelog-section clause** - `3b7c7735` (feat) -- clause 2 implemented in `scripts/check-release-consistency.sh`; all 23 assertions pass; real-tree run fails naming exactly ten crate changelogs
3. **Task 2 (RED): failing CI-conclusion fixtures** - `1e15c81e` (test) -- 10 new assertions covering all 7 `<behavior>` cases, confirmed 8/32 failing against the clause-1/2-only script (one case -- "single success is silent" -- already passed trivially since the old script ignored `--sha`/`--ci-runs-json`)
4. **Task 2 (GREEN): CI-conclusion clause** - `c11cc5cb` (feat) -- clause 3, `_crc_fetch_ci_runs`, `MISSING_SHA`/`CI_LOOKUP_FAILED` short-circuits, granularity rationale in the header; all 32 assertions pass

**Plan metadata:** pending (this commit, `docs(20-02): complete plan`, made after this SUMMARY)

_TDD gate compliance: for each task, a `test(20-02)` commit (RED) precedes the corresponding `feat(20-02)` commit (GREEN) in git log -- gate sequence satisfied for both tasks._

## Files Created/Modified
- `scripts/check-release-consistency.sh` - clause 2 (changelog-section agreement) and clause 3 (CI-conclusion agreement) added on top of plan 20-01's clause 1; new `_crc_fetch_ci_runs` helper; header comment documents both clauses plus the granularity rationale; remediation footer extended with the new status tokens
- `tests/scripts/check-release-consistency_test.sh` - `write_metadata_fixture` now emits a real `manifest_path` per package (a scratch directory + `Cargo.toml`); new `write_changelog_fixture` and `write_ci_runs_fixture` helpers; 19 new assertions (9 for clause 2, 10 for clause 3), 32 total

## Decisions Made
- CI-conclusion granularity is the whole-run `ci.yml` conclusion, not a named job subset -- see `key-decisions` in frontmatter for the full re-derivation from `ci.yml`'s job list.
- Assumption A2 (workflow-filename-in-path acceptance) is removed via an always-present 404-then-numeric-id fallback, rather than assumed to hold for this repository.
- `CI_LOOKUP_FAILED` and `MISSING_SHA` are handled entirely at the bash level (before the python parser ever runs), keeping them structurally impossible to conflate with a clause 3 verdict computed from actual runs data.
- The changelog heading regex anchors `\]` immediately after the escaped tag version so no prefix-matching path exists, even accidentally.

## Deviations from Plan

**1. [Rule 1 - Bug] Fixed the offline "not checked" NOTE not appearing when clauses 1/2 fail**

- **Found during:** Task 2, verifying the real-tree `<verify>` command's third clause ("the offline run... states that the CI-conclusion clause was not checked")
- **Issue:** The first implementation only printed `ci_note` on the `OK` success path (`print("OK"); ...; if ci_note: print(ci_note)`). Against the real tree, clauses 1-2 fail (ten changelog issues), so the script took the failure branch and the "NOTE: the CI-conclusion clause (D-10) was not checked..." line never printed -- silently failing the plan's own acceptance criterion that a local pass *or fail* must never be misread as a full-gate verdict on clause 3.
- **Fix:** Added the same `if ci_note: print(ci_note)` call to the failure branch (right after the per-package failure list, before `sys.exit(0)`), so the note appears on both the OK and FAIL paths whenever clause 3 was not checked.
- **Files modified:** `scripts/check-release-consistency.sh` (same commit as Task 2's GREEN, `c11cc5cb` -- caught and fixed before committing)
- **Verification:** Re-ran `./scripts/check-release-consistency.sh --tag v0.8.1-rc.2` (no `GITHUB_ACTIONS`) and confirmed the NOTE line now appears at the end of the failing output; all 32 regression assertions still pass.
- **Committed in:** `c11cc5cb` (part of Task 2's GREEN commit -- caught during manual verification before commit, not a separate fix commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - Bug)
**Impact on plan:** Caught during the plan's own `<verify>` step before committing; no scope creep, no architectural change.

## Issues Encountered
- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings` step (`always_run: true`) took several minutes on a cold build cache, exceeding the default 2-minute Bash tool timeout on the first commit attempt (same issue plan 20-01 recorded). Resolved by warming the cache with a background `cargo clippy` run first, then retrying the commit with an extended timeout. No code change; a tooling/timing accommodation only.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `scripts/check-release-consistency.sh` now implements D-08 clauses 1-3 of 4 (manifest agreement, changelog-section agreement, CI-conclusion agreement); the 4th (SHA agreement) is out of this plan's scope per its frontmatter.
- The gate fails today's tree for exactly the ten reasons PUBOPS-01 predicts (the ten crate changelogs), and refuses to pass a CI-tagged run with no recorded SHA (`MISSING_SHA`) -- both verified against the real tree, not just fixtures.
- `--sha` is accepted and fully functional; plan 20-05's `release.yml` pass is what wires the workflow-side `--sha ${{ github.sha }}` (or equivalent) into the existing `check-release-consistency` job plan 20-01 added. No script-side blocker for that wiring.
- `write_changelog_fixture` and `write_ci_runs_fixture` are reusable as-is for any future clause's fixture cases, following the same `mktemp -d` + per-package-directory pattern the harness already establishes.
- No blockers identified for downstream plans in this phase.

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-28*

## Self-Check: PASSED

- FOUND: scripts/check-release-consistency.sh
- FOUND: tests/scripts/check-release-consistency_test.sh
- FOUND: .planning/phases/20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi/20-02-SUMMARY.md
- FOUND: f1f27b46 (Task 1 RED commit)
- FOUND: 3b7c7735 (Task 1 GREEN commit)
- FOUND: 1e15c81e (Task 2 RED commit)
- FOUND: c11cc5cb (Task 2 GREEN commit)
- FOUND: 0131fb55 (plan metadata commit)
