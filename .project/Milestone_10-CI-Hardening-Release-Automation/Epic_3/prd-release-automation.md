# PRD: Release Automation

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 3 of 4
**Status:** Draft
**Target Version:** v0.4.0

---

## 1. Introduction / Overview

Today, cutting a Paladin release is a manual, error-prone sequence: bump the version in every
crate `Cargo.toml`, finalize the `CHANGELOG.md`, publish nine workspace crates to crates.io in the
correct dependency order, create a Git tag, and push so that the existing
`.github/workflows/release.yml` tag pipeline builds Docker images, binaries, and the SBOM.

Every one of those steps is currently done by hand (or via a loose `make release` target that only
runs dry-run publishes). A single mistake — a forgotten crate bump, an out-of-order publish, a
mismatched tag — breaks the release or produces an inconsistent set of published crates.

This Epic automates the release process end-to-end: a single, reproducible local command bumps the
version in lockstep, finalizes the changelog, commits, tags, and pushes; the push triggers the
existing tag pipeline which (now extended) **publishes the workspace crates to crates.io in
dependency order** in addition to the Docker images, binaries, and SBOM it already produces.

The goal is a **one-command, repeatable, dependency-order-correct release** with a clear evaluation
of the chosen tooling.

## 2. Goals

1. Select a release-automation tool (`cargo-release` vs. `release-plz`) with a documented rationale.
2. Encode the canonical workspace publish order so crates are always published dependency-first.
3. Make `cargo publish --dry-run` succeed (in order) for every workspace crate, proving release
   readiness.
4. Extend the tag-triggered CI pipeline to publish crates to crates.io as part of a release.
5. Provide a `make release VERSION=x.y.z` target that performs the version bump → changelog
   finalize → commit → tag → push locally, triggering the pipeline.
6. Document the entire release flow in `CONTRIBUTING.md` and the existing release docs.

## 3. User Stories

- **As a maintainer**, I want to run a single `make release VERSION=0.4.0` command so that I can cut
  a consistent release without manually editing nine `Cargo.toml` files.
- **As a maintainer**, I want the version bumped in lockstep across all public crates so that
  inter-crate dependency versions never drift.
- **As a maintainer**, I want crates published in dependency order automatically so that a publish
  never fails because an upstream crate is not yet available on crates.io.
- **As a release reviewer**, I want a documented, evaluated choice of release tooling so that I
  understand why the pipeline works the way it does and how to operate it.
- **As a contributor**, I want the release process documented in `CONTRIBUTING.md` so that I can
  understand how versions and changelogs are managed.

## 4. Functional Requirements

### Tooling Evaluation & Selection (Task 3.1)

1. The Epic must produce a written evaluation comparing `cargo-release` and `release-plz` across at
   least: trigger model (manual vs. PR-bot), changelog handling, workspace publish-order support,
   required secrets/permissions, and operational/maintenance cost.
2. The evaluation must record an explicit recommendation and the selected tool, captured in a
   version-controlled document (`docs/RELEASE_AUTOMATION.md`).
3. The selected tool must be installable in CI via a pinned, `--locked` install (or a pinned action)
   and must be reproducible locally.

### Workspace Publish Order (Task 3.2)

4. The release configuration must encode the canonical publish order (per Milestone 7 Appendix B):
   1. `paladin-core`
   2. `paladin-ports`
   3. `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-web`, `paladin-notifications`,
      `paladin-content`, `paladin-storage` (parallel-safe tier)
   4. `paladin` (facade)
   5. `paladin-cli` (only if/when it exists as a separate publishable crate)
5. The configuration must support **lockstep versioning**: all public crates share one version
   number that is bumped together (consistent with the existing `0.3.0`-everywhere convention and
   `docs/RELEASE_CHECKLIST.md`).
6. A dependency-first `cargo publish --dry-run` must succeed for every workspace crate. Where an
   upstream crate is not yet on crates.io, the dry-run ordering and expected-failure behavior must be
   documented (not treated as a hard failure for first-publish crates).
7. Any crate that must **not** be published (e.g., internal-only or example crates) must be
   explicitly marked `publish = false`; publishable crates must have complete required metadata
   (`description`, `license`, `repository`).

### Tag-Triggered Release Pipeline (Task 3.3)

8. The release pipeline must be triggered by Git tags matching `v*.*.*` (extend the existing
   `.github/workflows/release.yml`, which already has this trigger).
9. The pipeline must run the full test suite before any publish/release step (a release must not
   proceed if tests fail).
10. The pipeline must publish all publishable crates to crates.io **in dependency order**, gated on
    the test suite passing, using a `CARGO_REGISTRY_TOKEN` (or `CRATES_IO_TOKEN`) repository secret.
11. The publish job must be safe to re-run: re-publishing an already-published version must not fail
    the whole pipeline (idempotent / tolerant of "already uploaded").
12. The pipeline must continue to build Docker images, build binaries, generate the SBOM, and create
    the GitHub release with the changelog (existing behavior must be preserved).
