# Epic 5: Publish and Finalize

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 5 — Publish and Finalize
**Version Target:** v0.5.0
**Status:** Not Started
**Created:** 2026-05-29

---

## Milestone Context

The project has ~40 markdown documentation files accumulated across eight milestones. This Epic is the final step: review the complete MDBook for quality, deploy it to GitHub Pages, update the root `README.md` to serve as the public project landing page, and cut the v0.5.0 release.

### Milestone Success Criteria (for reference)

- Every existing doc file is audited: current, stale, or delete.
- MDBook builds locally and via CI with zero warnings.
- Documentation published to GitHub Pages (or equivalent).
- All code examples in docs compile against the current workspace.
- New documentation covers: orchestration guide, content processing, crate map, agent↔orchestrator bridge.
- The main `paladin-dev-env` monorepo includes the docs as a subdirectory.

---

## Parallel Execution Context

**Epic 5 is the terminal Epic** — it depends on all prior epics completing:

| Prerequisite | Reason |
|-------------|--------|
| Epic 1 (Content Audit) | All stale content must be identified before final review |
| Epic 2 (MDBook Setup) | MDBook structure must exist before deploying |
| Epic 3 (Content Rewrite) | All rewrites must be complete before final review |
| Epic 4 (New Documentation) | All new content must be in place before final review |

No tasks in this Epic can run in parallel with Epics 1–4. Tasks within this Epic run sequentially: final review → deploy → README update → release.

---

## Epic Overview

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epics 1, 2, 3, 4 (all complete)

### Objective

Finalize the documentation, deploy to GitHub Pages, update the root `README.md`, and tag the v0.5.0 release.

---

## Tasks

### Task 5.1: Final MDBook Build and Review

**Description:**

Perform a complete final review of the MDBook before deployment. The goal is to catch any remaining issues that escaped individual Epic reviews.

**Review Checklist:**

**Build Quality:**

- [ ] `mdbook build docs/` completes with zero errors.
- [ ] `mdbook build docs/` completes with zero warnings.
- [ ] All HTML output files are present in `docs/book/`.

**Link Integrity:**

- [ ] All internal MDBook links resolve (use `mdbook-linkcheck` plugin or equivalent).
- [ ] All external URLs in the documentation return 2xx status (spot-check at minimum).
- [ ] No broken relative file references in `SUMMARY.md`.

**Code Example Verification:**

- [ ] Run `cargo test --doc` across all workspace crates — zero failures attributable to doc examples.
- [ ] Spot-check at least 5 code examples from different guides by copying into a temporary `examples/` file and running `cargo check --example`.

**Content Completeness:**

- [ ] Every chapter listed in `SUMMARY.md` has real content (no placeholder-only pages in user-facing guides).
- [ ] `introduction.md` provides a clear project overview and navigation guidance.
- [ ] Getting Started section allows a new user to install, configure, and run a basic Paladin agent.
- [ ] API Reference covers all public stable APIs from `STABLE_API.md`.

**Cross-Reference Accuracy:**

- [ ] All "see also" links between chapters point to correct sections.
- [ ] `crate-map.md` matches the actual workspace `Cargo.toml` member list.
- [ ] Feature flag table in `crate-map.md` matches the actual feature flags in each crate's `Cargo.toml`.

**Rendering:**

- [ ] Mermaid diagrams render correctly in the built HTML.
- [ ] Code blocks have correct syntax highlighting.
- [ ] Tables render correctly.
- [ ] No raw HTML artifacts or malformed Markdown.

**Deliverables:**

- All checklist items resolved.
- Final `mdbook build` produces clean output with zero errors/warnings.

---

### Task 5.2: Deploy to GitHub Pages

**Description:**

Trigger the CI documentation workflow to deploy the finalized MDBook to GitHub Pages. Verify the published site is accessible and renders correctly.

**Steps:**

1. Merge the documentation branch to `main` (or push directly to `main` if working on `main`).
2. Confirm the `.github/workflows/docs.yml` workflow (created in Epic 2 Task 2.2) triggers on the push.
3. Monitor the GitHub Actions run to confirm it completes successfully.
4. Navigate to the published URL and verify:
   - Site loads correctly.
   - Navigation sidebar is present and all chapters are accessible.
   - Code blocks have syntax highlighting.
   - Mermaid diagrams render.
   - Search functionality works (if mdBook search is enabled).

**Published URL format:** `https://df3ndr.github.io/paladin-dev-env/`

**Deliverables:**

- Documentation live at the published URL.
- Published URL added to:
  - Repository description (GitHub repository "About" section).
  - Root `README.md` (completed in Task 5.3).
  - `Cargo.toml` workspace `documentation` field: `documentation = "https://df3ndr.github.io/paladin-dev-env/"`.

