## Relevant Files

- `docs/book.toml` - MDBook configuration file (FR-1, FR-2, FR-3)
- `docs/src/SUMMARY.md` - Chapter table of contents; single source of truth for MDBook navigation (FR-6, FR-7)
- `docs/src/introduction.md` - Introduction chapter (placeholder or migrated from `docs/README.md`)
- `docs/src/getting-started/` - Getting Started chapter files (migrated or placeholder)
- `docs/src/user-guides/` - User Guide chapter files (migrated or placeholder)
- `docs/src/architecture/` - Architecture chapter files (migrated from `docs/architecture/` and `docs/Design/`)
- `docs/src/deployment/` - Deployment chapter files (migrated from `docs/deployment/`)
- `docs/src/operations/` - Operations chapter files (migrated from `docs/operations/`)
- `docs/src/api-reference/` - API Reference chapter files (migrated or placeholder)
- `docs/src/contributing/` - Contributing chapter files (migrated from `docs/contributing/` and root `CONTRIBUTING.md`)
- `docs/src/appendix/` - Appendix chapter files (migrated uncategorized docs per FR-11)
- `docs/MIGRATION_LOG.md` - Record of all split/merge/appendix placement decisions (FR-18)
- `.github/workflows/docs.yml` - GitHub Actions CI build and GitHub Pages deploy workflow (FR-14, FR-15)
- `.gitignore` - Must include `docs/book/` entry (Technical Considerations)

### Notes

- This Epic adds **no Rust code**. Do not modify `Cargo.toml` or any `*.rs` files.
- All file migrations use `git mv` (not `cp`) to preserve git history.
- `mdbook build docs/` must complete with **zero errors and zero warnings** — this is the Definition of Done.
- Install MDBook tools locally before starting: `cargo install mdbook --version 0.4.40 --locked`, `cargo install mdbook-mermaid --version 0.13.0 --locked`, `cargo install mdbook-linkcheck --version 0.7.7 --locked`.
- After installing `mdbook-mermaid`, run `mdbook-mermaid install docs/` to inject JS assets into the theme directory — commit the resulting `docs/theme/` files.
- To verify the build: `mdbook build docs/` then `mdbook serve docs/` and open `http://localhost:3000`.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone-11-epic-2-mdbook-setup`

- [x] 1.0 Install MDBook toolchain locally
  - [x] 1.1 Install `mdbook` v0.4.40 via `cargo install mdbook --version 0.4.40 --locked`
  - [x] 1.2 Install `mdbook-mermaid` v0.13.0 via `cargo install mdbook-mermaid --version 0.13.0 --locked`
  - [x] 1.3 Install `mdbook-linkcheck` v0.7.7 via `cargo install mdbook-linkcheck --version 0.7.7 --locked`
  - [x] 1.4 Verify all three tools are on PATH: `mdbook --version`, `mdbook-mermaid --version`, `mdbook-linkcheck --version`

- [ ] 2.0 Create MDBook configuration
  - [ ] 2.1 Create `docs/book.toml` with the full configuration specified in FR-1 (book metadata, HTML output, fold settings, preprocessor declarations)
  - [ ] 2.2 Add the `[output.linkcheck]` section with `follow-web-links = false` to prevent flaky CI from network failures
  - [ ] 2.3 Run `mdbook-mermaid install docs/` to inject JavaScript assets into `docs/theme/`; verify `docs/theme/` files were created

- [ ] 3.0 Create docs/src/ chapter hierarchy and SUMMARY.md
  - [ ] 3.1 Create all chapter subdirectories: `docs/src/getting-started/`, `docs/src/user-guides/`, `docs/src/architecture/`, `docs/src/deployment/`, `docs/src/operations/`, `docs/src/api-reference/`, `docs/src/contributing/`, `docs/src/appendix/`
  - [ ] 3.2 Create `docs/src/SUMMARY.md` with the full chapter hierarchy linking every planned file (all sections: Introduction, Getting Started, User Guides, Architecture, Deployment, Operations, API Reference, Contributing, Appendix)
  - [ ] 3.3 Create `docs/src/introduction.md` — either migrate `docs/README.md` via `git mv` or create a placeholder; resolve OQ-1 at this step and record the decision in `MIGRATION_LOG.md`

- [ ] 4.0 Migrate existing docs into MDBook structure
  - [ ] 4.1 Migrate Getting Started files: `git mv docs/INSTALLATION.md docs/src/getting-started/installation.md`, `git mv docs/QUICKSTART.md docs/src/getting-started/quickstart.md`, `git mv docs/CONFIGURATION.md docs/src/getting-started/configuration.md`
  - [ ] 4.2 Migrate User Guide files: `git mv docs/ARSENAL.md docs/src/user-guides/arsenal-tools.md`, `git mv docs/BATTALION.md docs/src/user-guides/battalion-patterns.md`, `git mv docs/GARRISON.md docs/src/user-guides/garrison-memory.md`, `git mv docs/SANCTUM.md docs/src/user-guides/sanctum-vector-memory.md`, `git mv docs/HERALD.md docs/src/user-guides/herald-output.md`, `git mv docs/AUTONOMOUS.md docs/src/user-guides/paladin-agents.md`, `git mv docs/COMMANDER.md docs/src/user-guides/orchestration.md`
  - [ ] 4.3 Migrate API Reference files: `git mv docs/FEATURE_FLAGS.md docs/src/api-reference/feature-flags.md`, `git mv docs/MIGRATION.md docs/src/api-reference/migration-guide.md`
  - [ ] 4.4 Merge `docs/VERSIONING_POLICY.md` and `STABLE_API.md` into `docs/src/api-reference/stable-api.md` — concatenate with an H2 separator; record the merge in `docs/MIGRATION_LOG.md` (FR-10); resolve OQ-3 (CHANGELOG inclusion) at this step
  - [ ] 4.5 Migrate Contributing files: `git mv CONTRIBUTING.md docs/src/contributing/development-setup.md`, `git mv docs/contributing/testing-guide.md docs/src/contributing/testing-guide.md`, `git mv docs/contributing/adapter-development.md docs/src/contributing/architecture-decisions.md`
  - [ ] 4.6 Migrate Architecture files: resolve OQ-2 (which file becomes `overview.md`), then `git mv` the appropriate file to `docs/src/architecture/overview.md`, and move `docs/architecture/hexagonal-design.md`, `docs/architecture/domain-model.md`, `docs/architecture/design-patterns.md`, `docs/architecture/dependency-flow-diagrams.md` (→ `crate-map.md`)
  - [ ] 4.7 Migrate Deployment files: `git mv docs/deployment/docker.md docs/src/deployment/docker.md`, `git mv docs/deployment/kubernetes.md docs/src/deployment/kubernetes.md`, `git mv docs/deployment/production-best-practices.md docs/src/deployment/production.md`
  - [ ] 4.8 Migrate Operations files: `git mv docs/operations/logging.md docs/src/operations/logging.md`, `git mv docs/operations/monitoring.md docs/src/operations/monitoring.md`, `git mv docs/operations/performance-tuning.md docs/src/operations/performance-tuning.md`
  - [ ] 4.9 Migrate all remaining docs not yet moved into `docs/src/appendix/` using `git mv` (GROVE, COUNCIL, MANEUVER, SENTINEL, CONCLAVE if present, all benchmark/deployment/CLI/user-system files, and all remaining files in `docs/guides/`, `docs/cli/`, `docs/Design/`, `docs/contributing/` per FR-11); record every placement with a one-line reason in `docs/MIGRATION_LOG.md`

- [ ] 5.0 Create placeholder files for unmapped chapters
  - [ ] 5.1 For each chapter file in `SUMMARY.md` that has no migration source, create a placeholder with the required format: `# <Chapter Title>\n\n> Content coming in Epic 3.`
  - [ ] 5.2 Verify every file linked in `SUMMARY.md` exists on disk (`find docs/src -name "*.md" | sort` vs links in SUMMARY.md)

