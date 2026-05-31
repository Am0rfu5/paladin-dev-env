# Milestone 10 — Epic 4: Finalization — Task List

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 4 of 4
**PRD:** `project/Milestone_10-CI-Hardening-Release-Automation/Epic_4/prd-milestone10-finalization.md`
**Target Version:** v0.4.0
**Status:** In Progress

---

## Tasks

### Task 0.0: Create feature branch

- [ ] Checkout a new branch `feature/milestone_10-epic_4-finalization` from the current
  `feature/milestone_10-epic_3-release-automation` branch so all Epic 3 tooling (hooks, CI,
  `make release`) is available.

```bash
cd /workspace && git checkout -b feature/milestone_10-epic_4-finalization
```

---

### Task 4.1: Update CONTRIBUTING.md

**Description:** Two concrete changes are needed:

1. **Fix ToC gap** — `## Releasing` was added in Epic 3 but is not listed in the Table of Contents.
   Add the entry between `[Documentation]` and `[API Change Process]`.
2. **Add "Adding a New Dependency" section** — Write a numbered step-by-step guide so contributors
   know exactly what to do when they want to add a new crate dependency: add to `Cargo.toml`, check
   license with `make deny`, check vulnerabilities with `make audit`, update `deny.toml` exceptions
   if needed, and update `CHANGELOG.md`. Also add this section to the ToC.

The following sections already exist and need **no changes**:

