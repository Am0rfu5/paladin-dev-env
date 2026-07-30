# PRD: Production Build Infrastructure Adaptation

**Milestone:** 7 — Production Hardening (Tier 4)
**Epic:** 2 — Production Build Infrastructure Adaptation
**Status:** Ready for Implementation
**Author:** TBD
**Last Updated:** 2026-05-26

---

## 1. Introduction / Overview

The Paladin framework has been refactored over Milestones 1–7 Epic 1 from a monolithic crate into a nine-crate Cargo workspace:

```
paladin (facade)
├── paladin-core
├── paladin-ports
├── paladin-battalion
├── paladin-llm
├── paladin-memory
├── paladin-storage
├── paladin-notifications
├── paladin-content
└── paladin-web
```

The build tooling — `Dockerfile.chef`, `Dockerfile`, `Makefile`, and CI/CD pipeline — was originally written for a single-crate project. It does not understand workspace structure, does not provide per-crate isolation, and cannot cache or publish individual crates.

**Goal:** Update every layer of the build infrastructure so that the workspace builds correctly, efficiently, and with complete CI/CD coverage. Establish documented build-time and image-size baselines. Deliver a publishing dry-run pipeline ready to gate a future crates.io release.

---

## 2. Goals

1. `docker build` produces a working, optimally cached image from the workspace structure.
2. `cargo-chef` recipe covers all workspace member crates; the dependency layer only invalidates when a `Cargo.toml` changes.
3. All Makefile targets use `--workspace` flags; per-crate convenience targets exist for every workspace member.
4. A GitHub Actions CI/CD pipeline tests each crate in isolation (parallel jobs) and the full workspace together.
5. Integration test infrastructure (Docker Compose + `testcontainers`) runs correctly from the workspace root.
6. Build-time and Docker image-size baselines are measured and documented.
7. A `cargo publish --dry-run` stage is wired into CI for all publishable crates.

---

## 3. User Stories

**As a developer**, I want `make test` and `make lint` to run against all workspace crates automatically, so I don't have to remember per-crate commands.

**As a developer**, I want `make test-<crate>` targets (e.g., `make test-core`, `make test-battalion`), so I can run fast, focused tests during development on a specific crate without running the full suite.

**As a developer**, I want `docker build` to cache Rust dependencies efficiently across rebuilds, so I'm not waiting for a full dependency recompile when only application code changes.

**As a CI engineer**, I want per-crate parallel test jobs in GitHub Actions, so failures are pinpointed to the responsible crate and total CI time is minimised.

**As a release engineer**, I want a `cargo publish --dry-run` step to run automatically in CI, so I can confirm all crates are publishable before manually triggering a release.

**As a new contributor**, I want integration tests to work with a single `make test-integration-docker` command from the workspace root, so I don't need to know which crate owns which test.

---

## 4. Functional Requirements

### 4.1 Docker Build Pipeline (Task 2.1)

**FR-01** `Dockerfile.chef` (the `cargo-chef` variant) **must** be updated so the `planner` stage copies:
- Root `Cargo.toml` and `Cargo.lock`
- All `crates/*/Cargo.toml` files (nine crates)
- All source trees (`src/` and `crates/*/src/`)

**FR-02** The `cargo chef prepare` command **must** produce a `recipe.json` that captures all workspace member dependencies, not only the root crate's dependencies.

**FR-03** The `cargo chef cook` stage **must** run `cargo chef cook --release --workspace --recipe-path recipe.json` so all workspace dependency layers are pre-built and cached.

**FR-04** The application build stage **must** run `cargo build --release --workspace --bin paladin` to build from workspace context.

**FR-05** `Dockerfile` (the simple builder variant) **must** be updated to copy `crates/` alongside `src/`, `Cargo.toml`, and `Cargo.lock`. The `cargo build` command **must** use `--workspace`.

**FR-06** Both Dockerfiles **must** continue to produce a runnable `paladin` binary in a minimal runtime image (existing distroless / `debian:12-slim` base images must not change).

