---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 05
subsystem: infra
tags: [github-actions, release, crates-io, curl, jq, bash, shellcheck, tdd]

# Dependency graph
requires:
  - phase: 20-03
    provides: "check-release-consistency.sh house shape (LIB_ONLY sourcing guard, mktemp+trap fixture harness, --sha wiring) this plan's script and test mirror; release.yml's needs edge on check-release-consistency"
provides:
  - "scripts/publish-crates.sh -- the whole publish-crates loop: crates.io versioned-endpoint pre-check (200/404/429-with-backoff/other-hard-fail) replacing error-prose matching, a bounded sparse-index visibility poll replacing sleep 20, a per-crate outcome table (published-now/already-at-this-version/skipped/failed) in dependency order, abort-to-skipped semantics on the first failure, and a real-run zero-published-now failure naming the version and pointing at docs/src/appendix/release-recovery.md; CURL_BIN/CARGO_BIN/PUBLISH_CRATES_LIB_ONLY seams"
  - "tests/scripts/publish-crates_test.sh -- 54-assertion regression harness with stubbed curl/cargo binaries, covering every case in both tasks' <behavior> plus a leading-v version-stripping case"
  - "release.yml publish-crates job's publish step reduced to a single ./scripts/publish-crates.sh invocation -- the eleven-crate array, the error-prose tolerance branch, and the fixed sleep 20 are gone from YAML"
