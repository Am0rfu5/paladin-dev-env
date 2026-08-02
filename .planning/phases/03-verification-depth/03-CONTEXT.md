# Phase 3: Verification Depth - Context

**Gathered:** 2026-08-02
**Status:** Ready for planning

<domain>
## Phase Boundary

The project's quality claims become measurements rather than targets: coverage re-measured at
current HEAD and proven at ADR-0006's recorded floor, error paths executed rather than skipped,
and performance baselines that exist and can be compared against later.

**This phase's centre of gravity is measurement, not construction.** Phase 1 wrote records, Phase 2
edited product code. Phase 3 runs commands, records what they say, and writes the specific tests
that close a named, evidence-backed gap. Where a requirement's premise turns out to be stale, the
requirement is amended at source with dated provenance — the precedent Phase 1 set (it amended this
phase's own criterion 1) and Phase 2 followed.

**Four deliverable classes:**

1. **A coverage re-measurement** at current HEAD using ADR-0006's verbatim command, recorded to the
   `01-coverage-measurement.md` provenance standard, proving the 84% floor holds 98 commits later.
2. **Targeted test-writing** against the *re-derived* zero-coverage set — not the stale list
   QUAL-02 names.
3. **Error-path closure** — the four `#[ignore]`d, empty-bodied Commander tests given real bodies
   and a shared failing mock, plus five MCP tool-invocation failure-mode tests.
4. **A performance baseline** — the five shipped bench targets run and recorded, with the metric
   families QUAL-05 names that no shipped bench produces recorded as gaps rather than fabricated.

**Not in this phase:** the two module-scoped gates, Herald ≥ 95% and autonomous ≥ 90% (Phase 5,
VERIFY-05 — ADR-0006 assigns them explicitly); extending coverage scope to the Docker-backed
`--features integration-tests` suite (Phase 5 VERIFY-05 / Phase 15 PIPE-02); wiring the 84% floor
into CI (Phase 15, PIPE-02); the live-API harness skip-vs-fail semantics (Phase 5, VERIFY-06);
version/edition/advisory coherence (Phase 4, REL); writing new Paladin-execution and
Arsenal-invocation benchmark suites (see D-11).

</domain>

<decisions>
## Implementation Decisions

### What "coverage work" actually means in this phase

- **D-01: QUAL-01 is measure-and-hold, not raise.** ADR-0006's floor is **84%**, derived by
  truncating a measured **84.79%**. The tree already cleared it on the run that set the gate — by
  construction, the gate cannot be red on that run. So Phase 3's QUAL-01 obligation is **not** "write
  tests until a number rises". It is: re-run ADR-0006's command **verbatim** at current HEAD
  (`f29d4526`, **98 commits** past the measurement commit `9be788c8`, including all of Phase 2's new
  tests), record the result to the Phase 1 provenance standard, and prove the figure is still at or
  above 84%. Test-writing in this phase is scoped by QUAL-02's zero-coverage set (D-03) and QUAL-04's
  error paths (D-07/D-08), **not** by a coverage delta target.
  Chosen over treating QUAL-01 as a coverage-raising campaign (there is no recorded target to raise
  toward — ADR-0006 explicitly retired 80% as "a superseded historical aspiration" and records no
  operative target, so a raising campaign would have no finish line) and over trusting the Phase 1
  figure without re-measuring (98 commits of Phase 2 test-writing sit between it and HEAD; a
  quality-depth phase that cites a stale number is the exact failure this milestone closes).

- **D-02: The command is run verbatim; the denominator is not "improved" mid-phase.** ADR-0006 fixes
  the `RUSTFLAGS`/`LLVM_PROFILE_FILE`/`cargo test --workspace --offline` invocation, the
  `--ignore-filename-regex`, the doctest exclusion, and the default-feature scope. Phase 3 changes
  none of them. Any change to the regex or feature set changes the denominator and therefore the
  number, which requires an in-place ADR amendment with a new figure and date — not a phase-level
  convenience.
  — **Reversibility:** costly — the recorded figure and floor are cited by VERIFY-05 (Phase 5) and
  PIPE-02 (Phase 15); a Phase 3 figure produced under a quietly-different command would hand both a
  number they cannot reconcile, reintroducing the "choosing between 80% and 85%" failure RECON-07
  exists to eliminate.

### The stale QUAL-02 offender list — the largest finding in this phase's analysis

- **D-03: QUAL-02's named offenders are stale and get re-derived, then amended at source.**
  QUAL-02 names eleven files as 0% or sub-15%. **The Phase 1 measurement contradicts nine of them.**
  Verified against `01-coverage-measurement.md` during this discussion:

  | File QUAL-02 names | Claimed | Actually measured (2026-07-31) |
  |---|---|---|
  | `arsenal_execution_service.rs` | 0/46 lines | **92.26%** |
  | `arsenal_registry_service.rs` | 0/28 lines | **100.00%** |
  | `sqlite_user_repository.rs` | 0% | **91.75%** |
  | `user_controller.rs` | 0% | **77.54%** |
  | `main.rs` | 0% | **47.37%** |
  | `campaign_service.rs` | 4.26% | **85.66%** |
  | `chain_of_command_service.rs` | 13.41% | **90.75%** |
  | `mcp_protocol.rs` | 15.83% | **95.73%** |
  | `deepseek_adapter.rs` | 15.02% | **42.45%** |
  | `redis.rs` | 0% | **0.00%** — the one true positive |
  | `minio.rs` | 0% | **absent from the denominator** (see D-04) |

  The list derives from `unit-test-improvements/COVERAGE_ANALYSIS.md`, an ingested pre-workspace
  artifact. Phase 3 **re-derives the zero-coverage set from its own measurement (D-01)** and amends
  QUAL-02 and ROADMAP success criterion 2 at source with dated provenance, exactly as Phase 1
  amended this phase's criterion 1 and Phase 2 amended its own criteria 1 and 5.
  Chosen over taking the list literally (it would send the phase to write tests for files already at
  92%, 100% and 95.73% while the genuinely uncovered ones go untouched — verification theatre, the
  precise failure mode this milestone exists to end) and over silently substituting a new list
  (an unexplained swap leaves a future reader unable to tell correction from drift).

- **D-04: The re-derived first-party zero-coverage set, as of the Phase 1 measurement.** Five
  first-party source files reported 0.00%, and the planner should expect this set — not QUAL-02's —
  to be what the Phase 3 re-measurement mostly reproduces:
  - `crates/paladin-storage/src/redis.rs` — **361 counted lines, the largest by an order of
    magnitude** (see D-05)
  - `src/bin/paladin-server.rs` — 185 lines
  - `crates/paladin-ports/src/output/file_storage_port.rs` — 117 lines
  - `crates/paladin-llm/src/error.rs` — 19 lines. **This is the dead `LlmProviderError` conversion
    path** Phase 1's ledger surfaced and Phase 2 deferred (`error.rs:16,54`, zero constructors
    anywhere). Its 0% is a *symptom of dead code*, not a missing test — closing it means deciding
    whether the path lives or dies, which is a disposition, not a test.
  - `crates/paladin-ports/src/output/arsenal_port.rs` — 2 lines
  Excluded from this set as not first-party source: `tests/**` entries (`tests/lib.rs`,
  `tests/integration/mod.rs`, `tests/helpers/mock_paladin_port.rs` — see D-08 —
  `mcp_streamable_http_live_test.rs`) and one generated file under `target/` (D-06).

