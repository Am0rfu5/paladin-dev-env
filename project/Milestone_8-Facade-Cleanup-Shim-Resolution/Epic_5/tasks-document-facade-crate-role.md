## Relevant Files

- `src/lib.rs` — Extend `//!` doc comment with `## Facade Crate Role` section; update `## Architecture` description.
- `src/README.md` — New file: facade crate role prose + "What lives here" table.
- `STABLE_API.md` — Comprehensive audit and update: header metadata, stale paths, catalog entries.
- `CHANGELOG.md` — Promote `[Unreleased]` → `## [0.2.0] - 2026-05-30`; add `### Changed`; reset `[Unreleased]`.
- `api_surface_current.txt` — Regenerate as v0.2.0 baseline.
- `final-api.txt` — Update to v0.2.0 snapshot.
- `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_5/tasks-document-facade-crate-role.md` — This task file.

### Notes

- All work is on the existing branch `feature/milestone_8-epic_4-use_cases-services-rename`. No new branch needed.
- This Epic is **documentation-only** — no Rust logic is added or modified.
- Run `cargo fmt --all -- --check` and `cargo clippy --workspace -- -D warnings` before each commit (pre-commit hook enforces `cargo check` + `cargo fmt --check` automatically).
- Commit messages must use conventional commit format with `-m` flags. No `!` characters (bash history expansion guard).
- Check `STABLE_API.md` §Automated Tracking and the `Makefile` for the API surface extraction command before Task 4.0.
- `src/lib.rs.backup` must be deleted before the final quality gate (it is not a Rust module and should not be committed).

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [x] 0.0 Confirm active branch and resolve pre-flight open questions
  - [x] 0.1 Confirm the working branch is `feature/milestone_8-epic_4-use_cases-services-rename`: `git branch --show-current`. No new branch is needed — Epic 5 continues on the same branch.
  - [x] 0.2 Confirm `cargo-public-api` is available: `which cargo-public-api` — it is installed at `/usr/local/cargo/bin/cargo-public-api`.
  - [x] 0.3 Confirm the `v0.1.0` tag exists for the CHANGELOG compare URL: `git tag` — tag `v0.1.0-rc.1` exists; use the existing link-reference pattern already in `CHANGELOG.md` (`https://github.com/jamatulli/paladin/compare/...`).
  - [x] 0.4 Confirm `src/lib.rs.backup` exists and schedule it for deletion in Task 6: `ls src/lib.rs.backup`.
  - [x] 0.5 Note: `STABLE_API.md` has **zero** `use_cases` references (already clean from Epic 4). The audit in Task 3 only needs to update the header metadata and facade crate section.

- [x] 1.0 Update `src/lib.rs` facade crate documentation
  - [x] 1.1 Read the current `//!` block in `src/lib.rs` (lines 1–80) to understand the exact existing text before editing.
  - [x] 1.2 In `src/lib.rs`, update the `## Architecture` section: change the `Application Layer` bullet from `"Use cases and port trait definitions"` to `"Application services and coordination logic"` (FR-3).
  - [x] 1.3 Insert a new `## Facade Crate Role` section into the `//!` block **between** the `## Architecture` section and the `## Stable Public API` section. The new section must contain (FR-1):
    - A paragraph explaining this crate is the **application assembly point and composition root**.
    - What the facade **contains**: `ServiceRunner` (composition root), application-layer coordination services (`src/application/services/`), configuration loading (`src/config/`), CLI modules (`src/application/cli/`, `cli` feature-gated), and binary entry points (`main.rs`, `bin/paladin-cli.rs`).
    - What the facade does **not** contain: business logic, port trait definitions, or infrastructure adapter implementations (those live in the leaf crates).
    - A bulleted list of the 9 leaf crates: `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-notifications`, `paladin-storage`, `paladin-content`, `paladin-web`.
  - [x] 1.4 Run `cargo fmt` then `cargo fmt --all -- --check` to confirm the `lib.rs` edit has no formatting drift.
  - [x] 1.5 Run `cargo doc --workspace --no-deps 2>&1 | tail -5` — confirm exit 0 (warnings OK, must not error).
  - [x] 1.6 Commit: `git add src/lib.rs && git commit -m "docs(m8-e5): add Facade Crate Role section to lib.rs doc comment" -m "- New ## Facade Crate Role section between ## Architecture and ## Stable Public API" -m "- Lists what facade contains (ServiceRunner, services, config, cli, binaries)" -m "- Lists what facade does not contain (business logic, port traits, adapters)" -m "- Lists all 9 leaf crates" -m "- Updated Application Layer description: use cases -> application services"`

