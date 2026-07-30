# PRD: Milestone 10 — Epic 5: Release Branch Protection (Tag-from-Main Enforcement)

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 5 of 5
**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epic 3 (release automation), Epic 4 (v0.4.0 cut)
**Target Version:** No version bump (infrastructure/process hardening)

---

## 1. Introduction / Overview

Milestone 10 Epic 3 made releases fully tag-driven: pushing a `v*.*.*` tag triggers
`.github/workflows/release.yml`, which tests, publishes crates to crates.io, builds Docker images
and binaries, and generates an SBOM. Epic 4 cut the first such release (`v0.4.0`).

That release exposed a gap: the `v0.4.0` tag was created and pushed from a **feature branch**
(`feature/milestone_10-epic_4-finalization`) that had not been merged into `main`. Because the
release pipeline only keys off the tag — not the branch the tag points at — a release can be cut
from code that never passed through `main`. This breaks the GitFlow invariant that **`main` is the
single source of truth for released code**.

Epic 5 closes this gap by enforcing, at multiple layers, that **release tags may only be created
from commits that are contained in `main`**:

1. **CI enforcement (authoritative):** a guard job in `release.yml` that fails the entire release
   pipeline if the tagged commit is not an ancestor of `origin/main`.
2. **Local enforcement (fast feedback):** a guard in the `make release` target that refuses to cut
   a release unless the working tree is on an up-to-date `main`.
3. **Platform enforcement (defense in depth):** committed GitHub ruleset definitions that protect
   the `main` branch and restrict who may create `v*` release tags, with import instructions.
4. **Documentation:** the main-only release policy is documented in `CONTRIBUTING.md` and a new
   `docs/BRANCH_PROTECTION.md`.

This is a process- and infrastructure-hardening epic. It introduces **no library code** and
**no version bump**.

---

## 2. Goals

1. A release pipeline run for a tag whose commit is **not** in `main` fails fast with a clear error,
   before any crate is published.
2. `make release` refuses to run from any branch other than an up-to-date `main`, with a documented
   emergency override.
3. The repository ships importable GitHub ruleset definitions that (a) protect `main` (require PRs +
   passing checks) and (b) restrict creation of `v*` tags to authorized actors.
4. The main-only release policy is clearly documented for contributors and maintainers.
5. All existing CI/CD behavior for legitimate releases (tag cut from `main`) is unchanged.

---

## 3. User Stories

- **As a maintainer**, I want the release pipeline to reject any tag that was not cut from `main`,
  so that nothing reaches crates.io unless it passed through the reviewed `main` branch.

- **As a release engineer**, I want `make release` to stop me immediately if I am on the wrong branch
  or behind `origin/main`, so that I never accidentally publish unreviewed code.

- **As a repository administrator**, I want ready-to-import ruleset files for branch and tag
  protection, so that I can enforce the policy at the platform level without hand-crafting JSON.

- **As a new contributor**, I want the release policy documented in one place, so that I understand
  why I cannot tag a release from my feature branch.

---

## 4. Functional Requirements

### FR 1 — CI guard job: `verify-tag-source`

`release.yml` must gain a job that runs before any release work and blocks the pipeline if the
release commit is not contained in `main`.

1. The job checks out the repository with full history (`fetch-depth: 0`).
2. It resolves the commit under release:
   - For a `push` tag event, the commit is `github.sha`.
   - For a `workflow_dispatch` event, the commit is the one the supplied `inputs.tag` points to.
3. It fetches `origin/main` and verifies the release commit is an ancestor of (i.e., reachable
   from) `origin/main` using `git merge-base --is-ancestor`.
4. If the commit is **not** contained in `main`, the job prints a GitHub `::error::` annotation and
   exits non-zero.
5. The `test` and `create-release` jobs (the two roots that all other release jobs depend on) must
   declare `needs: verify-tag-source`, so a failed guard prevents publishing, Docker, binaries, and
   SBOM steps from running.

### FR 2 — Local guard in `make release`

The `release` Makefile target must verify, before bumping versions or tagging, that:

1. The current branch is `main` (`git rev-parse --abbrev-ref HEAD` equals `main`).
2. `origin/main` has been fetched and the local `HEAD` is not behind `origin/main`
   (`git rev-list HEAD..origin/main` is empty).
3. If either check fails, the target prints a clear red error and exits non-zero **before** any
   destructive action (version bump, commit, tag).
4. An explicit, documented escape hatch `RELEASE_ALLOW_ANY_BRANCH=1` bypasses **only** the
   branch-name check (for rare hotfix/maintenance-branch releases) while still printing a warning.
   The CI guard (FR 1) remains the authoritative gate.

