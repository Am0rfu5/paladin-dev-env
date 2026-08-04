# Phase 3: Verification Depth - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-02
**Phase:** 3-verification-depth
**Mode:** `--auto --chain` — all gray areas auto-selected, recommended option taken for each and
logged inline. No user prompts were issued.
**Areas discussed:** Coverage reframe, QUAL-02 offender-list staleness, coverage denominator scope,
QUAL-03's second number, failing-mock siting, MCP failure-mode surface, benchmark scope and metric
gaps, baseline document siting

---

## Coverage reframe (QUAL-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Measure-and-hold | Re-run ADR-0006's verbatim command at HEAD, prove ≥ 84%, scope test-writing by QUAL-02 | ✓ |
| Raising campaign | Treat QUAL-01 as "write tests until the number rises" | |
| Trust the Phase 1 figure | Cite 84.79% without re-measuring | |

**Choice:** Measure-and-hold (recommended default).
**Notes:** ADR-0006 records **no operative target** — it explicitly retires 80% as "a superseded
historical aspiration" — so a raising campaign would have no finish line. HEAD is **98 commits**
past the measurement commit and includes all of Phase 2's new tests, so citing the old figure in a
verification-depth phase would be the exact failure this milestone closes. → **D-01, D-02**

---

## QUAL-02 offender-list staleness

| Option | Description | Selected |
|--------|-------------|----------|
| Re-derive + amend at source | Derive the offender set from the Phase 3 measurement; amend QUAL-02 and ROADMAP SC2 with dated provenance | ✓ |
| Take the list literally | Write tests for the eleven files QUAL-02 names | |
| Substitute silently | Use a new list without explaining the change | |

**Choice:** Re-derive and amend at source (recommended default).
**Notes:** The decisive finding of this discussion. **Nine of eleven named offenders are
contradicted by the Phase 1 measurement** — `arsenal_registry_service.rs` is at 100%, not 0/28
lines; `mcp_protocol.rs` at 95.73%, not 15.83%; `chain_of_command_service.rs` at 90.75%, not 13.41%.
The list derives from an ingested pre-workspace `COVERAGE_ANALYSIS.md`, which loses to a measurement
of the shipped tree under the project's own precedence order. Taking it literally would send the
phase to write tests for files already above 90% while the genuine gaps go untouched. Phase 1 and
Phase 2 both set the amend-at-source precedent. → **D-03, D-04**

---

## Coverage denominator scope

| Option | Description | Selected |
|--------|-------------|----------|
| Bind QUAL-02 to ADR-0006's scope | `minio.rs` is out of scope and deferred; regex contamination reported, not fixed | ✓ |
| Widen the feature set | Enable `s3` so `minio.rs` enters the denominator | |
| Fix the ignore regex now | Exclude `target/` and re-measure | |

**Choice:** Bind to ADR-0006's recorded scope (recommended default).
**Notes:** `minio.rs` never appears in the measurement at all — the `s3` feature is not in the
workspace default set, so it is never compiled into the instrumented binaries. "No first-party file
reports 0%" cannot reach a file that reports nothing. Widening the feature set would create a second
number under a second scope, which ADR-0006 forbids and Phase 2's D-04 restates. The `target/`
contamination (one generated `utoipa-swagger-ui` file at 0%) is **1 line of 61,404** — immaterial,
and fixing the regex would change the denominator and require an in-place ADR amendment. Both go to
VERIFY-05 / PIPE-02. → **D-06, D-07**

---

## QUAL-03's second number

| Option | Description | Selected |
|--------|-------------|----------|
| Supersede the percentage, keep the substance | Record the ≥ 70% clause superseded by ADR-0006; prove each named critical path has a passing exerciser | ✓ |
| Measure integration coverage separately | Produce a second, integration-scoped figure | |
| Leave QUAL-03 as written | Attempt both numbers | |

**Choice:** Supersede the percentage, keep the substance (recommended default).
**Notes:** QUAL-03 still carries "integration coverage ≥ 70%, up from 67.79%" — the precise
second-number split ADR-0006 abolished and RECON-07 exists to prevent. ROADMAP criterion 1 was
amended by plan 01-12 to cite ADR-0006; **QUAL-03 was missed.** Same disposition Phase 2's D-04 gave
Epic 2 task 11.5, for the same reason. → **D-08**

---

## Failing-mock siting (QUAL-04)

| Option | Description | Selected |
|--------|-------------|----------|
| `tests/helpers/` + relocate to integration | Build `FaultyPaladinPort` in the existing shared mock home; move the four tests, honouring their own TODOs | ✓ |
| `test-support` feature on `paladin-battalion` | Feature-gate the mock so crate-local unit tests can reach it | |
| New `paladin-test-support` crate | A dedicated workspace member for shared test scaffolding | |

