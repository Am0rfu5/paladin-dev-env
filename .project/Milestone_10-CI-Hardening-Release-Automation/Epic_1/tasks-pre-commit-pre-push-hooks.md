# Tasks: Milestone 10 — Epic 1: Pre-commit and Pre-push Hooks

**PRD:** `prd-pre-commit-pre-push-hooks.md`
**Source Epic:** `Milestone_10-Epic_1-pre-commit-pre-push-hooks.md`
**Version Target:** v0.4.0

## Relevant Files

- `.pre-commit-config.yaml` (root, new) - Version-controlled hook definitions for pre-commit and pre-push stages.
- `.gitleaks.toml` (root, new, optional) - Allowlist for gitleaks false positives in example/test fixtures.
- `Makefile` - New `hooks` target wrapping `pre-commit install` (commit + pre-push stages).
- `CONTRIBUTING.md` - Document pre-commit installation, hook usage, and `--no-verify` override.
- `.github/workflows/pre-commit.yml` (new) - CI gate running `pre-commit run --all-files`.
- `.devcontainer/Dockerfile.dev` - Add `pipx` + `pre-commit` install so future rebuilds ship the tool.
- `.devcontainer/Dockerfile` - Mirror the `pipx` + `pre-commit` install for the alternate image.
- `.devcontainer/post-create.sh` - Replace the legacy ad-hoc hook writer with `pre-commit install` (commit + pre-push stages).
- `.git/hooks/pre-commit` (untracked, superseded) - Legacy ad-hoc hook replaced by the framework.
- Various tracked files - Mechanical whitespace / EOF-newline / config-syntax fixes surfaced by the first `--all-files` run.

### Notes

- The dev container has `python3` but not `pip`/`pre-commit`; install via
  `sudo apt-get install -y python3-pip pipx && pipx ensurepath && pipx install pre-commit`
  (or `pip install --user pre-commit`).
- Rust hooks (`cargo fmt`, `cargo clippy`) are `repo: local` `system` hooks with
  `pass_filenames: false` + `always_run: true` so they run once per commit, not per file.
- The pre-push unit-test subset is `cargo test --workspace --lib` (no integration/Docker tests) to
  keep pushes fast.
- Git rules for this repo: stage ONLY the specific files for this Epic (never `git add -A`); use
  conventional commits with multiple `-m` flags and a task reference.
- Run `snyk_code_scan` on first-party code changes; this Epic is config/docs/whitespace only. If the
  tool is unavailable, substitute the strict clippy/compiler gate and note it.
- If a hook touches generated/vendored dirs (`flat/`, fixtures), add an `exclude:` regex rather than
  disabling the hook.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update the
file after each sub-task, not just after a parent task. After all sub-tasks of a parent are done,
run the quality gate (`cargo test`, `cargo fmt --check`, `cargo clippy`), then commit the parent task
with a conventional message referencing the task number.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 From the current branch, create and checkout
        `feature/milestone_10-epic_1-pre-commit-pre-push-hooks`.
  - [x] 0.2 Confirm the working tree has no unrelated staged changes before starting.

- [x] 1.0 Install and verify the `pre-commit` framework (FR 1, FR 2)
  - [x] 1.1 Install `pre-commit` in the environment (`apt-get install -y python3-pip pipx` →
        `pipx install pre-commit`, or `pip install --user pre-commit`).
  - [x] 1.2 Verify `pre-commit --version` runs. Record the rejected alternative (`cargo-husky`) and
        the rationale for choosing `pre-commit` as a short note in this file / the PRD §9.

  > Installed `pre-commit` 4.6.0 via `pipx` (apt `pipx` 1.1.0 on Debian bookworm). `~/.local/bin`
  > added to PATH in `~/.bashrc`. **Decision:** chose `pre-commit` over `cargo-husky` because it
  > provides the ready-made secrets/TOML/YAML/whitespace hook ecosystem (`gitleaks`,
  > `pre-commit-hooks`) that `cargo-husky` lacks, while still running the Rust `cargo fmt`/`clippy`
  > gates as local `system` hooks.

- [x] 2.0 Author `.pre-commit-config.yaml` with pre-commit-stage hooks (FR 3–13)
  - [x] 2.1 Create `.pre-commit-config.yaml` with `pre-commit-hooks` repo (pinned `rev`):
        `check-toml`, `check-yaml`, `check-added-large-files` (`--maxkb=1024`),
        `check-merge-conflict`, `trailing-whitespace`, `end-of-file-fixer`.
  - [x] 2.2 Add the `gitleaks` hook (pinned `rev`) for secrets detection.
  - [x] 2.3 Add `repo: local` `system` hooks for `cargo fmt --all -- --check` and
        `cargo clippy --workspace --all-targets --all-features -- -D warnings`, both with
        `pass_filenames: false` and `always_run: true` (FR 13).
  - [x] 2.4 Add `exclude:` regex(es) for generated/vendored paths (`^target/`, `^flat/`, large
        fixtures) as needed once the first run reveals them.
  - [x] 2.5 Validate the YAML parses: `pre-commit validate-config`.

