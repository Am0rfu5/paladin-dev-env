---
phase: 15-coverage-ci-quality-gates
plan: 02
subsystem: infra
tags: [github-actions, ci, codecov, actionlint, shellcheck, coverage]

# Dependency graph
requires:
  - phase: 15-coverage-ci-quality-gates (plan 15-01)
    provides: "the coverage job in ci.yml (Measure coverage step), which this plan's Codecov upload step attaches to"
provides:
  - "seven deprecated GitHub Action references upgraded (actions-rs/toolchain@v1 -> dtolnay/rust-toolchain@stable x4, actions/cache@v3 -> @v4 x3); one removed by deletion (codecov/codecov-action@v3, D-03)"
  - ".codecov.yml at repo root: report-only (informational: true on both status blocks), src/bin/** excluded (D-06)"
  - "codecov/codecov-action@v5 upload wired into ci.yml's coverage job, non-blocking"
  - "actionlint CI job (job id actionlint, name Workflow Lint) linting all six workflow files, zero continue-on-error"
  - ".github/actionlint.yaml: one narrowly-scoped suppression for a documented, functionally-necessary services `command:` key actionlint's schema doesn't recognize"
affects: [15-03, 15-05, 15-10]

# Tech tracking
tech-stack:
  added: ["actionlint@1.7.12 (pinned release binary, CI-downloaded)", "codecov/codecov-action@v5"]
  patterns: ["actionlint config suppression lives under paths.<glob>.ignore, not a top-level ignore: key (confirmed via actionlint -init-config)", "env VAR=val cmd prefix disambiguates shellcheck SC2209 when val also matches a shell builtin name"]

key-files:
  created: [".codecov.yml", ".github/actionlint.yaml"]
  modified: [".github/workflows/ci.yml", ".github/workflows/integration-tests.yml", ".github/workflows/release.yml"]

key-decisions:
  - "The plan's sanctioned suppression path ('an inline # actionlint-ignore comment') does not exist as an actionlint feature -- actionlint has no inline pragma comment mechanism (confirmed via `actionlint --help`: only a CLI -ignore flag and a config-file paths.<glob>.ignore list). Used .github/actionlint.yaml's paths block instead, narrowly scoped to the exact message text, plus an inline human-readable comment next to each suppressed construct -- same intent (narrow, written, reviewable), different mechanism."
  - "Investigated deleting the three `command: server /data ...` services keys as dead configuration (actionlint's schema doesn't recognize `command` under services) but reversed course: integration-tests.yml's own pre-existing comment ('MinIO requires a command argument') indicates the official minio/minio image's entrypoint requires this argument to start as a server rather than print usage and exit. Deleting it risked silently breaking MinIO startup in three jobs, unverifiable without a live GitHub Actions runner. Suppressed instead of deleted."

patterns-established:
  - "actionlint job runs before any toolchain/cache/service setup, deliberately the cheapest job in the pipeline (no needs:)"

requirements-completed: [PIPE-02, PIPE-04]

