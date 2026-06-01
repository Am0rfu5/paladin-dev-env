# Release Automation

This document records the evaluation of workspace release tooling for the Paladin framework, the
selected tool, and the operator guide for cutting a release. It is part of **Milestone 10 — CI
Hardening and Release Automation, Epic 3**.

## Tooling Evaluation: `cargo-release` vs. `release-plz`

| Dimension | `cargo-release` | `release-plz` |
|-----------|-----------------|---------------|
| Trigger model | Manual, developer-invoked command (`cargo release`) | PR-bot: opens/maintains a "release PR" automatically from `main` |
| Changelog handling | Works with a curated `CHANGELOG.md`; can run hooks to edit it | Auto-generates changelog from Conventional Commits |
| Workspace publish order | Built-in: publishes members in dependency order, supports lockstep or independent versions | Built-in: computes order, also opinionated about per-crate versioning |
| Version bumping | Bumps `[package].version` + internal `workspace.dependencies` pins in lockstep | Bumps versions per-crate based on detected changes |
| Required secrets / infra | `CARGO_REGISTRY_TOKEN` for publish; no bot, no extra app | `CARGO_REGISTRY_TOKEN` **plus** a GitHub token/app for the release-PR bot |
| Operational model | Fits an existing tag-triggered pipeline: bump+tag locally, CI publishes on the tag | Replaces the manual flow with a continuously-updated release PR |
| Maintenance cost | Low: one config file (`release.toml`), no running bot | Higher: bot behavior, PR hygiene, commit-message discipline enforced |
| Fit with current practice | High — matches curated `CHANGELOG.md`, lockstep `0.3.0`-everywhere, and `release.yml` `v*.*.*` trigger | Lower — requires moving to Conventional-Commit-driven changelog + PR-bot workflow |

### Recommendation & Decision: **`cargo-release`**

`cargo-release` is selected. The Paladin repository already has:

- a **curated `CHANGELOG.md`** with a `## [Unreleased]` section (we want to keep authoring it, not
  auto-generate it),
- **lockstep versioning** (every public crate is `0.3.0`; `docs/RELEASE_CHECKLIST.md` mandates a
  "lockstep version update across public crates"), and
- a **tag-triggered pipeline** (`.github/workflows/release.yml` already fires on `v*.*.*`).

`cargo-release` slots directly into this model: a maintainer runs a single command (wrapped by
`make release VERSION=x.y.z`) that bumps all crates in lockstep, finalizes the changelog, commits,
tags `v x.y.z`, and pushes. The push triggers CI, which publishes the crates to crates.io in
dependency order. No PR-bot, no GitHub App, and no change to the curated-changelog or
Conventional-Commit practice is required.

`release-plz` is a strong tool but optimizes for a different workflow (PR-bot + auto-changelog +
per-crate version detection) that would be a larger process change for marginal benefit here. It can
be revisited if the project later adopts strict Conventional Commits and prefers a continuous
release-PR model.

## Reproducible Installation

`cargo-release` is installed the same way locally and in CI, pinned and `--locked`:

```bash
cargo install cargo-release --locked
```

(The CI publish job installs it with `--locked` so the build is reproducible from `Cargo.lock`.)

## Release Configuration (`release.toml`)

The repo-root `release.toml` encodes:

- **Lockstep versioning** — `shared-version = true` so all publishable crates move to the same
  version in one bump, and the internal `workspace.dependencies` pins are updated to match.
- **Dependency-ordered publishing** — `cargo-release` publishes workspace members in topological
  dependency order: `paladin-core` → `paladin-ports` → the leaf tier (`paladin-battalion`,
  `paladin-llm`, `paladin-memory`, `paladin-web`, `paladin-notifications`, `paladin-content`,
  `paladin-storage`) → `paladin` (facade).
- **Tag/commit conventions** — a single workspace tag `v{{version}}` is created (the
  `.github/workflows/release.yml` pipeline keys off `v*.*.*`).

## Canonical Publish Order

Per Milestone 7 Appendix B, publishable crates are released dependency-first:

1. `paladin-core` (package name `paladin-ai-core`)
2. `paladin-ports`
3. `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-web`,
   `paladin-notifications`, `paladin-content`, `paladin-storage` (parallel-safe tier)
4. `paladin` (facade, package name `paladin-ai`)
5. `paladin-cli` (only when/if it exists as a separate publishable crate)

## Operator Guide: Cutting a Release

A release is cut **locally** with a single command; CI does the publishing.

```bash
# 1. Ensure you are on the release branch with a clean tree and up-to-date CHANGELOG [Unreleased].
# 2. Cut the release (bumps all crates in lockstep, finalizes changelog, commits, tags, pushes):
make release VERSION=0.4.0
```

`make release`:

1. Validates `VERSION` is a valid semver string (fails fast otherwise).
2. Runs `make release-check` (format, lint, full tests, audit, release build).
3. Bumps every public crate to `VERSION` in lockstep and updates internal dependency pins.
4. Moves the `## [Unreleased]` changelog section under a `## [VERSION] - <date>` heading.
5. Commits, creates the `v VERSION` tag, and pushes branch + tag.

Pushing the `v*.*.*` tag triggers `.github/workflows/release.yml`, which runs the test suite and then
**publishes the crates to crates.io in dependency order**, builds Docker images and binaries,
generates the SBOM, and creates the GitHub release.

### Required Secret

crates.io publishing requires a repository secret:

- `CARGO_REGISTRY_TOKEN` — a crates.io API token with publish scope.

If the secret is absent, the publish job is **skipped** (the rest of the release still runs), so the
pipeline can be exercised safely before the token is configured.

### Dry Run (no live publish)

To exercise the pipeline without publishing to crates.io, trigger the workflow manually with the
`dry_run` input set to `true`:

```bash
gh workflow run release.yml -f tag=v0.4.0-rc.1 -f dry_run=true
```

In dry-run mode the publish job runs `cargo publish --dry-run` for each crate in order instead of a
real publish. Locally, the same validation is available via:

```bash
make publish-dry-run
```
