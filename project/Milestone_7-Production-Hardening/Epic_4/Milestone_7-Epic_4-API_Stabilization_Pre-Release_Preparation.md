
## Epic 4: API Stabilization and Pre-Release Preparation

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Large
**Dependencies:** Epics 1–3

### Objective

Prepare the Paladin crate ecosystem for its first publishable release. This includes finalizing the public API surface across all crates, implementing semantic versioning discipline, preparing crate metadata for crates.io, producing per-crate documentation, and running a release readiness audit.

### Background & Rationale

The workspace now contains approximately 10 crates. Each needs:

- A correct `Cargo.toml` with `[package]` metadata (description, license, repository, keywords, categories, documentation URL).
- A crate-level `README.md` for crates.io rendering.
- A crate-level `CHANGELOG.md` following Keep a Changelog format.
- Explicit `#![deny(missing_docs)]` or documented public API coverage.
- Semantic version alignment (lockstep initially, with a path to independent versioning).

The public API was hardened in Milestone 1 (Epic 2) and documented in `STABLE_API.md`. This Epic extends that work to the per-crate level and formalizes the release process.

### Acceptance Criteria

1. Every public crate has a complete `[package]` section with description, license (`MIT`), repository URL, keywords, and categories.
2. Every public crate has a `README.md` that renders correctly on crates.io.
3. Every public crate has a `CHANGELOG.md`.
4. `cargo publish --dry-run -p <crate>` succeeds for every publishable crate.
5. A versioning policy document defines lockstep vs. independent versioning rules.
6. A release checklist covers all steps from code freeze to publish.
7. `#![warn(missing_docs)]` enabled on all public crates; documentation coverage exceeds 90% of public items.
8. `STABLE_API.md` updated to the per-crate level with stability tiers per type.
9. The complete documentation suite is production-ready.

### Tasks

#### Task 4.1: Prepare Crate Metadata

**Description:** For each public crate, add or update the `[package]` section in `Cargo.toml` with: `name`, `version` (from `workspace.package`), `edition`, `authors`, `description`, `readme`, `repository`, `license`, `keywords`, `categories`, `documentation`.

**Deliverables:**
- All crate `Cargo.toml` files updated with complete metadata.
- `cargo publish --dry-run -p <crate>` succeeds for all crates.
- No crates.io validation errors.

**Estimated Effort:** Small

#### Task 4.2: Write Per-Crate README Files

**Description:** Each crate gets a `README.md` that describes the crate's purpose, key types, usage examples, feature flags, and links to the full documentation. The root `README.md` serves as the umbrella project overview with links to per-crate READMEs.

**Deliverables:**
- `crates/paladin-core/README.md`
- `crates/paladin-ports/README.md`
- `crates/paladin-battalion/README.md`
- `crates/paladin-llm/README.md`
- `crates/paladin-memory/README.md`
- `crates/paladin-web/README.md` (if extracted)
- `crates/paladin-notifications/README.md` (if extracted)
- `crates/paladin-content/README.md` (if extracted)
- `crates/paladin-storage/README.md` (if extracted)
- Updated root `README.md`.

**Estimated Effort:** Medium

#### Task 4.3: Write Per-Crate CHANGELOG Files

**Description:** Initialize per-crate `CHANGELOG.md` files with the history relevant to each crate (extracted from the monolithic `CHANGELOG.md`). Establish the format and conventions for future changelog maintenance.

**Deliverables:**
- Per-crate `CHANGELOG.md` files.
- Contribution guide updated with changelog maintenance instructions.

**Estimated Effort:** Small

#### Task 4.4: Documentation Coverage Audit

**Description:** Enable `#![warn(missing_docs)]` on all public crates. Run `cargo doc --workspace --no-deps` and fix all warnings. Audit documentation coverage — every public type, trait, function, and module should have a doc comment.

**Deliverables:**
- `#![warn(missing_docs)]` enabled in all crate `lib.rs` files.
- All documentation warnings resolved.
- `cargo doc --workspace --no-deps` produces zero warnings.
- Coverage report showing percentage of documented public items per crate.

**Estimated Effort:** Medium

#### Task 4.5: Define Versioning Policy and Release Process

**Description:** Produce a versioning policy document that defines:
- Initial lockstep versioning (all crates share a version, e.g., `0.2.0`).
- Criteria for transitioning to independent per-crate versioning.
- What constitutes a breaking change for each crate.
- The release checklist: code freeze → changelog finalization → version bump → CI green → dry-run publish → publish → tag → announcement.
- Publishing order (dependency-first: `paladin-core` → `paladin-ports` → leaf crates → `paladin` facade).

**Deliverables:**
- `docs/VERSIONING_POLICY.md`.
- `docs/RELEASE_CHECKLIST.md`.
- Publishing order documented and scripted (or Makefile target `make release`).

**Estimated Effort:** Medium

#### Task 4.6: Update STABLE_API.md to Per-Crate Level

**Description:** Extend `STABLE_API.md` to document the public API surface of each crate individually. Include stability tiers (Stable, Unstable, Experimental) per type. This becomes the contract for backward compatibility guarantees.

**Deliverables:**
- Per-crate sections in `STABLE_API.md` (or per-crate `API.md` files).
- Stability tier assigned to every public type and trait.
- Cross-crate dependency contract documented.

**Estimated Effort:** Medium

#### Task 4.7: Release Readiness Audit

**Description:** Conduct a comprehensive release readiness audit:
- All tests pass (`cargo test --workspace`).
- All clippy clean (`cargo clippy --workspace -- -D warnings`).
- All formatted (`cargo fmt --all -- --check`).
- All docs build (`cargo doc --workspace --no-deps`).
- All dry-run publishes succeed.
- Security audit (`cargo audit`).
- License compliance verified (all dependencies compatible with MIT).
- Binary size and dependency tree reviewed for unexpected bloat.

**Deliverables:**
- Release readiness audit report.
- Any blocking issues identified and resolved.
- Sign-off for release candidate tag.

**Estimated Effort:** Medium

---
