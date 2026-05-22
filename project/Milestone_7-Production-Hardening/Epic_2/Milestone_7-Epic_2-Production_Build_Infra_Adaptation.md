## Epic 2: Production Build Infrastructure Adaptation

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (workspace structure finalized)

### Objective

Adapt all build, test, and deployment tooling — Docker images, Makefile targets, CI/CD pipeline, and integration test infrastructure — to work correctly and efficiently with the multi-crate workspace.

### Background & Rationale

The current build infrastructure was designed for a monolithic crate:

- **Docker:** The `Dockerfile.chef` and `Dockerfile` copy `Cargo.toml`, `Cargo.lock`, and `src/` as a single unit. With a workspace, they must copy the root `Cargo.toml`, all `crates/*/Cargo.toml` files, and all source trees. The `cargo-chef` recipe must account for workspace member dependencies.

- **Makefile:** Targets like `make test`, `make lint`, `make build-release` use single-crate cargo commands. They need `--workspace` flags and per-crate variant targets.

- **CI/CD:** The pipeline runs `cargo test`, `cargo clippy`, and `cargo fmt` as monolithic commands. It needs to add per-crate isolated builds, a dependency-aware test ordering, and eventually per-crate publishing steps.

- **Integration tests:** Tests use `testcontainers` and Docker Compose for Redis, MinIO, and MySQL. The test binary imports from the monolithic `paladin` crate. With the workspace, integration tests should live in the workspace root and depend on the facade crate, or in per-crate `tests/` directories where appropriate.

### Acceptance Criteria

1. `docker build` produces a working image from the workspace structure with efficient layer caching.
2. `cargo-chef` recipe handles workspace dependencies correctly; dependency layer cache invalidates only when `Cargo.toml` files change.
3. Makefile targets work with the workspace: `make test`, `make lint`, `make build-release`, `make bench`, `make doc`.
4. Per-crate Makefile targets available: `make test-core`, `make test-battalion`, etc.
5. CI/CD pipeline tests each crate in isolation and the full workspace.
6. Integration test infrastructure (Docker Compose, testcontainers) works with workspace test binaries.
7. Docker image size does not regress significantly from the monolithic build (target: within 10%).

### Tasks

#### Task 2.1: Adapt Docker Build Pipeline

**Description:** Update `Dockerfile.chef` and `Dockerfile` to work with the workspace structure. The `COPY` instructions must include all crate `Cargo.toml` files for the chef recipe phase. The application build stage must handle workspace member resolution.

**Deliverables:**
- Updated `Dockerfile.chef` with workspace-aware COPY and build stages.
- Updated `Dockerfile` (simple builder) with workspace support.
- Docker image builds successfully from workspace root.
- Build time and image size benchmarked against monolithic baseline.

**Estimated Effort:** Medium

#### Task 2.2: Adapt Makefile for Workspace

**Description:** Update all Makefile targets to use workspace-aware cargo commands. Add per-crate convenience targets.

**Deliverables:**
- Updated Makefile with `--workspace` flags on build, test, lint, fmt, doc targets.
- New per-crate targets: `test-core`, `test-ports`, `test-battalion`, `test-llm`, `test-memory`, etc.
- `make test-all` runs full workspace test suite including integration tests.
- `make clean-code` runs `fmt --all`, `clippy --workspace`, `check --workspace`.

**Estimated Effort:** Small

#### Task 2.3: Adapt CI/CD Pipeline

**Description:** Update the CI configuration to:
- Build and test each crate in isolation (parallel jobs).
- Build and test the full workspace.
- Run the feature-flag matrix at workspace level.
- Cache workspace target directories efficiently.
- Add a publishing stage (dry-run mode initially) for per-crate `cargo publish`.

**Deliverables:**
- Updated CI configuration with per-crate parallel test jobs.
- Workspace-level integration test job.
- Publishing dry-run stage.
- Cache configuration optimized for workspace builds.

**Estimated Effort:** Medium

#### Task 2.4: Adapt Integration Test Infrastructure

**Description:** Ensure integration tests that depend on Docker services (Redis, MinIO, MySQL/SQLite) work correctly with the workspace structure. Tests that span multiple crates should live in the workspace root `tests/` directory. Per-crate integration tests should be self-contained.

**Deliverables:**
- Integration test structure documented (which tests live where).
- `scripts/run_integration_tests.sh` updated for workspace.
- `docker-compose.test.yml` verified compatible.
- All existing integration tests pass from the workspace root.

**Estimated Effort:** Medium

---
