# PRD: Document Facade Crate Role and Finalize (Milestone 8, Epic 5)

**Project:** Paladin Framework
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Epic:** 5 — Document Facade Crate Role and Finalize
**Version Target:** v0.2.0
**Status:** Ready for Implementation
**Created:** 2026-05-30
**Author:** AI Coding Agent (GitHub Copilot)

---

## 1. Introduction / Overview

Epic 5 is the closing act of Milestone 8. Epics 1–4 cleaned and stabilized the facade crate
(`src/`) — removing dead shims, relocating misplaced modules, and renaming `use_cases` →
`services`. Now the workspace needs two things to cross the v0.2.0 finish line:

1. **Documentation that explains what the facade crate _is_.** Developers landing on `src/lib.rs`
   or the crate docs should immediately understand that this crate is the application assembly
   point, not a business-logic crate. A brief `//!` section in `lib.rs` and a `src/README.md`
   provide two complementary entry points for this explanation.

2. **A synchronized `STABLE_API.md` and a properly cut v0.2.0 `CHANGELOG.md` entry.** The
   current `STABLE_API.md` header still says "Last Updated: 2026-05-28" and its catalog sections
   have not been audited against the post-Milestone-8 workspace. The `CHANGELOG.md` `[Unreleased]`
   block must be promoted to a formal `## [0.2.0]` release section.

A final quality gate (`cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`, `cargo doc`)
confirms the workspace is release-ready, and `api_surface_current.txt` / `final-api.txt` are
regenerated as the v0.2.0 baseline artifacts.

---

## 2. Goals

1. Every developer reading `src/lib.rs` or `cargo doc` output understands the facade crate's
   role as the application assembly point and composition root.
2. `STABLE_API.md` is fully in sync with the post-Milestone-8 workspace state — no stale paths,
   no missing entries.
3. `CHANGELOG.md` has a properly formatted `## [0.2.0]` release section promoted from
   `[Unreleased]`, with Added / Changed / Removed / Breaking sub-sections.
4. All quality gates pass (build, test, clippy, fmt, doc — exit 0).
5. `api_surface_current.txt` and `final-api.txt` are regenerated as the v0.2.0 API baseline.

---

## 3. User Stories

- **As a contributor** opening `src/lib.rs` for the first time, I want a clear `//!` section that
  explains the facade crate's role so I don't mistake it for a leaf crate or add logic to it that
  belongs in a leaf crate.

- **As a library consumer** reading `cargo doc`, I want the top-level crate documentation to
  accurately describe the assembly-point architecture and distinguish the facade from the leaf
  crates.

- **As a library consumer upgrading from v0.1.x to v0.2.0**, I want a complete, well-structured
  CHANGELOG entry and an up-to-date `STABLE_API.md` so I know exactly which paths changed and
  what the new stable surface looks like.

- **As a maintainer** cutting the v0.2.0 release, I want `api_surface_current.txt` and
  `final-api.txt` to reflect the actual post-Milestone-8 API so they can serve as the baseline
  for detecting future breakage.

---

## 4. Functional Requirements

### 4.1 — Facade Crate Documentation (`src/lib.rs` + `src/README.md`)

**FR-1.** The existing `//!` doc comment in `src/lib.rs` must be **extended** (not replaced) with
a new `## Facade Crate Role` section containing:
  - A one-paragraph explanation that this crate is the **application assembly point and
    composition root** for the Paladin workspace.
  - A clear statement of what the facade contains: `ServiceRunner` (the composition root),
    application-layer coordination services (`src/application/services/`), configuration loading
    (`src/config/`), CLI modules (`src/application/cli/`, feature-gated), and binary entry
    points (`main.rs`, `bin/paladin-cli.rs`).
  - A clear statement of what the facade does **not** contain: business logic, port trait
    definitions, or infrastructure adapter implementations (those live in the leaf crates).
  - A bulleted list of the leaf crates and their capabilities:
    `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`,
    `paladin-notifications`, `paladin-storage`, `paladin-content`, `paladin-web`.
  - The updated `## Architecture` section label must remain (do not delete existing content —
    append or augment).

**FR-2.** A new file `src/README.md` must be created that:
  - Has a `# Paladin Facade Crate` heading.
  - Contains a prose description of the facade-crate role (assembly point, composition root,
    `ServiceRunner`, application services, CLI, binaries).
  - Includes a "What lives here" table with three columns: **Path**, **Purpose**, **Notes**.
    The table must cover at minimum: `src/application/services/`, `src/application/cli/`,
    `src/config/`, `src/infrastructure/`, `src/core/`, `src/bin/`, `src/main.rs`.
  - Explains the dependency-flow rule: facade → leaf crates (one direction only; leaf crates
    must not import from the facade).
  - References `STABLE_API.md` for the public API contract.

