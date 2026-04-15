# Milestone 4: Production Hardening and Extended Workspace Decomposition

**Project:** Paladin Framework Refactoring Initiative
**Milestone:** Tier 4 — Production Hardening, Extended Crate Extraction, and API Stabilization
**Status:** Planning
**Target Start:** Upon completion of Milestone 3 (Tier 3)
**Target Completion:** TBD
**Document Version:** 1.0
**Last Updated:** 2026-04-14

---

## Executive Summary

Milestone 4 is the capstone of the Paladin refactoring initiative. Where Milestones 1–3 focused on structural correctness — feature isolation, workspace decomposition, and architectural layering — Milestone 4 focuses on production readiness, extended modularity, and the operational foundation needed to ship Paladin as a publishable, independently versionable, enterprise-grade crate ecosystem.

This Milestone addresses three themes that emerged from the Tier 3 retrospective deliverables and from the natural trajectory of the refactoring:

1. **Extended Workspace Decomposition** — Extract the remaining infrastructure subsystems (content processing, web server, notifications, repositories) into their own optional crates. This completes the "full workspace decomposition" described in the Init-Analysis as the right long-term shape.

2. **Production Build Infrastructure** — Adapt the Docker multi-stage build pipeline, CI/CD workflows, Makefile targets, benchmarking suite, and integration test infrastructure to the workspace structure. The current build tooling was designed for a monolithic crate and must be updated.

3. **API Stabilization and Pre-Release Preparation** — Formalize the public API across all crates, implement semantic versioning discipline, prepare crate metadata for crates.io publishing, and produce the documentation suite needed for a 0.2.0 (or 1.0.0) release.

### Prerequisites (Completed in Milestones 1–3)

- Feature flags expanded and CI matrix in place (Milestone 1).
- Core workspace crates extracted: `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-memory` (Milestone 2).
- Facade crate with backward-compatible re-exports (Milestone 2).
- `application_settings.rs` decomposed into per-domain config modules (Milestone 3).
- Manager services relocated to the application layer; core layer purity verified (Milestone 3).
- Maneuver DSL co-located in `paladin-battalion` (Milestone 3).
- `CircuitBreaker` relocated to infrastructure layer (Milestone 3).
- Architecture compliance report produced (Milestone 3).

### Success Criteria

- The workspace contains all logically separable crates; a downstream consumer can depend on exactly the crates they need.
- Docker images build from the workspace structure with per-crate layer caching.
- CI/CD pipeline supports per-crate testing, per-crate publishing, and workspace-level release orchestration.
- Every public crate has complete `rustdoc`, a `README.md`, a `CHANGELOG.md`, and correct crates.io metadata.
- All benchmarks pass and baseline performance is documented.
- Integration test infrastructure works with the workspace and Docker services.
- A release candidate tag can be cut with confidence.

---

## Milestone Scope & Boundaries

### In Scope

- Extraction of `paladin-web` crate (web server and API infrastructure).
- Extraction of `paladin-notifications` crate (notification adapters).
- Extraction of `paladin-content` crate (content processing pipeline).
- Extraction of `paladin-storage` crate (SQL repositories and migration management).
- Docker build pipeline adaptation for the workspace.
- CI/CD workspace publishing pipeline.
- Benchmark suite migration and baseline documentation.
- Integration test infrastructure workspace adaptation.
- Per-crate crates.io metadata and documentation preparation.
- Semantic versioning policy and release orchestration.

### Out of Scope

- New feature development (new orchestration patterns, new LLM providers).
- Performance optimization beyond documenting baselines.
- Actual crates.io publishing (this Milestone prepares for it; publishing is a release decision).
- Kubernetes deployment manifests (existing documentation is sufficient).
- GUI or dashboard development.

### Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Extended crate extraction yields diminishing returns for the effort | Medium | Low | Cost-benefit assessment per crate (Epic 1 Task 1.1) before committing to extraction; some extractions may be deferred |
| Docker workspace builds significantly slower due to multi-crate structure | Medium | Medium | Use `cargo-chef` with workspace recipe; layer caching per crate; benchmark build times |
| Per-crate versioning complexity creates dependency hell | Medium | High | Start with lockstep versioning (all crates share a version); transition to independent versions only when stability warrants it |
| Integration tests require all crates assembled, defeating isolation benefits | Low | Medium | Integration tests live in the workspace root `tests/` and depend on the facade crate; per-crate tests remain isolated |
| crates.io name squatting on `paladin-*` crate names | Low | High | Reserve crate names early with placeholder publishes; use `paladin-framework-*` prefix if needed |

---

## Epic 1: Extended Workspace Decomposition — Remaining Infrastructure Crates

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Large
**Dependencies:** Milestone 3 complete

### Objective

Complete the full workspace decomposition by extracting the four remaining infrastructure subsystems into their own optional crates. After this Epic, the `paladin` facade crate contains only re-exports, the application-layer use-case services not covered by dedicated crates, and the `ServiceRunner` composition root. Every substantial subsystem is independently compilable and testable.

### Background & Rationale

After Milestone 2, the workspace contains six crates. The remaining infrastructure code in the facade crate includes:

- **Web server** (`infrastructure/web/`, `infrastructure/adapters/output/api_content_deliverer.rs`) — Actix-web REST API, WebSocket handlers, middleware, content delivery endpoints. Gated behind the `web-server` feature flag since Milestone 1 but still compiled as part of the facade crate. Approximately 3–4k LOC plus the `actix-web` dependency tree.

- **Notification adapters** (`infrastructure/adapters/notifications/`) — Email (`lettre`), push, and system notification adapters implementing `NotificationDeliveryPort` and `NotificationTemplatePort`. The notification service coordinator was relocated to the application layer in Milestone 3; the concrete adapters remain in infrastructure. Approximately 2k LOC.

- **Content processing** (`infrastructure/adapters/document/`, `infrastructure/adapters/input/`, `application/use_cases/content/`) — PDF extraction (`pdf-extract`), web scraping (`scraper`), RSS ingestion (`rss`), HTTP content fetching (`reqwest` blocking client), content aggregation, filtering, summarization, and analysis services. Approximately 5–6k LOC with heavy external dependencies.

- **SQL repositories** (`infrastructure/repositories/`) — SQLite and MySQL repository implementations using `sqlx`, migration management, and the `SqliteStore` abstraction. Approximately 3k LOC.

Each of these is cleanly isolated behind feature flags and port traits. Extraction to separate crates makes the dependency cost explicit and opt-in at the Cargo dependency level rather than just the feature-flag level.

### Acceptance Criteria

1. Four new crates exist: `paladin-web`, `paladin-notifications`, `paladin-content`, `paladin-storage`.
2. Each crate depends only on `paladin-core` and `paladin-ports` (and optionally on shared workspace dependencies).
3. The facade crate's direct source code is reduced to re-exports, the `ServiceRunner` composition root, and application-layer use-case services that don't warrant their own crate.
4. `cargo build --workspace` succeeds.
5. `cargo test --workspace` passes all tests.
6. A downstream consumer can depend on `paladin-core` + `paladin-ports` + `paladin-battalion` + `paladin-llm` without transitively pulling in `actix-web`, `lettre`, `pdf-extract`, `scraper`, `sqlx`, or any other infrastructure dependency from the extracted crates.

### Tasks

#### Task 1.1: Cost-Benefit Assessment per Extraction

**Description:** Before extracting, assess each candidate crate for extraction value vs. effort. Criteria: (a) dependency weight introduced by the subsystem, (b) frequency of independent change, (c) likelihood of downstream consumers wanting it without the rest, (d) extraction complexity. Any subsystem that scores low may be deferred with justification.

**Deliverables:**
- Cost-benefit matrix for all four candidate extractions.
- Go/defer decision for each.
- Updated Epic scope if any extractions are deferred.

