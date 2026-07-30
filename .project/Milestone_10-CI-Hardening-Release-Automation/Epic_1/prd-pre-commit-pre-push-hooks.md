# PRD: Milestone 10 — Epic 1: Pre-commit and Pre-push Hooks

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 1 of 4
**Version Target:** v0.4.0
**Source Epic:** `Milestone_10-Epic_1-pre-commit-pre-push-hooks.md`
**Status:** Ready for Implementation

---

## 1. Introduction/Overview

The Paladin workspace has matured through nine milestones without formalized, automated
commit-time quality gates. Today the only enforcement is an **untracked**, ad-hoc
`.git/hooks/pre-commit` script (running `cargo fmt --check` and `cargo clippy`) that exists only on
machines where someone manually created it. Nothing is version-controlled, nothing runs on push,
and there is no protection against committing secrets, malformed `Cargo.toml`/`config.yml` files,
oversized binaries, or unresolved merge-conflict markers.

This Epic installs **version-controlled, reproducible commit-time and push-time quality gates** so
that formatting violations, lint failures, leaked secrets, and malformed config files are caught on
the developer's machine *before* they ever reach the remote — and are enforced again in CI so the
gate cannot be silently bypassed.

The goal: every contributor gets the same automatic checks with a single setup command, and CI
verifies those same checks on every pull request.

## 2. Goals

1. A version-controlled hook configuration exists in the repository (not just in each developer's
   local `.git/hooks`), installable with one documented command.
2. Pre-commit hooks enforce: Rust formatting, Rust linting, secrets detection, TOML/YAML validation,
   large-file prevention, merge-conflict-marker detection, trailing-whitespace cleanup, and
   end-of-file newline normalization.
3. Pre-push hooks run a fast quality subset (`cargo build` + unit tests) before code reaches the
   remote, with a documented emergency override.
4. CI runs the identical hook suite (`pre-commit run --all-files`) as a required verification gate so
   the local gate cannot be bypassed by skipping local installation.
5. Installation and override instructions are documented in `CONTRIBUTING.md`.
6. The entire existing codebase passes the configured hooks (or any necessary clean-up is performed
   so it does).

## 3. User Stories

- **As a new contributor**, I want a single command that installs all quality hooks, so that my very
  first commit is automatically formatted, linted, and secret-free without me memorizing a checklist.
- **As a maintainer**, I want secrets detection and config validation to run before every commit, so
  that API keys and broken `Cargo.toml`/`config.yml` files never enter the history.
- **As a reviewer**, I want CI to run the same hooks the author should have run locally, so that I
  never have to leave review comments about formatting or whitespace.
- **As a developer in a hurry**, I want a documented `--no-verify` escape hatch for genuine
  emergencies, so that the gate protects me without ever fully blocking me.
- **As a release engineer**, I want the pre-push hook to catch compilation and unit-test failures
  before a push, so that obviously broken code never starts a CI run.

## 4. Functional Requirements

### Hook Framework Selection (FR 1–3)