**FR-3.** The `## Architecture` section in `src/lib.rs` must update the label
  `Application Layer` description from "Use cases and port trait definitions" to
  "Application services and coordination logic".

### 4.2 — `STABLE_API.md` Comprehensive Audit and Update

**FR-4.** The `STABLE_API.md` header block must be updated:
  - `Version:` → `0.2.0`
  - `Last Updated:` → `2026-05-30`
  - `Epic:` → `Milestone 8, Epic 5 - Document Facade Crate Role and Finalize`
  - The existing breaking-change callout box must be updated to accurately reflect the
    current v0.2.0 breaking changes (shim removals from Epic 2/3 AND the `use_cases` →
    `services` rename from Epic 4).

**FR-5.** The `### paladin (facade crate)` section in the Per-Crate table must be audited and
  updated to reflect the post-Milestone-8 module layout (no `application/ports/`,
  no `application/storage/`, `application/services/` instead of `application/use_cases/`).

**FR-6.** Any item in the **Stable Public API Catalog** section that references `use_cases`
  path segments must be updated to `services`.

**FR-7.** Any items in the catalog that reference modules deleted in Milestone 8 (e.g.,
  `application::storage::sql_store`, `application::ports::*` shim paths) must be removed
  or annotated as removed.

**FR-8.** The `## Tracking API Changes` section must reference `api_surface_current.txt` as
  the v0.2.0 baseline (update any stale baseline dates or file references).

### 4.3 — `CHANGELOG.md` v0.2.0 Release Entry

**FR-9.** The `## [Unreleased]` block must be **promoted** to `## [0.2.0] - 2026-05-30` using
  standard Keep-a-Changelog format. A fresh empty `## [Unreleased]` section must be inserted
  above it for future changes.

**FR-10.** The `## [0.2.0]` section must contain all four sub-sections in this order:
  `### Breaking Changes`, `### Added`, `### Changed`, `### Removed`. Each sub-section must
  contain the entries already accumulated in `[Unreleased]`, redistributed to the correct
  sub-section.

