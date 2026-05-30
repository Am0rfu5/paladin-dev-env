## Relevant Files

- `.pre-commit-config.yaml` - Main configuration file defining all pre-commit and pre-push hooks for the repository.
- `CONTRIBUTING.md` - Existing contributor documentation; must be updated with hook installation instructions, hook descriptions, and bypass documentation.
- `.github/workflows/pre-commit-ci.yml` - New CI workflow file that runs `pre-commit run --all-files` on every PR to enforce hook compliance.
- `.gitleaks.toml` - Configuration file for gitleaks secrets detection tool (allowlists, custom rules, false positive management).

### Notes

- The `pre-commit` framework (Python-based) is used as the hook manager. It requires Python 3.x and is installed via `pip install pre-commit` or `pipx install pre-commit`.
- Rust-specific hooks (`cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`) are configured as `local` hooks within the `pre-commit` framework since they depend on the project's Rust toolchain.
- Use `pre-commit run --all-files` to test all hooks against the entire repository.
- Use `pre-commit run <hook-id>` to test a specific hook.
- Use `git commit --no-verify` or `git push --no-verify` to bypass hooks in emergencies.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

## Tasks

- [ ] 0.0 Create feature branch
  - [ ] 0.1 Create and checkout a new branch for this feature (e.g., `git checkout -b feature/milestone-10-epic-1-pre-commit-hooks`)

- [ ] 1.0 Install and configure the `pre-commit` framework
  - [ ] 1.1 Verify Python 3.x is available in the development environment
  - [ ] 1.2 Install the `pre-commit` package (`pip install pre-commit` or `pipx install pre-commit`)
  - [ ] 1.3 Create the `.pre-commit-config.yaml` file in the repository root with the top-level `repos:` key and set `default_stages: [pre-commit]`
  - [ ] 1.4 Run `pre-commit install` to register the hook with the local `.git/hooks/pre-commit`
  - [ ] 1.5 Run `pre-commit install --hook-type pre-push` to register the pre-push hook
  - [ ] 1.6 Verify hooks are installed by checking `.git/hooks/pre-commit` and `.git/hooks/pre-push` exist

- [ ] 2.0 Configure generic file-level pre-commit hooks
  - [ ] 2.1 Add the `pre-commit/pre-commit-hooks` repository to `.pre-commit-config.yaml`
  - [ ] 2.2 Configure `trailing-whitespace` hook (auto-fixes trailing whitespace in staged files — FR-11)
  - [ ] 2.3 Configure `end-of-file-fixer` hook (ensures all text files end with a newline — FR-12)
  - [ ] 2.4 Configure `check-added-large-files` hook with `args: ['--maxkb=1024']` to reject files larger than 1 MB (FR-9)
  - [ ] 2.5 Configure `check-merge-conflict` hook to detect unresolved merge conflict markers (FR-10)
  - [ ] 2.6 Run `pre-commit run --all-files` to verify these hooks pass on the current codebase
  - [ ] 2.7 Fix any violations found (trailing whitespace, missing EOF newlines, etc.)

- [ ] 3.0 Configure syntax validation hooks (TOML, YAML)
  - [ ] 3.1 Configure `check-toml` hook from `pre-commit/pre-commit-hooks` to validate all `.toml` files including `Cargo.toml` (FR-7)
  - [ ] 3.2 Configure `check-yaml` hook from `pre-commit/pre-commit-hooks` to validate all `.yaml` / `.yml` files (FR-8)
  - [ ] 3.3 Run `pre-commit run check-toml --all-files` and verify all TOML files pass
  - [ ] 3.4 Run `pre-commit run check-yaml --all-files` and verify all YAML files pass
  - [ ] 3.5 Fix any syntax errors found in existing TOML or YAML files

- [ ] 4.0 Configure secrets detection hook
  - [ ] 4.1 Decide between `detect-secrets` and `gitleaks` (recommendation: `gitleaks` for speed and single-binary distribution)
  - [ ] 4.2 Add the `gitleaks` pre-commit hook repository to `.pre-commit-config.yaml` (use `https://github.com/gitleaks/gitleaks` with the `gitleaks` hook ID)
  - [ ] 4.3 Create a `.gitleaks.toml` configuration file in the repository root with appropriate allowlists for known false positives
  - [ ] 4.4 Run a baseline scan of the repository (`gitleaks detect --source . --report-format json --report-path /tmp/gitleaks-baseline.json`) to identify existing issues
  - [ ] 4.5 Review baseline scan results and add false positives to `.gitleaks.toml` allowlist
  - [ ] 4.6 Run `pre-commit run gitleaks --all-files` and verify it passes cleanly
  - [ ] 4.7 Document the secrets detection approach and false positive management in a comment in `.pre-commit-config.yaml`

