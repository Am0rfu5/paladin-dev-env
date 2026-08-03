# Milestone 1 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 1 as-shipped ledger` section (D-17).
That section becomes a pointer to this file. Phases 5, 7, 10 and 13 each add a sibling ledger
(`milestone-02-03.md`, `milestone-04-06.md`, `milestone-07-08.md`, `milestone-09-12.md`) rather than
growing REQUIREMENTS.md further — REQUIREMENTS.md is already ~4,000 lines and five inline sets of
`file:line`-cited verdicts would make it unreadable.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the
requirement they belong to, not given their own identifiers — the ~40 outstanding Milestone-1 task
items are numbered positions inside `.project/` task-list files with no `REQ-*` key of their own,
so nesting them keeps this ledger joinable to `REQUIREMENTS.md` and `ROADMAP.md` without inventing
new IDs (D-18).

**Evidence bar.** A `satisfied` verdict requires a `file:line` citation **plus** a named passing
test, example, or command that exercises it. A `file:line` citation with nothing exercising it is
not `satisfied` — it gets its own verdict, `present, unproven` (D-19). This bar exists because "the
code exists" has already produced false-positive completions in this corpus: Milestone 4 Epic 3's
task list is fully checked while three CLI-only dependencies remain unconditional in library builds.

## Phase 2 amendments (2026-08-01)

This file is now **amended in place**, per Phase 2 CONTEXT.md D-02: when a Phase 2 plan's measured
result contradicted a row below, that row was edited directly with the new verdict, the command or
`file:line` that produced it, and this date — never split into a separate corrections file. Phases
5, 7, 10 and 13 inherit this same convention for their own sibling ledgers. Every amendment below is
sourced from a named Phase 2 plan's SUMMARY (`.planning/phases/02-functional-gap-closure/02-0{1..8}-SUMMARY.md`)
or from `02-test-baseline.md`, plan 02-01's measured `cargo test --workspace` baseline
(commit `7e55655ccf58e7f010f5fbbf9c2504f30a0ea29c`).

**Plan 02-01's baseline re-proof of GAP-01, GAP-02, GAP-04 and GAP-05 found zero contradictions** —
all four re-proofs *agreed* with the ledger rows they checked (`REQ-commander-auto-selection`,
`REQ-chain-of-command-execution`, `REQ-phalanx-concurrency`, `REQ-commander-result-normalization`,
`REQ-commander-telemetry`). No amendment in this section is driven by a GAP-01/02/04/05
contradiction — every amendment below instead corrects a `present, unproven` or
`genuinely outstanding` row that a later Phase 2 plan's own implementation or review work closed, or
records the new ADR-0007 deferral split. This is stated explicitly here so the absence of any
GAP-01/02/04/05 contradiction is a recorded fact, not an inferred silence.

### Corrections to this phase's own planning inputs

Three corrections surfaced during Phase 2 execution, each to a document this phase itself used as an
input — not to a Milestone-1 requirement:

1. **CONTEXT.md D-15's `ProviderCapabilities` construction-site list was incomplete.** D-15 was
   compiled by grep during Phase 2's discussion phase and named the following sites beyond the three
   adapters: `grove_service.rs:1110`, `mock.rs:267,378`, `temperature_service.rs:356`,
   `planning_service.rs:641`, `prompt_generation_service.rs:299`,
   `paladin_execution_service.rs:2541,2631,2715`, plus `llm_port.rs`'s own doc/test sites. **It
   omitted the OpenAI and Anthropic adapters' own `get_capabilities` literals** — both needed the
   same field addition as DeepSeek. Plan 02-02's SUMMARY carries the full, compiler-verified list
   (every exhaustive `ProviderCapabilities { ... }` construction site cross-referenced against
   `grep -rn "ProviderCapabilities\s*{"`, confirmed by a clean `cargo build --workspace` after every
   site was updated): D-15's list plus the two adapters, with `vision_llm_port.rs` and the
   `examples/herald_*.rs` files correctly excluded (they already used `..Default::default()`).
