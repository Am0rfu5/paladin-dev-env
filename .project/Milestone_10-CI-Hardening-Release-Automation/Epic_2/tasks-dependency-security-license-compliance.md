# Tasks: Milestone 10 — Epic 2: Dependency Security and License Compliance

**PRD:** `prd-dependency-security-license-compliance.md`
**Source Epic:** `Milestone_10-Epic_2-dependency-security-license-compliance.md`
**Version Target:** v0.4.0

## Relevant Files

- `deny.toml` (root, new) - `cargo-deny` policy: license allow-list, bans, duplicate-version policy, advisories.
- `audit.toml` (root, existing) - `cargo-audit` ignore-list / exception source of truth (already has two exceptions).
- `.github/workflows/ci.yml` (existing) - Update `security-audit` job to read from `audit.toml`; add `cargo deny` and OSV-Scanner jobs.
- `.github/workflows/release.yml` (existing) - Add CycloneDX SBOM generation + release-asset upload.
- `Makefile` (existing) - Add a `security` (and/or `deny`/`sbom`) target wrapping local checks.
- `CONTRIBUTING.md` (existing) - Document dependency-security tooling and the exception process.
- `docs/SECURITY_SCANNING.md` (new, optional) - Snyk evaluation + dependency-security tooling reference.

### Notes

- `cargo-deny`, `cargo-audit`, and `cargo-cyclonedx` are installable via `cargo install --locked <tool>`.
- Validate locally what is locally runnable: `cargo audit`, `cargo deny check`, `cargo cyclonedx`.
  OSV-Scanner / Snyk / release-asset upload are CI-only and validated via config (YAML/schema) correctness.
- Git rules for this repo: stage ONLY the specific files for this Epic (never `git add -A`); use
  conventional commits with multiple `-m` flags and a task reference; prefix commits with `set +H &&`.
- The pre-commit/pre-push hooks from Epic 1 are installed; commits/pushes will run fmt+clippy
  (commit) and build+lib-test (push). This Epic is config/CI/docs only — no Rust source changes
  expected, so the Rust gates should pass unchanged.
- Run `snyk_code_scan` on first-party code changes; this Epic is config/CI/docs only. If the tool is
  unavailable, substitute the strict clippy/compiler gate (already enforced by hooks) and note it.
- License allow-list per PRD FR 11: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib.

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update the
file after each sub-task. After all sub-tasks of a parent are done, run the quality gate
(`cargo fmt --check`, `cargo clippy`, plus the new security checks), then commit the parent task with
a conventional message referencing the task number.

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 From `develop`, create and checkout
        `feature/milestone_10-epic_2-dependency-security-license-compliance`.
  - [x] 0.2 Confirm the working tree has no unrelated staged changes before starting.

  > Branched from the Epic 1 branch (`feature/milestone_10-epic_1-pre-commit-pre-push-hooks`)
  > rather than `develop`, because Epic 1's `.pre-commit-config.yaml` (and the installed hook that
  > requires it) is not yet merged to `develop`; basing Epic 2 here keeps the commit-time hooks
  > functional and consistent.

- [x] 1.0 `cargo deny` policy + local validation (FR 10–14)
  - [x] 1.1 Install `cargo-deny` locally (`cargo install --locked cargo-deny`) and confirm
        `cargo deny --version`.
  - [x] 1.2 Author `deny.toml` with `[licenses]` allowing MIT, Apache-2.0, BSD-2-Clause,
        BSD-3-Clause, ISC, Zlib (SPDX ids, modern `allow = [...]` form).
  - [x] 1.3 Add `[bans]` (no banned crates initially; `multiple-versions = "warn"`) and an
        `[advisories]` section consistent with `audit.toml` (avoid contradicting the audit gate).
  - [x] 1.4 Run `cargo deny check` against the current tree; resolve any license that is not on the
        allow-list by adding the specific SPDX license with a justification comment, or a narrowly
        scoped per-crate `clarify`/exception (no blanket disabling).
  - [x] 1.5 Re-run `cargo deny check` until it passes (licenses + bans + advisories + sources).

  > `cargo-deny` v0.19.8 installed. `deny.toml` allows the six core permissive licenses plus four
  > additional genuinely-permissive ones (Unicode-3.0, 0BSD, CC0-1.0, CDLA-Permissive-2.0), each with
  > a justification comment. MPL-2.0 (weak copyleft) is granted only via eight narrowly-scoped
  > per-crate `[[licenses.exceptions]]` so the global allow-list stays permissive-only (FR 14). The
  > `[advisories].ignore` list mirrors the two vulnerability exceptions from `.cargo/audit.toml` and
  > additionally ignores eight transitive *unmaintained* advisories (which cargo-audit does not fail
  > on) so the two scanners do not contradict each other (FR 6). `[bans]`/`[sources]` use `warn`.
  > `cargo deny check` exits 0 (advisories/bans/licenses/sources all ok); remaining output is
  > non-blocking duplicate-version warnings.