**Estimated Effort:** Small

#### Task 1.2: Extract `paladin-web` Crate

**Description:** Move the web server infrastructure — Actix-web application factory, REST API route handlers, WebSocket handlers, middleware, and `ApiContentDeliverer` — into `crates/paladin-web/`. The crate depends on `paladin-core`, `paladin-ports`, and `actix-web`.

**Deliverables:**
- `crates/paladin-web/Cargo.toml` with `actix-web` as a primary dependency.
- All web-related modules relocated.
- `ServiceRunner` updated to conditionally depend on `paladin-web`.
- Web-related integration tests migrated.
- `cargo build -p paladin-web` succeeds in isolation.

**Estimated Effort:** Medium

#### Task 1.3: Extract `paladin-notifications` Crate

**Description:** Move the notification adapter implementations — `EmailNotificationAdapter` (with `lettre`), `SystemNotificationAdapter`, and `PushNotificationAdapter` — into `crates/paladin-notifications/`. Feature flags: `email` (gates `lettre` + `handlebars`), `push`, `system`.

**Deliverables:**
- `crates/paladin-notifications/Cargo.toml` with per-channel feature flags.
- Adapter modules relocated.
- Notification integration tests migrated.
- `cargo build -p paladin-notifications --no-default-features` succeeds.

**Estimated Effort:** Medium

#### Task 1.4: Extract `paladin-content` Crate

**Description:** Move the content processing pipeline — `PdfExtractor`, `HttpContentFetcher`, `FileContentListFetcher`, `NewsApiFetcher`, RSS adapter, web scraper, content aggregation/filtering/summarization/analysis use-case services — into `crates/paladin-content/`. Feature flags: `pdf` (gates `pdf-extract`), `web-scraping` (gates `scraper`), `rss` (gates `rss` crate).

**Deliverables:**
- `crates/paladin-content/Cargo.toml` with per-capability feature flags.
- Content processing modules and use-case services relocated.
- Content-related tests migrated.
- `cargo build -p paladin-content --no-default-features` succeeds.

**Estimated Effort:** Large

#### Task 1.5: Extract `paladin-storage` Crate

**Description:** Move SQL repository implementations — `SqliteStore`, `SqliteUserRepository`, `SqliteContentRepository`, MySQL variants, and migration management — into `crates/paladin-storage/`. Feature flags: `sqlite` (gates `sqlx/sqlite`), `mysql` (gates `sqlx/mysql`).

**Deliverables:**
- `crates/paladin-storage/Cargo.toml` with database backend feature flags.
- Repository modules and migration files relocated.
- Database integration tests migrated.
- `cargo build -p paladin-storage --features sqlite` succeeds.

**Estimated Effort:** Medium

#### Task 1.6: Update Facade Crate and Workspace Metadata

**Description:** Update the `paladin` facade crate to depend on all new crates with appropriate default features. Verify backward-compatible re-exports. Update `[workspace]` members list and `[workspace.dependencies]`.

**Deliverables:**
- Facade crate updated with new dependencies.
- All re-export paths verified.
- `cargo test --workspace` passes.
- `cargo doc --workspace --no-deps` clean.

**Estimated Effort:** Medium

---

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

## Epic 3: Benchmark Suite Migration and Performance Baseline

**Epic Owner:** TBD
**Priority:** Medium
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (crate structure finalized), Epic 2 (Makefile targets available)

### Objective

Migrate the benchmark suite to the workspace structure, assign benchmarks to their appropriate crates, establish baseline performance measurements for all critical paths, and document the results as the pre-release performance baseline.

### Background & Rationale

The current benchmark suite includes:

- `benches/sanctum_benchmarks.rs` — Vector store performance (search, insert, batch operations).
- Disabled benchmarks: `battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks`, `paladin_benchmarks`, `arsenal_benchmarks` — noted in `Cargo.toml` as needing API fixes.

