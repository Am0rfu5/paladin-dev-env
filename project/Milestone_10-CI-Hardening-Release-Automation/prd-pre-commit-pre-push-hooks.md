# PRD: Pre-commit and Pre-push Hooks

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 1 — Pre-commit and Pre-push Hooks
**Status:** Planning
**Created:** 2026-05-29
**Priority:** High
**Estimated Effort:** Small

---

## 1. Introduction/Overview

The Paladin Framework has been developed through seven refactoring milestones and two product milestones without formalized commit-time quality gates. This means malformed code, secrets, formatting violations, and other preventable issues can enter the repository unchecked.

This feature introduces **pre-commit and pre-push Git hooks** that automatically enforce code quality standards before changes reach the repository. By catching problems at the earliest possible stage (the developer's local machine), we reduce CI failures, prevent security incidents (leaked secrets), and maintain consistent code style across all contributors.

---

## 2. Goals

1. **Prevent malformed code from entering the repository** — formatting violations, linting errors, and compilation failures are caught before commit or push.
2. **Prevent secrets from being committed** — API keys, passwords, and tokens are detected and blocked at commit time.
3. **Enforce consistent file hygiene** — trailing whitespace, missing EOF newlines, merge conflict markers, and oversized files are automatically handled.
4. **Provide a fast feedback loop** — developers receive immediate feedback on their local machine rather than waiting for CI.
5. **Maintain CI parity** — a CI step verifies that all pre-commit hooks pass, ensuring hooks cannot be bypassed without detection.
6. **Document the setup** — new contributors can install and configure hooks with minimal friction.

---

## 3. User Stories

### US-1: Developer commits code with formatting issues
> As a developer, I want the pre-commit hook to automatically check code formatting so that I am alerted to formatting violations before they enter the repository.

### US-2: Developer accidentally stages a file containing a secret
> As a developer, I want the pre-commit hook to scan for secrets (API keys, passwords, tokens) so that sensitive data never reaches the remote repository.

### US-3: Developer pushes code that doesn't compile
> As a developer, I want the pre-push hook to run a quick build and test cycle so that I know my code compiles and passes unit tests before it reaches the remote.

### US-4: New contributor sets up the project
> As a new contributor, I want clear documentation on how to install pre-commit hooks so that I can get my local development environment configured quickly and correctly.

### US-5: Developer needs to push urgently
> As a developer, I want a documented way to bypass pre-push hooks in emergencies (e.g., `git push --no-verify`) so that I am not blocked in critical situations, while understanding the tradeoffs.

### US-6: CI enforces hook compliance
> As a maintainer, I want CI to run all pre-commit checks on every PR so that even if a developer bypasses local hooks, violations are still caught before merge.

---

## 4. Functional Requirements

### 4.1 Hook Framework Selection and Configuration

| # | Requirement |
|---|-------------|
| FR-1 | The system must use the `pre-commit` framework (Python-based, language-agnostic) as the hook manager. |
| FR-2 | The system must include a `.pre-commit-config.yaml` file in the repository root that defines all hooks. |
| FR-3 | The system must document `pre-commit` installation instructions in `CONTRIBUTING.md`. |

### 4.2 Pre-commit Hooks

| # | Requirement | Hook/Tool | Stage |
|---|-------------|-----------|-------|
| FR-4 | The system must run `cargo fmt --all -- --check` to enforce consistent Rust code formatting. | `cargo fmt` | pre-commit |
| FR-5 | The system must run `cargo clippy --workspace -- -D warnings` to catch common Rust mistakes and code smells. | `cargo clippy` | pre-commit |
| FR-6 | The system must run a secrets detection tool (`detect-secrets` or `gitleaks`) to prevent API keys, passwords, and tokens from being committed. | `detect-secrets` / `gitleaks` | pre-commit |
| FR-7 | The system must validate all `.toml` files (including `Cargo.toml`) for syntax correctness. | `check-toml` | pre-commit |
| FR-8 | The system must validate all `.yaml` / `.yml` files for syntax correctness. | `check-yaml` | pre-commit |
| FR-9 | The system must reject any newly added file larger than 1 MB to prevent accidental binary commits. | `check-added-large-files` | pre-commit |
| FR-10 | The system must detect unresolved merge conflict markers in staged files. | `check-merge-conflict` | pre-commit |
| FR-11 | The system must remove trailing whitespace from staged files. | `trailing-whitespace-fixer` | pre-commit |
| FR-12 | The system must ensure all text files end with a newline character. | `end-of-file-fixer` | pre-commit |

### 4.3 Pre-push Hooks

| # | Requirement | Command | Stage |
|---|-------------|---------|-------|
| FR-13 | The system must run `cargo build --workspace` before push to catch compilation errors. | `cargo build` | pre-push |
| FR-14 | The system must run `cargo test --workspace --lib` before push to run the fast unit test subset (excluding integration tests). | `cargo test` | pre-push |

### 4.4 CI Verification

| # | Requirement |
|---|-------------|
| FR-15 | The CI pipeline must include a step that runs `pre-commit run --all-files` as a verification gate on every PR. |
| FR-16 | The CI pre-commit step must fail the build if any hook reports a violation. |

### 4.5 Documentation

| # | Requirement |
|---|-------------|
| FR-17 | `CONTRIBUTING.md` must include step-by-step installation instructions for the `pre-commit` framework. |
| FR-18 | `CONTRIBUTING.md` must document the `git push --no-verify` override for emergency situations. |
| FR-19 | `CONTRIBUTING.md` must list all configured hooks and their purposes. |

---

## 5. Non-Goals (Out of Scope)

- **Dependency vulnerability scanning** — covered by Epic 2 of this Milestone.
- **License compliance checking** — covered by Epic 2 of this Milestone.
- **Release automation** — covered by Epic 3 of this Milestone.
- **Integration tests in pre-push** — only unit tests (`--lib`) are run pre-push to keep the feedback loop fast. Full integration tests remain in CI.
- **Automatic code fixing** — hooks that detect issues (like `cargo fmt --check`) will report errors but not auto-fix. Developers are expected to run `cargo fmt` manually. (Exception: `trailing-whitespace-fixer` and `end-of-file-fixer` do auto-fix as this is non-destructive.)
- **Custom hook development** — this feature uses existing, well-maintained open-source hooks only.
- **Windows-specific setup** — installation instructions target Unix-like environments (Linux/macOS). Windows compatibility is desirable but not required for initial delivery.

---

## 6. Design Considerations

### 6.1 `.pre-commit-config.yaml` Structure

The configuration file should be organized by hook source (repository), with clear comments explaining each hook's purpose. Group hooks logically:
1. Generic file checks (whitespace, EOF, large files, merge conflicts)
2. Language-specific validation (TOML, YAML)
3. Secrets detection
4. Rust toolchain hooks (fmt, clippy)

### 6.2 Hook Execution Order

Hooks should be ordered from fastest to slowest to provide early feedback:
1. File-level checks (trailing whitespace, EOF, file size, merge conflicts) — milliseconds
2. Syntax validation (TOML, YAML) — milliseconds
3. Secrets detection — seconds
4. `cargo fmt --check` — seconds
5. `cargo clippy` — seconds to minutes (depending on workspace size)

### 6.3 Developer Experience

- First-time setup should require no more than 2–3 commands.
- Hook failures must produce clear, actionable error messages.
- The `--no-verify` escape hatch must be documented but discouraged.

---

## 7. Technical Considerations

### 7.1 Dependencies

- **Python 3.x** — required for the `pre-commit` framework. Most development environments have Python available.
- **Rust toolchain** — `cargo fmt` and `cargo clippy` require the Rust toolchain (already a project requirement).
- **`pre-commit` Python package** — installed via `pip install pre-commit` or `pipx install pre-commit`.

### 7.2 Hook Framework: `pre-commit` vs. `cargo-husky`

| Criterion | `pre-commit` | `cargo-husky` |
|-----------|-------------|---------------|
| Language support | Language-agnostic | Rust-only |
| Secrets detection | Yes (via plugins) | No |
| YAML/TOML validation | Yes (built-in hooks) | No |
| File size checks | Yes (built-in) | No |
| Rust formatting/linting | Via local hooks | Built-in |
| Community ecosystem | Large (thousands of hooks) | Small |
| Setup complexity | Moderate (Python dependency) | Low (Cargo only) |

**Recommendation:** Use `pre-commit` for its broader ecosystem. Rust-specific hooks (`cargo fmt`, `cargo clippy`) are configured as `local` hooks within the `pre-commit` framework.

### 7.3 Secrets Detection: `detect-secrets` vs. `gitleaks`

| Criterion | `detect-secrets` | `gitleaks` |
|-----------|-----------------|------------|
| Language | Python | Go (single binary) |
| Baseline support | Yes (`.secrets.baseline`) | Yes (`.gitleaks.toml`) |
| Performance | Moderate | Fast |
| False positive management | Baseline file | Allow-list in config |
| pre-commit integration | Official hook available | Official hook available |

Either tool is acceptable. `gitleaks` may be preferred for its single-binary distribution and speed.

### 7.4 CI Integration

The CI step should:
- Install `pre-commit` in the CI environment.
- Run `pre-commit run --all-files` (not just staged files, since CI checks the full tree).
- Cache the pre-commit hook environments between runs for performance.

### 7.5 Performance Considerations

- `cargo clippy` on the full workspace may take 30–60 seconds on a cold build. Consider whether this is acceptable for pre-commit or if it should be moved to pre-push only.
- Pre-push hooks (`cargo build` + `cargo test --lib`) may take 1–3 minutes. This is acceptable for push frequency.

---

## 8. Success Metrics

| Metric | Target |
|--------|--------|
| All configured hooks pass on the current codebase | 100% pass rate |
| CI verification gate operational | `pre-commit run --all-files` runs on every PR |
| Zero secrets detected in repository history post-implementation | 0 new secrets committed |
| Developer setup time for hooks | < 5 minutes from documentation |
| Pre-commit hook execution time | < 2 minutes for a typical commit |
| Pre-push hook execution time | < 5 minutes for a typical push |
| Documentation completeness | `CONTRIBUTING.md` updated with all hook information |

---

## 9. Open Questions

1. **Secrets detection tool selection** — Should we use `detect-secrets` (Python, baseline-file approach) or `gitleaks` (Go binary, config-file approach)? Both have official pre-commit hooks. A decision should be made during Task 1.2 implementation.

2. **`cargo clippy` in pre-commit vs. pre-push** — Running clippy on every commit may be slow for large changes. Should it be a pre-commit hook (catches issues earlier, but slower) or pre-push hook (faster commits, but later feedback)?

3. **Existing secrets in repository** — Before enabling secrets detection, should we run a baseline scan of the repository to identify and remediate any existing secrets? This may require creating a `.secrets.baseline` or `.gitleaks.toml` allowlist for false positives.

4. **Hook auto-installation** — Should we add a mechanism (e.g., a `Makefile` target or `cargo-husky`-style auto-install) that automatically installs pre-commit hooks when a developer clones the repository and runs the initial setup? Or should installation remain a manual step documented in `CONTRIBUTING.md`?

5. **Python dependency concern** — Does requiring Python for `pre-commit` create friction for contributors who only have the Rust toolchain? Should we provide an alternative path (e.g., Docker-based hook execution)?
