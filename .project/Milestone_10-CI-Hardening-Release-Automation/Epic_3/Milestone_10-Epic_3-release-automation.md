## Epic 3: Release Automation

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 3 of 4
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
