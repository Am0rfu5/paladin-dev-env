# Milestone 9 — Epic 6: Finalization and Release

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 6 of 6
**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epics 1–5
**Version Target:** v0.3.0
**Status:** Planning

---

## Objective

Bring the milestone to a releasable state: pass the full quality gate across the workspace, update
the changelog, bump the version, and tag a v0.3.0 release candidate.

## Background

Epics 1–5 deliver the functional work of Milestone 9. This Epic is the finalization checkpoint that
ensures the workspace builds cleanly, all tests pass, documentation builds, and the release artifacts
(changelog, version, tag) are produced.

## Scope

**In scope:**
- Full workspace quality gate (build, test, clippy, fmt, doc).
- `CHANGELOG.md` updates summarizing all Epic deliverables.
- Workspace version bump to `0.3.0`.
- Tagging the v0.3.0 release candidate.

**Out of scope:**
- Any new feature work (belongs to Epics 1–5).
- Publishing to crates.io or external registries (handled by the release-automation milestone).

---

## Tasks

### Task 6.1: Full Quality Gate

**Description:** Run the complete workspace quality gate and resolve any failures before proceeding
to versioning.

**Commands:**
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo doc --workspace --no-deps`

**Deliverables:**
- All quality-gate commands pass with no errors or warnings.

**Acceptance criteria:**
- Each command above exits successfully in a clean checkout.
- No clippy warnings, no formatting diffs, and docs build without warnings.

---

### Task 6.2: CHANGELOG and Version Bump

**Description:** Update the changelog and version metadata, then tag the release candidate.

**Steps:**
- Update `CHANGELOG.md` with all Epic deliverables (Epics 1–5), grouped by feature area.
- Bump the workspace version to `0.3.0` (workspace `Cargo.toml` and any per-crate versions that
  track the workspace version).
- Tag a `v0.3.0` release candidate.

**Deliverables:**
- Updated `CHANGELOG.md`.
- Workspace version bumped to `0.3.0`.
- `v0.3.0` release-candidate tag.

**Acceptance criteria:**
- `CHANGELOG.md` reflects every Epic's user-visible deliverables.
- The workspace version is `0.3.0` and the project builds at that version.
- A release-candidate tag exists for `v0.3.0`.

---

## Definition of Done

- The full quality gate passes across the workspace.
- `CHANGELOG.md` is updated and the version is bumped to `0.3.0`.
- A `v0.3.0` release-candidate tag is created.