- **D-05: `redis.rs` is closed with Docker-free unit tests on its pure seams.** It is **1,570 lines
  with zero `#[cfg(test)]` modules** — not one unit test in the file. Its only exercisers are
  `tests/integration/redis_queue_integration_test.rs`, which are **testcontainers-based and require
  Docker, which is absent from this environment** (the same constraint that halted plan 01-04). So
  Phase 3 adds in-file `#[cfg(test)] mod tests` covering the parts that need no live server: config
  construction and defaults, key/namespace construction, queue-item serialization round-trips,
  priority ordering, and error mapping. The live-server paths stay uncovered and get a
  `deferred with reason` ledger row naming Phase 15 (PIPE) as owner, since Docker-backed CI coverage
  is PIPE-02's scope.
  Chosen over attempting to stand up Redis in-sandbox (Docker is absent, not merely unconfigured —
  this is the blocker plan 01-04 already burned a plan on; Phase 2's CONTEXT explicitly says "do not
  re-hit the blocker plan 01-04 halted on") and over accepting 0% (it is the single largest genuine
  gap in the tree and the one QUAL-02 item that survives verification).

- **D-06: The `target/` denominator contamination is reported, not fixed.** ADR-0006's ignore regex
  excludes `examples/`, `benches/`, `crates/doc-examples/`, the cargo registry and the Rust stdlib —
  **but not `target/`**. One generated file,
  `target/debug/build/utoipa-swagger-ui-10aed8599aeed486/out/embed.rs`, sits in the denominator at
  0.00%. Magnitude: **1 line of 61,404 — immaterial**, it cannot move the figure at two decimal
  places. Phase 3 records the finding with its magnitude and flags the regex for VERIFY-05 / PIPE-02;
  it does **not** amend the regex, per D-02.

