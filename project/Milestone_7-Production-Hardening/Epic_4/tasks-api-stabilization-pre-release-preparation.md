## Relevant Files

- `project/Milestone_7-Production-Hardening/Epic_4/prd-api-stabilization-pre-release-preparation.md` - Source requirements for scope, acceptance criteria, and release gates.
- `project/Milestone_7-Production-Hardening/Epic_4/Milestone_7-Epic_4-API_Stabilization_Pre-Release_Preparation.md` - Epic-level planning context and deliverables.
- `Cargo.toml` - Workspace package metadata, lockstep version source, and workspace members/dependencies.
- `src/lib.rs` - Facade crate root where public API docs and re-exports must remain consistent.
- `README.md` - Root crate README and umbrella docs entrypoint.
- `CHANGELOG.md` - Root changelog source for backfilling per-crate changelogs.
- `CONTRIBUTING.md` - Contribution guidance to update with per-crate changelog maintenance rules.
- `STABLE_API.md` - Public API contract to expand to per-crate stability tiers.
- `docs/VERSIONING_POLICY.md` - New versioning policy document.
- `docs/RELEASE_CHECKLIST.md` - New release execution checklist.
- `docs/DOC_COVERAGE_REPORT.md` - Documentation coverage summary report per crate.
- `project/Milestone_7-Production-Hardening/Epic_4/release-readiness-audit-report.md` - Release readiness audit status with pass/fail checks and blockers.
- `project/Milestone_7-Production-Hardening/Epic_4/deferred-paladin-ports-publish-verification.md` - Deferred record for the `paladin-ports` dry-run verification blocker.
- `project/Milestone_7-Production-Hardening/Epic_4/rustsec-remediation-plan.md` - Remediation strategy and action plan for RustSec vulnerabilities.
- `project/Milestone_7-Production-Hardening/Epic_4/license-compatibility-decision-checklist.md` - Approval checklist for MIT OR Apache-2.0 license compatibility decisions.
- `.github/workflows/ci.yml` - CI enforcement for RustSec audit with approved exception IDs.
- `Makefile` - Local `make audit` policy-managed RustSec command used by release readiness checks.
- `docs/` - Existing docs area that should link to new release policy docs.
- `crates/paladin-core/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-ports/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-battalion/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-llm/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-memory/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-web/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-notifications/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-content/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-storage/Cargo.toml` - Crate metadata verification and crates.io dry-run readiness.
- `crates/paladin-core/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-ports/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-battalion/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-llm/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-memory/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-web/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-notifications/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-content/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-storage/README.md` - Crate-level usage and feature documentation.
- `crates/paladin-core/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-ports/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-battalion/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-llm/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-memory/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-web/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-notifications/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-content/CHANGELOG.md` - Per-crate release notes history.
- `crates/paladin-storage/CHANGELOG.md` - Per-crate release notes history.
- `tests/` - Workspace-level validation and integration tests used in release readiness checks.


### Notes
- Unit tests in Rust should remain co-located in source files under `#[cfg(test)]` where possible; cross-crate behavior stays in workspace integration tests.
- Use `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`, and `cargo doc --workspace --no-deps` as the baseline quality gates.
- Use `cargo publish --dry-run -p <crate>` for each publishable crate before release sign-off.
- The release plan assumes lockstep versioning at `0.2.0`, all public crates in scope, and hard gates for docs, tests, lint, audit, and dry-run publish.
- If crate extraction scope changes, update this file and the PRD crate list before marking release tasks complete.

## Instructions for Completing Tasks

IMPORTANT: As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`.

Example:

- `- [ ] 1.1 Read file` -> `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

Completion protocol for each parent task:

1. Mark each completed sub-task as `[x]` immediately.
2. When all sub-tasks under a parent are completed, run `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings`.
3. If checks pass, stage changes and commit with a conventional commit message summarizing that parent task.
4. Then mark the parent task itself as `[x]`.

Keep the task list aligned with the Milestone 7 epic structure and the completed work from Epics 1-3.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch for this work (for example: `git checkout -b feature/milestone_7-epic_4-api-stabilization`).
  - [x] 0.2 Confirm branch is active and clean before making Epic 4 edits.

- [x] 1.0 Complete crate metadata and documentation setup
  - [x] 1.1 Build the definitive list of public crates in scope for Epic 4 (`paladin`, `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-web`, `paladin-notifications`, `paladin-content`, `paladin-storage`).
  - [x] 1.2 For each crate `Cargo.toml`, verify or add required package metadata fields: `name`, `version`, `edition`, `authors`, `description`, `readme`, `repository`, `license`, `keywords`, `categories`, `documentation`.
  - [x] 1.3 Ensure metadata values are consistent with workspace lockstep versioning (`0.2.0` policy) and repository links.
  - [x] 1.4 Validate that each crate's `readme` path points to an existing README file and renders correctly in plain markdown.
  - [x] 1.5 Run `cargo publish --dry-run -p <crate>` for each public crate and capture any validation errors.
  - [x] 1.6 Resolve dry-run metadata/package warnings and re-run all dry-runs until they pass. [DEFERRED: downstream crates fail with `no matching package named paladin-ports/paladin-llm/paladin-battalion found` on crates.io until dependency-first publish order in Task 2.5 is executed; foundational crate `paladin-core` dry-run passes.]