**Choice:** `tests/helpers/` plus relocation (recommended default).
**Notes:** The four tests are **empty, not merely ignored** — three comment lines each, no code —
and their own TODOs say `move to integration tests`. Both rejected options add a publishable crate
or a published feature flag immediately before Phase 4's REL-01 "one version, one story" work has to
reconcile every crate version. Phase 2's D-07 requirement that the asset be **shared, not local** is
honoured by `tests/helpers/`, the workspace's existing shared mock home, reachable by every root
integration test. Side benefit: `tests/helpers/mock_paladin_port.rs` currently measures **0.00%
because nothing imports it**, and this gives it its first consumer. → **D-09, D-10**

---

## MCP failure-mode surface

| Option | Description | Selected |
|--------|-------------|----------|
| Streamable-HTTP + `wiremock` | Host all five modes on `MCPStreamableHttpAdapter`, HTTP faults via wiremock, protocol faults via JSON-RPC error bodies | ✓ |
| Spread across stdio / SSE / HTTP | One or two modes per transport | |
| stdio adapter | Use the process-based transport | |

**Choice:** Streamable-HTTP with `wiremock` (recommended default).
**Notes:** Streamable-HTTP is the shipped transport that **supersedes SSE** — Phase 1's ledger
records that divergence, so new SSE failure tests would exercise a superseded surface. It also owns
the bearer-token path `codebase/CONCERNS.md` flags. `wiremock` v0.6 is already a dev-dependency, so
no new dependency. Unknown-tool and bad-arguments are protocol-level and assert error mapping on a
well-formed HTTP 200 carrying a JSON-RPC error, not a status code. → **D-11**

---

## Benchmark scope and metric gaps (QUAL-05)

| Option | Description | Selected |
|--------|-------------|----------|
| Run the five shipped suites; record gaps; harness the two non-criterion metrics | Measure what ships, derive percentiles from raw samples, defer the absent suites | ✓ |
| Write the missing Paladin/Arsenal bench suites | Author new criterion targets to cover every metric QUAL-05 names | |
| Declare QUAL-05 unmeetable | Defer the whole requirement | |

**Choice:** Run what ships, record gaps honestly (recommended default).
**Notes:** Five bench targets ship; the Milestone-1 `paladin_benchmarks.rs`, `herald_benchmarks.rs`
and `arsenal_benchmarks.rs` are **not in the tree**, a fact REQUIREMENTS.md already records. Writing
new suites is feature work inside a measurement phase, and a new benchmark's first run is by
definition not a baseline *against* anything. Criterion produces neither memory-per-Paladin nor
startup time, and reports mean/median/MAD/CI but **not P95/P99** — so percentiles are derived from
criterion's own per-iteration sample data with the derivation shown, and the two remaining metric
families come from a small documented harness rather than being left blank or fabricated.
Feasibility confirmed: `criterion 0.5.1` source is in the local cargo registry, so `cargo bench
--offline` builds despite crates.io returning 403. → **D-12, D-13, D-14**

---

## Baseline document siting

| Option | Description | Selected |
|--------|-------------|----------|
| Amend `docs/src/appendix/performance-baseline.md` in place | Add a dated run section; retain the prior run as superseded | ✓ |
| New `.planning/` baseline artifact | Keep the phase record separate from shipped docs | |
| Overwrite the existing document | Replace the 2026-05-27 run | |

**Choice:** Amend in place (recommended default).
**Notes:** The document already exists and is linked from the mdbook, where Phase 11 made linkcheck
an error — a parallel `.planning/` baseline would fork the record and orphan the shipped page.
Retaining the prior run (2026-05-27, commit `f4156ff6`, different hardware) as an explicitly
superseded section follows Phase 2's D-02 in-place-amendment convention. → **D-15**

---

## Claude's Discretion

Recorded in CONTEXT.md `<decisions>` §"Claude's Discretion" — plan decomposition and count; whether
`crates/paladin-llm/src/error.rs`'s dead conversion path is deleted or exercised; whether
`src/bin/paladin-server.rs` is closed or deferred; where the ROADMAP/REQUIREMENTS amendments
physically land; and whether `file_storage_port.rs` at 0% is a real gap or an artefact of a port
trait with only default bodies.

## Deferred Ideas

Full list in CONTEXT.md `<deferred>`. Headline items: the two module-scoped gates (Herald ≥ 95%,
autonomous ≥ 90%) and the function-vs-line coverage gap → Phase 5 VERIFY-05; the Docker-backed scope
extension and CI threshold wiring → Phase 15 PIPE-02; `minio.rs` and `redis.rs` live-server paths;
the `target/` ignore-regex fix; consolidating six parallel `MockPaladinPort` definitions; writing the
absent Paladin-execution and Arsenal-invocation bench suites; the ~25 legitimately-`#[ignore]`d
live-API provider tests → Phase 5 VERIFY-06; and Battalion-wide cancellation, still without a
forward owner.

## Notes on mode

`--auto` selected all eight gray areas and took the recommended option for each in a single pass, as
the mode requires. No self-feeding second pass was run. `--chain` persists, so plan-phase is
launched next.
