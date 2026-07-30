
## Epic 1: Pre-commit and Pre-push Hooks

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** None

### Objective

Install and configure commit-time quality gates that prevent malformed code, secrets, and formatting violations from entering the repository.

### Tasks

#### Task 1.1: Select and Configure Hook Framework

**Description:** Evaluate `pre-commit` (Python-based, language-agnostic) vs. `cargo-husky` (Rust-native, simpler). Recommend `pre-commit` for its broader ecosystem (secrets detection, YAML/TOML validation, file size checks) with Rust-specific hooks for `cargo fmt` and `cargo clippy`.

**Deliverables:**
- `.pre-commit-config.yaml` in repository root.
- `pre-commit` installation documented in `CONTRIBUTING.md`.

#### Task 1.2: Configure Pre-commit Hooks

**Description:** Configure the following hooks:

| Hook | Tool | Purpose |
|------|------|---------|
| Formatting | `cargo fmt --all -- --check` | Enforce consistent code style |
| Linting | `cargo clippy --workspace -- -D warnings` | Catch common mistakes |
| Secrets detection | `detect-secrets` or `gitleaks` | Prevent API keys, passwords in commits |
| TOML validation | `check-toml` | Catch `Cargo.toml` syntax errors |
| YAML validation | `check-yaml` | Catch `config.yml` syntax errors |
| File size | `check-added-large-files` (1MB limit) | Prevent accidental binary commits |
| Merge conflict markers | `check-merge-conflict` | Catch unresolved conflicts |
| Trailing whitespace | `trailing-whitespace-fixer` | Clean whitespace |
| EOF newline | `end-of-file-fixer` | Consistent file endings |

**Deliverables:**
- All hooks configured and tested.
- CI step that runs `pre-commit run --all-files` as a verification gate.

#### Task 1.3: Configure Pre-push Hooks

**Description:** Pre-push hooks run before `git push` and should execute a fast quality check:
- `cargo build --workspace` (catches compilation errors).
- `cargo test --workspace --lib` (unit tests only — fast subset, skips integration tests).

**Deliverables:**
- Pre-push hook configured.
- Documented override: `git push --no-verify` for emergencies.

---