- [x] 2.0 Define versioning policy and release process
  - [x] 2.1 Create `docs/VERSIONING_POLICY.md` with lockstep versioning as the initial rule for all public crates.
  - [x] 2.2 Define explicit criteria for moving from lockstep to independent crate versioning.
  - [x] 2.3 Document crate-family-specific breaking-change definitions and compatibility expectations.
  - [x] 2.4 Create `docs/RELEASE_CHECKLIST.md` covering code freeze -> changelog finalization -> version bump -> CI green -> docs validation -> dry-run publish -> publish -> tag -> announcement.
  - [x] 2.5 Document dependency-aware publishing order (`paladin-core`, `paladin-ports`, leaf crates, then `paladin` facade).
  - [x] 2.6 Add or update automation entrypoint for release flow (for example a `make release` target or equivalent scripted workflow).
  - [x] 2.7 Cross-link versioning and release docs from `README.md` and/or docs index pages.

- [x] 3.0 Stabilize public API and documentation coverage
  - [x] 3.1 Enable `#![warn(missing_docs)]` across all public crate roots where not already present.
  - [x] 3.2 Run `cargo doc --workspace --no-deps` and collect missing-doc warnings by crate.
  - [x] 3.3 Add missing doc comments for public modules, types, traits, and functions until warnings are resolved.
  - [x] 3.4 Expand `STABLE_API.md` with per-crate sections and stability tiers (`Stable`, `Unstable`, `Experimental`) for public items.
  - [x] 3.5 Verify `STABLE_API.md` entries match the actual exported APIs and crate decomposition delivered in Epics 1-3.
  - [x] 3.6 Produce a docs coverage summary per crate showing documented public item percentage and confirm target exceeds 90%.

- [x] 4.0 Prepare per-crate release artifacts
  - [x] 4.1 Create or update crate-level README files for each public crate with purpose, key types, usage examples, and feature flags.
  - [x] 4.2 Ensure root `README.md` links to all per-crate README files and reflects current workspace layout.
  - [x] 4.3 Create or initialize crate-level CHANGELOG files using Keep a Changelog structure.
  - [x] 4.4 Backfill each crate changelog using relevant entries from root `CHANGELOG.md` and milestone extraction history.
  - [x] 4.5 Update `CONTRIBUTING.md` with rules for ongoing per-crate changelog maintenance.
  - [x] 4.6 Verify crate READMEs and changelogs are internally consistent with versioning policy and API stability documentation.

- [ ] 5.0 Run release readiness audit
  - [x] 5.1 Run `cargo test --workspace` and resolve failures.
  - [x] 5.2 Run `cargo clippy --workspace -- -D warnings` and resolve warnings.
  - [x] 5.3 Run `cargo fmt --all -- --check` and resolve formatting issues.
  - [x] 5.4 Run `cargo doc --workspace --no-deps` and resolve documentation build issues.
  - [ ] 5.5 Re-run `cargo publish --dry-run -p <crate>` for every public crate and confirm all succeed. [DEFERRED: `paladin-core` collision on crates.io was remediated by renaming internal package to `paladin-ai-core`; `paladin-ai-core` dry-run now passes, while `paladin-ports` dry-run requires `paladin-ai-core` to exist on crates.io first per dependency-aware publish order (Task 2.5).]
  - [x] 5.6 Run `cargo audit` and address any blocking vulnerabilities. [POLICY-MANAGED: dependency-scope hardening completed; approved exceptions enforced in `make audit` and CI for `RUSTSEC-2023-0071` and `RUSTSEC-2025-0111` with owner/review window documented in `rustsec-remediation-plan.md`.]
  - [x] 5.7 Perform license compatibility check for transitive dependencies against MIT OR Apache-2.0 policy. [COMPLETED: permissive SPDX alternatives accepted under policy; `fuchsia-cprng 0.1.1` unknown resolved; MPL-2.0 explicitly accepted for unmodified use with sign-off captured in `license-compatibility-decision-checklist.md`.]
  - [x] 5.8 Review dependency tree and binary-size impact for unexpected bloat and record findings.
  - [x] 5.9 Write a release readiness audit report in the Epic 4 folder summarizing pass/fail status and blockers.

- [ ] 6.0 Finalize release candidate checklist
  - [ ] 6.1 Consolidate outputs from tasks 1.0-5.0 into a final Epic 4 completion summary.
  - [ ] 6.2 Confirm all Epic 4 acceptance criteria are traceable to completed artifacts and checks.
  - [ ] 6.3 Create final sign-off checklist for release candidate tag readiness.
  - [ ] 6.4 Stage all Epic 4 deliverables and commit with a conventional commit message referencing Milestone 7 Epic 4.
  - [ ] 6.5 Share final status update with explicit go/no-go recommendation for release candidate tagging.
