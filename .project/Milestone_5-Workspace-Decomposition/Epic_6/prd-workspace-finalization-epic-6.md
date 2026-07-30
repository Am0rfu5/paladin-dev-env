# PRD: Workspace Finalization — Facade Crate, CI Pipeline, and Build Benchmarks

**Milestone:** 5 — Cargo Workspace Decomposition (Tier 2)
**Epic:** 6 — Facade Crate, CI Pipeline, and Workspace Finalization
**Status:** Planning
**Created:** 2026-05-20
**Author:** Paladin Engineering

---

## 1. Introduction / Overview

Epics 1–5 of Milestone 5 successfully decomposed the Paladin monolith into six purpose-built workspace crates:

| Crate | Contents |
|---|---|
| `paladin-core` | Pure domain types, zero external deps |
| `paladin-ports` | Port trait contracts |
| `paladin-battalion` | Orchestration runtime |
| `paladin-llm` | LLM provider adapters (feature-flagged) |
| `paladin-memory` | Garrison and sanctum storage adapters |
| `paladin` (facade) | Backward-compatible re-export entry point |

Epic 6 closes out the workspace decomposition by completing three remaining deliverables:

1. **Facade audit and `prelude` module** — ensure the root `paladin` crate covers every existing import path used in examples and tests, and provide a new `paladin::prelude` convenience module.
2. **CI pipeline workspace upgrade** — extend the existing GitHub Actions workflows to add per-crate isolated build and test jobs, ensuring no hidden cross-crate dependency leaks go undetected.
3. **Build-time benchmark report** — establish incremental and clean build baselines in the new workspace and document the improvement over the pre-decomposition monolith.

> **Out of scope for this PRD:** Migration guide / external documentation (no current external consumers), and the team retrospective / Tier 3 preparation document.

---

## 2. Goals

1. **G1 — Facade completeness:** Every `use paladin::…` import path that appears in `examples/`, `tests/`, and `src/` continues to compile without modification.
2. **G2 — Prelude module:** A `paladin::prelude` module provides the ~20 most commonly used types so that typical users need only `use paladin::prelude::*`.
3. **G3 — Per-crate CI isolation:** GitHub Actions can build and test each workspace crate in isolation, failing immediately if a crate accidentally depends on a type it should not have access to.
4. **G4 — Workspace-level CI:** A single workflow run validates the full workspace (build + test + lint + docs) so that nothing slips through the crate-level jobs.
5. **G5 — Build-time evidence:** A committed report captures clean and incremental build times for the workspace and confirms the ≥ 50% incremental improvement target described in the Milestone 5 overview.

---

## 3. User Stories

**US-1 — Library consumer (current team member):**
> As a developer using the `paladin` crate internally, I want `use paladin::PaladinBuilder` and all other existing paths to keep working unchanged after the workspace split, so that I do not have to update any import statements.

**US-2 — New developer onboarding:**
> As a developer new to the project, I want a single `use paladin::prelude::*` that brings all the common agent types into scope, so I can write a working agent in ten lines without hunting for individual module paths.

**US-3 — CI contributor:**
> As a developer opening a pull request, I want the CI pipeline to tell me within minutes if my change to `paladin-core` accidentally broke `paladin-battalion`, so I can fix the issue before merging.

**US-4 — Crate contributor:**
> As a developer working only on `paladin-llm`, I want to be able to run `cargo test -p paladin-llm` and `cargo build -p paladin-llm` in isolation and have CI do the same, so I can iterate quickly without rebuilding the entire workspace.

**US-5 — Tech lead / architect:**
> As the project architect, I want a documented build-time benchmark showing measurable improvement in incremental rebuild time compared to the pre-decomposition monolith, so that the workspace split can be justified to stakeholders.

---

## 4. Functional Requirements

### Task 6.1 — Facade Crate Re-Export Audit and Prelude Module

**FR-1.1** The system must compile every `use paladin::…` import path currently present in `examples/**/*.rs`, `tests/**/*.rs`, and `src/**/*.rs` without modification.

**FR-1.2** A script or manual audit must scan all files matching `examples/**/*.rs` and `tests/**/*.rs` for `use paladin::` statements and produce a checklist confirming each path is covered by a re-export in `src/lib.rs`.

**FR-1.3** Any import path found during the audit that is not yet covered by a re-export in `src/lib.rs` must be added.