- [x] 3.0 Configure the pre-push stage (FR 14–16)
  - [x] 3.1 Add `repo: local` hooks with `stages: [pre-push]` running `cargo build --workspace` and
        `cargo test --workspace --lib` (both `pass_filenames: false`).
  - [x] 3.2 Confirm `default_install_hook_types` (or explicit install commands) cover both
        `pre-commit` and `pre-push` stages.

- [x] 4.0 Bootstrap install + Makefile target (FR 17)
  - [x] 4.1 Add a `make hooks` target that runs `pre-commit install` and
        `pre-commit install --hook-type pre-push`.
  - [x] 4.2 Run `make hooks` and confirm `.git/hooks/pre-commit` and `.git/hooks/pre-push` are
        generated by the framework (superseding the legacy ad-hoc hook, FR 19).

- [x] 5.0 Bring the existing tree into conformance (FR 22)
  - [x] 5.1 Run `pre-commit run --all-files`; capture the list of files modified by
        `trailing-whitespace`/`end-of-file-fixer` and any `check-toml`/`check-yaml` failures.
  - [x] 5.2 Apply mechanical fixes (whitespace/EOF/config syntax only — no logic changes). Re-run
        until `pre-commit run --all-files` exits 0.
  - [x] 5.3 If gitleaks flags fixtures/examples, add a scoped `.gitleaks.toml` allowlist (do not
        weaken real secret detection).

  > First `--all-files` run auto-fixed trailing whitespace / missing EOF newlines across ~170
  > tracked files (mostly Markdown/YAML/SVG). `lcov.info` (generated coverage) was excluded and
  > reverted. `check-toml`/`check-yaml` passed. gitleaks passed with **no** secrets detected, so no
  > `.gitleaks.toml` allowlist was needed (Task 5.3 N/A). Empty placeholder `.rs` modules caused an
  > oscillation between `end-of-file-fixer` (wants 0 bytes) and `rustfmt` (wants a trailing newline);
  > resolved by excluding `\.rs$` from the whitespace/EOF hooks so `cargo fmt` is the sole owner of
  > Rust-file formatting. Full suite (incl. fmt + clippy) now passes stably.

- [x] 6.0 CI verification gate (FR 20, FR 21)
  - [x] 6.1 Create `.github/workflows/pre-commit.yml` that checks out, installs Rust + `pre-commit`,
        and runs `pre-commit run --all-files` on pull requests and pushes to primary branches.
  - [x] 6.2 Verify the workflow YAML is valid (the `check-yaml` hook covers this).

- [x] 7.0 Documentation (FR 18)
  - [x] 7.1 Add a "Git Hooks (pre-commit)" subsection to `CONTRIBUTING.md`: how to install
        `pre-commit`, how to run `make hooks`, how to run `pre-commit run --all-files`, and the
        `git commit/push --no-verify` emergency override.

- [x] 9.0 Devcontainer provisioning for future rebuilds
  - [x] 9.1 Add `pipx` + `pre-commit` (system-wide via `PIPX_HOME=/usr/local/pipx`,
        `PIPX_BIN_DIR=/usr/local/bin`) to `.devcontainer/Dockerfile.dev` (the active image).
  - [x] 9.2 Mirror the same install in `.devcontainer/Dockerfile` for consistency.
  - [x] 9.3 Update `.devcontainer/post-create.sh` to run `pre-commit install` +
        `pre-commit install --hook-type pre-push` instead of writing the legacy ad-hoc
        `.git/hooks/pre-commit` script.
  - [x] 9.4 Verify the local environment has `pre-commit` available (installed in Task 1.1) so the
        current session works without a rebuild.

- [x] 8.0 Smoke tests & validation (Success Metrics)
  - [x] 8.1 Plant a temporary malformed TOML, a fake secret, and a trailing-whitespace line; confirm
        the corresponding hooks reject the commit; then revert the temporary changes.
  - [x] 8.2 Confirm the pre-push gate: `cargo build --workspace` and `cargo test --workspace --lib`
        pass.
  - [x] 8.3 Final `pre-commit run --all-files` exits 0; commit Epic 1 deliverables.

  > Smoke tests: malformed `*.toml` rejected by `check-toml`; trailing whitespace rejected by
  > `trailing-whitespace`; a staged file with a GitHub PAT + AWS access token was caught by
  > `gitleaks` (2 leaks → commit blocked). Pre-push stage (`cargo build --workspace` +
  > `cargo test --workspace --lib`) passed. Full `pre-commit run --all-files` (all stages) is green.
