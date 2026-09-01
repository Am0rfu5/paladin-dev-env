---
phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi
plan: 01
subsystem: infra
tags: [github-actions, release, cargo-metadata, bash, python3, shellcheck, tdd]

# Dependency graph
requires: []
provides:
  - "scripts/check-release-consistency.sh -- the pre-publish consistency gate script (D-08 clause 1: tag-vs-manifest-version agreement), sourceable via CHECK_RELEASE_CONSISTENCY_LIB_ONLY, with --sha/--ci-runs-json reserved flags for later plans' clauses"
  - "tests/scripts/check-release-consistency_test.sh -- 13-assertion regression harness in the house fixture-lifecycle shape (mktemp+trap, assert_fire/assert_silent, tree-mutation guard)"
  - "make check-release-consistency RELEASE_TAG=<tag> -- local-parity entry point, deliberately excluded from check-gates"
  - "release.yml check-release-consistency job -- wired into publish-crates's needs, so no cargo publish can start before it passes"
  - "make test-shell-guards glob loop over tests/scripts/*_test.sh -- the shape every later plan's guard test (20-03..20-05) plugs into automatically"
affects: [20-02, 20-03, 20-04, 20-05, 20-06, 20-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Gate script house shape: bash wrapper + python3 heredoc, STATUS_LINE/DETAIL report, collect-then-report (never fail-fast), named ZERO_*/MISSING_* failures for empty-discovery, byte-identical idempotent output"
    - "CHECK_RELEASE_CONSISTENCY_LIB_ONLY sourcing seam for future harnesses to call the script's main function directly"
    - "Makefile glob-loop test runner (test-shell-guards) replacing a hardcoded per-file invocation list"

key-files:
  created:
    - scripts/check-release-consistency.sh
    - tests/scripts/check-release-consistency_test.sh
  modified:
    - Makefile
    - .github/workflows/release.yml

key-decisions:
  - "Metadata is always passed to the python parser via a real file path (temp file when --metadata-json is absent), not an env var or heredoc stdin -- avoids the ~128KB MAX_ARG_STRLEN risk of embedding cargo metadata's ~90KB real-tree output in a single argv/envp string"
  - "Success output includes the literal token 'OK' (not just a checkmark and prose) so the plan's own <behavior> assertion ('output contains OK') is satisfied without special-casing the test"
  - "check-release-consistency deliberately excluded from check-gates (documented as a Makefile comment): every check-gates sibling is a no-argument offline guard; this one requires RELEASE_TAG, and folding it in without a default would recreate the exact silently-guessed-tag failure mode the MISSING_TAG check exists to prevent"

patterns-established:
  - "Pattern: gate script argument surface is flags-only (--tag, --metadata-json, --workspace-root, plus reserved --sha/--ci-runs-json), not positional, so later plans can extend the CLI without a breaking shape change"

requirements-completed: [PUBOPS-01, PUBOPS-03]

coverage:
  - id: D1
    description: "check-release-consistency.sh enumerates publishable Cargo workspace packages via cargo metadata (never a hardcoded list) and fails closed on any tag/manifest version mismatch, reporting every offender in one run"
    requirement: "PUBOPS-01"
    verification:
      - kind: unit
        ref: "tests/scripts/check-release-consistency_test.sh (13 assertions, all pass)"
        status: pass
      - kind: manual_procedural
        ref: "./scripts/check-release-consistency.sh --tag v0.8.1-rc.2 (exit 0, all 11 real manifests match); --tag v9.9.9 (exit 1, all 11 named, doc-examples correctly excluded)"
        status: pass
    human_judgment: false
  - id: D2
    description: "release.yml's publish-crates job cannot start cargo publish until check-release-consistency has passed (needs edge), and the gate is reachable identically via make locally"
    requirement: "PUBOPS-03"
    verification:
      - kind: unit
        ref: "python3 -c \"...assert 'check-release-consistency' in j['publish-crates']['needs']...\" -> 'workflow wiring OK'"
        status: pass
      - kind: manual_procedural
        ref: "make check-release-consistency RELEASE_TAG=v0.8.1-rc.2 (exit 0); make check-release-consistency with no argument (exit 1, names RELEASE_TAG)"
        status: pass
    human_judgment: false

# Metrics
duration: ~20min
completed: 2026-08-28
status: complete
---

# Phase 20 Plan 01: Pre-Publish Consistency Gate (Tracer) Summary

**One-clause pre-publish gate (tag vs. every publishable manifest version, exact string equality via `cargo metadata`) wired end to end: script -> `make check-release-consistency` -> a new `release.yml` job that `publish-crates` structurally cannot bypass.**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-08-28T14:26:59Z (session start; STATE.md `last_updated`)
- **Completed:** 2026-08-28T14:48:25Z
- **Tasks:** 2 (Task 1 tracer/tdd, Task 2 auto)
- **Files modified:** 4 (2 created, 2 modified)

## Accomplishments
- `scripts/check-release-consistency.sh`: D-08 clause 1 (manifest agreement) implemented as the single source of truth both CI and `make` invoke -- enumerates publishable packages from `cargo metadata --no-deps --format-version 1` (`publish == null`), compares each `version` to the tag's version (at most one leading `v` stripped) by exact string equality, accumulates every mismatch before reporting, and names `ZERO_PACKAGES`/`MISSING_TAG` as distinct non-zero failures rather than a vacuous pass
- `release.yml` gains a `check-release-consistency` job (`contents: read` only, `needs: verify-tag-source`, CR-01 env-indirection for the tag-derived version) added to `publish-crates`'s `needs: [test, create-release, check-release-consistency]` -- no `cargo publish` can run before this gate passes. Not added to `create-release`/`build-docker`/`build-binaries`/`sbom` (WR-05's documented asymmetry preserved deliberately, with a comment explaining why)
- `Makefile` gains `check-release-consistency` (RELEASE_TAG-required, usage error otherwise; deliberately excluded from `check-gates` with a comment explaining why) and a rewritten `test-shell-guards` that loops over `tests/scripts/*_test.sh` instead of three hardcoded filenames -- named failure on zero matches, so a future plan's guard test cannot be silently left out
- `tests/scripts/check-release-consistency_test.sh`: 13-assertion regression harness in the house fixture-lifecycle shape (single `mktemp -d` + `trap cleanup EXIT`, `write_metadata_fixture` helper, `assert_fire`/`assert_silent` needle-pinning pair, before/after `git status --porcelain` tree-mutation guard over `Cargo.toml`/`crates/`/`CHANGELOG.md`/`.github/workflows/`)
- Verified against the real tree: `--tag v0.8.1-rc.2` passes (all 11 publishable manifests at that version today); `--tag v9.9.9` fails and names all 11 (`paladin-ai`, `paladin-ai-core`, `paladin-battalion`, `paladin-content`, `paladin-herald`, `paladin-llm`, `paladin-memory`, `paladin-notifications`, `paladin-ports`, `paladin-storage`, `paladin-web`), correctly excluding `paladin-doc-examples` (`publish = false`); two runs of the same failing invocation are byte-identical

