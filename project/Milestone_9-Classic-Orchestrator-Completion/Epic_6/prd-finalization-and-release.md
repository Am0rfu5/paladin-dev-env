# PRD: Milestone 9 — Epic 6: Finalization and Release

**Project:** Paladin Framework
**Milestone:** 9 — Classic Orchestrator, Content Pipeline, and Agent-Orchestrator Bridge
**Epic:** 6 of 6
**Version Target:** v0.3.0
**Source Epic:** `Milestone_9-Epic_6-finalization-and-release.md`
**Status:** Ready for Implementation

---

## 1. Introduction/Overview

Epics 1–5 of Milestone 9 delivered the functional work of the milestone: a working
`Orchestrator` workflow-execution loop (Epic 1), validated scheduler and queue paths (Epic 2),
the content → agent bridge (Epic 3), the agent → orchestrator bridge (Epic 4), and the user/admin
system completion with authentication and RBAC (Epic 5).

This Epic is the **finalization checkpoint**. It does not add features. Its purpose is to bring the
workspace to a releasable state by (a) running the complete quality gate across the whole workspace
and fixing any failures, (b) recording every Epic's user-visible deliverables in `CHANGELOG.md`,
(c) bumping the workspace version to `0.3.0`, and (d) tagging a `v0.3.0` release candidate.

The problem this solves: without a deliberate finalization step, the milestone's work is spread
across feature branches with no single verified, versioned, documented artifact that downstream
consumers can depend on. This Epic produces that artifact.

## 2. Goals

1. The entire workspace builds, tests, lints, formats, and documents cleanly in a single pass.
2. `CHANGELOG.md` accurately summarizes the user-visible deliverables of Epics 1–5.
3. The workspace version is consistently `0.3.0` across the root crate and all member crates.
4. A `v0.3.0` release-candidate tag exists and the project builds at that version.
5. No new feature work is introduced; changes are limited to fixes required by the quality gate,
   documentation, and version metadata.

## 3. User Stories

- **As a framework maintainer**, I want a single command sequence that proves the workspace is
  healthy, so that I can trust the release candidate is buildable and correct.
- **As a downstream developer**, I want a changelog that lists what changed in this milestone, so
  that I can understand the new capabilities (orchestrator execution, content/agent bridges, RBAC)
  before upgrading.
- **As a release manager**, I want the workspace version bumped and a `v0.3.0` tag created, so that
  I have an immutable reference point for the release candidate.

## 4. Functional Requirements

### Quality Gate (FR 1–6)

1. The system must build the full workspace with `cargo build --workspace` with no errors.
2. The system must pass `cargo test --workspace` with all tests green. Feature-gated paths
   exercised in earlier Epics (e.g. `redis-queue`, `web-server`) must be included in the validation,
   running them explicitly where `--workspace` alone does not enable the relevant feature.
3. The system must pass `cargo clippy --workspace -- -D warnings` (and, where earlier Epics relied
   on feature flags, `cargo clippy --workspace --all-features -- -D warnings`) with zero warnings.
4. The system must pass `cargo fmt --all -- --check` with no formatting diffs.
5. The system must build documentation with `cargo doc --workspace --no-deps` without warnings.
6. Any failures surfaced by FR 1–5 must be fixed within the bounds of this Epic (no feature work),
   and the failing command re-run until it passes.

### Changelog (FR 7–9)

7. `CHANGELOG.md` must contain a `0.3.0` release entry summarizing the deliverables of Epics 1–5,
   grouped by feature area (e.g. Orchestration, Scheduler/Queue, Content Pipeline, Agent Bridge,
   User/Admin & Security).
8. The changelog entry must follow the existing changelog format/style already present in the file.
9. The changelog must describe user-visible/behavioral changes, not internal commit-by-commit detail.

### Version Bump (FR 10–13)

