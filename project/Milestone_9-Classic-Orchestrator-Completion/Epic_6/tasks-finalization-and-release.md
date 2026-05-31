# Tasks: Milestone 9 — Epic 6: Finalization and Release

**PRD:** `prd-finalization-and-release.md`
**Source Epic:** `Milestone_9-Epic_6-finalization-and-release.md`
**Version Target:** v0.3.0

## Relevant Files

- `Cargo.toml` (root) - Root crate `version`, `[workspace.dependencies]` internal crate version pins; bump to `0.3.0`.
- `crates/paladin-core/Cargo.toml` - `paladin-ai-core` crate version; bump to `0.3.0`.
- `crates/paladin-ports/Cargo.toml` - `paladin-ports` crate version; bump to `0.3.0`.
- `crates/paladin-battalion/Cargo.toml` - crate version; bump to `0.3.0`.
- `crates/paladin-content/Cargo.toml` - crate version; bump to `0.3.0`.
- `crates/paladin-llm/Cargo.toml` - crate version; bump to `0.3.0`.
- `crates/paladin-memory/Cargo.toml` - crate version; bump to `0.3.0`.
- `crates/paladin-notifications/Cargo.toml` - crate version; bump to `0.3.0`.
- `crates/paladin-storage/Cargo.toml` - crate version; bump to `0.3.0`.
- `crates/paladin-web/Cargo.toml` - crate version; bump to `0.3.0`.
- `Cargo.lock` - Regenerated/updated to reflect the new `0.3.0` versions.
- `CHANGELOG.md` - New `0.3.0` release entry summarizing Epics 1–5 deliverables.
- Any first-party `*.rs` file flagged by clippy/`cargo doc` - Documentation or warning fixes only (no feature work).

### Notes

- Unit tests live alongside code in `#[cfg(test)]` modules; integration tests live in each crate's `tests/` directory.
- Run the quality gate from the workspace root. Use `--all-features` (and/or targeted `--features`)
  to exercise feature-gated paths (`redis-queue`, `web-server`) per PRD FR 2/FR 3.
- Internal path-dependency version pins in `[workspace.dependencies]` MUST be bumped in lock-step
  with the per-crate `version` fields or `cargo` will fail to resolve.
- Git rules for this repo: prefix commits with `set +H &&`; stage ONLY the specific files for this
  Epic (never `git add -A`); use conventional commits with multiple `-m` flags and a task reference.
  Do NOT stage the pre-existing prompt-file changes (`.github/prompts/create-prd.prompt copy.md`
  delete and `create-prd-no-questions.prompt.md` untracked).
- If `snyk_code_scan` is unavailable, substitute strict clippy/compiler checks and note it.
- If a full `--all-features` build exhausts disk, clear stale `target/` artifacts (not a code failure).

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update the
file after each sub-task, not just after a parent task. After all sub-tasks of a parent are done,
run the quality gate, then commit the parent task with a conventional message.

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 From the current branch (which contains Epics 1–5), create and checkout
        `feature/milestone_9-epic_6-finalization-and-release`.
  - [ ] 0.2 Confirm the working tree is clean of unrelated changes (exclude the known pre-existing
        prompt-file changes) and that the branch contains the `m9-e1`…`m9-e5` commits.

- [ ] 1.0 Workspace build & test quality gate (FR 1, FR 2, FR 6)
  - [ ] 1.1 Run `cargo build --workspace`; fix any errors (no feature work).
  - [ ] 1.2 Run `cargo build --workspace --all-features`; fix any feature-gated build errors.
  - [ ] 1.3 Run `cargo test --workspace`; ensure all tests pass.
  - [ ] 1.4 Run `cargo test --workspace --all-features` (or targeted `--features redis-queue web-server`)
        to exercise feature-gated paths; fix failures. Note any tests skipped due to missing external
        services (Docker/Redis) in the close-out.
  - [ ] 1.5 Re-run any failing command until green.

- [ ] 2.0 Lint, format, and documentation quality gate (FR 3, FR 4, FR 5, FR 6)
  - [ ] 2.1 Run `cargo clippy --workspace -- -D warnings`; fix all warnings.
  - [ ] 2.2 Run `cargo clippy --workspace --all-features -- -D warnings`; fix all warnings.
  - [ ] 2.3 Run `cargo fmt --all -- --check`; if diffs exist, run `cargo fmt --all` and re-check.
  - [ ] 2.4 Run `cargo doc --workspace --no-deps`; add doc comments / fix doc warnings (docs only).
  - [ ] 2.5 Commit Tasks 1.0–2.0 fixes if any code/doc changes were required (skip commit if the gate
        was already clean and nothing changed).

- [ ] 3.0 Security scan of any changed first-party code (Technical Considerations)
  - [ ] 3.1 Run `snyk_code_scan` on first-party files modified for the quality gate; fix and rescan
        until clean. If the tool is unavailable, run `cargo clippy --workspace --all-features
        -- -D warnings` plus compiler checks as a substitute and record the substitution.

- [ ] 4.0 Update CHANGELOG (FR 7, FR 8, FR 9)
  - [ ] 4.1 Inspect the existing `CHANGELOG.md` format/style.
  - [ ] 4.2 Add a `0.3.0` entry grouped by feature area: Orchestration (Epic 1), Scheduler/Queue
        (Epic 2), Content Pipeline (Epic 3), Agent–Orchestrator Bridge (Epic 4), User/Admin & Security
        (Epic 5). Describe user-visible/behavioral changes, not commit-by-commit detail.
  - [ ] 4.3 Verify the new entry matches the existing changelog conventions.

- [ ] 5.0 Bump workspace version to 0.3.0 (FR 10, FR 11, FR 12, FR 13)
  - [ ] 5.1 Set the root crate `version` in `Cargo.toml` to `0.3.0`.
  - [ ] 5.2 Set every member crate `version` in `crates/*/Cargo.toml` to `0.3.0`.
  - [ ] 5.3 Update all internal path-dependency version pins in `[workspace.dependencies]` (and any
        per-crate dependency declarations) to `0.3.0`.
  - [ ] 5.4 Run `cargo build --workspace` and confirm `Cargo.lock` updates to the new versions.
  - [ ] 5.5 `grep` the workspace to confirm `0.3.0` is consistent and no stale `0.1.0` pins remain.

- [ ] 6.0 Final verification, commit, and release tag (FR 13, FR 14, FR 15)
  - [ ] 6.1 Re-run the full quality gate (build, test incl. feature paths, clippy, fmt, doc) to
        confirm everything is green at version `0.3.0`.
  - [ ] 6.2 Mark the PRD Task Checklist items complete and ensure "Relevant Files" above is accurate.
  - [ ] 6.3 Commit the changelog + version bump (stage only the specific files) with a conventional
        message referencing Task 5.0/6.0.
  - [ ] 6.4 Create the `v0.3.0` release-candidate tag on the finalized commit and verify it with
        `git tag`.