---

### Task 5.3: Update Root README.md

**Description:**

Rewrite the root `README.md` to serve as the public-facing project landing page. The current `README.md` is stale (old crate list, pre-workspace examples).

**Target Structure:**

```markdown
# Paladin Framework

<badge row: CI status | crates.io version | docs | license | MSRV>

One-paragraph project description: what Paladin is, who it's for, and what makes it distinct.

## Quick Example

<working end-to-end code example — minimal, compiles, produces visible output>

## Key Features

- Bullet list of 5–8 headline capabilities

## Crate Ecosystem

<table: crate name | purpose | feature flags | Cargo.toml snippet>

## Documentation

Link to published MDBook + link to crate docs on docs.rs

## Getting Started

1. Prerequisites
2. Add to Cargo.toml
3. Minimal example (or link to quickstart guide)

## Project Status

Current version, stability guarantees (link to STABLE_API.md), changelog link.

## Contributing

Brief paragraph + link to CONTRIBUTING.md and docs/src/contributing/

## License

License statement
```

**Quality Requirements:**

- The Quick Example must compile with the current workspace (`cargo check` verified).
- All badges must resolve to real URLs.
- The crate ecosystem table must match the actual workspace crates.
- All internal links must point to real files.

**Deliverables:**

- Updated `README.md` committed.
- All badges configured correctly.
- Quick Example verified to compile.

---

### Task 5.4: CHANGELOG and Version Bump

**Description:**

Finalize the changelog entry for v0.5.0 and cut the release.

**Steps:**

1. **Update `CHANGELOG.md`:**
   - Add a `## [0.5.0] - 2026-XX-XX` section.
   - Summarize changes from Milestones 8–11 under appropriate headings: `### Added`, `### Changed`, `### Fixed`, `### Documentation`.
   - The documentation section should specifically call out:
     - Complete MDBook documentation published to GitHub Pages.
     - New orchestration, content processing, and bridge guides.
     - Comprehensive crate map and feature flag reference.
     - All code examples verified against the current API.

2. **Bump workspace version to `0.5.0`:**
   - Update `version` in the root `Cargo.toml` `[workspace.package]` section.
   - Confirm all crate `Cargo.toml` files that inherit the workspace version are updated.
   - Run `cargo check` to confirm the version bump doesn't break anything.

3. **Update `release.toml`** (if applicable for automated release tooling).

4. **Tag the release:**
   ```bash
   git tag -a v0.5.0 -m "Release v0.5.0 — Documentation Overhaul and Publish"
   git push origin v0.5.0
   ```
   *(Confirm with the team before pushing the tag to the shared remote.)*

5. **Create GitHub Release:**
   - Use the GitHub CLI or web UI to create a release from the `v0.5.0` tag.
   - Use the `CHANGELOG.md` v0.5.0 section as the release body.
   - Link to the published documentation URL in the release notes.

**Deliverables:**

- `CHANGELOG.md` updated with complete v0.5.0 entry.
- Workspace version bumped to `0.5.0`.
- `v0.5.0` tag pushed.
- GitHub Release created with changelog content and documentation link.

---

## Deliverables Summary

| Artifact | Description |
|----------|-------------|
| Clean MDBook build | Zero errors/warnings, all links valid |
| Published GitHub Pages site | Documentation live at `https://df3ndr.github.io/paladin-dev-env/` |
| Updated `README.md` | Project landing page with working example, badges, crate table |
| Updated `CHANGELOG.md` | v0.5.0 entry covering Milestones 8–11 |
| `v0.5.0` Git tag | Tagged release on `main` |
| GitHub Release | Release created with changelog and docs link |

---

## Definition of Done

- [ ] `mdbook build` produces zero errors and zero warnings.
- [ ] All internal MDBook links resolve.
- [ ] All doc test examples pass.
- [ ] Published site is accessible at the GitHub Pages URL.
- [ ] Root `README.md` is rewritten with working quick example and current crate table.
- [ ] `CHANGELOG.md` has a complete v0.5.0 section.
- [ ] Workspace version is `0.5.0` in all `Cargo.toml` files.
- [ ] `v0.5.0` tag exists in the remote repository.
- [ ] GitHub Release is published.

---

## Schedule Reference

| Phase | This Epic | Duration | Predecessors |
|-------|-----------|----------|-------------|
| Phase 3 | Epic 5: Publish + Finalize | 0.5 sprint | Epics 1–4 |

This is the terminal Epic of Milestone 11. Completing it marks the Milestone as done and the project at v0.5.0.
