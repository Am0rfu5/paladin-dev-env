# PRD: CI/CD Pipeline Enhancement & Coverage Reporting

> **Correction (dated 2026-08-13, Phase 15 / PIPE-02, plan 15-10):** **Appendix C**'s phased
> 70% → 74% → 78% threshold rollout (below) and **FR-25.6** item 18's matching phase table (below,
> `#### FR-25.6`) are both **superseded**, for the same reason the parent PRD's 78% hard gate is
> superseded — see that document's own correction banner at its head.
> `.planning/decisions/0006-coverage-gate.md`'s `## Phase 15 amendment (2026-08-13)` rejects the
> ramp explicitly (`## Considered Options`: "Epic 25 FR-25.6 phased 70% → 74% → 78% ramp —
> rejected, per D-09 explicitly: three numbers where RECON-07 asked for one") and binds to **one**
> number, 82%, set once from a fresh measurement rather than advanced phase-by-phase. **This
> Epic's own Open Question 3** — "Should the existing `integration-tests.yml` coverage step be
> removed or kept with a different Codecov flag?" (`## 9. Open Questions`, below) — is **answered
> by removal**, matching this PRD's own recommended default ("recommend removing once `ci.yml`
> coverage is verified working"): the `Generate integration test coverage` /
> `Upload integration coverage` step pair was deleted from `integration-tests.yml` entirely (plan
> 15-02, commit `f9b5ad2`), not retained with a different Codecov flag. **Appendix B**'s
> "Current `ci.yml` Job Listing (Pre-Change Reference)" table (below) is long superseded by the
> tree regardless of this phase's changes — it predates several intervening milestones' own
> additions. Rather than editing that historical table, the current job count is recorded here:
> `.github/workflows/ci.yml` carries **19** job ids as of this correction
> (`python3 -c "import yaml; print(len(yaml.safe_load(open('.github/workflows/ci.yml'))['jobs']))"`),
> not the seven-job pre-change baseline the table records nor the "Jobs 8/9/10 added" post-change
> note beneath it. Original text below retained throughout, nothing deleted, per D-00c/D-00d.

## Document Info

| Field | Value |
|-------|-------|
| **PRD ID** | PRD-CICD-025 |
| **Feature Name** | CI/CD Pipeline Enhancement & Coverage Reporting |
| **Epic** | 25 |
| **Parent PRD** | PRD-DQC-001 (Deferred QA & Documentation Completion) |
| **Priority** | High (Next Sprint) |
| **Origin** | Epic 24 Deferred Subtasks (8.6, 8.9–8.13, 6.11–6.13) |
| **Date** | February 20, 2026 |
| **Status** | Draft |

---

## 1. Introduction / Overview

The Paladin project's CI/CD pipeline (GitHub Actions) has several gaps that allow regressions to reach the `main` branch undetected. Specifically:

- **CLI snapshot tests** (43 tests using `insta`) are never executed in CI — output regressions can merge unnoticed.
- **Benchmark compilation** is not checked on PRs — benchmarks can bitrot silently between scheduled runs.
- **Code coverage** is only generated for integration tests in a separate workflow (`integration-tests.yml`), and never for unit tests. No coverage threshold is enforced — coverage can decrease without blocking a PR.
- **Makefile** has no coverage or CLI test targets, making local developer workflows inconsistent with CI expectations.
- **Existing GitHub Actions are outdated**: `actions-rs/toolchain@v1` (deprecated), `actions/cache@v3` (v4 available), `codecov/codecov-action@v3` (v4 available). A dangling invalid `on: schedule` YAML block exists at the bottom of `ci.yml`.

This epic adds the missing CI jobs, creates a `.codecov.yml` with a phased coverage threshold rollout (starting at 70%, ramping to 78%), modernizes all GitHub Actions to current versions, and adds developer-friendly Makefile targets.

### Current State Summary

| Aspect | Current | Target |
|--------|---------|--------|
| CLI tests in CI | Not run | Run on every PR |
| Benchmark compilation in CI | Not checked (only full runs on schedule) | Compile-check on every PR |
| Unit test coverage reporting | None | Combined unit+integration LCOV uploaded to Codecov |
| Coverage threshold enforcement | None | 70% initial → 78% final |
| `.codecov.yml` | Does not exist | Created with project/patch thresholds |
| GitHub Action versions | v1/v3 (deprecated) | Latest stable (dtolnay, v4) |
| `ci.yml` YAML validity | Invalid (dangling schedule block) | Valid YAML, no syntax errors |
| Makefile coverage targets | None | `coverage`, `coverage-html`, `test-cli`, `bench-check` |
| CONTRIBUTING.md coverage docs | Mentions tools, no setup guide | Full setup and usage guide |

---

## 2. Goals

| ID | Goal | Measurable Target |
|----|------|--------------------|
| G-25.1 | CLI snapshot tests run in CI | `cargo test --test cli` executes on every push/PR to `main`/`develop` and blocks merge on failure |
| G-25.2 | Benchmark bitrot prevention | `cargo bench --no-run` executes on every push/PR and blocks merge on compile failure |
| G-25.3 | Combined coverage reporting | Single `cargo llvm-cov` run covering unit + integration tests, LCOV uploaded to Codecov on every PR |
| G-25.4 | Coverage threshold enforcement | `.codecov.yml` enforces project-level threshold, starting at 70% and ramping to 78% |
| G-25.5 | Modernize GitHub Actions | All actions updated to latest stable versions; deprecated `actions-rs/toolchain@v1` replaced |
| G-25.6 | Fix CI YAML issues | Dangling `on: schedule` block removed; `ci.yml` passes YAML lint |
| G-25.7 | Developer-friendly local targets | `make coverage`, `make coverage-html`, `make test-cli`, `make bench-check` all functional |
| G-25.8 | Document coverage setup | `CONTRIBUTING.md` includes full coverage tooling setup and usage guide |

---

## 3. User Stories

**US-25.1**: As a **developer**, I want CI to automatically run CLI snapshot tests on every PR so that CLI output regressions are caught before merge.

**US-25.2**: As a **developer**, I want CI to verify benchmark compilation on every PR so that benchmark bitrot is prevented without running full benchmarks.

**US-25.3**: As a **maintainer**, I want combined code coverage (unit + integration) generated and uploaded to Codecov on every PR so that I can see the coverage impact of any change.

**US-25.4**: As a **maintainer**, I want a coverage threshold that starts at 70% and ramps to 78% so that we can adopt enforcement gradually without blocking legitimate PRs during the ramp-up period.

**US-25.5**: As a **developer**, I want all GitHub Actions updated to their latest stable versions so that we benefit from performance improvements and avoid deprecation warnings.

**US-25.6**: As a **developer**, I want `make coverage` and `make coverage-html` targets so that I can generate coverage reports locally with a single command.

**US-25.7**: As a **developer**, I want `make test-cli` and `make bench-check` targets so that I can run the same checks locally that CI enforces.

**US-25.8**: As a **new contributor**, I want the `CONTRIBUTING.md` to explain how to install coverage tools and generate reports so that I can replicate CI behavior on my machine.

---

## 4. Functional Requirements

### FR-25.1: Modernize GitHub Actions Versions

All three workflow files (`.github/workflows/ci.yml`, `.github/workflows/integration-tests.yml`, `.github/workflows/release.yml`) must be updated:

> **Why**: `actions-rs/toolchain@v1` is deprecated and unmaintained. `actions/cache@v3` and `codecov/codecov-action@v3` have v4 releases with improved performance and security.

1. **Replace `actions-rs/toolchain@v1`** with `dtolnay/rust-toolchain@stable` (or `@beta`, `@nightly` as appropriate) in all workflow files.
   - `dtolnay/rust-toolchain` is the community-standard replacement.
   - It auto-handles component installation (`rustfmt`, `clippy`) via the `components` input.
   - Example:
     ```yaml
     - uses: dtolnay/rust-toolchain@stable
       with:
         components: rustfmt, clippy
     ```

2. **Replace `actions/cache@v3`** with `actions/cache@v4` in all workflow files.
   - v4 includes improved cache restore performance and better error handling.
   - Cache keys and paths remain the same.

3. **Replace `codecov/codecov-action@v3`** with `codecov/codecov-action@v4` in `integration-tests.yml` (and the new coverage step in `ci.yml`).
   - v4 requires or strongly recommends a `CODECOV_TOKEN` secret for private repos and improved reliability on public repos.
   - Add `token: ${{ secrets.CODECOV_TOKEN }}` to the action config.

4. **Replace `actions/checkout@v3`** with `actions/checkout@v4` if any instances use v3 (verify all files).

5. Verify no other deprecated actions exist across all workflow files.

### FR-25.2: Fix `ci.yml` YAML Issues

6. **Remove the dangling `on: schedule` block** at lines ~336–340 of `ci.yml`. This block is syntactically invalid because the top-level `on:` trigger is already defined at line 3. It currently has no effect but may confuse YAML parsers or future editors.

7. **Validate `ci.yml` with a YAML linter** after all changes (e.g., `yamllint` or `actionlint`). Ensure zero errors.

### FR-25.3: Add CLI Snapshot Test Job to `ci.yml`

8. Add a new job named `cli-tests` (or add a step to the existing `test` job) that runs:
   ```bash
   cargo test --test cli
   ```
   This command runs all snapshot tests in the `tests/cli/` directory (table, progress, error, help output tests — 43 total).

9. The job/step must:
   - Run on every `push` and `pull_request` to `main` and `develop` (matching existing triggers).
   - Use the `stable` Rust toolchain.
   - Cache cargo dependencies (same cache config as other jobs).
   - Fail the pipeline if any snapshot test fails.
   - Not require external services (Redis, MinIO) — CLI tests are self-contained.

10. If implemented as a separate job, it should run in parallel with existing `lint` and `test` jobs (no `needs:` dependency).

### FR-25.4: Add Benchmark Compilation Check to `ci.yml`

11. Add a new job named `bench-check` (or a step in an existing job) that runs:
    ```bash
    cargo bench --no-run
    ```
    This compiles all benchmarks without executing them, catching API breakage.

12. The job/step must:
    - Run on every `push` and `pull_request` to `main` and `develop`.
    - Use the `stable` Rust toolchain.
    - Fail the pipeline if any benchmark fails to compile.
    - Not actually run benchmarks (no performance numbers, no Criterion output).
    - Run in parallel with other jobs (no `needs:` dependency).

13. The existing `benchmark` job (which runs full benchmarks on schedule/manual) must remain unchanged.

### FR-25.5: Add Combined Coverage Job to `ci.yml`

14. Add a new job named `coverage` that:
    a. Installs `cargo-llvm-cov` (use `taiki-e/install-action@cargo-llvm-cov` for fast, cached installation instead of `cargo install`).
    b. Runs:
       ```bash
       cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
       ```
       This covers both unit and integration test code in a single run.
    c. Uploads the LCOV report to Codecov:
       ```yaml
       - uses: codecov/codecov-action@v4
         with:
           token: ${{ secrets.CODECOV_TOKEN }}
           files: lcov.info
           flags: combined
           fail_ci_if_error: true
       ```
    d. Saves an HTML coverage report as a downloadable artifact:
       ```bash
       cargo llvm-cov --all-features --workspace --html --output-dir target/coverage
       ```
       ```yaml
       - uses: actions/upload-artifact@v4
         with:
           name: coverage-report
           path: target/coverage/
           retention-days: 14
       ```

15. The coverage job must:
    - Run on every `push` and `pull_request` to `main` and `develop`.
    - Start Redis and MinIO services (same as `integration-tests` job) so integration tests can execute during coverage collection.
    - Use the `stable` Rust toolchain.
    - Set `fail_ci_if_error: true` on the Codecov upload so missing coverage reports are caught.

16. **Update `integration-tests.yml`**: The existing coverage step in `integration-tests.yml` (lines ~92–103) should be evaluated. Options:
    - **Remove it** if the new `ci.yml` coverage job fully subsumes it.
    - **Keep it with a different Codecov flag** (`flags: integration-only`) if separate integration-only coverage is desired.
    - **Recommended**: Remove it to avoid duplicate coverage uploads. The combined report in `ci.yml` replaces it.

### FR-25.6: Create `.codecov.yml` Configuration

17. Create a `.codecov.yml` file at the repository root with the following configuration:

    ```yaml
    codecov:
      require_ci_to_pass: true

    coverage:
      precision: 2
      round: down
      range: "70...100"

      status:
        project:
          default:
            target: 70%       # Phase 1: Start at 70%
            threshold: 2%      # Allow 2% fluctuation
            if_ci_failed: error
        patch:
          default:
            target: 80%       # New code must have 80% coverage
            threshold: 5%      # Allow 5% fluctuation on patches

    comment:
      layout: "reach,diff,flags,files"
      behavior: default
      require_changes: false
      require_base: false
      require_head: true

    ignore:
      - "tests/**"
      - "benches/**"
      - "examples/**"
      - "migrations/**"
      - "scripts/**"
      - "flat/**"
    ```

18. **Phased threshold rollout plan** (documented in `.codecov.yml` comments and CONTRIBUTING.md):

    | Phase | Timeline | Project Target | Patch Target |
    |-------|----------|----------------|--------------|
    | **Phase 1** (Initial) | Sprint 1–2 | 70% | 80% |
    | **Phase 2** (Ramp) | Sprint 3–4 | 74% | 80% |
    | **Phase 3** (Target) | Sprint 5+ | 78% | 80% |

    Each phase change requires a simple update to the `target:` value in `.codecov.yml`.

19. Configure Codecov to post PR comments with coverage diff, showing which files gained or lost coverage.

### FR-25.7: Add Makefile Targets

20. Add the following targets to the `Makefile`, in a new **Coverage** section placed between the existing **Testing** and **Code Quality** sections:

    ```makefile
    ## Coverage
    coverage: ## Generate LCOV coverage report
    	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
    	@echo "Coverage report written to lcov.info"

    coverage-html: ## Generate HTML coverage report
    	cargo llvm-cov --all-features --workspace --html --output-dir target/coverage
    	@echo "HTML report: target/coverage/index.html"
    	@command -v open >/dev/null 2>&1 && open target/coverage/index.html || true
    ```

21. Add the following targets to the existing **Testing** section:

    ```makefile
    test-cli: ## Run CLI snapshot tests
    	cargo test --test cli

    bench-check: ## Verify benchmarks compile (no execution)
    	cargo bench --no-run
    ```

22. Update the **CI/CD** section targets:
    - Update `ci-test` to include `test-cli`:
      ```makefile
      ci-test: clean-code test test-doc test-cli audit test-ci ## Run full CI test suite
      ```
    - Add a `ci-full` target that includes coverage:
      ```makefile
      ci-full: ci-test coverage ## Run full CI suite with coverage
      ```

### FR-25.8: Update `CONTRIBUTING.md` Coverage Documentation

23. Add a new section titled **"Code Coverage"** (or expand the existing coverage references) in `CONTRIBUTING.md` covering:

    a. **Prerequisites**: How to install `cargo-llvm-cov`:
       ```bash
       cargo install cargo-llvm-cov
       # Or faster via cargo-binstall:
       cargo binstall cargo-llvm-cov
       ```

    b. **Generating reports locally**:
       ```bash
       make coverage          # LCOV report → lcov.info
       make coverage-html     # HTML report → target/coverage/index.html
       ```

    c. **Understanding coverage output**: Brief explanation of LCOV format, how to read the HTML report, what "lines covered" vs "branches covered" means.

    d. **Codecov integration**: Explain that Codecov posts a PR comment showing coverage diff. Link to the project's Codecov dashboard (once available).

    e. **Threshold policy**: Document the phased rollout table (70% → 74% → 78%) and explain that:
       - `project` threshold applies to overall repository coverage.
       - `patch` threshold (80%) applies to _new code in the PR_.
       - Thresholds are enforced by Codecov status checks on the PR.

    f. **Troubleshooting**: Common issues:
       - `cargo-llvm-cov` not found → install instructions
       - Low coverage on new code → how to write tests for uncovered lines
       - Codecov upload failures → check `CODECOV_TOKEN` secret

24. Update any existing references to `cargo tarpaulin` in `CONTRIBUTING.md` to note that `cargo-llvm-cov` is the project standard (tarpaulin remains as an alternative).

### FR-25.9: Pin `cargo-llvm-cov` Installation in CI

25. In the new `ci.yml` coverage job, use `taiki-e/install-action@cargo-llvm-cov` instead of `cargo install cargo-llvm-cov`. This action:
    - Downloads pre-built binaries (30 seconds vs. 3–5 minutes for `cargo install`).
    - Handles version pinning.
    - Is the officially recommended installation method.

26. If `taiki-e/install-action` is used, pin to a specific `cargo-llvm-cov` version (e.g., `0.7.1`) for reproducibility:
    ```yaml
    - uses: taiki-e/install-action@v2
      with:
        tool: cargo-llvm-cov@0.7.1
    ```

27. Update `integration-tests.yml` to also use `taiki-e/install-action` instead of `cargo install` (if the coverage step is retained in that workflow).

### FR-25.10: Repository Secret Requirements

28. Document that a `CODECOV_TOKEN` repository secret must be configured in GitHub Settings → Secrets and variables → Actions.
    - This token is obtained from the [Codecov dashboard](https://app.codecov.io/) after adding the repository.
    - Without this token, coverage uploads may fail silently (especially on PRs from forks).

29. Add `CODECOV_TOKEN` to the `ci.yml` coverage job's environment variables or use it directly in the Codecov action.

---

## 5. Non-Goals (Out of Scope)

1. **Benchmark regression detection** — Only compile-checking benchmarks, not comparing results between runs. Regression detection via `critcmp` or `github-action-benchmark` is a future enhancement.
2. **New test creation** — This epic adds CI _jobs_ for existing tests. Writing new tests is covered by Epics 27–29.
3. **Deployment pipeline changes** — The `release.yml` workflow is updated only for action version bumps, not for new release process changes.
4. **Branch protection rules** — Configuring GitHub branch protection to require these new CI checks is a repository admin task, not a code change. It should be done after the CI jobs are verified working, but is not part of this epic's deliverables.
5. **Test parallelization / speed optimization** — CI job runtimes are not being optimized in this epic.
6. **Secrets rotation** — Adding the `CODECOV_TOKEN` secret to the repository is an ops task; this epic only documents the requirement.
7. **Coverage for test/bench/example files** — These are explicitly excluded in `.codecov.yml`. We only measure coverage of production code in `src/`.

---

## 6. Design Considerations

### Workflow Job Structure

The updated `ci.yml` should have this job dependency graph:

```
                    ┌─────────┐
                    │  lint    │
                    └────┬────┘
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
        ┌──────────┐ ┌────────┐ ┌───────────┐
        │   test   │ │cli-test│ │bench-check│
        │(stable/β)│ │(stable)│ │ (stable)  │
        └────┬─────┘ └────┬───┘ └─────┬─────┘
             │             │           │
             └──────┬──────┘           │
                    ▼                  │
              ┌──────────┐             │
              │ coverage │◄────────────┘
              │ (stable) │
              └────┬─────┘
                   ▼
              ┌──────────┐    ┌──────────────┐
              │  docker  │    │  integration  │
              └────┬─────┘    │    tests      │
                   │          └──────┬────────┘
                   ▼                 ▼
              ┌──────────┐    ┌──────────┐
              │ e2e-tests│    │ security │
              └──────────┘    └──────────┘
```

**Design choices:**
- `cli-tests` and `bench-check` run in parallel with `test` (no dependency on `lint` — fast feedback).
- `coverage` depends on all test jobs passing (no point generating coverage if tests fail).
- `docker` depends on `lint` and `test` (unchanged from current).
- `security` remains independent (unchanged).

### Alternative: Steps vs. Jobs

CLI tests and bench-check could be added as _steps_ within the existing `test` job instead of separate jobs.

**Separate jobs (recommended)**:
- Clearer failure messages (knows exactly which check failed)
- Parallel execution (faster total pipeline time)
- Easier to make optional or conditional later
- Individual re-run capability

**Steps (alternative)**:
- Fewer GitHub Actions runners consumed
- Simpler YAML structure
- No cache duplication

### Coverage Job Service Requirements

The combined coverage job needs Redis and MinIO services to run integration tests. This means:
- The `coverage` job in `ci.yml` must replicate the service configuration from the `integration-tests` job.
- Environment variables for test services must be set.
- This makes the coverage job heavier than a pure unit-test job but provides a comprehensive single report.

---

## 7. Technical Considerations

### Existing Workflow File Inventory

| File | Jobs | Changes Needed |
|------|------|----------------|
| `.github/workflows/ci.yml` | 7 jobs (lint, test, integration-tests, security, docker, e2e-tests, benchmark) | Add 3 jobs (cli-tests, bench-check, coverage), modernize actions, fix YAML |
| `.github/workflows/integration-tests.yml` | 3 jobs (integration-tests, docker-integration, k8s-smoke-test) | Modernize actions, evaluate removing duplicate coverage step |
| `.github/workflows/release.yml` | 3 jobs (create-release, build-docker, build-binaries) | Modernize actions only |

### Action Migration Reference

| Current | Replacement | Notes |
|---------|-------------|-------|
| `actions-rs/toolchain@v1` | `dtolnay/rust-toolchain@stable` | Drop-in replacement. Use `@beta` or `@nightly` where needed. |
| `actions/cache@v3` | `actions/cache@v4` | Same API, better performance. |
| `codecov/codecov-action@v3` | `codecov/codecov-action@v4` | Add `token: ${{ secrets.CODECOV_TOKEN }}`. |
| `actions/checkout@v3` | `actions/checkout@v4` | If any v3 instances remain. |
| `cargo install cargo-llvm-cov` | `taiki-e/install-action@v2` with `tool: cargo-llvm-cov@0.7.1` | 10× faster, pinned version. |

### `cargo-llvm-cov` Command Reference

```bash
# LCOV report (for Codecov upload)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# HTML report (for developer browsing)
cargo llvm-cov --all-features --workspace --html --output-dir target/coverage

# Specific module coverage (for targeted analysis)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info -- user_service

# Show summary in terminal
cargo llvm-cov --all-features --workspace
```

### Compatibility Notes

- `cargo-llvm-cov` requires the `llvm-tools` component. `dtolnay/rust-toolchain` installs this automatically when `cargo-llvm-cov` is used via `taiki-e/install-action`.
- The `--all-features` flag is important to include feature-gated tests (e.g., `live-api-tests`). However, live API tests require API keys — they will be skipped gracefully if keys aren't set.
- CLI snapshot tests (`insta`) may fail on first run if snapshots don't exist. All 43 snapshots were accepted in Epic 24, so this should not be an issue.

### YAML Validation

After all changes, validate workflows with:
```bash
# Install actionlint
brew install actionlint  # macOS
# or
go install github.com/rhysd/actionlint/cmd/actionlint@latest

# Validate
actionlint .github/workflows/ci.yml
actionlint .github/workflows/integration-tests.yml
actionlint .github/workflows/release.yml
```

### Files Created or Modified

| File | Action | Description |
|------|--------|-------------|
| `.github/workflows/ci.yml` | **Modified** | Add 3 jobs, modernize actions, fix YAML |
| `.github/workflows/integration-tests.yml` | **Modified** | Modernize actions, evaluate coverage step removal |
| `.github/workflows/release.yml` | **Modified** | Modernize actions only |
| `.codecov.yml` | **Created** | Coverage thresholds, ignore patterns, PR comments |
| `Makefile` | **Modified** | Add 4 new targets, update `ci-test` |
| `CONTRIBUTING.md` | **Modified** | Add/expand coverage documentation section |

---

## 8. Success Metrics

| Metric | Current | Phase 1 Target | Final Target | How to Measure |
|--------|---------|----------------|--------------|----------------|
| CLI tests in CI | Not run | Run + block on failure | Run + block on failure | `cli-tests` job status |
| Benchmark compile check | Not run | Run + block on failure | Run + block on failure | `bench-check` job status |
| Coverage reporting | Integration-only, no threshold | Combined report, 70% threshold | Combined report, 78% threshold | Codecov dashboard + PR checks |
| Codecov PR comments | None | Posted on every PR | Posted on every PR | PR comment presence |
| Deprecated actions | 3+ deprecated | 0 deprecated | 0 deprecated | Manual audit of YAML |
| YAML validity | Invalid (dangling block) | Valid | Valid | `actionlint` zero errors |
| Makefile targets | No coverage/CLI targets | 4 new targets working | 4 new targets working | `make coverage`, `make test-cli` exit 0 |
| CONTRIBUTING.md | Mentions tools, no setup | Full setup guide | Full setup guide | Manual review |
| Overall CI pipeline time | ~8–12 min (estimated) | ≤ 15 min | ≤ 15 min | GitHub Actions dashboard |

---

## 9. Open Questions

| ID | Question | Impact | Status |
|----|----------|--------|--------|
| OQ-1 | Should `CODECOV_TOKEN` be a repository secret or an organization-level secret? | Affects setup instructions and fork behavior. | **Open** |
| OQ-2 | Should the `coverage` job include `--all-features` (which enables `live-api-tests`)? Live API tests skip without keys, but the flag adds compilation overhead. | Could add 30–60 seconds to CI. | **Open** — recommend `--all-features` for completeness; skip is graceful. |
| OQ-3 | Should the existing `integration-tests.yml` coverage step be removed or kept with a different Codecov flag? | Duplicate uploads could confuse Codecov metrics. | **Open** — recommend removing once `ci.yml` coverage is verified working. |
| — | **Correction (dated 2026-08-13, Phase 15 / PIPE-02):** OQ-3 is **Answered: removed**, matching this row's own recommendation. The `integration-tests.yml` coverage step (`cargo llvm-cov --features integration-tests --lcov` plus `codecov/codecov-action@v3`) was deleted entirely, not retained with a different flag — `ci.yml`'s new `coverage` job supersedes it. See the correction banner at the top of this document. | see plan 15-02, commit `f9b5ad2` | Answered |
| OQ-4 | Should branch protection rules (requiring these new checks to pass before merge) be configured as part of this epic? | Enforcement only works if branch protection is enabled. | **Open** — recommend documenting as a follow-up ops task. |
| OQ-5 | Should `cargo-llvm-cov` version be pinned in the Makefile too (via a variable)? | Local dev vs CI version drift. | **Open** — recommend documenting minimum version in CONTRIBUTING.md but not pinning in Makefile. |
| OQ-6 | Should we add a `--fail-under-lines` flag to the `cargo llvm-cov` command in CI as a secondary enforcement mechanism (in addition to Codecov)? | Belt-and-suspenders approach vs. single source of truth. | **Open** — recommend Codecov-only threshold to avoid conflicting gates. |

---

## Appendix A: Deferred Task Traceability

| Original Epic 24 Task | Subtask | Description | FR in this PRD |
|------------------------|---------|-------------|----------------|
| 8.0 | 8.6 | Generate coverage report | FR-25.5, FR-25.6 |
| 8.0 | 8.9 | Read .github/workflows/ files | FR-25.1, FR-25.2 |
| 8.0 | 8.10 | Add CI job for CLI tests | FR-25.3 |
| 8.0 | 8.11 | Add CI job for benchmark compilation | FR-25.4 |
| 8.0 | 8.12 | Update CI to run coverage reporting | FR-25.5 |
| 8.0 | 8.13 | Verify CI configuration syntax | FR-25.2 |
| 6.0 | 6.11 | Re-generate coverage report | FR-25.5, FR-25.7 |
| 6.0 | 6.12 | Verify overall project coverage | FR-25.6 |
| 6.0 | 6.13 | Generate coverage badge/report | FR-25.5, FR-25.6 |

---

## Appendix B: Current `ci.yml` Job Listing (Pre-Change Reference)

| # | Job Name | Triggers | Services | Estimated Duration |
|---|----------|----------|----------|--------------------|
| 1 | `lint` | push/PR to main/develop | None | ~2 min |
| 2 | `test` | push/PR to main/develop | None | ~3 min (×2 matrix) |
| 3 | `integration-tests` | push/PR to main/develop | Redis, MinIO | ~5 min |
| 4 | `security` | push/PR to main/develop | None | ~2 min |
| 5 | `docker` | push/PR (after lint+test) | None | ~4 min |
| 6 | `e2e-tests` | push to main only | Redis, MinIO | ~8 min |
| 7 | `benchmark` | schedule/manual only | None | ~10 min |

**Post-change**: Jobs 8 (`cli-tests`), 9 (`bench-check`), 10 (`coverage`) added.

---

## Appendix C: `.codecov.yml` Threshold Rollout Schedule

```
Phase 1 (Sprint 1–2):  target: 70%   ← INITIAL DEPLOYMENT
Phase 2 (Sprint 3–4):  target: 74%   ← After Epics 28, 29 coverage improvements land
Phase 3 (Sprint 5+):   target: 78%   ← Steady state
```

To advance phases, update the `target:` value under `coverage.status.project.default` in `.codecov.yml` and commit.