coverage:
  - id: D1
    description: "Seven deprecated actions-rs/toolchain@v1 and actions/cache@v3 references upgraded to dtolnay/rust-toolchain@stable and actions/cache@v4; the eighth (codecov/codecov-action@v3) and its two surrounding steps deleted from integration-tests.yml rather than upgraded, per D-03"
    requirement: "PIPE-04"
    verification:
      - kind: other
        ref: "grep -rE 'actions-rs/|cache@v3|codecov-action@v3' .github/workflows/ -- exit 1, zero matches"
        status: pass
      - kind: other
        ref: "grep -r 'integration-lcov' .github/workflows/ -- exit 1, zero matches"
        status: pass
      - kind: other
        ref: "git diff .github/workflows/integration-tests.yml shows no modification inside the Run integration tests step or its services: block"
        status: pass
    human_judgment: false
  - id: D2
    description: ".codecov.yml lands at repo root, report-only (informational: true on project and patch status blocks), src/bin/** and the six other ignore paths present, precision/round/range/comment-layout exact"
    requirement: "PIPE-02"
    verification:
      - kind: other
        ref: "python3 yaml.safe_load assertions on .codecov.yml (ignore set, precision, round, range, informational count, comment layout) -- all pass"
        status: pass
    human_judgment: false
  - id: D3
    description: "codecov/codecov-action@v5 upload step added to ci.yml's coverage job after Measure coverage, files: lcov.info, flags: combined, token: secrets.CODECOV_TOKEN, no fail_ci_if_error"
    requirement: "PIPE-02"
    verification:
      - kind: other
        ref: "python3 yaml assertion: exactly one codecov/codecov-action@v5 step in jobs.coverage.steps, with block has no fail_ci_if_error: true; grep -c 'fail_ci_if_error: true' ci.yml == 0; grep -c 'secrets.CODECOV_TOKEN' ci.yml == 1"
        status: pass
    human_judgment: true
    rationale: "This job has not executed in real GitHub Actions CI yet (no runner available in this authoring environment, same constraint noted by plan 15-01) -- a human/first CI run must confirm the upload actually reaches Codecov and CODECOV_TOKEN is configured as a repository secret."
  - id: D4
    description: "actionlint CI job (job id actionlint, name Workflow Lint) added to ci.yml with no needs:, no continue-on-error, targeting .github/workflows/*.yml (all six files); actionlint v1.7.12 reports zero findings against the tree as of this commit"
    requirement: "PIPE-04"
    verification:
      - kind: other
        ref: "python3 yaml assertion: 'actionlint' job present, no needs key, no step sets continue-on-error; ci.yml job count 18 -> 19, all name: values unique"
        status: pass
      - kind: other
        ref: "downloaded actionlint v1.7.12 to a scratch directory and ran `actionlint .github/workflows/*.yml` from the repo root -- exit 0, no output, .github/actionlint.yaml auto-discovered"
        status: pass
    human_judgment: false

duration: ~25min
completed: 2026-08-13
status: complete
---

# Phase 15 Plan 02: Deprecated action modernization, Codecov reporting, actionlint gate Summary

**Retired all eight deprecated GitHub Action references (seven upgraded, one deleted per D-03), landed report-only `.codecov.yml` with `src/bin/**` excluded and a non-blocking Codecov v5 upload, and added a standing `actionlint` CI job over all six workflows that reports zero findings after fixing five real pre-existing shellcheck issues it surfaced.**

## Performance

- **Duration:** ~25 min
- **Completed:** 2026-08-13T02:17:04Z
- **Tasks:** 3 of 3
- **Files modified:** 5 (`.github/workflows/ci.yml`, `.github/workflows/integration-tests.yml`, `.github/workflows/release.yml`, `.codecov.yml` created, `.github/actionlint.yaml` created)

## Accomplishments

- Retired every deprecated action reference across `.github/workflows/`: `actions-rs/toolchain@v1` → `dtolnay/rust-toolchain@stable` at four sites (`ci.yml`'s `api-surface`, `integration-tests` and `benchmark` jobs; `integration-tests.yml`'s `integration-tests` job); `actions/cache@v3` → `@v4` at three sites (`integration-tests.yml`); and deleted the `Generate integration test coverage` / `Upload integration coverage` step pair (which carried `codecov/codecov-action@v3`) from `integration-tests.yml` entirely (D-03), rather than upgrading it, since the replacement upload lands in `ci.yml`'s `coverage` job instead. Eight references existed at authoring time; seven upgraded, one removed by deletion.
- Created `.codecov.yml` at the repository root: `require_ci_to_pass: true`, `precision: 2`, `round: down`, `range: "70...100"`, PR comment layout `reach,diff,flags,files`, both `project` and `patch` status blocks set `informational: true` (D-02 — Codecov reports, never gates), and `ignore` covering all seven of `src/bin/**` (D-06, keeps the report denominator in agreement with the gate since all three `[[bin]]` targets are feature-gated behind `cli`/`web-server`), `tests/**`, `benches/**`, `examples/**`, `migrations/**`, `scripts/**`, `flat/**`. Added a `codecov/codecov-action@v5` upload step to the end of `ci.yml`'s `coverage` job with no `fail_ci_if_error`.
- Added a new `actionlint` job (`name: Workflow Lint`) to `ci.yml` — no `needs:`, downloads the pinned actionlint v1.7.12 release binary and lints all six workflow files. Job count went from 18 to 19, all `name:` values unique. Resolved every finding actionlint reported against the pre-existing tree (11 findings across `ci.yml` and `release.yml`) before landing the job, rather than narrowing its scope — see Deviations below.

## Task Commits

Each task was committed atomically:

