# Phase 3: Verification Depth - Research

**Researched:** 2026-08-02
**Domain:** Rust workspace test coverage measurement (offline LLVM instrumentation), error-path
integration testing (Commander/Battalion), MCP protocol failure-mode testing (rmcp 2.1.0), and
`criterion` benchmark execution/percentile derivation.
**Confidence:** HIGH — every claim below was checked directly against the shipped tree, the vendored
crate sources in the local cargo registry, or a command actually executed in this sandbox. Nothing
in this file is carried over from CONTEXT.md without independent re-verification; three places below
**correct or refine** a CONTEXT.md assumption based on that re-verification (flagged inline as
`CORRECTION`).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**What "coverage work" actually means in this phase**

- **D-01: QUAL-01 is measure-and-hold, not raise.** ADR-0006's floor is **84%**, derived by
  truncating a measured **84.79%**. The tree already cleared it on the run that set the gate — by
  construction, the gate cannot be red on that run. So Phase 3's QUAL-01 obligation is **not** "write
  tests until a number rises". It is: re-run ADR-0006's command **verbatim** at current HEAD
  (`f29d4526`, **98 commits** past the measurement commit `9be788c8`, including all of Phase 2's new
  tests), record the result to the Phase 1 provenance standard, and prove the figure is still at or
  above 84%. Test-writing in this phase is scoped by QUAL-02's zero-coverage set (D-03) and QUAL-04's
  error paths (D-07/D-08), **not** by a coverage delta target.

- **D-02: The command is run verbatim; the denominator is not "improved" mid-phase.** ADR-0006 fixes
  the `RUSTFLAGS`/`LLVM_PROFILE_FILE`/`cargo test --workspace --offline` invocation, the
  `--ignore-filename-regex`, the doctest exclusion, and the default-feature scope. Phase 3 changes
  none of them. Any change to the regex or feature set changes the denominator and therefore the
  number, which requires an in-place ADR amendment with a new figure and date — not a phase-level
  convenience.

**The stale QUAL-02 offender list — the largest finding in this phase's analysis**

- **D-03: QUAL-02's named offenders are stale and get re-derived, then amended at source.**
  QUAL-02 names eleven files as 0% or sub-15%. The Phase 1 measurement contradicts nine of them
  (`arsenal_execution_service.rs` 92.26%, `arsenal_registry_service.rs` 100%,
  `sqlite_user_repository.rs` 91.75%, `user_controller.rs` 77.54%, `main.rs` 47.37%,
  `campaign_service.rs` 85.66%, `chain_of_command_service.rs` 90.75%, `mcp_protocol.rs` 95.73%,
  `deepseek_adapter.rs` 42.45%). Only `redis.rs` (0.00%) is a true positive; `minio.rs` is absent
  from the denominator (D-04). Phase 3 re-derives the zero-coverage set from its own measurement and
  amends QUAL-02 and ROADMAP success criterion 2 at source with dated provenance.

- **D-04: The re-derived first-party zero-coverage set, as of the Phase 1 measurement.** Five
  first-party source files reported 0.00%: `crates/paladin-storage/src/redis.rs` (361 counted lines,
  the largest by an order of magnitude), `src/bin/paladin-server.rs` (185 lines),
  `crates/paladin-ports/src/output/file_storage_port.rs` (117 lines),
  `crates/paladin-llm/src/error.rs` (19 lines — the dead `LlmProviderError` conversion path),
  `crates/paladin-ports/src/output/arsenal_port.rs` (2 lines). Excluded: `tests/**` entries and one
  generated file under `target/` (D-06).

- **D-05: `redis.rs` is closed with Docker-free unit tests on its pure seams.** 1,570 lines, zero
  `#[cfg(test)]` modules. Its only exercisers are testcontainers-based and require Docker, which is
  absent. Phase 3 adds in-file `#[cfg(test)] mod tests` covering config construction/defaults,
  key/namespace construction, queue-item serialization round-trips, priority ordering, and error
  mapping. Live-server paths stay uncovered with a `deferred with reason` row naming Phase 15 (PIPE).

- **D-06: The `target/` denominator contamination is reported, not fixed.** ADR-0006's ignore regex
  excludes `examples/`, `benches/`, `crates/doc-examples/`, cargo registry, and stdlib — but not
  `target/`. One generated file at 0.00% (1 line of 61,404, immaterial) is recorded with its
  magnitude and flagged for VERIFY-05/PIPE-02; the regex is not amended (per D-02).

- **D-07: QUAL-02 binds only inside ADR-0006's recorded scope.** `minio.rs` exists but never appears
  in the measurement — the `s3` feature is not in the workspace default set. Gets a
  `deferred with reason` row naming VERIFY-05/PIPE-02 as owner. ROADMAP criterion 2 names "the Redis
  and MinIO adapters" together; the D-03 amendment must record they are not in the same position.

**QUAL-03's second number**

- **D-08: QUAL-03's percentage clause is recorded superseded by ADR-0006; its substance survives.**
  As written QUAL-03 asks for "integration coverage of critical paths ≥ 70%" — a second number under
  a second scope, which ADR-0006 abolished. Phase 3 records the percentage clause superseded and
  satisfies the surviving substance: each of the three named critical paths (Paladin execution,
  Battalion orchestration, tool invocation) has a named, passing, non-`#[ignore]`d integration
  exerciser meeting the D-19 bar (`file:line` plus a named exerciser). Amended at source in
  REQUIREMENTS.md with dated provenance.

**QUAL-04 — the error paths**