affects: [20-06, 20-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Registry-state decision instead of matched command output: an HTTP status (200/404/429/other) from crates.io's versioned crate endpoint decides already-published, never text matched from cargo publish's stdout/stderr"
    - "Bounded poll instead of fixed pause: a real condition (the sparse index reporting the target version, non-yanked) is polled at a configurable interval up to a configurable timeout, and a timeout is recorded failed rather than assumed safe"
    - "set -e-safe non-zero-return capture: every call to a helper function that can legitimately return non-zero (404, poll-not-yet-visible) is written as `rc=0; helper ... || rc=$?` rather than a bare statement followed by `rc=$?` -- the bare form silently exits the script under set -euo pipefail before the assignment runs"
    - "CURL_BIN/CARGO_BIN test seams (mirrors the check-release-consistency.sh METADATA_JSON / create-or-reuse-release.sh GH_BIN seam pattern): stubbed binaries read scripted HTTP status/body per crate name from scratch-dir fixture files and append call-log lines, so the regression harness never touches the network or actually invokes cargo"
    - "grep -v with a literal '|' must use -F (fixed-string), not a BRE pattern with `\\|` -- GNU grep's BRE treats `\\|` as alternation, not a literal pipe, so `grep -v '^\\| Crate \\| Outcome \\|$'` silently excludes every line, not just the header row"
    - "A crate-name-suffixed local variable (RUN_CRATES) that ends in the same substring as another literal grep target (CRATES=() collides with the acceptance check's `grep -c 'CRATES=(' script` count) required renaming to CRATE_LIST to keep the canonical array's `CRATES=(` occurrence unique in the script"

key-files:
  created:
    - scripts/publish-crates.sh
    - tests/scripts/publish-crates_test.sh
  modified:
    - .github/workflows/release.yml

key-decisions:
  - "D-06 carrier verdict (native `cargo publish --workspace` rejected) recorded in the script's own header rather than only in RESEARCH.md/CONTEXT.md, so a future reader who opens the script directly sees the reasoning and the reopening condition without cross-referencing planning docs"
  - "The versioned crates.io api/v1 endpoint (not the sparse index) decides the pre-check (already-published); the sparse index (not the api/v1 endpoint) decides the post-publish visibility wait -- matching RESEARCH.md Pitfall 2's finding that the index, not the DB record, is what cargo's resolver actually reads for the next crate's dependency check"
  - "A yanked version is decided purely by the pre-check's HTTP status (200), not by inspecting the response body's `yanked` field -- crates.io's versioned endpoint returns 200 for a yanked version, so the status alone already encodes 'already published, can never be re-uploaded' without needing to parse the body for that specific check"
  - "The module-level default crate array is named `CRATES` (unchanged from the workflow's original variable name, moved verbatim per the plan's instruction), while the per-run selected list (which can be overridden by --crates-file) is named `CRATE_LIST` -- keeping `CRATES=(` a single, unambiguous occurrence in the script for the plan's own acceptance check"
  - "A leading 'v' is stripped from --version inside the script (`VERSION=\"${VERSION#v}\"`), mirroring check-release-consistency.sh's identical `TAG_VERSION=\"${TAG#v}\"` convention -- release.yml passes the raw tag-derived version (which carries the v prefix) through env:, and crates.io version strings never carry one"

patterns-established:
  - "Pattern: a per-crate registry pre-check + bounded index-visibility poll pair, each backed by its own CURL_BIN-seamed helper function, composable into a per-crate outcome recorder that any future script publishing to a package registry with similar propagation-lag semantics can reuse"

requirements-completed: [PUBOPS-03, PUBOPS-04]

coverage:
  - id: D1
    description: "scripts/publish-crates.sh decides already-published from a crates.io registry-state HTTP status (200/404, with bounded 429 retry, other status a hard per-crate failure) rather than matched cargo publish output, and a yanked version still counts as already-published"
    requirement: "PUBOPS-03"
    verification:
      - kind: unit
        ref: "tests/scripts/publish-crates_test.sh (cases 1, 2, 6, 7 -- pre-check 200/yanked-200/429-retry/500, 20 assertions)"
        status: pass
    human_judgment: false
  - id: D2
    description: "A bounded poll of the crates.io sparse index replaces the fixed sleep 20 between crates -- a version visible before timeout is published-now, and a version not visible by timeout is failed, never assumed safe"
    requirement: "PUBOPS-03"
    verification:
      - kind: unit
        ref: "tests/scripts/publish-crates_test.sh (cases 3, 4, 5 -- first-iteration success, third-iteration success, timeout-equals-timeout/interval failure)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Every crate in a run ends in exactly one of published-now/already-at-this-version/skipped/failed; a crate failure aborts every subsequent crate in dependency order to skipped with no publish attempted; a Markdown outcome table in declared crate order is written to stdout and $GITHUB_STEP_SUMMARY, byte-identical across two runs over identical registry state"
    requirement: "PUBOPS-04"
    verification:
      - kind: unit
        ref: "tests/scripts/publish-crates_test.sh (cases A-I -- all-404, all-200, partial-recovery, mid-loop failure with exact publish-attempt count, dry-run, GITHUB_STEP_SUMMARY set/unset, row order, idempotent output, state validity)"
        status: pass
    human_judgment: false
  - id: D4
    description: "A real run in which zero crates reach published-now fails with a message naming the version, stating the tag appears fully published, and pointing at docs/src/appendix/release-recovery.md; a dry-run is exempt from this rule and always exits zero"
    requirement: "PUBOPS-04"
    verification:
      - kind: unit
        ref: "tests/scripts/publish-crates_test.sh (case B -- all-already-published exits non-zero with the three message components; case E -- dry-run exits zero and never fires the message)"
        status: pass
    human_judgment: false
  - id: D5
    description: "release.yml's publish-crates job publish step is a single ./scripts/publish-crates.sh invocation -- the eleven-crate array, error-prose tolerance, and fixed sleep are gone from YAML; environment/permissions/needs and the on: trigger block are unchanged"
    requirement: "PUBOPS-03"
    verification:
      - kind: unit
        ref: "python3 structural assertion (job wiring, needs edges, absence of CRATES=(/already (exists|uploaded) in YAML) -> 'publish-crates wiring OK'; CR-01 no-${{-in-run: assertion -> 'CR-01 OK'"
        status: pass
      - kind: integration
        ref: "make check-workflow-triggers (on: block byte-identical, verified directly against the diff)"
        status: pass
    human_judgment: false

# Metrics
duration: ~75min (includes two cold cargo-clippy pre-commit-hook cache warm-ups, background-executed, not authoring time)
completed: 2026-08-28
status: complete
---

# Phase 20 Plan 05: Registry-State Publish Loop + Honest Outcome Reporting Summary

**`scripts/publish-crates.sh` replaces the publish loop's error-prose matching and fixed `sleep 20` with a crates.io registry-state pre-check and a bounded sparse-index visibility poll, emits a per-crate outcome table in dependency order, and fails a real run that moved zero crates -- `release.yml`'s `publish-crates` job is reduced to a single invocation of it.**

## Performance

- **Duration:** ~75 min (2026-08-28T14:38Z – 2026-08-28T15:53Z UTC, wall clock across the session), of which roughly 40 min was two cold `cargo clippy --workspace --all-targets --all-features -- -D warnings` pre-commit-hook cache warm-ups run in the background, not script/workflow authoring or debugging time
- **Tasks:** 3 (Task 1 tracer/tdd + Task 2 tdd delivered as one RED/GREEN pair, plus Task 3 auto)
- **Files modified:** 3 (2 created, 1 modified)

## Accomplishments
- `scripts/publish-crates.sh`: a `_pc_crate_published` helper decides already-published purely from the versioned `https://crates.io/api/v1/crates/<name>/<version>` endpoint's HTTP status -- `200` (including a yanked version, which still returns 200 and can never be re-uploaded) skips the publish and records `already-at-this-version`; `404` proceeds to `cargo publish`; `429` retries up to 3 times with a growing pause (`attempt * 3` seconds) before becoming a hard failure; any other status is an immediate hard failure with zero publish attempts. A `_pc_wait_for_index_visibility` helper polls `https://index.crates.io/<sparse-index-path>` at the configured interval until the just-published version appears non-yanked or the configured timeout is reached, recording `published-now` or `failed` accordingly -- the timeout case's iteration count is exactly `timeout / interval`, never a guess. Every crates.io call carries the required `User-Agent` header and never passes `-L`/`--location`. No bare `sleep <literal>` exists anywhere in the file (verified by the plan's own grep); every wait is `sleep "$variable"`.
- Per-crate outcomes accumulate into an associative `OUTCOME` array; a failure aborts every remaining crate in declared dependency order to `skipped` with zero further publish attempts (verified directly against the cargo stub's call log: exactly 3 publish attempts across an 11-crate mid-loop-failure fixture). A Markdown outcome table -- iterated in declared crate order so two runs over identical registry state produce byte-identical output -- is written to stdout and, when `$GITHUB_STEP_SUMMARY` is set, appended there too (byte-identical to stdout, asserted directly). A real run in which zero crates reach `published-now` fails with a message naming the version, stating the tag appears fully published, and pointing at `docs/src/appendix/release-recovery.md`; a dry-run always exits zero regardless of outcome counts and never fires that message.
- `CURL_BIN`/`CARGO_BIN` seams (default `curl`/`cargo`) and a `PUBLISH_CRATES_LIB_ONLY` sourcing guard let the regression harness exercise every function -- including internal helpers like `_pc_crate_published` and `_pc_wait_for_index_visibility` directly -- with no network call and no actual `cargo publish` invocation. The eleven-entry `CRATES=(...)` array and its `paladin-herald`-after-`paladin-ports` dependency-order comment moved into the script unchanged (Phase 19's reconciled output, consumed not re-derived); a `--crates-file` flag overrides it purely as a harness seam, with an empty resulting list treated as a named `ZERO_CRATES`-class failure rather than a silent no-op pass.
- `tests/scripts/publish-crates_test.sh`: 54-assertion fixture-driven harness. Two stub scripts (`curl-stub.sh`, `cargo-stub.sh`) are written per test-case scratch directory; `curl-stub.sh` distinguishes the versioned pre-check URL shape from the sparse-index URL shape, reads scripted HTTP status/body per crate name from fixture files (single-value or sequenced, for retry/multi-poll cases), and logs every invocation's full argv (proving User-Agent presence and `-L`/`--location` absence); `cargo-stub.sh` logs every `publish` invocation and can be told to fail for a named crate via a marker file. Covers all eleven Task 1 `<behavior>` cases, all nine Task 2 `<behavior>` cases, and an added case verifying a leading `v` on `--version` is stripped (release.yml passes the raw tag). A tree-mutation guard confirms the harness never writes into `scripts/` or `.github/workflows/`.
- `release.yml`'s `publish-crates` job "Publish crates in dependency order" step is now `./scripts/publish-crates.sh --version "$RELEASE_VERSION" [--dry-run]`, with `RELEASE_VERSION` sourced from `needs.create-release.outputs.version` and the dry-run flag from the existing `mode` step's output, both through `env:` (CR-01). The "Determine publish mode" step's dispatch-input read (`github.event.inputs.dry_run`) also moved from direct `${{ }}` interpolation in the `run:` body into an `env:` key -- the last remaining direct interpolation in the job. `environment: crates-io`, the `permissions` block, and the `needs: [test, create-release, check-release-consistency]` edge are byte-for-byte unchanged; the `on:` trigger block is untouched (confirmed directly against the diff, which starts at line 485). The job's leading comment now records what changed (bounded polls, registry-state skip-ahead on recovery re-runs) and what did not (the T-19-13 single-token-mint accepted risk).

## Task Commits

Each task was committed atomically. Tasks 1 and 2 (both `tdd="true"`, both modifying the same two files with cumulative, tightly-coupled behavior) were delivered as a single RED/GREEN pair rather than two separate pairs -- see Deviations below:

1. **Tasks 1+2 (RED): failing regression harness** - `f627fcbb` (test) -- `tests/scripts/publish-crates_test.sh` written first (54 assertions covering both tasks' `<behavior>`); confirmed to fail with `ERROR: guard script not found` before `scripts/publish-crates.sh` existed
2. **Tasks 1+2 (GREEN): registry-state publish loop + outcome table** - `5890dbe4` (feat) -- `scripts/publish-crates.sh`, making all 54 RED-test assertions and both tasks' `<verify>` blocks pass
3. **Task 3: reduce publish-crates job to a thin invocation** - `66e72fa0` (feat) -- `.github/workflows/release.yml`: the publish step now calls the script; eleven-crate array, error-prose tolerance and fixed sleep removed from YAML

**Plan metadata:** pending (this commit, `docs(20-05): complete plan`, made after this SUMMARY)

_TDD gate compliance: `test(20-05)` commit (RED) precedes `feat(20-05)` commit (GREEN) in git log -- gate sequence satisfied._

## Files Created/Modified
- `scripts/publish-crates.sh` - the whole publish-crates loop: registry-state pre-check, bounded index-visibility poll, per-crate outcome table, abort-to-skipped semantics, no-crate-moved failure
- `tests/scripts/publish-crates_test.sh` - 54-assertion regression harness with stubbed `curl`/`cargo` binaries (`CURL_BIN`/`CARGO_BIN` seams)
- `.github/workflows/release.yml` - `publish-crates` job's publish step reduced to a single script invocation; dispatch-input read moved into `env:`

## Decisions Made
- The D-06 carrier verdict (native `cargo publish --workspace` rejected as the loop's carrier) is recorded in the script's own header comment, not only in the phase's planning docs, so a future reader who opens `scripts/publish-crates.sh` directly sees the reasoning and the specific condition that would reopen it, without needing to cross-reference `RESEARCH.md`.
- The pre-check uses the versioned `api/v1` endpoint (DB-record visibility, the right question for "did we already publish this"); the post-publish wait polls the sparse index (the thing cargo's own resolver reads for a dependent's build) -- two different crates.io surfaces for two different questions, per `RESEARCH.md` Pitfall 2, rather than one endpoint pressed into both roles.
- A yanked version is decided purely from the pre-check's HTTP status (still 200), not by parsing the response body for a `yanked` field -- simpler and sufficient, since the status code alone already answers "can this version ever be uploaded again."
- Renamed the per-run selected crate list from `CRATES` to `CRATE_LIST` (keeping the module-level default array named `CRATES`, moved verbatim from the workflow) after discovering the plan's own acceptance check (`grep -c 'CRATES=(' scripts/publish-crates.sh` must equal 1) is a literal substring match that a `local -a CRATES=()` declaration would also satisfy, inflating the count to 3.
- A leading `v` is stripped from `--version` inside the script, mirroring `check-release-consistency.sh`'s identical convention, since `release.yml` passes the raw tag-derived version (which carries the `v` prefix from `${GITHUB_REF#refs/tags/}` or the dispatch input) and crates.io version strings never carry one.

## Deviations from Plan

**1. [Process deviation, not a Rule 1-4 auto-fix] Tasks 1 and 2 delivered as a single RED/GREEN TDD pair instead of two.**
- **Found during:** Planning the task-by-task commit sequence before starting Task 1.
- **Reasoning:** Both tasks are `tdd="true"` and both modify the exact same two files (`scripts/publish-crates.sh`, `tests/scripts/publish-crates_test.sh`) with cumulative behavior -- Task 2's outcome-table and exit-rule logic operates directly on top of Task 1's per-crate routine and cannot be meaningfully tested or reasoned about in isolation without either (a) shipping an incomplete script whose `<verify>` block would fail Task 2's own later acceptance criteria, or (b) writing and discarding an intermediate test file twice. A single RED (54-assertion test file covering both tasks' full `<behavior>`) followed by a single GREEN (the complete script satisfying both tasks' acceptance criteria) was chosen instead, since it preserves the RED-before-GREEN gate the TDD discipline exists to enforce (confirmed: the test file fails with "guard script not found" before the script exists) without producing a misleading intermediate "complete" state that isn't.
- **Impact:** No behavior gap -- every `<behavior>` case from both tasks is covered by a distinct, named assertion in the final 54-assertion suite, and every acceptance criterion from both tasks' `<acceptance_criteria>` sections was independently verified via the exact commands specified. This is a commit-sequencing choice, not a scope or correctness change; flagged here rather than silently merged so a reviewer can see the reasoning.

**2. [Rule 1 - Bug] Fixed `set -e` silently truncating the publish loop on an expected non-zero return.**
- **Found during:** First test run against the initial script draft (Tasks 1+2 combined implementation), before any commit.
- **Issue:** `_pc_crate_published` (404 = not yet published, a normal condition) and `_pc_version_in_index` (not-yet-visible, a normal polling condition) both legitimately return non-zero for expected states, not errors. Written as a bare statement (`_pc_crate_published "$name" "$version"; pc_rc=$?`), `set -euo pipefail` treats that non-zero return as a command failure and exits the entire script before the following `pc_rc=$?` assignment ever runs -- silently truncating every real-404 or real-not-yet-visible path.
- **Fix:** Rewrote both call sites as `rc=0; helper ... || rc=$?`, which the `||` protects from `set -e` (the whole `A || B` list is exempt), while still capturing the real exit code.
- **Verification:** Confirmed by running the test suite before and after -- 9 of the initial 50 assertions failed identically to this pattern (cases 3-6, timing/iteration-count assertions that depend on the poll loop actually completing multiple iterations); all 9 passed once fixed, with no other change to test expectations.
- **Files modified:** `scripts/publish-crates.sh`
- **Committed in:** `5890dbe4` (part of the Tasks 1+2 GREEN commit -- the bug was found and fixed before any commit, so no separate fix commit exists)

**3. [Rule 1 - Bug] Fixed a BRE `\|` alternation bug in the test harness's own header-row exclusion.**
- **Found during:** Debugging the "table row order" and "every crate appears exactly once" assertions, which returned empty actual-output despite the table clearly being present in `LAST_OUTPUT`.
- **Issue:** `grep -v '^\| Crate \| Outcome \|$'` (no `-E`) is a POSIX BRE in which GNU grep's `\|` is an *alternation* operator, not an escaped literal pipe -- the pattern was silently interpreted as an alternation across several near-empty branches, one of which matched every line, excluding the entire table instead of just the header row.
- **Fix:** Replaced with `grep -vF '| Crate | Outcome |'` (fixed-string match), which has no regex metacharacter interpretation at all.
- **Verification:** Both previously-empty assertions ("table row order equals the declared crate order", "every crate appears exactly once with a valid state") pass after the fix; isolated with a minimal `printf | grep -v` reproduction before applying the fix to confirm the root cause.
- **Files modified:** `tests/scripts/publish-crates_test.sh`
- **Committed in:** `f627fcbb` (part of the RED commit -- found and fixed while authoring the test file, before the first commit attempt)

---

**Total deviations:** 1 process deviation (task-pairing) + 2 auto-fixed bugs (both found and fixed before any commit landed, so neither produced a separate fix commit).
**Impact on plan:** All three necessary for a correct, passing implementation. No scope creep -- no functionality beyond both tasks' `<action>`/`<behavior>` was added.

## Issues Encountered
- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings` step exceeded the Bash tool's default and extended timeouts on a cold build cache during the first two commit attempts (RED and GREEN), matching the identical tooling/timing accommodation 20-03's SUMMARY recorded. Resolved the same way: ran `cargo clippy` to completion in the background first (warm-up completed successfully), then retried each commit against the warm cache, which completed in well under a minute each time. No code change was needed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `scripts/publish-crates.sh`'s `CURL_BIN`/`CARGO_BIN` seams and its `_pc_crate_published`/`_pc_wait_for_index_visibility` helper pair are reusable patterns for any future script that needs a registry-state pre-check + bounded-visibility-wait against a package registry with similar propagation-lag semantics.
- `docs/src/appendix/release-recovery.md`, which the no-crate-moved failure message points at, does not exist yet -- it is Phase 20's D-13 deliverable, scoped to a later plan in this phase (not this one). The message's reference is forward-looking and correct once that plan lands; verified the path string matches D-13's stated location exactly.
- No blockers identified for downstream plans in this phase.

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-28*

## Self-Check: PASSED

- FOUND: scripts/publish-crates.sh
- FOUND: tests/scripts/publish-crates_test.sh
- FOUND: f627fcbb (RED test commit)
- FOUND: 5890dbe4 (GREEN feat commit)
- FOUND: 66e72fa0 (Task 3 workflow-rewiring commit)