With the workspace decomposition, benchmarks should live in the crate they measure: sanctum benchmarks in `paladin-memory`, battalion benchmarks in `paladin-battalion`, etc. Additionally, the disabled benchmarks should be evaluated for reactivation now that the API has been refactored.

### Acceptance Criteria

1. Active benchmarks (`sanctum_benchmarks`) migrated to `paladin-memory/benches/`.
2. Disabled benchmarks evaluated; reactivated where possible with the refactored API, or formally deprecated with documented reasons.
3. New benchmarks added for critical paths: battalion orchestration (Formation, Phalanx, Campaign execution), LLM adapter call overhead, garrison read/write, and config loading.
4. `cargo bench --workspace` runs all active benchmarks.
5. Baseline performance document produced with measurements, hardware specification, and methodology.
6. Performance regression CI gate established (optional — flag regressions exceeding a threshold).

### Tasks

#### Task 3.1: Migrate Active Benchmarks to Crate Locations

**Description:** Move `sanctum_benchmarks.rs` to `paladin-memory/benches/`. Update `Cargo.toml` `[[bench]]` entries in the appropriate crate.

**Deliverables:**
- Benchmark files relocated.
- `cargo bench -p paladin-memory` runs sanctum benchmarks.
- Results match pre-migration baseline within noise margin.

**Estimated Effort:** Small

#### Task 3.2: Evaluate and Reactivate Disabled Benchmarks

**Description:** Review each disabled benchmark (`battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks`, `paladin_benchmarks`, `arsenal_benchmarks`) against the current API. Fix or rewrite where the API has changed. Formally deprecate any that are no longer meaningful.

**Deliverables:**
- Assessment document for each disabled benchmark.
- Reactivated benchmarks placed in their appropriate crates.
- Deprecated benchmarks removed with `CHANGELOG.md` entry.

**Estimated Effort:** Medium

#### Task 3.3: Add Critical Path Benchmarks

**Description:** Write new benchmarks for the core performance-sensitive operations:
- Battalion: Formation (3-agent sequential), Phalanx (5-agent parallel), Campaign (DAG with branches). Uses mock `PaladinPort` to isolate orchestration overhead.
- LLM: Adapter request/response serialization overhead (excluding actual HTTP call).
- Garrison: In-memory read/write at various history sizes (100, 1000, 10000 entries).
- Config: `Settings::new()` and per-domain config loading.

**Deliverables:**
- Benchmark files in their respective crates.
- `cargo bench --workspace` exercises all new benchmarks.

**Estimated Effort:** Medium

#### Task 3.4: Document Performance Baseline

**Description:** Run the full benchmark suite on a standardized environment. Produce a performance baseline document with methodology, hardware specs, results, and analysis.

**Deliverables:**
- `docs/PERFORMANCE_BASELINE.md` with all benchmark results.
- Hardware and environment specification.
- Analysis of any performance-sensitive areas.
- Comparison to pre-workspace monolithic build (where comparable benchmarks exist).

**Estimated Effort:** Small

---

## Epic 4: API Stabilization and Pre-Release Preparation

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Large
**Dependencies:** Epics 1–3

### Objective

Prepare the Paladin crate ecosystem for its first publishable release. This includes finalizing the public API surface across all crates, implementing semantic versioning discipline, preparing crate metadata for crates.io, producing per-crate documentation, and running a release readiness audit.

### Background & Rationale

The workspace now contains approximately 10 crates. Each needs:

- A correct `Cargo.toml` with `[package]` metadata (description, license, repository, keywords, categories, documentation URL).
- A crate-level `README.md` for crates.io rendering.
- A crate-level `CHANGELOG.md` following Keep a Changelog format.
- Explicit `#![deny(missing_docs)]` or documented public API coverage.
- Semantic version alignment (lockstep initially, with a path to independent versioning).

The public API was hardened in Milestone 1 (Epic 2) and documented in `STABLE_API.md`. This Epic extends that work to the per-crate level and formalizes the release process.

