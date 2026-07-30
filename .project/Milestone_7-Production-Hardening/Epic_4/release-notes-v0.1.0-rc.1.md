# Release Notes: v0.1.0-rc.1

Date: 2026-05-28
Tag: v0.1.0-rc.1
Commit: a9530fc

## Overview

This release candidate finalizes Milestone 7 Epic 4 (API Stabilization and Pre-Release Preparation) and publishes the decomposed Paladin crate family to crates.io with dependency-order validation complete.

## Published Crates

- paladin-ai-core 0.1.0
- paladin-ports 0.1.0
- paladin-battalion 0.1.0
- paladin-llm 0.1.0
- paladin-memory 0.1.0
- paladin-storage 0.1.0
- paladin-notifications 0.1.0
- paladin-content 0.1.0
- paladin-web 0.1.0
- paladin-ai 0.1.0

## Highlights

- Resolved crates.io package-name collision for the core crate by renaming package to `paladin-ai-core`.
- Resolved root facade package-name conflict by renaming package to `paladin-ai` while preserving library import path compatibility.
- Closed previously deferred publish-verification blocker for `paladin-ports`.
- Completed full release readiness gates:
  - cargo test --workspace
  - cargo fmt --all -- --check
  - cargo clippy --workspace -- -D warnings
  - cargo doc --workspace --no-deps
  - cargo publish --dry-run for all public crates
- Finalized Epic 4 completion and sign-off artifacts with GO recommendation.

## Key Commits

- a9530fc docs: complete Milestone 7 Epic 4 release closure
- cb9a5eb fix: stabilize settings config tests and close Task 5 audit
- 957c3a4 refactor: rename root package to paladin-ai
- 687ca1f refactor: rename core package to paladin-ai-core
- 8bbdea7 fix: resolve crates.io paladin-core collision

## Notes

- crates.io publish rate limits were encountered during sequential publication and were handled with retry at provider-specified windows.
- Security audit is policy-managed with documented exceptions in CI and local audit flows.

## Recommendation

GO for release candidate adoption and downstream integration testing.
