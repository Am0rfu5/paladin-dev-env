## Relevant Files

### Files to Modify

- `Dockerfile` — Add `--workspace` flag to `cargo build`; verify `COPY crates ./crates` is present (FR-05).
- `Dockerfile.chef` — Rewrite planner and cook stages for workspace: copy all `crates/*/Cargo.toml` files, run `cargo chef prepare` with workspace context, add `--workspace` to `cargo chef cook` and final `cargo build` (FR-01 to FR-04).
- `Makefile` — Add `--workspace` / `--all` flags to all build, test, lint, fmt, check, doc, bench targets; add 10 per-crate `test-<name>` targets (FR-08 to FR-20).
- `scripts/run_integration_tests.sh` — Update `cargo test` invocations to use `--workspace` (FR-31).

### Files to Create

- `.github/workflows/ci.yml` — GitHub Actions pipeline: per-crate matrix, workspace-level, integration, publish dry-run, and feature-flag jobs (FR-21 to FR-28).
- `docs/BUILD_BASELINES.md` — Build time and Docker image size baselines for the 10-crate workspace (FR-07).
- `docs/INTEGRATION_TESTS.md` — Integration test ownership map, run commands, and service requirements (FR-34).

### Reference Files (read but do not modify)

- `project/Milestone_7-Production-Hardening/Epic_2/prd-production-build-infra-adaptation.md` — Source of truth for all requirements.
- `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` — Prior baseline (6-crate workspace); reference for historical context in `BUILD_BASELINES.md`.
- `docker/docker-compose.test.yml` — Verified for workspace compatibility in Task 4.4.
- `tests/integration/*.rs` — Existing integration tests; all must pass after changes.
- `tests/functional/*.rs` — Existing functional tests; all must pass after changes.
- `Cargo.toml` — Workspace manifest; source of crate names and member list.

---

### Notes

- Unit tests in Rust live inside `#[cfg(test)]` modules in the same file. Integration tests live in `tests/`.
- Run `cargo test --workspace` after each task to confirm nothing is broken.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo fmt --all` before committing each parent task.
- Commit after each parent task using conventional commit format (`feat:`, `build:`, `ci:`, `docs:` etc.).
- The 10 workspace members are: `paladin`, `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory`, `paladin-storage`, `paladin-notifications`, `paladin-content`, `paladin-web`.

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:

- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

---

## Tasks

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/milestone_7-epic_2-build-infra`

- [x] 1.0 Adapt Docker build pipeline for workspace
  - [x] 1.1 Read `Dockerfile.chef` and `Dockerfile` in full to understand the current stage structure before making any changes.
  - [x] 1.2 Determine the latest stable `cargo-chef` version (`cargo search cargo-chef`) and pin it in the `chef` base stage of `Dockerfile.chef` (e.g., `RUN cargo install cargo-chef --version <x.y.z> --locked`).
  - [x] 1.3 Rewrite the `planner` stage of `Dockerfile.chef`: copy root `Cargo.toml` + `Cargo.lock` first, then one `COPY crates/<name>/Cargo.toml crates/<name>/Cargo.toml` line per crate (all 9), then `COPY src ./src` and `COPY crates ./crates`, then run `cargo chef prepare --recipe-path recipe.json`.
  - [x] 1.4 Update the `builder` stage of `Dockerfile.chef`: change `cargo chef cook` to `cargo chef cook --release --workspace --recipe-path recipe.json`.
  - [x] 1.5 Update the `app-builder` stage of `Dockerfile.chef`: add `COPY crates ./crates` after `COPY src ./src`; change `cargo build --release --bin paladin` to `cargo build --release --workspace --bin paladin`; remove `COPY config.yml ./` (config is provided at runtime via volume mount, not baked in).
  - [x] 1.6 Update `Dockerfile` (simple builder): confirm `COPY crates ./crates` is present; change `cargo build --release --bin paladin` to `cargo build --release --workspace --bin paladin`.
  - [ ] 1.7 Build `Dockerfile` and confirm it exits `0`: `docker build -f Dockerfile -t paladin-simple:test .` ⚠️ **Docker not available in dev container — verify in Docker-capable environment.**
  - [ ] 1.8 Build `Dockerfile.chef` cold-cache and confirm it exits `0`: `docker build -f Dockerfile.chef -t paladin-chef:test .` ⚠️ **Requires Docker — verify externally.**
  - [ ] 1.9 Verify the produced binary runs: `docker run --rm paladin-chef:test --help` ⚠️ **Requires Docker — verify externally.**
  - [x] 1.10 Run `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace` to confirm nothing is broken, then commit: `git commit -m "build(docker): adapt Dockerfile and Dockerfile.chef for 10-crate workspace"`.

