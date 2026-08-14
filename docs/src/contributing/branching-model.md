# Branching Model

Paladin follows a trunk-based flow: `main` receives every change through a pull request, feature
branches are short-lived, and releases are tags cut from `main` rather than a staging branch —
a release branch only exists afterward, to backport a fix into a line that has already shipped.
This page is written for contributors; see [Branch Protection](../appendix/branch-protection.md)
for the administrator-facing enforcement detail behind the checks this page describes.

## Quick Reference

```bash
# Start work
git checkout main && git pull --ff-only origin main
git checkout -b feature/short-description

# Open a PR when ready — required checks must pass before it can merge.
gh pr create --base main

# Cut a release (from an up-to-date main only)
make release VERSION=x.y.z
```

## Starting a branch

Branch off an up-to-date `main`. The conventional prefixes are `feature/`, `fix/`, `docs/`,
`chore/`, and `release/` for a backport branch cut from a published line — but this convention is
**not** enforced by CI. It is a documented human convention only. The repository's own history is
the standing evidence for why a naming guard was considered and declined: the remote carries 30
branches under `feature/` alongside a single bare `feat/`, and a prior incident (`fix/ci-workflow-health`,
itself a `fix/**` branch) ran with no CI coverage at all for two weeks because the trigger surface
at the time enumerated a handful of sanctioned prefixes rather than matching every branch. A naming
guard would have required guessing the next prefix someone invents; instead the trigger surface
(below) matches every branch unconditionally, and naming stays advisory.

## What runs when

Every workflow that takes a `push` trigger matches `push: branches: ['**']` — every branch runs CI,
with two deliberate, recorded exceptions. The table below is the trigger-policy register: one row
per workflow file in `.github/workflows/`, parsed by `scripts/check-workflow-triggers.sh` in CI, so
a workflow added without a row, or a branch filter narrowed back from the match-all pattern, fails
a required check instead of merging unnoticed.

**Table formatting constraint:** the guard parses this table with a line-based reader that splits
each row on `|` — keep it a plain pipe-delimited table with one row per file, no merged cells, and
no multi-line cells, or the guard's parser breaks on the next edit.

| Workflow | Triggers | Push branch filter | Rationale |
|----------|----------|---------------------|-----------|
| `ci.yml` | `push`, `pull_request`, `workflow_dispatch` | `['**']` | Core gate (lint, security audit, license/dependency policy, unit tests, examples, crate isolation, integration tests, coverage, CLI snapshots, API surface, benchmark compile check). Runs on every branch push under D-03 so no branch is ever silently uncovered; absorbed `integration-tests.yml`'s jobs and its nightly cron was retired rather than relocated — the broad integration suite now runs on every push to every branch, which is strictly more coverage than a once-daily run, so no `schedule:` key was added here. Revisit condition for reinstating a narrower schedule: if floating service-container image tags (`redis:7-alpine`, `minio/minio:latest`) need drift detection between pushes, a scheduled workflow targeting only those two tags would be the reinstatement, not resurrecting the deleted file. |
| `feature-flags.yml` | `push`, `pull_request`, `workflow_dispatch` | `['**']` | The 14-job feature matrix. Match-all push filter for the same reason as `ci.yml`: a maintained prefix allowlist goes dark the moment an unsanctioned prefix is used, which is exactly how this matrix went unexercised on six branch prefixes for two weeks. |
| `pre-commit.yml` | `push`, `pull_request` | `['**']` | Runs the version-controlled pre-commit hook suite as a required gate. A PR-only trigger would leave a branch with no CI until a PR opens, which is how eight broken action references survived undetected; match-all push closes that gap the same way it does for `ci.yml`. |
| `docs.yml` | `push`, `pull_request` | `[main]` (deliberate exception) | The `deploy` job publishes the mdBook site to GitHub Pages and must run only when documentation lands on `main`, not on every feature-branch push. The `pull_request` trigger (path-filtered to `docs/**`, `crates/doc-examples/**`, and the workflow file itself) still builds and link-checks the book on every PR regardless of branch. |
| `release.yml` | `push` (tags only), `workflow_dispatch` | not applicable — tag-triggered by design (deliberate exception) | Releases are cut from tags (`v*.*.*`), never from a branch push. `verify-tag-source` additionally confirms the tagged commit is an ancestor of `main` before anything publishes. |
| `benchmarks.yml` | `schedule`, `workflow_dispatch` | not applicable — declares no push trigger at all | Weekly Monday 06:00 UTC cadence for the long-running benchmark suite. Deliberately its own file rather than a `schedule:` key on `ci.yml`, because only a handful of `ci.yml`'s jobs carry conditional gates and a cron there would trigger the entire pipeline weekly, including the hour-plus multi-architecture Docker build. |

## How a change reaches `main`

`main` is protected: a pull request is mandatory, and every required status check must pass before
the merge button is available. The ruleset sets `required_approving_review_count: 0` — no second
human approval is required, because the repository currently has a single active committer and
GitHub does not allow self-approval — but the pull request itself, and every required check passing
against it, stay mandatory regardless. Force-pushes and branch deletion are blocked on `main`. If
the project gains a second active committer, revisit the approval count; the pull-request and
required-checks requirements do not change either way.

## Cutting a release

Releases are tags, not branches: `make release VERSION=x.y.z` from an up-to-date `main` creates and
pushes a `v*.*.*` tag, which triggers `release.yml`. See
[Branch Protection](../appendix/branch-protection.md) for the full enforcement detail — the
`verify-tag-source` guard, the release-tag ruleset, and the administrator steps for applying or
auditing the rulesets that back this page's description.