### FR 3 — Committed GitHub ruleset definitions

The repository must ship importable ruleset JSON under `.github/rulesets/`:

1. `protect-main-branch.json` — a branch ruleset targeting `main` (and optionally `develop`) that:
   - Requires a pull request before merging.
   - Requires status checks to pass (the CI `lint`, `security-audit`, and `cargo-deny` jobs).
   - Blocks force-pushes and branch deletion.
2. `protect-release-tags.json` — a tag ruleset targeting `refs/tags/v*` that restricts tag creation
   and deletion to bypass actors (repository admins / maintainers), preventing arbitrary
   contributors from cutting releases.
3. Each file must be valid JSON importable via the GitHub UI (Settings → Rules → Rulesets → Import)
   or `gh api`.

### FR 4 — `docs/BRANCH_PROTECTION.md`

A new document must explain:

1. The main-only release policy and **why** it exists (the `v0.4.0`-from-feature-branch incident).
2. The three enforcement layers (CI guard, Makefile guard, GitHub rulesets) and how they relate.
3. Step-by-step instructions for an administrator to import each ruleset file (UI and `gh api`).
4. The correct release flow under the policy: merge to `main` via PR → pull `main` locally →
   `make release VERSION=…` from `main`.
5. The documented emergency override (`RELEASE_ALLOW_ANY_BRANCH=1`) and when it is acceptable.

### FR 5 — `CONTRIBUTING.md` update

The `## Releasing` section must be updated to:

1. State that releases are cut **only from `main`**, after the release commit is merged via PR.
2. Cross-link `docs/BRANCH_PROTECTION.md`.
3. Update the "Cutting a release" steps so step 0 is "ensure you are on an up-to-date `main`".

### FR 6 — `CHANGELOG.md` update

Add entries under `## [Unreleased]` documenting the Epic 5 additions (CI guard job, Makefile guard,
ruleset definitions, `docs/BRANCH_PROTECTION.md`).

---

## 5. Non-Goals (Out of Scope)

- **No version bump / no new release tag.** Epic 5 hardens the process; it does not cut a release.
  (Cutting a tag from this feature branch would, correctly, now be blocked by the new guards.)
- **No automated application of GitHub rulesets.** The repository ships importable definitions and
  instructions; an administrator applies them manually (rulesets require repo-admin scope and cannot
  be safely self-applied from CI).
- **No changes to the publish order, Docker, binary, or SBOM jobs** beyond adding the `needs`
  dependency on the guard.
- **No rewrite of the existing `v0.4.0` tag/release.** Reconciling `main` with the released code is
  a maintainer merge action, noted in docs but not performed by this epic's code changes.
- **No library/runtime code changes.**

---

## 6. Technical Considerations

- **`git merge-base --is-ancestor <sha> origin/main`** is the precise primitive for "is this commit
  contained in main". It exits 0 when `<sha>` is an ancestor of `origin/main`, non-zero otherwise.
  Full history (`fetch-depth: 0`) is required for it to be reliable.
- **GitHub tag rulesets cannot express "tag must come from main"** — they govern *who* may create a
  tag matching a pattern, not the source branch. The branch-source rule is therefore enforced by the
  CI guard (FR 1), with the tag ruleset (FR 3.2) providing complementary who-can-tag protection.
- **Workflow_dispatch path:** the guard must resolve `inputs.tag` to a commit; for a tag-push event
  it uses `github.sha` directly.
- **Makefile guard ordering:** the branch/up-to-date checks must run before `make release-check` so
  the failure is immediate and no version bump occurs.
- **Self-validation:** because the current Epic 5 work happens on a feature branch, attempting
  `make release` here must fail the new guard — a positive confirmation the guard works.

---

## 7. Success Metrics

- A dry-run / simulated tag whose commit is not in `main` causes `verify-tag-source` to fail and
  blocks `create-release`, `publish-crates`, `build-docker`, `build-binaries`, and `sbom`.
- `make release VERSION=…` exits non-zero with a clear message when run from a non-`main` branch.
- Both ruleset JSON files parse as valid JSON and import without error.
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo audit` all pass
  (no regressions from the Makefile/workflow/doc changes).

---

## 8. Open Questions

1. Should the `protect-main-branch.json` ruleset also target `develop`, or only `main`? (Default:
   target `main` only; `develop` mentioned as an optional addition in the doc.)
2. Should the emergency override (`RELEASE_ALLOW_ANY_BRANCH=1`) also relax the CI guard via a
   `workflow_dispatch` input, or remain CI-authoritative with no override? (Default: CI remains
   authoritative — no CI override — to keep `main` the source of truth.)