| Section | Added in | ToC entry present? |
|---------|----------|--------------------|
| Git Hooks (pre-commit) | Epic 1 | ✓ |
| Security (### subsection under Code Quality Standards) | Epic 2 | n/a — subsection |
| Releasing | Epic 3 | ✗ — **needs ToC entry** |

- [ ] Add `[Releasing](#releasing)` to the Table of Contents in `CONTRIBUTING.md` (between
  `[Documentation]` and `[API Change Process]`).
- [ ] Add `[Adding a New Dependency](#adding-a-new-dependency)` to the Table of Contents
  (between `[Releasing]` and `[API Change Process]`).
- [ ] Write the `## Adding a New Dependency` section in `CONTRIBUTING.md` (after the
  `## Releasing` section, before `## API Change Process`) covering:
  - Step 1: `cargo add <crate>` (or manually edit `Cargo.toml`)
  - Step 2: Run `make deny` — verify license is in the allowed list in `deny.toml`
  - Step 3: Run `make audit` — verify no new vulnerability advisories
  - Step 4: If `cargo-deny` rejects a license, check the license policy in `deny.toml`; open a
    discussion before adding a non-standard license exception
  - Step 5: If `make audit` reports a new unmaintained or vulnerability advisory, add a scoped
    `[advisories].ignore` entry in `deny.toml` **with an explanatory comment** and rerun `make audit`
  - Step 6: Update `CHANGELOG.md` `[Unreleased]` if the new dependency enables a user-visible change
  - Step 7: The CI `cargo-deny` and `security-audit` jobs are the final gate — do not merge with
    failing checks

**Acceptance criteria:**
- `CONTRIBUTING.md` ToC contains `Releasing` and `Adding a New Dependency` entries.
- `## Adding a New Dependency` section is present, clear, and numbered.
- No other existing text is modified.

---

### Task 4.2a: Populate CHANGELOG.md with Milestone 10 content

**Description:** Write the `[Unreleased]` block content documenting all Milestone 10 epics so it is
ready for the version-finalization step in Task 4.2b.

Current state: `CHANGELOG.md` `## [Unreleased]` section contains only Milestone 8 Epic 6 fix
entries.

Add subsections documenting:

**Epic 1 — CI Hardening: Pre-commit / Pre-push Hook Framework**
- Added `.pre-commit-config.yaml` with commit-stage hooks: `cargo fmt --check`, `cargo clippy`,
  `gitleaks` secret detection, TOML/YAML/JSON validation, large-file and merge-conflict checks,
  trailing-whitespace and end-of-file normalization.
- Added pre-push stage: `cargo build --workspace`, `cargo test --workspace --lib`.
- Added `make hooks` target to install both hook stages.
- Provisioned `pre-commit` (4.6.0) and `gitleaks` in `.devcontainer/Dockerfile.dev`.
- Normalized trailing-whitespace and end-of-file across all source files.

**Epic 2 — Dependency Security and License Compliance**
- Added `.cargo/audit.toml` exception list; `cargo audit` exits 0 with documented allowances.
- Added `deny.toml` with license allow-list (MIT/Apache-2.0/BSD-2-Clause/BSD-3-Clause/ISC/Zlib)
  and per-crate exceptions; advisory ignores mirror `audit.toml`.
- Added CycloneDX SBOM generation to `release.yml` (`make sbom` / `cargo cyclonedx --all`).
- Added OSV-Scanner annotate-only job in `ci.yml`.
- Added `make security`, `make audit`, `make deny`, `make sbom` Makefile targets.
- Added `docs/SECURITY_SCANNING.md` — full tooling overview, license policy, exception process.
- Updated `CONTRIBUTING.md` with Security subsection and cross-references.

**Epic 3 — Release Automation**
- Added `release.toml` with `shared-version = true`, `publish = false`, `push = false` for
  lockstep workspace versioning via `cargo release`.
- Added tag-triggered `publish-crates` job to `release.yml`: dependency-ordered publish
  (core → ports → leaf tier → facade), dry-run/skip modes, 20 s publish gaps.
- Added `workflow_dispatch` `dry_run` input to `release.yml`.
- Added `make release VERSION=` and `make publish-dry-run` Makefile targets.
- Added `docs/RELEASE_AUTOMATION.md` — tooling decision doc and operator guide.
- Updated `docs/RELEASE_CHECKLIST.md` with cross-reference to `RELEASE_AUTOMATION.md`.
- Updated `CONTRIBUTING.md` with `## Releasing` section.
- Provisioned `cargo-release` (1.1.2), `cargo-deny` (0.19.8), `cargo-cyclonedx` (0.5.9) in
  `.devcontainer/Dockerfile.dev` and `make setup`.

**Epic 4 — Finalization**
- Added "Adding a New Dependency" section to `CONTRIBUTING.md` with license/audit check workflow.
- Fixed `CONTRIBUTING.md` Table of Contents (added `Releasing` entry).
- Bumped all workspace crates from `0.3.0` to `0.4.0` in lockstep.

- [ ] Append the Milestone 10 Epic 1–4 `### Added` entries to the `## [Unreleased]` block in
  `CHANGELOG.md`.

**Acceptance criteria:**
- `CHANGELOG.md` `## [Unreleased]` block contains all four Epic summaries listed above.
- Existing Fixed entries (Milestone 8 Epic 6) remain intact beneath the new Added block.

---

### Task 4.2b: Run `make release VERSION=0.4.0`

**Description:** Execute the full automated release flow.

`make release VERSION=0.4.0` performs the following steps automatically:

1. Validates `0.4.0` is valid semver.
2. Runs `make release-check`: `make clean-code` (fmt + clippy), `make test-all`, `make audit`,
   `make build-release`.
3. Bumps all 10 workspace crates lockstep: `cargo release version 0.4.0 --execute --no-confirm --workspace`.
4. Finalizes `CHANGELOG.md`: replaces `## [Unreleased]` with `## [0.4.0] - 2026-05-31`.
5. Stages all changed files with `git add -u`.
6. Commits: `chore(release): version 0.4.0`.
7. Creates annotated tag `v0.4.0`.
8. Pushes branch and tag — triggering the Epic 3 CI pipeline.

- [ ] Verify `cargo-release` is installed: `cargo release --version` (expect `1.1.2`).
- [ ] Run `make release VERSION=0.4.0` — monitor output for errors.
- [ ] Verify the tag was created: `git tag -l "v0.4.0"`.
- [ ] Verify all Cargo.toml version fields are `0.4.0`: `grep -r '"0\.3\.0"' Cargo.toml crates/*/Cargo.toml` should return empty.
- [ ] Confirm push succeeded and tag appears on remote: `git log --oneline -3`.

**Acceptance criteria:**
- All quality gates pass during `make release-check`.
- All workspace crate versions are `0.4.0`.
- `v0.4.0` annotated tag exists locally and on remote.
- `CHANGELOG.md` contains `## [0.4.0] - 2026-05-31` (not `## [Unreleased]`).

---

### Task 4.3: Conformance gate and task file update

**Description:** Verify the final state of the branch and mark the task file complete.

- [ ] Run `cargo fmt --check` — must pass.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings` — must pass.
- [ ] Run `cargo test --workspace` — must pass.
- [ ] Run `cargo audit` — must exit 0.
- [ ] Run `pre-commit run --all-files` — must pass (or only auto-fixed files if hooks self-fix).
- [ ] Mark all subtasks `[x]` in this file.
- [ ] Commit this task file.

---

## Definition of Done

- [ ] `CONTRIBUTING.md` Table of Contents includes `Releasing` and `Adding a New Dependency`.
- [ ] `CONTRIBUTING.md` contains the `## Adding a New Dependency` section with numbered steps.
- [ ] `CHANGELOG.md` `## [0.4.0] - 2026-05-31` entry covers all Milestone 10 epics.
- [ ] All workspace crates are at version `0.4.0` (no stale `0.3.0` references).
- [ ] Annotated tag `v0.4.0` exists and has been pushed to remote.
- [ ] All conformance gates pass: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`.

---

## Relevant Files

| File | Purpose |
|------|---------|
| `CONTRIBUTING.md` | Contributor guide — ToC fix + new dependency section |
| `CHANGELOG.md` | Project changelog — M10 entries + v0.4.0 finalization |
| `Cargo.toml` | Workspace root — version bump 0.3.0 → 0.4.0 |
| `crates/paladin-core/Cargo.toml` | paladin-ai-core — version bump |
| `crates/paladin-ports/Cargo.toml` | paladin-ports — version bump |
| `crates/paladin-battalion/Cargo.toml` | paladin-battalion — version bump |
| `crates/paladin-llm/Cargo.toml` | paladin-llm — version bump |
| `crates/paladin-memory/Cargo.toml` | paladin-memory — version bump |
| `crates/paladin-storage/Cargo.toml` | paladin-storage — version bump |
| `crates/paladin-notifications/Cargo.toml` | paladin-notifications — version bump |
| `crates/paladin-content/Cargo.toml` | paladin-content — version bump |
| `crates/paladin-web/Cargo.toml` | paladin-web — version bump |
| `project/Milestone_10-CI-Hardening-Release-Automation/Epic_4/prd-milestone10-finalization.md` | PRD |
| `project/Milestone_10-CI-Hardening-Release-Automation/Epic_4/tasks-finalization.md` | This file |
