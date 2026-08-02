---
phase: 3
slug: verification-depth
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-02
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded from `03-RESEARCH.md` §Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (built-in) for unit/integration; `criterion 0.5.1` for benchmarks |
| **Config file** | none dedicated — driven by `[[test]]`/`[[bench]]` entries in `Cargo.toml` and the `tests/integration/mod.rs` / `tests/unit/mod.rs` barrels |
| **Quick run command** | `cargo test -p <crate> --offline` scoped to the crate just touched |
| **Full suite command** | `RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="$PWD/target/coverage/paladin-%p-%m.profraw" cargo test --workspace --offline` (the ADR-0006 command — doubles as the coverage-measurement run) |
| **Estimated runtime** | ~60–90s per-crate quick run; full instrumented workspace run is measured, not estimated (it *is* QUAL-01's deliverable) |

**Every command carries `--offline`** — crates.io returns HTTP 403 in this environment (D-16).

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p <crate> --offline` for the crate just touched
- **After every plan wave:** Run the full instrumented `cargo test --workspace --offline` — this doubles as both "does everything still pass" and "what does coverage measure now", since measurement is this phase's whole point
- **Before `/gsd-verify-work`:** Full coverage pipeline + `cargo bench` across all 5 targets must be green
- **Max feedback latency:** ~90 seconds for the per-crate tier

**Exception:** if the MCP handshake-timeout test lands as a real 30-second test (see Manual-Only Verifications), exclude it from the quick tier and run it only at wave/phase gates.

---

## Per-Task Verification Map

Task IDs are assigned when `gsd-planner` writes the PLAN.md files; this draft is seeded at
requirement granularity per the research's Phase Requirements → Test Map.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | 1 | QUAL-01 | — | N/A | measurement | full ADR-0006 pipeline (`RUSTFLAGS=-C instrument-coverage` → `llvm-profdata merge` → `llvm-cov report`) | ✅ pipeline proven by plan 01-09; re-run needed | ⬜ pending |
| TBD | TBD | 2 | QUAL-02 | — | N/A | unit | `cargo test -p paladin-storage redis::tests --offline` (after the Pitfall-1 refactor) + per-file equivalents | ❌ W0 — no test modules exist for the re-derived 0% set | ⬜ pending |
| TBD | TBD | 2 | QUAL-03 | — | N/A | integration | `cargo test --offline --test lib` (bundles `tests/integration/*`) | ✅ largely exists — `commander_integration_tests.rs`, `battalion/load_test.rs`, `arsenal_execution_integration_test.rs` already pass; mostly a citation/amendment task per D-08 | ⬜ pending |
| TBD | TBD | 2 | QUAL-04 (Commander) | — | N/A | integration | `cargo test --offline --test lib commander_error_paths` | ❌ W0 — `commander_error_paths_test.rs` does not exist | ⬜ pending |
| TBD | TBD | 2 | QUAL-04 (MCP) | T-03-01 / T-03-02 | Bearer token never appears in `Debug`/log output; malformed response returns `Err`, never panics or hangs | integration | `cargo test --offline --test lib mcp_streamable_http` | ❌ W0 — 3 of 5 modes need `FixtureServer` extensions, 2 need a new non-compliant fixture | ⬜ pending |
| TBD | TBD | 3 | QUAL-05 | — | N/A | bench + manual harness | 5 per-target `cargo bench --offline` commands + percentile derivation from `target/criterion/*/new/sample.json` | ✅ all 5 bench targets exist and compile (verified via `cargo check --bench`); baseline doc needs a new dated section | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/paladin-storage/src/redis.rs` — the `&self` → free-function refactor on the private key/serialize helpers **must land before any test module can be added** (Research Pitfall 1: `RedisQueueAdapter::new()` requires a live connection, so `&self`-taking helpers are unreachable without Docker)
- [ ] `tests/helpers/mock_paladin_port.rs` — add `FaultyPaladinPort` (fail-always, fail-Nth, fail-N-then-succeed with invocation counter, controllable delay); today the file holds only the always-succeeding `MockPaladinPort`
- [ ] `tests/integration/commander_error_paths_test.rs` — does not exist
- [ ] MCP failure-mode test bodies in/near `tests/integration/mcp_streamable_http_test.rs` — 3 of 5 modes extend the existing hermetic rmcp+axum `FixtureServer`; 2 of 5 (malformed response, handshake timeout) need a new non-spec-compliant fixture
- [ ] `docs/src/appendix/performance-baseline.md` — needs a new dated section (D-15), **not** a new file

*No framework install needed — `cargo test` / `cargo bench` are already fully wired.*

**Research correction to CONTEXT.md:** new files under `tests/integration/` do **NOT** need a new
`[[test]]` entry in `Cargo.toml` — `tests/lib.rs` already pulls the whole tree in via
`pub mod integration;`. Adding a redundant `[[test]]` entry risks the `clippy::duplicate_mod`
failure that a comment in `tests/lib.rs` documents as having happened once already with `tests/cli/`.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| MCP handshake timeout | QUAL-04 | `STREAMABLE_HTTP_HANDSHAKE_TIMEOUT` (`mcp_protocol.rs:50`) is a hardcoded private 30s constant with no test seam. Tokio's `start_paused` does not help — the timeout wraps real socket I/O. A literal test costs 30+ real seconds. | Resolved by adding an additive `connect_streamable_http_with_timeout` variant defaulting to the existing constant, so the test passes `Duration::from_millis(200)`. If that route is rejected at execution time, the fallback is a real 30s test excluded from the quick tier — recorded, never silently dropped. |
| Memory-per-Paladin and startup time | QUAL-05 | criterion produces neither metric; these come from a purpose-built harness (process RSS delta across N constructed Paladins; wall-clock to first-Paladin-constructed / server ready) | Record with the D-16 provenance block (`rustc -vV`, `cargo --version`, `git rev-parse HEAD`, `date -u`, CPU/cores/kernel) and state which source produced each metric family so no reader mistakes a derived figure for a criterion result. |
| Live-Redis and MinIO code paths | QUAL-02 | Docker is absent from this environment | Out of scope by D-05/D-07 — `deferred with reason` rows naming Phase 15 (PIPE) as owner. Not a manual step for this phase; recorded here so the gap is visible. |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 90s for the per-crate tier
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