- **D-09: The four Commander tests move to integration, and the failing mock lands in
  `tests/helpers/`.** The four tests at `commander.rs:2180,2188,2196,2204` are `#[ignore]`d with
  completely empty bodies. Phase 3 builds a configurable `FaultyPaladinPort` in `tests/helpers/` and
  relocates the four tests to a new `tests/integration/commander_error_paths_test.rs`, with the
  ledger rows citing those four line numbers amended in place. The mock must support: fail-always;
  fail-the-Nth-Paladin (partial-failure collection); fail-N-then-succeed with an invocation counter
  (retry-count assertions); and a controllable delay (timeout-stops-siblings). Must be `Send + Sync`.
  Chosen over a new `paladin-test-support` workspace crate or a `test-support` feature gate (both add
  publishable/feature surface right before Phase 4's REL-01 version-coherence work). Sited in
  `tests/helpers/` — the workspace's existing shared mock home.

- **D-10: Extend the dead `tests/helpers/mock_paladin_port.rs`, do not add a sixth parallel mock.**
  That file is at 0.00% coverage because nothing imports it. At least six independent
  `MockPaladinPort` definitions exist elsewhere. D-09's `FaultyPaladinPort` extends the shared helper
  file and gives it its first real consumer. Consolidating the other five is explicitly out of scope.

- **D-11: The five MCP failure modes land on the Streamable-HTTP adapter, driven by `wiremock`.**
  QUAL-04 and ROADMAP criterion 4 name five modes: expired/rejected token, malformed response,
  handshake timeout, unknown tool, bad arguments. All five are hosted against
  `MCPStreamableHttpAdapter` in `src/infrastructure/adapters/arsenal/mcp_protocol.rs` — the shipped
  transport that supersedes SSE. `wiremock` v0.6 is already a dev-dependency. Unknown-tool and
  bad-arguments are protocol-level — they assert the adapter's error mapping on a well-formed HTTP
  200 carrying a JSON-RPC error, not an HTTP status code. Note: `mcp_protocol.rs` already measures
  95.73%, so this work is about the five named failure modes having tests, not moving a number.
  *(This research's Architecture Patterns/Pattern 3 refines the mechanism: prefer extending the
  already-shipped rmcp+axum hermetic fixture server over a wiremock-only harness for 3 of the 5
  modes — see that section for the full reasoning; this does not change the requirement, only the
  recommended implementation path.)*

**QUAL-05 — the performance baseline**

- **D-12: Run the five shipped bench targets; record the missing suites as gaps rather than building
  them.** The tree ships exactly five bench targets, all declared and all present. QUAL-05
  additionally names the Paladin execution loop and Arsenal invocation — the Milestone-1
  `paladin_benchmarks.rs`, `herald_benchmarks.rs` and `arsenal_benchmarks.rs` are not in the tree.
  Phase 3 runs what ships, records per-metric coverage explicitly, and files `deferred with reason`
  rows for the two absent surfaces. `criterion 0.5.1` source is present in the local cargo registry,
  so `cargo bench --offline` builds despite crates.io returning 403 (confirmed again in this
  research session via `cargo check --bench`).

- **D-13: Memory-per-Paladin and startup time are measured by a small recorded harness, not by new
  criterion suites.** Criterion produces throughput and latency; it produces neither
  memory-per-Paladin nor startup time. Phase 3 records them from a purpose-built, documented
  measurement (process RSS delta across a controlled number of constructed Paladins; wall-clock to a
  ready `paladin-server`/first-Paladin-constructed). The baseline document states which source
  produced each metric family.

- **D-14: P50/P95/P99 are derived from criterion's raw samples, with the derivation documented.**
  Criterion reports mean, median, MAD and confidence intervals — not P95 or P99. The percentiles are
  computed from criterion's own per-iteration sample data under `target/criterion/*/new/`, and the
  baseline document shows the derivation. *(This research's Code Examples section provides the exact
  verified `SavedSample` schema and formula.)*

- **D-15: The baseline is amended into `docs/src/appendix/performance-baseline.md` in place.** That
  document already exists and is already linked from the mdbook. Its current run is dated
  2026-05-27 against commit `f4156ff6` on different hardware. Phase 3 adds a new dated run section
  and retains the prior run as an explicitly superseded section.

**Measurement provenance — applies to every measurement in this phase**

- **D-16: Every recorded figure carries the Phase 1 provenance block, and every command carries
  `--offline`.** `rustc -vV`, `cargo --version`, `git rev-parse HEAD`, `date -u`, plus CPU model,
  core/thread count and kernel for the benchmark runs — captured immediately before the command,
  with raw stdout pasted, and arithmetic a reader can re-derive. Benchmark figures are recorded as
  this machine's baseline with the environment stated, explicitly not as a portable performance
  claim. Sandbox constraints stay stated plainly: no Docker, crates.io HTTP 403, `cargo-llvm-cov` not
  installable.

### Claude's Discretion

- **Plan decomposition and count.** No tracer is mandated. The natural shape is measurement-first
  (D-01 gates D-03/D-04, which gate the test-writing), but whether the re-measurement is its own
  plan or task 1 of a larger one is the planner's call.
- **Whether `crates/paladin-llm/src/error.rs`'s dead conversion path is deleted or exercised.**
  D-04 identifies it as a disposition question, not a test-writing one. Either a test that gives the
  `From<LlmProviderError> for LlmError` impl its first caller, or removal with a
  `superseded by shipped code` row — the planner should pick one and record which.
- **Whether `src/bin/paladin-server.rs` (185 lines, 0%) is closed or deferred.** A binary entrypoint
  is legitimately hard to unit-test; `main.rs` sits at 47.37% by comparison (confirmed in this
  research: the covered half is `Opt`-parsing tests already present; the uncovered half is
  `#[tokio::main] async fn main()`'s body itself). Closing it may mean extracting a testable `run()`
  seam, which is a refactor. Deferring it needs a named owner.
- **Where the ROADMAP/REQUIREMENTS amendments under D-03 and D-08 physically land** — in-place edits
  plus a Phase 3 amendments record, or in-place with provenance notes only.
- **Whether `crates/paladin-ports/src/output/file_storage_port.rs` (117 lines, 0%) is a real gap.**
  A port trait file at 0% usually means default method bodies and doc examples that nothing
  instantiated; check before writing tests against it. *(Confirmed in this research: the 0% slice is
  exactly the `FileStorageUtils` trait's default, no-`&self` associated functions
  (`detect_content_type`, `calculate_md5`, `validate_path`, `sanitize_filename`, etc.,
  `file_storage_port.rs:1347-1447`) — genuinely closeable with direct calls via
  `<SomeType as FileStorageUtils>::fn_name(...)`, not a refactor-first item like `redis.rs`.)*

**New discretion item surfaced by this research (not present in CONTEXT.md):**

- **Whether to add a configurable timeout parameter to `MCPClient::connect_streamable_http` /
  `MCPStreamableHttpAdapter::connect` to make the "handshake timeout" MCP failure-mode test
  affordable.** See Pitfall 4 below — the shipped `STREAMABLE_HTTP_HANDSHAKE_TIMEOUT` is a hardcoded
  private 30-second constant with no test seam, and Tokio's paused-time testing does not help because
  the timeout wraps real socket I/O. This decision point was not discussed in `/gsd-discuss-phase`
  and should be raised explicitly rather than resolved silently by the executor.

### Deferred Ideas (OUT OF SCOPE)

- **The two module-scoped coverage gates — Herald ≥ 95%, autonomous ≥ 90%.** Owner: Phase 5,
  VERIFY-05, assigned explicitly by ADR-0006.
- **The function-vs-line coverage gap** (77.34% function vs 84.79% line). Owner: Phase 5, VERIFY-05.
- **Extending the coverage scope to the Docker-backed `--features integration-tests` suite.**
  Owners: Phase 5 VERIFY-05 and Phase 15 PIPE-02. Blocked here by Docker's absence.
- **Wiring the 84% floor into CI.** Owner: Phase 15, PIPE-02.
- **`crates/paladin-storage/src/minio.rs` coverage** (D-07). Owner: VERIFY-05/PIPE-02.
- **`redis.rs`'s live-server code paths** (D-05). Owner: Phase 15, PIPE.
- **Fixing ADR-0006's `--ignore-filename-regex` to exclude `target/`** (D-06). Owner: VERIFY-05/PIPE-02.
- **Consolidating the six parallel `MockPaladinPort` definitions** (D-10). No owner assigned.
- **Writing Paladin-execution-loop and Arsenal-invocation benchmark suites** (D-12). No owner
  assigned in this discussion.
- **The live-API harness skip-vs-fail semantics.** Owner: Phase 5, VERIFY-06. The ~25 `#[ignore]`d
  provider tests are explicitly not in this phase's un-ignoring scope (which is the four Commander
  tests only).
- **CI configuration for the `live-api-tests` feature.** Owner: Phase 15, PIPE.
- **Battalion-wide cancellation for Formation, Campaign and ChainOfCommand.** Still no forward owner;
  Phase 3 is explicitly not it.
- **Version, edition and advisory coherence.** Owner: Phase 4, REL-01…REL-05.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|--------------------|
| QUAL-01 | Workspace line coverage at or above ADR-0006's 84% floor, re-measured verbatim at current HEAD | Architecture Patterns diagram 1 (exact command sequence); Environment Availability (toolchain confirmed present/working); Pitfall 3 (object-discovery semantics, do not "fix" the count) |
| QUAL-02 | No first-party source file reports 0% coverage (re-derived set, not the stale named list) | Pattern 4 + Pitfall 1 (`redis.rs` blocker and required refactor); Claude's Discretion notes (`file_storage_port.rs`, `arsenal_port.rs`, `main.rs`, `error.rs` diagnosed concretely) |
| QUAL-03 | Critical-path integration exercisers exist and pass (Paladin execution, Battalion orchestration, tool invocation) | Pattern 2 (how existing `tests/integration/*` files are already wired and passing); Validation Architecture Phase Requirements → Test Map row |
| QUAL-04 | Commander error-path tests (retry/partial-failure/timeout) pass, not `#[ignore]`d; 5 MCP failure modes pass | Pattern 1 (`FaultyPaladinPort` construction from existing mocks); Pattern 2 (wiring, no new `[[test]]` entry); Pattern 3 + Pitfalls 4/5 (MCP fixture-server extension, timeout blocker, auth-heuristic nuance) |
| QUAL-05 | `cargo bench` completes; baseline document records throughput/P50/P95/P99/memory/startup | Code Examples (percentile derivation from `sample.json`, existing bench-command pattern); Don't Hand-Roll (`sample.json` vs custom tooling) |

</phase_requirements>

## Summary

Phase 3 is almost entirely a re-run-and-record phase, and the tooling for every one of its five
deliverables already exists and works in this sandbox: the offline `rustc`/`llvm-profdata`/`llvm-cov`
coverage pipeline, `criterion 0.5.1`, `wiremock 0.6`, and — most importantly — a **working, in-tree,
hermetic rmcp+axum MCP fixture server** that already proves three of the five required MCP
failure-mode assertions in substance. The two genuine construction tasks are: (1) Docker-free unit
tests for `crates/paladin-storage/src/redis.rs`, which requires a small, low-risk **signature
refactor** before it is even possible (its private key/serialization helpers take `&self`, and
`RedisQueueAdapter` cannot be constructed without a live Redis connection — see Pitfall 1), and
(2) a `FaultyPaladinPort` mock for the four empty Commander tests, which can be built almost
verbatim from two mocks that already exist in-crate (`formation_service.rs` and `phalanx_service.rs`
test modules already implement fail-until-attempt and fail-by-name+delay behaviour separately; the
new mock only needs to combine both).

The single most consequential correction this research makes to CONTEXT.md: **new files placed
under `tests/integration/` do NOT need a new `[[test]]` Cargo.toml entry.** `tests/lib.rs` already
declares `pub mod integration;`, which pulls in the whole `tests/integration/mod.rs` module tree
(including every file it `pub mod`s) as part of the auto-discovered `lib` test binary. Adding a
`[[test]]` entry for a file *also* reachable through that `pub mod` chain risks the exact
`clippy::duplicate_mod` failure a code comment in `tests/lib.rs:70-77` documents as the reason a
prior `pub mod cli;` declaration was removed. The correct action for both `commander_error_paths_test.rs`
and any new MCP test file is: create the file under `tests/integration/`, add one `pub mod` line to
`tests/integration/mod.rs`'s existing alphabetical list, and change nothing in Cargo.toml.

**Primary recommendation:** treat this phase as four independent, mostly-mechanical re-verification
tasks gated by one non-negotiable prerequisite (the coverage re-measurement, D-01), and budget real
engineering time only for the `redis.rs` refactor-then-test and the MCP handshake-timeout test (see
Pitfall 4 — the timeout is a hardcoded 30s private constant with no test seam today).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Coverage re-measurement | Build/CI tooling (offline `rustc`/LLVM) | — | Not application code; a toolchain-level measurement against the compiled test binaries. |
| `redis.rs` unit tests | Infrastructure adapter (`paladin-storage`) | — | Tests a queue adapter's pure, connection-independent seams (key formatting, (de)serialization). |
| Commander error-path tests | Application/orchestration (`paladin-battalion`) | Test infrastructure (`tests/helpers/`) | Commander is the orchestration-layer strategy router; the mock lives in shared test infra per D-09/D-10. |
| MCP failure-mode tests | Infrastructure adapter (Arsenal/MCP, `src/infrastructure/adapters/arsenal/`) | Test infrastructure (hermetic fixture server) | `MCPStreamableHttpAdapter`/`MCPClient` are infrastructure-layer adapters over the `rmcp` SDK; the fixture server is test-only infra, not shipped code. |
| Performance baseline | Build/CI tooling (`criterion`, per-crate `benches/`) | Documentation (`docs/src/appendix/`) | Benchmarks live beside the code they measure; the baseline document is documentation, not code. |

## Package Legitimacy Audit

**No new packages are installed by this phase.** Every crate this phase's test-writing needs
(`wiremock`, `criterion`, `mockito`, `testcontainers`, and transitively `rmcp`, `axum`) is **already**
a declared dependency in the root `Cargo.toml` (`[dev-dependencies]`, lines 144-168) — this phase
only exercises `wiremock` and the already-vendored `axum`+`rmcp` server-transport features for the
first time in anger. Checked against the package-legitimacy seam anyway, for completeness:

| Package | Registry | Age | Downloads/wk | Source Repo | Verdict | Disposition |
|---------|----------|-----|---------------|--------------|---------|-------------|
| `wiremock` | crates.io | ~6 yrs (2020-04) | 1,181,196 | github.com/LukeMathWalker/wiremock-rs | OK | Already a dependency; no action |
| `criterion` | crates.io | ~9 yrs (2017-12) | 4,089,059 | github.com/criterion-rs/criterion.rs | OK | Already a dependency; no action |
| `mockito` | crates.io | ~10 yrs (2016-01) | 812,529 | github.com/lipanski/mockito | OK | Already a dependency; no action |
| `testcontainers` | crates.io | ~8 yrs (2018-08) | 878,116 | github.com/testcontainers/testcontainers-rs | OK | Already a dependency; no action |
| `rmcp` | crates.io | ~1.4 yrs (2025-03) | 716,063 | github.com/modelcontextprotocol/rust-sdk | OK | Already a dependency (pinned `=2.1.0`); no action |

**Packages removed due to SLOP verdict:** none.
**Packages flagged as suspicious (SUS):** none.

## Standard Stack

There is no new library adoption in this phase. The "stack" is the set of already-shipped
dev-dependencies this phase must use correctly:

### Core (already present, verified via `cargo view`/registry)

| Library | Version (Cargo.toml pin) | Purpose in this phase | Provenance |
|---------|---------------------------|------------------------|------------|
| offline `rustc`/`llvm-profdata`/`llvm-cov` | toolchain-bundled (rustc 1.97.1, LLVM 22.1.6) | Coverage re-measurement (D-01/D-02) | `[VERIFIED: shipped toolchain — rustc -vV, ls $(rustc --print sysroot)/lib/rustlib/.../bin]` |
| `criterion` | `= 0.5` (root `Cargo.toml:151`, feature `async_tokio`) | `cargo bench` execution + percentile derivation | `[VERIFIED: shipped Cargo.toml + registry check]` |
| `wiremock` | `= 0.6` (root `Cargo.toml:155`) | MCP handshake-timeout / malformed-response tests only (see Architecture Patterns) | `[VERIFIED: shipped Cargo.toml + registry check; zero existing usages — `grep -rln wiremock tests src crates` returns nothing]` |
| `rmcp` (server transport features) | `= 2.1.0`, `features = ["server", "transport-streamable-http-server"]` (root `Cargo.toml:164-167`, dev-only) | MCP token-rejection / unknown-tool / bad-arguments tests, via the existing hermetic fixture-server pattern | `[VERIFIED: shipped Cargo.toml, comment explicitly documents this exact use case]` |
| `axum` | `0.8.4` (root `Cargo.toml:168`, dev-only) | Hosts the hermetic MCP fixture server | `[VERIFIED: shipped Cargo.toml]` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extending the existing rmcp+axum fixture server (token-rejection/unknown-tool/bad-arguments) | `wiremock`-only stubs for all 5 failure modes | Rejected as primary approach — `wiremock` would have to hand-replicate rmcp's exact JSON-RPC `initialize -> notifications/initialized` handshake wire format to reach the post-handshake failure modes (unknown tool, bad arguments), which the real rmcp server produces for free and correctly by construction. Keep `wiremock` for the two failure modes a *spec-compliant* server cannot produce (malformed response, handshake timeout) — see Architecture Patterns. |
| Refactoring `redis.rs`'s private key/serialize helpers off `&self` | Standing up Redis via `testcontainers` | Rejected per D-05/CONTEXT.md — Docker is absent from this sandbox (`command -v docker` → exit 1, verified). The refactor is the only path to Docker-free coverage of these seams. |
| `cargo bench`'s raw `sample.json` for percentiles | `criterion`'s own summary stats (mean/median/MAD) | Rejected per D-14 — criterion does not compute P95/P99 itself; the raw per-iteration samples must be read and derived (see Code Examples). |

**Installation:** none required — nothing above is a net-new dependency.

**Version verification:** confirmed live in this sandbox:
```
$ rustc -vV
rustc 1.97.1 (8bab26f4f 2026-07-14), LLVM version: 22.1.6
$ cargo --version
cargo 1.97.1 (c980f4866 2026-06-30)
```
identical to `01-coverage-measurement.md`'s recorded toolchain — no drift since Phase 1.

## Architecture Patterns

### System Architecture Diagram — coverage pipeline (D-01/D-02)

```
cargo test --workspace --offline           (RUSTFLAGS="-C instrument-coverage",
        │                                    LLVM_PROFILE_FILE=.../paladin-%p-%m.profraw)
        ▼
 N .profraw files under target/coverage/
        │
        ▼  llvm-profdata merge -sparse target/coverage/*.profraw -o paladin.profdata
 paladin.profdata
        │
        ▼  cargo test --workspace --no-run --message-format=json --offline
        │     | jq -r 'select(.profile.test==true) | .filenames[]' | sort -u
 31 test-binary object paths (one per compiled test TARGET, not per source file —
 see Pitfall 3)
        │
        ▼  llvm-cov report --instr-profile=paladin.profdata
        │     --ignore-filename-regex='(^|/)(examples|benches)/|crates/doc-examples/|registry/src/|rustlib/src/'
        │     --object=<obj1> --object=<obj2> ... --object=<obj31>
 TOTAL row: Lines block "61404  9340  84.79%" (Phase 1's recorded figure — to be reproduced,
 not retyped, at current HEAD)
```

### System Architecture Diagram — MCP failure-mode tests (D-11)

```
 Test fn                          Transport under test              What produces the failure
 ─────────────────────────────────────────────────────────────────────────────────────────────
 token rejection (missing/wrong)  MCPClient::connect_streamable_http  Real hermetic rmcp+axum
                                   → rmcp initialize handshake         fixture server + axum
                                                                       auth middleware (401 +
                                                                       WWW-Authenticate) — ALREADY
                                                                       SHIPPED, extend don't rebuild
 unknown tool                     MCPClient::invoke_tool               Real fixture server's
                                   → rmcp tools/call                   FixtureServer::call_tool
                                                                       already returns
                                                                       McpError::invalid_params for
                                                                       any name != "echo" — add an
                                                                       assertion, minimal new code
 bad arguments                    MCPClient::invoke_tool               Extend FixtureServer's
                                   → rmcp tools/call                   call_tool to validate the
                                                                       "message" arg is present
                                                                       (currently unwrap_or_default)
 malformed response                MCPClient::discover_tools /         A spec-VIOLATING fake
                                   invoke_tool → rmcp tools/list        server — a real rmcp server
                                   or tools/call                       cannot produce this by
                                                                       construction. Needs either a
                                                                       raw axum handler that returns
                                                                       truncated JSON on a specific
                                                                       route, or a wiremock stub
                                                                       that first completes a
                                                                       hand-crafted valid `initialize`
                                                                       response, then returns
                                                                       garbage for the next POST.
 handshake timeout                 MCPClient::connect_streamable_http  A server (axum handler or
                                   → wrapped in                        wiremock delay) that never
                                   STREAMABLE_HTTP_HANDSHAKE_TIMEOUT    responds. BLOCKER: the
                                   (private const = 30s,                30s timeout is a hardcoded
                                   mcp_protocol.rs:50)                  private const with no test
                                                                       seam — see Pitfall 4.
```

### Recommended Project Structure (files touched, not new directories)

```
crates/paladin-storage/src/redis.rs        # gains #[cfg(test)] mod tests (after refactor, Pitfall 1)
tests/helpers/mock_paladin_port.rs         # gains FaultyPaladinPort (D-09/D-10)
tests/helpers/mod.rs                       # gains `pub use mock_paladin_port::FaultyPaladinPort;`
tests/integration/mod.rs                   # gains ONE `pub mod commander_error_paths_test;` line
tests/integration/commander_error_paths_test.rs   # NEW — the 4 relocated tests
tests/integration/mcp_streamable_http_test.rs     # EXTEND — token-rejection/unknown-tool/bad-args
                                                   # tests added here (fixture already lives here)
tests/integration/mcp_streamable_http_malformed_test.rs  # NEW (optional, or extend same file) —
                                                   # malformed-response + handshake-timeout, using
                                                   # a raw non-rmcp axum handler
docs/src/appendix/performance-baseline.md  # amended in place (D-15), new dated section
```

### Pattern 1: Combine the two existing fail-capable mocks into `FaultyPaladinPort`

**What:** `crates/paladin-battalion/src/formation_service.rs`'s test-module `MockPaladinPort`
already implements `fail_until_attempt: Option<usize>` (an atomic call counter + "fail until the
Nth attempt, then succeed" — exactly the retry-count assertion ROADMAP criterion 3 needs) and
`should_fail: bool` (always-fail). `crates/paladin-battalion/src/phalanx_service.rs`'s test-module
`MockPaladinPort` implements `fail_paladin_names: Vec<String>` (fail specific Paladins by name —
the partial-failure/Nth-Paladin case) plus a configurable `delay_ms` (the timeout-stops-siblings
case). Neither file's mock alone covers all four cases D-09 lists; `FaultyPaladinPort` is their
union, built once in `tests/helpers/mock_paladin_port.rs`.

**When to use:** for all four relocated Commander tests (`commander_error_paths_test.rs`) and any
future Battalion-pattern error-path test.

**Example (verified against the actual shipped mocks, not paraphrased):**
```rust
// Source: crates/paladin-battalion/src/formation_service.rs:395-410 (fail_until_attempt pattern)
//         crates/paladin-battalion/src/phalanx_service.rs:490-540 (fail_paladin_names + delay_ms)
// tests/helpers/mock_paladin_port.rs — FaultyPaladinPort, combining both:
pub struct FaultyPaladinPort {
    call_count: Arc<Mutex<usize>>,
    fail_always: bool,
    fail_paladin_names: Arc<Mutex<Vec<String>>>,
    fail_until_attempt: Option<usize>,   // per-name attempt counters if per-Paladin retry needed
    delay_ms: u64,
}
// PaladinError here is paladin_ports::output::paladin_port's PaladinError — confirmed identical
// to what commander.rs imports (paladin_core::platform::container::paladin_error::PaladinError,
// which src/core/platform/mod.rs re-exports as `pub use paladin_core::platform::container::paladin_error;`
// — same type, not a lookalike).
```

**Must be `Send + Sync`** (per D-09) — both source mocks already satisfy this trivially since they
use `Arc<Mutex<_>>`/`Arc<Mutex<Vec<_>>>` for interior mutability, no `Rc`/`RefCell` anywhere.

### Pattern 2: `tests/integration/mod.rs` wiring — no `[[test]]` entry needed

**What:** `tests/lib.rs:60` declares `pub mod integration;`, and `tests/lib.rs` is itself an
**auto-discovered** Cargo test target (name `lib`, confirmed via
`cargo test --workspace --offline --no-run --message-format=json`). Every file `tests/integration/mod.rs`
declares via `pub mod X;` therefore compiles as part of the single `lib` test binary — **not** as
its own separate binary, and **not** requiring its own `[[test]]` Cargo.toml entry. This is *why*
`commander_integration_tests.rs`, `mcp_streamable_http_test.rs`, `battalion_campaign_integration_test.rs`,
etc. all measured real, non-zero coverage in `01-coverage-measurement.md` despite having no
`[[test]]` entry of their own — a fact this research confirmed by literally running
`cargo test --workspace --offline --no-run --message-format=json | jq` and finding only 31 distinct
compiled test-binary TARGETS (matching Phase 1's "31 discovered test-binary objects"), most of which
are these `pub mod`-nested files bundled inside `lib`/`unit`.

**When to use:** every time this phase adds a new file under `tests/integration/`.

**CORRECTION to CONTEXT.md canonical_refs**, which states "Cargo.toml `[[test]]` targets — gains
`commander_error_paths` and any new MCP failure-mode target." This is unnecessary and risks a real
build failure: `tests/lib.rs:70-77` carries a code comment explaining that a previous `pub mod cli;`
declaration was **removed as dead code** specifically because compiling the same module tree twice
(once via its own `[[test]]` binary, once via `lib`'s `pub mod`) tripped `clippy::duplicate_mod`
under `-D warnings` — this workspace's own CI-equivalent lint gate. Do not add a `[[test]]` entry for
`commander_error_paths_test.rs`; only add the `pub mod commander_error_paths_test;` line to
`tests/integration/mod.rs`'s existing alphabetically-sorted list.

**Example:**
```rust
// tests/integration/mod.rs — insert alphabetically next to the existing entries:
pub mod commander_error_paths_test;
```
That is the entire wiring change required. No Cargo.toml edit.

### Pattern 3: Extend the existing hermetic MCP fixture server, don't build a parallel wiremock harness

**What:** `tests/integration/mcp_streamable_http_test.rs` already stands up a REAL, in-process,
spec-strict rmcp Streamable-HTTP server (`FixtureServer` + `spawn_fixture_server()`) behind an axum
bearer-token middleware, and already has two passing negative tests:
`streamable_http_round_trip_rejects_missing_bearer_token` and
`streamable_http_round_trip_rejects_incorrect_bearer_token`, both asserting
`Err(ArsenalError::AuthFailed(_))`. This is a stronger, more realistic proof than a `wiremock` stub
(it exercises the actual `initialize -> notifications/initialized` handshake, session semantics, and
JSON-RPC framing `rmcp::transport::streamable_http_server::StreamableHttpService` implements) and it
is already the pattern the shipped Cargo.toml dev-dependency comment (lines 158-167) explicitly
names for this exact purpose.

**When to use:** for the "expired/rejected token", "unknown tool", and "bad arguments" failure
modes — all three are reachable by extending `FixtureServer` rather than introducing `wiremock`.

**Example — extending `call_tool` for the "bad arguments" mode (verified against the shipped fixture,
`tests/integration/mcp_streamable_http_test.rs:73-93`):**
```rust
// Source: tests/integration/mcp_streamable_http_test.rs (shipped, extend in place)
async fn call_tool(
    &self,
    request: CallToolRequestParams,
    _context: RequestContext<RoleServer>,
) -> Result<CallToolResult, McpError> {
    if request.name != ECHO_TOOL_NAME {
        return Err(McpError::invalid_params(
            format!("unknown tool `{}`", request.name),
            None,
        ));                                    // <-- ALREADY covers "unknown tool"; add a test
    }
    let message = request
        .arguments
        .as_ref()
        .and_then(|args| args.get("message"))
        .and_then(|v| v.as_str());
    let Some(message) = message else {
        return Err(McpError::invalid_params("missing required `message` argument", None));
        // ^ ADD THIS for "bad arguments" — currently `.unwrap_or_default()` silently accepts
        // missing/wrong-typed args, which is itself worth a one-line source fix, not just a test.
    };
    Ok(CallToolResult::success(vec![ContentBlock::text(format!("echo: {message}"))]))
}
```

**Where `wiremock` (or a hand-rolled non-rmcp axum route) IS the right tool:** "malformed response"
and "handshake timeout" cannot be produced by a spec-compliant rmcp server by construction. Use a
second, deliberately-non-compliant HTTP endpoint for these two — either a raw axum route returning
truncated JSON / no response at all, or a `wiremock::MockServer` stub. Given zero prior `wiremock`
usage exists anywhere in this repo (`grep -rln wiremock tests src crates` → no matches) and the
in-tree axum precedent is proven working, **prefer a second small axum handler in the same style as
`spawn_fixture_server()`** for consistency, falling back to `wiremock` only if a raw axum handler
proves awkward for the "malformed response after a successful handshake" case (which needs the
handler to correctly answer `initialize`/`notifications/initialized` with valid JSON-RPC before
returning garbage for `tools/list`/`tools/call` — either tool can express this; axum keeps the
pattern uniform with the rest of the file).

### Pattern 4: `redis.rs`'s testable seams — after the required refactor

**What:** `RedisQueueAdapter::new()` calls `redis::aio::ConnectionManager::new(client)`, which per
the vendored `redis-0.32.7` source (`src/aio/connection_manager.rs:301-304`, doc comment: "Connect to
the server and store the connection... wait for it to be established") **eagerly connects and
retries** — it cannot succeed without a live Redis server. This means `RedisQueueAdapter`'s private,
`&self`-taking helper methods (`queue_key`, `priority_queue_key`, `queue_meta_key`, `processing_key`,
`completed_key`, `failed_key`, `serialize_item`, `deserialize_item` — `redis.rs:94-180`) **cannot be
unit-tested by constructing a real adapter instance in this Docker-less sandbox**, even though their
bodies only touch `self.config.key_prefix` (or nothing at all — `serialize_item`/`deserialize_item`
never reference `self`). See Pitfall 1 for the required minimal fix.

**Already directly testable, no refactor needed:** `RedisQueueAdapter::get_priority_levels()`
(`redis.rs:193-200`) is declared `fn get_priority_levels() -> Vec<MessagePriority>` — no `&self` at
all — callable today as `RedisQueueAdapter::get_priority_levels()` without any instance.

**When to use:** write the `#[cfg(test)] mod tests` block only after the refactor in Pitfall 1 lands;
until then, only `get_priority_levels()` is testable.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| MCP handshake simulation for failure-mode tests | A hand-rolled JSON-RPC client/server pair | The existing rmcp+axum `FixtureServer` pattern in `tests/integration/mcp_streamable_http_test.rs` | rmcp's own upstream test suite (`rmcp-2.1.0/src/transport/common/reqwest/streamable_http_client.rs:369+`) uses the identical axum-server-in-a-`tokio::spawn` pattern to test its own client against its own server — this workspace already copied that pattern once; copy it again rather than inventing a third approach. |
| P95/P99 latency derivation | A custom criterion plugin or `--output-format=bencher` scraping | Direct read of `target/criterion/<bench>/<function>/new/sample.json` (`SavedSample { iters: Vec<f64>, times: Vec<f64> }`, per-iteration time = `times[i] / iters[i]`) | This is exactly what criterion itself writes and what its own HTML report reads (`criterion-0.5.1/src/analysis/mod.rs:68-69,164`); no plugin exists for percentiles because none is needed — the raw data is already on disk. |
| Retry-count / partial-failure mock | A new `mockall`-generated or hand-rolled-from-scratch `PaladinPort` impl | Union of the two ALREADY-SHIPPED in-crate test mocks (`formation_service.rs`'s attempt-counter + `phalanx_service.rs`'s per-name-fail/delay) | Both capabilities already exist, separately, in the same crate `commander.rs` belongs to; recombining is strictly less code and stays consistent with the crate's own established test-mock idiom. |

**Key insight:** every "don't hand-roll" item in this phase resolves to "an equivalent already ships
in this repo — extend it." That is itself evidence for CONTEXT.md's framing that this phase is
measurement, not construction: the construction happened in earlier phases and left reusable seams.

## Runtime State Inventory

**N/A — this is not a rename/refactor/migration phase.** No renamed identifiers, no data-store keys,
no OS-registered state, no stale secret names are in scope. Verified by re-reading CONTEXT.md's
`<domain>` section: the phase adds tests and re-runs measurement commands; it does not rename or move
existing runtime-addressable state. Skipping this section per the trigger condition.

## Common Pitfalls

### Pitfall 1: `RedisQueueAdapter`'s private key/serialize helpers cannot be called without a live connection

**What goes wrong:** A naive `#[cfg(test)] mod tests` that tries `RedisQueueAdapter::new(config, None).await.unwrap()`
to get an instance for testing `queue_key()`/`serialize_item()` etc. will hang or fail — there is no
Redis server in this sandbox, and `ConnectionManager::new()` retries with exponential backoff before
giving up (per `ConnectionManagerConfig::DEFAULT_NUMBER_OF_CONNECTION_RETRIES = 6`, vendored source).

**Why it happens:** These eight private methods (`redis.rs:94-180`) are declared as `&self` instance
methods purely by convention/proximity, but their bodies use only `self.config.key_prefix` (the key
builders) or nothing at all (`serialize_item`/`deserialize_item` — verified by reading every line of
both bodies; neither references `self`).

**How to avoid:** before writing `redis.rs`'s test module, make a small, low-risk signature change:
convert these eight methods from `&self`-taking instance methods to either (a) free functions/
associated functions taking `&RedisQueueConfig` (or a bare `&str` key_prefix) instead of `&self`, or
(b) keep them as inherent methods on `RedisQueueConfig` rather than `RedisQueueAdapter`. Either
change is private-surface-only (none of these eight methods is `pub`), touches no public API, and
changes no runtime behaviour — every call site inside `RedisQueueAdapter`'s trait impls
(`self.queue_key(name)` → `queue_key(&self.config, name)` or similar) is a mechanical one-line edit.
`serialize_item`/`deserialize_item` can drop `&self` entirely and become plain functions/associated
functions with no config dependency at all.

**Warning signs:** if the plan's task list has "write `#[cfg(test)] mod tests` in `redis.rs`" as a
single, unqualified task with no preceding refactor task, it will stall on `cargo test` timing out or
erroring on connection — split it into two tasks (refactor helpers off `&self`; then add tests).

### Pitfall 2: `tests/lib.rs`'s `pub mod cli;` removal is a documented cautionary precedent for `[[test]]` duplication

**What goes wrong:** Adding both a Cargo.toml `[[test]]` entry AND a `pub mod` declaration in
`tests/integration/mod.rs` for the same file compiles that file's module tree twice inside two
different test binaries. If either binary also (transitively) loads `tests/helpers/mod.rs` a second
time relative to the other, `clippy::duplicate_mod` fails the build under `-D warnings` — exactly
what happened previously with `tests/cli/` (documented in `tests/lib.rs:70-77`) and was fixed by
deleting the redundant `pub mod cli;`, not by adding a `[[test]]` entry.

**Why it happens:** Cargo compiles each `[[test]]` `path` as an independent crate root; if that root
(directly or via a `pub mod` chain) also appears under another already-compiled root, both binaries
statically include the same source, and `clippy`'s workspace-wide lint pass sees it twice.

**How to avoid:** see Architecture Patterns, Pattern 2 — new files under `tests/integration/` get a
`pub mod` line in `tests/integration/mod.rs` only. Never both.

**Warning signs:** `cargo clippy -- -D warnings` failing with `duplicate_mod` naming a test file that
was "just added."

### Pitfall 3: `--object` discovery is per compiled TEST BINARY, not per source file

**What goes wrong:** Assuming each `tests/integration/*.rs` file needs (or produces) its own
`--object` argument in the `llvm-cov report` command, or assuming a missing per-file `[[test]]`
target means the file's coverage is silently dropped from the TOTAL row.

**Why it happens:** `cargo test --workspace --no-run --message-format=json | jq -r 'select(.profile.test==true) | .filenames[]'`
returns one path per **compiled test artifact** (a linked binary), not one per source file. This
research directly ran that discovery command and got exactly 31 distinct object paths — matching
Phase 1's recorded "31 discovered test-binary objects" — even though there are ~35+ individual
`tests/integration/*.rs` files, because most of them are bundled into the single `lib` binary
(`tests/lib.rs`) via the `pub mod integration;` chain (see Pattern 2). The command as written in
ADR-0006/`01-coverage-measurement.md` is correct as-is; do not "fix" the object count to match the
file count.

**How to avoid:** re-run the discovery command verbatim (D-02) and record whatever count it produces
(expect ~31, may differ by ±1-2 if a new `[[test]]` target was added/removed between commits, or if
this phase deliberately avoids adding one per Pattern 2 — which it should, keeping the count stable).

**Warning signs:** a re-measurement plan step that says "one `--object` per test file" or that
manually enumerates test files instead of running the `jq` discovery command.

### Pitfall 4: The MCP Streamable-HTTP handshake timeout is a hardcoded, non-configurable 30-second private constant

**What goes wrong:** A literal "handshake timeout" test — a fixture server that never responds,
asserting `MCPClient::connect_streamable_http(...)` eventually returns `Err(ArsenalError::Timeout(_))`
— will take **at least 30 real seconds** to complete, because
`STREAMABLE_HTTP_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30)` (`mcp_protocol.rs:50`) is a
private module-level constant with no parameter, config field, or feature flag to shorten it for
tests. Tokio's `#[tokio::test(start_paused = true)]` virtual-time trick does **not** help here: the
timeout wraps a REAL network round-trip over a REAL TCP socket to a REAL (if in-process) axum
listener, and paused-time only intercepts pure in-process timers, not actual socket I/O completion
latency, which will not be sped up.

**Why it happens:** the constant was added under T-12.1-06 as a defensive bound, with no
test-injection seam considered at the time (its own doc comment only discusses the production
behaviour it prevents — an indefinitely hung caller).

**How to avoid — this is a genuine, previously-unflagged decision point for the planner, not just a
test-writing task:** three options, in order of preference:
1. **(Recommended) Add a `connect_streamable_http_with_timeout` variant** (or thread an optional
   `Duration` parameter through `connect_streamable_http`/`MCPStreamableHttpAdapter::connect`),
   defaulting to the existing 30s constant, so tests can pass e.g. `Duration::from_millis(200)`. This
   is a minimal, additive, non-breaking signature change (or an additive method) — the same class of
   low-risk private-surface change as Pitfall 1's refactor, just on a slightly more public boundary
   (the constant is private but `connect_streamable_http` is a documented `pub async fn`).
2. Accept the real 30+ second cost and mark the test `#[ignore]` with a reason, or exclude it from
   the "quick run" sampling tier and only run it at phase-gate time (see Validation Architecture
   below) — defensible but directly works against this phase's own "measurement, not aspiration"
   ethos if it becomes a `deferred with reason` row instead of a passing test.
3. Do not attempt this specific failure mode as a Streamable-HTTP connect-timeout test at all, and
   instead test timeout behaviour at a lower layer that already has a configurable timeout (none was
   found in this research at the `MCPClient`/`MCPStreamableHttpAdapter` layer — this option is listed
   for completeness but is not recommended; QUAL-04/ROADMAP criterion 4 names "handshake timeout"
   explicitly).

**Warning signs:** a plan task estimating this test at the same size as the other four MCP
failure-mode tests, or a CI/test-suite runtime budget that doesn't account for one 30-second test.

### Pitfall 5: 401/403 auth-failure classification is a string heuristic, not a typed error match, at the connect layer

**What goes wrong:** Assuming `MCPClient::connect_streamable_http`'s `Err(ArsenalError::AuthFailed(_))`
requires the mocked/fixture server to send a `WWW-Authenticate` header, and building a more elaborate
fixture than necessary.

**Why it happens:** `map_streamable_http_connect_error` (`mcp_protocol.rs:295-302`) does NOT switch on
rmcp's typed `StreamableHttpError::AuthRequired`/`InsufficientScope` variants at all — those are
`post_message`-level types used for **post-handshake** calls (`invoke_tool`/`discover_tools`, mapped
by the separate `map_service_error`, which has **no explicit auth branch** — an auth failure there
falls into `other => ArsenalError::ProtocolError(...)`, not `AuthFailed`). At the **connect/handshake**
layer, the classification is `looks_like_auth_failure(&msg)` (`mcp_protocol.rs:308-316`), a
lowercase substring match on the `.to_string()` of rmcp's `ClientInitializeError` for `"401"`,
`"403"`, `"unauthorized"`, `"forbidden"`, `"auth"`, or `"api key"`. Verified against
`redis-0.32.7`'s... — no, against `rmcp-0.32.7`'s (correction: `rmcp-2.1.0`'s) own `post_message`
(`streamable_http_client.rs:115-235`): a bare 401/403 with no `WWW-Authenticate` header still falls
through to the generic `UnexpectedServerResponse(Cow::Owned(format!("HTTP {status}: {body}")))` path
— and that formatted string still contains the literal substring `"401"`/`"403"`, so
`looks_like_auth_failure` still classifies it correctly via the fallback path. **Net effect: a plain
401/403 response, with or without `WWW-Authenticate`, is sufficient** — the shipped fixture server
happens to also send `WWW-Authenticate` (for realism, matching `rmcp`'s typed path), but a test does
not need to replicate that to exercise `AuthFailed` classification.

**How to avoid:** for a NEW, minimal fixture that only needs to prove `AuthFailed` classification, a
bare axum handler returning `(StatusCode::UNAUTHORIZED, "")` is sufficient. For proving the adapter
*sends* the correct `Authorization: Bearer <token>` header (a stronger, more valuable assertion,
already what the shipped fixture does), keep the `WWW-Authenticate`-emitting middleware pattern.

**Warning signs:** none observed yet in this codebase — this is a forward-looking note so the
planner does not over-build the token-rejection fixture.

## Code Examples

### Deriving P50/P95/P99 from criterion's raw samples

```rust
// Source: schema read directly from vendored criterion-0.5.1 source
// (src/lib.rs:1502-1505 `struct SavedSample { iters: Vec<f64>, times: Vec<f64> }`,
// written to target/criterion/<bench_id>/new/sample.json by
// src/analysis/mod.rs:164-178). NOT documented in criterion's public API — this is
// its internal on-disk format, stable since criterion 0.3.x.
//
// sample.json shape:
// { "iters": [f64, f64, ...], "times": [f64, f64, ...] }
// times[i] is the TOTAL measured duration (nanoseconds) for iters[i] iterations of
// that sample batch — NOT a per-iteration time directly.

fn per_iteration_times_ns(sample: &SavedSample) -> Vec<f64> {
    sample.iters.iter().zip(sample.times.iter())
        .map(|(iters, total_ns)| total_ns / iters)
        .collect()
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx]
}
// Usage: read target/criterion/<group>/<function>/new/sample.json, call
// per_iteration_times_ns, sort, then percentile(&sorted, 0.50 | 0.95 | 0.99).
// Document this derivation verbatim in the baseline doc per D-14 — do not just
// paste the numbers.
```

### Existing bench-run command pattern (from the current, superseded baseline doc)

```bash
# Source: docs/src/appendix/performance-baseline.md (2026-05-27 run, to be superseded
# by a new dated section per D-15) — the config bench needs APP_ENV=test to load a
# schema-compatible profile; this is easy to miss and silently changes what's measured
# if omitted.
APP_ENV=test cargo bench --bench config_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-battalion --bench battalion_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-memory --bench sanctum_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-memory --bench garrison_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-llm --bench llm_serialization_benchmarks -- --noplot
```
`cargo check -p paladin-battalion --bench battalion_benchmarks --offline` was run in this research
session and completed successfully in 75s against the local registry (`criterion-0.5.1` compiled with
no network access) — confirms D-12's feasibility claim without needing the slower full `cargo bench`
run during research.

### `FaultyPaladinPort` construction sketch, `Commander` side (verified trait/type surface)

```rust
// Source: crates/paladin-battalion/src/commander.rs:23-24 (imports),
// crates/paladin-ports/src/output/paladin_port.rs:631 (trait PaladinPort: Send + Sync)
use paladin_ports::output::paladin_port::PaladinPort; // same trait Commander is generic over
// tests/integration/commander_error_paths_test.rs:
use paladin::application::services::battalion::commander::CommanderBuilder; // facade re-export,
    // same import path tests/integration/commander_integration_tests.rs already uses successfully
use paladin::core::platform::container::battalion::{BattalionConfig, ErrorStrategy, RetryPolicy};

// FailFast: first failure stops sibling execution.
let port = Arc::new(FaultyPaladinPort::new().fail_paladin("Paladin-2"));
let config = BattalionConfig::new("t").with_error_strategy(ErrorStrategy::FailFast);
// ... construct paladins, CommanderBuilder::new(port).strategy(...).paladins(...).config(config)
//     .build().unwrap().execute("input").await — assert Err + that Paladin-3 never ran
//     (FaultyPaladinPort must expose an execution_log like IntegrationMockPaladinPort does,
//     tests/integration/commander_integration_tests.rs:27-28,64-70, to assert "did not run").
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| Hand-rolled MCP JSON-RPC engine (`MCPMessage`/`MCPRequest`/`MCPTransport`) | Thin `MCPClient` facade over `rmcp::service::RunningService`, `rmcp::ServiceExt::serve()` performs the ENTIRE handshake | "Phase 12.1 D-01/D-04" (per `mcp_protocol.rs`'s own module doc, predates this GSD milestone's Phase numbering) | The hand-rolled engine is fully retired; do not write tests against the old `MCPTransport` trait or `MCPMessage` types — they no longer exist. All 5 D-11 failure-mode tests must target `MCPClient`/`MCPStreamableHttpAdapter`, confirmed still the correct, current names. |
| Plain unauthenticated HTTP POST "SSE" adapter (`MCPSseAdapter`) | `MCPStreamableHttpAdapter`, real Streamable-HTTP via rmcp | Same Phase 12.1 window, per `mcp_streamable_http_adapter.rs`'s own doc comment ("the honestly-named replacement for the retired MCPSseAdapter") | Confirms CONTEXT.md D-11's instruction to target the Streamable-HTTP surface, not SSE, is still correct — `MCPSseAdapter` no longer exists in the tree (grep confirms zero matches). |

**Deprecated/outdated:** `cargo tarpaulin` as a coverage tool-of-record (`codebase/TESTING.md:319-322`)
— superseded by the offline `rustc`/LLVM pipeline per ADR-0006; do not compare a tarpaulin figure
against the 84% floor if one is ever produced.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The recommended `redis.rs` refactor (moving key/serialize helpers off `&self`) is the *only* practical way to unit-test those seams without Docker | Pitfall 1 | LOW — directly verified via the vendored `redis-0.32.7` source showing `ConnectionManager::new` blocks on a real connection; no alternative construction path exists in the crate today. Confidence: HIGH, not really an assumption, but flagged since it prescribes a source-code change beyond pure test-writing. |
| A2 | The planner will choose Pattern 3 (extend the existing rmcp+axum fixture) over building a parallel `wiremock`-only harness for token-rejection/unknown-tool/bad-arguments | Architecture Patterns, Pattern 3 | MEDIUM — this is a recommendation, not a locked constraint; a `wiremock`-only approach is technically possible but was assessed as more work and less consistent with existing precedent. If the planner disagrees, the two pre-handshake modes (token, timeout) are the safer `wiremock` candidates regardless. |
| A3 | Adding a timeout parameter/variant to `connect_streamable_http` (Pitfall 4, option 1) is an acceptable minimal source change for a "measurement phase" | Pitfall 4 | MEDIUM — CONTEXT.md's phase framing emphasizes measurement over construction; this is a small but real production-code change. If rejected, the handshake-timeout test either costs 30+ real seconds or becomes a `deferred with reason` ledger row — the planner/user should explicitly choose, since CONTEXT.md did not anticipate this decision point at all. |

## Open Questions

1. **Should the handshake-timeout test's 30-second cost (Pitfall 4) be paid, engineered around, or
   deferred?**
   - What we know: the constant is hardcoded and private; no existing seam shortens it; Tokio's
     paused-time does not help because the I/O is real.
   - What's unclear: whether adding a configurable-timeout parameter is within this phase's
     "measurement, not construction" charter, or whether the user would rather accept a 30s test or
     defer this one failure mode.
   - Recommendation: raise explicitly at plan-review or as a `checkpoint` in the plan; do not let the
     executor silently pick one of the three options in Pitfall 4 without recording which and why.

2. **Malformed-response fixture: hand-rolled axum route, or `wiremock`?**
   - What we know: a real rmcp server cannot produce this by construction; both tools can express a
     "valid handshake, then garbage" sequence.
   - What's unclear: which is less code / more maintainable given zero prior `wiremock` usage exists
     to compare against.
   - Recommendation: try the axum route first (keeps `tests/integration/mcp_streamable_http_test.rs`
     internally consistent); fall back to `wiremock` only if matching the exact `initialize` JSON-RPC
     response shape by hand proves fragile (fields required: `jsonrpc`, `id`, `result.protocolVersion`,
     `result.capabilities`, `result.serverInfo` — cross-check against `rmcp::model::InitializeResult`'s
     `Serialize` impl before hand-writing this JSON).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|--------------|-----------|---------|----------|
| `docker` | Redis-backed `redis.rs` full-path tests, MinIO tests | ✗ (`command -v docker` exit 1) | — | Docker-free unit tests on pure seams only (D-05); live-server paths stay `deferred with reason`, owner Phase 15 PIPE |
| `llvm-profdata`/`llvm-cov` | Coverage re-measurement | ✓ | bundled with rustc 1.97.1 toolchain (`llvm-tools` component), resolved at `/usr/local/rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/` | — |
| `cargo-llvm-cov` (the cargo subcommand) | Not used by this phase (ADR-0006 explicitly avoids it) | ✗ (`error: no such command`) | — | N/A — not needed; the direct `rustc`/`llvm-profdata`/`llvm-cov` pipeline is used instead |
| `jq` | Object-file discovery (`--object` list) | ✓ | jq-1.6 at `/usr/bin/jq` | — |
| crates.io network access | Installing any NEW crate | ✗ (HTTP 403, per environment facts) | — | Not needed — every crate this phase touches is already vendored in the local registry, confirmed present under `/usr/local/cargo/registry/src/index.crates.io-*/` for `criterion-0.5.1`, `redis-0.32.7`, `rmcp-2.1.0` |

**Missing dependencies with no fallback:** none block this phase.
**Missing dependencies with fallback:** `docker` — fallback already designed into D-05/D-07 (deferred rows, not blocking).

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in) for unit/integration; `criterion 0.5.1` for benchmarks |
| Config file | none dedicated — driven by `[[test]]`/`[[bench]]` entries in `Cargo.toml` and the `tests/integration/mod.rs`/`tests/unit/mod.rs` barrels |
| Quick run command | `cargo test -p paladin-battalion --offline` (Commander tests) / `cargo test -p paladin-storage --offline` (`redis.rs`) / `cargo test --offline --test lib -- mcp` (MCP tests, once wired via `pub mod`) |
| Full suite command | `RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" cargo test --workspace --offline` (the ADR-0006 command, doubles as the coverage-measurement run) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|---------------------|--------------|
| QUAL-01 | Workspace line coverage ≥ 84% at current HEAD | measurement (not a pass/fail unit test) | full ADR-0006 pipeline, see Architecture Patterns diagram 1 | ✅ pipeline exists (plan 01-09); re-run needed |
| QUAL-02 | No first-party 0% file (re-derived set: `redis.rs`, `paladin-server.rs`, `file_storage_port.rs`, `error.rs`, `arsenal_port.rs`) | unit | `cargo test -p paladin-storage redis::tests --offline` (after Pitfall 1 refactor) + per-file equivalents | ❌ Wave 0 — none of these test modules exist yet |
| QUAL-03 | Critical-path integration exercisers exist and pass (Paladin execution, Battalion orchestration, tool invocation) | integration | `cargo test --offline --test lib` (bundles `tests/integration/*`) | ✅ largely exists — `commander_integration_tests.rs`, `battalion/load_test.rs`, `arsenal_execution_integration_test.rs` already pass; this req is mostly a naming/citation task per D-08 |
| QUAL-04 | Commander retry/partial-failure/timeout tests pass, not `#[ignore]`d; 5 MCP failure modes pass | integration | `cargo test --offline --test lib commander_error_paths` / `cargo test --offline --test lib mcp_streamable_http` | ❌ Wave 0 — `commander_error_paths_test.rs` doesn't exist; 3 of 5 MCP modes need `FixtureServer` extensions, 2 need a new fixture |
| QUAL-05 | `cargo bench` completes; baseline doc records throughput/P50/P95/P99/memory/startup | bench + manual harness | 5 commands in Code Examples | ✅ bench targets exist and compile (`cargo check --bench` verified in this session); baseline doc exists, needs a new dated section |

### Sampling Rate

- **Per task commit:** `cargo test -p <crate> --offline` scoped to the crate just touched (fast).
- **Per wave merge:** the full `RUSTFLAGS="-C instrument-coverage" ... cargo test --workspace --offline`
  run — this doubles as both "does everything still pass" and "what does coverage measure now,"
  since this phase's whole point is that measurement.
- **Phase gate:** full coverage pipeline + `cargo bench` (all 5 targets) green before `/gsd-verify-work`.

### Wave 0 Gaps

- [ ] `tests/helpers/mock_paladin_port.rs` — needs `FaultyPaladinPort` (currently only has the
      always-succeeding `MockPaladinPort` wrapping `PaladinExecutionService`)
- [ ] `crates/paladin-storage/src/redis.rs` — needs the `&self`→free-function refactor on 8 private
      methods BEFORE any test module can be added (Pitfall 1)
- [ ] `tests/integration/commander_error_paths_test.rs` — does not exist
- [ ] MCP failure-mode test bodies in/near `tests/integration/mcp_streamable_http_test.rs` — 3 of 5
      modes need `FixtureServer` extensions (unknown-tool test assertion, bad-arguments validation +
      test), 2 of 5 need a new non-compliant fixture (malformed response, handshake timeout) and a
      decision on Pitfall 4's timeout-configurability question
- [ ] `docs/src/appendix/performance-baseline.md` — needs a new dated section (D-15), not a new file

*(No framework install needed — `cargo test`/`cargo bench` are already fully wired.)*

## Security Domain

`security_enforcement` is not disabled in `.planning/config.json` (absent = enabled), so this section
is included, scoped tightly to what this phase actually touches — it does not add new security
controls, it tests existing ones.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|----------------|---------|-------------------|
| V2 Authentication | Yes, narrowly | The MCP bearer-token path this phase writes token-rejection tests against. No new auth mechanism is introduced — `BearerToken` (`mcp_streamable_http_adapter.rs`) already zeroizes on drop and hand-implements a redacting `Debug`; tests must not defeat this by, e.g., asserting on a raw token value logged anywhere. |
| V6 Cryptography / secret handling | Yes, narrowly | `BearerToken`'s `Zeroize`/`ZeroizeOnDrop` — already shipped and already has passing unit tests (`bearer_token_debug_never_leaks_the_raw_value`, `adapter_debug_never_leaks_the_bearer_token` in `mcp_streamable_http_adapter.rs`). This phase's new tests should follow the same "assert the secret never appears in Debug/log output" idiom for any test-fixture token they introduce. |
| V5 Input Validation | Yes, narrowly | The "bad arguments" MCP failure mode IS an input-validation test by definition — `FixtureServer::call_tool`'s current `unwrap_or_default()` on a missing `message` argument is itself a minor validation gap worth a one-line fix (see Pattern 3 example), not just a test. |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|------------------------|
| Bearer token leaked via `Debug`/log output | Information Disclosure | `BearerToken`'s hand-implemented redacting `Debug` — already shipped, verified by existing passing tests; this phase's new token-rejection tests must not introduce a second, non-redacting way to print the token (e.g., `println!("{}", token)` instead of `{:?}`). |
| Malformed/adversarial MCP server response crashing or hanging the client | Denial of Service | `MCPClient`'s `Result<_, ArsenalError>` returns rather than panics on parse failure (verified: no `.unwrap()`/`.expect()` on response bodies in `mcp_protocol.rs`'s public methods) — the "malformed response" test is itself the proof this holds. |

## Sources

### Primary (HIGH confidence — direct tool verification against this shipped tree, this session)
- `rustc -vV` / `cargo --version` / `command -v docker` / `ls $(rustc --print sysroot)/.../bin` — toolchain and Docker-absence confirmation
- `git log`, `git diff`, `git rev-list --count` against `9be788c8`/`f29d452`/HEAD — commit-distance and Cargo.toml-stability confirmation
- Direct `Read`/`grep` of `crates/paladin-storage/src/redis.rs`, `crates/paladin-battalion/src/commander.rs`, `crates/paladin-battalion/src/formation_service.rs`, `crates/paladin-battalion/src/phalanx_service.rs`, `tests/helpers/mock_paladin_port.rs`, `tests/helpers/mod.rs`, `tests/integration/commander_integration_tests.rs`, `tests/integration/mcp_streamable_http_test.rs`, `tests/lib.rs`, `tests/integration/mod.rs`, `src/infrastructure/adapters/arsenal/mcp_protocol.rs`, `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs`, `src/core/platform/mod.rs`, `crates/paladin-ports/src/output/{paladin_port,file_storage_port,arsenal_port}.rs`, `crates/paladin-llm/src/error.rs`, `src/main.rs`, `docs/src/appendix/performance-baseline.md`, root `Cargo.toml`
- `cargo test -p paladin-ai --offline --no-run --message-format=json` and `cargo test --workspace --offline --no-run --message-format=json`, piped through `jq`, actually executed — the basis for Pattern 2/Pitfall 2/Pitfall 3's `tests/lib.rs` wiring finding
- `cargo check -p paladin-battalion --bench battalion_benchmarks --offline` — actually executed, confirms criterion builds offline
- Direct `Read` of vendored `redis-0.32.7` (`src/aio/connection_manager.rs`) and `rmcp-2.1.0` (`src/transport/common/reqwest/streamable_http_client.rs`, `src/transport/streamable_http_client.rs`, `src/analysis/mod.rs`/`src/lib.rs` from `criterion-0.5.1`) source under `/usr/local/cargo/registry/src/index.crates.io-*/`
- `gsd-tools query package-legitimacy check --ecosystem crates wiremock criterion mockito testcontainers rmcp` — all OK

### Secondary (MEDIUM confidence)
- `.planning/decisions/0006-coverage-gate.md`, `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`, `.planning/phases/02-functional-gap-closure/02-CONTEXT.md`, `.planning/phases/03-verification-depth/03-CONTEXT.md` — all cross-checked against the live tree above, not taken on faith; the one place a CONTEXT.md claim needed correction is flagged explicitly (Pattern 2).

### Tertiary (LOW confidence)
- None — every claim in this document traces to a primary source above.

## Metadata

**Confidence breakdown:**
- Coverage pipeline reproduction: HIGH — toolchain paths and versions verified byte-identical to Phase 1's record; the pipeline was not re-run end-to-end in this research session (that is the phase's own Task 1), but every prerequisite tool/path was confirmed present and working.
- `redis.rs` testability: HIGH — the blocking constraint (ConnectionManager needs a live connection) and the fix (refactor off `&self`) are both directly verified against vendored source, not inferred.
- Commander/MCP test construction: HIGH for the mock/fixture patterns (all read from shipped code); MEDIUM for the two "hard" MCP modes (malformed response, handshake timeout) since no existing in-tree code produces either today — the recommended approach is reasoned from verified constraints, not itself pre-existing and proven.
- Benchmark/percentile derivation: HIGH — `SavedSample` schema read directly from criterion's vendored source; the derivation formula follows directly from that schema.

**Research date:** 2026-08-02
**Valid until:** 14 days (this phase re-measures a fast-moving number — coverage — against a HEAD that will keep advancing; re-verify toolchain/Docker/network facts if planning is delayed past that window)