10. The root crate version in `Cargo.toml` must be set to `0.3.0`.
11. Every workspace member crate that tracks the workspace version must be set to `0.3.0`.
12. All internal path-dependency version specifications in `[workspace.dependencies]` (and any
    per-crate dependency declarations) that pin the member crates must be updated to `0.3.0` so the
    workspace resolves consistently.
13. After the bump, `cargo build --workspace` must still succeed, and `Cargo.lock` must be updated to
    reflect the new versions.

### Release Tag (FR 14–15)

14. A `v0.3.0` release-candidate tag must be created on the finalized commit.
15. The tag must point at a commit where the full quality gate passes and the version is `0.3.0`.

## 5. Non-Goals (Out of Scope)

- Any new feature work (all features belong to Epics 1–5).
- Publishing to crates.io or any external registry.
- Changing the public API surface beyond what is strictly required to make the quality gate pass.
- Merging the milestone branch into `main`/`develop` (handled by the project's normal merge process,
  not this Epic).
- Reconciling whether the previous published version "should" have been `0.2.0`; this Epic targets
  `0.3.0` per the Epic specification regardless of intervening version numbers.

## 6. Technical Considerations

- **Branching:** Per the task workflow, create a feature branch
  `feature/milestone_9-epic_6-finalization-and-release` from the current branch, which already
  contains the commits for Epics 1–5.
- **Version coupling:** `[workspace.dependencies]` in the root `Cargo.toml` pins internal crates with
  explicit `version = "0.1.0"` specs (e.g. `paladin-core`, `paladin-ports`, `paladin-battalion`, …).
  These must be bumped in lock-step with the per-crate `version` fields, or `cargo` will fail to
  resolve the path dependencies. `paladin-core` uses `package = "paladin-ai-core"`.
- **Feature-gated validation:** Earlier Epics introduced/relied on feature flags (`redis-queue`,
  `web-server`, etc.). A bare `cargo test --workspace` may not compile those paths, so an
  `--all-features` (or targeted `--features`) pass is required to honor FR 2/FR 3. Integration tests
  that require external services (e.g. Redis via testcontainers/Docker) should be run where the
  environment allows and otherwise noted.
- **Security scanning:** Per repository instructions, run `snyk_code_scan` on any first-party code
  changed to satisfy the quality gate. If the tool is unavailable in the environment, substitute the
  strict clippy/compiler checks and record that substitution in the task close-out.
- **Disk space:** Full `--all-features` builds (which compile every example) have previously
  exhausted build disk; clear stale `target/` artifacts if needed rather than treating an
  out-of-space error as a code failure.
- **Doc build:** `cargo doc --workspace --no-deps` must be warning-free; missing-docs lints on public
  items introduced in earlier Epics may need doc comments added (documentation only, not feature work).

## 7. Success Metrics

- `cargo build --workspace`, `cargo test --workspace` (incl. feature paths), `cargo clippy
  --workspace -- -D warnings`, `cargo fmt --all -- --check`, and `cargo doc --workspace --no-deps`
  each exit `0` on a clean checkout.
- `grep` for `version` across the workspace shows `0.3.0` consistently for all member crates and the
  root crate, and internal dependency pins match.
- `git tag` lists `v0.3.0` (release candidate) pointing at the finalized commit.
- `CHANGELOG.md` has a `0.3.0` section covering all five Epics' deliverables.

## 8. Open Questions

- None blocking. The Epic specification is explicit on the version target (`0.3.0`) and the required
  artifacts. Any environment-specific limitations (Docker-backed integration tests, Snyk
  availability) will be handled per the Technical Considerations and recorded in the task close-out.

---

## Task Checklist (for the generated task list)

- [x] Full workspace quality gate passes (build, test incl. feature paths, clippy, fmt, doc).
- [x] `CHANGELOG.md` updated with a `0.3.0` entry covering Epics 1–5.
- [x] Workspace version bumped to `0.3.0` (root + all member crates + internal dependency pins).
- [x] `v0.3.0` release-candidate tag created on the finalized commit.
