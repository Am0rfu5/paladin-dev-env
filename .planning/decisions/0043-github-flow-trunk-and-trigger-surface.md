# ADR-0043: GitHub Flow trunk model and the trigger surface

## Status

Accepted

**Date:** 2026-08-14

## Context

Before this phase, five separate facts described a git and CI topology that had drifted out from
under itself, verified against the tree and the live GitHub API this session:

- `origin/main` sat **921 commits behind** `origin/release/v0.7.0` with nothing ahead of it
  (`git rev-list --count origin/main..origin/release/v0.7.0` → `921`; `... origin/release/v0.7.0..origin/main`
  → `0`) — the repository's default branch, not its integration branch, and hundreds of commits
  stale.
- `origin/develop` was a stale midpoint branch nobody used for integration — an ancestor of
  `origin/release/v0.7.0`, 151 commits ahead of `origin/main`, not a divergent line.
- `origin/release/v0.7.0`, a versioned branch, was doing the integration branch's job: every
  phase's work landed there via PR, not on `main`.
- `ci.yml`'s push branch filter named four prefixes — `[main, develop, 'feature/**', 'release/**']`
  — while `origin` carried eight distinct prefixes in use: `feature/` (30), `fix/` (2),
  `copilot/` (2), `release/` (1), `feat/` (1), `docs/` (1), `chore/` (1), `agent/` (1). The branch
  that fixed a two-week CI blind spot (`fix/ci-workflow-health`) was itself a `fix/**` branch,
  outside the filter, and received CI only once a PR existed.
- `ci.yml`'s `integration-tests` job and the now-deleted `integration-tests.yml`'s own
  `integration-tests` job both rendered under the identical GitHub status-check context
  **`Integration Tests`**; `ci.yml`'s `kubernetes-smoke` and `integration-tests.yml`'s
  `kubernetes-smoke-test` both rendered as **`Kubernetes Smoke Test`**. GitHub matches required
  status checks by context name, not job id — the same defect class as the two identically-named
  `Security Audit` jobs Milestone 10 shipped and Phase 12 closed.

## Decision

**`main` is the trunk under GitHub Flow.** Every change lands via pull request; `feature/**` and
`fix/**` branches are short-lived; releases are tags cut from `main`, never a staging branch;
`release/0.x` branches exist only to backport a fix into an already-published line. `develop` and
`release/v0.7.0` are retired once their content is on `main` (executed at plan `15.1-07`: `main`
fast-forwarded by 994 commits — the 921 from `release/v0.7.0` plus 73 more from this phase's own
waves 1-5 — to `d87d11ea2968ebc185afddf79a34cb0a200eff85`; both retired refs proven ancestors and
deleted from origin and locally, no archival tag needed).

**Every workflow with a `push` trigger uses the match-all filter `push: branches: ['**']`**, with
exactly two named, recorded exceptions: `docs.yml` keeps `push: [main]` because its `deploy` job
publishes to GitHub Pages and must not fire on every feature-branch push; `release.yml` stays
tag-triggered by design, gated by its own `verify-tag-source` ancestry check rather than a branch
filter. Branch **naming** stays a documented human convention (`docs/src/contributing/branching-model.md`),
deliberately not a CI gate.

**`integration-tests.yml` is deleted; its jobs are consolidated into `ci.yml`.** The broad
`--features integration-tests` suite, `docker-integration`, and one `Kubernetes Smoke Test` job now
live in `ci.yml` alone, resolving the context-name collision and — as a direct consequence of the
match-all filter — running the broad integration suite on every branch push instead of only on PR
and a daily cron.

**The trigger policy is recorded in a register and enforced by a guard, not left to convention
alone.** `docs/src/contributing/branching-model.md` carries a six-row table — one row per workflow
file, trigger types, push branch filter, and rationale — and `scripts/check-workflow-triggers.sh`
parses every `.github/workflows/*.yml` `on:` block against it in CI, failing on an uncovered
workflow, a drifted filter, or a required-check context that resolves to no declared job.

## Considered Options

- **An integration-branch model, keeping a dedicated branch between feature work and `main`** (rejected) — it makes the CI-exclusive suites (Docker, kind, Redis/MinIO integration) run *later* rather than earlier, and contradicts the invariant `verify-tag-source` and `make release` already enforce: that `main` is the single source of truth for released code.
- **Keeping `release/v0.7.0` specifically as the integration point** (rejected) — the same rejection as above, applied to the branch already doing the job; retaining it would also leave the version-in-branch-name mismatch (`release/v0.7.0` against manifests reading `0.8.0`) uncorrected.
- **Enumerating sanctioned branch-name prefixes across the workflow files** (rejected) — the repository's own history is the standing evidence against this: a maintained allowlist goes dark the moment an unsanctioned prefix is used, which is exactly how `fix/ci-workflow-health` ran with no push CI for two weeks while fixing CI itself.
- **A branch-naming CI gate, rejecting a push from an unrecognised prefix** (rejected) — offered and declined in favour of a documented human convention; a gate would have required guessing the next prefix someone invents, and match-all coverage makes the naming question moot for CI purposes.

## Code Locations

- `.github/workflows/ci.yml` — the consolidated workflow; `push: branches: ['**']`, `integration-tests`, `kubernetes-smoke`, and the `cargo-deny` job's "Check workflow trigger policy" / "Run shell-guard regression tests" steps
- `.github/workflows/feature-flags.yml` — `push: branches: ['**']`, the 14-job feature matrix
- `.github/workflows/pre-commit.yml` — `push: branches: ['**']`
- `.github/workflows/docs.yml` — the one deliberate push-filter exception, `push: [main]`
- `.github/workflows/release.yml` — the tag-triggered exception; `verify-tag-source`'s `git merge-base --is-ancestor` step
- `.github/workflows/benchmarks.yml` — `schedule` + `workflow_dispatch`, no push trigger at all
- `scripts/check-workflow-triggers.sh` — the coverage/drift/context guard, parsing the register and every workflow's `on:` block
- `tests/scripts/check-workflow-triggers_test.sh` — the guard's regression harness
- `Makefile` — the `check-workflow-triggers` target, wired into `check-gates` and `test-shell-guards`
- `docs/src/contributing/branching-model.md` — the trigger-policy register and the contributor-facing branching narrative
- `.github/workflows/integration-tests.yml` — deleted by plan `15.1-05`, absorbed into `ci.yml`

## Code Conformance

conforms

`origin/main` is confirmed fast-forwarded to `d87d11ea2968ebc185afddf79a34cb0a200eff85`
(`15.1-07-SUMMARY.md`: 994 commits ahead, zero behind, no merge commit, no force-push); `origin/develop`
and `origin/release/v0.7.0` are confirmed absent from origin and local (`git ls-remote --heads
origin` shows neither). `docs/src/contributing/branching-model.md` and
`scripts/check-workflow-triggers.sh` are confirmed shipped and passing against the current tree
(`15.1-09-SUMMARY.md`). No code change is made by this record; it ratifies work already executed by
plans `15.1-05`, `15.1-07` and `15.1-09`.

## Downstream Consumers

- **The trigger-policy register** (`docs/src/contributing/branching-model.md`'s table) — this
  record is the decision the register's rationale column summarises per row.
- **`scripts/check-workflow-triggers.sh`** — enforces this record's match-all-with-two-exceptions
  invariant mechanically in CI rather than by convention alone.
- **`release.yml`'s `verify-tag-source` ancestry guard** — its `git merge-base --is-ancestor`
  assertion against `origin/main` is now meaningful because `main` carries the shipped work this
  record's fast-forward moved onto it; before the move, the assertion was checking ancestry against
  a trunk hundreds of commits stale.