**FR-07** After the Docker changes, a `docs/BUILD_BASELINES.md` file **must** be created recording the following measurements for the **current 10-crate workspace** (post Milestone 7 Epic 1):
- `cargo build --workspace` clean build time (3 runs, report median).
- Per-crate incremental build times for: `paladin-core`, `paladin-llm`, `paladin-battalion`, `paladin-storage`, `paladin-web` (3 runs each, report median).
- `docker build -f Dockerfile.chef .` total time: cold cache (3 runs, median) and warm cache / source-only change (3 runs, median).
- Final compressed image size for both `Dockerfile.chef` and `Dockerfile` outputs.

> **Prior baseline:** `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` documents the 6-crate workspace (Milestone 5) clean and incremental build times. Use it as historical context but **do not** use it as the current baseline — it predates the 4 crates added in Milestone 7 Epic 1 and does not include Docker image size measurements. Milestone 6 architectural refinements are not expected to have materially changed build times, so no separate M6 baseline is required.

### 4.2 Makefile Workspace Adaptation (Task 2.2)

**FR-08** The `build` target **must** run `cargo build --workspace`.

**FR-09** The `build-release` target **must** run `cargo build --release --workspace`.

**FR-10** The `test` target **must** run `cargo test --workspace --lib --bins`.

**FR-11** The `test-doc` target **must** run `cargo test --workspace --doc`.

**FR-12** The `test-all` target **must** invoke `test`, `test-doc`, and `test-integration` in sequence.

**FR-13** The `lint` target **must** run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.

**FR-14** The `fmt` target **must** run `cargo fmt --all`.

**FR-15** The `check` target **must** run `cargo check --workspace --all-targets`.

**FR-16** The `clean-code` target **must** invoke `fmt`, `lint`, and `check` in sequence.

**FR-17** The `doc` target **must** run `cargo doc --workspace --no-deps`.

**FR-18** Per-crate test targets **must** exist for **every** workspace member using the pattern `test-<crate-short-name>`:

| Target | Command |
|---|---|
| `test-core` | `cargo test -p paladin-core` |
| `test-ports` | `cargo test -p paladin-ports` |
| `test-battalion` | `cargo test -p paladin-battalion` |
| `test-llm` | `cargo test -p paladin-llm` |
| `test-memory` | `cargo test -p paladin-memory` |
| `test-storage` | `cargo test -p paladin-storage` |
| `test-notifications` | `cargo test -p paladin-notifications` |
| `test-content` | `cargo test -p paladin-content` |
| `test-web` | `cargo test -p paladin-web` |
| `test-facade` | `cargo test -p paladin` |

**FR-19** Per-crate targets **must** be listed in `make help` output.

**FR-20** A `bench` target **must** run `cargo bench --workspace`.

### 4.3 GitHub Actions CI/CD Pipeline (Task 2.3)

**FR-21** A GitHub Actions workflow file **must** be created at `.github/workflows/ci.yml`.

**FR-22** The workflow **must** trigger on:
- `push` to `main` and `feature/**` branches.
- `pull_request` targeting `main`.

**FR-23** The workflow **must** contain a **per-crate matrix job** (`job: test-crate`) that:
- Runs in parallel for all nine workspace crates plus the facade crate.
- Executes `cargo test -p <crate-name>` for each matrix entry.
- Caches the `~/.cargo/registry`, `~/.cargo/git`, and `target/` directories using the `actions/cache` action, keyed on the OS, toolchain, and the hash of all `Cargo.lock` + `Cargo.toml` files.