- [x] 2.0 Create `src/README.md` facade crate reference file
  - [x] 2.1 Create `src/README.md` with the heading `# Paladin Facade Crate` (FR-2).
  - [x] 2.2 Add a prose section explaining the facade-crate role: application assembly point, composition root, `ServiceRunner`, application services, CLI, binaries, dependency-flow rule (facade → leaf crates; one direction only).
  - [x] 2.3 Add the "What lives here" table with columns **Path**, **Purpose**, **Notes**, covering at minimum these 7 rows:
    - `src/application/services/` — Application coordination services — 11 sub-modules, 39 `.rs` files
    - `src/application/cli/` — CLI command implementations — Feature-gated (`cli` flag)
    - `src/config/` — Configuration loading & settings types — Composition root needs config
    - `src/infrastructure/` — Infrastructure adapter implementations — Adapters not extracted to leaf crates
    - `src/core/` — Re-export bridge to `paladin-core` — Minimal structure; real logic in `paladin-core`
    - `src/bin/` — Binary entry points — `paladin-cli.rs` (feature-gated)
    - `src/main.rs` — Default binary entry point — Thin wrapper; bootstraps `ServiceRunner`
  - [x] 2.4 Add a "Leaf Crates" section listing the 9 leaf crates with a one-line description each, and a note that leaf crates must not import from the facade.
  - [x] 2.5 Add a footer line: "See [STABLE_API.md](../STABLE_API.md) for the public API contract."
  - [x] 2.6 Commit: `git add src/README.md && git commit -m "docs(m8-e5): create src/README.md documenting facade crate role" -m "- Assembly point and composition root description" -m "- What lives here table: 7 paths with purpose and notes" -m "- Leaf crate list with dependency-flow rule" -m "- Reference to STABLE_API.md"`

- [x] 3.0 Audit and update `STABLE_API.md`
  - [x] 3.1 Read the `STABLE_API.md` header block (lines 1–15) to see the current `Version`, `Last Updated`, and `Epic` fields, and the breaking-change callout box.
  - [x] 3.2 Update the header block (FR-4):
    - `Last Updated:` → `2026-05-30`
    - `Epic:` → `Milestone 8, Epic 5 - Document Facade Crate Role and Finalize`
    - Update the breaking-change callout box to mention **both** the shim removals (Epic 2/3) **and** the `use_cases` → `services` rename (Epic 4) as the v0.2.0 breaking changes.
  - [x] 3.3 Read the `### paladin (facade crate)` section (around line 271) and update it to reflect the post-Milestone-8 module layout (FR-5): remove any references to `application/ports/`, `application/storage/`, or `application/use_cases/`; add `application/services/`.
  - [x] 3.4 Verify `grep "use_cases\|application::storage\|application::ports" STABLE_API.md` returns 0 hits (catalog is already clean; this confirms no regressions from the header edit).
  - [x] 3.5 Read the `## Tracking API Changes` section (around line 296–355) and update any stale baseline dates or file references to note `api_surface_current.txt` as the v0.2.0 baseline (FR-8).
  - [x] 3.6 Commit: `git add STABLE_API.md && git commit -m "docs(m8-e5): update STABLE_API.md for v0.2.0 release" -m "- Updated Last Updated to 2026-05-30" -m "- Updated Epic to Milestone 8 Epic 5" -m "- Updated breaking-change callout to cover Epic 2/3 shim removals and Epic 4 rename" -m "- Updated paladin facade crate section for post-M8 module layout" -m "- Updated Tracking section to reference v0.2.0 baseline"`