- [x] 2.0 Harden `cargo audit` to read from `audit.toml` (FR 1–4)
  - [x] 2.1 Update the CI `security-audit` job so `cargo audit` relies on `audit.toml` (remove inline
        `--ignore` flags) — single source of truth.
  - [x] 2.2 Confirm `audit.toml` exception comments satisfy FR 3 (ID, affected crate + why present,
        why unfixable, revisit condition); expand any that are thin.
  - [x] 2.3 Run `cargo audit` locally from the repo root to confirm it honors `audit.toml` and
        passes.

  > Relocated the canonical config from repo-root `audit.toml` to `.cargo/audit.toml`, the path
  > cargo-audit v0.22.1 auto-discovers. (Root `audit.toml` was never actually read by cargo-audit,
  > so CI had relied on inline `--ignore` flags.) CI `security-audit` job and the `make audit` target
  > now run plain `cargo audit` with exceptions sourced solely from `.cargo/audit.toml` — a true
  > single source of truth (FR 1). `cargo audit` exits 0: the two vulnerability advisories are
  > ignored; the eight unmaintained + one unsound (atty RUSTSEC-2021-0145) findings are cargo-audit's
  > default non-failing informational warnings. Comments in `.cargo/audit.toml`, `deny.toml`,
  > `ci.yml`, and `docs/SECURITY_SCANNING.md` updated to the new path.

- [x] 3.0 Add OSV-Scanner CI job (FR 5–7)
  - [x] 3.1 Add an `osv-scanner` job (pinned official action) configured to scan `Cargo.lock`.
  - [x] 3.2 Configure SARIF output + `upload-sarif` for PR annotations; choose annotate-only
        (non-blocking) policy initially and document the choice (FR 6, Open Q1).
  - [x] 3.3 Ensure the job runs on pull requests (and optionally a schedule for the primary branch).
  - [x] 3.4 Validate the workflow YAML parses (pre-commit `check-yaml` covers this).

- [x] 4.0 Add `cargo deny` CI gate (FR 13)
  - [x] 4.1 Add a `cargo-deny` job to CI that installs `cargo-deny --locked` and runs
        `cargo deny check`, on PRs and pushes to primary branches.
  - [x] 4.2 Cache the cargo registry/index consistent with the other CI jobs.

- [x] 5.0 Snyk evaluation + decision (FR 8–9)
  - [x] 5.1 Write a short evaluation comparing Snyk free tier vs. `cargo audit` + OSV-Scanner +
        `cargo deny` (added value, required secrets, maintenance cost).
  - [x] 5.2 Record an explicit decision: integrate or document a deferral with revisit conditions
        (recommendation: defer). Capture in `docs/SECURITY_SCANNING.md`.

  > Decision: **deferred**. `docs/SECURITY_SCANNING.md` records the comparison table, rationale
  > (the three OSS gates cover advisories + OSV DB + license/bans without external secrets), and the
  > conditions under which Snyk would be revisited.

- [x] 6.0 SBOM generation in the release pipeline (FR 15–17)
  - [x] 6.1 Install `cargo-cyclonedx` locally and generate a CycloneDX SBOM from the workspace to
        confirm the command + output format.
  - [x] 6.2 Add an SBOM step to `.github/workflows/release.yml` that generates the CycloneDX SBOM and
        uploads it as a release asset.
  - [x] 6.3 Add a `make sbom` target (and document the local command) for reproducible local
        generation (FR 17).

  > `cargo-cyclonedx` v0.5.9 installed; `cargo cyclonedx --all --format json` writes one
  > `<crate>.cdx.json` next to each crate manifest (root = `paladin-ai.cdx.json`). The `release.yml`
  > `sbom` job copies the root package SBOM to `paladin-<version>.cdx.json` and uploads it as a
  > release asset. `make sbom` reproduces generation locally; `*.cdx.json` / `bom.json` are
  > git-ignored.

- [x] 7.0 Makefile + documentation (FR 18–19)
  - [x] 7.1 Add a `make security` target wrapping `cargo audit` + `cargo deny check`.
  - [x] 7.2 Document the tooling + exception process in `CONTRIBUTING.md` (run locally, add an
        approved exception to `audit.toml`/`deny.toml`, where SBOMs are published) and/or
        `docs/SECURITY_SCANNING.md`.

  > Makefile gains `audit` (plain `cargo audit`), `deny`, `security` (audit + deny), and `sbom`
  > targets. `CONTRIBUTING.md` Security section now points to `make security` / `make sbom`, the
  > exception process, and `docs/SECURITY_SCANNING.md`.

- [x] 8.0 Conformance, validation, and commit (FR 20)
  - [x] 8.1 Confirm all locally-runnable gates pass: `cargo audit`, `cargo deny check`, `cargo fmt
        --check`, `cargo clippy -- -D warnings`.
  - [x] 8.2 Validate all new/modified workflow YAML via `pre-commit run --all-files`.
  - [x] 8.3 Run `snyk_code_scan` on first-party changes (or note substitution by the clippy/compiler
        gate if unavailable).
  - [x] 8.4 Stage only the Epic-2 files, commit with conventional messages referencing the tasks,
        and push the branch.

  > Gates: `make security` (cargo audit + cargo deny check) exits 0; `cargo fmt --check` and
  > `cargo clippy --workspace --all-targets --all-features -- -D warnings` both clean; `pre-commit
  > run check-yaml` and `check-toml` pass. No first-party Rust source changed in this Epic (config,
  > CI workflows, Makefile, and docs only), so the clippy/compiler gate substitutes for
  > `snyk_code_scan`.