**FR-11.** A new `### Changed` entry must document the facade crate architecture documentation
  additions (this Epic's work): "Documented facade crate role as application assembly point;
  added `src/README.md` and updated `src/lib.rs` `//!` docs."

**FR-12.** A `[0.2.0]: <compare URL>` link entry must be added to the bottom link-reference
  block of `CHANGELOG.md` (Keep-a-Changelog convention).

### 4.4 — API Surface Baseline Files

**FR-13.** `api_surface_current.txt` must be regenerated using the workspace's public API
  extraction method (documented in `STABLE_API.md` §Automated Tracking). The file header
  comment (if present) must note `v0.2.0 baseline — 2026-05-30`.

**FR-14.** `final-api.txt` must be updated to reflect the same v0.2.0 snapshot.

### 4.5 — Final Quality Gate

**FR-15.** `cargo build --workspace` — exit 0, zero errors.
**FR-16.** `cargo test --workspace` — all tests pass, zero failures.
**FR-17.** `cargo clippy --workspace -- -D warnings` — zero warnings.
**FR-18.** `cargo fmt --all -- --check` — exit 0, no formatting drift.
**FR-19.** `cargo doc --workspace --no-deps` — exit 0 (warnings acceptable; must not fail).

---

## 5. Non-Goals (Out of Scope)

- **No logic changes.** This Epic is documentation-only for `src/lib.rs`, `src/README.md`,
  `STABLE_API.md`, and `CHANGELOG.md`. No Rust source code logic is added or modified.
- **No leaf crate documentation.** Crate-level docs for `paladin-core`, `paladin-ports`, etc.
  are out of scope — those belong to Milestone 11 (Documentation Overhaul).
- **No new feature flags or feature-flag changes.** 
- **No new Rust modules, structs, or traits.**
- **No API surface changes** — if anything is found during the `STABLE_API.md` audit to be
  genuinely missing from the public API, it must be flagged in §9 Open Questions rather than
  silently added.
- **No merge to `main` / version tag** — the branch is left as a release candidate; the actual
  tag and merge are a separate process.

---

## 6. Design Considerations

### `src/lib.rs` `//!` Section Structure (Target)

The final `//!` block should have these top-level `##` headings in order:

1. _(implicit title — first `//!` line)_
2. `## Core Concepts`
3. `## Facade Crate Role` ← **new section** (FR-1)
4. `## Architecture` ← update "Application Layer" description (FR-3)
5. `## Stable Public API`
6. `## Quick Start`
7. `## Feature Flags`

### `src/README.md` "What lives here" table (example structure)

| Path | Purpose | Notes |
|------|---------|-------|
| `src/application/services/` | Application coordination services | 11 sub-modules, 39 `.rs` files |
| `src/application/cli/` | CLI command implementations | Feature-gated (`cli` flag) |
| `src/config/` | Configuration loading & settings types | Stays in facade (composition root needs config) |
| `src/infrastructure/` | Infrastructure adapter implementations | Adapters not extracted to leaf crates |
| `src/core/` | Re-export bridge to `paladin-core` | Minimal structure; real logic in `paladin-core` |
| `src/bin/` | Binary entry points | `paladin-cli.rs` (feature-gated) |
| `src/main.rs` | Default binary entry point | Thin wrapper; bootstraps `ServiceRunner` |

---

## 7. Technical Considerations

- **`src/lib.rs` edit strategy:** The existing `//!` block is 60 lines. The new section should be
  inserted between `## Architecture` and `## Stable Public API` to keep reading order logical.
  Do not rewrite the whole file — append the new section in place.

- **`src/README.md` and `cargo doc`:** Rust's `cargo doc` does not automatically include
  `src/README.md`; it is for human readers browsing the source. The `//!` docs in `lib.rs`
  serve the `cargo doc` audience. Both are needed per the user's 1C selection.

- **`STABLE_API.md` audit method:** Read the current file sections, then cross-reference against
  the actual module tree (`find src/ crates/ -name "*.rs" | sort`). Flag stale paths. Do not
  delete the stability-tier prose sections — only update the catalog table entries and header.

- **`CHANGELOG.md` promotion:** The existing `[Unreleased]` block already has `### Breaking
  Changes`, `### Added`, and `### Removed` sub-sections. When promoting to `[0.2.0]`, add
  a `### Changed` entry for this Epic's documentation work, then rename the block.

- **API surface extraction:** Check `STABLE_API.md` §Automated Tracking or `Makefile` for the
  exact command used to generate `api_surface_current.txt`. If the command is
  `cargo public-api` or similar, use that. If no tooling is available, use
  `cargo doc --workspace --no-deps 2>&1` output. Document which method was used in the file
  header comment.

- **Branch context:** All work occurs on
  `feature/milestone_8-epic_4-use_cases-services-rename` (the branch carrying all Milestone 8
  work). No new branch is needed for Epic 5.

---

## 8. Success Metrics

| Metric | Target |
|--------|--------|
| `src/lib.rs` has `## Facade Crate Role` section | Present |
| `src/README.md` exists with "What lives here" table | Present |
| `STABLE_API.md` header `Last Updated` | `2026-05-30` |
| `STABLE_API.md` zero `use_cases` path references | 0 hits |
| `CHANGELOG.md` has `## [0.2.0] - 2026-05-30` section | Present |
| `CHANGELOG.md` `[Unreleased]` is empty / reset | Present |
| `cargo build --workspace` | Exit 0 |
| `cargo test --workspace` | 0 failures |
| `cargo clippy --workspace -- -D warnings` | 0 warnings |
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo doc --workspace --no-deps` | Exit 0 |
| `api_surface_current.txt` updated with v0.2.0 header | Present |
| `final-api.txt` updated | Present |

---

## 9. Open Questions

1. **`api_surface_current.txt` generation command:** The `STABLE_API.md` §Automated Tracking
   section references `cargo-public-api`. Confirm whether `cargo-public-api` is installed in
   the dev container before Task 4.1, and fall back to an alternative method if not.

2. **STABLE_API.md catalog completeness:** During the audit, if items in the catalog are found
   to be _missing_ from the current public API (types that exist in code but are not listed),
   should they be added? (Current assumption: yes, add them — this is a comprehensive sync.)

3. **`[0.2.0]` compare URL:** The Keep-a-Changelog link reference at the bottom needs a
   `[0.2.0]:` URL. Use the GitHub compare URL pattern:
   `https://github.com/DF3NDR/paladin-dev-env/compare/v0.1.0...v0.2.0`. Confirm the `v0.1.0`
   tag exists in the repo before adding the link.

4. **`lib.rs.backup`:** A `src/lib.rs.backup` file exists in the workspace. Confirm it should
   be deleted (it is not a `.rs` module file and should not be committed) before the final
   quality gate.
