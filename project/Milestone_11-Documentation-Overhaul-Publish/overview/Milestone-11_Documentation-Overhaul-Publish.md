# Milestone 11: Documentation Overhaul and Publish

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Version Target:** v0.5.0
**Status:** Planning
**Created:** 2026-05-29
**Document Version:** 1.0

---

## Executive Summary

The project has ~40 markdown documentation files accumulated across eight milestones of development and refactoring. Many reference pre-refactoring paths (`application_settings.rs`, old import paths), deprecated configuration patterns, or contain placeholder content. The documentation has never been published as a structured, navigable reference. This Milestone audits all existing docs, reorganizes them into an MDBook chapter hierarchy, rewrites stale content, writes new content for features completed in Milestones 8–9, and publishes via GitHub Pages.

### Success Criteria

- Every existing doc file is audited: current, stale, or delete.
- MDBook builds locally and via CI with zero warnings.
- Documentation published to GitHub Pages (or equivalent).
- All code examples in docs compile against the current workspace.
- New documentation covers: orchestration guide, content processing, crate map, agent↔orchestrator bridge.
- A separate docs repository (or `docs/` subdirectory) is established with MDBook configuration.
- The main `paladin-dev-env` monorepo includes the docs as a subdirectory.

---

## Parallel Execution Context

This Milestone has a **dependency on Milestones 8 and 9** for content accuracy — documentation must reflect the final directory structure (Milestone 8) and the working orchestrator API (Milestone 9). However:

- **Epic 1 (Audit)** and **Epic 2 (MDBook Setup)** can begin immediately in parallel with Milestones 8–10.
- **Epic 3 (Rewrite)** and **Epic 4 (New Content)** should wait until Milestone 9 Epics 1–3 are complete so the orchestrator and bridge APIs are stable.
- **Epic 5 (Publish)** is the final step.

---

## Epic 1: Documentation Content Audit

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** None (can start immediately)

### Objective

