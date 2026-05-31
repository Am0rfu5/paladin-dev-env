# Tasks: Release Automation

**PRD:** `prd-release-automation.md`
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 3 of 4
**Target Version:** v0.4.0

## Relevant Files

- `docs/RELEASE_AUTOMATION.md` - New: tooling evaluation (cargo-release vs. release-plz), the
  selected tool, and the operator guide for the automated release flow.
- `release.toml` - New: `cargo-release` configuration (lockstep versioning, publish order, hooks).
- `.github/workflows/release.yml` - Existing: extend with a dependency-ordered crates.io `publish`
  job gated on the test suite; preserve Docker/binary/SBOM/release jobs.
- `Makefile` - Existing: rename current `release` (dry-run publishes) → `publish-dry-run`; add a new
  `release VERSION=x.y.z` target (bump → changelog → commit → tag → push) with semver validation.
- `Cargo.toml` (root) + `crates/*/Cargo.toml` - Verify publish metadata; mark non-publishable crates
  `publish = false`; confirm lockstep version + dependency pins are bump-friendly.
- `CONTRIBUTING.md` - Existing: document the release flow, required secrets, and dry-run path.
- `docs/RELEASE_CHECKLIST.md` - Existing: cross-reference the automated flow.
- `CHANGELOG.md` - Existing: confirm the `## [Unreleased]` structure the `make release` target edits.
- `.devcontainer/Dockerfile.dev` - Existing: pre-install `cargo-release` in the dev image so rebuilt
  containers provision the release tool for every developer.
- `Makefile` (`setup` target) - Existing: install `cargo-release` for non-devcontainer local setups.

### Notes

- This Epic is CI/build/release **tooling only** — no `src/` domain code changes are expected, so the
  conformance gate (`cargo build`/`test`/`fmt`/`clippy`) is a regression check, not feature testing.
- `cargo-release` and `cargo publish --dry-run` are the primary local validation tools.
- Workflow/TOML changes must pass `pre-commit run check-yaml` / `check-toml`.
- The repo uses pre-commit (commit: fmt+clippy) and pre-push (build+test) hooks from Milestone 10
  Epic 1; commits/pushes will exercise them.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update the
