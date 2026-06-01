# PRD: MDBook Setup and Structure

**Project:** Paladin Framework
**Milestone:** 11 — Documentation Review, Reorganization, MDBook Publish
**Epic:** 2 — MDBook Setup and Structure
**Version Target:** v0.5.0
**Status:** Ready for Implementation
**Created:** 2026-06-01

---

## 1. Introduction / Overview

The Paladin framework has accumulated approximately 124 markdown documentation files across eight milestones of development. These files currently live in a flat `docs/` directory and have never been published as a structured, navigable reference. Users and contributors have no single entry point for learning the framework.

This Epic installs the scaffolding that turns those files into a publishable book. It covers:

- Creating the `docs/book.toml` configuration and the `docs/src/` chapter hierarchy.
- Migrating all retained existing documentation (using `git mv`) into the new structure.
- Adding preprocessors for link validation (`mdbook-linkcheck`) and diagram rendering (`mdbook-mermaid`).
- Setting up a GitHub Actions CI workflow that verifies the build on every PR touching `docs/` and deploys to GitHub Pages on merge to `main`.
- Configuring GitHub Pages on the repository for the first time (source: `gh-pages` branch).

Content accuracy is **not** in scope — that is Epic 3 (Rewrite) and Epic 4 (New Content). This Epic's only goal is: `mdbook build docs/` succeeds with zero errors and zero warnings, and the resulting site is deployed.

---

## 2. Goals

1. `mdbook build docs/` completes with zero errors and zero warnings on the developer's local machine.
2. Every chapter in `SUMMARY.md` links to a real file; no dangling links.
3. `mdbook-linkcheck` passes (no broken internal links).
4. `mdbook-mermaid` renders architecture diagrams present in migrated files.
5. A GitHub Actions workflow (`docs.yml`) builds the book on every PR that touches `docs/**`.
6. On merge to `main`, the same workflow deploys the built site to GitHub Pages.
7. The GitHub Pages site is accessible at `https://df3ndr.github.io/paladin-dev-env/`.
8. All existing docs not flagged for deletion by the Epic 1 audit are present somewhere in the new structure.
9. Docs with no single-chapter home are placed in an `appendix/` chapter rather than dropped.

---

## 3. User Stories

**As a framework user**, I want a single URL where I can browse all Paladin documentation in a structured, searchable format, so I can find what I need without opening individual GitHub files.

**As a contributor**, I want every PR that touches `docs/` to fail CI if `mdbook build` breaks, so documentation regressions are caught before merge.

**As a new developer onboarding to the project**, I want the docs to have a clear Getting Started → User Guides → Architecture → API Reference hierarchy, so I know where to look for each type of information.

**As a developer writing docs in Epic 3**, I want a confirmed directory structure and a working build, so I can add or edit content without touching the scaffolding.

---

## 4. Functional Requirements

### 4.1 MDBook Configuration

**FR-1.** Create `docs/book.toml` with the following mandatory fields:

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

[preprocessor.mermaid]
command = "mdbook-mermaid"

[preprocessor.links]
```

**FR-2.** `mdbook-linkcheck` must be declared in `book.toml` and must pass with zero broken internal links. External link checking may be disabled if it causes flaky CI (network-dependent).

**FR-3.** `mdbook-mermaid` must be declared in `book.toml` and must successfully render any Mermaid fenced code blocks present in migrated files. Use `mdbook-mermaid install docs/` to inject the required JavaScript assets.

---

### 4.2 Chapter Hierarchy

**FR-4.** Create the following directory and file structure under `docs/src/`. Files that have a migration source (see §4.4) are populated by migration; all others are placeholders.

```
docs/src/
├── SUMMARY.md
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
├── contributing/
│   ├── development-setup.md
│   ├── testing-guide.md
│   └── architecture-decisions.md
└── appendix/
    └── (additional files that have no single chapter home — see §4.5)
```

**FR-5.** Every file listed in `SUMMARY.md` must exist on disk before the first `mdbook build` is run.

---

### 4.3 SUMMARY.md

**FR-6.** `docs/src/SUMMARY.md` must list every chapter file in the structure using MDBook's required format:

```markdown
# Summary

- [Introduction](introduction.md)

## Getting Started
- [Installation](getting-started/installation.md)
- [Quickstart](getting-started/quickstart.md)
- [Configuration](getting-started/configuration.md)

