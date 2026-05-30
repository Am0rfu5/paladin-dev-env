# Milestone 10: CI Hardening and Release Automation

**Project:** Paladin Framework
**Milestone:** 10 — Pre-commit Hooks, Security Scanning, and Release Pipeline
**Version Target:** v0.4.0
**Status:** Planning
**Created:** 2026-05-29
**Document Version:** 1.0

---

## Executive Summary

The project has been developed through seven refactoring milestones and two product milestones without formalized commit-time quality gates, dependency security scanning, or automated release tooling. This Milestone introduces the CI hardening and release automation infrastructure needed for a production-grade open-source project.

### Success Criteria

- Pre-commit hooks enforce formatting, linting, and secrets detection on every commit.
- Pre-push hooks run a fast test subset before code reaches the remote.
- CI pipeline includes dependency vulnerability scanning on every PR.
- License compliance is verified automatically.
- A `make release` (or equivalent) command can cut a workspace release with correct crate publishing order.
- SBOM is generated for each release.

---

## Parallel Execution Context

This Milestone has **no hard dependencies on Milestones 8, 9, or 11**. It can begin immediately and run fully in parallel. The only soft dependency is that the release automation (Epic 3) benefits from having the workspace structure stabilized by Milestone 8, so Epic 3 should not be finalized until Milestone 8 is complete.

---

## Epic 1: Pre-commit and Pre-push Hooks

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** None

### Objective

Install and configure commit-time quality gates that prevent malformed code, secrets, and formatting violations from entering the repository.

### Tasks

#### Task 1.1: Select and Configure Hook Framework

**Description:** Evaluate `pre-commit` (Python-based, language-agnostic) vs. `cargo-husky` (Rust-native, simpler). Recommend `pre-commit` for its broader ecosystem (secrets detection, YAML/TOML validation, file size checks) with Rust-specific hooks for `cargo fmt` and `cargo clippy`.

**Deliverables:**
- `.pre-commit-config.yaml` in repository root.
- `pre-commit` installation documented in `CONTRIBUTING.md`.

#### Task 1.2: Configure Pre-commit Hooks

**Description:** Configure the following hooks:

| Hook | Tool | Purpose |
|------|------|---------|
| Formatting | `cargo fmt --all -- --check` | Enforce consistent code style |
| Linting | `cargo clippy --workspace -- -D warnings` | Catch common mistakes |
| Secrets detection | `detect-secrets` or `gitleaks` | Prevent API keys, passwords in commits |
| TOML validation | `check-toml` | Catch `Cargo.toml` syntax errors |
| YAML validation | `check-yaml` | Catch `config.yml` syntax errors |
| File size | `check-added-large-files` (1MB limit) | Prevent accidental binary commits |
| Merge conflict markers | `check-merge-conflict` | Catch unresolved conflicts |
| Trailing whitespace | `trailing-whitespace-fixer` | Clean whitespace |
| EOF newline | `end-of-file-fixer` | Consistent file endings |

**Deliverables:**
- All hooks configured and tested.
- CI step that runs `pre-commit run --all-files` as a verification gate.

#### Task 1.3: Configure Pre-push Hooks

**Description:** Pre-push hooks run before `git push` and should execute a fast quality check:
- `cargo build --workspace` (catches compilation errors).
- `cargo test --workspace --lib` (unit tests only — fast subset, skips integration tests).

**Deliverables:**
- Pre-push hook configured.
- Documented override: `git push --no-verify` for emergencies.

---

## Epic 2: Dependency Security and License Compliance

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** None

### Objective

Add automated scanning for known vulnerabilities in dependencies and verify license compatibility across the dependency tree.

### Tasks

#### Task 2.1: Integrate `cargo audit`

**Description:** `cargo audit` checks the RustSec Advisory Database for known vulnerabilities in `Cargo.lock` dependencies. Add it to the CI pipeline as a required check.

**Deliverables:**
- `cargo audit` added to CI pipeline.
- Runs on every PR and on `main` branch pushes.
- Fails the build on known vulnerabilities (with a documented exception process for false positives or unpatched advisories).

#### Task 2.2: Integrate OSV-Scanner

