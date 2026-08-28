---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 03
subsystem: infra
tags: [github-actions, release, gh-cli, jq, bash, shellcheck, tdd]

# Dependency graph
requires:
  - phase: 20-01
    provides: "check-release-consistency.sh house shape (LIB_ONLY sourcing guard, mktemp+trap fixture harness) this plan's script and test mirror"
provides:
  - "scripts/create-or-reuse-release.sh -- create-or-reuse GitHub release via `gh api`, HTTP-status-driven (200 reuse / 404 create / other hard-fail), 422-then-refetch recovery, jq-built payload, CREATE_OR_REUSE_RELEASE_LIB_ONLY sourcing seam"
  - "tests/scripts/create-or-reuse-release_test.sh -- 19-assertion regression harness with a stubbed gh binary (GH_BIN seam), covering all 8 <behavior> cases plus usage errors and a tree-mutation guard"
  - "release.yml create-release job rewired onto create_or_reuse_release, no archived Action, upload_url/version outputs preserved"
  - "release.yml verify-tag-source job -- new `sha` output exposing the release commit verify-tag-source already resolves"
  - "release.yml check-release-consistency job -- now receives --sha and GH_TOKEN"
affects: [20-04, 20-05, 20-06, 20-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "gh api -i (--include) parsed for an explicit HTTP status line, never trusting gh's bare exit code, so a 404 (create) is never confused with a transport/auth failure (401/500) that must hard-fail with zero create attempts"
    - "GH_BIN test seam (mirrors the check-release-consistency.sh METADATA_JSON/WORKSPACE_ROOT seam pattern from 20-01): a stubbed gh shell script on GH_BIN reads scripted HTTP status/body from scratch-dir fixture files and appends a call-log line per invocation, so the regression harness never touches the network"
    - "jq -n --arg payload construction: commit-subject-derived release-body text reaches the GitHub API as a JSON string field, never interpolated into a shell command line or $GITHUB_OUTPUT"
    - "$GITHUB_OUTPUT heredoc replaced by a $RUNNER_TEMP file + single-line output (removes the injectable-delimiter hazard class entirely rather than defending it with a randomised delimiter)"

key-files:
  created:
    - scripts/create-or-reuse-release.sh
    - tests/scripts/create-or-reuse-release_test.sh
  modified:
    - .github/workflows/release.yml

key-decisions:
  - "gh's own exit code is deliberately ignored for lookup/create calls -- HTTP_STATUS is parsed from the `-i` response's status line via _cor_gh_call, and that parsed status (not the process exit code) drives every branch. This is the direct fix for Pitfall 1 (exit-code-only checks conflate 404 with a network failure)"
  - "The re-fetch-on-422 path re-uses the same lookup call path (_cor_gh_call GET .../releases/tags/<tag>) rather than a separate code path, so the 'reuse' outcome is identical whether reached via the initial 200 or the 422-recovery 200"
  - "REGISTRY and IMAGE_NAME (workflow-level env: values) are consumed as plain shell variables in the rewritten changelog step rather than via `${{ env.REGISTRY }}` interpolation -- since they are already real environment variables inside every run: step, this keeps the changelog step's run: body at zero `${{ }}` occurrences, satisfying the CR-01 acceptance check verbatim rather than needing a special-case exemption"
  - "--prerelease is not explicitly passed from release.yml to the script -- the script's own default (hyphen-in-tag) exactly reproduces the removed `contains(steps.get_version.outputs.version, '-')` expression, so no redundant duplicate computation was added to the workflow"

patterns-established:
  - "Pattern: an HTTP status is read from `gh api -i`'s response text via a helper (_cor_gh_call) that ignores the command's own exit code entirely and sets caller-scoped HTTP_STATUS/HTTP_BODY through bash's dynamic-scoping visibility of `local` variables across nested function calls -- reusable by any future script that needs a create-or-reuse or exists-check against a GitHub API resource"

requirements-completed: [PUBOPS-03]

coverage:
  - id: D1
    description: "create-or-reuse-release.sh makes a HTTP-status-driven decision (200 reuse / 404 create / other hard-fail) so create-release is safe to run twice on the same tag, with a 422-on-create (concurrent creation) recovered by one re-fetch"
    requirement: "PUBOPS-03"
    verification:
      - kind: unit
        ref: "tests/scripts/create-or-reuse-release_test.sh (19 assertions, all pass)"
        status: pass
      - kind: manual_procedural
        ref: "Manually exercised all 8 <behavior> cases plus missing-tag/unknown-flag against the stubbed gh binary during authoring (200-reuse, 404+201-create, 500, 401, 422-then-200, 422-then-404, 200-no-upload_url, EOF/backtick body-file literal, GITHUB_OUTPUT wiring)"
        status: pass
    human_judgment: false
  - id: D2
    description: "release.yml's create-release job is rewired onto the create-or-reuse script (archived actions/create-release@v1 removed, no new marketplace Action), preserving upload_url/version outputs for build-binaries/sbom, and verify-tag-source exposes its resolved sha for the consistency gate"
    requirement: "PUBOPS-03"
    verification:
      - kind: unit
        ref: "python3 structural assertion (outputs shape, needs edges, grep counts) -> 'release.yml wiring OK'; CR-01 no-`${{`-in-run: assertion -> 'CR-01 OK'"
        status: pass
      - kind: integration
        ref: "make check-workflow-triggers (trigger surface + required-check context resolution, unaffected by this plan's changes)"
        status: pass
    human_judgment: false

# Metrics
duration: ~28min (includes ~7min of cold cargo-clippy pre-commit-hook cache warm-up, not authoring time)
completed: 2026-08-28
status: complete
---

# Phase 20 Plan 03: Create-or-Reuse Release + Release-SHA Exposure Summary

**`create-release` now looks a GitHub release up by tag first (HTTP 200 reuses it, 404 creates it, anything else is a named hard failure) via a new `scripts/create-or-reuse-release.sh`, replacing the archived `actions/create-release@v1`; `verify-tag-source` exposes its resolved commit SHA as a job output for the consistency gate.**

## Performance

- **Duration:** ~28 min (2026-08-28T14:52Z – 2026-08-28T15:20Z), of which roughly 7 min was a cold `cargo clippy --workspace --all-targets --all-features` pre-commit-hook cache warm-up run twice in the background, not script/workflow authoring time
- **Tasks:** 2 (Task 1 tracer/tdd, Task 2 auto)
- **Files modified:** 3 (2 created, 1 modified)

## Accomplishments
- `scripts/create-or-reuse-release.sh`: the create-or-reuse decision is made purely from a parsed `gh api -i` HTTP status line (`_cor_gh_call` helper) -- `200` reuses, `404` creates, anything else (401, 403, 500, a transport failure with no status line at all) is a hard, named failure with **zero** create calls attempted. A `422` on create (a concurrent run creating the release between lookup and create) triggers exactly one re-fetch by tag, reusing on `200` and failing loudly otherwise. The release payload is built structurally with `jq -n --arg` and sent to `gh api --input -` on stdin, so a commit-subject-derived body -- including a line that is exactly `EOF`, or one with backticks and `$(...)` -- reaches the GitHub API as JSON data, never as interpolated shell text. `upload_url` is extracted with `jq -e` so a missing or null field fails loudly rather than emitting empty output; both `upload_url=` and `version=` are emitted to stdout and, when set, `$GITHUB_OUTPUT`. `gh` is resolved through `GH_BIN` (default `gh`) -- the seam the regression harness stubs so no assertion ever touches the network. `CREATE_OR_REUSE_RELEASE_LIB_ONLY` sourcing guard matches the house shape `check-release-consistency.sh` established in plan 20-01.
- `tests/scripts/create-or-reuse-release_test.sh`: 19-assertion regression harness. A `gh-stub.sh` written per-fixture-directory recognises the two endpoint shapes the script calls (`.../releases/tags/<tag>` and `.../releases`), reads scripted HTTP status/body from files an assertion writes beforehand, appends `LOOKUP`/`CREATE` to a call log, and captures a create call's stdin payload verbatim for the literal-content assertions. Covers all eight `<behavior>` cases (200-reuse, 404+201-create, 500, 401, 422-then-200-refetch, 422-then-404-refetch, 200-with-no-`upload_url`, `GITHUB_OUTPUT` wiring, `EOF`/backtick body-file literal) plus missing-`--tag` and unknown-flag usage errors and a real-tree no-mutation guard -- 7 more assertions than the plan's 12-assertion floor.
- `release.yml` `create-release` job: `actions/create-release@v1` deleted (0 remaining references), replaced by a `create_or_reuse_release` run: step invoking the new script with tag/repo/body-file passed through `env:` (CR-01) and `GH_TOKEN` from `secrets.GITHUB_TOKEN`. The `outputs:` block now reads `upload_url` from `steps.create_or_reuse_release.outputs.upload_url` while `version` is unchanged from `get_version` -- `build-binaries` and `sbom`'s `needs.create-release.outputs.upload_url` consumption (3 references, `actions/upload-release-asset@v1` also unchanged at 3) is untouched.
- `release.yml` `Generate changelog` step: the `$GITHUB_OUTPUT` heredoc with a randomised delimiter is replaced by writing the changelog text to `$RUNNER_TEMP/release-changelog.md` and exposing only that path as `changelog_file` -- the file the new script's `--body-file` consumes directly. This removes the injectable-delimiter hazard class entirely (a commit subject colliding with a random hex delimiter was already astronomically unlikely, but a file write has no delimiter to collide with at all).
- `release.yml` `verify-tag-source` job: gained an `outputs: sha: ${{ steps.resolve.outputs.sha }}` block, exposing the release commit SHA the job's existing `resolve` step already computes.
- `release.yml` `check-release-consistency` job: its `Run pre-publish consistency gate` step now passes `--sha "$RELEASE_SHA"` (sourced from `needs.verify-tag-source.outputs.sha` via `env:`) and `GH_TOKEN` from `secrets.GITHUB_TOKEN`, so a later plan's CI-conclusion clause in `check-release-consistency.sh` can authenticate and check against the exact release commit rather than re-deriving it.
- Every modified/added `run:` step in both jobs has zero `${{ }}` occurrences except the pre-existing, exempted `get_version` steps -- verified by the plan's own CR-01 structural assertion (`CR-01 OK`).

## Task Commits

Each task was committed atomically (Task 1 followed the TDD RED/GREEN cycle since it carries `tdd="true"`):

1. **Task 1 (RED): failing regression test** - `630af60b` (test) -- `tests/scripts/create-or-reuse-release_test.sh` written first; confirmed to fail (`ERROR: guard script not found`) before `scripts/create-or-reuse-release.sh` existed
2. **Task 1 (GREEN): create-or-reuse-release script** - `4b7e5bc9` (feat) -- `scripts/create-or-reuse-release.sh`, making the RED test (all 19 assertions) and the plan's own `<verify>` block pass
3. **Task 2: rewire create-release job, expose release SHA** - `c68ed651` (feat) -- `.github/workflows/release.yml`: `create-release` job rewired, `verify-tag-source` gains `sha` output, `check-release-consistency` wired to receive it

**Plan metadata:** pending (this commit, `docs(20-03): complete plan`, made after this SUMMARY)

_TDD gate compliance: `test(20-03)` commit (RED) precedes `feat(20-03)` commit (GREEN) in git log -- gate sequence satisfied for Task 1._

## Files Created/Modified
- `scripts/create-or-reuse-release.sh` - create-or-reuse GitHub release via `gh api -i`, HTTP-status-driven decision, jq-built payload, `CREATE_OR_REUSE_RELEASE_LIB_ONLY` sourcing seam
- `tests/scripts/create-or-reuse-release_test.sh` - 19-assertion regression harness with a stubbed `gh` binary (`GH_BIN` seam)
- `.github/workflows/release.yml` - `create-release` job rewired onto `create_or_reuse_release`; `Generate changelog` step writes to a `$RUNNER_TEMP` file instead of a `$GITHUB_OUTPUT` heredoc; `verify-tag-source` gains a `sha` output; `check-release-consistency` receives `--sha`/`GH_TOKEN`

## Decisions Made
- `gh`'s own process exit code is never consulted for the lookup/create decision -- only the parsed HTTP status line from `gh api -i`'s output. This is the direct application of the plan's Pitfall 1 guidance and is exercised by the 500/401 test cases, which assert both "no create call was made" (via the stub's call log) and that the failure message names the actual status.
- The 422-recovery re-fetch reuses the exact same `_cor_gh_call GET .../releases/tags/<tag>` call the initial lookup makes, rather than a separate code path -- so "reuse via initial 200" and "reuse via 422-then-refetch-200" produce byte-identical downstream behavior (same extraction/output logic runs either way).
- `REGISTRY`/`IMAGE_NAME` (workflow-level `env:` values) are read as plain shell variables (`$REGISTRY`, `$IMAGE_NAME`) in the rewritten changelog step rather than via `${{ env.REGISTRY }}` interpolation inside `run:` -- they are already real environment variables in every step by virtue of the workflow-level `env:` block, so this keeps that step's `run:` body at zero `${{ }}` occurrences without needing a special-case exemption in the CR-01 acceptance check.
- `--prerelease` is not passed explicitly from `release.yml` to the new script; the script's own default (`true` iff the tag contains a hyphen) exactly reproduces the removed `contains(steps.get_version.outputs.version, '-')` expression, avoiding a redundant duplicate computation in the workflow.