## User Guides
- [Paladin Agents](user-guides/paladin-agents.md)
...
```

**FR-7.** Every chapter must be reachable from `SUMMARY.md` — no orphan files.

---

### 4.4 Migration of Existing Docs

**FR-8.** Existing docs must be migrated using `git mv` (not copy) so that git history is preserved. The original flat paths under `docs/` must no longer exist after migration.

**FR-9.** The following migration mapping must be applied exactly. Files flagged as **Stale** in the Epic 1 audit (`docs-audit.md`) are migrated as-is — content is fixed in Epic 3, not here.

| Source (existing) | Destination (MDBook chapter) |
|---|---|
| `docs/INSTALLATION.md` | `docs/src/getting-started/installation.md` |
| `docs/QUICKSTART.md` | `docs/src/getting-started/quickstart.md` |
| `docs/CONFIGURATION.md` | `docs/src/getting-started/configuration.md` |
| `docs/ARSENAL.md` | `docs/src/user-guides/arsenal-tools.md` |
| `docs/BATTALION.md` | `docs/src/user-guides/battalion-patterns.md` |
| `docs/GARRISON.md` | `docs/src/user-guides/garrison-memory.md` |
| `docs/SANCTUM.md` | `docs/src/user-guides/sanctum-vector-memory.md` |
| `docs/HERALD.md` | `docs/src/user-guides/herald-output.md` |
| `docs/AUTONOMOUS.md` | `docs/src/user-guides/paladin-agents.md` |
| `docs/COMMANDER.md` | `docs/src/user-guides/orchestration.md` |
| `docs/FEATURE_FLAGS.md` | `docs/src/api-reference/feature-flags.md` |
| `docs/MIGRATION.md` | `docs/src/api-reference/migration-guide.md` |
| `docs/VERSIONING_POLICY.md` | `docs/src/api-reference/stable-api.md` |
| `STABLE_API.md` | `docs/src/api-reference/stable-api.md` *(append or merge — see FR-10)* |
| `CONTRIBUTING.md` | `docs/src/contributing/development-setup.md` |
| `docs/Design/Design_and_Architecture.md` | `docs/src/architecture/overview.md` |
| `docs/architecture/hexagonal-design.md` | `docs/src/architecture/hexagonal-design.md` |
| `docs/architecture/domain-model.md` | `docs/src/architecture/domain-model.md` |
| `docs/architecture/design-patterns.md` | `docs/src/architecture/design-patterns.md` |
| `docs/architecture/dependency-flow-diagrams.md` | `docs/src/architecture/crate-map.md` |
| `docs/deployment/docker.md` | `docs/src/deployment/docker.md` |
| `docs/deployment/kubernetes.md` | `docs/src/deployment/kubernetes.md` |
| `docs/deployment/production-best-practices.md` | `docs/src/deployment/production.md` |
| `docs/operations/logging.md` | `docs/src/operations/logging.md` |
| `docs/operations/monitoring.md` | `docs/src/operations/monitoring.md` |
| `docs/operations/performance-tuning.md` | `docs/src/operations/performance-tuning.md` |
| `docs/contributing/testing-guide.md` | `docs/src/contributing/testing-guide.md` |
| `docs/contributing/adapter-development.md` | `docs/src/contributing/architecture-decisions.md` |

**FR-10.** When two source files map to the same destination (e.g., `STABLE_API.md` and `docs/VERSIONING_POLICY.md` both → `stable-api.md`), concatenate them in the destination file with an H2 separator and record the decision in `docs/MIGRATION_LOG.md`. Do not silently discard either file's content.

---

### 4.5 Appendix Chapter for Uncategorized Docs

**FR-11.** All existing docs that have no mapping in §4.4 must be placed in `docs/src/appendix/` rather than omitted. Each file keeps its original filename (lowercased, `.md` extension). Add each file to `SUMMARY.md` under an `## Appendix` section.

Files that must land in `appendix/` include (but are not limited to):