**FR-24** The workflow **must** contain a **workspace-level job** (`job: test-workspace`) that:
- Depends on the per-crate matrix job completing successfully.
- Runs `cargo test --workspace`.
- Runs `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Runs `cargo fmt --all --check`.

**FR-25** The workflow **must** contain an **integration test job** (`job: integration-tests`) that:
- Depends on `test-workspace`.
- Spins up Redis, MinIO, and MySQL using `docker-compose -f docker/docker-compose.test.yml up -d`.
- Runs `./scripts/run_integration_tests.sh -m ci`.
- Tears down services in an `if: always()` step.

**FR-26** The workflow **must** contain a **publish dry-run job** (`job: publish-dry-run`) that:
- Depends on `test-workspace`.
- Runs only on pushes to `main` (not on pull requests).
- Executes `cargo publish --dry-run -p <crate>` for each crate in dependency order:
  1. `paladin-core`
  2. `paladin-ports`
  3. `paladin-battalion`
  4. `paladin-llm`
  5. `paladin-memory`
  6. `paladin-storage`
  7. `paladin-notifications`
  8. `paladin-content`
  9. `paladin-web`
  10. `paladin` (facade)
- **Must not** actually publish to crates.io (dry-run only).

**FR-27** The workflow **must** contain a **feature-flag matrix job** (`job: feature-flags`) that builds the workspace with the following feature-flag combinations to catch compilation errors under different configurations:
- `--no-default-features`
- `--all-features`
- Default features only (no extra flags)

**FR-28** All jobs **must** use a pinned Rust toolchain version consistent with the workspace's `rust-toolchain.toml` (or `Cargo.toml` `rust-version` field if present).

### 4.4 Integration Test Infrastructure Adaptation (Task 2.4)

**FR-29** Integration tests that import from multiple crates **must** live in the workspace-root `tests/` directory and depend on the `paladin` facade crate.

**FR-30** Integration tests that exercise a single crate in isolation **must** live in that crate's `tests/` directory (e.g., `crates/paladin-storage/tests/`).

**FR-31** `scripts/run_integration_tests.sh` **must** be reviewed and updated so it runs `cargo test --workspace --test '*'` (or the appropriate per-test invocation) rather than a single-crate path.

**FR-32** `docker/docker-compose.test.yml` **must** be verified to start and connect correctly when invoked from the workspace root. No path changes should be required; document any discoveries.

**FR-33** All existing integration tests **must** pass after the changes: `make test-integration-docker` must exit `0`.

**FR-34** A short document **must** be added at `docs/INTEGRATION_TESTS.md` describing:
- Which tests live where (workspace root vs. per-crate).
- How to run integration tests locally.
- What services each test group requires.

---

## 5. Non-Goals (Out of Scope)

- **Actual crates.io publishing.** The publish dry-run stage confirms readiness; the decision to publish is made separately.
- **New feature development.** No new orchestration patterns, LLM providers, or API endpoints.
- **Performance optimization.** Build times and image size are *measured* as baselines; deliberate optimization is deferred.
- **Kubernetes manifest changes.** Existing k8s documentation is sufficient.
- **GUI, dashboard, or monitoring tooling.**
- **Changing the Rust edition or MSRV.** Toolchain version must remain consistent with what Milestone 6 left it at.

---

## 6. Design Considerations

### Dockerfile Layer Ordering

For maximum cache efficiency in `Dockerfile.chef`, the COPY order **must** follow this pattern:

```dockerfile
# 1. Copy all Cargo.toml / Cargo.lock files first (rarely change)
COPY Cargo.toml Cargo.lock ./
COPY crates/paladin-core/Cargo.toml     crates/paladin-core/Cargo.toml
COPY crates/paladin-ports/Cargo.toml    crates/paladin-ports/Cargo.toml
# ... (one line per crate)

# 2. Run chef prepare (produces recipe.json from manifests only)
RUN cargo chef prepare --recipe-path recipe.json

# 3. Cook dependencies (this layer only rebuilds when manifests change)
RUN cargo chef cook --release --workspace --recipe-path recipe.json

# 4. Copy source code (changes frequently — comes last)
COPY src ./src
COPY crates ./crates
COPY migrations ./migrations
```

The `cargo chef prepare` step in the planner stage requires **stub source files** to exist for each crate (one empty `src/lib.rs` per crate), or must use the full source copy. Prefer copying the full source only after the recipe is prepared to keep the dependency cache layer tight.

### GitHub Actions Cache Key Strategy

Cache keys must be stable across runs when only source changes, but must invalidate when dependencies change:

```yaml
key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock', '**/Cargo.toml') }}
restore-keys: |
  ${{ runner.os }}-cargo-
