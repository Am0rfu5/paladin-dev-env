# Epic 2: MDBook Setup and Structure

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 2 — MDBook Setup and Structure
**Version Target:** v0.5.0
**Status:** Not Started
**Created:** 2026-05-29

---

## Milestone Context

The project has ~40 markdown documentation files accumulated across eight milestones of development and refactoring. The documentation has never been published as a structured, navigable reference. This Epic establishes the MDBook infrastructure: the directory layout, `book.toml`, `SUMMARY.md`, CI build pipeline, and the migration of existing documents into the new chapter hierarchy.

### Milestone Success Criteria (for reference)

- Every existing doc file is audited: current, stale, or delete.
- MDBook builds locally and via CI with zero warnings.
- Documentation published to GitHub Pages (or equivalent).
- All code examples in docs compile against the current workspace.
- New documentation covers: orchestration guide, content processing, crate map, agent↔orchestrator bridge.
- The main `paladin-dev-env` monorepo includes the docs as a subdirectory.

---

## Parallel Execution Context

**Epic 2 (MDBook Setup) can begin immediately** — there is no dependency on Milestones 8–10 or on Epic 1 completing. It runs in parallel with Epic 1 (Content Audit).

Epic 3 (Content Rewrite) and Epic 4 (New Documentation) depend on both this Epic (for the structure to write into) and on Milestone 9 (for stable APIs to document). Epic 5 (Publish) depends on all prior epics.

---

## Epic Overview

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** None — can start immediately, parallel with Epic 1

### Objective

Set up the MDBook infrastructure: repository structure, `book.toml`, `SUMMARY.md`, CI build pipeline, and chapter hierarchy. Move existing markdown files into the structure so `mdbook build` succeeds (content accuracy is addressed in Epic 3).

---

## Target Directory Structure

The MDBook lives **in-monorepo at `docs/`** — simpler CI, single PR for code+docs changes, no submodule complexity.

```
docs/
├── book.toml                          # MDBook configuration
└── src/
    ├── SUMMARY.md                     # Navigation structure (table of contents)
    ├── introduction.md
    ├── getting-started/
    │   ├── installation.md
    │   ├── quickstart.md
    │   └── configuration.md
    ├── user-guides/
    │   ├── paladin-agents.md
    │   ├── battalion-patterns.md
    │   ├── arsenal-tools.md
    │   ├── garrison-memory.md
    │   ├── sanctum-vector-memory.md
    │   ├── herald-output.md
    │   ├── orchestration.md
    │   └── content-processing.md
    ├── architecture/
    │   ├── overview.md
    │   ├── hexagonal-design.md
    │   ├── domain-model.md
    │   ├── crate-map.md
    │   └── design-patterns.md
    ├── deployment/
    │   ├── docker.md
    │   ├── kubernetes.md
    │   └── production.md
    ├── operations/
    │   ├── logging.md
    │   ├── monitoring.md
    │   └── performance-tuning.md
    ├── api-reference/
    │   ├── stable-api.md
    │   ├── feature-flags.md
    │   └── migration-guide.md
    └── contributing/
        ├── development-setup.md
        ├── testing-guide.md
        └── architecture-decisions.md
```

---

## Tasks

### Task 2.1: Create Documentation Structure

**Description:**

1. Create the `docs/src/` directory hierarchy as specified above.
2. Create `docs/book.toml` with the following minimum configuration:

```toml
[book]
authors = ["Paladin Framework Contributors"]
language = "en"
multilingual = false
src = "src"
title = "Paladin Framework"
description = "Enterprise multi-agent orchestration framework in Rust"

[output.html]
git-repository-url = "https://github.com/DF3NDR/paladin-dev-env"
edit-url-template = "https://github.com/DF3NDR/paladin-dev-env/edit/main/docs/src/{path}"
site-url = "/paladin-dev-env/"

[output.html.fold]
enable = true
level = 1
```

3. Create `docs/src/SUMMARY.md` with the full chapter hierarchy linking to every file in the structure.
4. Create placeholder `.md` files for all chapters not yet covered by existing content. Each placeholder should include the chapter title as an H1 heading and a single line: `> Content coming in Epic 3.`

**Deliverables:**

- `docs/book.toml` configured and committed.
- `docs/src/SUMMARY.md` with full chapter hierarchy.
- All placeholder files in place.
- `mdbook build` succeeds locally with zero errors.

---

### Task 2.2: Configure CI for MDBook

**Description:**

Add a GitHub Actions workflow that:

1. **On every PR** that touches `docs/`: builds the MDBook and fails the PR if `mdbook build` exits non-zero.
2. **On merge to `main`**: builds and deploys to GitHub Pages via `peaceiris/actions-gh-pages` (or the native GitHub Pages Action).

**Workflow file:** `.github/workflows/docs.yml`