**FR-1.4** The system must provide a `pub mod prelude` in `src/lib.rs` (or `src/prelude.rs` re-exported as `pub mod prelude`) containing at minimum the following commonly used types:
- `PaladinBuilder`, `Paladin`, `PaladinData`, `PaladinStatus`, `PaladinConfig`, `PaladinError`
- `Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`
- `BattalionConfig`, `BattalionError`, `BattalionResult`
- `CommanderBuilder`, `CouncilBuilder`, `GroveBuilder`
- `LlmPort`, `LlmRequest`, `LlmResponse`, `LlmError`
- `GarrisonPort`, `GarrisonError`
- `SanctumPort`, `SanctumError`
- `InMemoryGarrison`, `InMemorySanctum`
- `ArsenalPort`, `ArsenalRegistry`, `Armament`
- `PaladinResult`, `StopReason`

**FR-1.5** The prelude must be documented with a short `//!` module doc comment explaining what it contains and how to use it.

**FR-1.6** `cargo build --workspace` must continue to succeed after all facade changes with zero errors or warnings.

**FR-1.7** `cargo doc -p paladin --no-deps 2>&1 | grep -i "warn\|error"` must produce no output after all changes.

---

### Task 6.2 — GitHub Actions CI Workspace Upgrade

**FR-2.0** Before any CI workflow changes can be tested or triggered from inside the devcontainer, the GitHub CLI (`gh`) must be installed in the development environment. The `gh` CLI is currently absent from the container (confirmed: `which gh` returns nothing). The devcontainer build must be updated as follows:

- Add the official GitHub CLI apt repository and install `gh` in `.devcontainer/Dockerfile.dev`, using the upstream Debian package installation method:

  ```dockerfile
  # Install GitHub CLI
  RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
        -o /usr/share/keyrings/githubcli-archive-keyring.gpg \
   && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
   && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
        | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
   && apt-get update \
   && apt-get install -y --no-install-recommends gh \
   && apt-get clean -y \
   && rm -rf /var/lib/apt/lists/*
  ```

- This block must be added to `Dockerfile.dev` **after** the existing `apt-get` package installation block and **before** the `rustup` component installation, so that it benefits from the same Docker layer cache as other system packages.
- After the devcontainer is rebuilt, `gh --version` must succeed inside the container.
- Developers must run `gh auth login` once after the rebuild to authenticate with GitHub before using `gh workflow run`, `gh run watch`, or `gh run list`.

**FR-2.1** The updated CI must include a **per-crate isolation job** that builds and tests each of the following crates independently:
- `paladin-core` — `cargo build -p paladin-core` and `cargo test -p paladin-core`
- `paladin-ports` — `cargo build -p paladin-ports` and `cargo test -p paladin-ports`
- `paladin-battalion` — `cargo build -p paladin-battalion` and `cargo test -p paladin-battalion`
- `paladin-llm` — `cargo build -p paladin-llm --all-features` and `cargo test -p paladin-llm --all-features`
- `paladin-memory` — `cargo build -p paladin-memory --all-features` and `cargo test -p paladin-memory --all-features`
- `paladin` (facade) — `cargo build -p paladin` and `cargo test -p paladin`

**FR-2.2** The per-crate isolation job must run with `cargo build -p <crate> --no-default-features` in addition to the full-features variant, to verify that each crate compiles cleanly without optional dependencies.

**FR-2.3** The existing `ci.yml` workspace-level jobs must be updated to run against `--workspace` explicitly (e.g., `cargo test --workspace`, `cargo build --workspace`) rather than implicitly targeting only the root crate.

**FR-2.4** The existing `feature-flags.yml` feature matrix must be updated to also run `cargo build --workspace <flags>` in addition to the current default (root crate only), ensuring feature flags propagate correctly across all workspace members.

**FR-2.5** The lint job in `ci.yml` must run `cargo clippy --workspace -- -D warnings` (currently `--all-targets --all-features`). The `--workspace` flag must be added if not already present.

**FR-2.6** The documentation check step must run `cargo doc --workspace --no-deps` and must fail the job if any warnings are produced (`2>&1 | grep -c "warning:"` must return 0).

**FR-2.7** All existing CI jobs (`lint`, `api-surface`, `test`, `integration-tests`) must remain green after the workflow updates. No existing job may be removed or disabled.