1. **Task 1: Retire every deprecated action reference and delete the duplicate coverage path** - `f9b5ad2` (feat)
2. **Task 2: .codecov.yml and a non-blocking Codecov upload** - `7d74caa` (feat)
3. **Task 3: actionlint as a standing job over all six workflows** - `801a300` (feat)

_No plan-metadata commit — orchestrator policy for this worktree defers STATE.md/ROADMAP.md updates to post-wave._

## Files Created/Modified

- `.github/workflows/ci.yml` - Three `actions-rs/toolchain@v1` sites replaced; Codecov v5 upload added to `coverage` job; `actionlint` job added; two `services.command:` findings suppressed with inline comments; two `SC2034` unused-loop-var fixes; four `SC2209` false-positive fixes (`env` prefix)
- `.github/workflows/integration-tests.yml` - `actions-rs/toolchain@v1` → `dtolnay/rust-toolchain@stable`; three `actions/cache@v3` → `@v4`; `Generate integration test coverage` and `Upload integration coverage` steps deleted (D-03)
- `.github/workflows/release.yml` - Six `SC2086`/`SC2129` shellcheck findings fixed (quoted `$GITHUB_OUTPUT`/`$PREV_TAG`, grouped three appends into one redirect block) — pre-existing issues actionlint surfaced, resolved per this task's explicit "resolve every finding" bar
- `.codecov.yml` (new) - Report-only Codecov configuration, `src/bin/**` excluded
- `.github/actionlint.yaml` (new) - One narrowly-scoped `paths` suppression for the `services.command` key, documented inline

## Decisions Made

- **The plan's literal suppression mechanism ("inline `# actionlint-ignore` comment") does not exist.** actionlint has no inline pragma-comment feature (verified via `actionlint --help`: only a `-ignore` CLI flag and a config-file `paths.<glob>.ignore` list). Used `.github/actionlint.yaml` instead — same intent (a narrow, written, reviewable suppression scoped to one exact message, not a whole rule class or file), different mechanism. First attempt used a top-level `ignore:` key, which actionlint's config schema does not recognize (silently no-ops); corrected by running `actionlint -init-config` to discover the real schema (`paths.<glob>.ignore`), then re-verified locally that the corrected config actually suppresses the finding.
- **Did not delete the three `command: server /data [...]` services keys**, despite actionlint flagging them as an unrecognized schema key. Initially deleted them as presumed-dead configuration, then reversed course after noticing `integration-tests.yml`'s own pre-existing comment ("MinIO requires a command argument") — the official `minio/minio` image's entrypoint requires this argument to start as a server rather than print usage and exit, so removing it risks silently breaking MinIO startup in three service-backed jobs, unverifiable without a live GitHub Actions runner in this authoring environment. Suppressed via `.github/actionlint.yaml` instead, with an inline comment on each `command:` line pointing at the config and the reason.
- **Fixed rather than suppressed five real, pre-existing shellcheck findings** the new `actionlint` job surfaced in `ci.yml` and `release.yml` (unused loop variables, an `APP_ENV=test cmd` construct shellcheck misreads as a command-substitution typo since `test` is also a builtin name, unquoted `$GITHUB_OUTPUT`/`$PREV_TAG`, and un-grouped output-file appends). These predate this plan but the task's own action text is explicit: "Resolve every finding actionlint reports across the six files before this task is done" — this is the plan's own directive overriding the general pre-existing-issue scope boundary for this specific task, since establishing a zero-finding baseline is the task's entire point. All five fixes verified locally with `shellcheck` 0.9.0 before landing.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected the actionlint suppression mechanism from a non-existent inline pragma to the real config-file schema**
- **Found during:** Task 3 (actionlint job)
- **Issue:** The plan's acceptance criteria describe silencing false positives with "an inline `# actionlint-ignore` comment carrying a one-line reason next to the construct." actionlint has no such feature.
- **Fix:** Used `.github/actionlint.yaml`'s `paths.<glob>.ignore` list (the real, documented suppression mechanism, confirmed via `actionlint -init-config`), narrowly scoped to the exact `"unexpected key \"command\" for \"services\" section"` message text, plus an inline human-readable comment next to each affected `command:` line explaining the reason and pointing at the config.
- **Files modified:** `.github/actionlint.yaml` (new), `.github/workflows/ci.yml`
- **Verification:** Downloaded actionlint v1.7.12 locally and confirmed `actionlint .github/workflows/*.yml` exits 0 with the config in place, and exits 1 with the specific finding when the config is absent
- **Committed in:** `801a300` (Task 3 commit)