- [ ] 2.0 Adapt Makefile for workspace
  - [ ] 2.1 Update the `build` target: change `cargo build` → `cargo build --workspace`.
  - [ ] 2.2 Update the `build-release` target: change `cargo build --release` → `cargo build --release --workspace`.
  - [ ] 2.3 Update the `test` target: change `cargo test --lib --bins` → `cargo test --workspace --lib --bins`.
  - [ ] 2.4 Update the `test-doc` target: change `cargo test --doc` → `cargo test --workspace --doc`.
  - [ ] 2.5 Update the `lint` target: change `cargo clippy --all-targets --all-features -- -D warnings` → `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
  - [ ] 2.6 Update the `fmt` target: change `cargo fmt --all` (already uses `--all`) — verify it is `--all`, not just `cargo fmt`.
  - [ ] 2.7 Update the `check` target: change `cargo check --all-targets` → `cargo check --workspace --all-targets`.
  - [ ] 2.8 Update the `doc` target: change `cargo doc --no-deps --document-private-items --open` → `cargo doc --workspace --no-deps --open`.
  - [ ] 2.9 Update the `bench` target: change `cargo bench` → `cargo bench --workspace`.
  - [ ] 2.10 Update the `ci-test` and `release-check` targets to call `$(MAKE) clean-code` (which now uses `--workspace` after 2.5–2.7 are done); no further change needed if they delegate to other targets.
  - [ ] 2.11 Add a new `##@ Per-Crate Testing` section to the Makefile with 10 `.PHONY` targets following this pattern for each of the 10 workspace members:
    ```makefile
    .PHONY: test-core
    test-core: ## Run tests for paladin-core
        @$(CARGO) test -p paladin-core
    ```
    Targets: `test-core`, `test-ports`, `test-battalion`, `test-llm`, `test-memory`, `test-storage`, `test-notifications`, `test-content`, `test-web`, `test-facade`.
  - [ ] 2.12 Run `make help` and verify all 10 per-crate targets appear in the output.
  - [ ] 2.13 Run `make clean-code` to confirm formatting, linting, and check all pass with workspace flags.
  - [ ] 2.14 Run each of the 10 per-crate targets (`make test-core`, `make test-ports`, … `make test-facade`) and confirm all exit `0`.
  - [ ] 2.15 Commit: `git commit -m "build(makefile): add --workspace flags and per-crate test targets"`.

- [ ] 3.0 Create GitHub Actions CI/CD pipeline
  - [ ] 3.1 Create directory `.github/workflows/` if it does not exist; create `.github/workflows/ci.yml` with the workflow name, `on:` trigger block (push to `main` and `feature/**`; pull_request targeting `main`), and a top-level `env:` block setting `CARGO_TERM_COLOR: always`.
  - [ ] 3.2 Check whether a `rust-toolchain.toml` file exists at the workspace root. If it does, read it and use that channel in the workflow. If not, read the `rust-version` field in `Cargo.toml`. Add a `toolchain:` input to each job's `actions-rs/toolchain` step (or `dtolnay/rust-toolchain`) set to the pinned version (e.g., `1.93`).
  - [ ] 3.3 Add the `test-crate` matrix job to `ci.yml`:
    - `strategy.matrix.crate`: list all 10 crate package names.
    - Steps: checkout → install Rust toolchain → restore cargo cache (`actions/cache` keyed on `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock', '**/Cargo.toml') }}`) → `cargo test -p ${{ matrix.crate }} --all-features`.
  - [ ] 3.4 Add the `test-workspace` job to `ci.yml`:
    - `needs: test-crate`.
    - Steps: checkout → toolchain → cache → `cargo test --workspace` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo fmt --all --check`.
  - [ ] 3.5 Add the `integration-tests` job to `ci.yml`:
    - `needs: test-workspace`.
    - Steps: checkout → toolchain → cache → `docker-compose -f docker/docker-compose.test.yml up -d` → wait/health-check step (sleep or `curl` retry loop) → `./scripts/run_integration_tests.sh -m ci` → `docker-compose -f docker/docker-compose.test.yml down` in an `if: always()` step.
  - [ ] 3.6 Add the `publish-dry-run` job to `ci.yml`:
    - `needs: test-workspace`.
    - `if: github.event_name == 'push' && github.ref == 'refs/heads/main'`.
    - Steps: checkout → toolchain → cache → sequential `cargo publish --dry-run -p <crate>` calls in dependency order (paladin-core → paladin-ports → paladin-battalion → paladin-llm → paladin-memory → paladin-storage → paladin-notifications → paladin-content → paladin-web → paladin).
  - [ ] 3.7 Add the `feature-flags` job to `ci.yml`:
    - `strategy.matrix.flags`: `["--no-default-features", "--all-features", ""]`.
    - Steps: checkout → toolchain → cache → `cargo build --workspace ${{ matrix.flags }}`.
  - [ ] 3.8 Validate `ci.yml` YAML syntax locally: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` (or `yq` if available).
  - [ ] 3.9 Commit: `git commit -m "ci: add GitHub Actions workflow with per-crate matrix, workspace, integration, and publish dry-run jobs"`.