- [ ] 4.0 Promote `CHANGELOG.md` `[Unreleased]` to `[0.2.0]`
  - [ ] 4.1 Read the current `[Unreleased]` block in `CHANGELOG.md` (lines 1–70) to see the exact existing sub-sections (`### Breaking Changes`, `### Added`, `### Removed`) before editing.
  - [ ] 4.2 Insert a new empty `## [Unreleased]` section at the top (above the existing block) for future changes (FR-9).
  - [ ] 4.3 Rename the existing block header from `## [Unreleased]` to `## [0.2.0] - 2026-05-30` (FR-9).
  - [ ] 4.4 Add a `### Changed` sub-section to the `## [0.2.0]` block (between `### Added` and `### Removed`) containing (FR-11): "Documented facade crate role as application assembly point; added `src/README.md` and updated `src/lib.rs` `//!` docs with `## Facade Crate Role` section."
  - [ ] 4.5 At the bottom of `CHANGELOG.md`, update the link-reference block (FR-12):
    - Change `[Unreleased]: https://github.com/jamatulli/paladin/compare/v0.1.0...HEAD` to `[Unreleased]: https://github.com/jamatulli/paladin/compare/v0.2.0...HEAD`
    - Add a new line: `[0.2.0]: https://github.com/jamatulli/paladin/compare/v0.1.0...v0.2.0`
    - Keep the existing `[0.1.0]:` line.
  - [ ] 4.6 Verify structure: `grep -n "^\#\# \[" CHANGELOG.md` — must show `[Unreleased]` above `[0.2.0] - 2026-05-30` above `[0.1.0]`.
  - [ ] 4.7 Commit: `git add CHANGELOG.md && git commit -m "docs(m8-e5): promote Unreleased to 0.2.0 in CHANGELOG" -m "- Promoted [Unreleased] to ## [0.2.0] - 2026-05-30" -m "- Added ### Changed entry for facade crate documentation" -m "- Reset [Unreleased] section above 0.2.0" -m "- Updated link-reference block: Unreleased now compares v0.2.0...HEAD" -m "- Added [0.2.0] compare link"`

- [ ] 5.0 Regenerate API surface baseline files
  - [ ] 5.1 Check the `Makefile` for the exact `cargo-public-api` invocation used to generate `api_surface_current.txt`: `grep -A3 "public-api\|api_surface\|final-api" Makefile | head -30`.
  - [ ] 5.2 Run the API surface extraction command (from the Makefile or directly: `cargo public-api 2>/dev/null | head -5` to confirm it works). If the Makefile has a target, use `make <target>`.
  - [ ] 5.3 Regenerate `api_surface_current.txt` with a v0.2.0 header comment (FR-13). If the file already has a header comment, update the version and date to `v0.2.0 baseline — 2026-05-30`.
  - [ ] 5.4 Regenerate `final-api.txt` using the same method, updating its header to `v0.2.0 baseline — 2026-05-30` (FR-14).
  - [ ] 5.5 Commit: `git add api_surface_current.txt final-api.txt && git commit -m "chore(m8-e5): regenerate API surface baseline files for v0.2.0" -m "- api_surface_current.txt: v0.2.0 baseline 2026-05-30" -m "- final-api.txt: v0.2.0 baseline 2026-05-30"`

- [ ] 6.0 Final quality gate and epic close
  - [ ] 6.1 Delete `src/lib.rs.backup` (it is not a Rust module and must not be in the committed workspace): `rm src/lib.rs.backup`.
  - [ ] 6.2 Run `cargo build --workspace` — confirm exit 0, zero errors (FR-15).
  - [ ] 6.3 Run `cargo test --workspace` — confirm all tests pass, zero failures (FR-16).
  - [ ] 6.4 Run `cargo clippy --workspace -- -D warnings` — confirm zero warnings (FR-17).
  - [ ] 6.5 Run `cargo fmt --all -- --check` — confirm exit 0, no formatting drift (FR-18).
  - [ ] 6.6 Run `cargo doc --workspace --no-deps 2>&1 | tail -5` — confirm exit 0 (warnings acceptable) (FR-19).
  - [ ] 6.7 Verify success metrics:
    - `grep "Facade Crate Role" src/lib.rs` — 1 hit
    - `test -f src/README.md && echo exists` — exists
    - `grep "Last Updated" STABLE_API.md` — shows `2026-05-30`
    - `grep "^\#\# \[0.2.0\]" CHANGELOG.md` — 1 hit
    - `grep "^\#\# \[Unreleased\]" CHANGELOG.md` — 1 hit (the empty reset section)
  - [ ] 6.8 Mark all tasks `[x]` in this file.
  - [ ] 6.9 Commit: `git add -A && git commit -m "chore(m8-e5): final quality gate pass and Epic 5 close" -m "- Deleted src/lib.rs.backup" -m "- Build, test, clippy, fmt, doc all exit 0" -m "- All success metrics verified" -m "- Tasks 0.0-6.0 marked complete" -m "- Milestone 8 Epic 5 complete: v0.2.0 release candidate ready"`