2. **ADR-0001's `Code Locations` section omitted the ports-crate consumer sites for the citadel
   rename.** ADR-0001 named the rename's consumer sites, but plan 02-03 found two additional
   compiled consumers only when `cargo test --workspace` failed to compile after the declared edits:
   `crates/paladin-ports/src/output/citadel_port.rs` (4 sites — a compiled `#[cfg(test)]` module's
   import and construction call, plus a `rust,no_run` doc example's import and construction call) and
   `tests/integration/citadel_integration_test.rs` (3 sites — an import and two `BattalionState::new`
   construction calls). Plan 02-03's SUMMARY carries the full corrected count (6 files touched total,
   2 beyond ADR-0001's own declared list).
3. **The research document's claim that a missed doc example would fail `cargo test` workspace-wide
   is false for `paladin-ports`.** `crates/paladin-ports/Cargo.toml` sets `[lib] doctest = false`
   (the same DEBT-03 finding `PROJECT.md` already records), so a stale doc example inside that crate
   does not fail `cargo test --workspace` at all — it silently never runs. Plan 02-03 updated
   `citadel_port.rs`'s doc example anyway, on the plan's own instruction, specifically **because**
   DEBT-03 leaves it unexercised today: the moment `doctest = false` is lifted (DEBT-03's own future
   fix), a stale example would then fail the build for the first time. This makes DEBT-03
   **load-bearing** for any future rename inside this crate — a future renamer must not rely on
   `cargo test --workspace` to catch a missed doc example here, since the safety net DEBT-03 removed
   is exactly the one that would have caught it. The real compile-breaking sites for the citadel
   rename were the ones inside `citadel_port.rs`'s compiled `#[cfg(test)]` module, not its doc
   example.

## Phase 3 amendments (2026-08-02)

This file is amended in place again, per Phase 3 CONTEXT.md D-08/D-02 and the same convention Phase
2 recorded above: every amendment below is additive, retains any superseded text, and carries a
dated note naming its evidence. Every row below is sourced from a named Phase 3 plan's SUMMARY or
from `.planning/phases/03-verification-depth/03-coverage-measurement.md` /
`03-critical-path-exercisers.md` — the phase's raw evidence records. The `REQ-commander-error-strategy`
row above (Epic 5) was amended in place, not duplicated, to record QUAL-04's resolution.

### New `deferred with reason` rows — this phase's open items, each with a named owner

| ID | Verdict | Evidence |
|---|---|---|
| QUAL-02 — `src/bin/paladin-server.rs` | deferred with reason | 0.00% at both entry and exit (145/145 missed lines, `03-coverage-measurement.md`). Closing it needs a testable `run()` seam extracted from `#[tokio::main] async fn main()` — a refactor outside a measurement phase's charter, per plan 03-06's recorded decision. **Owner: Phase 5 / VERIFY-05.** |
| QUAL-02 — `crates/paladin-storage/src/redis.rs` live-server paths | deferred with reason | Plan 03-05 closed the pure, connection-free seams (11 Docker-free unit tests, `redis.rs:1586` `mod tests`, exercised by `cargo test -p paladin-storage --offline --features redis-queue redis::` — 11 passed). The Docker-backed live-server integration paths remain unexercised: `command -v docker` returns exit 1 (absent) in this environment. **Owner: Phase 15 / PIPE.** |
| QUAL-02 — `crates/paladin-storage/src/minio.rs` | deferred with reason | No denominator entry in either the entry or exit `llvm-cov report` — the `s3` feature that gates its compilation is not part of the workspace default-feature set ADR-0006 measures under, so it is outside ADR-0006's recorded scope rather than a 0% file (`03-coverage-measurement.md` §Scope exclusions (b)). **Owner: VERIFY-05 / PIPE-02.** |
| PIPE-02 — ADR-0006's `--ignore-filename-regex` does not exclude `target/` | deferred with reason | `target/debug/build/utoipa-swagger-ui-10aed8599aeed486/out/embed.rs` (1 counted line, 1 missed line, 0.00%) enters both the entry and exit TOTAL row denominators — a generated build-script artifact, not first-party source. Magnitude: `1 / 62953 = 0.0016%` of the entry run's denominator — immaterial today at two decimals (`03-coverage-measurement.md` §Scope exclusions (a)). Not fixed here per D-02/D-06 — the regex is not amended by this phase. **Owner: VERIFY-05 / PIPE-02.** |
| QUAL-05 — Paladin-execution-loop and Arsenal-invocation benchmark suites | deferred with reason | Named by QUAL-05 as metric families the baseline should cover; neither has a shipped bench target — the Milestone-1 `paladin_benchmarks.rs` and `arsenal_benchmarks.rs` suites are absent from the tree, confirmed by `find` against `benches/` and every crate's `benches/` directory (`docs/src/appendix/performance-baseline.md` §Not produced by this run). Writing two new criterion suites is feature work inside a measurement phase, and a first run has nothing prior to compare against. **No owner is assigned in this phase**, stated plainly rather than left implied — Phase 3's own CONTEXT.md D-12 records the same disposition. |

### New `satisfied` rows — this phase's closures, each at the D-19 bar

| ID | Verdict | Evidence |
|---|---|---|
| QUAL-04 — MCP failure mode: expired/rejected bearer token | satisfied | `streamable_http_rejects_expired_bearer_token` (`tests/integration/mcp_streamable_http_test.rs:428`); re-run 2026-08-02 passing. |
| QUAL-04 — MCP failure mode: unknown tool | satisfied | `streamable_http_invoke_unknown_tool_maps_to_error` (`tests/integration/mcp_streamable_http_test.rs:450`); re-run 2026-08-02 passing. |
| QUAL-04 — MCP failure mode: bad arguments | satisfied | `streamable_http_invoke_with_missing_message_argument_maps_to_error` (`tests/integration/mcp_streamable_http_test.rs:492`); re-run 2026-08-02 passing. |
| QUAL-04 — MCP failure mode: malformed response | satisfied | `streamable_http_malformed_tools_list_response_returns_error` (`tests/integration/mcp_streamable_http_test.rs:534`); re-run 2026-08-02 passing. |
| QUAL-04 — MCP failure mode: handshake timeout | satisfied | `streamable_http_handshake_timeout_returns_timeout_error` (`tests/integration/mcp_streamable_http_test.rs:561`); re-run 2026-08-02 passing. All five re-run together 2026-08-02: `cargo test --offline --test lib -- mcp_streamable_http_test::streamable_http_rejects_expired_bearer_token mcp_streamable_http_test::streamable_http_invoke_unknown_tool_maps_to_error mcp_streamable_http_test::streamable_http_invoke_with_missing_message_argument_maps_to_error mcp_streamable_http_test::streamable_http_malformed_tools_list_response_returns_error mcp_streamable_http_test::streamable_http_handshake_timeout_returns_timeout_error` → `test result: ok. 5 passed; 0 failed; 0 ignored`. |
| QUAL-02 — `crates/paladin-storage/src/redis.rs` (pure seams) | satisfied | `crates/paladin-storage/src/redis.rs:1586` `mod tests`, 11 tests covering config defaults, all six key builders, priority-key collision-freedom, serialize/deserialize round-trip, error mapping and `get_priority_levels` order; exercised by `cargo test -p paladin-storage --offline --features redis-queue redis::`, re-run 2026-08-02: `test result: ok. 11 passed; 0 failed; 0 ignored`. Closes the entry run's largest zero-coverage first-party file to 34.69% at exit. |
| QUAL-02 — `crates/paladin-ports/src/output/file_storage_port.rs` | satisfied | `crates/paladin-ports/src/output/file_storage_port.rs:1463` `mod tests`, 19 tests; exercised by `cargo test -p paladin-ports --offline output::file_storage_port::tests::`, re-run 2026-08-02: `test result: ok. 19 passed; 0 failed; 0 ignored`. Closes 0.00% → 79.11% (exit). |
| QUAL-02 — `crates/paladin-ports/src/output/arsenal_port.rs` | satisfied | `crates/paladin-ports/src/output/arsenal_port.rs:891` `mod tests`, 2 tests exercising `ArsenalRegistry::list`'s default body; exercised by `cargo test -p paladin-ports --offline output::arsenal_port::tests::`, re-run 2026-08-02: `test result: ok. 2 passed; 0 failed; 0 ignored`. Closes 0.00% → 95.00% (exit). |
| QUAL-02 — `crates/paladin-llm/src/error.rs` | satisfied | `crates/paladin-llm/src/error.rs:86` `mod tests`, 11 tests giving `LlmProviderError`→`LlmError` its first caller across all 9 variants plus an exhaustiveness witness; exercised by `cargo test -p paladin-llm --offline error::tests::`, re-run 2026-08-02: `test result: ok. 11 passed; 0 failed; 0 ignored`. Closes 0.00% → 83.50% (exit). |
| QUAL-05 — Performance baseline document | satisfied | `docs/src/appendix/performance-baseline.md:3` `## Run — 2026-08-02` — throughput, P50/P95/P99 latency (derived from criterion's `sample.json` files), memory-per-Paladin (`examples/muster_baseline.rs`, 479 bytes via `/proc/self/status` VmRSS delta) and startup time, across the five shipped bench targets (`config_benchmarks`, `battalion_benchmarks`, `garrison_benchmarks`, `sanctum_benchmarks`, `llm_serialization_benchmarks`); the 2026-05-27 run retained in place under a superseded callout with its figures intact. |

## Phase 4 amendments (2026-08-03)

This file is amended in place again, per Phase 2/3's own convention: every row below is additive,
retains any superseded text, and carries a dated note naming its evidence. Every row is sourced from
a named Phase 4 plan's SUMMARY or from this phase's three raw-evidence records —
`.planning/phases/04-release-coherence/04-release-measurement.md`,
`04-advisory-findings.md`, and `04-ci-gate-deferrals.md` — never re-derived from memory. The four
rows amended in place above (`REQ-api-documentation`, `REQ-user-documentation`,
`REQ-deployment-artifacts`, `REQ-epic10-quality-gates`) are this phase's amendments too, cross-
referenced from here rather than repeated.

### REL-01 … REL-05 — this phase's own five requirements

| ID | Verdict | Evidence |
|---|---|---|
| REL-01 — version metadata converges on 0.7.0 | satisfied | All twelve manifests (root `Cargo.toml` + eleven member crates, including `crates/doc-examples`) and every internal `[workspace.dependencies]` pin, plus the one exact `=0.7.0` pin in `crates/paladin-ports/Cargo.toml`, converged via `cargo release version 0.7.0 --execute --no-confirm --workspace`; `cargo build --workspace --offline` exits 0 against the new pins. `CHANGELOG.md` carries a dated `## [0.7.0] - 2026-08-03` heading (former `## [Unreleased]` content) and a retroactively dated `## [0.6.0] - 2026-06-10` heading (`git log -S'## [0.6.0]' -- CHANGELOG.md` → commit `67b6207`). The human user confirmed `0.7.0` on 2026-08-03, local-only scope. Recorded as `ADR-0008`. Cite `04-release-measurement.md` §"Entry measurement — version convergence to 0.7.0" and §"...CHANGELOG finalize, tag deferral, and the human release gate"; `04-05-SUMMARY.md`. |
| REL-01 — the git tag and the push/publish sequence | deferred with reason | The annotated tag `v0.7.0` was deliberately **not** created inside the plan-04-05 worktree — a repo-global ref created on an ephemeral per-agent branch would orphan once that branch is deleted. The exact command (`git tag -a v0.7.0 -m "Release 0.7.0" <merged-commit-sha>`) is documented in `04-release-measurement.md` for the orchestrator's post-merge step. The push/publish sequence (`git push origin release/v0.7.0` then `git push origin v0.7.0`, the latter triggering `release.yml`'s ten-crate crates.io publish) is documented, ordered, and unexecuted, per D-03. **Owner: the orchestrator's post-merge step (tag creation); the human release gate, D-03 (push/publish).** |
| REL-02 — one Rust edition across the workspace | satisfied | All twelve workspace manifests declare `edition = "2024"` (the two stragglers, `crates/paladin-ports` and `crates/paladin-notifications`, bumped from `2021`); `cargo fix --edition` produced zero source rewrites for either crate. Both required build legs — `cargo build --workspace --offline` and `cargo build --workspace --no-default-features --offline` — exit 0. Recorded as `ADR-0009`. Cite `04-release-measurement.md` §"Entry measurement — edition 2024 on paladin-ports" and §"...on paladin-notifications (workspace now uniform)"; `04-01-SUMMARY.md`. |
| REL-03 — advisory posture measured and recorded | satisfied | `cargo audit` → 0 vulnerabilities (advisory DB commit `d91a8fc9`, 1186 advisories, fetched 2026-08-03); `cargo deny check` → `advisories ok, bans ok, licenses ok, sources ok`. The one stale suppression (`RUSTSEC-2025-0121`, gcc) removed, `deny.toml`'s ignore array 15 → 14 entries, 12 of 14 now carry a migration/review note. Cite `04-advisory-findings.md` §"Entry measurement — `cargo audit` verdict" and §"...`cargo deny check` verdict"; `04-02-SUMMARY.md`. |
| REL-04 — documentation review and the QUICKSTART measurement | satisfied | The documentation-review clause is discharged by citing RECON-08's recorded verdict (this file, §"Epic 10 Task 7.0 — dispute resolution") rather than inventing a review — no artifact anywhere in the 263-document corpus supplies a "Final Documentation Review." QUICKSTART's structurally-broken sample (wrong import path, missing `paladin-ai` dependency, wrong `paladin-llm` feature name, wrong constructor arity) was repaired and proven by an actual offline compile against the shipped tree, not eyeballed. The reachable in-workspace prefix measured 4 minutes 22 seconds; the target is settled at 15 minutes, reconciling `quickstart.md:3` with `introduction.md:9`. Cite `04-release-measurement.md` §"Entry measurement — QUICKSTART elapsed time (first measurement)"; `04-06-SUMMARY.md`. |
| REL-05 — `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --workspace`, doc tests, every example target | satisfied | **(Amended by Phase 4 exit re-measurement, dated 2026-08-03, citing `04-release-measurement.md` §"Exit re-measurement — full test suite at the final phase commit": the 2,924/0/122 figure below was measured at commit `d2898a3`, before plan 04-05's version bump, and this row originally presented it as if it still held. Re-running at the final tree exposed a real regression — `openapi::tests::openapi_matches_committed_baseline` failed because `crates/paladin-web/openapi.json`'s committed baseline still carried `0.6.0` while the spec generates its version from `CARGO_PKG_VERSION`. Fixed in `f57a34d` (one line, the version field). The suite re-run at `f57a34d` returns the same 2,924 passed / 0 failed / 122 ignored — now measured at the final phase commit rather than inherited from a pre-bump one. Found by `04-VERIFICATION.md` re-deriving rather than trusting the SUMMARY.)** `cargo fmt --all -- --check` green after one mechanical import-style fix (`d2898a3`); `cargo clippy --workspace --all-targets --all-features -- -D warnings` — 0 warnings; `cargo test --workspace --offline` — **2,924 passed / 0 failed / 122 ignored**; `cargo test --workspace --doc --exclude paladin-ports --offline` — **185 passed / 0 failed / 104 ignored** (the `paladin-ports` exclusion mirrors `ci.yml:225`, owned by DEBT-03/Phase 8 and HARD-07/Phase 10, not a concession this phase makes); every one of the 47 example targets under `examples/` builds, proven by a four-invocation feature matrix plus a binary-presence assertion (`comm -23` against the built set returns empty), closing the "22 examples" figure's staleness at source. **(Extended by Phase 4, dated 2026-08-03: the `Example Muster (Feature Matrix)` CI job subsequently EXECUTED on a clean GitHub runner and PASSED — the four-invocation matrix and the 47-binary assertion both held off this machine. This clause is now proven by execution, not only by local measurement, and its deferral row is superseded.)** Cite `04-release-measurement.md` §"Entry measurement — `cargo fmt --all -- --check`" through §"...every example target builds"; `04-04-SUMMARY.md`. |
| REL-05 — multi-arch Docker build within the 500 MB / 300 s budget | **measured, failed** (was: deferred with reason) | `ci.yml`'s `docker` job authored with `linux/amd64,linux/arm64` (via `docker/setup-qemu-action@v3`) and two hard-failing (`::error::` + `exit 1`) budget assertions; statically validated (YAML parses, action refs and `Dockerfile` path resolve). `docker` is absent from this environment — the job has never executed and neither budget has ever been measured against a built image. The only Docker build-time figure in the corpus, `PROJECT.md:767`'s "112 MB built in 5m31s," is single-arch and already over the 300 s figure this multi-arch job asserts — the 300 s gate is expected red on first real execution, and that red is the measurement REL-05 has never taken. Cite `04-ci-gate-deferrals.md` rows 1 and 5. **(Amended by Phase 4, dated 2026-08-03, citing `04-ci-gate-deferrals.md` §"Second CI execution": this row was written when `gh` was unauthenticated and no CI result was readable. The branch was subsequently pushed and the job EXECUTED. The image built successfully; the wall-clock assertion failed at **2946 s against the 300 s budget** — job duration 49m43s. The finding is that the budget is **mis-specified**, not merely unmet: the 500 MB / 5-minute figures derive from `PROJECT.md:767`'s single-arch "112 MB built in 5m31s", while SC5 demands a multi-arch build, and `linux/arm64` under QEMU emulation costs roughly an order of magnitude more. **The image-size budget remains unmeasured** — its assertion step follows the failing one and never ran. No budget was relaxed to obtain a pass.)** **Owner: Phase 15 / PIPE** (re-measure under native arm64 runners or cross-compilation, and settle whether the budget is single-arch-scoped). |
| REL-05 — kind-based Kubernetes smoke test within the 30 s pod-startup budget | deferred with reason | `ci.yml`'s `kubernetes-smoke` job authored reusing `integration-tests.yml`'s working kind/kubectl shape, with a hard-failing 30 s startup-time assertion; statically validated. `kind` and `kubectl` are both absent from this environment — the job has never executed and the budget has never been measured. `k8s/deployment.yaml:66-68` runs a placeholder `sleep 3600` with all readiness probes commented out (`:137-174`), so even once first executed the figure measures container scheduling, not application readiness. Cite `04-ci-gate-deferrals.md` row 2. **Owner: Phase 15 / PIPE** (first execution); **Owner: Phase 14 / WEB** (real readiness-probe wiring, row 3 below). |

### Phase 4 deferrals consolidated with named owners

Every row below is drawn from what plans 04-02, 04-03 and 04-06 already recorded — none re-derived —
per this phase's own prohibition against resolving a question this phase handed to a named owner.

| ID | Verdict | Evidence |
|---|---|---|
| REL-03 — four newly-surfaced advisories (`RUSTSEC-2021-0145` atty unsound, `RUSTSEC-2026-0221` event-listener unsound, `RUSTSEC-2026-0205` scc unsound, `spin` 0.9.8 yanked) | deferred with reason | Surfaced live by `cargo audit` against advisory-db snapshot `d91a8fc9` (1186 advisories); present in neither `deny.toml` nor `.cargo/audit.toml`. None fails either gate today. Dependency paths derived via `cargo tree --offline --invert`, recorded in `04-advisory-findings.md` §"Entry measurement — Four newly-surfaced advisories". Adding a suppression here would be a new governance decision inside a phase whose governance owner is someone else. **Owner: Phase 9 / SEC-01 and Phase 12 / SUPPLY-02.** |
| REL-03 — owner/expiry field schema for advisory suppressions, and the 2026-09-30 risk-acceptance disposition | deferred with reason | Neither `deny.toml` nor `.cargo/audit.toml` gained an owner or expiry field this phase (`git diff -- deny.toml \| grep -c '^+.*\(owner\|expiry\|expires\)'` → 0). Recorded in `04-advisory-findings.md` §"Not decided here". **Owner: Phase 9 / SEC-01** (the set and the expiry); **Phase 12 / SUPPLY-02** (the schema itself and the three unratified 2026 ignores). |
| REL-03 — duplicate `Security Audit` CI job (`ci.yml:60-77` `security-audit` vs. `ci.yml:389-406` `security`) | deferred with reason | Reproducing the `security` job's exact command (`cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`) locally exits 0 with the identical 13-warning set the bare `cargo audit` (`security-audit` job) produces — `--ignore` augments `.cargo/audit.toml` rather than replacing it, so neither job blocks SC5 on this tree. The 18-line deletion of the redundant job is the payoff for a Milestone 10 acceptance criterion Phase 12 owns, not performed here. Cite `04-advisory-findings.md` §"Entry measurement — The duplicate `Security Audit` CI job". **Owner: Phase 12 / SUPPLY-01.** |
| REL-03 — dual `reqwest` 0.12.28 / 0.13.4 exposure | satisfied | Already documented at `.planning/codebase/CONCERNS.md:293-303` (§"reqwest Dual Version Risk"); `deny.toml` treats duplicate versions as a warning by deliberate policy (FR 12 / Open Question 4). Not a RustSec advisory and does not fail SC3. Recorded here as a known, policy-accepted duplicate rather than a new document, per this plan's own Claude's-Discretion disposition — no owner needed, no suppression added. |
| REL-04 — the true clean-machine, cold-registry QUICKSTART timing | deferred with reason | crates.io returns HTTP 403 in this sandbox and the 0.7.0 crates are not published anywhere yet — no runner in this environment has the network route or the published artifact needed to time a genuine cold-start walkthrough. Cite `04-release-measurement.md` §"Entry measurement — QUICKSTART elapsed time (first measurement)". **Owner: Phase 15 / PIPE.** |
| REL-04 — live LLM execution of the repaired QUICKSTART sample | deferred with reason | No usable LLM API key exists in this environment (`OPENAI_API_KEY` empty, `LLM_API_KEY` a placeholder) — `OpenAIAdapter::from_env()?` reaching the real OpenAI API and returning the documented "Hello!" output cannot be attempted, let alone timed, without a real credential and network egress to `api.openai.com`. Cite `04-release-measurement.md` §"Entry measurement — QUICKSTART elapsed time (first measurement)". **Owner: Phase 15 / PIPE.** |
| REL-05 — real readiness-probe-based Kubernetes startup measurement | deferred with reason | `k8s/deployment.yaml:66-68` runs a placeholder `sleep 3600` with all three probes (liveness/readiness/startup) commented out (`:137-174`, "needs HTTP server endpoint"). Even once the `kubernetes-smoke` job first executes, its 30 s figure measures container scheduling, not application readiness, until real probes are wired. Wiring `paladin-web`'s health endpoints into the Deployment's probes is new product capability, outside this coherence phase's boundary. Cite `04-ci-gate-deferrals.md` row 3. **Owner: Phase 14 / WEB.** |
| REL-05 — CI actually running on a `release/**` push | deferred with reason | Plan 04-03 restored the `push:` trigger so `release/**` is now covered, but nothing in this sandbox can push a commit or dispatch a workflow run to observe it fire — `gh` is read-only-authorized here and not authenticated in this execution sandbox. First observation happens the next time a commit lands on `release/v0.7.0` and is pushed. Cite `04-ci-gate-deferrals.md` row 4. **Owner: the human gate that owns the tag push (D-03).** |
| REL-05 — whether `integration-tests.yml` and `feature-flags.yml` should also gain a `release/**` trigger | deferred with reason | Both carry the identical commented-out `push:` stanza `ci.yml` had before plan 04-03's edit. D-14 names `ci.yml` alone; the sibling files' modernization belongs to Phase 15's deprecated-actions sweep. Deliberately left untouched so a reader does not mistake the omission for an oversight. Cite `04-ci-gate-deferrals.md` row 6. **Owner: Phase 15 / PIPE-04.** |

## Verdict legend

| Verdict | Meaning |
|---|---|
| `satisfied` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `present, unproven` | `file:line` citation exists, but nothing exercises it |
| `genuinely outstanding` | No shipped code satisfies the requirement |
| `deferred with reason` | Explicitly deferred, with the deferring document and reason cited |
| `superseded by shipped code` | Shipped code answers the requirement differently than the ingested document specified, and the shipped answer is recorded as authoritative |

## Divergences — shipped code superseded an ingested requirement

> **This divergence is a documented non-goal that shipped anyway.** Epic 9 explicitly declared "no
> REPL or interactive shell" a non-goal (NG-7). An interactive REPL now ships. This is the corpus's
> own evidence for why nothing in this planning record is treated as locked — even an explicit,
> written non-goal was superseded by later work with no recorded decision reversing it.

| Requirement | Ingested position | Shipped position | Verdict |
|---|---|---|---|
| `REQ-cli-interactive-mode` (Epic 9 non-goal NG-7) | "No REPL or interactive shell" — explicitly out of scope | An interactive REPL ships in the Armory CLI | shipped as **an interactive REPL**, not the declared non-goal of no REPL at all; **superseded by shipped code** |
| `REQ-mcp-sse-transport` | `.project/Milestone_1-MVP/Epic_3/prd-arsenal-tool-system.md` FR-5 (lines 95-110) specifies `MCPSseAdapter` — "Uses Server-Sent Events (SSE) for receiving messages" | MCP ships on the official `rmcp` 2.1.0 SDK with a Streamable-HTTP transport: `pub struct MCPStreamableHttpAdapter` at `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs:76`; exercised by the passing test `streamable_http_round_trip_with_correct_bearer_token_succeeds` in `tests/integration/mcp_streamable_http_test.rs:176` | shipped as **Streamable-HTTP**, not the specified SSE transport; supersedes `REQ-mcp-sse-transport` (`REQUIREMENTS.md:2408` already records this as `Code diverges`); **superseded by shipped code** |
| `REQ-garrison-longterm-port`, `REQ-garrison-sqlite` | `.project/Milestone_1-MVP/Epic_2/prd-garrison-memory-system.md` specifies a `sqlite-vss` SQLite extension as Garrison's long-term/vector-search port | Semantic retrieval ships as **Sanctum** over **Qdrant** — `pub struct QdrantSanctumAdapter` at `crates/paladin-memory/src/sanctum/qdrant_adapter.rs:59` — plus an in-memory backend — `pub struct InMemorySanctum` at `crates/paladin-memory/src/sanctum/in_memory_adapter.rs:73`, exercised by the passing test `test_store_and_retrieve` in `tests/integration/in_memory_sanctum_tests.rs:38` (that file carries 0 `#[ignore]` attributes). The Qdrant-specific path is additionally covered by `test_store_and_retrieve` in `tests/integration/qdrant_sanctum_tests.rs:63`, but that test carries `#[ignore = "Requires Qdrant running on localhost:6334"]` at line 47 — **present, unproven** for the Qdrant-exerciser half specifically, not upgraded on the strength of the code existing. Missing coverage is supplied by run-2 requirements `REQ-sanctum-port`, `REQ-embedding-port`, `REQ-sanctum-domain-model` | shipped as **Sanctum**/**Qdrant**, not the specified `sqlite-vss` extension; supersedes `REQ-garrison-longterm-port` and `REQ-garrison-sqlite` (`REQUIREMENTS.md:2392,2394` already record these as `Code diverges`); **superseded by shipped code** |

All three rows carry a `file:line` citation plus a named test, example, or command that exercises
the shipped alternative, per the D-19 evidence bar, except where noted above (the Qdrant-specific
exerciser is `present, unproven` rather than upgraded on the strength of the code existing).

Plan 01-05 Task 1 resolved the RECON-08 Epic 10 Task 7.0 dispute below. The per-epic sections
further below are left as headings for plans 01-06 and 01-07 to fill with `REQ-*` rows; this plan
does not author any per-epic row.

## Epic 10 Task 7.0 — dispute resolution (RECON-08)

This section resolves the conflict recorded at `INGEST-CONFLICTS.md:125-127` ("Contradictory Epic 10
completion state"). Order matches that warning: task list first, validation report second.

**1. The task list's claim.**
`.project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md` marks all of its own
checklist items complete across parent tasks 0.0 through 6.0 and contains **no Task 7.0 heading
anywhere** (`grep -n "Task 7" tasks-epic10-validation-documentation.md` returns zero matches). Its
own item total, counted deterministically (`grep -c '^\s*- \[x\]'`), is **103** — every one of them
checked, zero unchecked. `REQUIREMENTS.md:2519` already records this epic heading as "(103/103
items; Task 7.0 disputed)".

**2. The validation report's claim.**
`.project/Milestone_1-MVP/Epic_10/task6.0-validation-report.md` states, at line 440: "Epic 10
progress: 101 of 102 subtasks (99%)" and at line 441: "Only Task 7.0 (Final Documentation Review)
remains" — and at line 533, in its closing line: "**Next Task:** Task 7.0 - Final Documentation
Review (6 subtasks)". The report never itemizes what those six subtasks are anywhere in its own
body; "(6 subtasks)" is the only detail given, with no subtask-description text to corroborate
elsewhere.

**3. Where both documents make the same claim about the same parent task (Task 6.0), side by side.**
The task list marks `- [x] 6.0 Validation & Quality Assurance (All FRs)` complete, with all sixteen
of its own subtasks (6.1-6.16) individually checked. The validation report's own title and status
line assert the identical claim independently: "**Task:** Task 6.0 - Validation & Quality Assurance
Report … **Status:** ✅ **COMPLETE**" and "Task 6.0 - Validation & Quality Assurance has been
successfully completed with 16 of 16 subtasks (100%) validated." **The two documents agree on Task
6.0** — this is not the disputed claim. The dispute is entirely about whether a Task 7.0 exists
beyond it, which only the validation report asserts.

**4. The search record.** Commands run against this worktree and their results:

```
$ ls .project/Milestone_1-MVP/Epic_10/
epic10.md  prd-epic10-validation-documentation.md  task5.0-completion-summary.md
task6.0-validation-report.md  tasks-epic10-validation-documentation.md
# 5 files. No dedicated "Final Documentation Review" or "Task 7.0" artifact of any kind.

$ grep -rn "Final Documentation Review" .project/
task6.0-validation-report.md:441 and :533 — the only two hits in all 263 corpus documents.

$ grep -rn "Final Documentation Review" docs/
# 0 matches.

$ grep -rniE "documentation review|documentation sign-off|final review checklist" .project/
# Matches only in unrelated documents: this same Epic 10 PRD's own aspirational Phase-6 checklist
# ("Conduct documentation review with fresh eyes", prd-epic10-validation-documentation.md:655) and
# Acceptance Criteria ("Documentation review completed by technical writer (if available)", :694) —
# both pre-execution PRD checklist items, not evidence a Task 7.0 was ever executed or scoped as a
# distinct numbered task — plus incidental hits in Milestone 11 and Milestone 2 documents unrelated
# to Epic 10.

$ grep -rniE "documentation review|documentation sign-off|final review checklist" docs/
docs/src/appendix/contributing-legacy.md:324 — "Documentation Review: Check docs are clear", a
generic contributing-guide checklist item with no connection to Epic 10 or Task 7.0.

$ grep -rln "Task 7.0" .project/Milestone_1-MVP/
task6.0-validation-report.md — the only file in the whole milestone that mentions "Task 7.0".

$ grep -c "Final Documentation Review" .project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md
0

$ grep -c '^\s*- \[x\]' .project/Milestone_1-MVP/Epic_10/tasks-epic10-validation-documentation.md
103
```

Cross-checked against `.planning/intel/task-completion-state.md`'s deterministic Milestone_1-MVP
"Open items by list" breakdown: Epic 10's task list is **absent** from that list (only eight other
Epic/workstream task lists are named there with nonzero open items), which independently confirms
Epic 10's deterministic open-item count is **0** — corroborating the task list's own 103/103 claim
via a route that does not depend on reading the task list itself.

No trace of "Final Documentation Review" content, a six-subtask breakdown, or any artifact
resembling a documentation sign-off exists anywhere under `.project/Milestone_1-MVP/Epic_10/`,
the rest of `.project/Milestone_1-MVP/`, or `docs/`. The absence is the evidence.

**5. The verdict.** **The validation report is recorded as wrong.** The task list is the
corroborated document: it is internally complete (103/103, no Task 7.0), independently corroborated
by the deterministic checkbox count in `task-completion-state.md`, and no artifact anywhere in the
263-document corpus or the shipped tree supplies content for a Task 7.0 "Final Documentation Review"
of any kind. Epic 10's completion status is classified **`satisfied`** on this point — there is no
outstanding "Final Documentation Review" work item, named or otherwise, and no owner is assigned
because none is needed. Plans 01-06/01-07, when authoring the Epic 10 per-epic `REQ-*` section, use
this verdict rather than re-opening the dispute.

**6. The 102-vs-103 explanation.** The task list's own total is **103** (all of tasks 0.0-6.0,
verbatim from the deterministic checkbox count above). The validation report's total is **102**
("101 of 102 subtasks"), naming a Task 7.0 the task list never contained. The two totals cannot both
describe the same underlying set: if a real six-subtask Task 7.0 existed on top of the task list's
103 checked items, the combined total would be at least 109 (103 + 6), not 102; if the validation
report's 102 is meant to describe tasks 0.0-6.0 alone, that also does not match the task list's own
103. Under neither reading does 102 reconcile against anything the task list actually contains. The
102 figure is therefore not a re-derivation of the task list's total with six items subtracted or
added — it is an unreconciled number, consistent with the "Task 7.0" claim itself being fabricated
rather than a real, differently-counted view of the same work. This ledger uses **103** — the task
list's deterministic, independently-corroborated total — going forward.