**Description:** OSV-Scanner (Google's Open Source Vulnerability scanner) provides broader coverage than RustSec alone. Add it as a supplementary scanner.

**Deliverables:**
- OSV-Scanner added to CI pipeline.
- Configured to scan `Cargo.lock`.
- Results reported as PR annotations.

#### Task 2.3: Evaluate and Optionally Integrate Snyk

**Description:** Snyk provides commercial-grade vulnerability scanning with deeper analysis. Evaluate whether the free tier provides value beyond `cargo audit` + OSV-Scanner. If so, integrate; if not, document the decision and skip.

**Deliverables:**
- Evaluation document.
- Integration if warranted, or documented deferral.

#### Task 2.4: Configure `cargo deny`

**Description:** `cargo deny` enforces license compliance, bans specific crates, and detects duplicate dependency versions. Configure a `deny.toml` with:
- **Licenses:** Allow MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib. Deny copyleft (GPL, AGPL, LGPL) unless explicitly approved.
- **Bans:** No banned crates initially; add as needed.
- **Duplicates:** Warn on duplicate crate versions (different major versions of the same crate in the tree).

**Deliverables:**
- `deny.toml` in repository root.
- `cargo deny check` added to CI pipeline.
- All current dependencies pass.

#### Task 2.5: SBOM Generation

**Description:** Generate a Software Bill of Materials for each release. Use `cargo cyclonedx` or `cargo sbom` to produce CycloneDX or SPDX format.

**Deliverables:**
- SBOM generation added to the release pipeline.
- SBOM artifact attached to GitHub releases.

---

## Epic 3: Release Automation

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Milestone 8 (workspace structure stabilized)

### Objective

Automate the release process: version bump, changelog finalization, workspace crate publishing in dependency order, Git tag, and GitHub release creation.

### Tasks

#### Task 3.1: Evaluate Release Tooling

**Description:** Evaluate `cargo-release` vs. `release-plz` for workspace release automation:
- `cargo-release`: Manual trigger, configurable per-crate, handles version bumps and `cargo publish`.
- `release-plz`: PR-based workflow, auto-generates changelog from conventional commits, handles workspace ordering.

**Deliverables:**
- Evaluation document with recommendation.
- Selected tool.

#### Task 3.2: Configure Workspace Publishing Order

**Description:** Configure the release tool to publish crates in dependency order (as documented in Milestone 7 Appendix B):
1. `paladin-core`
2. `paladin-ports`
3. `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-web`, `paladin-notifications`, `paladin-content`, `paladin-storage` (parallel-safe)
4. `paladin` (facade)
5. `paladin-cli`

**Deliverables:**
- Release configuration file specifying publishing order.
- Dry-run publish succeeds for all crates.

#### Task 3.3: Configure Tag-Triggered Release Pipeline

**Description:** Create a CI workflow triggered by Git tags matching `v*.*.*` that:
1. Runs the full test suite.
2. Publishes all crates to crates.io in order.
3. Builds Docker images.
4. Creates a GitHub release with changelog and SBOM.

**Deliverables:**
- CI workflow file (GitHub Actions).
- Tag-triggered pipeline tested with a dry-run tag.

#### Task 3.4: Create `make release` Target

**Description:** Add a Makefile target that orchestrates the local release process: version bump → changelog update → commit → tag → push (which triggers the CI pipeline).

**Deliverables:**
- `make release VERSION=0.4.0` target.
- Documented in `CONTRIBUTING.md`.

---

## Epic 4: Finalization

**Priority:** Medium
**Estimated Effort:** Small
**Dependencies:** Epics 1–3

### Tasks

#### Task 4.1: Update CONTRIBUTING.md

**Description:** Document:
- Pre-commit hook installation instructions.
- How to run security scans locally.
- Release process step-by-step.
- How to add a new dependency (license check, audit check).

**Deliverables:**
- Updated `CONTRIBUTING.md`.

#### Task 4.2: CHANGELOG and Version Bump

- Update `CHANGELOG.md`.
- Bump to v0.4.0.
- Tag release.

---

## Schedule Overview

| Phase | Epic | Duration | Predecessors |
|-------|------|----------|-------------|
| Phase 1A | Epic 1: Pre-commit/Pre-push | 0.5–1 sprint | None |
| Phase 1B | Epic 2: Security/License Scanning | 1–1.5 sprints | None |
| Phase 2 | Epic 3: Release Automation | 1–1.5 sprints | Milestone 8 (soft) |
| Phase 3 | Epic 4: Finalize | 0.5 sprint | Epics 1–3 |

**Total: 2–3 sprints** (Epics 1 and 2 fully parallelizable)