## Deviations from Plan

None - plan executed exactly as written. No Rule 1-4 auto-fixes were needed. The plan's `<action>`/`<read_first>` sections and the phase's `RESEARCH.md` (Pattern 1, Pitfall 1) and `PATTERNS.md` (the `check-release-consistency.sh` house-shape and CR-01 env-indirection analogs) were detailed enough to implement directly.

## Issues Encountered
- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings` step (`always_run: true`) exceeded the Bash tool's default and even a 3-minute extended timeout on a cold build cache during the first two commit attempts, matching the same tooling/timing accommodation 20-01's SUMMARY recorded. Resolved by running `cargo clippy` to completion in the background first (final run: 3m14s) and then retrying the commit against the warm cache, which completed quickly. No code change was needed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `scripts/create-or-reuse-release.sh`'s `GH_BIN` seam and `_cor_gh_call`'s HTTP-status-parsing helper are reusable patterns for any future script needing a create-or-reuse or exists-check against a GitHub API resource.
- `check-release-consistency.sh` now receives `--sha` in CI (via `release.yml`'s `check-release-consistency` job), matching the flag surface plan 20-01 already reserved as a no-op and plan 20-02's `MISSING_SHA`/CI-conclusion clause is expected to give behavior to -- the wiring is in place ahead of that script-side implementation landing.
- No blockers identified for downstream plans in this phase.

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-28*