## Task Commits

Each task was committed atomically (Task 1 followed the TDD RED/GREEN cycle since it carries `tdd="true"`):

1. **Task 1 (RED): failing regression test** - `50bb5340` (test) -- `tests/scripts/check-release-consistency_test.sh` written first; confirmed to fail (`ERROR: guard script not found`) before any implementation existed
2. **Task 1 (GREEN): gate script + make target + release.yml wiring** - `de9b4773` (feat) -- `scripts/check-release-consistency.sh`, the `Makefile` target, and the `release.yml` job/needs-edge, making the RED test (and the plan's own `<verify>` block) pass end to end
3. **Task 2: harden the harness to the full house pattern** - `b5bda678` (test) -- `write_metadata_fixture`, tree-mutation guard, four new fixture cases (single-package, publish-false-only, no-leading-v, unknown-flag), and the `test-shell-guards` glob-loop rewrite

**Plan metadata:** pending (this commit, `docs(20-01): complete plan`, made after this SUMMARY)

_TDD gate compliance: `test(20-01)` commit (RED) precedes `feat(20-01)` commit (GREEN) in git log -- gate sequence satisfied for Task 1._

## Files Created/Modified
- `scripts/check-release-consistency.sh` - the pre-publish consistency gate (D-08 clause 1: manifest agreement); flags-only CLI (`--tag`, `--metadata-json`, `--workspace-root`, reserved `--sha`/`--ci-runs-json`); `CHECK_RELEASE_CONSISTENCY_LIB_ONLY` sourcing seam
- `tests/scripts/check-release-consistency_test.sh` - 13-assertion regression harness in the repo's established guard-test shape
- `Makefile` - new `check-release-consistency` target (excluded from `check-gates`, wired into `test-shell-guards`); `test-shell-guards` rewritten as a glob loop over `tests/scripts/*_test.sh`
- `.github/workflows/release.yml` - new `check-release-consistency` job; `publish-crates`'s `needs` list extended with it

## Decisions Made
- Metadata JSON is always read from a real file path in the python parser (a temp file when `--metadata-json` is not given), not passed via an environment variable or heredoc stdin -- the real tree's `cargo metadata --no-deps` output is ~90KB, close enough to Linux's ~128KB `MAX_ARG_STRLEN` per-argv/envp-string limit that an env-var approach was judged too fragile as the workspace grows
- The success-path output includes the literal token `OK` (not just a checkmark), because the plan's own `<behavior>` block states the success case's output must contain `OK` -- the repo's other gate scripts (`check-workflow-triggers.sh`, `check-changelogs.sh`) don't literally print `OK` in their success prose, so this is a small, deliberate departure from those siblings' exact success-message wording, documented inline in the script's usage comment
- `check-release-consistency` is deliberately NOT added to the `check-gates` composite target, with a Makefile comment explaining why: every other `check-gates` member is a no-argument offline guard runnable against the tree as-is, while this one requires a release tag to check against -- folding it in without a default would recreate exactly the silently-guessed-tag failure mode the guard's own `MISSING_TAG` check exists to prevent

## Deviations from Plan

None - plan executed exactly as written. No Rule 1-4 auto-fixes were needed; the plan's `<action>` and `<read_first>` sections were detailed enough to implement directly against the named analogs (`check-workflow-triggers.sh`, `check-changelogs.sh`, `check-workflow-triggers_test.sh`, the `test`/`sbom` jobs in `release.yml`, and the `check-changelogs`/`release` Makefile targets).

## Issues Encountered
- The pre-commit hook's `cargo clippy --workspace --all-targets --all-features -- -D warnings` step runs on every commit regardless of which files changed (`always_run: true` in `.pre-commit-config.yaml`) and took several minutes on a cold build cache, exceeding the default 2-minute Bash tool timeout on the first commit attempt. Resolved by running `cargo clippy` directly in the background first to warm the cache, then retrying the commit (which completed quickly against the warm cache). No code change was needed; this was purely a tooling/timing accommodation, not a deviation from the plan.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The tracer's feedback gate (this plan's own `<verify>` block, run against the real tree) confirms the wired path end to end: script standalone, `make check-release-consistency`, and `release.yml`'s structural `needs` edge all agree. Plan 20-02 can now add D-08's remaining three clauses (root/per-crate changelog dates, PUBOPS-02 CI-conclusion check) on top of a proven foundation rather than a hypothetical one.
- `--sha` and `--ci-runs-json` are already accepted (as no-op reserved flags) by `check-release-consistency.sh`, so plan 20-02 can give them behavior without changing the CLI's shape.
- `tests/scripts/check-release-consistency_test.sh`'s `write_metadata_fixture` helper and `assert_fire`/`assert_silent` pair are reusable as-is for any new clause's fixture cases.
- No blockers identified for downstream plans in this phase.

---
*Phase: 20-release-pipeline-recovery-idempotent-re-runs-and-a-pre-publi*
*Completed: 2026-08-28*

## Self-Check: PASSED

- FOUND: scripts/check-release-consistency.sh
- FOUND: tests/scripts/check-release-consistency_test.sh
- FOUND: 50bb5340 (RED test commit)
- FOUND: de9b4773 (GREEN feat commit)
- FOUND: b5bda678 (harness-hardening commit)