**2. [Rule 1 - Bug] Fixed five real pre-existing shellcheck findings the new actionlint job surfaced, rather than deferring them as out-of-scope**
- **Found during:** Task 3 (actionlint job)
- **Issue:** `actionlint .github/workflows/*.yml` reported 11 findings on the pre-existing tree before this task's own additions were even considered: 2x `SC2034` (unused loop variable `i` in two "Wait for services" steps), 4x `SC2209` (`APP_ENV=test cargo bench ...` misread as a command-substitution typo because `test` is also a shell builtin name), and 6x `SC2086`/`SC2129` in `release.yml` (unquoted `$GITHUB_OUTPUT`/`$PREV_TAG`, three separate appends that should be one redirect group).
- **Fix:** Renamed unused loop vars to `_`; prefixed the `APP_ENV=test` invocations with `env` to disambiguate (verified with `shellcheck` locally as the standard fix for this exact false-positive class); quoted the two variables in `release.yml` and grouped the three `$GITHUB_OUTPUT` appends into a single `{ ...; } >> "$GITHUB_OUTPUT"` block.
- **Files modified:** `.github/workflows/ci.yml`, `.github/workflows/release.yml`
- **Verification:** `shellcheck` 0.9.0 run locally against extracted script fragments for each fix, zero remaining findings; full `actionlint .github/workflows/*.yml` run locally, exit 0, no output
- **Committed in:** `801a300` (Task 3 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1 — the plan's literal instruction text conflicted with either actionlint's real feature set or the pre-existing tree's actual state; both fixes preserve the plan's stated intent)
**Impact on plan:** Both fixes preserve the plan's stated intent (zero actionlint errors, narrowly-scoped and documented suppressions only) exactly; no scope change, no architectural decision.

## Issues Encountered

- `actionlint` and `shellcheck` were not pre-installed in this environment. Per the orchestrator's explicit instruction, the `actionlint` release binary was downloaded to a scratch directory (not installed system-wide, not committed) and run locally against the real workflow tree to verify the job's `<verify>` bar (`actionlint .github/workflows/*.yml` exits 0, no output) before committing Task 3. `shellcheck` was already present in the environment (`/usr/bin/shellcheck`, 0.9.0) and used to verify each individual shell-script fix in isolation before applying it to the workflow files.
- No full-workspace `cargo build`/`clippy`/`test` was run for this plan, per explicit orchestrator instruction — this plan touches zero Rust source (only workflow YAML and one new config file), so those checks would provide no signal and would burn several minutes cold-compiling the workspace.

## User Setup Required

`CODECOV_TOKEN` must exist as a repository secret for Codecov reporting to function (carried over from plan 15-01's context, applies to this plan's upload step). Its absence does not fail any build (D-02) — it only means no report is posted. Recorded here rather than as a blocking `user_setup` frontmatter entry, matching the plan's own framing.

## Next Phase Readiness

- Plan 15-03 (coverage threshold / `--fail-under-lines`) can proceed — the `coverage` job's Codecov upload is wired and non-blocking, so setting the in-workflow `cargo llvm-cov --fail-under-lines` threshold does not need to coordinate with this plan's changes.
- **The `coverage` job's Codecov upload (D3) and the `actionlint` job (D4) have not executed in real GitHub Actions CI yet** — both are `human_judgment: true` in the coverage block above (D3 for the same reason plan 15-01 flagged its own coverage job: no live runner in this authoring environment; D4's local verification used a scratch-downloaded binary, not the actual CI runner environment, though the command is identical). The first real CI run on this branch should confirm both, particularly that `CODECOV_TOKEN` is configured and that the pinned actionlint download step succeeds in the GitHub-hosted runner.
- Plan 15-10 (requirement text amendments) can use this SUMMARY's before-state accounting verbatim: eight references existed, seven upgraded (four `actions-rs/toolchain@v1`, three `actions/cache@v3`), one deleted (`codecov/codecov-action@v3`, D-03); PIPE-04's stale line-number citations (`ci.yml:147`, `:317`, `:507`, `integration-tests.yml:71`, `:78`, `:84`, `:90`, `:123`) should be corrected against this plan's commits rather than re-derived.

---
*Phase: 15-coverage-ci-quality-gates*
*Completed: 2026-08-13*
