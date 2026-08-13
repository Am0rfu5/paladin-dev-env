# Phase 15: Coverage & CI Quality Gates - Research

**Researched:** 2026-08-13
**Domain:** GitHub Actions CI/CD for a Rust cargo workspace — coverage gating (`cargo-llvm-cov`),
snapshot-test CI wiring (`insta`), benchmark compile-checks, deprecated-action modernization,
hand-written async test mocks.
**Confidence:** HIGH — every code-site claim below was re-verified live against the tree on
2026-08-13 (one day after CONTEXT.md's 2026-08-12 pass); external tool/action versions verified via
WebSearch and `cargo search`/registry checks this session.

## Summary

This phase has almost no "which library" ambiguity — `15-CONTEXT.md` already resolved every real
design fork (D-01 … D-14) through an interactive discussion session, and that document is the
primary input a planner should read in full. This RESEARCH.md exists to (a) independently
re-verify the load-bearing facts CONTEXT.md's `<specifics>` section asserts, since planning will be
built directly on top of them, and (b) fill in the external-ecosystem knowledge CONTEXT.md doesn't
carry: current `cargo-llvm-cov` mechanics, current GitHub Action versions for the eight deprecated
references, and the one CI gotcha (`llvm-tools-preview`) that isn't mentioned anywhere in the
project's own documents but will silently break the new `coverage` job if omitted.

Every numeric/structural claim in CONTEXT.md's `<specifics>` was re-checked directly against the
tree this session and confirmed unchanged: `ci.yml` still has 15 jobs at the cited line numbers;
`actions-rs/toolchain@v1` sits at exactly `ci.yml:163,408,788` and `integration-tests.yml:71`;
`actions/cache@v3` sits at `integration-tests.yml:78,84,90`; `codecov/codecov-action@v3` sits at
`integration-tests.yml:123` — eight references total, matching D-00/finding-1 exactly. 86 snapshot
files and 97 `#[test]` functions in `tests/cli/` (not REQUIREMENTS.md's stale "43"). `Cargo.toml`
confirms all three binaries are feature-gated (`cli`, `cli`, `web-server`) and `integration-tests`
is a plain marker feature (`integration-tests = []`). `user_service.rs` is 583 lines with five
existing `#[tokio::test]`s over real in-memory adapters; `listener.rs` is 538 lines with three
existing tests. `register_channel_handler` on `NotificationService` is `pub async fn`, confirming
D-10's `FailingChannelHandler` seam. `.codecov.yml` does not exist. The `Makefile` has zero
`llvm-cov` references and zero Coverage section. All three instruction files (`CLAUDE.md:59-60`,
`.github/copilot-instructions.md:155-156`, `.planning/codebase/TESTING.md:311-322`) still assert
the rejected 80%/70% figure and `TESTING.md` still names `cargo tarpaulin`. `docs/src/contributing/`
already contains `testing-guide.md`, confirming D-13's landing target exists and is ready to be
extended rather than created.

The one gap this research adds beyond CONTEXT.md: **`llvm-tools-preview` is not installed anywhere
in this workspace's CI today** (`grep -rn llvm-tools .github/workflows/` returns nothing), and
`cargo-llvm-cov` requires it (or a nightly toolchain) to instrument coverage at all. Every existing
`dtolnay/rust-toolchain@stable` step in `ci.yml` must gain `components: llvm-tools-preview` in the
new `coverage` job's toolchain-install step, or the job fails immediately with a missing-component
error — this is the single most common `cargo-llvm-cov` CI failure mode reported upstream and isn't
mentioned in any ingested requirement or in CONTEXT.md.

**Primary recommendation:** Follow `15-CONTEXT.md` verbatim for every design decision (D-01 through
D-14); use this document for the exact command syntax, current action versions, and the
`llvm-tools-preview` requirement when writing the actual workflow YAML and Makefile targets.

## User Constraints (from CONTEXT.md)

> Interactive-mode session, 2026-08-12. All fourteen numbered decisions below were selected by a
> human from a presented option set; none were `--auto`-selected. Copied verbatim from
> `15-CONTEXT.md`.

### Locked Decisions

- **D-01:** The CI coverage job measures `--workspace --features integration-tests` with Redis and
  MinIO running. Floor is re-derived from the figure measured under this scope using ADR-0006's
  truncate-toward-zero rule. `--all-features` and default-feature-only were both rejected.
- **D-02:** `cargo llvm-cov --fail-under-lines <floor>` in the workflow is the gate. Codecov
  reports, it does not gate. `.codecov.yml` still lands (PR comments, diff view, dashboard) but
  carries no blocking status.
- **D-03:** A dedicated `coverage` job in `ci.yml`, and `integration-tests.yml`'s coverage step
  (and its `codecov-action@v3` reference) is deleted.
- **D-04:** Two-step landing — measure first (commit 1, no `--fail-under-lines`), transcribe the
  CI-produced figure byte-identical into the ADR-0006 amendment, then gate second (commit 2).
  Forced by environment: Docker is absent locally, so the `--features integration-tests` figure
  cannot be produced any other way.
- **D-05:** The two module-scoped gates (Herald ≥95%, autonomous ≥90%) are re-measured and
  recorded in the ADR-0006 amendment with gaps stated, **not enforced in CI**. The 84%→(re-derived)
  workspace floor stays the only binding gate.
- **D-06:** The three binaries (`paladin`, `paladin-cli`, `paladin-server`) are outside the gated
  denominator by construction (feature-gated, don't compile under D-01's scope). `.codecov.yml`
  gets `src/bin/**` in `ignore`. The `run()` seam (ADR-0006 D-14a) is already extracted; the 0.00%
  figure is stale (a `#[cfg(test)]` module now exists at `paladin-server.rs:256`) — both corrected
  by observation, no code change needed.
- **D-07:** A separate `cli-tests` job (`cargo test -p paladin-ai --features cli --test cli`), no
  `needs:`, parallel with `lint` and `test`.
- **D-08:** Shared test infrastructure lives in `src/test_support/` (name at planner's discretion),
  behind `#[cfg(test)]` — **not** `tests/common/`, because both coverage targets test from inside
  `src/` via co-located `#[cfg(test)] mod tests`, and `tests/` is a separate crate that cannot be
  imported from `src/`.
- **D-09:** Hand-written mocks. `mockall` is **not** adopted (answers DEFER-01 Open Question 2
  explicitly). No `mockall` reference exists in any of the twelve workspace manifests.
- **D-10:** The mock set is demand-driven — build only what DEFER-02/DEFER-03 actually consume,
  verified against real signatures, with a recorded verdict per DEFER-01 name (built / replaced by
  X / unnecessary because Y). `MockNotificationService` becomes a `FailingChannelHandler` (the
  concrete `Arc<NotificationService>` field can't be swapped, but `register_channel_handler` is
  public). `MockUserRepository` is likely unnecessary — `SqliteUserRepository::new("sqlite::memory:")`
  already fills that role.
- **D-11:** One phase — PIPE-01…05 first, then DEFER-01…03, wave-decomposed. The register's
  35-45h estimate is a stale upper bound (Herald climb removed, mock set shrunk, both target
  modules gained tests since February).
- **D-12:** ≥80% per module (DEFER-02's own figure) is a **phase acceptance criterion**, not a
  standing CI gate. DEFER-03 inherits the same bar. Verified via module-targeted `cargo llvm-cov`.
- **D-13:** PIPE-05's coverage documentation lands in `docs/src/contributing/testing-guide.md`
  (already exists — `CONTRIBUTING.md` does not exist anywhere in the tree; contributor docs
  relocated into the mdbook by Milestone 11).
- **D-14:** The rejected 80%/70% figure is corrected in all three instruction files: `CLAUDE.md`,
  `.github/copilot-instructions.md`, `.planning/codebase/TESTING.md`. Scope guard: coverage-number
  claims only, not general content currency (that's DOCS-01, Phase 16).

### Claude's Discretion

- PIPE-04's exact action-version mapping and whether `actionlint` scope is the three workflows the
  requirement text names or all six that actually exist (six exist: `ci`, `docs`, `feature-flags`,
  `integration-tests`, `pre-commit`, `release` — lint all six).
- `bench-check` shape/caching beyond the unambiguous `cargo bench --no-run`.
- `.codecov.yml` contents beyond D-02 (no blocking status) and D-06 (`src/bin/**` ignored).
- The `make coverage` / `make services-up` relationship (declare dependency vs. fail loudly).
- Naming of the `src/` test-support module; whether `tests/helpers/` is eventually consolidated
  into it (out of scope here if it grows past a re-export).
- Tokio time-control utilities (`pause()`/`advance()`) — std tokio, no wrapper needed unless the
  listener tests want a shared helper.
- Wave decomposition and plan boundaries, subject to D-11's ordering and D-04's two-commit sequence.
- ADR allocation: whether the CI-gate topology gets its own record or lives entirely as an
  ADR-0006 amendment (the amendment is mandatory either way).
- Whether the advisory Docker build-time budget at `ci.yml:539` ("Owner: Phase 15 / PIPE",
  proposing native `ubuntu-24.04-arm` runners over QEMU) is taken up here.
- Whether DEFERRED_COVERAGE.md's two remaining prerequisites ("document testing best practices",
  "establish concurrency testing patterns") get explicit closure records.

### Deferred Ideas (OUT OF SCOPE)

- Closing Herald's ~14.5-point gap to its ≥95% module target — recorded/re-measured (D-05), not
  closed here; handed forward as named work.
- Enforcing per-module coverage gates in CI via a report-parsing check script with no-regression
  ratchets — considered and declined under D-05 (reintroduces a multi-number failure).
- Widening the coverage denominator to `web-server` and `cli` so the shipped binaries are gated —
  declined under D-06; D-07's `cli-tests` job covers the CLI surface more cheaply.
- Adopting `mockall` — declined by D-09; trigger for revisiting is the hand-written mock set
  growing past the point where expectation assertions dominate test code.
- Consolidating `tests/helpers/` into the new `src/` test-support home — out of scope; the two
  coexist (D-08 serves `src/`-side co-located tests, `tests/helpers/` serves integration suites).
- The native-arm64 CI rework replacing QEMU multi-arch emulation (`ci.yml:525-552`) — named owner
  "Phase 15 / PIPE" but no requirement ID; left to planner discretion, otherwise a future phase.
- Benchmark regression *detection* (`critcmp`, `github-action-benchmark`) — Epic 25's explicit
  non-goal; `benchmark-regression-signal` already ships at `ci.yml:812` from Milestone 7.
- A second, feature-scoped coverage measurement (the `minio.rs` open question) — D-01 answers it by
  choosing one scope; revisit only if a feature-gated subsystem needs its own recorded number.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PIPE-01 | CLI snapshot tests (86 snapshots / 97 `#[test]` fns, not the stale "43") and a benchmark compile-check run in CI | D-07 (`cli-tests` job spec below); `cargo bench --no-run` verified as the standard compile-only benchmark pattern; `Cargo.toml:210-213` confirms `required-features = ["cli"]` is why the CLI suite never ran in CI |
| PIPE-02 | Combined unit+integration coverage measured in CI, gated at one threshold | `cargo-llvm-cov` command syntax, `--fail-under-lines`, `llvm-tools-preview` requirement, Redis/MinIO service-block pattern (all below) |
| PIPE-03 | Coverage and the two new test targets runnable locally via `make` | Makefile confirmed to have zero `llvm-cov` references; exact target syntax provided below |
| PIPE-04 | No deprecated GitHub Action remains; `actionlint` clean | All eight deprecated references re-verified at exact line numbers; current replacement versions verified via WebSearch below |
| PIPE-05 | Coverage documentation a contributor can reproduce the CI number from | `docs/src/contributing/testing-guide.md` confirmed to exist already (D-13) |
| DEFER-01 | Shared `Send + Sync` mock/test infrastructure | Existing `tests/helpers/mock_llm_adapter.rs` pattern read directly (Arc<Mutex<..>> recording, factory fns) — the shape D-09/D-10 says to copy |
| DEFER-02 | `user_service.rs` reaches ≥80% module coverage | File re-read this session: 583 lines, 5 existing tests, `register_user:228` already handles notification failure non-blockingly (`if let Err(e) = ...`, not `?`), `register_channel_handler` confirmed public |
| DEFER-03 | Listener orchestrator coverage re-measured and closed to ≥80% | File re-read this session: `listener.rs`, 538 lines, 3 existing tests at lines 471/488/514 |

## Architectural Responsibility Map

This phase operates almost entirely at a tier outside the hexagonal application architecture — it
is CI/CD infrastructure and test-only code. Mapped for completeness against the standard tiers:

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| CLI snapshot regression gate | CI / CD (GitHub Actions) | — | `cli-tests` job; no application-layer change |
| Benchmark compile-check | CI / CD | — | `cargo bench --no-run`; catches API breakage only |
| Coverage measurement + gate | CI / CD | Database/Storage (Redis, MinIO as CI services) | Coverage job needs the same service containers `integration-tests` already starts |
| Local coverage reproduction | Build tooling (Makefile) | — | Must mirror the CI command exactly (PIPE-03) |
| GitHub Action version currency | CI / CD | — | Workflow YAML only, no application code |
| Shared async test mocks | Application / Backend (test-only) | Core (co-located `#[cfg(test)]`) | `src/test_support/` sits inside the application crate's `src/` tree so `#[cfg(test)]` modules in `user_service.rs`/`listener.rs` can import it — `tests/` (a separate crate) cannot serve this |
| `user_service.rs` coverage | Application / Backend | Core (auth/password hashing) | Business logic (registration, argon2 hashing, notification dispatch) — ASVS-relevant surface (see Security Domain) |
| Listener orchestrator coverage | Application / Backend | — | Event dispatch, concurrency, trigger matching — no external API surface |
| Documentation correction (coverage numbers) | Docs (mdbook + root instruction files) | — | `CLAUDE.md`, `.github/copilot-instructions.md`, `docs/src/contributing/testing-guide.md` |

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `cargo-llvm-cov` | **0.8.7** [VERIFIED: crates.io — `cargo search cargo-llvm-cov` this session; 113,206 weekly downloads, published since 2021-01, repo `github.com/taiki-e/cargo-llvm-cov`, `package-legitimacy check` verdict `OK`] | Cargo subcommand wrapping LLVM source-based coverage instrumentation | Already the tool-of-record per ADR-0006 and already invoked in `integration-tests.yml:117`; no reason to change tools, only to move where it runs |
| `taiki-e/install-action@v2` | tag `v2` [CITED: taiki-e/install-action TOOLS.md, verified via WebSearch] | Installs `cargo-llvm-cov` (and other Rust CLI tools) as a pre-built binary in CI, ~30s vs. 3-5min for `cargo install` | Standard fast-install pattern for Rust CI tooling; REQUIREMENTS.md's PIPE-02 text already names it with a pinned tool version (`cargo-llvm-cov@0.7.1`) — **that pin is stale**; either omit the version pin (installs latest, currently 0.8.7) or update it to match the currently-published version. Pin removal/update is a planner discretion item not covered by CONTEXT.md's locked decisions. |
| `dtolnay/rust-toolchain@stable` | branch ref (not a version tag) [VERIFIED: repo, already in use at `ci.yml:29,224,254,351,822,908` etc.] | Installs the Rust toolchain in CI with component support | Already the project's standard toolchain-install action everywhere except the three `actions-rs/toolchain@v1` sites this phase must replace |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `insta` | 1.34 [CITED: `.planning/codebase/TESTING.md` dev-dependency inventory, per 15-CONTEXT.md canonical_refs — not independently re-verified against `Cargo.toml` this session] | Snapshot-testing framework backing the 86 `.snap` files under `tests/cli/snapshots/` | Already in use; PIPE-01 does not add a new snapshot tool, only makes the existing suite run in CI |
| `actionlint` | **v1.7.12** [VERIFIED: WebSearch, `rhysd/actionlint` GitHub Releases] | Static checker for GitHub Actions workflow YAML/expressions | PIPE-04's explicit acceptance bar ("actionlint reports zero errors") |
| `reviewdog/action-actionlint` | **v1.72.0** (wraps actionlint 1.7.12) [VERIFIED: WebSearch, `reviewdog/action-actionlint` Releases] | Optional CI-integrated actionlint runner with PR annotations | Only needed if the planner chooses to run `actionlint` as a CI job rather than (or in addition to) a local `make`/pre-commit check — PIPE-04 does not mandate a specific runner, only zero errors |
| `codecov/codecov-action` | **v5** [VERIFIED: WebSearch — v5 is Codecov's current recommendation; v4 and v5 both dropped tokenless upload for GH Actions except fork-PR-to-public-repo, replaced by an opt-in org-level Global Upload Token] | Uploads `lcov.info` to Codecov for PR-comment/diff/dashboard reporting (non-blocking, per D-02) | REQUIREMENTS.md's PIPE-04 text says upgrade the deleted `integration-tests.yml:123` reference to `codecov-action@v4` — **that is superseded by D-03**, which deletes the reference outright rather than upgrading it. If the new `ci.yml` `coverage` job still wants Codecov reporting (D-02 permits it, non-blocking), land it there as v5, not v4, since v4 predates the current recommendation. |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Hand-written mocks (D-09, locked) | `mockall` | Derive-based, less boilerplate, but a new proc-macro dev-dependency governed by ADR-0024/ADR-0036's suppression register, and a second mocking idiom alongside every existing hand-written mock in the workspace. Declined. |
| `cargo llvm-cov --fail-under-lines` in-workflow (D-02, locked) | Codecov project/patch status checks as the gate | Free, matches PIPE-02's literal text, but depends on an external service and `CODECOV_TOKEN`; a fork PR whose upload silently fails would pass green. Declined. |
| `cargo-llvm-cov` (already tool-of-record) | `cargo-tarpaulin` | `TESTING.md:319-322` still documents tarpaulin locally; ADR-0006's tool-of-record note already flags this as stale and D-14 corrects it. Tarpaulin uses ptrace-based instrumentation on Linux and produces different (usually lower) coverage numbers than LLVM source-based instrumentation — the two are not comparable, which is exactly why ADR-0006 designates one tool of record. |
| `dtolnay/rust-toolchain` for the new `coverage` job | `actions-rs/toolchain@v1` | `actions-rs/toolchain` is unmaintained/deprecated (last published 2021) and is the exact class of reference PIPE-04 exists to remove — do not introduce a ninth instance while removing the other eight. |

**Installation (CI-side, in the new `coverage` job):**
```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    components: llvm-tools-preview
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-llvm-cov
```

**Installation (local, via the new `make coverage` target — PIPE-03):**
```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
```

**Version verification performed this session:**
- `cargo search cargo-llvm-cov` → `cargo-llvm-cov = "0.8.7"` (crates.io reachable in this
  environment; this is the same finding CONTEXT.md's specifics-finding-11 already recorded).
- `cargo llvm-cov --version` → `error: no such command` (not installed locally; expected — D-04's
  whole reason for existing is that the wider-scope figure can only come from CI).
- `docker info` → `docker: command not found` (confirms CONTEXT.md's environment note: Docker is
  still absent locally, so the Redis-/MinIO-backed `--features integration-tests` measurement
  cannot be produced in this environment either).
- `actionlint --version` → not installed locally; current release verified via WebSearch instead
  (v1.7.12).

## Package Legitimacy Audit

Only one new tool crosses into "install a package" territory for this phase: `cargo-llvm-cov`
(everything else — `insta`, `serial_test`, `proptest`, etc. — is already a workspace
dev-dependency and out of this phase's install surface). GitHub Actions (`taiki-e/install-action`,
`dtolnay/rust-toolchain`, `codecov/codecov-action`) are not cargo/npm packages and are evaluated by
maintainer reputation and WebSearch-verified release cadence instead of the crates-ecosystem
legitimacy check.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|--------------|---------|-------------|
| `cargo-llvm-cov` | crates (`cargo search`) | published since 2021-01-22 (~5.5 yrs) | 113,206/week | `github.com/taiki-e/cargo-llvm-cov` | **OK** | Approved — `gsd-tools package-legitimacy check --ecosystem crates cargo-llvm-cov` returned `OK` with no flags |

**Packages removed due to [SLOP] verdict:** none.
**Packages flagged as suspicious [SUS]:** none.

GitHub Actions used by this phase (not subject to the crates legitimacy gate, evaluated by
maintainer/release verification instead):

| Action | Replaces | Current version (verified) | Maintainer signal |
|--------|----------|----------------------------|--------------------|
| `dtolnay/rust-toolchain@stable` | `actions-rs/toolchain@v1` | branch ref, actively maintained [VERIFIED: already used 15+ times elsewhere in this repo's own `ci.yml`] | David Tolnay, prolific Rust-ecosystem maintainer; the project's own existing standard |
| `actions/cache@v4` | `actions/cache@v3` | v4 [VERIFIED: already in use everywhere else in `ci.yml`] | GitHub-official action |
| `codecov/codecov-action@v5` | `codecov/codecov-action@v3` | v5 current [VERIFIED: WebSearch, Codecov's own current recommendation] | Codecov-official action |
| `taiki-e/install-action@v2` | (new addition, not a replacement) | v2 [CITED: taiki-e/install-action TOOLS.md] | Same maintainer as `cargo-llvm-cov` itself |

*Packages/actions discovered via WebSearch and not independently re-verified against an
authoritative release feed in this session (`insta` 1.34, `serial_test` 3.2, `proptest` 1.4,
`testcontainers` 0.24) are tagged `[CITED: TESTING.md]` above and were not part of this phase's
install surface, so no `checkpoint:human-verify` gate is required for them.*

## Architecture Patterns

### System Architecture Diagram

```
                    ┌─────────────────────────────────────────────┐
                    │              PR / push to main               │
                    └───────────────────────┬───────────────────────┘
                                             │
                 ┌───────────────┬───────────┼───────────┬────────────────┐
                 ▼               ▼           ▼           ▼                ▼
            ┌─────────┐   ┌───────────┐ ┌─────────┐ ┌──────────┐   ┌───────────┐
            │  lint   │   │cli-tests  │ │  test   │ │bench-    │   │  ...other │
            │ (fmt +  │   │(NEW: cargo│ │(--lib   │ │check     │   │ existing  │
            │ clippy) │   │test --test│ │ --bins) │ │(NEW: cargo│  │  15 jobs  │
            │         │   │ cli       │ │         │ │bench     │   │           │
            │         │   │ --features│ │         │ │--no-run) │   │           │
            │         │   │  cli)     │ │         │ │          │   │           │
            └─────────┘   └───────────┘ └─────────┘ └──────────┘   └───────────┘
                                             │
                                             ▼
                              ┌──────────────────────────────┐
                              │   coverage (NEW job)          │
                              │   services: redis, minio      │
                              │   (copy ci.yml:374-400 block)  │
                              │                                │
                              │  1. dtolnay/rust-toolchain     │
                              │     components: llvm-tools-    │
                              │     preview                    │
                              │  2. taiki-e/install-action     │
                              │     tool: cargo-llvm-cov       │
                              │  3. cargo llvm-cov              │
                              │     --workspace                │
                              │     --features integration-    │
                              │       tests                    │
                              │     --fail-under-lines <floor> │
                              │     --lcov --output-path       │
                              │       lcov.info                │
                              │  4. codecov-action@v5 upload   │
                              │     (non-blocking, D-02)       │
                              └───────────────┬────────────────┘
                                              │
                                              ▼
                              ┌──────────────────────────────┐
                              │  merge gate: coverage +       │
                              │  cli-tests + bench-check +    │
                              │  existing required checks     │
                              │  ALL must be green             │
                              └──────────────────────────────┘

Local reproduction (PIPE-03):
  make coverage  ──▶  cargo llvm-cov --workspace --features integration-tests
                       --lcov --output-path lcov.info
                       (requires make services-up: Redis + MinIO running locally
                        for the figure to match CI — planner's call whether the
                        target declares this dependency or fails loudly)
```

### Recommended Project Structure (additions only — existing tree unchanged)

```
src/
├── test_support/            # NEW (D-08) — #[cfg(test)]-gated, importable by
│   │                        # co-located `#[cfg(test)] mod tests` blocks in
│   │                        # src/ (tests/ is a separate crate, can't reach this)
│   ├── mod.rs                # barrel re-export, mirrors tests/helpers/mod.rs shape
│   └── failing_channel_handler.rs  # D-10's NotificationChannelHandler double
├── core/platform/manager/user_service.rs      # DEFER-02 target, existing tests extended
└── application/services/orchestration/listener.rs  # DEFER-03 target, existing tests extended

.github/workflows/
├── ci.yml                   # +cli-tests, +bench-check, +coverage jobs (D-03, D-07)
└── integration-tests.yml    # coverage step + codecov-action@v3 DELETED (D-03)

Makefile                     # +Coverage section between Testing and Code Quality (PIPE-03)
.codecov.yml                 # NEW — reporting only, non-blocking (D-02)
docs/src/contributing/testing-guide.md  # extended, not created (D-13)
```

### Pattern 1: Reuse the existing Redis/MinIO service block for the new `coverage` job

**What:** `ci.yml`'s existing `integration-tests` job (`ci.yml:374-400`) already defines a working
`services:` block with health checks for Redis (port 6380→6379) and MinIO (port 9010→9000).
**When to use:** The new `coverage` job needs the exact same services, since D-01 measures coverage
under `--features integration-tests` (the same feature the existing `integration-tests` job
exercises).
**Example:**
```yaml
# Source: .github/workflows/ci.yml:374-388 (existing pattern, copy as-is)
coverage:
  name: Coverage
  runs-on: ubuntu-latest
  services:
    redis:
      image: redis:7-alpine
      ports:
        - 6380:6379
      options: >-
        --health-cmd "redis-cli ping"
        --health-interval 10s
        --health-timeout 5s
        --health-retries 5
    minio:
      image: minio/minio:latest
      ports:
        - 9010:9000
        - 9011:9001
      env:
        MINIO_ROOT_USER: testuser
        MINIO_ROOT_PASSWORD: testpass123
      options: >-
        --health-cmd "curl -f http://localhost:9000/minio/health/live"
        --health-interval 30s
        --health-timeout 20s
        --health-retries 3
      command: server /data --console-address ":9001"
  steps:
    - uses: actions/checkout@v5
    - uses: dtolnay/rust-toolchain@stable
      with:
        components: llvm-tools-preview   # REQUIRED — cargo-llvm-cov needs this component
    - uses: taiki-e/install-action@v2
      with:
        tool: cargo-llvm-cov
    # ... MinIO client setup + wait-for-services steps, copied from ci.yml:404-432
    - name: Measure coverage
      env:
        USE_EXTERNAL_TEST_SERVICES: "true"
        TEST_REDIS_HOST: localhost
        TEST_REDIS_PORT: 6380
        TEST_MINIO_ENDPOINT: localhost:9010
        TEST_MINIO_ACCESS_KEY: testuser
        TEST_MINIO_SECRET_KEY: testpass123
      run: |
        cargo llvm-cov --workspace --features integration-tests \
          --lcov --output-path lcov.info
          # D-04 commit 1: no --fail-under-lines yet (measure-only mode)
          # D-04 commit 2: add --fail-under-lines <re-derived floor>
```

### Pattern 2: `cargo bench --no-run` as a compile-only benchmark gate

**What:** Compiles every `[[bench]]` target without executing it — catches API breakage and
benchmark bitrot with no Criterion runtime cost.
**When to use:** PIPE-01's `bench-check` job. Distinct from the existing scheduled/manual
`benchmark` job (`ci.yml:779`), which actually runs the benchmarks and is unchanged by this phase.
**Example:**
```yaml
# Source: cargo-bench CLI docs (`cargo bench --help`); pattern already implied by
# REQUIREMENTS.md PIPE-01 text and confirmed as a standard Rust CI idiom.
bench-check:
  name: Benchmark Compile Check
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v5
    - uses: dtolnay/rust-toolchain@stable
    - uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-bench-${{ hashFiles('**/Cargo.lock') }}
    - run: cargo bench --workspace --no-run
```

### Pattern 3: Feature-flagging the CLI test target correctly

**What:** `Cargo.toml:210-213` sets `required-features = ["cli"]` on the `cli` integration-test
target. Without `--features cli`, `cargo test --test cli` silently compiles nothing (Cargo skips
targets whose required features aren't active — no error, no output, zero tests run).
**When to use:** D-07's `cli-tests` job. This is CONTEXT.md's finding 2 and is the entire reason
the 86 snapshots have never run in CI: `crate-isolation`'s `paladin-ai` leg (`ci.yml:319-372`)
already runs the rest of `tests/` under default features, but silently excludes this one target.
**Example:**
```yaml
# Source: Cargo.toml:210-213 (target definition) + verified this session via
# `grep -c '#\[test\]' tests/cli/*.rs` → 97 across 7 files, 86 .snap files
cli-tests:
  name: CLI Snapshot Tests
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v5
    - uses: dtolnay/rust-toolchain@stable
    - uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-cli-${{ hashFiles('**/Cargo.lock') }}
    - run: cargo test -p paladin-ai --features cli --test cli
```

### Pattern 4: Hand-written `Arc<Mutex<..>>` recording mock (the shape D-09/D-10 copy)

**What:** The existing `tests/helpers/mock_llm_adapter.rs` convention — a queue of canned
responses plus a call-recording vector, both behind `Arc<Mutex<..>>` for `Send + Sync`
async-test compatibility.
**When to use:** Any new mock DEFER-01 actually needs (demand-driven, per D-10) — e.g. the
`FailingChannelHandler` for `NotificationChannelHandler`.
**Example:**
```rust
// Source: tests/helpers/mock_llm_adapter.rs (read directly this session) —
// the pattern to replicate in src/test_support/, relocated per D-08.
pub struct MockLlmAdapter {
    responses: Arc<Mutex<VecDeque<MockResponse>>>,
    invocations: Arc<Mutex<Vec<Invocation>>>,
}
// New seam confirmed this session — notification_orchestrator/mod.rs:424-427:
//   pub async fn register_channel_handler(&self, handler: Arc<dyn NotificationChannelHandler>)
// notification_orchestrator/types.rs:49-66 — the trait to implement:
//   #[async_trait]
//   pub trait NotificationChannelHandler: Send + Sync {
//       fn channel(&self) -> NotificationChannel;
//       fn can_handle(&self, notification: &Notification) -> bool;
//       async fn handle_notification(&self, notification: Notification)
//           -> NotificationOrchestratorResult<NotificationDeliveryResult>;
//       async fn health_check(&self) -> bool;
//   }
// A FailingChannelHandler::handle_notification that always returns Err(..) forces
// the failure path user_service.rs:228 already handles non-blockingly:
//   if let Err(e) = self.send_welcome_notification(&saved_user).await { /* log, don't fail */ }
```

### Anti-Patterns to Avoid

- **Installing `cargo-llvm-cov` without `llvm-tools-preview`:** the job will fail with a missing
  LLVM-tools component error on the very first `cargo llvm-cov` invocation. Not currently present
  anywhere in this workspace's CI (`grep -rn llvm-tools .github/workflows/` → no matches) — it must
  be added explicitly in the new `coverage` job's toolchain step.
- **Re-pinning `taiki-e/install-action`'s `tool:` to the stale `cargo-llvm-cov@0.7.1`** from
  REQUIREMENTS.md's original text: 0.8.7 is current on crates.io as of this session. Either omit
  the version (installs latest) or update the pin — do not carry the stale pin forward silently.
- **Upgrading `codecov/codecov-action@v3` to `@v4` per REQUIREMENTS.md's literal PIPE-04 text:**
  D-03 already supersedes this — the reference at `integration-tests.yml:123` is **deleted**, not
  upgraded. If a new Codecov upload step lands in the `coverage` job per D-02, use `@v5` (current),
  not `@v4`.
- **Trusting REQUIREMENTS.md's "43 CLI snapshot tests" / "14 job ci.yml" / any cited line number
  without re-grepping.** Confirmed again this session: 86 `.snap` files, 97 `#[test]` fns, 15 jobs.
  CONTEXT.md's own opening line says this generalizes project-wide; this research reconfirms it.
- **Using `--all-features` for the coverage measurement**, as REQUIREMENTS.md's PIPE-02 text
  literally specifies. D-01 explicitly rejects this: `qdrant` needs a live Qdrant service and the
  vision/embedding suites need real API keys, neither available in CI, so that code would enter
  the denominator with nothing able to exercise it, depressing the number for no signal.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|--------------|-----|
| Coverage instrumentation | A custom `rustc -C instrument-coverage` + `llvm-profdata`/`llvm-cov` pipeline (what ADR-0006's own measurement used, forced by a Docker/network-absent environment) | `cargo-llvm-cov` (wraps the same LLVM pipeline) | ADR-0006 itself already documents both paths and states they are "expected to agree only when the ignore regex, doctest decision, and feature set all match" — the workflow-based path is simpler and is already tool-of-record in `integration-tests.yml` |
| CI-produced coverage threshold enforcement | A custom script that parses `lcov.info`/JSON and fails the build | `cargo llvm-cov --fail-under-lines <N>` | Built-in CLI flag, exact behavior documented, no report-parsing script to maintain (D-02's own rationale) |
| Async test doubles for `Send + Sync` traits | A one-off mock per test, or a hand-rolled trait-object dispatcher | The existing `Arc<Mutex<VecDeque<..>>>` recording pattern from `tests/helpers/mock_llm_adapter.rs` | Proven shape already used three times in the workspace; D-09 explicitly declines `mockall` as a second idiom nobody would retrofit |
| GitHub Actions workflow linting | Custom YAML/expression-syntax checks | `actionlint` (+ optionally `reviewdog/action-actionlint` for PR annotations) | Purpose-built static checker for GitHub Actions YAML and its expression language; PIPE-04's explicit acceptance bar |

**Key insight:** every "don't hand-roll" item in this phase already has a tool the project either
already uses (`cargo-llvm-cov`) or has explicitly decided not to duplicate (mocks). This phase is
almost entirely wiring, not novel engineering — the risk is in configuration mistakes
(missing `llvm-tools-preview`, wrong feature scope, stale action-version pins), not in solving a
hard problem.

## Runtime State Inventory

Not applicable — this phase does not rename, refactor, or migrate any identifier, database key, or
external-service configuration. It adds new CI jobs/Makefile targets, corrects stale documentation
claims in place, and adds test code. No stored data, live service config, OS-registered state, or
build-artifact renaming is in scope. **Nothing found in any category — verified by reading the
phase's own `<domain>` boundary in CONTEXT.md and confirming no rename/relocate language appears
in any of PIPE-01…05 or DEFER-01…03.**

## Common Pitfalls

### Pitfall 1: `cargo-llvm-cov` job fails immediately with a missing-component error
**What goes wrong:** `cargo llvm-cov` requires the `llvm-tools-preview` rustup component (or a
nightly toolchain with `-Z instrument-coverage` support) to instrument coverage. Every existing
`dtolnay/rust-toolchain@stable` invocation in this repo's `ci.yml` installs the toolchain without
it.
**Why it happens:** `llvm-tools-preview` is not a default rustup component and none of the
project's fifteen existing `ci.yml` jobs need it (they don't measure coverage).
**How to avoid:** Add `components: llvm-tools-preview` to the `dtolnay/rust-toolchain@stable` step
inside the new `coverage` job specifically — do not add it globally, since it has no benefit for
the other fourteen jobs.
**Warning signs:** `error: failed to run llvm-cov` or a "component not found" message immediately
after `cargo llvm-cov` invocation, before any test runs.

### Pitfall 2: The coverage figure doesn't match ADR-0006's prior offline measurement
**What goes wrong:** Comparing the new CI-produced `--features integration-tests` figure against
ADR-0006's existing 85.85% default-feature-scope figure and treating any delta as a regression.
**Why it happens:** ADR-0006 itself already warns: the two commands' denominators are "expected to
agree only when the ignore regex, the doctest decision, and the feature set all match" — and today
they explicitly don't (CI additionally runs `--features integration-tests`).
**How to avoid:** D-01/D-04 handle this correctly by construction — measure first (no gate), record
the new figure as the new baseline, re-derive the floor from *that* figure, never compare it
against the old default-feature number as if they were the same scope.
**Warning signs:** A PR that "regresses coverage" on day one of the new job landing, when nothing
in the diff touches test code — this means the scope-change wasn't isolated from the gate-turn-on
step.

### Pitfall 3: `--all-features` silently tanks the coverage percentage
**What goes wrong:** Passing `--all-features` (as REQUIREMENTS.md's PIPE-02 text literally
specifies) pulls in `qdrant` (needs a live Qdrant service, not present in CI) and vision/embedding
code paths (need real provider API keys, not present in CI) — that code enters the denominator
with zero CI-exercisable lines, depressing the percentage with no corresponding signal.
**Why it happens:** `--all-features` is the most obvious flag to reach for and is what the original
requirement text specifies, without accounting for which features have a live CI backing service.
**How to avoid:** D-01 already settles this — use `--workspace --features integration-tests`
specifically, not `--all-features`.
**Warning signs:** A sudden double-digit coverage drop the moment `--all-features` is tried locally
against a tree that hasn't changed its test suite.

### Pitfall 4: Deleting `integration-tests.yml`'s coverage step breaks its own job silently
**What goes wrong:** The coverage-generation step in `integration-tests.yml:113-127` has
`continue-on-error: true` on both the generation and upload steps, meaning its historical failure
mode has been invisible. Deleting it (D-03) is correct, but a planner should confirm no other step
in that same job or a downstream consumer depends on `integration-lcov.info` existing.
**Why it happens:** `continue-on-error: true` was originally added because Codecov uploads can fail
silently without a token (per PIPE-02's own text) — the flag masks failures, which is a separate
problem from whether the step should exist at all.
**How to avoid:** Grep the full workflow set for `integration-lcov.info` before deleting the step
that produces it (this session's grep of `.github/workflows/*.yml` found no other reference to that
filename, confirming it's safe to delete outright).
**Warning signs:** A downstream `if: needs.integration-tests.outputs.something` or artifact-upload
step referencing the deleted file.

### Pitfall 5: Trusting REQUIREMENTS.md's literal action-line-number citations
**What goes wrong:** PIPE-04's text cites `ci.yml:147`, `ci.yml:317`, `ci.yml:507` for the three
`actions-rs/toolchain@v1` occurrences. Re-verified this session, the actual lines are
**163, 408, 788** — all three have moved since the requirement was written (the file has grown as
other phases added jobs).
**Why it happens:** `ci.yml` is a 33KB, actively-edited file across many phases; any line-number
citation in a document older than the most recent edit is presumptively stale.
**How to avoid:** Always re-grep (`grep -n "actions-rs/toolchain@v1" .github/workflows/*.yml`)
immediately before editing — never patch by line number from a requirement or research document.
**Warning signs:** An `Edit` tool call whose `old_string` context doesn't match what's actually at
the cited location.

## Code Examples

### Makefile Coverage section (PIPE-03)

```makefile
# Source: pattern derived from cargo-llvm-cov's own documented CLI flags
# (verified via WebSearch this session), matching the CI command in D-01/D-02.
##@ Coverage

.PHONY: coverage
coverage: ## Measure workspace coverage (mirrors CI's `coverage` job — requires make services-up)
	@echo "$(CYAN)Measuring coverage...$(NC)"
	@$(CARGO) llvm-cov --workspace --features integration-tests \
		--lcov --output-path lcov.info

.PHONY: coverage-html
coverage-html: ## Generate an HTML coverage report at target/coverage
	@echo "$(CYAN)Generating HTML coverage report...$(NC)"
	@$(CARGO) llvm-cov --workspace --features integration-tests \
		--html --output-dir target/coverage
	@echo "Report at target/coverage/html/index.html"
```

### CI action-version replacement table (PIPE-04 — all eight verified this session)

```
# actions-rs/toolchain@v1  →  dtolnay/rust-toolchain@stable
#   ci.yml:163  (api-surface job)
#   ci.yml:408  (integration-tests job)
#   ci.yml:788  (benchmark job)
#   integration-tests.yml:71

# actions/cache@v3  →  actions/cache@v4
#   integration-tests.yml:78, :84, :90

# codecov/codecov-action@v3  →  DELETED (D-03), not upgraded
#   integration-tests.yml:123
```

### Threshold arithmetic (ADR-0006's own rule, unchanged by this phase — restated for reference)

```
floor = truncate_toward_zero(measured_percentage)   # e.g. 84.79% → 84%
comparison: measured >= floor   # at-or-above passes; below fails
# Applied to whatever new figure D-01's --features integration-tests scope produces.
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|-------------------|---------------|--------|
| `actions-rs/toolchain@v1` | `dtolnay/rust-toolchain@stable`/`@master` | `actions-rs/toolchain` unmaintained since ~2021 | Loses automatic Rust-version resolution improvements, has no security patching |
| `actions/cache@v3` | `actions/cache@v4` | v4 released 2024 | v3 relies on a deprecated cache-service API GitHub is sunsetting |
| `codecov/codecov-action@v3` (Node uploader, supports tokenless) | `codecov/codecov-action@v5` (CLI-based, requires token except fork-PR-to-public-repo) | v4 (2024) dropped tokenless uploads on GH Actions; v5 continues that model but adds Codecov Wrapper for faster updates | If public-repo tokenless behavior is relied on, confirm `CODECOV_TOKEN` is configured or an org-level Global Upload Token is set — otherwise uploads fail (non-blocking here per D-02, but still worth getting right for the reporting to actually work) |
| `cargo tarpaulin` (ptrace-based instrumentation) | `cargo-llvm-cov` (LLVM source-based instrumentation) | ADR-0006 already designates `cargo-llvm-cov` as tool-of-record; the two are not comparable | `TESTING.md:319-322` still documents tarpaulin locally — D-14 corrects this, noting tarpaulin can be retained as an informal alternative but not the CI-matching number |

**Deprecated/outdated:**
- `actions-rs/*` action family generally: the maintainer archived the org years ago; any remaining
  reference anywhere in this repo (only the three `actions-rs/toolchain@v1` sites found) should be
  treated as technical debt regardless of whether this phase's scope reaches it.
- `codecov-action@v3`'s Node-based uploader: superseded by the CLI-based uploader in v4+, though
  this phase deletes rather than upgrades the one instance where it appears.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `insta` is pinned at 1.34 in `Cargo.toml`'s dev-dependencies (not independently re-verified against `Cargo.toml` this session, only cited from TESTING.md via CONTEXT.md) | Standard Stack → Supporting | Low — PIPE-01 doesn't touch the snapshot-testing library itself, only makes the existing suite run in CI; an inaccurate version number here has no execution impact |
| A2 | `serial_test` 3.2, `proptest` 1.4, `testcontainers` 0.24 as listed in TESTING.md's dev-dependency inventory are current in `Cargo.toml` | Standard Stack → Supporting | Low — none of these are touched by this phase's requirements; listed for planner awareness only |
| A3 | The planner will choose to keep a (non-blocking) Codecov upload step in the new `coverage` job rather than dropping Codecov entirely | Standard Stack → codecov-action row | Low-Medium — D-02 explicitly permits `.codecov.yml` reporting without a gate, but doesn't mandate the upload step exist in the *new* job specifically; if the planner drops it, the `codecov-action@v5` recommendation above is moot |

**If this table is empty:** N/A — three low-risk assumptions logged above, none blocking planning.

## Open Questions

1. **Should `taiki-e/install-action`'s `tool:` input pin an exact `cargo-llvm-cov` version, or float to latest?**
   - What we know: crates.io currently serves 0.8.7; REQUIREMENTS.md's original text specifies a
     stale `0.7.1` pin.
   - What's unclear: whether the project's general policy (seen elsewhere, e.g.
     `cargo install --locked`) favors pinned tool versions for reproducibility.
   - Recommendation: pin explicitly to the version verified at plan-time (0.8.7 as of this
     research) rather than floating, consistent with the project's `--locked` convention used
     elsewhere in the Makefile (`cargo install --locked cargo-release`, `cargo-deny`).

2. **Does the new `coverage` job also need a Codecov upload step, or does `.codecov.yml` alone suffice for reporting?**
   - What we know: D-02 says Codecov reports but doesn't gate; `.codecov.yml` "lands" per PIPE-02.
   - What's unclear: `.codecov.yml` configures Codecov's *interpretation* of uploaded reports (PR
     comment layout, ignore paths, status blocks) — it has no effect without something actually
     uploading a report to Codecov. If the only upload path (`integration-tests.yml:113-127`) is
     deleted per D-03 and nothing replaces it in `ci.yml`, `.codecov.yml` would be dead
     configuration with nothing to configure.
   - Recommendation: the new `coverage` job in `ci.yml` should include a
     `codecov/codecov-action@v5` upload step (non-blocking: no `fail_ci_if_error: true`,
     consistent with D-02's "Codecov reports, it does not gate") so `.codecov.yml` has an upload to
     act on. Flag this for the planner to confirm explicitly since it isn't spelled out verbatim in
     any of CONTEXT.md's D-01…D-14.

3. **Which six workflows get `actionlint` run against them, and how (CI job vs. local/pre-commit)?**
   - What we know: PIPE-04's text says "all three workflows" but six exist
     (`ci`, `docs`, `feature-flags`, `integration-tests`, `pre-commit`, `release`) — CONTEXT.md's
     Claude's Discretion section already flags this and defers the choice to the planner.
   - What's unclear: whether `actionlint` runs as a new CI job (most robust — catches regressions
     on every PR) or only as a one-time/local check (per PIPE-04's literal "zero errors" bar being
     a point-in-time acceptance criterion, not necessarily an ongoing gate).
   - Recommendation: add it as a lightweight CI job (actionlint is fast, no external services) so
     the "zero deprecated action" and "zero actionlint errors" bars stay enforced going forward,
     not just verified once at phase-close. This is consistent with the phase's own theme ("measure
     quality on every push instead of asserting it").

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|--------------|-----------|---------|----------|
| Docker | PIPE-02 (Redis/MinIO-backed coverage measurement) | ✗ (verified this session: `docker: command not found`) | — | None locally — D-04's two-step landing exists specifically because the wider-scope figure can only be produced by CI itself, not reproduced here. No local fallback measurement is possible or expected. |
| `cargo-llvm-cov` | PIPE-02, PIPE-03, DEFER-02, DEFER-03 (module-targeted coverage) | ✗ locally (not installed) | crates.io serves 0.8.7 | Installable via `cargo install cargo-llvm-cov --locked` or `taiki-e/install-action` in CI — network to crates.io confirmed reachable this session (`cargo search` succeeded), so installation itself is not blocked, only not pre-installed |
| `actionlint` | PIPE-04 | ✗ locally (not installed) | v1.7.12 current | Installable via `go install github.com/rhysd/actionlint/cmd/actionlint@latest`, a downloaded release binary, or run only inside a `reviewdog/action-actionlint` CI job (no local install required at all) |
| crates.io registry | `cargo search`, tool installation | ✓ (verified this session: `cargo search cargo-llvm-cov` succeeded) | — | — |

**Missing dependencies with no fallback:**
- Docker (blocks any local reproduction of the `--features integration-tests` coverage figure —
  this is exactly D-04's premise, not a new problem this research surfaces).

**Missing dependencies with fallback:**
- `cargo-llvm-cov` and `actionlint` are both trivially installable when needed; their absence here
  reflects a clean environment, not a real blocker.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in) + `insta` [CITED: TESTING.md] for CLI snapshot assertions |
| Config file | none dedicated — feature-gated `[[test]]` targets in `Cargo.toml` (`:191-217`) |
| Quick run command | `cargo test --workspace --lib --bins` (existing `make test`) |
| Full suite command | `cargo test --workspace --features integration-tests -- --test-threads=1` (existing `integration-tests.yml:111` pattern, requires Redis+MinIO) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|---------------------|--------------|
| PIPE-01 | CLI snapshots run and gate CI | snapshot (insta) | `cargo test -p paladin-ai --features cli --test cli` | ✅ `tests/cli/` (86 snapshots, 97 tests, all exist) |
| PIPE-01 | Benchmarks compile | compile-check | `cargo bench --workspace --no-run` | ✅ `[[bench]] config_benchmarks` at `Cargo.toml:254-256` |
| PIPE-02 | Combined coverage measured and gated | CI job (integration, service-backed) | `cargo llvm-cov --workspace --features integration-tests --fail-under-lines <floor>` | ❌ Wave 0 — new `coverage` job to be authored |
| PIPE-03 | Coverage reproducible locally | manual/local | `make coverage` | ❌ Wave 0 — new Makefile target |
| PIPE-04 | Zero actionlint errors | static analysis | `actionlint .github/workflows/*.yml` | ❌ Wave 0 — actionlint not currently run anywhere in this repo |
| DEFER-02 | `user_service.rs` ≥80% module coverage | unit (`#[tokio::test]`) | `cargo llvm-cov --workspace -- --package paladin` targeted, or module-filtered `cargo llvm-cov report` | ✅ `user_service.rs:467-583` has 5 existing tests to extend |
| DEFER-03 | Listener orchestrator ≥80%, concurrency suite | unit + concurrency-focused | `cargo llvm-cov` module-targeted; concurrency tests via `tokio::test(flavor = "multi_thread")` | ✅ `listener.rs:400,471,488,514` has 3 existing tests to extend |

### Sampling Rate

- **Per task commit:** `cargo test --workspace --lib --bins` (fast, no services)
- **Per wave merge:** `cargo llvm-cov --workspace --features integration-tests` (requires
  `make services-up`), plus `cargo test -p paladin-ai --features cli --test cli`
- **Phase gate:** Full suite green (`make ci-test` extended with `test-cli`, plus the new
  `coverage` job's threshold) before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `.github/workflows/ci.yml` — new `cli-tests`, `bench-check`, `coverage` job definitions
- [ ] `Makefile` — new Coverage section (`coverage`, `coverage-html`), `test-cli`, `bench-check`
  targets, `ci-test` extended, new `ci-full` target
- [ ] `.codecov.yml` — new file at repo root
- [ ] `src/test_support/` — new module (or planner-chosen name) for D-08's shared mocks
- [ ] `actionlint` invocation — not currently run anywhere in CI or locally; needs either a new CI
  job or a documented local/pre-commit check

*(No existing test infrastructure gap for the coverage-target modules themselves — both
`user_service.rs` and `listener.rs` already have working `#[cfg(test)]`/`#[tokio::test]` scaffolding
to extend, confirmed by direct read this session.)*

## Security Domain

`security_enforcement` is absent from `.planning/config.json` → treated as enabled. This phase's
security-relevant surface is narrow: DEFER-02 adds test coverage (not new production code paths) to
`user_service.rs`, which contains the workspace's password-hashing logic
(`Argon2<'static>`, confirmed at `user_service.rs:9,32`).

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|----------------|---------|-------------------|
| V2 Authentication | Yes (test coverage only, no new logic) | `argon2` crate, already in use — DEFER-02 adds tests for correct/incorrect password paths, not a new hashing implementation |
| V3 Session Management | Indirectly | `login_user`/token-issuance path already tested (`login_issues_token_when_auth_port_configured`, `user_service.rs` existing test) — DEFER-02 extends, does not change, the mechanism |
| V4 Access Control | No | Out of scope — no authorization logic changes in this phase |
| V5 Input Validation | Yes (test coverage only) | DEFER-02's own scope text explicitly lists "invalid username formats, invalid email... Unicode inputs, empty/whitespace inputs" as required edge-case tests |
| V6 Cryptography | Yes (test coverage only, no new code) | `argon2` password hashing — DEFER-02 adds assertions, must **not** introduce a hand-rolled hash comparison or a new crypto dependency; use the existing `Argon2`/`PasswordHash`/`PasswordVerifier` imports already present at `user_service.rs:10-11` |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|------------------------|
| Timing attack on password comparison | Information Disclosure | Already mitigated — `argon2::PasswordVerifier` performs constant-time comparison internally; DEFER-02 must not replace this with a manual `==` string comparison in any new test helper or mock |
| Notification-failure-blocks-registration (partial DoS on legitimate signups) | Denial of Service | Already correctly implemented — `register_user:228` uses `if let Err(e) = ...` (non-blocking), not `?`. DEFER-02's job is to *prove* this with a test (the `FailingChannelHandler` seam), not to change the behavior. Do not "fix" what is already correct. |
| Test-only mock accidentally reachable in production builds | Tampering (if a mock could be swapped in for the real dependency) | The `#[cfg(test)]`-gated placement in `src/test_support/` (D-08) already ensures this can't compile into release builds — confirm the module is behind `#[cfg(test)]` at the module-declaration site, not only inside individual functions |

## Sources

### Primary (HIGH confidence)
- Direct file reads this session: `.github/workflows/ci.yml` (full `uses:`/job-name grep),
  `.github/workflows/integration-tests.yml` (full read), `.github/workflows/docs.yml`,
  `feature-flags.yml`, `pre-commit.yml`, `release.yml` (action-version greps), `Makefile` (full
  read), `Cargo.toml` (test/bin/feature sections), `.planning/decisions/0006-coverage-gate.md`
  (full read), `src/core/platform/manager/user_service.rs` (full read of fields + tests module),
  `src/application/services/orchestration/listener.rs` (header + test-line greps),
  `src/application/services/notification_orchestrator/mod.rs` and `types.rs` (seam confirmation),
  `tests/helpers/mock_llm_adapter.rs` (pattern read), `tests/cli/` directory listing + snapshot/test
  counts, `CLAUDE.md`/`.github/copilot-instructions.md`/`.planning/codebase/TESTING.md`
  (coverage-claim greps).
- `gsd-tools query package-legitimacy check --ecosystem crates cargo-llvm-cov` — `OK` verdict.
- `cargo search cargo-llvm-cov` (crates.io reachable, 0.8.7 current).

### Secondary (MEDIUM confidence)
- WebSearch: `codecov-action` v4/v5 tokenless-upload behavior (codecov/codecov-action GitHub
  issues/releases).
- WebSearch: `cargo-llvm-cov --fail-under-lines` usage patterns and `taiki-e/install-action`
  invocation shape (taiki-e/cargo-llvm-cov's own CI workflow, cross-rs/cross's reusable action).
- WebSearch: `actionlint`/`reviewdog/action-actionlint` current release versions (rhysd/actionlint
  and reviewdog/action-actionlint GitHub Releases pages).

### Tertiary (LOW confidence)
- `insta` 1.34 / `serial_test` 3.2 / `proptest` 1.4 / `testcontainers` 0.24 version claims —
  sourced from `.planning/codebase/TESTING.md` via `15-CONTEXT.md`'s canonical_refs, not
  independently re-verified against `Cargo.toml`'s `[dev-dependencies]` this session (see
  Assumptions Log A1/A2). Low risk since this phase doesn't touch those dependencies.

## Metadata

**Confidence breakdown:**
- Standard stack (cargo-llvm-cov, actions versions): HIGH — every version independently verified
  via `cargo search`, `gsd-tools package-legitimacy check`, or WebSearch against official release
  pages this session.
- Architecture (CI job structure, feature scoping): HIGH — every code site (job names, line
  numbers, feature flags, existing service blocks) directly re-read from the live tree this
  session, matching CONTEXT.md's independently-verified findings.
- Pitfalls: HIGH for the `llvm-tools-preview` finding (independently discovered this session, not
  present in CONTEXT.md or REQUIREMENTS.md) and the `--all-features`/action-version pitfalls
  (directly traceable to CONTEXT.md's locked decisions); MEDIUM for the Codecov-upload-step open
  question (a genuine gap neither CONTEXT.md nor REQUIREMENTS.md resolves).

**Research date:** 2026-08-13
**Valid until:** 7 days for exact tool/action version pins (fast-moving GitHub Actions ecosystem);
30 days for the architectural/pitfall guidance (stable regardless of point-version drift). Re-grep
all cited line numbers immediately before editing regardless of this document's age — Pitfall 5
above documents why.

---

*Phase: 15-coverage-ci-quality-gates*
*Research completed: 2026-08-13*