1. The system must adopt the **`pre-commit`** framework (Python-based,
   [pre-commit.com](https://pre-commit.com)) as the hook manager, configured via a version-controlled
   `.pre-commit-config.yaml` in the repository root. Rationale and the rejected alternative
   (`cargo-husky`) must be recorded (see §9 / Task evaluation note).
2. The framework choice must support both Rust-specific hooks (`cargo fmt`, `cargo clippy`) and the
   broader ecosystem hooks (secrets, TOML, YAML, whitespace, file-size, merge-conflict).
3. The configuration must pin each hook repository to a specific released revision (`rev:`) so hook
   behavior is reproducible across machines and over time.

### Pre-commit Hooks (FR 4–13)

The `.pre-commit-config.yaml` must configure the following hooks, each of which must run on every
`git commit`:

4. **Formatting** — run `cargo fmt --all -- --check` and fail the commit on any formatting diff.
5. **Linting** — run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fail
   the commit on any warning. (Mirrors the existing `make lint` target.)
6. **Secrets detection** — run a secrets scanner (`gitleaks` via its maintained pre-commit hook) and
   fail the commit if a credential/API key/password pattern is detected.
7. **TOML validation** — run `check-toml` to catch `Cargo.toml`/`deny.toml`/`audit.toml` syntax
   errors.
8. **YAML validation** — run `check-yaml` to catch `config.yml`/`config.test.yml`/compose-file
   syntax errors.
9. **Large-file prevention** — run `check-added-large-files` with a **1 MB** limit to block
   accidental binary commits.
10. **Merge-conflict markers** — run `check-merge-conflict` to catch unresolved `<<<<<<<` markers.
11. **Trailing whitespace** — run `trailing-whitespace` to strip trailing whitespace (Markdown line
    breaks preserved where the hook supports it).
12. **End-of-file newline** — run `end-of-file-fixer` to ensure every file ends with exactly one
    newline.
13. The Rust hooks (FR 4, FR 5) must be configured so they run once for the workspace rather than
    once per changed file (i.e. as repo-level/`pass_filenames: false` system hooks), to avoid
    invoking `cargo` redundantly per file.

### Pre-push Hooks (FR 14–16)

14. A pre-push hook must run `cargo build --workspace` to catch compilation errors before a push.
15. A pre-push hook must run the **unit-test subset** `cargo test --workspace --lib` (fast; excludes
    integration/`tests/` directory and doc tests) before a push.
16. The pre-push hook must be wired through the same `pre-commit` framework
    (`pre-commit install --hook-type pre-push` / a `pre-push` stage) so a single install command
    enables both stages.

### Installation & Bootstrap (FR 17–19)

17. A single documented command must install all hook stages. A `make hooks` (or equivalently named)
    Makefile target should wrap `pre-commit install` + `pre-commit install --hook-type pre-push`.
18. `CONTRIBUTING.md` must document: how to install `pre-commit`, how to install the hooks, how to
    run them manually (`pre-commit run --all-files`), and the emergency override
    (`git commit --no-verify` / `git push --no-verify`).
19. The previously untracked ad-hoc `.git/hooks/pre-commit` script must be superseded by the
    version-controlled configuration (its behavior is fully covered by FR 4–5); contributors should
    not be relying on a hand-written local hook.

### CI Verification Gate (FR 20–21)

20. A CI step must run `pre-commit run --all-files` as a required gate so the configured hooks are
    enforced even for contributors who did not install them locally.
21. The CI gate must run on every pull request (and on pushes to the primary branches), and must fail
    the build when any hook fails.

### Whole-Repo Conformance (FR 22)

22. Running `pre-commit run --all-files` against the current repository must pass. Any pre-existing
    violations surfaced by the hooks (trailing whitespace, missing EOF newlines, malformed TOML/YAML,
    etc.) must be remediated as part of this Epic. Such remediation is limited to formatting/whitespace
    and config-syntax fixes — **no feature or behavioral code changes**.

## 5. Non-Goals (Out of Scope)

- Dependency vulnerability scanning (`cargo audit`, OSV-Scanner, Snyk) and license compliance
  (`cargo deny`) — these belong to **Epic 2**.
- Release automation, version bumping, crate publishing, and SBOM generation — **Epics 3 and 4**.
- Building out the full CI pipeline beyond the single `pre-commit` verification gate (the broader CI
  matrix is owned by other Milestone 10 work / existing CI).
- Rewriting or reformatting source code for style reasons beyond what the configured hooks require.
- Enforcing commit-message conventions (e.g. a `commit-msg` conventional-commit linter) — not
  requested by the Epic.
- Mandating that every developer environment has `pre-commit` pre-installed; the Epic provides
  install instructions and a CI safety net instead.

## 6. Design Considerations

- **Single source of truth:** `.pre-commit-config.yaml` is the authoritative, version-controlled hook
  definition. The local `.git/hooks/*` scripts become generated artifacts produced by
  `pre-commit install`, not hand-maintained files.
- **Reproducibility:** every third-party hook repo is pinned to a `rev`. Rust hooks are defined as
  local `repo: local` `system` hooks that shell out to the project's own `cargo` toolchain, so they
  always match the developer's pinned Rust version rather than a vendored copy.
- **Performance:** `cargo fmt`/`cargo clippy` are workspace-level operations; they must be configured
  with `pass_filenames: false` and `always_run: true` so they execute once per commit, not once per
  staged file. The pre-push unit-test subset deliberately excludes integration tests (which need
  Docker services) to keep pushes fast.
- **Mirroring CI/Makefile:** the lint hook uses the same flags as `make lint`
  (`--workspace --all-targets --all-features -- -D warnings`) so local and CI results agree.

## 7. Technical Considerations

- **Branching:** create a feature branch `feature/milestone_10-epic_1-pre-commit-pre-push-hooks` from
  the current branch.
- **Environment / tooling install:** the dev container does not ship `pre-commit` (`python3` is
  present but `pip`/`ensurepip` are not). Installation in this environment will use the available
  package manager (`apt-get install -y python3-pip pipx` then `pipx install pre-commit`, or the
  distro `pre-commit` package). `CONTRIBUTING.md` must document the portable options
  (`pipx install pre-commit`, `pip install --user pre-commit`, or OS package).
- **`gitleaks` hook:** use the maintained `gitleaks` pre-commit hook repo (pinned `rev`); the
  `pre-commit` framework fetches and caches the binary, so contributors do not need a separate manual
  `gitleaks` install. A `.gitleaks.toml` allowlist may be added if the existing tree contains false
  positives (e.g. example/test fixtures).
- **Existing tree conformance:** the repo is large; expect `end-of-file-fixer`/`trailing-whitespace`
  to touch a number of files on first run. These are mechanical, reviewable changes. If a directory
  should be excluded (e.g. generated `flat/`, `target/`, large fixture data), add an `exclude:`
  regex rather than disabling the hook.
- **Security scanning:** per repository instructions, run `snyk_code_scan` on any first-party code
  changed. This Epic changes config/docs/whitespace rather than logic; if the tool is unavailable,
  substitute the strict clippy/compiler gate and record the substitution.
- **Git rules for this repo (from prior Epics):** stage only the specific files for this Epic (never
  `git add -A`); use conventional commits with multiple `-m` flags and a task reference.

## 8. Success Metrics

- `pre-commit run --all-files` exits `0` on the finalized branch.
- `pre-commit install` + `pre-commit install --hook-type pre-push` (via `make hooks`) wires both
  stages from a single command on a clean checkout.
- A deliberately malformed change (e.g. a bad-syntax `*.toml`, a planted fake secret, a trailing-
  whitespace line) is rejected by the corresponding hook in a smoke test.
- The CI `pre-commit` job runs on a PR and fails when a hook fails, passes when the tree is clean.
- `cargo build --workspace` and `cargo test --workspace --lib` succeed (pre-push gate passes).

## 9. Open Questions

1. **Framework decision is pre-made.** The Epic recommends `pre-commit` over `cargo-husky`; this PRD
   adopts that recommendation. If the team later prefers a zero-Python, Rust-native approach, the
   equivalent can be re-expressed as tracked `.githooks/` scripts wired via `core.hooksPath` — but
   that loses the ready-made secrets/TOML/YAML hook ecosystem, which is why `pre-commit` is chosen.
2. **Secrets scanner:** `gitleaks` is selected over `detect-secrets` for its zero-config defaults and
   broader rule set. Confirm no licensing concern for the org (gitleaks is MIT).
3. **`exclude` scope:** the precise set of directories to exclude from whitespace/EOF hooks
   (`flat/`, generated fixtures, vendored data) will be finalized during implementation based on what
   the first `--all-files` run touches.
4. **CI provider wiring:** the repo currently has no `.github/workflows/`. This Epic adds a minimal
   `pre-commit` workflow; if a broader CI workflow file is later introduced by other Milestone 10
   Epics, this job should be merged into it rather than duplicated.