### Acceptance Criteria

1. Every public crate has a complete `[package]` section with description, license (`MIT`), repository URL, keywords, and categories.
2. Every public crate has a `README.md` that renders correctly on crates.io.
3. Every public crate has a `CHANGELOG.md`.
4. `cargo publish --dry-run -p <crate>` succeeds for every publishable crate.
5. A versioning policy document defines lockstep vs. independent versioning rules.
6. A release checklist covers all steps from code freeze to publish.
7. `#![warn(missing_docs)]` enabled on all public crates; documentation coverage exceeds 90% of public items.
8. `STABLE_API.md` updated to the per-crate level with stability tiers per type.
9. The complete documentation suite is production-ready.

### Tasks

#### Task 4.1: Prepare Crate Metadata

**Description:** For each public crate, add or update the `[package]` section in `Cargo.toml` with: `name`, `version` (from `workspace.package`), `edition`, `authors`, `description`, `readme`, `repository`, `license`, `keywords`, `categories`, `documentation`.

**Deliverables:**
- All crate `Cargo.toml` files updated with complete metadata.
- `cargo publish --dry-run -p <crate>` succeeds for all crates.
- No crates.io validation errors.

**Estimated Effort:** Small

#### Task 4.2: Write Per-Crate README Files

**Description:** Each crate gets a `README.md` that describes the crate's purpose, key types, usage examples, feature flags, and links to the full documentation. The root `README.md` serves as the umbrella project overview with links to per-crate READMEs.

**Deliverables:**
- `crates/paladin-core/README.md`
- `crates/paladin-ports/README.md`
- `crates/paladin-battalion/README.md`
- `crates/paladin-llm/README.md`
- `crates/paladin-memory/README.md`
- `crates/paladin-web/README.md` (if extracted)
- `crates/paladin-notifications/README.md` (if extracted)
- `crates/paladin-content/README.md` (if extracted)
- `crates/paladin-storage/README.md` (if extracted)
- Updated root `README.md`.

**Estimated Effort:** Medium

#### Task 4.3: Write Per-Crate CHANGELOG Files

**Description:** Initialize per-crate `CHANGELOG.md` files with the history relevant to each crate (extracted from the monolithic `CHANGELOG.md`). Establish the format and conventions for future changelog maintenance.

**Deliverables:**
- Per-crate `CHANGELOG.md` files.
- Contribution guide updated with changelog maintenance instructions.

**Estimated Effort:** Small

#### Task 4.4: Documentation Coverage Audit

**Description:** Enable `#![warn(missing_docs)]` on all public crates. Run `cargo doc --workspace --no-deps` and fix all warnings. Audit documentation coverage — every public type, trait, function, and module should have a doc comment.

**Deliverables:**
- `#![warn(missing_docs)]` enabled in all crate `lib.rs` files.
- All documentation warnings resolved.
- `cargo doc --workspace --no-deps` produces zero warnings.
- Coverage report showing percentage of documented public items per crate.

**Estimated Effort:** Medium

#### Task 4.5: Define Versioning Policy and Release Process

**Description:** Produce a versioning policy document that defines:
- Initial lockstep versioning (all crates share a version, e.g., `0.2.0`).
- Criteria for transitioning to independent per-crate versioning.
- What constitutes a breaking change for each crate.
- The release checklist: code freeze → changelog finalization → version bump → CI green → dry-run publish → publish → tag → announcement.
- Publishing order (dependency-first: `paladin-core` → `paladin-ports` → leaf crates → `paladin` facade).

**Deliverables:**
- `docs/VERSIONING_POLICY.md`.
- `docs/RELEASE_CHECKLIST.md`.
- Publishing order documented and scripted (or Makefile target `make release`).

**Estimated Effort:** Medium

#### Task 4.6: Update STABLE_API.md to Per-Crate Level

**Description:** Extend `STABLE_API.md` to document the public API surface of each crate individually. Include stability tiers (Stable, Unstable, Experimental) per type. This becomes the contract for backward compatibility guarantees.