## Ingest bookkeeping corrections (RECON-01)

### Battalion base module path

`INGEST-CONFLICTS.md:130-134` ("Contradictory Battalion base module path") records that
`.project/Milestone_1-MVP/Epic_4/epic4.md` names the Battalion base module `battalion/mod.rs`,
matching Appendix B of the project plan, while `.project/Milestone_1-MVP/Paladin Project Completion
Plan.md` names it `battalion/battalion.rs` in its own Epic 4 technical-design section —
contradicting its own Appendix B.

The code-observed answer, confirmed by listing the directory directly:

```
$ ls crates/paladin-core/src/platform/container/battalion/
campaign.rs  chain_of_command.rs  conclave.rs  council.rs  formation.rs  grove.rs  mod.rs  phalanx.rs
```

`crates/paladin-core/src/platform/container/battalion/mod.rs` **exists**;
`crates/paladin-core/src/platform/container/battalion/battalion.rs` **does not**. Two of the three
references (`epic4.md` and both instances of Appendix B) were already right. **The Epic 4 technical
design section of `Paladin Project Completion Plan.md` is the corrected document** — its
`battalion/battalion.rs` reference is wrong and its own Appendix B already disagreed with it.

### Requirement-count discrepancy

Counting the `REQ-*` rows in `REQUIREMENTS.md`'s `## Milestone 1 as-shipped ledger` section
deterministically — a grep over the text between that heading (`REQUIREMENTS.md:2361`) and the next
`##` heading (`REQUIREMENTS.md:2542`, `## Milestone 2-3 as-shipped ledger`) — gives:

```
$ awk '/^## Milestone 1 as-shipped ledger/{flag=1; next} /^## /{if(flag){exit}} flag' REQUIREMENTS.md | grep -c '^| REQ-'
112
```

Every other total this corpus reports for the same nominal set:

- `REQUIREMENTS.md:31` (the file's own "How to read this file" summary table): "**Milestone 1 as-shipped ledger** | All 115 run-1 requirement IDs, with status. Not forward scope."
- `.planning/intel/SYNTHESIS.md:72`: "Requirements extracted: 348 cumulative (run 1: **115**, run 2: 118, run 3: 115)."
- A third, independent cross-check — counting the distinct `## REQ-*` headings in the run-1 section of `.planning/intel/requirements.md` (before the run-2 `MODE=merge` marker at line 1195) — also returns **115**.

**112 enumerated ledger rows, 115 reported IDs. The difference is exactly three, and it is explained
by how competing-variant pairs are recorded, not by any ID actually missing.** Three of the 115
ingested run-1 IDs — `REQ-herald-trait-v2`, `REQ-temperature-range-v2`, `REQ-test-coverage-target-v2`
— are each the "-v2" half of a competing-variant pair whose "-v1" half already occupies its own row
in the as-shipped ledger:

- `REQ-temperature-range-v1 / -v2` — `REQUIREMENTS.md:2469`
- `REQ-herald-trait-v1 / -v2` — `REQUIREMENTS.md:2490`
- `REQ-test-coverage-target-v1 / -v2` — `REQUIREMENTS.md:2537`

Both IDs of each pair are genuinely present in the file — the "-v2" half is fully described in
`## Competing variants (preserved unmerged)` (`REQUIREMENTS.md:1661,1677,1768`) — but the ledger
folds each pair into a single `| REQ-X-v1 / -v2 | Variant (group N) |` row rather than giving the
"-v2" ID a distinct grep-matchable row of its own. So a literal `grep -c '^| REQ-'` reads 112 while
"all 115 IDs are accounted for" is also true; the two figures measure different things (distinct
ledger rows vs. distinct requirement IDs), not competing counts of the same thing, and neither is
wrong. **This ledger uses 112 for "number of ledger rows in the Milestone 1 as-shipped ledger" and
115 for "number of distinct run-1 requirement IDs" going forward — the two labels are not
interchangeable, and a future reference to "the Milestone 1 requirement count" must say which one
it means.**

### Epic 1 — Paladin Domain Foundation

No open task items (182/182 complete per `intel/task-completion-state.md`) — every row below carries
no nested block.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-paladin-entity | satisfied | `PaladinData` struct at `crates/paladin-core/src/platform/container/paladin.rs:142`, `Paladin` type alias at `paladin.rs:229`; exercised by `test_paladin_data_default` (`paladin.rs:350`) |
| REQ-paladin-builder | satisfied | `PaladinBuilder` struct at `src/application/services/paladin/paladin_builder.rs:76`; exercised by `test_builder_validation_empty_prompt` (`paladin_builder.rs:1346`) |
| REQ-paladin-config | satisfied | `PaladinConfig` struct at `crates/paladin-core/src/platform/container/paladin_config.rs:44`; exercised by `test_paladin_config_defaults` (`paladin_config.rs:173`) |
| REQ-paladin-port | satisfied | `PaladinPort` trait (`execute`/`execute_stream`) at `crates/paladin-ports/src/output/paladin_port.rs:631,752`; exercised end-to-end through `IntegrationMockPaladinPort`'s trait impl (`tests/integration/commander_integration_tests.rs:78`) by `test_commander_executes_formation_end_to_end` (`commander_integration_tests.rs:150`) |
| REQ-paladin-execution-service | satisfied | `PaladinExecutionService` struct at `src/application/services/paladin/paladin_execution_service.rs:105`, `execute()` at `:470`; exercised by `test_paladin_without_garrison_single_turn` (`tests/integration/paladin_garrison_integration_test.rs:143`), which constructs the service directly and asserts `execute()` succeeds |
| REQ-paladin-error-handling | satisfied | `PaladinError` enum at `crates/paladin-core/src/platform/container/paladin_error.rs:19`; exercised by `test_is_retryable` (`paladin_error.rs:100`) and `test_garrison_error_conversion` (`:116`) |
| REQ-paladin-observability | present, unproven | Code uses `log`/`env_logger` (`use log::{debug, error, info, warn};` at `paladin_execution_service.rs:69`) alongside the workspace's `tracing-subscriber` dependency (`Cargo.toml:120`) — the same divergence `REQUIREMENTS.md:2541` already recorded (PRD specified `tracing`, code uses `log`). Logging calls are real and present at the citation, but no named test asserts log output content, so the exerciser half of the bar is unmet |
| REQ-paladin-testing-infra | satisfied | `MockLlmAdapter` at `crates/paladin-llm/src/mock.rs:73` (and the parallel `tests/helpers/mock_llm_adapter.rs:66`); exercised by `test_mock_returns_default_response` (`crates/paladin-llm/src/mock.rs:412`) and used throughout the integration suite (e.g. `paladin_garrison_integration_test.rs`) |

### Epic 2 — Garrison Memory System

4 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_2/tasks-garrison-memory-system.md`.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-garrison-entry | satisfied | `GarrisonEntry` struct at `crates/paladin-core/src/platform/container/garrison.rs:41`; exercised by `test_garrison_entry_creation` (`garrison.rs:422`) |
| REQ-garrison-windowing | satisfied | `ConversationHistory` windowing logic in `garrison.rs`; exercised by `test_conversation_history_windowing_by_count` (`garrison.rs:490`) |
| | | **Nested outstanding item:** `- [ ] 9.14 Write test: \`test_large_conversation_performance\` - benchmark with 1000 entries (future enhancement)` (`tasks-garrison-memory-system.md:222`) — **deferred with reason**. `REQUIREMENTS.md:2549` already records this exact test as "deferred → v2", and `STATE.md`'s Deferred Items table records it "Deferred — marked future enhancement" (Ingest run 1). No code change is expected here; the deferral is the task list's own words. |
| REQ-garrison-port | satisfied | `GarrisonPort` trait at `crates/paladin-ports/src/output/garrison_port.rs:380`; exercised via `InMemoryGarrison`'s trait impl by `test_remember_and_recall` (`crates/paladin-memory/src/garrison/in_memory_garrison.rs:229`) |
| REQ-garrison-longterm-port | superseded by shipped code | See the Divergences table above (`REQ-garrison-longterm-port`, `REQ-garrison-sqlite` row) — semantic retrieval ships as Sanctum/Qdrant, not a `sqlite-vss` extension of this port. Not re-decided here. |
| REQ-garrison-in-memory | satisfied | `InMemoryGarrison` struct at `crates/paladin-memory/src/garrison/in_memory_garrison.rs:58`; exercised by `test_remember_and_recall` (`:229`) and `test_importance_based_eviction` (`:354`) |
| REQ-garrison-sqlite | satisfied | The SQLite Garrison adapter itself shipped as specified: `SqliteGarrison` struct at `crates/paladin-memory/src/garrison/sqlite_garrison.rs:52`; exercised by `test_sqlite_remember_and_recall` (`sqlite_garrison.rs:521`) and `test_sqlite_persistence` (`:537`). Only the `sqlite-vss` **vector-search** extension diverged — that half is recorded in the Divergences table above (superseded by Sanctum/Qdrant), not repeated here as a contradiction. |
| REQ-garrison-paladin-integration | satisfied | Exercised by `test_paladin_multi_turn_conversation` (`tests/integration/paladin_garrison_integration_test.rs:169`) and `test_paladin_without_garrison_single_turn` (`:143`) |
| REQ-garrison-config | satisfied | `GarrisonSettings` struct at `crates/paladin-memory/src/config/garrison.rs:11`; exercised by `test_garrison_settings_validation_success` (`:100`) |
| REQ-garrison-errors | satisfied | `GarrisonError` enum at `crates/paladin-core/src/platform/container/garrison_error.rs:8`; exercised by `test_storage_error_display` (`:51`) |
| REQ-garrison-testing | satisfied | **Amended 2026-08-01, plan 02-09, citing plan 02-08's SUMMARY.** The bulk of the Garrison testing infrastructure is real and passing (all rows above cite live tests; the task list's own annotation records "19 total: 12 paladin_garrison + 7 sqlite_garrison" integration tests), and the closure claim the two nested items below once left unconfirmed is now closed by a real, written PRD-acceptance review: `.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md` (50 criterion rows: 37 satisfied / 9 superseded / 2 genuinely outstanding / 2 present-unproven), which is itself the citable artifact this row previously lacked. |
| | | **Nested outstanding item:** `- [ ] 11.0 Final Validation and Cleanup` (`tasks-garrison-memory-system.md:246`) — **satisfied** (parent). Four of its six children (11.1-11.4: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build --release`) were already marked done; the two remaining children (11.5, 11.6, below) are both now closed by plan 02-08. |
| | | **Nested outstanding item:** `- [ ] 11.5 Verify test coverage ≥ 80% using \`cargo llvm-cov\`` (`tasks-garrison-memory-system.md:251`) — **Amended 2026-08-01, plan 02-09, citing plan 02-08's SUMMARY: superseded by shipped code**. [ADR-0006](../decisions/0006-coverage-gate.md) replaced this per-module 80% target with a single workspace-wide 84% floor (measured 84.79%); re-measuring Garrison alone would reintroduce the second coverage scope ADR-0006 exists to eliminate, per D-04. Forward owner for the workspace-wide figure: **QUAL-01**. |
| | | **Nested outstanding item:** `- [ ] 11.6 Review all acceptance criteria from PRD - ensure all met` (`tasks-garrison-memory-system.md:252`) — **Amended 2026-08-01, plan 02-09, citing plan 02-08's SUMMARY: satisfied**. `.planning/phases/02-functional-gap-closure/02-garrison-prd-review.md` is exactly this review — 50 rows, one verdict per PRD criterion at the D-19 evidence bar, closing the "no PRD-acceptance review artifact exists for Epic 2" gap this row previously recorded. |
| | | **New finding (plan 02-08's review, recorded here per its own instruction):** `PaladinError::GarrisonRequired` (`crates/paladin-core/src/platform/container/paladin_error.rs:54`, matched in `is_terminal()` at `:78`) — **genuinely outstanding**. The variant is defined and unit-tested (`:111`) but never *constructed* anywhere in the tree (`grep -rn "GarrisonRequired" src crates` shows only the definition, its own test, and one unrelated routing `match` arm in `conclave_execution_service.rs:364`); no code path detects "multi-turn attempted without Garrison" — execution simply proceeds with an empty history. **Forward owner: the v2 backlog (candidate)** — no existing GAP-*/QUAL-* requirement names this construction path; parallels this ledger's `LlmProviderError` dead-conversion finding (`02-test-wiring-sweep.md`) in shape. |
| | | **New finding (plan 02-08's review, recorded here per its own instruction):** `GarrisonSettings::validate()` (`crates/paladin-memory/src/config/garrison.rs:46`) — **present, unproven**. The method exists and is directly unit-tested, but returns `Result<(), String>` (not `Result<(), GarrisonError>`) and has zero call sites in the `Settings`-loading path (`grep -rn "\.validate()" src` confirms) — an actually-invalid `garrison:` config block currently produces no error at load time, so the PRD's literal claim (invalid config → `GarrisonError::Configuration` on load) is unexercised end-to-end. **Forward owner: the v2 backlog (candidate)**, same reasoning as the row above. |

### Epic 3 — Arsenal Tool System

3 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_3/tasks-arsenal-tool-system.md`.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-arsenal-domain-types | satisfied | `Armament`/`ArmamentCall`/`ArmamentResult` at `crates/paladin-core/src/platform/container/arsenal/core.rs:17,36,80`; exercised by `invoke_routes_to_the_registered_client_and_returns_real_output` (`src/application/services/arsenal/arsenal_execution_service.rs:303`) |
| REQ-arsenal-port | satisfied | `ArsenalPort` trait at `crates/paladin-ports/src/output/arsenal_port.rs:470`; exercised via `MockArsenalAdapter`'s trait impl by `test_mock_arsenal_invoke_success` (`tests/helpers/mock_arsenal_adapter.rs:246`). This upgrades the 2026-01 "services untested → QUAL-02" note (`REQUIREMENTS.md:2564`): `arsenal_execution_service.rs` now carries a full passing test module (re-verified 2026-07-31). |
| REQ-mcp-protocol | superseded by shipped code | `MCPClient` at `src/infrastructure/adapters/arsenal/mcp_protocol.rs:62` is, by its own doc comment (`mcp_protocol.rs:1,12-16`), "a thin facade over `rmcp::service::RunningService`" — the official `rmcp` 2.1.0 SDK performs the handshake, superseding the hand-rolled JSON-RPC client the Epic 3 PRD specified. Exercised by the passing test suite starting at `mcp_protocol.rs:370`. Same class of divergence as the `REQ-mcp-sse-transport` row already recorded in the Divergences table above, but this specific ID was not itself in that table — recorded here instead. |
| REQ-mcp-stdio-transport | satisfied | `MCPStdioAdapter` at `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs:34`, built on `rmcp::transport::TokioChildProcess`; exercised by `test_stdio_connect` (`tests/integration/mcp_stdio_test.rs:23`) and `test_stdio_invoke_tool_calculator` (`:110`) |
| REQ-mcp-sse-transport | superseded by shipped code | See the Divergences table above — shipped as Streamable-HTTP (`MCPStreamableHttpAdapter`), not SSE. Not re-decided here. |
| REQ-arsenal-builder-integration | satisfied | `PaladinBuilder::with_arsenal_registry` at `src/application/services/paladin/paladin_builder.rs:685`; exercised by `test_builder_auto_registers_handoff_tool_when_configured` (`paladin_builder.rs:2098`) |
| REQ-arsenal-resource-controls | satisfied | `TimeoutWrapper`/`ConcurrencyLimiter` at `src/infrastructure/adapters/arsenal/resource_controls.rs:53,160`; exercised by `test_concurrency_limit_enforced` (`resource_controls.rs:280`) |
| REQ-arsenal-resilience | satisfied | `ArsenalError::ToolNotFound` failure path exercised by `invoke_with_no_serving_client_returns_tool_not_found` (`arsenal_execution_service.rs:341`); `grep -c '#\[ignore' src/infrastructure/adapters/arsenal/*.rs src/application/services/arsenal/*.rs` returns 0. This upgrades the 2026-01 "Partial → QUAL-04 (failure paths untested)" note (`REQUIREMENTS.md:2570`) — the failure paths are tested and none is `#[ignore]`d, re-verified 2026-07-31. |
| REQ-arsenal-context-injection | satisfied | Exercised by `test_tool_invocation_and_injection` (`tests/integration/context_injection_test.rs:324`) and `test_paladin_continues_after_tool_failure` (`:399`) |
| | | **Nested outstanding item:** `- [ ] 9.30 Commit all changes with message: "feat: implement Arsenal Tool System (Epic 3)"` (`tasks-arsenal-tool-system.md:302`) — **superseded by shipped code**. A git-workflow step, not a functional requirement; no requirement-bearing row above is a closer semantic match, so it is recorded here against the epic's last row. The literal commit message this text describes has no discoverable trace as its own commit, but the deliverable it describes — the Arsenal Tool System — is fully present and compiles in the current `release/v0.7.0` tree (every citation above resolves against it), which is what "shipped" means for a housekeeping step whose only purpose was landing the code. |
| | | **Nested outstanding item:** `- [ ] 9.31 Push feature branch: \`git push -u origin feature/epic3-arsenal-tool-system\`` (`tasks-arsenal-tool-system.md:303`) — **superseded by shipped code**. Same reasoning as 9.30 above: the named feature branch has no discoverable trace, but the code it would have carried already ships on `release/v0.7.0`. |
| | | **Nested outstanding item:** `- [ ] 9.31 Push feature branch: \`git push -u origin feature/epic3-arsenal-tool-system\`` (`tasks-arsenal-tool-system.md:304` — the source file literally duplicates line 303 verbatim at this line, a defect in the source document itself, not a second distinct task) — **superseded by shipped code**, same reasoning. |

### Epic 4 — Battalion Orchestration

2 open task items per `intel/task-completion-state.md` (parent tasks 6.0 and 7.0), both under
`.project/Milestone_1-MVP/Epic_4/tasks-battalion-orchestration.md`. Rows whose subject is
`BattalionConfig`, `BattalionResult` or the Formation minimum Paladin count link to
[`ADR-0001`](../decisions/0001-battalion-config.md), [`ADR-0002`](../decisions/0002-battalion-result.md)
and [`ADR-0003`](../decisions/0003-formation-min-paladins.md) respectively rather than re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-battalion-config-v1 | satisfied | See [ADR-0001](../decisions/0001-battalion-config.md). `BattalionConfig` struct at `crates/paladin-core/src/platform/container/battalion/mod.rs:37` is this exact field set (ADR-0001's Considered Options: "chosen"); exercised by `test_battalion_config_builder` (`battalion/mod.rs:886`). ADR-0001's "must change" conformance note applies only to the unrelated placeholder struct at `citadel.rs:280` (renamed to `BattalionCheckpointConfig` by GAP-07) — this row's struct is unaffected and already conforms. |
| REQ-battalion-config-v2 | superseded by shipped code | See [ADR-0001](../decisions/0001-battalion-config.md), which records this variant "rejected; not what shipped" — its `retry_attempts: u32` and `enable_checkpointing: bool` fields are absent from the tree, and the `description` field it proposed dropping was in fact kept. Not re-decided here. |
| REQ-battalion-result-v1 | superseded by shipped code | See [ADR-0002](../decisions/0002-battalion-result.md). The shipped `BattalionResult` (`battalion/mod.rs:549`) is a merged superset in which every field this Epic 4 position specified — `battalion_id`, `battalion_name`, timestamps, `final_output`, `paladin_results`, `status`, and per-Paladin and overall timing — is present, so this position contributed no field the shipped struct dropped and there is no substitution to record; ADR-0002's phrasing for that is "wholly subsumed". Not re-decided per-epic. |
| REQ-battalion-error-strategy | satisfied | `AggregatedError` at `crates/paladin-battalion/src/error_aggregation.rs:13`; exercised by `test_add_error` (`error_aggregation.rs:186`) |
| REQ-battalion-retry-policy | satisfied | `RetryPolicy` struct at `battalion/mod.rs:189`, `calculate_retry_delay` at `crates/paladin-battalion/src/retry.rs:40`; exercised by `test_calculate_retry_delay_linear` (`retry.rs:115`) |
| REQ-formation-min-paladins-v1 | satisfied | See [ADR-0003](../decisions/0003-formation-min-paladins.md). `Formation::validate` at `crates/paladin-core/src/platform/container/battalion/formation.rs:109-111` currently rejects fewer than 2 Paladins, exactly this row's specification; exercised by `test_formation_validation_minimum_paladins` (`formation.rs:173`). ADR-0003 decides this bound **will relax to 1** — Formation's own rejection contradicts the Commander's passing `test_auto_selects_formation_for_single_paladin` (`crates/paladin-battalion/src/commander.rs:1912`), which routes a single Paladin to Formation. Per ADR-0003's "must change" conformance, this row records the code as it stands today; the change is owned by **GAP-07**, not asserted here as already done. |
| REQ-formation-construction | satisfied | Exercised by `test_formation_creation_valid` (`formation.rs:163`) |
| REQ-formation-execution | satisfied | Exercised by `test_sequential_execution_success` (`crates/paladin-battalion/src/formation_service.rs:472`) |
| REQ-formation-output | satisfied | Exercised by `test_output_passing_between_paladins` (`formation_service.rs:493`) |
| REQ-phalanx-construction | satisfied | Exercised by `test_phalanx_creation_valid` (`crates/paladin-core/src/platform/container/battalion/phalanx.rs:177`) |
| REQ-phalanx-concurrency | satisfied | `ConcurrencyLimiter`-backed `max_concurrency` exercised by `test_collect_all_with_concurrency_limit` (`crates/paladin-battalion/src/phalanx_service.rs:613`) and validated under real load by `test_load_phalanx_concurrent_execution` (`tests/integration/battalion/load_test.rs:192`) and `test_stress_high_concurrency_limit` (`load_test.rs:273`). This upgrades the 2026-01 "Partial → GAP-02 (concurrency claims unvalidated)" note (`REQUIREMENTS.md:2586`) — concurrency is now validated under load, re-verified 2026-07-31. |
| REQ-phalanx-aggregation | satisfied | Exercised by `test_collect_all_strategy_success` (`phalanx_service.rs:593`) |
| REQ-campaign-graph | satisfied | `EdgeCondition` enum and `CampaignEdge` struct at `crates/paladin-core/src/platform/container/battalion/campaign.rs:34,50` (petgraph-backed); exercised by `test_add_edge_success` (`campaign.rs:349`) |
| REQ-campaign-edge-conditions | satisfied | `EdgeCondition::Always` exercised end-to-end by `test_branching_campaign_fan_out` (`tests/integration/battalion_campaign_integration_test.rs:154`) |
| REQ-campaign-execution | satisfied | `CampaignExecutionService::execute` at `crates/paladin-battalion/src/campaign_service.rs:104`; exercised end-to-end by `test_linear_campaign_execution` (`tests/integration/battalion_campaign_integration_test.rs:121`). This upgrades the 2026-01 "Verify → QUAL-02 (`campaign_service.rs` at 4.26% coverage)" note (`REQUIREMENTS.md:2590`) — that figure measured only in-crate unit tests (`campaign_service.rs`'s own `#[cfg(test)]` module has just `test_service_creation`), but the integration suite exercises `execute()` directly and passes with 0 `#[ignore]`s, re-verified 2026-07-31. The low unit-file coverage figure itself is not re-measured here (that is Phase 1's coverage-measurement plan); this row only re-verifies that a real, passing, non-ignored exerciser exists. |
| REQ-chain-of-command-construction | satisfied | `ChainOfCommand` struct at `crates/paladin-core/src/platform/container/battalion/chain_of_command.rs:64`; exercised by `test_chain_of_command_new_with_valid_setup` (`tests/unit/battalion/chain_of_command_tests.rs:34`) |
| REQ-chain-of-command-execution | satisfied | `ChainOfCommandExecutionService::execute` at `crates/paladin-battalion/src/chain_of_command_service.rs:125`; exercised end-to-end by `test_commander_executes_chain_of_command_end_to_end` (`tests/integration/commander_integration_tests.rs:283`). This upgrades the 2026-01 "Verify → GAP-01" note (`REQUIREMENTS.md:2591`) — GAP-01's own description already stated shipped code contains `chain_of_command_service.rs`; a full run of `cargo test --test lib chain_of_command` on 2026-07-31 shows 54 passed, 0 failed, 0 ignored. |
| REQ-chain-of-command-aggregation | satisfied | Exercised by `test_broadcast_executes_all_specialists` (`tests/unit/battalion/chain_of_command_service_tests.rs:302`), which aggregates concurrent specialist results |
| REQ-battalion-status | satisfied | `BattalionStatus` enum at `battalion/mod.rs:471`; exercised by `test_sequential_execution_success` (`formation_service.rs:472`), which asserts `battalion_result.status == BattalionStatus::Completed` |
| REQ-battalion-logging | present, unproven | `log::info!`/`warn!` calls exist at `formation_service.rs:58,156,173` and the equivalent call sites in `phalanx_service.rs`, `campaign_service.rs` and `chain_of_command_service.rs`; no named test asserts log output content, so the exerciser half of the bar is unmet |
| REQ-battalion-cancellation | satisfied | **Amended 2026-08-01, plan 02-09, citing [ADR-0007](../decisions/0007-battalion-cancellation-deferral.md).** `CancellationToken`-based `execute_with_cancellation` at `phalanx_service.rs:151`, exercised by `test_cancellation_support` (`phalanx_service.rs:758`) — satisfied for **Phalanx**, which ADR-0007 records as the requirement's shipped scope. The other three patterns are addressed in the nested row below, split out per ADR-0007's own "split the row" instruction rather than averaged into one verdict. |
| | | **Nested item (ADR-0007 split): Formation, Campaign and ChainOfCommand cancellation** — **deferred with reason**, citing [ADR-0007](../decisions/0007-battalion-cancellation-deferral.md) as the deferring authority. Verified 2026-08-01: `grep -rn "execute_with_cancellation\|CancellationToken" crates/paladin-battalion/src/{formation_service.rs,campaign_service.rs,chain_of_command_service.rs,commander.rs}` returns zero matches — no cancellation entry point exists for any of the three, not merely an untested one. Building it is new capability (a cancellation contract across three more services, including two open design questions — what a cancelled run returns mid-DAG in Campaign and mid-delegation in ChainOfCommand), not gap closure; ROADMAP Phase 2's five success criteria never mention cancellation, corroborating that this was never Phase 2's scope. **Forward owner: the v2 backlog, gated on a recorded cancellation-contract decision** (ADR-0007's own named prerequisite). |
| | | **Nested outstanding item:** `- [ ] 6.0 Implement Chain of Command Pattern (Phase 2 - Hierarchical Delegation)` (`tasks-battalion-orchestration.md:258`) — **satisfied** (parent checkbox stale). All 42 of its own subtasks (6.1-6.42) are individually checked, and direct re-verification confirms the code: `ChainOfCommand` (`chain_of_command.rs:64`), `ChainOfCommandExecutionService` (`chain_of_command_service.rs`), and a full `cargo test --test lib chain_of_command` run on 2026-07-31 passing 54/54 with 0 ignored (see `REQ-chain-of-command-execution` above). This is the same "stale parent over complete subtasks" shape `REQUIREMENTS.md:2573` already flagged in its own heading ("tasks 6.0 and 7.0 open"), and matches the pattern Milestone 1 run 1 already found for this exact epic (Chain of Command wiring existed despite the January task list marking it incomplete). |
| | | **Nested outstanding item:** `- [ ] 7.0 Integration Testing, Performance Validation & Documentation` (`tasks-battalion-orchestration.md:302`) — **satisfied** (parent checkbox stale). All 22 of its own subtasks (7.1-7.22) are individually checked; re-verified: `tests/integration/battalion/load_test.rs` exists with 5 real, non-`#[ignore]`d load/stress tests (`test_load_formation_50_concurrent_battalions` at `:102`, `test_load_phalanx_concurrent_execution` at `:192`, `test_stress_high_concurrency_limit` at `:273`), `crates/paladin-battalion/benches/battalion_benchmarks.rs` exists, and `examples/chain_of_command_delegation.rs` exists and compiles. `docs/BATTALION.md` (7.9's literal path) does not exist at that path, but its content shipped as `docs/src/appendix/battalion-patterns-guide.md` — the same mdbook relocation pattern `PROJECT.md`'s ARCH-05 already records for other Milestone-1 docs deliverables (relocated, not missing). |

**Planning-input correction (2026-08-01, plan 02-09):** [ADR-0001](../decisions/0001-battalion-config.md)'s
`Code Locations` section, cited by `REQ-battalion-config-v1` above, omitted two of the six files the
`citadel.rs` checkpoint-config rename actually touched. Plan 02-03 found the omission only when
`cargo test --workspace` failed to compile after ADR-0001's own declared four-file edit:
`crates/paladin-ports/src/output/citadel_port.rs` (4 sites — a compiled `#[cfg(test)]` module's
import and construction call, plus a `rust,no_run` doc example's import and construction call) and
`tests/integration/citadel_integration_test.rs` (3 sites — an import and two `BattalionState::new`
calls). Plan 02-03's SUMMARY carries the full, corrected six-file count. See also the doctest-false
correction recorded in this ledger's "Phase 2 amendments" section above (item 3), which explains why
`citadel_port.rs`'s doc example did not itself force this discovery.

### Epic 5 — Commander Strategy Router

4 open task items per `intel/task-completion-state.md`, all under
`.project/Milestone_1-MVP/Epic_5/tasks-commander-strategy-router.md`. Rows whose subject is
`BattalionConfig`, `BattalionResult` or the Formation minimum Paladin count link to the same three
ADRs Epic 4 uses above, rather than re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-battalion-config-v2 | superseded by shipped code | See [ADR-0001](../decisions/0001-battalion-config.md). Same row/reasoning as Epic 4's `REQ-battalion-config-v2` above; not re-decided per-epic. |
| REQ-battalion-result-v2 | superseded by shipped code | See [ADR-0002](../decisions/0002-battalion-result.md). The shipped `BattalionResult` (`battalion/mod.rs:549`) is a merged superset; this variant's `execution_time_ms` is displaced by `per_paladin_times`, and its `errors: Vec<PaladinError>` is displaced by `node_errors: Vec<NodeError>` (serialization reason recorded in the ADR). Not re-decided here. |
| REQ-formation-min-paladins-v2 | genuinely outstanding | See [ADR-0003](../decisions/0003-formation-min-paladins.md). The Commander-level half of this variant (construction validates only ≥1 Paladin, Auto routes a single Paladin to Formation) is real: `test_auto_selects_formation_for_single_paladin` (`crates/paladin-battalion/src/commander.rs:1912`) passes today. But the full behavioral claim this row makes — that a single-Paladin Battalion **executes** via Formation rather than failing — does not hold: `Formation::validate` (`crates/paladin-core/src/platform/container/battalion/formation.rs:109-111`) still rejects it, per ADR-0003's own contradiction record. No end-to-end test exists where a real single-Paladin Formation execution succeeds (the passing test only proves strategy *selection*, not successful execution). Forward owner: **GAP-07**, which lands ADR-0003's relaxed bound. |
| REQ-commander-construction | satisfied | `Commander` struct at `commander.rs:151`, `CommanderBuilder` at `:1272`; exercised by `test_commander_builder_success` (`commander.rs:1689`), `test_commander_builder_missing_paladins` (`:1728`) and `test_commander_builder_invalid_config` (`:1767`) |
| REQ-commander-strategy-types | satisfied | `BattalionStrategy` enum at `crates/paladin-core/src/platform/container/battalion/mod.rs:375`; exercised by `test_commander_all_strategies` (`commander.rs:1790`) |
| REQ-commander-auto-selection | satisfied | `analyze_and_select` exercised by 11 passing keyword-selection tests including `test_auto_selects_campaign_for_workflow_keywords`. This upgrades the 2026-01 "Partial → GAP-05 (one failing keyword test)" note (`REQUIREMENTS.md:2607`): the task list's own line 99 records `test_auto_selects_campaign_for_workflow_keywords` as "(FAILING - needs fix)", but `cargo test -p paladin-battalion --lib commander:: -- --test-threads=4` run on 2026-07-31 shows this test, and all 11 auto-selection tests, passing with 0 failures. **Amended 2026-08-01, plan 02-09, citing plan 02-01's `02-test-baseline.md` re-proof (GAP-05):** re-executed live, not cited from the January run — `cargo test -p paladin-battalion test_auto_selects` on commit `7e55655ccf58e7f010f5fbbf9c2504f30a0ea29c` shows 7 passed, 0 failed, 0 ignored, naming all 7 `test_auto_selects_*` tests including this one, replacing inference with a command that ran on this phase's own measured commit. See the nested item below. |
| | | **Nested outstanding item:** `- [ ] 3.11 Write unit test: test_auto_selects_campaign_for_workflow_keywords (FAILING - needs fix)` (`tasks-commander-strategy-router.md:99`) — **satisfied** (checkbox stale). Directly re-run 2026-07-31: `test commander::tests::test_auto_selects_campaign_for_workflow_keywords ... ok`. Whatever caused the January failure has since been fixed; no trace of the original bug remains in the tree. |
| REQ-commander-execute | satisfied | `Commander::execute` at `commander.rs:337`; exercised by `test_execute_resolves_auto_strategy` (`commander.rs:2063`), `test_execute_routes_to_campaign_service` (`:2006`) and `test_execute_routes_to_chain_service` (`:2035`) |
| REQ-commander-result-normalization | satisfied | `BattalionResult` metadata population exercised by `test_result_contains_telemetry_metadata` (`commander.rs:2155`). This upgrades the 2026-01 "Partial → GAP-04 (task 5.0 open)" note (`REQUIREMENTS.md:2609`) — see the nested items below, both of which re-verify as done. |
| | | **Nested outstanding item:** `- [ ] 5.0 Implement result normalization and telemetry metadata` (`tasks-commander-strategy-router.md:122`) — **satisfied** (parent checkbox stale). 13 of its 15 children are checked; the two unchecked children (5.10, 5.14, both below) are directly re-verified as implemented and tested, so nothing in this parent's scope is actually outstanding. |
| | | **Nested outstanding item:** `- [ ] 5.10 Implement metadata export to file if \`metadata_output_dir\` is configured (deferred - requires file I/O)` (`tasks-commander-strategy-router.md:132`) — **satisfied** (checkbox stale). `export_metadata` at `commander.rs:880` implements exactly this; exercised by `test_metadata_export_creates_file` (`commander.rs:2894`) and `test_metadata_export_correct_naming` (`:2932`), both passing. The "(deferred - requires file I/O)" annotation does not match the shipped tree. |
| | | **Nested outstanding item:** `- [ ] 5.14 Write unit test: test_metadata_export_to_file (deferred - requires file I/O setup)` (`tasks-commander-strategy-router.md:136`) — **satisfied** (checkbox stale). `test_metadata_export_json_structure` (`commander.rs:2980`) and `test_metadata_export_no_dir_configured` (`:3048`) are exactly this test, under different names than the task list anticipated; both pass. |
| REQ-commander-error-strategy | satisfied | Base capability proven by `test_error_handling_fail_fast` (`commander.rs:3072`), `test_error_handling_continue_on_error` (`:3108`) and `test_error_handling_retry_then_continue` (`:3141`), all passing. A residual caveat: 4 edge-case tests remain `#[ignore]`d with empty bodies — `test_fail_fast_stops_on_first_error` (`:2180`), `test_continue_on_error_collects_all_errors` (`:2188`), `test_retry_then_continue_retries_failed_paladins` (`:2196`), `test_partial_results_returned_with_errors` (`:2204`) — confirming the 2026-01 "4 remain in commander.rs" count (`REQUIREMENTS.md:2610`) still holds, re-verified 2026-07-31. Forward note: **QUAL-04**. **Amended 2026-08-02, plan 03-08, citing `tests/integration/commander_error_paths_test.rs` and plan `03-02-SUMMARY.md`:** the residual caveat above is resolved. The four edge-case tests were relocated with real bodies and no `#[ignore]` attribute to `tests/integration/commander_error_paths_test.rs` — `test_fail_fast_stops_on_first_error` (`tests/integration/commander_error_paths_test.rs:40`), `test_continue_on_error_collects_all_errors` (`:81`), `test_retry_then_continue_retries_failed_paladins` (`:144`), `test_partial_results_returned_with_errors` (`:204`) — exercised by `cargo test --offline --test lib -- commander_error_paths`, re-run 2026-08-02: `test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 692 filtered out`. QUAL-04 is closed by this relocation. |
| REQ-commander-config-passthrough | satisfied | Exercised by `test_config_passthrough_to_services` (`commander.rs:2212`) |
| REQ-commander-service-composition | satisfied | Exercised by `test_execute_routes_to_campaign_service` (`commander.rs:2006`) and `test_execute_routes_to_chain_service` (`:2035`) |
| REQ-commander-telemetry | satisfied | `export_metadata` at `commander.rs:880`; exercised by `test_metadata_export_creates_file` (`:2894`) and `test_metadata_export_json_structure` (`:2980`). This upgrades the 2026-01 "Partial → GAP-04" note (`REQUIREMENTS.md:2613`), whose own "Tree observation: export path exists at `crates/paladin-battalion/src/commander.rs:870`" is confirmed (the current line is `880`, a small drift consistent with intervening commits, not a contradiction). |
| REQ-commander-validation | satisfied | Exercised by `test_commander_builder_missing_paladins` (`commander.rs:1728`) and `test_commander_builder_invalid_config` (`:1767`) |

### Epic 6 — Provider Expansion

19 open task items per `intel/task-completion-state.md` — the single largest concentration in
Milestone 1, all under `.project/Milestone_1-MVP/Epic_6/tasks-provider-expansion.md` task 7.0 and
its 18 subtasks. The row whose subject is the DeepSeek temperature range links to
[ADR-0004](../decisions/0004-temperature-validation.md) rather than re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-llm-port-interface | satisfied | `ProviderCapabilities` struct at `crates/paladin-ports/src/output/llm_port.rs:754`; `get_capabilities()` trait method at `llm_port.rs:1264`; exercised by `test_get_capabilities` (`crates/paladin-llm/src/openai/adapter.rs:698`) and `test_deepseek_provider_capabilities` (`crates/paladin-llm/src/deepseek/adapter.rs:618`) |
| REQ-deepseek-adapter | satisfied | `DeepSeekAdapter`/`DeepSeekConfig` at `crates/paladin-llm/src/deepseek/adapter.rs:212,27`; exercised by `test_deepseek_adapter_creation` (`:607`) and `test_deepseek_provider_capabilities` (`:618`), both passing on a fresh `cargo test -p paladin-llm --lib --all-features` run (2026-07-31, 67 passed / 0 failed). The task list's own "15.02% coverage" figure (`REQUIREMENTS.md:2621`) is carried forward from the prior measurement, not re-run here — no `cargo-llvm-cov` binary is available in this sandbox (same blocker plan 01-04/RECON-07 recorded); a fresh figure is that plan's output, not this one's |
| REQ-anthropic-adapter | satisfied | `AnthropicAdapter`/`AnthropicConfig` at `crates/paladin-llm/src/anthropic/adapter.rs:119,29`; exercised by `test_anthropic_adapter_creation` and `test_anthropic_provider_capabilities` (same file), both passing in the same 2026-07-31 run. The "28.19% coverage" figure (`REQUIREMENTS.md:2622`) is likewise carried forward, not re-measured |
| REQ-provider-configuration | satisfied | `LlmProviderFactory::create()` at `crates/paladin-llm/src/provider_factory.rs:62`, matching provider names to `openai`/`deepseek`/`anthropic`; `LlmConfig::default_provider` at `crates/paladin-llm/src/config/llm.rs:27`; exercised by `test_factory_creation` and `test_unknown_provider_returns_error` (`provider_factory.rs:163,169`) |
| REQ-provider-backward-compat | satisfied | `test_llm_config_default` (`crates/paladin-llm/src/config/llm.rs:123`) asserts `default_provider == Some("openai")`, i.e. OpenAI remains the default when no provider is configured, matching the requirement |
| REQ-provider-error-mapping | satisfied | Each adapter maps HTTP status codes to `LlmError` variants directly at the call site — e.g. `crates/paladin-llm/src/deepseek/adapter.rs:343-350` maps 401→`AuthenticationError`, 429→`RateLimitExceeded`, 404→`ModelNotAvailable`, 400→`InvalidPrompt`, else→`ProcessingError`. **Finding**: a separate `LlmProviderError` type with its own `From<LlmProviderError> for LlmError` impl exists at `crates/paladin-llm/src/error.rs:16,54`, but `grep -rn "LlmProviderError" crates/paladin-llm/src/` outside that one file returns zero matches — no adapter actually constructs it. The mapping the requirement asks for is real, but via direct `LlmError` construction at each site, not through the named `LlmProviderError` conversion path the file's own doc comment describes |
| REQ-provider-testing | satisfied | **Amended 2026-08-01, plan 02-09, citing plan 02-06's SUMMARY.** `tests/unit/llm/{deepseek_adapter_test,anthropic_adapter_test,provider_factory_test}.rs` is no longer dead code: `tests/unit/mod.rs` was missing the single `pub mod llm;` line that connected these three files to the `unit` `[[test]]` target — plan 02-06 added it (D-10/D-11) and repaired the mechanical construction-API drift the files had accumulated since being written. Per-file counts (present / passing / removed): `deepseek_adapter_test.rs` 8/8/0, `anthropic_adapter_test.rs` 9/9/0, `provider_factory_test.rs` 8/8/0 — **25/25 passing, 0 removed** under the structural-breakage rule (every breakage found was mechanical, not structural; no test asserted behaviour the current adapters no longer have). Exercised by `cargo test --test unit -- llm` (41 passed: the 25 target functions plus 16 pre-existing name matches, 0 failed, 0 ignored). The 401/429/timeout/streaming/malformed-response HTTP-level failure paths these tests cover are ones the 67 live `paladin-llm` tests do not reach. The live-API integration suite (`tests/integration/llm_live_api_tests.rs`) still compiles and runs behind the `live-api-tests` feature flag; `require_api_key()`'s panic-on-missing-key semantics are unchanged by this phase and remain **VERIFY-06**'s (Phase 5) to resolve — not re-decided here. |
| REQ-provider-documentation | satisfied | `docs/src/appendix/provider-expansion.md` (521 lines) and `docs/src/contributing/contributing-providers.md` (458 lines) — the same mdbook-relocation pattern already recorded elsewhere in this ledger for other Milestone-1 docs deliverables (originally `docs/PROVIDER_EXPANSION.md` / `docs/CONTRIBUTING_PROVIDERS.md` per the task list, relocated not missing) |
| REQ-temperature-range-v1 / -v2 | superseded by shipped code | See [ADR-0004](../decisions/0004-temperature-validation.md), which records this as `must change`: `ProviderCapabilities` has no `temperature_range` field today, so neither the v1 global-clamp nor the v2 DeepSeek-specific position is currently reachable through the port boundary as the ADR's provider-aware design requires. The builder's shipped clamp is `[0.0, 1.0]` (`paladin_builder.rs:1112-1117`, matching v1 as today's fallback default); the DeepSeek `0.0-2.0` range v2 asks for remains unreachable until **GAP-07** lands the field. Not re-decided here |
| | | **Nested outstanding item:** `- [ ] 7.0 Write integration tests for live API validation (DEFERRED - unit tests with mocks provide sufficient coverage)` (`tasks-provider-expansion.md:225`) — **present, unproven** (parent). The task's own inline annotation calls this deferred, but per the second prohibition a deferral verdict needs a citation to an actual deferring document, not an inference from the checkbox text. `STATE.md` §Deferred Items records this row differently: "Un-deferred by run 2 — suite ships behind `live-api-tests`; only the skip-vs-fail semantics remain open (VERIFY-06)". The suite does ship (`tests/integration/llm_live_api_tests.rs` exists, gated by the real `live-api-tests` feature), so it is not `genuinely outstanding` either — it is real code with one unresolved semantic question, hence `present, unproven` rather than either extreme. Forward owner: **VERIFY-06** |
| | | **Nested outstanding item:** `- [ ] 7.1 Create \`tests/integration/llm/mod.rs\` for integration test organization` (`tasks-provider-expansion.md:226`) — **superseded by shipped code**. This exact reorganization never happened at this path, but the live-API tests it would have organized ship as a single flat file, `tests/integration/llm_live_api_tests.rs`, achieving the same functional purpose (a dedicated home for live-API tests) via a different structure |
| | | **Nested outstanding item:** `- [ ] 7.2 Create \`tests/integration/llm/deepseek_integration_test.rs\`` (`tasks-provider-expansion.md:227`) — **superseded by shipped code**. DeepSeek live-API coverage ships inside `llm_live_api_tests.rs` (`require_api_key("DEEPSEEK_API_KEY", ...)` at lines 282,313,363,390) rather than in a dedicated per-provider file |
| | | **Nested outstanding item:** `- [ ] 7.3 Write integration test: \`test_deepseek_live_completion()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:228`) — **present, unproven**. `llm_live_api_tests.rs` has DeepSeek completion tests gated by the `live-api-tests` feature rather than `#[ignore]`, functionally equivalent (neither runs by default), but `require_api_key`'s panic-on-missing-key means the test fails loudly rather than skipping if run without a key — the exact VERIFY-06 gap |
| | | **Nested outstanding item:** `- [ ] 7.4 Write integration test: \`test_deepseek_live_streaming()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:229`) — **present, unproven**, same reasoning as 7.3 |
| | | **Nested outstanding item:** `- [ ] 7.5 Write integration test: \`test_deepseek_model_validation()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:230`) — **present, unproven**, same reasoning as 7.3 |
| | | **Nested outstanding item:** `- [ ] 7.6 Create \`tests/integration/llm/anthropic_integration_test.rs\`` (`tasks-provider-expansion.md:231`) — **superseded by shipped code**, same reasoning as 7.2 — Anthropic live-API coverage ships inside the single flat file instead |
| | | **Nested outstanding item:** `- [ ] 7.7 Write integration test: \`test_anthropic_live_completion()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:232`) — **present, unproven**, same reasoning as 7.3 (Anthropic case, `llm_live_api_tests.rs:424,456,514,542`) |
| | | **Nested outstanding item:** `- [ ] 7.8 Write integration test: \`test_anthropic_live_streaming()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:233`) — **present, unproven**, same reasoning |
| | | **Nested outstanding item:** `- [ ] 7.9 Write integration test: \`test_anthropic_model_validation()\` marked with \`#[ignore]\`` (`tasks-provider-expansion.md:234`) — **present, unproven**, same reasoning |
| | | **Nested outstanding item:** `- [ ] 7.10 Create \`tests/integration/llm/provider_switching_test.rs\`` (`tasks-provider-expansion.md:235`) — **Amended 2026-08-01, plan 02-09, citing plan 02-06's SUMMARY: satisfied**. `tests/integration/provider_switching_test.rs` now exists (a flat file, not under an `llm/` subdirectory — the same functional-relocation pattern rows 7.1/7.6 above already record for this epic), exercised by `cargo test --test lib -- integration::provider_switching_test` (2 passed, 0 failed): `test_provider_switch_preserves_request_contract` proves two distinct `LlmPort` implementations (a mock and a real `DeepSeekAdapter` against a local mockito server) can be selected at runtime behind the same `Arc<dyn LlmPort>` with the request/response contract preserved and the providers' `temperature_range` capabilities genuinely differing; `test_provider_switch_rejects_unknown_provider` proves the typed-error path. Runs fully offline, no live provider keys. |
| | | **Nested outstanding item:** `- [ ] 7.11 Write integration test: \`test_switch_providers_via_config()\` with mocks` (`tasks-provider-expansion.md:236`) — **genuinely outstanding**, no matching test found anywhere in the tree. Not amended by this plan — 02-CONTEXT.md D-09 scoped only task 7.10 (the provider-switching test file itself) into Phase 2; this specific named test (config-driven switching, distinct from 7.10's runtime-switching proof) remains open |
| | | **Nested outstanding item:** `- [ ] 7.12 Write integration test: \`test_multiple_providers_simultaneously()\`` (`tasks-provider-expansion.md:237`) — **genuinely outstanding**, no matching test found. Not amended by this plan, same D-09 scoping reason as 7.11 |
| | | **Nested outstanding item:** `- [ ] 7.13 Write integration test: \`test_provider_capabilities_detection()\`` (`tasks-provider-expansion.md:238`) — **present, unproven** as a dedicated integration test by this name, though the underlying capability is unit-tested per-adapter (`test_get_capabilities`, `test_deepseek_provider_capabilities`, `test_anthropic_provider_capabilities` cited above). Not amended by this plan |
| | | **Nested outstanding item:** `- [ ] 7.14 Add CI configuration notes for optional live API tests (REQ-26)` (`tasks-provider-expansion.md:239`) — **Amended 2026-08-01, plan 02-09, per D-09: deferred with reason**. `grep -n "live-api-tests" .github/workflows/*.yml` still returns zero matches — this is a CI-workflow change, D-09's own text names it explicitly out of Phase 2's scope and Phase 15's PIPE territory. **Forward owner: Phase 15 (PIPE)**, blocked in substance on Phase 5's **VERIFY-06**, which has not yet decided whether a keyless CI run should fail loudly or skip cleanly — exactly what such a job would encode. |
| | | **Nested outstanding item:** `- [ ] 7.15 Run \`cargo test --test deepseek_integration_test\` (without --ignored) to verify non-live tests` (`tasks-provider-expansion.md:240`) — **superseded by shipped code**. No target named `deepseek_integration_test` exists (per 7.2/7.6 above); the equivalent command against the shipped structure is `cargo test --test llm_live_api_tests --features live-api-tests` |
| | | **Nested outstanding item:** `- [ ] 7.16 Optionally run \`cargo test --ignored\` with API keys set to test live APIs` (`tasks-provider-expansion.md:241`) — **present, unproven**. Running this command requires live provider API keys not available in this sandbox; not executed as part of this ledger entry |
| | | **Nested outstanding item:** `- [ ] 7.17 Fix any failing integration tests` (`tasks-provider-expansion.md:242`) — **present, unproven**, contingent on 7.16 actually being run |
| | | **Nested outstanding item:** `- [ ] 7.18 Document how to run integration tests in README or test files` (`tasks-provider-expansion.md:243`) — **satisfied**. `llm_live_api_tests.rs:10-27` carries a full "## Running Tests" doc comment with both `.env`-file and exported-variable invocation methods, provider-scoped run commands, and a cost warning — this documentation already exists, just inline in the test file rather than in the README |

**Planning-input correction (2026-08-01, plan 02-09):** Phase 2 CONTEXT.md's own D-15 enumerated
every non-adapter `ProviderCapabilities` construction site beyond the three shipped adapters
(`REQ-llm-port-interface` above), compiled by grep during Phase 2's discussion phase. That list
**omitted the OpenAI and Anthropic adapters' own `get_capabilities` literals** — both needed the
`temperature_range` field addition exactly like DeepSeek did. Plan 02-02's SUMMARY carries the full,
compiler-verified construction-site list (every exhaustive `ProviderCapabilities { … }` literal in
the workspace, cross-checked against `grep -rn "ProviderCapabilities\s*{"` with a clean
`cargo build --workspace` after every site was updated). See also this ledger's top-level "Phase 2
amendments" section, item 1, for the full corrected list.

### Epic 7 — Citadel State Persistence

No open task items (169/169 complete per `intel/task-completion-state.md` and confirmed by
`grep -cE '^\s*- \[ \]' tasks-citadel-state-persistence.md` returning 0) — every row below carries
no nested block.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-citadel-paladin-state-serialization | satisfied | `PaladinState` struct at `crates/paladin-core/src/platform/container/citadel.rs:128`; exercised by `test_paladin_state_serialization_roundtrip` (`citadel.rs:412`) |
| REQ-citadel-autosave | satisfied | `PaladinBuilder::enable_autosave()` at `src/application/services/paladin/paladin_builder.rs:1000`; `CitadelConfig::autosave_enabled` at `src/config/citadel.rs:14`; exercised by `test_builder_enable_autosave` (`paladin_builder.rs:1458`) and demonstrated end-to-end by `examples/citadel_autosave.rs` |
| REQ-citadel-paladin-restore | satisfied | `PaladinBuilder::restore_from()` at `paladin_builder.rs:1061`, returning `Result<Self, PaladinError>` (fallible, matching the PRD); exercised by `test_builder_restore_from_state_not_found` (`paladin_builder.rs:1685`) and demonstrated by `examples/citadel_restore.rs` |
| REQ-citadel-battalion-state-serialization | satisfied | `BattalionState` struct at `citadel.rs:227`; exercised by `test_battalion_state_serialization_roundtrip` (`citadel.rs:453`) |
| REQ-citadel-battalion-checkpoint-restore | satisfied | `CheckpointData` struct at `citadel.rs:304`, `mark_completed`/`mark_failed` at `:327,334`; exercised by `test_checkpoint_mark_completed`/`test_checkpoint_mark_failed` (`citadel.rs:539,553`) and demonstrated end-to-end by `examples/battalion_checkpoint_recovery.rs` |
| REQ-citadel-port | satisfied | `CitadelPort` trait at `crates/paladin-ports/src/output/citadel_port.rs:567`; exercised via a mock implementation by `test_mock_citadel_implements_trait` (`citadel_port.rs:624`) and object-safety-checked by `test_trait_is_object_safe` (`:658`) |
| REQ-citadel-errors | satisfied | `CitadelError` enum at `crates/paladin-core/src/platform/container/citadel_error.rs:25`; exercised by `test_state_not_found_error` (`:99`) and `test_incompatible_version_error` (`:116`) |
| REQ-citadel-builder-integration | satisfied | `PaladinBuilder::with_citadel()` at `paladin_builder.rs:977`; exercised by `test_builder_with_citadel` (`paladin_builder.rs:1448`) |
| REQ-citadel-state-directory | satisfied | `CitadelConfig::state_dir` at `src/config/citadel.rs:12`, default `"./paladin-states"` (`:25`); `FileCitadel` creates the directory on construction — exercised by `test_file_citadel_creates_directory` (`crates/paladin-memory/src/citadel/file_citadel.rs:379`) and `test_file_citadel_rejects_file_as_directory` (`:391`) |
| REQ-citadel-logging-docs | satisfied | `log::{info, warn}` calls in `file_citadel.rs:29` and call sites throughout; Citadel is documented across multiple mdbook pages rather than one dedicated file — `docs/src/architecture/domain-model.md`, `docs/src/getting-started/configuration.md`, `docs/src/architecture/overview.md`, `docs/src/api-reference/stable-api.md` all reference it, the same multi-page-relocation pattern already recorded elsewhere in this ledger for other Milestone-1 docs deliverables (no single `docs/CITADEL.md` exists, but the content is present, not missing) |

### Epic 8 — Herald Output Formatting

2 open task items per `intel/task-completion-state.md` (parent task 7.0 and its child 7.13), both
under `.project/Milestone_1-MVP/Epic_8/tasks-herald-output-formatting.md`. The row whose subject is
the `Herald` trait method set links to [ADR-0005](../decisions/0005-herald-trait.md) rather than
re-deciding.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-herald-trait-v1 / -v2 | superseded by shipped code | See [ADR-0005](../decisions/0005-herald-trait.md), which records this decision as `conforms`: the shipped trait at `crates/paladin-core/src/platform/container/herald.rs:49` ships the v2 fallible form (`Result<String, HeraldError>` throughout except the deliberately-infallible `format_error`), superseding v1's infallible-`String` form. Not re-decided here |
| REQ-herald-builtin-formatters | satisfied | `JsonHerald`/`MarkdownHerald`/`TableHerald` at `crates/paladin-herald/src/{json_herald.rs:73,markdown_herald.rs:105,table_herald.rs:63}`; exercised by `test_format_paladin_result_success` (`json_herald.rs:311`), `test_format_paladin_result_structure` (`markdown_herald.rs:451`) and `test_format_paladin_result` (`table_herald.rs:287`) |
| REQ-herald-streaming | satisfied | `format_stream_chunk`/`finalize_stream` implemented in all three formatters; exercised by `test_format_stream_chunk_ndjson` (`json_herald.rs:381`) and `test_finalize_stream` (`json_herald.rs:406`, `markdown_herald.rs:503`, `table_herald.rs:363`) |
| REQ-herald-configuration | satisfied | `HeraldConfig` struct at `src/config/herald.rs:62`, `default_formatter` field defaulting to `"json"` (`:64,76`); exercised by `test_herald_config_default` (`:159`) and `test_herald_config_validate_invalid_formatter` (`:191`) |
| REQ-herald-default-and-override | satisfied | `HeraldRegistry::default()` auto-registers all three built-in formatters (`src/application/services/herald/herald_registry.rs:216`); runtime override via `PaladinBuilder::with_herald()` (`src/application/services/paladin/paladin_builder.rs:713`); exercised by `test_default_registry` (`herald_registry.rs:425`) and `test_default_registry_can_override_builtin_formatters` (`:469`) |
| REQ-herald-paladin-result-fields | satisfied | `PaladinExecutionService::format_result()` at `src/application/services/paladin/paladin_execution_service.rs:423` calls `herald.format_paladin_result()` when a Herald is configured; exercised by `test_format_paladin_result_includes_metadata` (`json_herald.rs:327`), which asserts the formatted output's metadata fields against a real `PaladinResult`. This upgrades the 2026-01 "Verify" note (`REQUIREMENTS.md:2654`) — the wiring the note flagged as unconfirmed is confirmed and tested, re-verified 2026-07-31 |
| REQ-herald-battalion-result-fields | satisfied | **Amended 2026-08-01, plan 02-09, citing plans 02-04 and 02-05's SUMMARYs.** `format_battalion_result` trait method at `crates/paladin-core/src/platform/container/herald.rs:85`. **Contradiction found and closed:** plan 02-04's direct code reading contradicted this row's prior citation — `test_format_battalion_result_success`/`_includes_metadata` (`json_herald.rs:354,369`) never exercised the Table Herald at all, and the Table Herald's own pre-existing test asserted only its own two hardcoded placeholder rows ("paladin_1"/"paladin_2") against an empty `paladin_results` input, a test that would pass against a formatter that ignored its argument entirely. Plan 02-04 closed the gap for real: Formation now populates `per_paladin_times`/`per_paladin_tokens`/`total_tokens`/`node_errors` (ported from Phalanx's pattern), and all three Heralds (JSON, Markdown, Table) render the Battalion's name/id/strategy, per-Paladin results in execution order, aggregate tokens and failure detail — the Table Herald's stub replaced entirely, its self-confirming test replaced by four content-asserting tests including a differing-output litmus test. Plan 02-05 then closed the producer-to-renderer proof end-to-end: `tests/integration/battalion_herald_end_to_end_test.rs`'s `test_formation_result_through_json_markdown_table_heralds` and `test_formation_partial_results_through_all_three_heralds` drive a **real** `FormationExecutionService::execute` (no hand-built `BattalionResult` anywhere in the file) through all three Heralds, exercised by `cargo test --test lib -- integration::battalion_herald_end_to_end_test` (2 passed, 0 failed). |
| REQ-herald-registry | satisfied | `HeraldRegistry` struct at `src/application/services/herald/herald_registry.rs:85`, `register`/`get` at `:120,144`; exercised by `test_register_and_get_formatter` (`:316`) and thread-safety-checked by `test_registry_thread_safety` (`:401`) |
| REQ-herald-builder-integration | satisfied | `PaladinBuilder::with_herald()` at `paladin_builder.rs:713`; exercised by the same builder test suite pattern as `REQ-citadel-builder-integration` above (builder methods for optional adapters are tested identically across Garrison/Citadel/Herald) |
| REQ-herald-error-handling | satisfied | `format_error(&self, error: &PaladinError) -> String` — the deliberately infallible method per ADR-0005 — at `json_herald.rs:215`; exercised by `test_format_error` (`json_herald.rs:433`, and the equivalent in `markdown_herald.rs:529`, `table_herald.rs:389`) |
| | | **Nested outstanding item:** `- [ ] 7.0 Integrate Herald with Paladin/Battalion execution` (`tasks-herald-output-formatting.md:167`) — **satisfied** (parent checkbox stale). 11 of its 12 subtasks (7.1-7.11) are checked; the sole open child is 7.13 below, and the parent's own scope (Paladin and Battalion integration) is otherwise complete — `with_herald`, `format_result`, and Formation/Phalanx Herald support all cited above |
| | | **Nested outstanding item:** `- [ ] 7.13 Write integration tests for Battalion with Herald (deferred needs Battalion execution setup)` (`tasks-herald-output-formatting.md:180`) — **Amended 2026-08-01, plan 02-09, citing plan 02-05's SUMMARY: satisfied**. `tests/integration/battalion_herald_end_to_end_test.rs`'s `test_formation_result_through_json_markdown_table_heralds` and `test_formation_partial_results_through_all_three_heralds` are exactly this integration test — a real `FormationExecutionService` run over three named mock Paladins, piped through JSON, Markdown and Table Heralds, including one deliberately-failed Paladin for the partial-results case. Command: `cargo test --test lib -- integration::battalion_herald_end_to_end_test` (2 passed, 0 failed). The task's own note said the gap was "needs Battalion execution setup" — the closure drove a real execution service (`FormationExecutionService`), not a hand-built `BattalionResult`. |

### Epic 9 — Armory CLI Tools

3 open task items per `intel/task-completion-state.md` (tasks 13.4-13.6), all under
`.project/Milestone_1-MVP/Epic_9/tasks-armory-cli-tools.md`. `STATE.md` §Deferred Items records
these as un-deferred by ingest run 2 — the blocking mock provider shipped
(`REQ-mock-llm-adapter`) — verified against the tree below rather than carried from the January note.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-cli-structure | satisfied | `#[derive(Parser)] struct Cli` and `enum Commands` at `src/bin/paladin-cli.rs:18,32` (relocated to `src/application/cli/` for the command handlers themselves, matching `REQUIREMENTS.md:2664`'s "relocated" note); exercised end-to-end by `test_cli_help_command` (`tests/integration/cli_integration_test.rs:59`), which builds and runs the actual compiled binary with `--help` |
| REQ-cli-agent-run | satisfied | `handle_agent_run()` at `src/application/cli/commands/agent.rs:150`; exercised by `test_agent_run_args_creation` (`agent.rs:523`) and end-to-end by `test_missing_api_key_error` (`cli_integration_test.rs:637`), which runs the real binary |
| REQ-cli-agent-new | satisfied | `handle_agent_new()` at `agent.rs:98`; exercised by `test_handle_agent_new_success` (`agent.rs:589`) and end-to-end by `test_agent_new_generates_valid_template_default_provider` (`cli_integration_test.rs:101`) |
| REQ-cli-battalion-run | satisfied | `handle_battalion_run()` at `src/application/cli/commands/battalion.rs:134`; exercised by `test_battalion_run_args_creation` (`battalion.rs:1050`) |
| REQ-cli-battalion-new | satisfied | `handle_battalion_new()` at `battalion.rs:77`; exercised by `test_handle_battalion_new_formation`/`_phalanx`/`_campaign`/`_chain_of_command` (`battalion.rs:1065,1085,1104,1123`) and end-to-end by `test_battalion_new_generates_formation_template` (`cli_integration_test.rs:226`) |
| REQ-cli-arsenal-list | satisfied | `handle_arsenal_list()` at `src/application/cli/commands/arsenal.rs:76`; exercised end-to-end by `test_arsenal_list_command_exists` (`cli_integration_test.rs:882`) |
| REQ-cli-arsenal-test | satisfied | `handle_arsenal_test()` at `arsenal.rs:314`; exercised by `test_arsenal_test_args_mutual_exclusivity_at_runtime` (`arsenal.rs:633`) and end-to-end by `test_arsenal_test_command_exists`/`test_arsenal_test_requires_mcp_option` (`cli_integration_test.rs:901,919`) |
| REQ-cli-config-format | satisfied | `PaladinYamlConfig` struct at `src/application/cli/config/paladin_config.rs:40`, YAML-only per the requirement; exercised by `test_valid_config` (`:356`) and `test_invalid_provider` (`:431`) |
| REQ-cli-env-vars | satisfied | Provider API-key lookup via `std::env::var(env_var_name)` at `agent.rs:267`; exercised end-to-end by `test_missing_api_key_error` (`cli_integration_test.rs:637`), which asserts the actionable error message when the environment variable is absent |
| REQ-cli-validation-errors | satisfied | `CliError` enum at `src/application/cli/error.rs:21`; exercised by `test_validation_error_formatting` (`src/application/cli/error_impl.rs:415`) and end-to-end by `test_invalid_config_file_error`/`test_config_file_not_found_error` (`cli_integration_test.rs:676,714`) |
| REQ-cli-output-formatting | satisfied | `OutputFormatter::format_paladin_result()` at `src/application/cli/formatters/output.rs:234`; exercised end-to-end by `test_output_to_file_flag`/`test_verbose_mode_flag` (`cli_integration_test.rs:748,779`) |
| REQ-cli-interactive-mode | superseded by shipped code | See the Divergences table above (`REQ-cli-interactive-mode` row) — an interactive REPL ships in the Armory CLI, exceeding Epic 9's own non-goal NG-7 ("No REPL or interactive shell"). Not re-decided here; the divergence is stated once |
| | | **Nested outstanding item:** `- [ ] 13.4 Write test: run Paladin from config with mock LLM adapter (deferred - requires CLI mock provider support)` (`tasks-armory-cli-tools.md:281`) — **Amended 2026-08-01, plan 02-09, citing plan 02-07's SUMMARY: satisfied**. **Correction to the prior "never written" finding**: `tests/cli/paladin_execution_test.rs`'s `test_paladin_basic_execution` is exactly this test — it existed, complete, using current construction APIs, and needed only the missing `tests/cli/helpers.rs` shim to compile; it was never rewritten, only reconnected. Command: `cargo test --features cli --test cli -- paladin_execution_test::test_paladin_basic_execution`. |
| | | **Nested outstanding item:** `- [ ] 13.5 Write test: run Formation with multiple mock Paladins (deferred - requires CLI mock provider support)` (`tasks-armory-cli-tools.md:282`) — **Amended 2026-08-01, plan 02-09, citing plan 02-07's SUMMARY: satisfied**, same correction as 13.4 — `tests/cli/formation_execution_test.rs`'s `test_formation_basic_sequential_execution` is the named exerciser. Command: `cargo test --features cli --test cli -- formation_execution_test::test_formation_basic_sequential_execution`. |
| | | **Nested outstanding item:** `- [ ] 13.6 Write test: run Phalanx with parallel execution (deferred - requires CLI mock provider support)` (`tasks-armory-cli-tools.md:283`) — **Amended 2026-08-01, plan 02-09, citing plan 02-07's SUMMARY: satisfied**, same correction as 13.4 — `tests/cli/phalanx_execution_test.rs`'s `test_phalanx_basic_parallel_execution` is the named exerciser. Command: `cargo test --features cli --test cli -- phalanx_execution_test::test_phalanx_basic_parallel_execution`. |

### Epic 10 — Validation & Documentation

No open task items (103/103 complete per `intel/task-completion-state.md` and confirmed by
`grep -cE '^\s*- \[ \]' tasks-epic10-validation-documentation.md` returning 0) — every row below
carries no nested block. Task 7.0's status is not re-decided here; see the
`## Epic 10 Task 7.0 — dispute resolution (RECON-08)` section above, which this table's own
implicit "no Task 7.0" agrees with (the task list contains no Task 7.0 heading at all).

| ID | Verdict | Evidence |
|---|---|---|
| REQ-integration-testing | present, unproven | `tests/integration/` holds 20+ files exercising cross-component paths (cited throughout Epics 1-9 above); the specific "67.79% vs 70% gate" figure (`REQUIREMENTS.md:2682`) is carried forward from the January measurement, not re-run here — `cargo-llvm-cov` is unavailable in this sandbox (same blocker recorded against plan 01-04/RECON-07). Forward owner: **GAP-02, QUAL-03**, and the single global figure once produced is **RECON-07**'s output |
| REQ-performance-benchmarking | present, unproven | Per-crate `benches/` directories confirmed at `crates/paladin-memory/benches`, `crates/paladin-battalion/benches`, `crates/paladin-llm/benches` (relocated from a single top-level `benches/`, matching `REQUIREMENTS.md:2683`'s "relocated" note); not re-run as part of this ledger entry (`cargo bench` was not executed). Forward owner: **QUAL-05**, position **REQ-battalion-benchmark-repair** |
| REQ-api-documentation | satisfied | `cargo doc --workspace` target compiles cleanly per the workspace's own CI (`ci.yml` doc job); 80 mdbook pages under `docs/src/` cross-reference the generated rustdoc. This upgrades the 2026-01 "Verify → REL-04" note (`REQUIREMENTS.md:2684`) to the extent that the documentation surface itself is confirmed to exist and build; REL-04 remains the owner of any remaining per-criterion completeness review. **Amended 2026-08-03, plan 04-07, citing 04-06-SUMMARY.md and `.planning/ledgers/milestone-01.md` §"Epic 10 Task 7.0 — dispute resolution (RECON-08)"**: REL-04 is now discharged — the documentation-review clause by citing RECON-08's recorded `satisfied` verdict (no artifact anywhere in the corpus supplies a "Final Documentation Review"), and the QUICKSTART measurement clause by plan 04-06's first timing record. See the REL-04 row below. |
| REQ-user-documentation | satisfied | `docs/src/getting-started/` exists with a quickstart page. **Amended 2026-08-03, plan 04-07, citing 04-06-SUMMARY.md and `04-release-measurement.md` §"Entry measurement — QUICKSTART elapsed time (first measurement)"**: the "< 15 min quickstart target" open measurement this row previously carried forward is now closed — the reachable in-workspace prefix measured 4 minutes 22 seconds under stated conditions (warm registry, no crates.io network, no Docker, this machine's CPU/kernel), the target is settled at 15 minutes (reconciling `quickstart.md:3` with `introduction.md:9`), and the sample was repaired and proven to compile offline against the shipped tree. The true clean-machine, cold-registry timing remains unmeasured in this environment — **Owner: Phase 15 / PIPE** (see the deferred row below), not a reason to withhold `satisfied` on the measurement this phase could actually take. |
| REQ-architecture-documentation | satisfied | `docs/src/` now holds **80** markdown files (re-counted 2026-07-31 via `find docs/src -iname '*.md' | wc -l`), exceeding the "24 docs, ~5,000 lines" figure `REQUIREMENTS.md:2686` recorded — consistent with `STATE.md`'s note that Milestone 11 substantially grew this surface after the original Milestone-1 count was taken |
| REQ-deployment-artifacts | satisfied | `k8s/{deployment,service,configmap,namespace,redis,minio}.yaml` and `k8s/secret.yaml.example` confirmed present; `.github/workflows/{release,integration-tests}.yml` confirmed present. This matches `REQUIREMENTS.md:2687`'s citation; **REL-05** re-verifies per-criterion completeness, not re-decided here. **Amended 2026-08-03, plan 04-07, citing `04-ci-gate-deferrals.md` and `04-release-measurement.md`**: REL-05's per-criterion completeness review is now recorded below, split into facets — `cargo fmt`/`clippy`/`cargo test --workspace`/doc tests/every example target are `satisfied`; the multi-arch Docker build and the kind-based Kubernetes smoke test are `deferred with reason`, authored and statically validated but never executed in this environment (`docker`/`kind`/`kubectl` absent). **Owner: Phase 15 / PIPE** for both. |
| REQ-operations-documentation | satisfied | `docs/src/deployment/{production,docker}.md` confirmed present |
| REQ-contribution-documentation | satisfied | `docs/src/contributing/{contributing-providers,architecture-decisions}.md` and `docs/src/appendix/contributing-legacy.md` confirmed present |
| REQ-epic10-quality-gates | satisfied | The gates this row bundles (coverage, benchmarks, release readiness) are each individually `present, unproven` or forward-owned above (`REQ-integration-testing`, `REQ-performance-benchmarking`) rather than resolved as a single bundle here. Forward owners unchanged from `REQUIREMENTS.md:2690`: **QUAL-01, QUAL-03, REL-03, REL-05**. **Amended 2026-08-03, plan 04-07, citing `01-coverage-measurement.md` (QUAL-01), `03-coverage-measurement.md` (QUAL-03), `04-advisory-findings.md` (REL-03) and `04-release-measurement.md` (REL-05)**: all four forward owners are now discharged — QUAL-01 and QUAL-03 `Complete` since Phase 3; REL-03 `satisfied` (0 vulnerabilities, clean `cargo deny check`); REL-05 `satisfied` for its five locally-runnable gates with the Docker/Kubernetes clauses `deferred with reason`, **Owner: Phase 15 / PIPE**. See the REL-03 and REL-05 rows below. |

### unit-test-improvements workstream

2 open task items per `intel/task-completion-state.md` (parent tasks 2.0 and 6.0), both under
`.project/Milestone_1-MVP/unit-test-improvements/tasks-improve-unit-test-coverage.md`. **Note on
scope**: [ADR-0006](../decisions/0006-coverage-gate.md) records the single workspace-wide coverage
number this workstream should measure against — **84.79%** workspace line coverage, measured by
plan **01-09**'s offline `rustc`/`llvm-profdata`/`llvm-cov` instrumentation path and recorded as an
84% hard-fail floor by plan **01-10**. This workstream's own self-reported figures below remain
workstream-local numbers produced by its own task file, not the RECON-07 figure the ADR now
records.

| ID | Verdict | Evidence |
|---|---|---|
| REQ-test-coverage-target-v1 / -v2 | genuinely outstanding | Variant (group 1) — the coverage-target dispute (80% / 85% / 75%-layered / 78%-hard / 70→74→78 ramp, per `STATE.md`'s "sixth position on the coverage gate" note) is exactly what **RECON-07** and [ADR-0006](../decisions/0006-coverage-gate.md) are scoped to resolve with one number. RECON-07 records the measured figure (84.79%, floored to an 84% gate); the ADR does not pick a winner among these competing *targets* — that choice is forward-owned by **QUAL-01** (Phase 3) and **VERIFY-05** (Phase 5), not resolved here |
| REQ-unit-test-gap-closure | present, unproven | The task file's own "Current Progress" section (line 48) self-reports "70.56% regions / 68.29% lines" as of its own last update — a workstream-local figure, not independently re-measured in this session, and not the same thing as the single workspace-wide RECON-07 number. **Finding**: task 6.3's own claim, at line 131 ("Improved \[Provider Factory\] from 49.73% → 86.71%... Added 16 comprehensive unit tests covering `create_with_config()`"), does not match the current tree — `crates/paladin-llm/src/provider_factory.rs` has exactly 3 test functions today (`test_factory_creation`, `test_unknown_provider_returns_error`, `test_list_available_providers_returns_vec`, all at `provider_factory.rs:163,169,181`) and no `create_with_config()` method exists anywhere in that file. Either the workspace decomposition (Milestone 5) rewrote this file after the claim was recorded, or the claim was inflated at the time — this ledger does not speculate which, it records the mismatch. Forward owner: **QUAL-01, QUAL-02** |
| | | **Nested outstanding item:** `- [ ] 2.0 Add unit tests for files with 0% coverage` (`tasks-improve-unit-test-coverage.md:91`) — **satisfied** (parent checkbox stale). All 7 of its subtasks (2.1-2.7) are individually checked with per-file coverage deltas recorded inline (e.g. "2.3 ... improved from 0% to ~89.77%") |
| | | **Nested outstanding item:** `- [ ] 6.0 Improve Unit Test Coverage in gaps and verify coverage improvements` (`tasks-improve-unit-test-coverage.md:128`) — **present, unproven** (parent). Two of its six children (6.1, 6.2) are explicitly `[DEFERRED]` inline (User Service, Listener Service — matching `STATE.md`'s DEFER-01/DEFER-03 forward items), two more (6.4, 6.5) are also `[DEFERRED]` (MySQL/SQLite content repositories), and the deterministic checkbox counter does not count `[DEFERRED]` as open (it counts only literal `- [ ]`) — so this parent's open status comes entirely from its own un-ticked top-level box, not from any of its children. 6.3's own claim is contradicted by the tree (see `REQ-unit-test-gap-closure` finding above), and 6.6's "final coverage achieved 71.91%" is, again, a workstream-local self-report, not the RECON-07 figure |

## Outstanding item reconciliation (RECON-01)

**Total outstanding Milestone-1 task items nested in this ledger, sourced from a `.project/`
task-list checkbox: still 39.** This matches `intel/task-completion-state.md`'s deterministic
Milestone_1-MVP total exactly (39 open, extracted 2026-07-30 by counting literal `- [ ]` GFM
checkboxes across the milestone's 11 task lists — not an LLM classification of what "open" means).
Amending a nested item's *verdict* does not remove its row, so this figure is unchanged by Phase 2 —
none of the amendments below deleted a task-list-sourced nested item, they only updated its verdict
in place.

**Per-file breakdown, across all 8 task files that carry at least one open item** (source:
`intel/task-completion-state.md`'s "Open items by list" breakdown for `Milestone_1-MVP`):

| Task file | Open items (source) | Open items nested in this ledger | Agree? |
|---|---|---|---|
| `Epic_6/tasks-provider-expansion.md` | 19 | 19 | yes |
| `Epic_2/tasks-garrison-memory-system.md` | 4 | 4 (plan 01-06) | yes |
| `Epic_5/tasks-commander-strategy-router.md` | 4 | 4 (plan 01-06) | yes |
| `Epic_3/tasks-arsenal-tool-system.md` | 3 | 3 (plan 01-06) | yes |
| `Epic_9/tasks-armory-cli-tools.md` | 3 | 3 | yes |
| `Epic_4/tasks-battalion-orchestration.md` | 2 | 2 (plan 01-06) | yes |
| `Epic_8/tasks-herald-output-formatting.md` | 2 | 2 | yes |
| `unit-test-improvements/tasks-improve-unit-test-coverage.md` | 2 | 2 | yes |
| **Total** | **39** | **39** | **yes** |

The two figures still agree exactly: 39 nested outstanding-item bullets sourced from a task-list
checkbox exist in this ledger (13 authored by plan 01-06 across Epics 2-5, 26 authored by plan 01-07
across Epic 6, Epic 8, Epic 9 and unit-test-improvements), matching the deterministic source with no
adjustment needed in either direction.

**Phase 2 added 3 further nested rows, none sourced from a `.project/` task-list checkbox** — they
do not count against the 39/39 reconciliation above, and are named here so their absence from that
count is a recorded fact, not an inferred silence: the ADR-0007 cancellation-deferral split under
`REQ-battalion-cancellation` (Epic 4), and the two gaps plan 02-08's Garrison PRD-acceptance review
surfaced (`PaladinError::GarrisonRequired`, `GarrisonSettings::validate()`, both Epic 2). Physical
nested-item rows in the ledger: 39 + 3 = **42**.

**Verdict-class distribution across the whole ledger, re-counted 2026-08-01 after Phase 2's
amendments** (every `REQ-*` table row, every nested outstanding item — the original 39 plus the 3
Phase 2 added — and the 3 Divergences-table rows, counted by extracting each row's own Verdict
token: `grep -oE "^\| REQ-[^|]+\| [^|]+ \|"` for the 113 `REQ-*` rows' Verdict column, and the final
bolded verdict word in each `| | |`-prefixed nested-item line for the 42 nested rows, rather than a
loose full-file text grep, which would double-count verdict words that appear inside evidence
prose):

| Verdict | Count (2026-07-31, Phase 1) | Count (2026-08-01, Phase 2) | Net change |
|---|---|---|---|
| `satisfied` | 100 | 110 | +10 |
| `present, unproven` | 23 | 19 | -4 |
| `genuinely outstanding` | 11 | 5 | -6 |
| `superseded by shipped code` | 20 | 21 | +1 |
| `deferred with reason` | 1 | 3 | +2 |
| **Total (113 `REQ-*` rows + 42 nested items + 3 Divergences rows)** | **155** | **158** | **+3** |

**The arithmetic, shown:** row-count growth is exactly +3 (155 → 158), matching the 3 new nested
rows named above (no `REQ-*` row was added or removed — only existing verdicts changed, plus three
brand-new nested rows). Net verdict-class changes sum to zero (+10 -4 -6 +1 +2 = +3, matching the
row-count growth exactly): eleven existing rows changed class this phase (Epic 8 task 7.13, Epic 9
tasks 13.4/13.5/13.6, `REQ-provider-testing`, Epic 6 task 7.10, Epic 6 task 7.14, `REQ-battalion-cancellation`,
`REQ-garrison-testing`, Epic 2 task 11.5, Epic 2 task 11.6), and three brand-new rows were added
(the ADR-0007 cancellation split → `deferred with reason`; `PaladinError::GarrisonRequired` →
`genuinely outstanding`; `GarrisonSettings::validate()` → `present, unproven`). `REQ-herald-battalion-result-fields`
and `REQ-commander-auto-selection` were also amended this phase but stayed in the `satisfied` class
(their evidence was corrected/strengthened, not their verdict), so neither contributes to the net
change above.

`present, unproven` (19) remains the largest non-`satisfied` class and is exactly the size D-19's
evidence bar was designed to surface: each of those 19 has a real `file:line` citation but no named
exerciser proves it, a different and more precise finding than either "done" or "not done".
`genuinely outstanding` (5) shrank the most this phase (-6) — most of what Phase 2 set out to close
turned out to have a citable exerciser once someone wired the barrel declaration or wrote the
missing test, not nothing at all.