file after each sub-task. After all sub-tasks of a parent are done, run the quality gate
(`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, plus `pre-commit`), then commit the
parent task with a conventional message referencing the task number.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout
        `feature/milestone_10-epic_3-release-automation` (based on the Epic 2 branch so the
        Milestone 10 pre-commit/pre-push hooks and security tooling remain present until they reach
        `develop`).
  - [x] 0.2 Confirm the working tree has no unrelated staged changes before starting.

  > Branched from `feature/milestone_10-epic_2-dependency-security-license-compliance` (not
  > `develop`) so the Milestone 10 Epic 1 hooks and Epic 2 security/license tooling remain active and
  > consistent until those branches merge to `develop`.

- [x] 1.0 Evaluate release tooling and record the decision (FR 1–3)
  - [x] 1.1 Write `docs/RELEASE_AUTOMATION.md` comparing `cargo-release` vs. `release-plz` across
        trigger model, changelog handling, workspace publish-order support, required
        secrets/permissions, and maintenance cost.
  - [x] 1.2 Record an explicit recommendation + selected tool (recommendation: `cargo-release`) with
        rationale tied to the existing tag-triggered, curated-changelog, lockstep workflow.
  - [x] 1.3 Document how the tool is installed reproducibly (pinned `cargo install --locked
        cargo-release`) both locally and in CI.

  > Decision: **`cargo-release`** (see `docs/RELEASE_AUTOMATION.md`). Chosen for fit with the
  > existing curated-changelog + lockstep + tag-triggered pipeline, with no PR-bot/GitHub-App
  > requirement. `release-plz` documented as the revisit option if the project adopts strict
  > Conventional Commits + continuous release-PR model.

- [x] 2.0 Configure workspace publish order + release config (FR 4–7)
  - [x] 2.1 Audit every workspace crate's `Cargo.toml` for required publish metadata
        (`description`, `license`, `repository`); note any gaps.
  - [x] 2.2 Mark any non-publishable crate `publish = false` (e.g., internal/example-only members);
        confirm the publishable set matches the canonical order.
  - [x] 2.3 Author `release.toml` (`cargo-release`) encoding lockstep versioning and the canonical
        publish order: core → ports → leaf tier → facade (→ cli if present).
  - [x] 2.4 Run dependency-first `cargo publish --dry-run -p <crate>` for every crate; capture
        results and document expected first-publish ordering failures (FR 6).

  > All 10 crates carry complete publish metadata (`name`/`version`/`description`/`license = MIT`/
  > `repository`); none required `publish = false`. The CLI is a `[[bin]]` in the root `paladin-ai`
  > crate, not a separate member, so the canonical publish order is core → ports → leaf tier
  > (`battalion`, `llm`, `memory`, `web`, `notifications`, `content`, `storage`) → facade
  > (`paladin-ai`). Authored `release.toml` (lockstep `shared-version`, `v{{version}}` tags, no
  > `pre-release-replacements` — the changelog edit is owned by `make release` to avoid per-crate
  > duplication). Dry-run validation: `cargo publish --dry-run -p paladin-ai-core` succeeds
  > (packaged 74 files); dependents (e.g. `paladin-ports`) fail with
  > `failed to select a version for ... paladin-ai-core = "=0.3.0"` because the new version is not yet
  > on crates.io — this is the **expected first-publish ordering behavior** (FR 6), resolved once
  > upstream crates are published in order by the pipeline, not a hard failure.

- [x] 3.0 Extend the tag-triggered release pipeline to publish crates (FR 8–14)
  - [x] 3.1 Add a `test` (or reuse existing) gate job so publish/release steps only run when the full
        test suite passes.
  - [x] 3.2 Add a `publish-crates` job to `.github/workflows/release.yml` that publishes all
        publishable crates to crates.io in dependency order using `CARGO_REGISTRY_TOKEN`.
  - [x] 3.3 Make the publish job idempotent/tolerant of already-published versions and guard it so it
        is skipped when the token secret is absent (FR 11, FR 13).
  - [x] 3.4 Provide a documented dry-run path (e.g., `workflow_dispatch` input or pre-release tag)
        that exercises the pipeline without a live crates.io publish (FR 13).
  - [x] 3.5 Preserve the existing Docker/binary/SBOM/GitHub-release jobs; wire `needs:` so publish is
        ordered correctly relative to them.
  - [x] 3.6 Validate the workflow YAML (`pre-commit run check-yaml --all-files`).

  > Added a `test` job (full `cargo test --workspace`) plus a `publish-crates` job
  > (`needs: [test, create-release]`) that publishes in dependency order: core → ports → leaf tier
  > → facade. The job detects its mode (dry-run / publish / skip): `workflow_dispatch` with
  > `dry_run=true` runs `cargo publish --dry-run` (no token needed); a missing `CARGO_REGISTRY_TOKEN`
  > skips the publish with a warning; a real run tolerates "already published" so it is re-runnable.
  > Existing Docker/binary/SBOM/release jobs are unchanged.

- [x] 4.0 Create the `make release VERSION=` target (FR 15–19)
  - [x] 4.1 Rename the current dry-run `release` target to `publish-dry-run` (preserve behavior) and
        update any references (help text, docs).
  - [x] 4.2 Add `release: VERSION` target that validates `VERSION` is present + valid semver, failing
        fast with a clear message otherwise.
  - [x] 4.3 Implement the flow: run release-readiness checks (reuse `release-check`) → lockstep
        version bump (via `cargo-release` or scripted edit) → finalize `CHANGELOG.md`
        (`Unreleased` → version + date) → commit → `v x.y.z` tag → push.
  - [x] 4.4 Document the full release flow, required `CARGO_REGISTRY_TOKEN` secret, and the dry-run
        path in `CONTRIBUTING.md`; cross-reference from `docs/RELEASE_CHECKLIST.md` and
        `docs/RELEASE_AUTOMATION.md`.

  > `make release` (dry-run publishes) renamed to `make publish-dry-run`. New `make release
  > VERSION=x.y.z` validates semver, requires `cargo-release`, runs `release-check`, bumps all crates
  > in lockstep via `cargo release version`, finalizes `CHANGELOG.md`, commits, tags `v<version>`,
  > and pushes. `CONTRIBUTING.md` gains a Releasing section; `RELEASE_CHECKLIST.md` cross-references
  > `RELEASE_AUTOMATION.md`.

- [x] 5.0 Conformance, validation, and commit (FR 20)
  - [x] 5.1 Confirm `cargo build`, `cargo test`, `cargo fmt --check`, and
        `cargo clippy --workspace --all-targets --all-features -- -D warnings` all pass.
  - [x] 5.2 Validate all new/modified workflow + TOML via `pre-commit run --all-files`
        (`check-yaml`, `check-toml`).
  - [x] 5.3 Run `snyk_code_scan` on first-party changes (or note substitution by the clippy/compiler
        gate if there are no Rust source changes).
  - [x] 5.4 Stage only the Epic-3 files, commit with conventional messages referencing the tasks, and
        push the branch.

  > `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  > `pre-commit run check-yaml --all-files`, and `check-toml --all-files` (validates `release.toml`)
  > all pass; `cargo build`/`cargo test` re-verified by the pre-push hook. This Epic changes only
  > CI/build/release configuration and docs — no first-party Rust source — so `snyk_code_scan` has no
  > new code surface; the clippy + compiler gate stands in as the static-analysis check. Staged only
  > the Epic-3 files (`release.toml`, `.github/workflows/release.yml`, `Makefile`, `CONTRIBUTING.md`,
  > `docs/RELEASE_AUTOMATION.md`, `docs/RELEASE_CHECKLIST.md`, and the Epic-3 PRD + tasks) and
  > committed/pushed on `feature/milestone_10-epic_3-release-automation`.

- [x] 6.0 Provision `cargo-release` for future devcontainer/local builds (follow-up)
  - [x] 6.1 Add a pinned `cargo install --locked --version <ver> cargo-release` step to
        `.devcontainer/Dockerfile.dev` so a rebuilt dev image ships the release tool for every
        developer.
  - [x] 6.2 Add `cargo-release` to the Makefile `setup` target for non-devcontainer local setups.
  - [x] 6.3 Confirm the CI release pipeline does **not** require `cargo-release` (it publishes with
        plain `cargo publish`), so no CI workflow change is needed.
  - [x] 6.4 Validate, stage only the changed provisioning files, commit, and push.

  > `make release` (and `cargo-release`) only runs locally; CI publishes with plain `cargo publish`,
  > so no workflow change was needed. Pinned `cargo install --locked --version 1.1.2 cargo-release`
  > added to `.devcontainer/Dockerfile.dev` (alongside the other `cargo install` tool steps) so a
  > rebuilt dev image provisions the tool for every developer; also added (unpinned, matching the
  > other entries) to the Makefile `setup` target for local non-devcontainer setups. Existing
  > containers can install on demand via `cargo install --locked cargo-release` (already documented
  > in CONTRIBUTING.md / docs/RELEASE_AUTOMATION.md) or by rebuilding the devcontainer.