**Deliverables:**
- Per-crate sections in `STABLE_API.md` (or per-crate `API.md` files).
- Stability tier assigned to every public type and trait.
- Cross-crate dependency contract documented.

**Estimated Effort:** Medium

#### Task 4.7: Release Readiness Audit

**Description:** Conduct a comprehensive release readiness audit:
- All tests pass (`cargo test --workspace`).
- All clippy clean (`cargo clippy --workspace -- -D warnings`).
- All formatted (`cargo fmt --all -- --check`).
- All docs build (`cargo doc --workspace --no-deps`).
- All dry-run publishes succeed.
- Security audit (`cargo audit`).
- License compliance verified (all dependencies compatible with MIT).
- Binary size and dependency tree reviewed for unexpected bloat.

**Deliverables:**
- Release readiness audit report.
- Any blocking issues identified and resolved.
- Sign-off for release candidate tag.

**Estimated Effort:** Medium

---

## Milestone Schedule Overview

| Phase | Epic | Estimated Duration | Predecessors |
|-------|------|--------------------|--------------|
| Phase 1 | Epic 1: Extended Workspace Decomposition | 3–4 sprints | Milestone 3 complete |
| Phase 2A | Epic 2: Build Infrastructure Adaptation | 2–3 sprints | Epic 1 |
| Phase 2B | Epic 3: Benchmark Migration (parallel with Epic 2) | 1–2 sprints | Epic 1 |
| Phase 3 | Epic 4: API Stabilization and Pre-Release | 2–3 sprints | Epics 1–3 |

**Total Estimated Duration:** 5–8 sprints (Epics 2 and 3 parallelizable)

---

## Completion Checklist

### Extended Decomposition
- [ ] Cost-benefit assessment completed; go/defer decisions documented.
- [ ] `paladin-web` extracted (if approved).
- [ ] `paladin-notifications` extracted (if approved).
- [ ] `paladin-content` extracted (if approved).
- [ ] `paladin-storage` extracted (if approved).
- [ ] Facade crate updated with new dependencies and re-exports.
- [ ] Minimal dependency consumer verified (`core` + `ports` + `battalion` + `llm` only).

### Build Infrastructure
- [ ] Docker workspace build working with layer caching.
- [ ] Docker image size within 10% of monolithic baseline.
- [ ] Makefile targets updated for workspace; per-crate targets available.
- [ ] CI/CD pipeline runs per-crate isolated builds and tests.
- [ ] CI/CD publishing dry-run stage green.
- [ ] Integration test infrastructure functional with workspace.

### Benchmarks
- [ ] Active benchmarks migrated to per-crate locations.
- [ ] Disabled benchmarks evaluated; reactivated or formally deprecated.
- [ ] Critical path benchmarks added (battalion, LLM, garrison, config).
- [ ] Performance baseline document published.

### API Stabilization
- [ ] All crate `Cargo.toml` metadata complete; dry-run publishes succeed.
- [ ] Per-crate `README.md` files written.
- [ ] Per-crate `CHANGELOG.md` files initialized.
- [ ] `#![warn(missing_docs)]` enabled; zero doc warnings.
- [ ] Versioning policy and release checklist documented.
- [ ] `STABLE_API.md` extended to per-crate level with stability tiers.
- [ ] Release readiness audit passed.
- [ ] Security audit (`cargo audit`) clean.
- [ ] License compliance verified.

### Final
- [ ] `cargo build --workspace` succeeds.
- [ ] `cargo test --workspace` passes all tests.
- [ ] `cargo bench --workspace` runs all benchmarks.
- [ ] `cargo clippy --workspace -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo doc --workspace --no-deps` clean.
- [ ] Release candidate tag ready to cut.

---

## Appendix A: Target Final Workspace Structure