- [ ] 4.0 Adapt integration test infrastructure
  - [ ] 4.1 Read `tests/integration/mod.rs` and scan the list of test files in `tests/integration/` to understand what's currently there.
  - [ ] 4.2 Audit each integration test file's imports: identify which tests import from a single crate (candidate for relocation to `crates/<name>/tests/`) vs. which import from multiple crates and must stay in `tests/integration/`. Record findings — most tests are expected to span multiple crates and stay at workspace root.
  - [ ] 4.3 Audit service dependencies: for each test file, identify whether it needs Redis, MinIO, MySQL, or none (look for `testcontainers`, `redis`, `minio`, or `mysql` env vars / imports). This information feeds `docs/INTEGRATION_TESTS.md`.
  - [ ] 4.4 Read `scripts/run_integration_tests.sh` fully. Identify every `cargo test` invocation and update each to use `--workspace` or `-p paladin` as appropriate so no tests are missed.
  - [ ] 4.5 Verify `docker/docker-compose.test.yml` can be brought up from the workspace root: `docker-compose -f docker/docker-compose.test.yml up -d && docker-compose -f docker/docker-compose.test.yml ps`. Document any path-related issues found (even if no changes are needed, note "verified OK").
  - [ ] 4.6 Run `make test-integration-docker` from the workspace root and confirm it exits `0`. Fix any failures before proceeding.
  - [ ] 4.7 Create `docs/INTEGRATION_TESTS.md` with three sections: (1) Test ownership table (file → crate or workspace-root, services required), (2) How to run integration tests locally (commands), (3) How services are started in CI.
  - [ ] 4.8 Commit: `git commit -m "ci: adapt integration test infrastructure for workspace; add INTEGRATION_TESTS.md"`.

- [ ] 5.0 Measure and document build baselines
  - [ ] 5.1 Run a clean workspace build 3 times and record each duration: `cargo clean && time cargo build --workspace` × 3. Record results.
  - [ ] 5.2 Measure per-crate incremental build times (3 runs each) by touching the crate's `lib.rs` and rebuilding: `touch crates/<name>/src/lib.rs && time cargo build -p <name>`. Measure for: `paladin-core`, `paladin-llm`, `paladin-battalion`, `paladin-storage`, `paladin-web`.
  - [ ] 5.3 Measure `docker build -f Dockerfile.chef .` cold-cache time (3 runs after `docker builder prune -af` between each): record each duration.
  - [ ] 5.4 Measure `docker build -f Dockerfile.chef .` warm-cache time with a source-only change (touch `src/main.rs` or equivalent, rebuild 3 times without pruning): record each duration.
  - [ ] 5.5 Record Docker image sizes: `docker image inspect paladin-chef:test --format '{{.Size}}'` and `docker image inspect paladin-simple:test --format '{{.Size}}'`. Convert to MB.
  - [ ] 5.6 Create `docs/BUILD_BASELINES.md` using the Milestone 5 `build-benchmarks.md` as a structural template. Include: environment table (CPU, RAM, OS, Rust toolchain, date, commit SHA), raw timings table with medians, summary table, and a brief analysis note. Reference the M5 benchmark file for historical comparison.
  - [ ] 5.7 Commit: `git commit -m "docs: add BUILD_BASELINES.md with 10-crate workspace build and Docker image measurements"`.

- [ ] 6.0 End-to-end verification
  - [ ] 6.1 Run `cargo build --workspace` — confirm exit `0`.
  - [ ] 6.2 Run `cargo test --workspace` — confirm exit `0`.
  - [ ] 6.3 Run `make clean-code` — confirm exit `0`.
  - [ ] 6.4 Run all 10 per-crate targets sequentially (`make test-core test-ports test-battalion test-llm test-memory test-storage test-notifications test-content test-web test-facade`) — confirm all exit `0`.
  - [ ] 6.5 Run `make test-integration-docker` — confirm exit `0`.
  - [ ] 6.6 Run `cargo publish --dry-run -p paladin-core` — confirm exit `0` (spot-check; the full ordered sequence runs in CI).
  - [ ] 6.7 Review the success metrics table in Section 8 of the PRD and confirm every "Required" row is green.
  - [ ] 6.8 Commit final state: `git commit -m "chore: milestone 7 epic 2 complete — production build infrastructure adapted for workspace"`.