- **D-07: QUAL-02 binds only inside ADR-0006's recorded scope.** `crates/paladin-storage/src/minio.rs`
  exists but **never appears in the measurement** — the `s3` feature is not in the workspace default
  set, so the file is never compiled into the instrumented binaries and has no denominator entry at
  all. "No first-party file reports 0%" cannot reach a file that reports nothing. `minio.rs` gets a
  `deferred with reason` row naming VERIFY-05 / PIPE-02 as owner.
  Chosen over widening the feature set to bring `minio.rs` into scope — that creates a second
  coverage number under a second scope, which ADR-0006 forbids and Phase 2's D-04 restates. ROADMAP
  criterion 2 names "the Redis and MinIO adapters" together; the amendment under D-03 must record
  that they are **not** in the same position, and why.

### QUAL-03's second number

- **D-08: QUAL-03's percentage clause is recorded superseded by ADR-0006; its substance survives.**
  As written QUAL-03 asks for "integration coverage of critical paths ≥ 70%, up from the 67.79%
  baseline" — a second number under a second scope, which is exactly what ADR-0006 abolished and what
  RECON-07 exists to prevent. ROADMAP criterion 1 was already amended (plan 01-12) to cite ADR-0006's
  single figure; **QUAL-03 was not, and still carries the abolished split.** Phase 3 records the
  percentage clause `superseded by shipped code`/decision — the same disposition Phase 2's D-04 gave
  Epic 2 task 11.5 for the identical reason — and satisfies the surviving substance: each of the
  three named critical paths (**Paladin execution, Battalion orchestration, tool invocation**) has a
  named, passing, non-`#[ignore]`d integration exerciser meeting the D-19 bar (`file:line` **plus** a
  named exerciser). Amended at source in REQUIREMENTS.md with dated provenance.

### QUAL-04 — the error paths

- **D-09: The four Commander tests move to integration, and the failing mock lands in
  `tests/helpers/`.** The four tests at `crates/paladin-battalion/src/commander.rs:2180,2188,2196,2204`
  are `#[ignore]`d with **completely empty bodies** — three comment lines each, no code (verified in
  this discussion). Their own TODO comments say `move to integration tests`. Phase 3 honours them:
  build a configurable `FaultyPaladinPort` in `tests/helpers/` and relocate the four tests to a new
  `tests/integration/commander_error_paths_test.rs`, with the ledger rows citing those four line
  numbers amended in place per Phase 2's D-02.

  The mock must support, because the four tests plus ROADMAP criterion 3 need exactly these:
  fail-always; fail-the-Nth-Paladin (partial-failure collection); fail-N-then-succeed with an
  invocation counter (retry-count assertions); and a controllable delay (timeout-stops-siblings).
  It must be `Send + Sync`.

  Chosen over a new `paladin-test-support` workspace crate (adds a publishable member immediately
  before Phase 4's REL-01 "one version, one story" work has to reconcile every crate version —
  the wrong week for a new crate) and over a `test-support` feature gate on `paladin-battalion`
  (a feature flag on a published crate purely to expose test scaffolding, again landing right before
  the release-coherence phase). **Phase 2's D-07 instruction that this be a shared asset, not a local
  one, is honoured by siting it in `tests/helpers/` — the workspace's existing shared mock home
  (`MockLlmAdapter`, `MockArsenalAdapter`, `MockPaladinPort`), reachable by every root integration
  test.** Phase 15's DEFER register names this asset as a prerequisite three registers have asked for
  and none has built.
  — **Reversibility:** reversible — additive test scaffolding; no product code and no published
  surface changes.

- **D-10: Extend the dead `tests/helpers/mock_paladin_port.rs`, do not add a sixth parallel mock.**
  That file is at **0.00% coverage because nothing imports it** — only the barrel `tests/helpers/mod.rs`
  re-exports it. Meanwhile **at least six independent `MockPaladinPort` definitions exist**:
  `commander.rs:1570` (in-crate, unit struct, always succeeds), `tests/helpers/mock_paladin_port.rs`,
  `tests/integration/battalion_campaign_integration_test.rs:20`,
  `IntegrationMockPaladinPort` in `tests/integration/commander_integration_tests.rs:78`, and three
  in `examples/`. D-09's `FaultyPaladinPort` extends the shared helper file and gives it its first
  real consumer, which closes a zero-coverage entry as a side effect. **Consolidating the other five
  is explicitly out of scope** (deferred).