**FR-2.8** Per-crate isolation jobs must be added as a new job named `crate-isolation` in `ci.yml`, running in parallel with the existing `test` job (not blocking it).

**FR-2.9** The CI `test` job matrix (`rust-version: [stable, beta]`) must run `cargo test --workspace` rather than the implicit root-crate-only command.

---

### Task 6.3 — Build-Time Benchmark Report

**FR-3.1** The benchmark must record the following timed scenarios using `time cargo build …` (or equivalent):

| Scenario | Command |
|---|---|
| Workspace clean build | `cargo clean && cargo build --workspace` |
| Workspace incremental (core change) | Touch a file in `paladin-core/src/`, then `cargo build --workspace` |
| Workspace incremental (llm adapter change) | Touch a file in `paladin-llm/src/`, then `cargo build --workspace` |
| Workspace incremental (memory adapter change) | Touch a file in `paladin-memory/src/`, then `cargo build --workspace` |
| Core + ports + battalion only | `cargo build -p paladin-battalion` (no memory/llm in dep tree) |

**FR-3.2** The benchmark must also record the equivalent pre-decomposition times by checking out the last commit on `main` before the workspace split (or the last tagged monolith commit) and running `time cargo build` against it.

**FR-3.3** The benchmark results must be committed to `project/Milestone_5-Workspace-Decomposition/Epic_6/build-benchmarks.md` containing: the hardware/OS environment, raw timing numbers, and a summary table with percentage improvement for each scenario.

**FR-3.4** If any incremental scenario shows a regression (workspace takes _longer_ to rebuild than the monolith for an equivalent change), this must be called out explicitly in the report with a root-cause analysis.

**FR-3.5** The report must confirm whether the ≥ 50% incremental rebuild improvement target (defined in the Milestone 5 overview) was achieved. If not achieved, the report must recommend follow-up actions.

---

## 5. Non-Goals (Out of Scope)

- **Migration guide / documentation update** — There are no current external consumers of the `paladin` crate. Documentation for downstream consumers is deferred to a future milestone.
- **Team retrospective document** — Not required for this PRD; deferred to a team decision outside the scope of automated implementation.
- **Extracting a `paladin-cli` crate** — The CLI remains isolated behind the `cli` feature flag in the root crate. A separate crate extraction is a Tier 3 concern.
- **Splitting configuration** (`application_settings.rs`) into per-domain configs — Tier 3 scope.
- **Content processing, notification, or web server crates** — Out of scope for this workspace decomposition milestone.
- **Any changes to existing stable port trait APIs** — All port traits are frozen per the `STABLE_API.md` contract.

---

## 6. Design Considerations

### Prelude Module Layout

The recommended structure is a dedicated `src/prelude.rs` re-exported as `pub mod prelude` from `src/lib.rs`:

```rust
// src/prelude.rs
//! Convenient re-exports of the most commonly used Paladin types.
//!
//! # Usage
//! ```rust,no_run
//! use paladin::prelude::*;
//! ```

pub use crate::{
    // Agents
    Paladin, PaladinBuilder, PaladinConfig, PaladinData, PaladinError, PaladinStatus,
    // Battalions
    BattalionConfig, BattalionError, BattalionResult,
    Campaign, ChainOfCommand, CommanderBuilder, Formation, Phalanx,
    // LLM
    LlmError, LlmPort, LlmRequest, LlmResponse,
    // Memory
    GarrisonError, GarrisonPort, InMemoryGarrison, InMemorySanctum, SanctumError, SanctumPort,
    // Tools
    Armament, ArsenalPort, ArsenalRegistry,
    // Execution
    PaladinResult, StopReason,
};
```

### CI Job Structure

The new `crate-isolation` job should use a matrix strategy identical in style to the existing `test` job to keep the YAML clean:

```yaml
crate-isolation:
  name: Crate Isolation (${{ matrix.crate }})
  strategy:
    matrix:
      include:
        - crate: paladin-core
          extra_flags: ""
        - crate: paladin-ports
          extra_flags: ""
        - crate: paladin-battalion
          extra_flags: ""
        - crate: paladin-llm
          extra_flags: "--all-features"
        - crate: paladin-memory
          extra_flags: "--all-features"
        - crate: paladin
          extra_flags: ""