```yaml
name: Documentation

on:
  push:
    branches: [main]
    paths: ["docs/**"]
  pull_request:
    paths: ["docs/**"]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install mdBook
        run: |
          curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-unknown-linux-gnu.tar.gz \
            | tar -xz --directory=/usr/local/bin
      - name: Build docs
        run: mdbook build docs/
      - name: Deploy to GitHub Pages
        if: github.ref == 'refs/heads/main'
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: docs/book
```

**Deliverables:**

- `.github/workflows/docs.yml` created and committed.
- GitHub Pages deployment configured on the repository (source: `gh-pages` branch, `/ (root)` directory).
- Build succeeds on CI.

---

### Task 2.3: Migrate Existing Docs to MDBook Structure

**Description:**

Move or copy existing markdown files from the current `docs/` flat layout into the appropriate chapter location under `docs/src/`. The content does **not** need to be rewritten here — that is Epic 3. The goal is structural validity: `mdbook build` succeeds and all chapters are linked.

**Migration Mapping (initial draft — confirm against Epic 1 audit output):**

| Existing File | MDBook Target |
|---------------|--------------|
| `docs/INSTALLATION.md` | `docs/src/getting-started/installation.md` |
| `docs/QUICKSTART.md` | `docs/src/getting-started/quickstart.md` |
| `docs/CONFIGURATION.md` | `docs/src/getting-started/configuration.md` |
| `docs/ARSENAL.md` | `docs/src/user-guides/arsenal-tools.md` |
| `docs/BATTALION.md` | `docs/src/user-guides/battalion-patterns.md` |
| `docs/GARRISON.md` | `docs/src/user-guides/garrison-memory.md` |
| `docs/SANCTUM.md` | `docs/src/user-guides/sanctum-vector-memory.md` |
| `docs/HERALD.md` | `docs/src/user-guides/herald-output.md` |
| `docs/AUTONOMOUS.md` | `docs/src/user-guides/paladin-agents.md` |
| `docs/COMMANDER.md` | `docs/src/user-guides/orchestration.md` *(partial)* |
| `docs/FEATURE_FLAGS.md` | `docs/src/api-reference/feature-flags.md` |
| `docs/MIGRATION.md` | `docs/src/api-reference/migration-guide.md` |
| `docs/VERSIONING_POLICY.md` | `docs/src/api-reference/stable-api.md` *(partial)* |
| `STABLE_API.md` | `docs/src/api-reference/stable-api.md` *(partial)* |
| `CONTRIBUTING.md` | `docs/src/contributing/development-setup.md` *(partial)* |
| `docs/SENTINEL.md` | `docs/src/operations/monitoring.md` *(partial)* |
| `docs/GROVE.md` | *(review in Epic 1 audit — may merge with garrison)* |
| `docs/COUNCIL.md` | `docs/src/user-guides/battalion-patterns.md` *(partial)* |
| `docs/MANEUVER.md` | `docs/src/user-guides/orchestration.md` *(partial)* |
| `docs/Design/` | `docs/src/architecture/` |
| `docs/architecture/` | `docs/src/architecture/` |
| `docs/deployment/` | `docs/src/deployment/` |
| `docs/operations/` | `docs/src/operations/` |
| `docs/contributing/` | `docs/src/contributing/` |
| `docs/guides/` | *(distribute per topic)* |

**Notes:**

- Files classified as **Delete** in the Epic 1 audit should not be migrated.
- Files that partially overlap a chapter should be merged or split — note this in a migration log.
- The original `docs/` flat files may be removed once migration is confirmed working (or retained with a deprecation notice until Epic 3 rewrites are complete).

**Deliverables:**

- All retained existing docs placed in MDBook structure.
- `SUMMARY.md` links to all migrated files.
- `mdbook build` succeeds with zero errors (content may be stale but structurally valid).
- A brief `docs/MIGRATION_LOG.md` noting any split/merge decisions.

---

## Deliverables Summary

| Artifact | Description |
|----------|-------------|
| `docs/book.toml` | MDBook configuration |
| `docs/src/SUMMARY.md` | Full chapter hierarchy / table of contents |
| `docs/src/**/*.md` | All chapter files (migrated or placeholder) |
| `.github/workflows/docs.yml` | CI build and deploy workflow |
| `docs/MIGRATION_LOG.md` | Notes on split/merge decisions during migration |

---

## Definition of Done

- [ ] `mdbook build docs/` completes locally with zero errors and zero warnings.
- [ ] All chapters in `SUMMARY.md` link to real files.
- [ ] CI workflow file is committed and a test run confirms the build step passes.
- [ ] GitHub Pages deployment is configured and the placeholder site is accessible.
- [ ] All existing docs (not flagged for deletion by Epic 1) appear somewhere in the new structure.

---

## Schedule Reference

| Phase | This Epic | Duration | Predecessors |
|-------|-----------|----------|-------------|
| Phase 1B | Epic 2: MDBook Setup | 0.5–1 sprint | None |

Runs **in parallel** with Epic 1 (Content Audit). Epic 3 and Epic 4 depend on this Epic completing first.
