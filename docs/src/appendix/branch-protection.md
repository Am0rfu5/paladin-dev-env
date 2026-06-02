# Branch & Release-Tag Protection

This document describes the **main-only release policy** for the Paladin Framework and the three
layers that enforce it. It also gives administrators step-by-step instructions for applying the
committed GitHub ruleset definitions.

> **Policy in one sentence:** release tags (`v*.*.*`) may only be created from commits that are
> contained in the `main` branch. `main` is the single source of truth for released code.

---

## Why this policy exists

Milestone 10 Epic 3 made releases fully tag-driven: pushing a `v*.*.*` tag triggers
[`.github/workflows/release.yml`](https://github.com/DF3NDR/paladin-dev-env/tree/main/.github/workflows), which runs the test suite,
publishes crates to crates.io, builds Docker images and binaries, and generates an SBOM.

When the first release (`v0.4.0`, Epic 4) was cut, the tag was pushed from a **feature branch** that
had not yet been merged into `main`. The pipeline only keyed off the tag, not the branch, so it would
have published code that never passed through the reviewed `main` branch. Epic 5 closes that gap.

---

## The three enforcement layers

| Layer | Where | What it enforces | Authoritative? |
|-------|-------|------------------|----------------|
| 1. CI guard | `verify-tag-source` job in `release.yml` | The tagged commit is an ancestor of `origin/main`; otherwise the whole pipeline fails before publishing. | **Yes** |
| 2. Local guard | `make release` target in `Makefile` | Refuses to bump/tag unless on an up-to-date `main`. Fast feedback before any push. | No (advisory) |
| 3. Platform rulesets | `.github/rulesets/*.json` (applied by an admin) | PR + passing checks required to land on `main`; only authorized actors may create `v*` tags. | Defense in depth |

### Layer 1 — CI guard (`verify-tag-source`)

The release workflow's first job resolves the release commit (`github.sha` for a tag push, or the
commit the dispatched `inputs.tag` points to) and runs:

```bash
git merge-base --is-ancestor "$RELEASE_SHA" origin/main
```

If the commit is **not** contained in `main`, the job emits a `::error::` annotation and exits
non-zero. The `test` and `create-release` jobs declare `needs: verify-tag-source`, so a failed guard
prevents publishing, Docker, binaries, and SBOM from running. This layer is authoritative because it
cannot be bypassed locally.

### Layer 2 — Local guard (`make release`)

Before bumping versions or tagging, `make release`:

1. Checks the current branch is `main`.
2. Fetches `origin/main` and fails if local `HEAD` is behind it.

Both checks run **before** any destructive action, so a wrong-branch release stops immediately with
no version bump, commit, or tag.

**Emergency override (hotfix branches only):**

```bash
RELEASE_ALLOW_ANY_BRANCH=1 make release VERSION=0.4.1
```

This bypasses **only** the branch-name check (the up-to-date check still runs). The CI guard (Layer 1)
remains authoritative — an override here does not let an unmerged commit publish from CI.

### Layer 3 — GitHub rulesets

Two importable ruleset definitions live in [`.github/rulesets/`](https://github.com/DF3NDR/paladin-dev-env/tree/main/.github/rulesets):

- `protect-main-branch.json` — requires a pull request and passing status checks (`Code Quality`,
  `Security Audit`, `License & Dependency Policy`) to merge into `main`, and blocks force-pushes and
  branch deletion.
- `protect-release-tags.json` — restricts **creation** and deletion of `refs/tags/v*` to bypass
  actors (repository admins), so arbitrary contributors cannot cut releases.

> GitHub tag rulesets govern *who* may create a tag matching a pattern — they cannot express
> "the tag must come from main". The branch-source rule is therefore enforced by Layer 1; the tag
> ruleset is complementary who-can-tag protection.

---

## Applying the rulesets (administrators)

Rulesets require repository-admin scope and are applied manually (they are intentionally **not**
self-applied from CI).

### Option A — GitHub UI

1. Go to **Settings → Rules → Rulesets → New ruleset → Import a ruleset**.
2. Upload `.github/rulesets/protect-main-branch.json`. Review the targets and status-check contexts,
   then **Create**.
3. Repeat for `.github/rulesets/protect-release-tags.json`.

### Option B — `gh` CLI

```bash
# Requires admin scope on the repository.
gh api --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/DF3NDR/paladin-dev-env/rulesets \
  --input .github/rulesets/protect-main-branch.json

gh api --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/DF3NDR/paladin-dev-env/rulesets \
  --input .github/rulesets/protect-release-tags.json
```

Verify the active rulesets:

```bash
gh api /repos/DF3NDR/paladin-dev-env/rulesets
```

> The `bypass_actors` entry uses `actor_id: 5` (`RepositoryRole` = Admin). Adjust the role id or add
> team/app actors to match your organization before importing.

---

## The correct release flow under this policy

```bash
# 1. Open a PR for your changes and get it merged into main (checks must pass).
# 2. Update your local main.
git checkout main
git pull --ff-only origin main

# 3. Cut the release from main.
make release VERSION=0.4.1
```

Pushing the resulting `v0.4.1` tag triggers `release.yml`; `verify-tag-source` confirms the tagged
commit is in `main`, and the pipeline proceeds to publish.

---

## Reconciling the existing `v0.4.0` tag

`v0.4.0` was cut from `feature/milestone_10-epic_4-finalization` before this policy existed. To make
`main` reflect the released code, a maintainer should merge that branch (and the subsequent Epic 5
work) into `main` via PR. This is a one-time reconciliation and is not performed automatically by the
Epic 5 changes.

---

## Related documents

- [docs/RELEASE_AUTOMATION.md](release-automation.md) — release tooling decision and operator guide.
- [docs/RELEASE_CHECKLIST.md](release-checklist.md) — manual release checklist.
- [CONTRIBUTING.md](../contributing/development-setup.md) — `## Releasing` section.