- [ ] 5.0 Configure Rust toolchain pre-commit hooks (fmt, clippy)
  - [ ] 5.1 Add a `local` hooks repository section to `.pre-commit-config.yaml` for Rust toolchain hooks
  - [ ] 5.2 Configure `cargo-fmt` as a local hook: `entry: cargo fmt --all -- --check`, `language: system`, `types: [rust]`, `pass_filenames: false` (FR-4)
  - [ ] 5.3 Configure `cargo-clippy` as a local hook: `entry: cargo clippy --workspace -- -D warnings`, `language: system`, `types: [rust]`, `pass_filenames: false` (FR-5)
  - [ ] 5.4 Run `pre-commit run cargo-fmt --all-files` and verify formatting passes
  - [ ] 5.5 Run `pre-commit run cargo-clippy --all-files` and verify clippy passes
  - [ ] 5.6 Fix any formatting or clippy violations found in the codebase
  - [ ] 5.7 Evaluate whether `cargo-clippy` execution time is acceptable for pre-commit (< 2 min target); if too slow, consider moving to pre-push stage only

- [ ] 6.0 Configure pre-push hooks (build, test)
  - [ ] 6.1 Add a `cargo-build` local hook with `stages: [pre-push]`, `entry: cargo build --workspace`, `language: system`, `pass_filenames: false` (FR-13)
  - [ ] 6.2 Add a `cargo-test` local hook with `stages: [pre-push]`, `entry: cargo test --workspace --lib`, `language: system`, `pass_filenames: false` (FR-14)
  - [ ] 6.3 Run `pre-commit run --hook-stage pre-push --all-files` to verify pre-push hooks pass
  - [ ] 6.4 Verify the pre-push hook execution time is within acceptable bounds (< 5 minutes target)

- [ ] 7.0 Add CI verification step for pre-commit hooks
  - [ ] 7.1 Create `.github/workflows/pre-commit-ci.yml` workflow file
  - [ ] 7.2 Configure the workflow to trigger on `pull_request` events
  - [ ] 7.3 Add steps to: checkout code, install Python, install Rust toolchain, install `pre-commit`, and cache pre-commit environments
  - [ ] 7.4 Add the main step: `pre-commit run --all-files` (FR-15)
  - [ ] 7.5 Ensure the workflow fails the build if any hook reports a violation (FR-16)
  - [ ] 7.6 Test the CI workflow by pushing a branch and verifying it runs successfully

- [ ] 8.0 Update CONTRIBUTING.md with hook documentation
  - [ ] 8.1 Add a "Pre-commit Hooks" section to `CONTRIBUTING.md`
  - [ ] 8.2 Document step-by-step installation instructions: install Python, install `pre-commit`, run `pre-commit install`, run `pre-commit install --hook-type pre-push` (FR-17)
  - [ ] 8.3 Create a table listing all configured hooks, their purpose, and their stage (pre-commit vs. pre-push) (FR-19)
  - [ ] 8.4 Document the `--no-verify` bypass option for emergency situations with a clear warning about tradeoffs (FR-18)
  - [ ] 8.5 Document how to run hooks manually (`pre-commit run --all-files`, `pre-commit run <hook-id>`)
  - [ ] 8.6 Document how to update hook versions (`pre-commit autoupdate`)
  - [ ] 8.7 Review the documentation for clarity and completeness from a new contributor's perspective

- [ ] 9.0 Validate all hooks pass on the current codebase
  - [ ] 9.1 Run `pre-commit run --all-files` and confirm all pre-commit stage hooks pass with zero violations
  - [ ] 9.2 Run `pre-commit run --hook-stage pre-push --all-files` and confirm all pre-push stage hooks pass
  - [ ] 9.3 Perform a test commit to verify the pre-commit hook triggers correctly
  - [ ] 9.4 Perform a test push (to a test branch) to verify the pre-push hook triggers correctly
  - [ ] 9.5 Verify the CI workflow passes on the PR
  - [ ] 9.6 Document any known issues, edge cases, or false positives in the PR description
