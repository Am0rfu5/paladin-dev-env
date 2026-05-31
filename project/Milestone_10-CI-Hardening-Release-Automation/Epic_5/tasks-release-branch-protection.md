# Milestone 10 — Epic 5: Release Branch Protection — Task List

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 5 of 5
**PRD:** `project/Milestone_10-CI-Hardening-Release-Automation/Epic_5/prd-milestone10-release-branch-protection.md`
**Target Version:** No version bump (process/infrastructure hardening)
**Status:** Complete

---

## Relevant Files

| File | Purpose |
|------|---------|
| `.github/workflows/release.yml` | Add `verify-tag-source` guard job; gate `test` + `create-release` on it |
| `Makefile` | Add main-branch / up-to-date guard to the `release` target |
| `.github/rulesets/protect-main-branch.json` | Importable branch ruleset protecting `main` |
| `.github/rulesets/protect-release-tags.json` | Importable tag ruleset restricting `v*` tag creation |
| `docs/BRANCH_PROTECTION.md` | Policy rationale, enforcement layers, ruleset import instructions |
| `CONTRIBUTING.md` | `## Releasing` section updated with main-only policy + cross-link |
| `CHANGELOG.md` | `[Unreleased]` entries for Epic 5 |

### Notes

- This epic introduces **no Rust code** and **no version bump**. Conformance gates are still run to
  confirm no regressions in the Makefile/workflow/doc changes.
- The CI guard (`verify-tag-source`) is the authoritative enforcement; the Makefile guard is local
  fast-feedback; the rulesets are platform defense-in-depth applied manually by an admin.
- Because this work is on a feature branch, running `make release` here must **fail** the new
  guard — that is the intended self-validation.

## Instructions for Completing Tasks

As each sub-task is completed, change `- [ ]` to `- [x]`. Update the file after each sub-task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout `feature/milestone_10-epic_5-release-branch-protection` from the
        current branch.

- [x] 1.0 Add CI guard job `verify-tag-source` to `release.yml`
  - [x] 1.1 Add a `verify-tag-source` job that checks out with `fetch-depth: 0`.
  - [x] 1.2 Resolve the release commit: `github.sha` for tag pushes; the commit `inputs.tag` points
        to for `workflow_dispatch`.
  - [x] 1.3 Fetch `origin/main` and assert the release commit is an ancestor via
        `git merge-base --is-ancestor`; emit a `::error::` annotation and exit non-zero on failure.
  - [x] 1.4 Add `needs: verify-tag-source` to the `test` and `create-release` jobs.
  - [x] 1.5 Validate the workflow YAML parses (lint / `python -c yaml.safe_load`).

- [x] 2.0 Add local guard to the `make release` target
  - [x] 2.1 Before `release-check`, assert current branch is `main` (unless
        `RELEASE_ALLOW_ANY_BRANCH=1`, which warns and continues).
  - [x] 2.2 Fetch `origin/main` and fail if local `HEAD` is behind `origin/main`.
  - [x] 2.3 Ensure both checks run before any version bump / commit / tag (fail-fast ordering).
  - [x] 2.4 Verify `make release` (without `RELEASE_ALLOW_ANY_BRANCH`) fails on the current feature
        branch with the expected message (self-validation; do not actually release).

- [x] 3.0 Add importable GitHub ruleset definitions
  - [x] 3.1 Create `.github/rulesets/protect-main-branch.json` (PR required, required status checks,
        block force-push and deletion) targeting `main`.
  - [x] 3.2 Create `.github/rulesets/protect-release-tags.json` targeting `refs/tags/v*` restricting
        creation/deletion to bypass actors.
  - [x] 3.3 Validate both files parse as JSON (`python -m json.tool`).

- [x] 4.0 Write `docs/BRANCH_PROTECTION.md`
  - [x] 4.1 Document the main-only policy and the `v0.4.0`-from-feature-branch rationale.
  - [x] 4.2 Describe the three enforcement layers and how they relate.
  - [x] 4.3 Provide ruleset import steps (GitHub UI + `gh api`) for both files.
  - [x] 4.4 Document the correct release flow and the `RELEASE_ALLOW_ANY_BRANCH` override.

- [x] 5.0 Update `CONTRIBUTING.md` and `CHANGELOG.md`
  - [x] 5.1 Update `## Releasing` to state releases are cut only from an up-to-date `main` and
        cross-link `docs/BRANCH_PROTECTION.md`.
  - [x] 5.2 Add Epic 5 entries to `CHANGELOG.md` `[Unreleased]`.

- [x] 6.0 Conformance gate and task file completion
  - [x] 6.1 Run `cargo fmt --check` — must pass.
  - [x] 6.2 Run `cargo clippy --workspace --all-targets -- -D warnings` — must pass.
  - [x] 6.3 Run `cargo test --workspace` — must pass.
  - [x] 6.4 Run `cargo audit` — must exit 0.
  - [x] 6.5 Run `pre-commit run --all-files` — must pass (YAML/TOML/JSON/whitespace hooks cover the
        new files).
  - [x] 6.6 Mark all subtasks `[x]`, commit, and push the feature branch.

---

## Definition of Done

- [x] `release.yml` has a `verify-tag-source` guard that gates `test` and `create-release`; a tag
      not contained in `main` fails the pipeline before publishing.
- [x] `make release` fails fast on a non-`main` or behind-`main` working tree, with a documented
      override.
- [x] `.github/rulesets/protect-main-branch.json` and `protect-release-tags.json` exist and are
      valid JSON.
- [x] `docs/BRANCH_PROTECTION.md` documents the policy, layers, and import steps.
- [x] `CONTRIBUTING.md` `## Releasing` reflects the main-only policy.
- [x] `CHANGELOG.md` `[Unreleased]` records Epic 5.
- [x] All conformance gates pass: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`,
      `cargo audit`.
