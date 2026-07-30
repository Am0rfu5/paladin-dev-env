# PRD: Milestone 10 — Epic 4: Finalization

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 4 of 4
**Priority:** Medium
**Estimated Effort:** Small
**Dependencies:** Epics 1–3 (all completed)
**Target Version:** v0.4.0

---

## 1. Introduction / Overview

Milestone 10 introduced three major operational capabilities:

- **Epic 1** — Pre-commit / pre-push hook framework (cargo fmt, clippy, build, test, secret detection)
- **Epic 2** — Dependency security and license compliance (cargo-audit, cargo-deny, CycloneDX SBOM)
- **Epic 3** — Release automation (cargo-release, tag-triggered crates.io publish pipeline, `make release`)

Epic 4 closes the milestone by:

1. **Completing CONTRIBUTING.md** — adding the one section not yet written: a step-by-step guide
   for contributors on **how to add a new dependency** (license check, audit check, deny.toml
   update workflow). Sections for hooks, security scanning, and releasing were added in Epics
   1–3 and only need their Table-of-Contents entries verified.

2. **Populating and finalizing CHANGELOG.md** — writing the Milestone 10 entry (Epics 1–4) into
   the `[Unreleased]` block so it is ready for the version-finalization step.

3. **Cutting the v0.4.0 release** — bumping all crates lockstep to v0.4.0, finishing the
   CHANGELOG entry, committing, creating the `v0.4.0` annotated tag, and pushing. The
   tag-triggered CI pipeline (from Epic 3) then publishes to crates.io automatically.

---

## 2. Goals

1. Every contributor-facing workflow introduced in Milestone 10 is documented in CONTRIBUTING.md.
2. CHANGELOG.md accurately records all Milestone 10 work under `## [0.4.0]`.
3. All workspace crates are bumped lockstep from `0.3.0` → `0.4.0`.
4. A signed `v0.4.0` annotated tag exists on the branch and the CI release pipeline is triggered.
5. The repository passes `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and
   `cargo audit` at v0.4.0 before the tag is pushed.

---

## 3. User Stories

- **As a new contributor**, I want a clear step-by-step guide in CONTRIBUTING.md for adding a
  dependency so I know how to pick a crate, verify its license, run the security checks, and
  update `deny.toml` if needed — without having to read multiple separate documents.

- **As a maintainer**, I want CHANGELOG.md to capture all Milestone 10 changes under a version
  entry so the release notes for v0.4.0 are accurate and complete.

- **As a consumer of the Paladin crates**, I want a v0.4.0 tag and release on crates.io that
  contains the hardened CI/CD tooling so I can depend on a published version.

---

## 4. Functional Requirements

### FR 1 — CONTRIBUTING.md: "Adding a New Dependency" section

The section must explain the full workflow a developer must follow when adding a new crate dependency:

1. Choose the crate and add it to the appropriate `Cargo.toml` (workspace member or root).
2. Verify the license is in the allowed set in `deny.toml` (`[licenses].allow`) or that an
   exception already exists. If not, open a discussion before adding.
3. Run `make deny` (or `cargo deny check`) locally. A license rejection means the crate is not
   permitted; resolve before merging.
4. Run `make audit` (or `cargo audit`) to check for known vulnerabilities. A fresh dependency must
   produce zero new vulnerability errors.
5. If `cargo-deny` reports a new unmaintained advisory for the dependency, document the rationale
   and add a scoped `[advisories].ignore` entry in `deny.toml` **with an explanatory comment**.
6. Update `CHANGELOG.md` (the `[Unreleased]` block) if the dependency enables a user-visible
   feature or behavioral change.
7. The CI `cargo-deny` and `security-audit` jobs serve as the final gate; do not merge if either
   fails.

### FR 2 — CONTRIBUTING.md: Table of Contents completeness

The Table of Contents must include entries for all top-level `##` sections present in the file,
including the sections added in Epics 1–3:

- `Git Hooks (pre-commit)` ✓ (already in ToC)
- `Security` — must be added to the ToC if missing
- `Releasing` — must be added to the ToC if missing
- `Per-Crate Changelog Maintenance` — must be added to the ToC if missing
- `Adding a New Dependency` — must be added once FR 1 is written

### FR 3 — CHANGELOG.md: Milestone 10 entries

Populate the `## [Unreleased]` block with all Milestone 10 changes. The block must include entries
for:

- **Epic 1 (CI Hardening / Hooks):** pre-commit framework, commit and push stages, hook bypass
  instructions, devcontainer provisioning.