- `docs/GROVE.md` → `docs/src/appendix/grove.md`
- `docs/COUNCIL.md` → `docs/src/appendix/council.md`
- `docs/CONCLAVE.md` (if present) → `docs/src/appendix/conclave.md`
- `docs/MANEUVER.md` → `docs/src/appendix/maneuver.md`
- `docs/SENTINEL.md` → `docs/src/appendix/sentinel.md`
- `docs/AUTONOMOUS.md` (if not already moved to user-guides) → `docs/src/appendix/autonomous.md`
- `docs/COMMANDER.md` (if partially moved) → content remainder in `appendix/commander-reference.md`
- `docs/BATTALION_BENCHMARKS.md` → `docs/src/appendix/battalion-benchmarks.md`
- `docs/SANCTUM_BENCHMARKS.md` → `docs/src/appendix/sanctum-benchmarks.md`
- `docs/SANCTUM_MIGRATION.md` → `docs/src/appendix/sanctum-migration.md`
- `docs/SANCTUM_DEPLOYMENT.md` → `docs/src/appendix/sanctum-deployment.md`
- `docs/BATTALION_VISION_SUPPORT.md` → `docs/src/appendix/battalion-vision.md`
- `docs/PROVIDER_EXPANSION.md` → `docs/src/appendix/provider-expansion.md`
- `docs/CONTRIBUTING_PROVIDERS.md` → `docs/src/appendix/contributing-providers.md`
- `docs/CLI_USAGE.md` → `docs/src/appendix/cli-usage.md`
- `docs/USER_SYSTEM.md` → `docs/src/appendix/user-system.md`
- `docs/user_rest_api_usage.md` → `docs/src/appendix/rest-api-usage.md`
- `docs/HERALD.md` *(if already mapped, skip)*
- Any remaining files in `docs/guides/`, `docs/cli/`, `docs/Design/`, `docs/contributing/`, `docs/operations/`, `docs/deployment/` not explicitly mapped above

**FR-12.** Record every appendix placement decision in `docs/MIGRATION_LOG.md` with a one-line reason.

---

### 4.6 Placeholder Files

**FR-13.** Any chapter file in the target structure that has no migration source must be created as a placeholder with this exact content:

```markdown
# <Chapter Title>

> Content coming in Epic 3.
```

Where `<Chapter Title>` is the human-readable title matching the SUMMARY.md link text for that chapter.

---

### 4.7 GitHub Actions CI Workflow

**FR-14.** Create `.github/workflows/docs.yml` with the following behavior:

- **Trigger (PR):** Run on any pull request that changes a file matching `docs/**`. Does **not** trigger on Rust source file changes.
- **Trigger (push):** Run on push to `main` when files matching `docs/**` change.
- **Build job:** Installs `mdbook` (pinned version `0.4.40`), `mdbook-mermaid`, and `mdbook-linkcheck`. Runs `mdbook build docs/`. Fails the job if the exit code is non-zero.
- **Deploy job:** Runs only on `push` to `main` (not on PRs). Uses `peaceiris/actions-gh-pages@v3` to publish `docs/book/` to the `gh-pages` branch.

**FR-15.** Both `mdbook-mermaid` and `mdbook-linkcheck` versions must be pinned (not `latest`) in the workflow to prevent silent breakage.

**FR-16.** The workflow must pass the build step on the initial skeleton (placeholder files + migrated content) before the Epic is considered complete.

---

### 4.8 GitHub Pages Repository Configuration

**FR-17.** Configure GitHub Pages on the `DF3NDR/paladin-dev-env` repository:
- Source: `gh-pages` branch
- Directory: `/ (root)`
- The deployed site must be accessible at `https://df3ndr.github.io/paladin-dev-env/`

> **Note for implementer:** This is a one-time repository settings change. Go to **Settings → Pages → Source** and set branch to `gh-pages`, folder to `/ (root)`. The `gh-pages` branch is created automatically by the deploy action on first run.

---

### 4.9 Migration Log

**FR-18.** Create `docs/MIGRATION_LOG.md` during the migration task. It must record:
- Every file that was merged/split (and what content went where)
- Every file that was placed in `appendix/` and why
- Every file that was **not** migrated (if any) and the reason (e.g., "flagged Delete in Epic 1 audit — no such files found")

---

## 5. Non-Goals (Out of Scope)

- **Content accuracy** — Stale API paths, wrong version strings, broken Rust examples are Epic 3's responsibility. This Epic migrates content as-is.
- **Rewriting any documentation** — No content changes beyond structural/placeholder work.
- **`cargo test --doc` in this workflow** — Rust source CI is a separate concern; this workflow triggers on `docs/**` only.
- **MDBook plugins beyond `mdbook-mermaid` and `mdbook-linkcheck`** — No additional preprocessors (search, i18n, etc.) in this Epic.
- **Custom CSS/theme** — Default MDBook theme only.
- **Versioned docs** — Single-version site only; no `mdbook-versioning` or multi-version support.
- **Removing the old flat `docs/` structure from git history** — Files are moved (git mv), history is preserved; no history rewriting.