- [ ] 6.0 Update .gitignore
  - [ ] 6.1 Add `docs/book/` to `.gitignore` so the MDBook build output is never committed to the repository

- [ ] 7.0 Create MIGRATION_LOG.md
  - [ ] 7.1 Create `docs/MIGRATION_LOG.md` and ensure it documents: every file merged/split (with what content went where), every appendix placement decision (one-line reason each), and every file not migrated (with reason)
  - [ ] 7.2 Confirm the log is complete by cross-referencing the full file list from the Epic 1 audit (`project/Milestone_11-Documentation-Overhaul-Publish/Epic_1/docs-audit.md`) against entries in the log

- [ ] 8.0 Verify local MDBook build passes
  - [ ] 8.1 Run `mdbook build docs/` and confirm it exits with code 0, zero errors, and zero warnings
  - [ ] 8.2 Run `mdbook serve docs/` and manually verify: SUMMARY navigation renders, at least one Mermaid diagram renders as a diagram (not raw code), and `mdbook-linkcheck` reports zero broken internal links
  - [ ] 8.3 Fix any build errors or broken links before proceeding (content accuracy is not in scope, but structural validity is)

- [ ] 9.0 Create GitHub Actions CI/CD workflow
  - [ ] 9.1 Create `.github/workflows/docs.yml` with a `build` job triggered on PRs and pushes to `main` that touch `docs/**`; the job installs pinned versions of `mdbook` (0.4.40), `mdbook-mermaid`, and `mdbook-linkcheck`, then runs `mdbook build docs/`
  - [ ] 9.2 Add a `deploy` job to the same workflow that runs only on `push` to `main`; uses `peaceiris/actions-gh-pages@v3` to publish `docs/book/` to the `gh-pages` branch with `GITHUB_TOKEN`
  - [ ] 9.3 Confirm the workflow YAML is syntactically valid (e.g., `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/docs.yml'))"` or use `actionlint` if available)

- [ ] 10.0 Configure GitHub Pages and commit
  - [ ] 10.1 In the GitHub repository settings (`DF3NDR/paladin-dev-env` → Settings → Pages), set the source branch to `gh-pages` and directory to `/ (root)` — this is a one-time manual step (FR-17); verify OQ-4 (branch protection) is not blocking the `GITHUB_TOKEN` deploy
  - [ ] 10.2 Run `cargo test` to confirm no Rust regressions were introduced by the migration
  - [ ] 10.3 Stage all changes: `git add docs/ .github/workflows/docs.yml .gitignore`
  - [ ] 10.4 Commit with a descriptive message: `git commit -m "feat(milestone-11/epic-2): MDBook setup, chapter hierarchy, and doc migration" -m "- Creates docs/book.toml with mdbook-mermaid and mdbook-linkcheck preprocessors" -m "- Migrates all existing docs into docs/src/ chapter hierarchy via git mv" -m "- Adds appendix/ for uncategorized docs; MIGRATION_LOG.md records all decisions" -m "- Adds .github/workflows/docs.yml CI build and GitHub Pages deploy" -m "- docs/book/ added to .gitignore" -m "Milestone 11, Epic 2"`