13. The publish job must only run for real releases (not for dry-run/test tags) — there must be a
    documented way to test the pipeline without publishing to crates.io (e.g., a dry-run path or a
    pre-release/`workflow_dispatch` mode that skips the actual publish).
14. The workflow YAML must be valid (pass `pre-commit run check-yaml`).

### `make release` Target (Task 3.4)

15. A `make release VERSION=x.y.z` target must orchestrate the local release: bump the version in
    lockstep across all public crates, finalize the changelog (move `Unreleased` → the new version),
    commit, create the `v x.y.z` tag, and push (the push triggers the CI pipeline).
16. The target must fail fast with a clear error if `VERSION` is not supplied or is not a valid
    semver string.
17. The target must run release-readiness checks (or reuse the existing `release-check`) before
    tagging, so a broken tree is never tagged and pushed.
18. The existing `make release` behavior (dry-run publishes) must be preserved under a clearly named
    target (e.g., `make publish-dry-run`) so no existing capability is lost.
19. The new release flow must be documented in `CONTRIBUTING.md` (and cross-referenced from
    `docs/RELEASE_CHECKLIST.md` / `docs/RELEASE_AUTOMATION.md`), including required secrets and how to
    perform a dry run.

### Conformance (all tasks)

20. `cargo build`, `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` must all pass
    after the changes. All new workflow/TOML/YAML must pass the repo's `pre-commit` hooks.

## 5. Non-Goals (Out of Scope)

- Actually publishing a real release to crates.io as part of this Epic (the machinery is built and
  dry-run-validated; the first live publish is a separate operational action).
- Changing the versioning **policy** itself (lockstep vs. independent) — this Epic encodes the
  existing lockstep convention, it does not redesign it.
- Auto-generating the human-readable changelog body from commit messages beyond what the existing
  pipeline already does (the changelog remains curated; tooling may assist but does not replace it).
- Building or publishing a separate `paladin-cli` crate if one does not yet exist as an independent
  workspace member (the config must be ready for it, but creating the crate is out of scope).
- Signing artifacts / provenance attestation (potential future Epic).

## 6. Technical Considerations

- **Existing pipeline:** `.github/workflows/release.yml` already triggers on `v*.*.*` and
  `workflow_dispatch`, and has `create-release`, `build-docker`, `build-binaries`, and `sbom` jobs.
  The new publish job should slot in alongside these, gated on tests.
- **Versioning today:** every crate hardcodes `version = "0.3.0"` (no `[workspace.package]`
  inheritance). `workspace.dependencies` pins each internal crate to `0.3.0` with a `path`. A
  lockstep bump must update both the per-crate `[package].version` and the `workspace.dependencies`
  version pins. `cargo-release` handles this via its workspace version-bump support.
- **Tool recommendation (to be confirmed in Task 3.1):** `cargo-release` is the likely fit — it is
  manual-trigger (aligns with the existing tag-based pipeline and curated `CHANGELOG.md`), supports
  workspace lockstep bumps and dependency-ordered publishing, and needs no external bot. `release-plz`
  favors a PR-bot/auto-changelog model that is a larger change to current practice.
- **Secrets:** crates.io publishing requires a `CARGO_REGISTRY_TOKEN` repo secret. The PRD assumes a
  maintainer will add it; the pipeline must degrade gracefully / be documented when it is absent.
- **Naming collision:** the current `make release` target runs dry-run publishes. It must be renamed
  (e.g., `publish-dry-run`) so the new `make release VERSION=` can own the `release` name (FR 18).
- **Hexagonal architecture:** this Epic is CI/build/release tooling only — no `src/` domain code
  changes are expected.

## 7. Success Metrics

- A maintainer can cut a release with a single `make release VERSION=x.y.z` command.
- `cargo publish --dry-run` succeeds (in order) for all workspace crates locally and in CI.
- The tag pipeline publishes crates in dependency order on a `v*.*.*` tag, with tests gating publish.
- Zero manual per-crate `Cargo.toml` edits are required to bump a version.
- The release tooling choice and flow are documented and discoverable from `CONTRIBUTING.md`.

## 8. Open Questions (with recommendations)

1. **Which release tool?** — *Recommendation: `cargo-release`.* It matches the existing manual,
   tag-triggered, curated-changelog workflow and supports lockstep workspace bumps + ordered
   publishing without introducing a PR bot. `release-plz` would be a larger process change.
2. **Should CI auto-publish on every `v*.*.*` tag, or require manual approval?** — *Recommendation:*
   auto-publish gated on the test suite, but guard the publish job so it is skipped when
   `CARGO_REGISTRY_TOKEN` is absent and provide a `workflow_dispatch` dry-run path (FR 13), so the
   pipeline is testable without a live publish.
3. **How is the changelog finalized?** — *Recommendation:* keep the curated `CHANGELOG.md` and have
   `make release` move the `## [Unreleased]` section under the new version heading with the date;
   the existing pipeline's commit-derived notes remain the GitHub release body.
4. **Lockstep vs. independent crate versions?** — *Recommendation:* keep **lockstep** (all crates
   share one version), matching the current `0.3.0`-everywhere state and `docs/RELEASE_CHECKLIST.md`.
