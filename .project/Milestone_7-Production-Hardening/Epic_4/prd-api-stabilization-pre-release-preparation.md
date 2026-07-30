# PRD: API Stabilization and Pre-Release Preparation

**Feature Name:** api-stabilization-pre-release-preparation
**Milestone:** 7 - Production Hardening and Extended Workspace Decomposition
**Epic:** 4 - API Stabilization and Pre-Release Preparation
**Status:** Ready for Implementation
**Created:** 2026-05-27
**Author:** AI-assisted, reviewed by team

## 1. Introduction / Overview

Milestones 1-3 established the workspace structure, extracted the core crates, and hardened the architectural boundaries. Milestone 7 Epics 1-3 extend that work by extracting the remaining infrastructure crates, adapting the build and test tooling, and migrating benchmarks into the workspace structure.

Epic 4 prepares the Paladin crate ecosystem for its first publishable release. The release target is a lockstep pre-1.0 version, anchored to `0.2.0`, with all public crates versioned together until the project has enough stability to consider independent versioning. The release posture is production-minded: every public crate must be documented, metadata-complete, auditable, and ready for a crates.io dry run.

The problem this epic solves is that the workspace can be structurally correct but still not release-ready. Without this epic, consumers may encounter incomplete crate metadata, undocumented APIs, unclear versioning rules, or missing release procedures. The goal is to turn the workspace into a coherent, publishable crate ecosystem with a defined API contract and a repeatable release process.

## 2. Goals

1. Finalize crates.io-ready metadata for every public crate in the workspace.
2. Ensure every public crate has a crate-level `README.md` and `CHANGELOG.md`.
3. Enforce documentation coverage expectations for all public types, traits, functions, and modules.
4. Publish a clear versioning policy that starts with lockstep versioning at `0.2.0`.
5. Define a release checklist that covers code freeze, documentation, validation, dry-run publishing, tagging, and announcement.
6. Extend `STABLE_API.md` so each crate has a documented public API boundary and stability tier.
7. Verify the workspace is actually release-ready by passing test, lint, format, documentation, security, and publish checks.

## 3. User Stories

As a downstream consumer, I want each Paladin crate to have complete metadata and documentation so that I can evaluate and depend on only the crates I need.

As a maintainer, I want a stable release process with a clear publishing order so that version bumps and publish steps do not break dependency relationships.

As a reviewer, I want the public API to be documented per crate with stability labels so that I can tell which items are safe to rely on and which remain experimental.

As a release engineer, I want dry-run publishes, docs, tests, and audit checks to act as hard gates so that a release candidate cannot be tagged before the workspace is ready.

As a contributor, I want per-crate READMEs and changelogs so that I can understand a crate's purpose and history without reading the whole repository.

## 4. Functional Requirements

### 4.1 Crate Metadata Completion

4.1.1 Every public crate must have a complete `[package]` section in `Cargo.toml` with at least `name`, `version`, `edition`, `authors`, `description`, `readme`, `repository`, `license`, `keywords`, `categories`, and `documentation`.

4.1.2 The metadata must match the workspace versioning policy and use the lockstep workspace version for the initial release series.

4.1.3 Each crate must be able to pass `cargo publish --dry-run -p <crate>` without crates.io validation errors.

4.1.4 Publishable crates must have accurate crate ownership boundaries reflected in their dependencies so that metadata and dependency declarations agree.

### 4.2 Per-Crate README Files

4.2.1 Every public crate must have a crate-level `README.md`.

4.2.2 Each README must explain the crate's purpose, the problem it solves, the main public types, the feature flags it exposes, and how it relates to the rest of the workspace.

4.2.3 Each README must include enough information for crates.io rendering to be useful to downstream consumers.

4.2.4 The root `README.md` must act as the umbrella overview and link to the individual crate READMEs.

### 4.3 Per-Crate CHANGELOG Files

4.3.1 Every public crate must have a crate-level `CHANGELOG.md` using Keep a Changelog conventions.

4.3.2 The crate changelog must reflect the crate's own history, including the extraction or stabilization history relevant to that crate.

4.3.3 The contribution guidance must explain how changelog entries are maintained for future releases.

### 4.4 Documentation Coverage Audit

4.4.1 All public crates must enable `#![warn(missing_docs)]`.

4.4.2 Public items must have documentation comments unless explicitly exempted by the codebase convention.

4.4.3 `cargo doc --workspace --no-deps` must complete without documentation warnings.