- **Epic 2 (Security & License Compliance):** cargo-audit `.cargo/audit.toml`, cargo-deny
  `deny.toml` with license policy and advisory exceptions, CycloneDX SBOM in the release
  pipeline, OSV-Scanner annotate-only job, `make security` / `make sbom` targets,
  `docs/SECURITY_SCANNING.md`.
- **Epic 3 (Release Automation):** cargo-release selection and `release.toml`, tag-triggered
  `publish-crates` CI job (test gate, dependency-ordered publish, dry-run/skip modes),
  `make release VERSION=` / `make publish-dry-run` targets, `docs/RELEASE_AUTOMATION.md`.
- **Epic 4 (Finalization):** "Adding a New Dependency" CONTRIBUTING.md section, devcontainer tool
  provisioning (cargo-release, cargo-deny, cargo-cyclonedx), v0.4.0 version bump.

### FR 4 — Lockstep version bump to v0.4.0

All workspace crates must be bumped from `0.3.0` to `0.4.0` in lockstep:

- Every `[package].version` field in `Cargo.toml` and `crates/*/Cargo.toml`.
- Every internal dependency pin in `[workspace.dependencies]` and `crates/*/Cargo.toml`.

Use `cargo release version 0.4.0 --execute --no-confirm --workspace` to perform the bump
atomically so no crate is left at the old version.

### FR 5 — CHANGELOG.md finalization

The `make release` target's perl one-liner must convert `## [Unreleased]` → `## [0.4.0] - <date>`
automatically as part of the release commit. This is handled by the `make release` target
(implemented in Epic 3); no manual edit is needed if the `[Unreleased]` block is correctly
populated per FR 3 before running `make release`.

### FR 6 — Annotated v0.4.0 tag and push

Run `make release VERSION=0.4.0` on the Epic 4 feature branch to:

1. Validate semver.
2. Run `make release-check` (fmt + clippy + tests + audit + release build).
3. Bump all crates to 0.4.0 (FR 4).
4. Finalize CHANGELOG.md (FR 5).
5. Commit (`chore(release): version 0.4.0`).
6. Create annotated tag `v0.4.0`.
7. Push branch and tag — triggering the Epic 3 CI release pipeline.

### FR 7 — Conformance gate before release

Before running `make release`, the following must all pass independently:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo audit`
- `pre-commit run --all-files` (check-yaml, check-toml, fmt, clippy, secret detection)

---

## 5. Non-Goals (Out of Scope)

- No new Rust source code changes (`src/` or `crates/`) — this is a pure docs/tooling/release epic.
- No changes to existing CONTRIBUTING.md sections beyond adding the "Adding a New Dependency"
  section and fixing ToC gaps.
- No manual crates.io publish — the CI pipeline from Epic 3 handles publishing on tag push.
- No changes to `deny.toml` or `.cargo/audit.toml` — the existing exception lists are not revisited.
- No new CI jobs — the Epic 3 pipeline is complete.

---

## 6. Technical Considerations

- **`cargo release version` subcommand** bumps both `[package].version` and internal dependency
  pins. After running it, verify with `grep -r "0\.3\.0" Cargo.toml crates/*/Cargo.toml` that no
  stale references remain.
- **`make release` Makefile target** (Epic 3) does the full commit/tag/push flow. Do NOT create
  the tag manually before running it.
- The `release.toml` `publish = false` setting means cargo-release does not publish; the CI
  `publish-crates` job (triggered by the tag) does.
- The pre-push hook runs `cargo build` + `cargo test --lib`; the full `cargo test --workspace`
  conformance check should be run manually before `make release` to catch integration test failures.
- **Version pin consistency**: the root `Cargo.toml` has both `[package].version` *and*
  `[workspace.dependencies]` entries — `cargo release version` updates both.

---

## 7. Success Metrics

1. `CONTRIBUTING.md` contains a clear, numbered "Adding a New Dependency" workflow section, and
   the Table of Contents is complete.
2. `CHANGELOG.md` has a `## [0.4.0]` entry covering all Milestone 10 changes after `make release`.
3. `cargo search paladin-ai` (after CI completes) returns version `0.4.0`.
4. All quality gates (`fmt`, `clippy`, `test`, `audit`, `deny`) pass at `0.4.0`.
5. The `v0.4.0` annotated tag exists in the repository.

---

## 8. Open Questions

1. **Workspace.dependencies version pins in root Cargo.toml (lines 19–27)**: `cargo release
   version` should update these; verify with a dry run first (`cargo release version 0.4.0
   --workspace` without `--execute`) to confirm the diff includes all pin locations.
2. **Tag push destination**: `make release` pushes to `HEAD` (the Epic 4 feature branch, not
   `develop` or `main`). Confirm the tag-triggered CI pipeline fires correctly from a feature
   branch, or if it requires the tag to be on `main`/`develop`.