```

### Per-Crate Publish Order

The `publish-dry-run` job must respect the dependency graph. The order in FR-26 is derived from the actual dependency relationships (`paladin-core` has no internal deps; `paladin-web` depends on `paladin-ports`, etc.). Violating this order will cause `cargo publish --dry-run` to fail.

---

## 7. Technical Considerations

### Existing Files to Modify

| File | Change Required |
|---|---|
| `Dockerfile` | Add `COPY crates ./crates`; add `--workspace` to `cargo build` |
| `Dockerfile.chef` | Full planner/cook stage rewrite for workspace (see FR-01 to FR-06) |
| `Makefile` | Add `--workspace` flags; add per-crate test targets (FR-08 to FR-20) |
| `scripts/run_integration_tests.sh` | Update cargo test invocation for workspace (FR-31) |

### New Files to Create

| File | Purpose |
|---|---|
| `.github/workflows/ci.yml` | Full GitHub Actions CI/CD pipeline (FR-21 to FR-28) |
| `docs/BUILD_BASELINES.md` | Build time and image size baselines (FR-07) |
| `docs/INTEGRATION_TESTS.md` | Integration test structure documentation (FR-34) |

### Workspace Members

The complete list of workspace members (for use in matrix configs and per-crate targets):

```
paladin, paladin-core, paladin-ports, paladin-battalion,
paladin-llm, paladin-memory, paladin-storage,
paladin-notifications, paladin-content, paladin-web
```

### `cargo-chef` Version

Pin `cargo-chef` to a specific version in `Dockerfile.chef` to avoid unexpected breakage. Verify the installed version is compatible with the workspace resolver `"2"` setting in `Cargo.toml`.

### Feature Flags in CI

Several crates use feature flags (e.g., `paladin-llm` has `openai`, `anthropic`, `deepseek`). The per-crate matrix jobs should test with `--all-features` unless a crate-specific exception is documented in `docs/FEATURE_FLAGS.md`.

---

## 8. Success Metrics

| Metric | Target |
|---|---|
| `cargo build --workspace` exits `0` | Required |
| `cargo test --workspace` exits `0` | Required |
| `make clean-code` exits `0` (fmt + clippy + check) | Required |
| `docker build -f Dockerfile.chef .` exits `0` | Required |
| `docker build -f Dockerfile .` exits `0` | Required |
| All 10 per-crate `make test-<name>` targets exit `0` | Required |
| GitHub Actions CI pipeline green on `main` | Required |
| `publish-dry-run` job exits `0` for all 10 crates | Required |
| `make test-integration-docker` exits `0` | Required |
| Warm-cache Docker rebuild time (source change only) | Documented in `BUILD_BASELINES.md` |
| Docker image size (both Dockerfiles) | Documented in `BUILD_BASELINES.md` |
| Docker image size regression vs. hypothetical monolithic build | Within 10% (target from Epic spec) |

---

## 9. Open Questions

1. **`rust-toolchain.toml`**: Does the workspace have a `rust-toolchain.toml` file, or should one be created as part of this Epic to pin the toolchain for CI? If it already exists, confirm the version is used consistently in the `ci.yml`.

2. **Crate name reservation**: Before `publish-dry-run` is wired up, have the `paladin-*` crate names been reserved on crates.io? If not, `cargo publish --dry-run` may succeed locally but fail if names are squatted at actual publish time. This is a prerequisite check, not a blocking item for this Epic.

3. **Integration test service dependencies**: Which integration tests require which services (Redis, MinIO, MySQL)? This mapping is needed to write `docs/INTEGRATION_TESTS.md` accurately. A quick audit of `tests/integration/` should answer this.

4. **`cargo-chef` stub source strategy**: Does the current `cargo chef prepare` command in `Dockerfile.chef` require actual source files to be present, or can it work from manifests only? Confirm the correct `cargo-chef` invocation pattern before rewriting the stage.

5. **GitHub Actions runner**: Is `ubuntu-latest` the target runner, or is there a specific version requirement (e.g., `ubuntu-22.04`) for compatibility with the `debian:12-bookworm` base image used in the Dockerfiles?