4.4.4 A documentation coverage audit must be produced that shows the documented-public-item percentage per crate, with the target exceeding 90%.

4.4.5 Public API documentation must be consistent with the existing `STABLE_API.md` contract and the crate READMEs.

### 4.5 Versioning Policy and Release Process

4.5.1 A versioning policy document must define lockstep versioning as the default policy for the first release line.

4.5.2 The policy must state the criteria for transitioning to independent per-crate versioning later.

4.5.3 The policy must define what counts as a breaking change for each crate family.

4.5.4 A release checklist must describe the full release path from code freeze to publish and announcement.

4.5.5 The release checklist must include the following steps in order: code freeze, changelog finalization, version bump, CI green, documentation validation, dry-run publish, publish, tag, and announcement.

4.5.6 Publishing order must be dependency-aware and documented as: `paladin-core` first, then `paladin-ports`, then leaf crates, and finally the `paladin` facade.

4.5.7 The release process must be scripted or represented by an equivalent workspace command such as a `make release` target.

### 4.6 STABLE_API.md Per-Crate Stabilization

4.6.1 `STABLE_API.md` must be expanded to document the public API surface of each crate individually.

4.6.2 Every public type and trait must have a stability tier: Stable, Unstable, or Experimental.

4.6.3 Cross-crate dependency contracts must be documented so consumers understand which crates are safe to rely on together.

4.6.4 The API documentation must reflect the actual workspace decomposition completed in Epics 1-3.

### 4.7 Release Readiness Audit

4.7.1 The workspace must pass `cargo test --workspace`.

4.7.2 The workspace must pass `cargo clippy --workspace -- -D warnings`.

4.7.3 The workspace must pass `cargo fmt --all -- --check`.

4.7.4 The workspace must pass `cargo doc --workspace --no-deps`.

4.7.5 Every publishable crate must pass `cargo publish --dry-run`.

4.7.6 A security audit must be performed with `cargo audit`.

4.7.7 License compatibility must be verified so that dependencies remain compatible with the project's MIT licensing posture.

4.7.8 The release readiness audit must review dependency tree and binary size for unexpected bloat before the release candidate tag is approved.

## 5. Non-Goals / Out of Scope

1. Actual crates.io publishing is out of scope for this epic; this epic prepares the workspace for publishing and validates it with dry runs.
2. New product features, new orchestration patterns, and new LLM providers are out of scope.
3. Additional performance work beyond release readiness checks is out of scope.
4. Kubernetes or infrastructure deployment changes are out of scope.
5. Independent per-crate versioning is not the initial release policy and should not be implemented prematurely.

## 6. Design Considerations

The documentation should reflect the medieval military naming convention already used in the codebase, but the external presentation must remain clear for crates.io consumers who may not know the internal naming scheme.

Crate READMEs should prioritize short examples, dependency guidance, and feature-flag explanation over internal architecture prose.

The versioning policy should be explicit enough that contributors can determine whether a change is breaking without needing release-engineering context.

## 7. Technical Considerations

The workspace is expected to contain multiple publishable crates after Epics 1-3. Epic 4 must treat the public crate list as a first-class release artifact rather than assuming the monolithic facade is the only publishable unit.

Documentation coverage should be checked against the actual public surface after extraction, not against idealized API lists.

Lockstep versioning at `0.2.0` reduces dependency coordination risk while the crate set is still settling. Independent versioning should only be introduced when the public surfaces are stable enough to justify the added release complexity.

Dry-run publish failures should be treated as release blockers, because they usually indicate metadata or dependency problems that downstream consumers will also encounter.

The release checklist should be concise, repeatable, and unambiguous so it can be followed by both maintainers and automation.

## 8. Success Metrics

1. Every public crate passes crates.io dry-run validation.
2. Every public crate has a README and CHANGELOG.
3. Public API documentation coverage exceeds 90% across the workspace.
4. `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`, and `cargo doc --workspace --no-deps` all succeed.
5. `cargo audit` reports no blocking security issues for the release candidate.
6. `STABLE_API.md` fully covers the public crates with stability tiers.
7. The release process is reproducible and documented well enough for a maintainer to execute without guesswork.

## 9. Open Questions

1. None at this time for the initial release policy: the PRD assumes lockstep versioning at `0.2.0`, hard release gates, and all public crates in scope.
2. If the crate set changes again before release, the crate list in the metadata and documentation tasks should be updated to match the new workspace state.