Audit every markdown file in `docs/`, the root `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, and per-crate READMEs. Classify each as current, stale (needs rewrite), or deletable.

### Tasks

#### Task 1.1: Inventory and Classify All Documentation

**Description:** For each document, check:
- Do code examples compile against the current API?
- Do import paths match the current workspace structure?
- Do configuration examples match the current `config.yml` schema?
- Are referenced files/directories still present?
- Is the content complete or placeholder?

**Deliverables:**
- `docs-audit.md` with classification per file:

| Document | Status | Issues | Action |
|----------|--------|--------|--------|
| `README.md` | Stale | Old crate list, pre-workspace examples | Rewrite |
| `docs/CONFIGURATION.md` | Stale | References deleted `application_settings.rs` | Rewrite |
| `docs/QUICKSTART.md` | Stale | Code examples may not compile | Verify and fix |
| ... | ... | ... | ... |

#### Task 1.2: Verify All Code Examples Compile

**Description:** Extract every code block from every markdown file. Attempt to compile each against the current workspace. Flag failures.

**Deliverables:**
- List of broken code examples with file and line number.
- Proposed fixes or rewrites.

---

## Epic 2: MDBook Setup and Structure

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** None (can start immediately, parallel with Epic 1)

### Objective

Set up the MDBook infrastructure: repository structure, `book.toml`, `SUMMARY.md`, CI build pipeline, and chapter hierarchy.

### Tasks

#### Task 2.1: Create Documentation Structure

**Description:** Establish the MDBook project within the monorepo at `docs/` (or a separate `paladin-docs` repo included as a Git submodule or subtree).

**Recommended: in-monorepo at `docs/`** — simpler CI, single PR for code+docs changes, no submodule complexity.

```
docs/
├── book.toml                    # MDBook configuration
├── src/
│   ├── SUMMARY.md               # Navigation structure
│   ├── introduction.md
│   ├── getting-started/
│   │   ├── installation.md
│   │   ├── quickstart.md
│   │   └── configuration.md
│   ├── user-guides/
│   │   ├── paladin-agents.md
│   │   ├── battalion-patterns.md
│   │   ├── arsenal-tools.md
│   │   ├── garrison-memory.md
│   │   ├── sanctum-vector-memory.md
│   │   ├── herald-output.md
│   │   ├── orchestration.md
│   │   └── content-processing.md
│   ├── architecture/
│   │   ├── overview.md
│   │   ├── hexagonal-design.md
│   │   ├── domain-model.md
│   │   ├── crate-map.md
│   │   └── design-patterns.md
│   ├── deployment/
│   │   ├── docker.md
│   │   ├── kubernetes.md
│   │   └── production.md
│   ├── operations/
│   │   ├── logging.md
│   │   ├── monitoring.md
│   │   └── performance-tuning.md
│   ├── api-reference/
│   │   ├── stable-api.md
│   │   ├── feature-flags.md
│   │   └── migration-guide.md
│   └── contributing/
│       ├── development-setup.md
│       ├── testing-guide.md
│       └── architecture-decisions.md
```

**Deliverables:**
- `book.toml` configured.
- `SUMMARY.md` with chapter hierarchy.
- Placeholder files for each section.
- `mdbook build` succeeds locally.

#### Task 2.2: Configure CI for MDBook

**Description:** Add a CI workflow that:
- Builds the MDBook on every PR that touches `docs/`.
- Deploys to GitHub Pages on merge to `main`.

**Deliverables:**
- CI workflow file.
- GitHub Pages deployment configured.
- Build succeeds on CI.

#### Task 2.3: Migrate Existing Docs to MDBook Structure

**Description:** Move existing markdown files from `docs/` into the MDBook `src/` directory, mapping each to its chapter. Files that need rewriting get their current content moved as-is (rewrites happen in Epic 3).

**Deliverables:**
- All existing docs placed in MDBook structure.
- `SUMMARY.md` links to all migrated files.
- `mdbook build` succeeds (content may be stale but structurally valid).

---

## Epic 3: Content Rewrite

**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Milestone 8 (final paths), Milestone 9 Epics 1–3 (orchestrator API stable)

### Objective

Rewrite all stale documentation identified in the audit. Every code example must compile. Every path must be current. Every configuration example must match the current schema.

### Tasks

#### Task 3.1: Rewrite Getting Started Guides

**Description:** Update `installation.md`, `quickstart.md`, and `configuration.md` with:
- Current toolchain requirements.
- Current crate names and feature flags.
- Working code examples.
- Current `config.yml` schema (post-Milestone 6 Epic 1 decomposition).

**Deliverables:**
- Three rewritten guides.
- All code examples verified to compile.

#### Task 3.2: Rewrite Architecture Documentation

**Description:** Update architecture docs to reflect the final workspace structure:
- Crate dependency diagram.
- Hexagonal layer descriptions with current module paths.
- Domain model reflecting Milestone 6 relocations.

**Deliverables:**
- Updated architecture docs.
- New `crate-map.md` showing all workspace crates, their purposes, and dependencies.

#### Task 3.3: Rewrite User Guides

**Description:** Update each user guide (agents, battalion, arsenal, garrison, sanctum, herald) with:
- Current import paths.
- Current API signatures.
- Working examples.
- Cross-references to related guides.

**Deliverables:**
- All user guides rewritten and verified.

#### Task 3.4: Rewrite Deployment and Operations Docs

**Description:** Update Docker, Kubernetes, and operations docs for workspace build structure, current configuration schema, and current feature flags.

**Deliverables:**
- Deployment docs current.
- Operations docs current.

---

## Epic 4: New Documentation

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Milestone 9 (features must exist to document)

### Objective

Write new documentation for features and capabilities that don't have existing docs.

### Tasks

#### Task 4.1: Orchestration Guide

**Description:** Write a comprehensive guide covering:
- Creating workflows (sequential, parallel, event-driven).
- Scheduling jobs.
- Queue management.
- Event and trigger system.
- Content processing pipelines.
- Examples with working code.

**Deliverables:**
- `user-guides/orchestration.md`.

#### Task 4.2: Content Processing Guide

**Description:** Write a guide covering:
- Content ingestion (PDF, web scraping, RSS, HTTP).
- Content aggregation and filtering.
- Content → Agent bridge (using AI agents for enrichment).
- Content delivery.

**Deliverables:**
- `user-guides/content-processing.md`.

#### Task 4.3: Agent ↔ Orchestrator Bridge Guide

**Description:** Write a guide covering:
- How AI agents can trigger orchestration actions.
- How orchestration workflows can invoke AI agents.
- Configuration examples.
- Use case recipes (e.g., "news monitoring pipeline with AI analysis").

**Deliverables:**
- Included in `user-guides/orchestration.md` or a separate bridge guide.

#### Task 4.4: Crate Map and Feature Flag Reference

**Description:** Write a comprehensive reference showing:
- Every workspace crate, its purpose, and what it depends on.
- Every feature flag, what it enables, and what dependencies it gates.
- Consumer profiles: "I only need X" → "depend on these crates with these features."

**Deliverables:**
- `api-reference/crate-map.md` (or update existing `FEATURE_FLAGS.md`).

---

## Epic 5: Publish and Finalize

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epics 1–4

### Tasks

#### Task 5.1: Final MDBook Build and Review

**Description:** Build the complete MDBook. Review every page for accuracy, broken links, and rendering issues. Fix any problems.

**Deliverables:**
- `mdbook build` produces clean output.
- All internal links resolve.
- All code examples verified.

#### Task 5.2: Deploy to GitHub Pages

**Description:** Trigger the CI deployment. Verify the published site is accessible and renders correctly.

**Deliverables:**
- Documentation live at the published URL.
- Link added to repository description, `README.md`, and `Cargo.toml` `documentation` field.

#### Task 5.3: Update Root README.md

**Description:** Rewrite the root `README.md` to serve as the project landing page:
- Clear project description.
- Quick example.
- Links to MDBook documentation.
- Crate ecosystem overview.
- Badge row (CI status, crates.io version, docs, license).

**Deliverables:**
- Updated `README.md`.

#### Task 5.4: CHANGELOG and Version Bump

- Update `CHANGELOG.md`.
- Bump to v0.5.0.
- Tag release.

---

## Schedule Overview

| Phase | Epic | Duration | Predecessors |
|-------|------|----------|-------------|
| Phase 1A | Epic 1: Content Audit | 0.5–1 sprint | None |
| Phase 1B | Epic 2: MDBook Setup | 0.5–1 sprint | None |
| Phase 2 | Epic 3: Content Rewrite | 2–3 sprints | Milestones 8, 9 |
| Phase 2 | Epic 4: New Documentation | 1–2 sprints | Milestone 9 |
| Phase 3 | Epic 5: Publish + Finalize | 0.5 sprint | Epics 1–4 |

**Total: 4–6 sprints** (Epics 1 and 2 start immediately; 3 and 4 wait for Milestones 8–9)