- **D-11: The five MCP failure modes land on the Streamable-HTTP adapter, driven by `wiremock`.**
  QUAL-04 and ROADMAP criterion 4 name five modes: expired/rejected token, malformed response,
  handshake timeout, unknown tool, bad arguments. All five are hosted against
  `MCPStreamableHttpAdapter` in `src/infrastructure/adapters/arsenal/mcp_protocol.rs` — it is the
  shipped transport that supersedes SSE (recorded as a divergence by Phase 1's ledger, so writing
  new SSE failure tests would test a superseded surface), and it owns the bearer-token path
  `codebase/CONCERNS.md` flags. `wiremock` v0.6 is already a dev-dependency: it drives 401/403 for
  token rejection, malformed bodies, and a deliberately-stalled response for the handshake timeout
  (asserted with `tokio::time::timeout`). Unknown-tool and bad-arguments are **protocol-level** —
  they assert the adapter's error mapping on a well-formed HTTP 200 carrying a JSON-RPC error, not
  an HTTP status code.
  Note for the planner: `mcp_protocol.rs` already measures **95.73%**, so this work is about the
  five *named failure modes having tests*, not about moving a coverage number.

### QUAL-05 — the performance baseline

- **D-12: Run the five shipped bench targets; record the missing suites as gaps rather than building
  them.** The tree ships exactly five bench targets, all declared and all present:
  `benches/config_benchmarks.rs` (root), `crates/paladin-battalion/benches/battalion_benchmarks.rs`,
  `crates/paladin-memory/benches/{garrison,sanctum}_benchmarks.rs`, and
  `crates/paladin-llm/benches/llm_serialization_benchmarks.rs`. QUAL-05 additionally names the
  **Paladin execution loop** and **Arsenal invocation** — the Milestone-1 `paladin_benchmarks.rs`,
  `herald_benchmarks.rs` and `arsenal_benchmarks.rs` are **not in the tree**, a fact REQUIREMENTS.md
  already records. Writing two new criterion suites is feature work in a verification phase; Phase 3
  runs what ships, records per-metric coverage explicitly, and files `deferred with reason` rows for
  the two absent surfaces.
  Chosen over authoring the missing suites (unbudgeted construction inside a measurement phase, and
  a new benchmark's first run is by definition not a *baseline against* anything) and over declaring
  QUAL-05 unmeetable (four of the five bench targets and both memory/startup metrics are reachable
  today — see D-13).
  **Feasibility confirmed in this discussion:** `criterion 0.5.1` source is present in the local
  cargo registry, so `cargo bench --offline` builds despite crates.io returning 403.

- **D-13: Memory-per-Paladin and startup time are measured by a small recorded harness, not by new
  criterion suites.** QUAL-05 names four metric families. Criterion produces throughput and latency;
  it produces neither memory-per-Paladin nor startup time. Rather than leave two of four blank or
  fabricate them, Phase 3 records them from a purpose-built, documented measurement (process RSS
  delta across a controlled number of constructed Paladins; wall-clock to a ready
  `paladin-server` / first-Paladin-constructed). The baseline document states which source produced
  each metric family so no reader mistakes a derived figure for a criterion result.

- **D-14: P50/P95/P99 are derived from criterion's raw samples, with the derivation documented.**
  Criterion reports mean, median, MAD and confidence intervals — **not** P95 or P99. The percentiles
  are computed from criterion's own per-iteration sample data under `target/criterion/*/new/`, and
  the baseline document shows the derivation.
  Chosen over relabelling criterion's median as "P50" and leaving P95/P99 empty (silently
  under-delivers three of the criterion's named metrics) and over reporting them without stating the
  derivation (an underived percentile in this corpus becomes a cited number later — the failure
  ADR-0006 was written to end).

- **D-15: The baseline is amended into `docs/src/appendix/performance-baseline.md` in place.** That
  document already exists and is already linked from the mdbook (Phase 11 made linkcheck an error,
  so a parallel `.planning/` baseline would fork the record and orphan the shipped page). Its
  current run is dated **2026-05-27** against commit `f4156ff6` on different hardware. Phase 3 adds
  a new dated run section and **retains the prior run as an explicitly superseded section** rather
  than overwriting it — the same in-place-amendment convention Phase 2's D-02 established for the
  ledger.

### Measurement provenance — applies to every measurement in this phase

- **D-16: Every recorded figure carries the Phase 1 provenance block, and every command carries
  `--offline`.** `rustc -vV`, `cargo --version`, `git rev-parse HEAD`, `date -u`, plus CPU model,
  core/thread count and kernel for the benchmark runs — captured immediately before the command,
  with raw stdout pasted, and arithmetic a reader can re-derive.
  `01-coverage-measurement.md` is the template; Phase 1's verifier called this the highest-value
  check in that phase, and Phase 2's D-01 already inherited it.
  **Benchmark figures are recorded as this machine's baseline with the environment stated, and
  explicitly not as a portable performance claim** — ROADMAP criterion 5's own purpose is "so the
  next performance change can be compared against something", which needs a stated environment, not
  a universal number. The sandbox constraints stay stated plainly, never glossed: no Docker,
  crates.io HTTP 403, `cargo-llvm-cov` not installable.

### Claude's Discretion

- **Plan decomposition and count.** No tracer is mandated. The natural shape is measurement-first
  (D-01 gates D-03/D-04, which gate the test-writing), but whether the re-measurement is its own
  plan or task 1 of a larger one is the planner's call.
- **Whether `crates/paladin-llm/src/error.rs`'s dead conversion path is deleted or exercised.**
  D-04 identifies it as a disposition question, not a test-writing one. Phase 2 left it open. Either
  a test that gives the `From<LlmProviderError> for LlmError` impl its first caller, or removal with
  a `superseded by shipped code` row — the planner should pick one and record which.
- **Whether `src/bin/paladin-server.rs` (185 lines, 0%) is closed or deferred.** A binary entrypoint
  is legitimately hard to unit-test; `main.rs` sits at 47.37% by comparison. Closing it may mean
  extracting a testable `run()` seam, which is a refactor. Deferring it needs a named owner.
- **Where the ROADMAP/REQUIREMENTS amendments under D-03 and D-08 physically land** — in-place edits
  plus a Phase 3 amendments record, or in-place with provenance notes only. Phase 2 did the latter.
- **Whether `crates/paladin-ports/src/output/file_storage_port.rs` (117 lines, 0%) is a real gap.**
  A port trait file at 0% usually means default method bodies and doc examples that nothing
  instantiated; check before writing tests against it.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### The binding decision — read first

- `.planning/decisions/0006-coverage-gate.md` — **the single most important input to this phase.**
  The one number (84% floor from a measured 84.79%), the verbatim command with every flag and env
  var, the `--ignore-filename-regex` in words, the doctest exclusion, the default-feature scope
  narrowing, the truncation rounding rule, the at-or-above comparison, the ratchet trigger, and the
  explicit hand-off of the Herald ≥ 95% / autonomous ≥ 90% module gates to Phase 5's VERIFY-05.
  Also flags the function-vs-line coverage gap (77.34% vs 84.79%) as VERIFY-05's, not this phase's.
- `.planning/decisions/PROMOTION.md` — ADR house conventions, if any Phase 3 disposition escalates
  to an ADR.

### The measurement standard this phase must match

- `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` — **the template
  for every measurement in this phase** (D-16): toolchain versions, verbatim commands, raw pasted
  stdout, the full `llvm-cov report` TOTAL row, per-file rows, and a human-confirmation section.
  Its per-file rows are also the evidence base for D-03's staleness table and D-04's re-derived
  zero-coverage set.
- `.planning/phases/01-ground-truth-decision-records/01-VERIFICATION.md` §"The measurement's
  honesty" — the model for recording a measured claim without overstating it.

### Prior-phase decisions this phase inherits

- `.planning/phases/02-functional-gap-closure/02-CONTEXT.md` — **D-07 assigns this phase the four
  `#[ignore]`d Commander tests, the MCP failure-mode tests, and the instruction that the failing
  mock be built as a shared asset, not a local one.** Also D-01 (re-prove by execution), D-02
  (amend the ledger in place), D-04 (no second coverage number).
- `.planning/phases/02-functional-gap-closure/02-VERIFICATION.md` — 5/5 passed; records
  `cargo test --workspace` at **2864 passed, 0 failed** on this tree, and the re-verification
  discipline (re-check independently, do not trust a SUMMARY).
- `.planning/phases/01-ground-truth-decision-records/01-CONTEXT.md` — D-19's evidence bar
  (`file:line` **plus** a named passing exerciser) and D-20's five verdict classes, which every
  ledger amendment in this phase must satisfy.
- `.planning/ledgers/milestone-01.md` §"Verdict legend" — the same, at source. Amended in place by
  this phase per Phase 2's D-02.

### Requirements and roadmap — including what this phase amends

- `.planning/REQUIREMENTS.md:257-289` — QUAL-01 … QUAL-05 in full with `Derives:` provenance.
  **QUAL-02's offender list and QUAL-03's percentage clause are both amended by this phase**
  (D-03, D-08).
- `.planning/REQUIREMENTS.md:3830-3834` — the QUAL-01…QUAL-05 Traceability rows, all `Pending`.
- `.planning/REQUIREMENTS.md:3948` — the recorded `RECON-07 → VERIFY-05 → QUAL-01/QUAL-03` coupling.
- `.planning/ROADMAP.md` §"Phase 3: Verification Depth" — the five success criteria. **Criterion 1
  was already amended by plan 01-12 to cite ADR-0006; criterion 2's file list is stale (D-03).**
- `.planning/ROADMAP.md` §"Phase 5: Milestone 2-3 Ground Truth" — VERIFY-05, owner of the module
  gates, the wider scope, and the function-coverage gap.
- `.planning/ROADMAP.md` §"Phase 15" — PIPE-02, which wires the 84% floor into CI and owns the
  Docker-backed scope extension.
- `.planning/PROJECT.md` §Context — the precedence order (ADR → shipped tree → `.planning/codebase/`
  map → `intel/code-verification.md` → PRD → DOC → task-list checkbox). **This phase applies it
  directly: an ingested COVERAGE_ANALYSIS.md loses to a measurement of the shipped tree.**

### Code-state intelligence

- `.planning/intel/code-verification.md` — third in the precedence order; source of standing
  "do not plan this" instructions.
- `.planning/codebase/TESTING.md` — the three-tier strategy, the `tests/unit/mod.rs` barrel, the
  `tests/helpers/` mock conventions D-09/D-10 build on, the `[[test]]` target declarations, and the
  dev-dependency list (`wiremock` v0.6, `mockito` v1.7, `criterion` v0.5, `testcontainers` v0.24).
  **Note §Coverage is stale** — it documents `cargo tarpaulin`, which ADR-0006 explicitly says must
  not be compared against its gate.
- `.planning/codebase/CONCERNS.md` — the MCP bearer-token visibility concern D-11's token-rejection
  tests touch, the unwrap/expect inventory, and the "add integration tests that exercise error
  paths" recommendation this phase partly discharges.

### Shipped code this phase measures, tests or edits

- `crates/paladin-battalion/src/commander.rs:2176-2208` — the four `#[ignore]`d tests with **empty
  bodies**; relocated by D-09.
- `crates/paladin-battalion/src/commander.rs:1570-1601` — the in-crate always-succeeding
  `MockPaladinPort` unit struct that is why those four tests were never written.
- `tests/helpers/mock_paladin_port.rs` — the shared mock at **0.00% coverage, imported by nobody**;
  extended by D-10 with `FaultyPaladinPort`.
- `tests/helpers/mod.rs` — the barrel that must re-export the new mock.
- `tests/integration/commander_integration_tests.rs:78` — `IntegrationMockPaladinPort`, the pattern
  D-09's relocated tests should follow.
- `crates/paladin-storage/src/redis.rs` — **1,570 lines, zero `#[cfg(test)]` modules, 0.00%
  coverage.** D-05's Docker-free unit tests land here.
- `tests/integration/redis_queue_integration_test.rs` — testcontainers-based, needs Docker; why
  `redis.rs` measures 0% here.
- `crates/paladin-storage/src/minio.rs` + `crates/paladin-storage/Cargo.toml` §`[features]` —
  the `s3` feature that keeps `minio.rs` out of the denominator entirely (D-07).
- `src/infrastructure/adapters/arsenal/mcp_protocol.rs` — `MCPStreamableHttpAdapter`, host of D-11's
  five failure-mode tests; already at 95.73%.
- `crates/paladin-llm/src/error.rs:16,54` — the dead `LlmProviderError` conversion path at 0.00%
  (discretion item).
- `src/bin/paladin-server.rs` — 185 lines at 0.00% (discretion item).
- `crates/paladin-ports/src/output/file_storage_port.rs` — 117 lines at 0.00% (discretion item).
- `benches/config_benchmarks.rs`, `crates/paladin-battalion/benches/battalion_benchmarks.rs`,
  `crates/paladin-memory/benches/{garrison,sanctum}_benchmarks.rs`,
  `crates/paladin-llm/benches/llm_serialization_benchmarks.rs` — the five shipped bench targets
  (D-12), all with `harness = false` and declared in their crates' `Cargo.toml`.
- `docs/src/appendix/performance-baseline.md` — the existing baseline (2026-05-27, commit
  `f4156ff6`), amended in place by D-15.
- `crates/paladin-storage/src/scheduler.rs:498-505` — a documented, deliberately-`#[ignore]`d block
  (the real `tokio-cron-scheduler` engine needs a multi-thread runtime). **Legitimately ignored —
  not in this phase's un-ignoring scope.**

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- **The offline coverage pipeline from plan 01-09** — `RUSTFLAGS="-C instrument-coverage"` +
  `llvm-profdata merge` + `llvm-cov report`, driving `rustc` directly with no crates.io fetch.
  This is the only working coverage path in this environment; `cargo-llvm-cov` is not installable
  (crates.io 403) and there is no Docker. **Do not re-hit the blocker plan 01-04 halted on.**
- **`criterion 0.5.1` is present in the local cargo registry**
  (`/usr/local/cargo/registry/src/index.crates.io-*/criterion-0.5.1`) and pre-built rlibs exist in
  `target/debug/deps/`. `cargo bench --offline` is therefore feasible — verified in this discussion.
- **`wiremock` v0.6 and `mockito` v1.7** are already dev-dependencies. D-11 uses wiremock; no new
  dependency is needed for the MCP failure-mode tests.
- **`tests/helpers/`** — `mock_llm_adapter.rs` (queued responses, error injection, invocation
  tracking), `mock_arsenal_adapter.rs`, `mock_paladin_port.rs` and the barrel `mod.rs`. The
  established shared-mock home; D-09/D-10 extend it rather than starting elsewhere.
- **`tests/integration/battalion/load_test.rs`** — five real, non-`#[ignore]`d load/stress tests
  already validating Phalanx's ≥ 10 concurrent / < 1 s claims. Relevant to D-08's
  Battalion-orchestration critical path: verify, do not rebuild.
- **`.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md`** — reusable as
  both the provenance template and the per-file evidence base.

### Established Patterns

- **Precedence is the project's core mechanic**: ADR → shipped tree → `.planning/codebase/` map →
  `intel/code-verification.md` → PRD → DOC → task-list checkbox. **D-03 is this rule applied at full
  strength** — an ingested `COVERAGE_ANALYSIS.md` loses to a measurement of the shipped tree.
- **The D-19 evidence bar and D-20 verdict classes govern every ledger amendment.** `satisfied` needs
  a `file:line` **plus** a named passing exerciser. Never upgrade a row because code exists.
- **Amend at source with dated provenance.** Phase 1 amended this phase's criterion 1; Phase 2
  amended its own criteria 1 and 5 and the ledger in place. D-03 and D-08 follow the same route.
- **Test organization**: unit tests co-located in `#[cfg(test)] mod tests`; integration tests as
  `tests/integration/*_test.rs` declared as `[[test]]` targets in `Cargo.toml`; `tests/unit/mod.rs`
  is the barrel. **A new integration test file needs its `[[test]]` declaration** — the missing-barrel
  class of defect is exactly what Phase 2's D-10/D-12 found and fixed.
- **Medieval military ubiquitous language is mandatory** — including new test names and
  `FaultyPaladinPort`.
- **Repo working agreement**: `cargo test` → `cargo fmt --check` → `cargo clippy` before committing;
  no `unwrap()`/`expect()`/`panic!` in library code (test code is exempt); conventional commits.

### Integration Points

- **`tests/helpers/mod.rs`** — the barrel gaining `FaultyPaladinPort`'s re-export (D-09/D-10).
- **`Cargo.toml` `[[test]]` targets** — gains `commander_error_paths` and any new MCP failure-mode
  target.
- **`.planning/ledgers/milestone-01.md`** — amended in place: the four Commander rows (relocated),
  `redis.rs` and `minio.rs` deferrals, the QUAL-02 staleness correction, and the QUAL-03
  supersession.
- **`.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md`** — amended at source per D-03 and D-08.
- **`docs/src/appendix/performance-baseline.md`** — amended in place (D-15); it is linked from the
  mdbook, where Phase 11 made linkcheck an error.
- **Phase 5's VERIFY-05** — receives the module-scoped gates, the wider coverage scope, the
  function-vs-line gap, the `target/` regex finding (D-06) and the `minio.rs` deferral (D-07).
- **Phase 15's PIPE-02** — receives the 84% CI threshold, the Docker-backed scope extension, and
  `redis.rs`'s live-server coverage (D-05).
- **Phase 4's REL** — D-09 deliberately avoids adding a workspace crate or a published feature flag
  immediately before REL-01's version-coherence work.

</code_context>

<specifics>
## Specific Ideas

- **Facts verified live during this discussion. Treat them as established, not as hypotheses to
  re-check:**
  1. **QUAL-02's offender list is stale in nine of eleven entries** — the full comparison table is in
     D-03, transcribed from `01-coverage-measurement.md`'s per-file rows. Any plan that takes the
     list literally will write tests for files already at 92%, 100% and 95.73%.
  2. **The four Commander tests are not merely `#[ignore]`d — they are empty.** Each is three comment
     lines and no code (`commander.rs:2176-2208`). There is nothing to un-ignore; they must be
     written.
  3. **`redis.rs` has 1,570 lines and zero `#[cfg(test)]` modules**, and its only exercisers need
     Docker, which is absent. It is the single largest genuine coverage gap in the tree.
  4. **`minio.rs` is not in the coverage denominator at all** — the `s3` feature is not in the
     workspace default set, so it is never compiled into the instrumented binaries.
  5. **`criterion 0.5.1` source is in the local registry**, so `cargo bench --offline` builds despite
     crates.io returning 403.
  6. **At least six independent `MockPaladinPort` definitions exist**, and the shared one in
     `tests/helpers/` is at 0.00% because nothing imports it.
  7. **HEAD is 98 commits past the coverage measurement commit** (`f29d4526` vs `9be788c8`),
     including all of Phase 2's new tests — which is why D-01 requires re-measurement.

- **Expect the re-measurement to move the number upward, and do not treat that as a problem.**
  Phase 2 added tests across Herald, the LLM unit module, the CLI cluster and the Formation
  end-to-end path. If the figure rises two or more whole points above 84%, **ADR-0006's ratchet
  trigger fires** — the floor is raised to the next whole percent below the new figure by amending
  ADR-0006 in place with the new figure, command and date. The planner should decide whether the
  ratchet is Phase 3's to apply or a milestone-close action, and record which.

- **`present, unproven` remains an acceptable outcome class.** Phase 3 shrinks the bucket where it
  can and records the rest honestly at the D-19 bar; it does not need to reach zero.

- **The phase's honesty risk is specific and nameable:** every deliverable here is a number, and
  numbers in this corpus have repeatedly been cited rather than measured. Every figure Phase 3
  records must be traceable to a pasted command output on a stated commit — that is what separates
  this phase from the six coverage positions ADR-0006 had to adjudicate.

</specifics>

<deferred>
## Deferred Ideas

- **The two module-scoped coverage gates — Herald ≥ 95%, autonomous ≥ 90%.** Owner: Phase 5,
  VERIFY-05, assigned explicitly by ADR-0006. `herald.rs` measured 80.49%, below its 95% target.
  **Not this phase's** — pulling them in would create the second scope ADR-0006 exists to prevent.
- **The function-vs-line coverage gap** (77.34% function vs 84.79% line). Owner: Phase 5, VERIFY-05,
  flagged by ADR-0006 as context for its module-gate work.
- **Extending the coverage scope to the Docker-backed `--features integration-tests` suite.**
  Owners: Phase 5 VERIFY-05 and Phase 15 PIPE-02. Blocked here by Docker's absence.
- **Wiring the 84% floor into CI.** Owner: Phase 15, PIPE-02. No coverage job exists in `ci.yml` at
  all today.
- **`crates/paladin-storage/src/minio.rs` coverage** (D-07) — outside ADR-0006's recorded scope.
  Owner: VERIFY-05 / PIPE-02.
- **`redis.rs`'s live-server code paths** (D-05) — need Docker. Owner: Phase 15, PIPE.
- **Fixing ADR-0006's `--ignore-filename-regex` to exclude `target/`** (D-06). Immaterial today
  (1 line of 61,404) but it will not stay immaterial forever. Owner: VERIFY-05 / PIPE-02.
- **Consolidating the six parallel `MockPaladinPort` definitions** (D-10). Phase 3 adds one real
  consumer to the shared helper; unifying the rest — including the three in `examples/`, which
  Phase 4's SC5 requires to build in CI — is separate work with no owner assigned.
- **Writing Paladin-execution-loop and Arsenal-invocation benchmark suites** (D-12). The Milestone-1
  `paladin_benchmarks.rs`, `herald_benchmarks.rs` and `arsenal_benchmarks.rs` are not in the tree.
  Feature work; no owner assigned in this discussion.
- **The live-API harness skip-vs-fail semantics** (`require_api_key()` panics on a missing key).
  Owner: Phase 5, VERIFY-06. The ~25 `#[ignore]`d provider tests in
  `tests/integration/{openai,anthropic,deepseek}_provider_test.rs` and
  `cli_real_services_test.rs` are legitimately ignored pending that decision — **explicitly not in
  this phase's un-ignoring scope**, which is the four Commander tests only.
- **CI configuration for the `live-api-tests` feature** (Epic 6 task 7.14). Owner: Phase 15, PIPE.
- **Battalion-wide cancellation for Formation, Campaign and ChainOfCommand** (Phase 2's D-05/ADR-0007).
  Still no forward owner. **Phase 3 is not it** — this phase's scope is measurement and error-path
  proof, not a new cancellation contract across four execution services.
- **Version, edition and advisory coherence.** Owner: Phase 4, REL-01…REL-05. Relevant only as the
  reason D-09 avoids adding a workspace crate or published feature flag now.

</deferred>

---

*Phase: 3-verification-depth*
*Context gathered: 2026-08-02*