```
paladin/
├── Cargo.toml                         # Workspace root
├── crates/
│   ├── paladin-core/                  # Pure domain types (~29k LOC)
│   ├── paladin-ports/                 # Port trait definitions (~6k LOC)
│   ├── paladin-battalion/             # Orchestration runtime + Maneuver DSL (~22k LOC)
│   ├── paladin-llm/                   # LLM provider adapters (~4k LOC)
│   │   └── features: openai, anthropic, deepseek, mock
│   ├── paladin-memory/                # Garrison + Sanctum adapters (~5k LOC)
│   │   └── features: sqlite, qdrant
│   ├── paladin-web/                   # Web server + API (~4k LOC)
│   │   └── features: (default all)
│   ├── paladin-notifications/         # Notification adapters (~2k LOC)
│   │   └── features: email, push, system
│   ├── paladin-content/               # Content processing pipeline (~6k LOC)
│   │   └── features: pdf, web-scraping, rss
│   ├── paladin-storage/               # SQL repositories (~3k LOC)
│   │   └── features: sqlite, mysql
│   └── paladin-cli/                   # CLI binary (~12k LOC)
│
├── src/                               # Facade crate (re-exports + ServiceRunner)
│   └── lib.rs
├── tests/                             # Workspace integration tests
├── benches/                           # Workspace-level benchmark coordination
├── examples/                          # Workspace-level examples
├── migrations/                        # Database migrations
└── docker/                            # Docker build files
```

## Appendix B: Crate Publishing Order

Publishing must respect the dependency graph. Crates are published bottom-up:

```
Step 1: paladin-core              (no workspace dependencies)
Step 2: paladin-ports             (depends on: paladin-core)
Step 3: paladin-battalion          (depends on: core, ports)
        paladin-llm               (depends on: core, ports)       ← parallel
        paladin-memory            (depends on: core, ports)       ← parallel
        paladin-web               (depends on: core, ports)       ← parallel
        paladin-notifications     (depends on: core, ports)       ← parallel
        paladin-content           (depends on: core, ports)       ← parallel
        paladin-storage           (depends on: core, ports)       ← parallel
Step 4: paladin                   (facade: depends on all above)
Step 5: paladin-cli               (depends on: paladin facade)
```

## Appendix C: Versioning Strategy

### Phase 1: Lockstep Versioning (Initial)

All crates share the same version number, managed via `[workspace.package]`.

```toml
[workspace.package]
version = "0.2.0"
```

Individual crates inherit: `version.workspace = true`.

**Rationale:** Simplifies initial release; all crates are tested together; avoids premature version fragmentation.

### Phase 2: Independent Versioning (Future)

Transition criteria (all must be met):
- At least two stable releases under lockstep versioning.
- Clear evidence that crates evolve at different rates.
- Downstream consumers request pinning individual crate versions.
- Team has tooling (e.g., `cargo-release`, `release-plz`) for multi-crate publishing.

When transitioning: `paladin-core` and `paladin-ports` version independently but maintain backward-compatible guarantees. Leaf crates can version freely within their port trait contract.

## Appendix D: Downstream Consumer Dependency Profiles

| Consumer Profile | Crates Needed | Dependencies Excluded |
|-----------------|---------------|-----------------------|
| **Orchestration Only** | `paladin-core`, `paladin-ports`, `paladin-battalion`, one `paladin-llm` provider | web server, notifications, content processing, storage, all other LLM providers |
| **Full Framework** | `paladin` (facade with default features) | Nothing excluded |
| **Custom LLM Provider** | `paladin-core`, `paladin-ports` | All adapters — consumer implements `LlmPort` |
| **Memory System Only** | `paladin-core`, `paladin-ports`, `paladin-memory` | Orchestration, LLM, web, notifications, content |
| **Content Pipeline** | `paladin-core`, `paladin-ports`, `paladin-content`, `paladin-storage` | Orchestration, LLM, web, notifications |
| **Web API + Orchestration** | `paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`, `paladin-web` | Notifications, content processing, storage |