---

## 6. Design Considerations

### Directory placement

The MDBook lives at `docs/` within the monorepo (not a separate repo or submodule). This means:
- `docs/book.toml` is the MDBook root
- `docs/src/` contains all chapter source files
- `docs/book/` is the build output (must be added to `.gitignore`)
- The old flat files (`docs/ARSENAL.md`, etc.) are removed by the migration (`git mv`)

### SUMMARY.md is the MDBook's single source of truth for navigation

MDBook renders only files listed in `SUMMARY.md`. Any file not listed there is invisible to readers even if it exists on disk. All migrated and placeholder files must be listed.

### `mdbook-linkcheck` scope

Configure `mdbook-linkcheck` to skip external URL checking (to avoid flaky CI from network issues). Only internal `[link](./path.md)` style links must pass. Add to `book.toml`:

```toml
[output.linkcheck]
follow-web-links = false
```

### Mermaid diagram support

Several architecture docs contain ` ```mermaid ` fenced blocks. Without `mdbook-mermaid`, these render as raw code blocks. Run `mdbook-mermaid install docs/` once after installing the tool to inject the required JS into the theme — this modifies `docs/theme/` files which should be committed.

---

## 7. Technical Considerations

- **mdbook version:** Pin to `0.4.40` in CI. Install locally with `cargo install mdbook --version 0.4.40 --locked` or download the pre-built binary (faster CI).
- **mdbook-mermaid version:** Pin to `0.13.0` or the latest stable at time of implementation.
- **mdbook-linkcheck version:** Pin to `0.7.7` or the latest stable at time of implementation. Note: `mdbook-linkcheck` is a separate binary, not a Cargo dependency of the workspace — install it independently in CI.
- **`.gitignore`:** Add `docs/book/` to `.gitignore` so the build output is never committed to `main`.
- **`gh-pages` branch:** Created automatically by `peaceiris/actions-gh-pages@v3` on first successful deploy. No manual branch creation required.
- **Merge conflicts:** The migration moves many files. This Epic should be on its own branch (`feature/milestone-11-epic-2-mdbook-setup`) and merged to `develop` via a single PR to minimize conflicts with Epic 1 (which only touches `project/` files, not `docs/`).
- **No workspace Cargo changes** — This Epic adds no Rust code and should not touch `Cargo.toml` or any `*.rs` files.

---

## 8. Success Metrics

| Metric | Acceptance Criterion |
|---|---|
| Local build | `mdbook build docs/` exits 0 with zero errors and zero warnings |
| Link validation | `mdbook-linkcheck` passes with zero broken internal links |
| Diagram rendering | All Mermaid blocks in migrated files render as diagrams (not raw code) |
| CI build | GitHub Actions `docs.yml` build step passes on the implementing PR |
| Deployment | Merged to `main` → site is accessible at `https://df3ndr.github.io/paladin-dev-env/` |
| Chapter coverage | All 124 audited files (minus any flagged Delete — currently 0) appear somewhere in the MDBook structure |
| Migration log | `docs/MIGRATION_LOG.md` exists and documents every non-trivial placement decision |
| No regressions | `cargo test` still passes after the migration (no Rust files modified) |

---

## 9. Open Questions

| # | Question | Impact if Unresolved |
|---|---|---|
| OQ-1 | Should `docs/README.md` (the current flat-docs index) become `docs/src/introduction.md`, or should `introduction.md` be a fresh placeholder? | Minor — affects 1 file placement |
| OQ-2 | The `docs/architecture/overview.md` file is already named `overview.md` but the migration table maps `docs/Design/Design_and_Architecture.md` to `docs/src/architecture/overview.md`. Which file takes priority? | Affects merge/concatenation decision in FR-10 |
| OQ-3 | Should `CHANGELOG.md` (root level) appear anywhere in the MDBook, or is it intentionally excluded? | If included, needs a chapter slot in SUMMARY.md |
| OQ-4 | The `deploy` job requires `GITHUB_TOKEN` write permissions for Pages. Confirm that branch protection on `gh-pages` does not block the `peaceiris/actions-gh-pages` action. | Blocks deployment until resolved |