```

### Benchmark Tooling

Use the `time` shell built-in for simplicity. Record wall-clock time (`real`) for each scenario. Three runs per scenario, report the median. A helper shell script (`scripts/benchmark-builds.sh`) should automate the measurement loop so results can be reproduced.

---

## 7. Technical Considerations

- **`paladin-core` and `paladin-ports` have no workspace dependencies** — their isolation builds have no network calls or external services, making them very fast in CI (~10–30 s per run). This makes them ideal early-fail candidates.
- **`paladin-llm` and `paladin-memory` have optional heavy deps** (`qdrant-client`, `tiktoken-rs`) — the `--all-features` isolation build will be the slowest isolation job (~60–90 s based on Epic 5 observations). It should be run with appropriate caching.
- **`cargo-nextest`** may be used in place of `cargo test` in CI for parallel test execution and improved output. This is optional but recommended for the `crate-isolation` job.
- **Pre-decomposition baseline** — The last commit on `main` branch before the workspace split is the correct comparison point for Task 6.3. Use `git stash` or a temporary checkout rather than `cargo clean` on the current workspace to avoid losing build artifacts.
- **GitHub CLI (`gh`) is not installed in the devcontainer** — `gh` is required to trigger and monitor GitHub Actions workflows from within the container (e.g., `gh workflow run ci.yml`, `gh run watch`, `gh run list --workflow=ci.yml`). It must be installed by updating `.devcontainer/Dockerfile.dev` as specified in FR-2.0. The container must be rebuilt once after that change. This is a prerequisite for Task 6.2 and must be completed first.
- **Existing CI workflow versions** — The current `ci.yml` uses `actions-rs/toolchain@v1` which is deprecated. Consider upgrading to `dtolnay/rust-toolchain@stable` in the same PR as the workflow update. This is a low-risk improvement that should not be deferred.
- **`paladin-core` and `paladin-ports` are not yet in `[workspace.dependencies]`** — the `Cargo.toml` only lists `paladin-battalion`, `paladin-llm`, and `paladin-memory` there. Add `paladin-core` and `paladin-ports` as workspace deps during Task 6.1 cleanup.

---

## 8. Success Metrics

| Metric | Target | How Measured |
|---|---|---|
| SM-0: Devcontainer `gh` CLI | `gh --version` succeeds inside container | Manual verification after rebuild |
| SM-1: Facade path coverage | 100% of existing `use paladin::...` paths compile | `cargo build --workspace` with zero errors after audit |
| SM-2: Prelude completeness | All 20+ types from FR-1.4 in `paladin::prelude` | `cargo doc -p paladin --no-deps` with zero broken links |
| SM-3: CI per-crate isolation | All 6 crate isolation jobs green | GitHub Actions job summary |
| SM-4: Workspace CI coverage | `cargo test --workspace` passes in CI | CI `test` job green |
| SM-5: Lint / fmt clean | Zero warnings from clippy and fmt | CI `lint` job green |
| SM-6: Doc build clean | Zero warnings from `cargo doc --workspace --no-deps` | CI `lint` job doc step |
| SM-7: Incremental build improvement | ≥ 50% faster incremental rebuild for isolated crate changes | `build-benchmarks.md` report |
| SM-8: No build regressions | No scenario is slower than the pre-decomposition monolith | `build-benchmarks.md` report |

---

## 9. Open Questions

**OQ-1:** Should `paladin-core` and `paladin-ports` be added to `[workspace.dependencies]` in this Epic, or is that a Tier 3 housekeeping item? (Recommendation: do it in Task 6.1 since it's a low-risk one-line change per crate.)

**OQ-2:** The pre-decomposition monolith baseline for Task 6.3 requires identifying the correct git commit (the last commit on `main` before Epic 1 landed). Which branch or tag should be used as the benchmark comparison point? (Recommendation: use the last commit on `main` before `feature/milestone_5-epic_1-paladin-core-extraction` was merged.)

**OQ-3:** Should the `crate-isolation` CI job block merging to `main` (required status check), or run as an informational check? (Recommendation: required, since it directly validates the architectural invariant of the workspace split.)

**OQ-4:** Is `cargo-nextest` already installed in the CI runner images, or does it need to be added as an installation step? (Recommendation: add an optional installation step with `cargo install cargo-nextest --locked`; fall back to `cargo test` if nextest is not desired.)
