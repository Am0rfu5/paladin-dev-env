---
phase: 14
slug: api-contract-truthfulness
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-08-12
---

# Phase 14 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded by `/gsd-plan-phase 14` from `14-RESEARCH.md` § Validation Architecture.
> The Per-Task Verification Map is filled in by the planner/executor once task IDs exist.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` / `cargo test`, workspace-wide |
| **Config file** | none — standard `Cargo.toml` per crate; no `nextest.toml` or custom harness |
| **Quick run command** | `cargo test -p <crate>` (crate-scoped: `paladin-web`, `paladin-llm`, `paladin`, `doc-examples`) |
| **Full suite command** | `cargo test` → `cargo fmt --check` → `cargo clippy -- -D warnings` (CLAUDE.md pre-commit sequence) |
| **Estimated runtime** | ~30–60s crate-scoped; workspace suite plus a cold clippy build is materially longer |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p <crate>` for the crate(s) touched, plus `cargo fmt --check` and `cargo clippy -- -D warnings`
- **After every plan wave:** Run `cargo test` (workspace), `./scripts/check-api-surface.sh`, and `cargo test -p paladin-web openapi_matches_committed_baseline`
- **Before `/gsd-verify-work`:** Full suite green AND both machine baselines (`crates/paladin-web/openapi.json`, `.project/current-exports.txt`) regenerated and matching
- **Max feedback latency:** 60 seconds for the crate-scoped quick run

---

## Per-Task Verification Map

Seeded from the research requirement→test map. Task IDs are assigned by the planner; the `Task ID` and `Plan` columns are filled in during planning/execution.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 14-01 T2 | 14-01 | 1 | WEB-01 | T-14-06 | The renamed config key is not silently aliased to the superseded one; a config naming the old key fails to load | unit | `cargo test -p paladin-ai --lib config::agents` | ✅ existing (three tests near the end of `src/config/agents.rs`) | ⬜ pending |
| 14-01 T2 | 14-01 | 1 | WEB-01 | T-14-05 | OpenAPI security scheme matches the committed baseline after the rename (first of two baseline moves) | integration (drift guard) | `cargo test -p paladin-web openapi_matches_committed_baseline` | ✅ existing (drift-guard test module in `crates/paladin-web/src/openapi.rs`) | ⬜ pending |
| 14-01 T2 | 14-01 | 1 | WEB-01 | T-14-05 | Public export surface matches `.project/current-exports.txt` after the config-type rename | integration (CI script) | `./scripts/check-api-surface.sh` | ✅ existing (`scripts/check-api-surface.sh`) | ⬜ pending |
| 14-01 T2 | 14-01 | 1 | WEB-01 | T-14-01, T-14-02 | Constant-time credential comparison and the non-echoing 401 body survive the rename | unit + router-level | `cargo test -p paladin-web` | ✅ existing (test module in `crates/paladin-web/src/agent_auth.rs`) | ⬜ pending |
| 14-01 T3 | 14-01 | 1 | WEB-01 | T-14-03 | No superseded-acronym prose survives on the four non-Rust surfaces the compiler cannot check | source assertion | `grep -ci 'jwt'` over the four files, each reporting 0 | ✅ existing files | ⬜ pending |
| 14-02 T1 | 14-02 | 1 | WEB-03 | T-14-07, T-14-08 | Both capability flags match actual request/response-surface reachability across all shipped adapters, pinned by two separate named constants | unit | `cargo test -p paladin-llm --features openai,anthropic,deepseek test_capabilities_tool_calling_matches_request_surface` (extended) | ✅ existing (`capability_invariants` module in `crates/paladin-llm/src/lib.rs`) | ⬜ pending |
| 14-02 T2 | 14-02 | 1 | WEB-04 / D-15a | **T-13-20 → closed (AR-13-01)** | Sidecar doc-example route literal matches `paladin_web::agent_controller::API_V1_PREFIX` | unit | `cargo test -p paladin-doc-examples sidecar_example_route_matches_api_v1_prefix` (new) | ❌ **W0** — created by this task | ⬜ pending |
| 14-03 T1 | 14-03 | 1 | WEB-04 | T-14-10 | The reachability rustdoc compiles and its illustrative example still runs | doctest | `cargo test -p paladin-ports --doc` | ✅ existing (doctests enabled by Phase 8 / DEBT-03) | ⬜ pending |
| 14-03 T2 | 14-03 | 1 | WEB-04 | T-14-11 | The provider-author template declares the capability honestly | source assertion | `grep -c 'supports_function_calling: false' docs/src/contributing/contributing-providers.md` is 1 | ✅ existing file | ⬜ pending |
| 14-04 T1 | 14-04 | 2 | WEB-02 | T-14-13, T-14-15 | Startup WARN fires whenever the in-process token store is wired, observed through a capturing logger | unit | `cargo test --bin paladin-server --features web-server build_auth_config_warns_when_in_process_token_store_is_wired` | ❌ **W0** — created by this task | ⬜ pending |
| 14-04 T1 | 14-04 | 2 | REQ-fail-closed-auth-posture (D-15b) | T-14-14 | `build_auth_config` returns `Err` when auth is enabled with an empty API-key list and the token verifier disabled | unit | `cargo test --bin paladin-server --features web-server build_auth_config_fails_closed_when_enabled_with_no_credentials` | ❌ **W0** — created by this task | ⬜ pending |
| 14-04 T2 | 14-04 | 2 | WEB-02 | T-14-16 | The shipped Deployment manifests are unmodified while the limitation is stated in three artefacts | source assertion | `git diff --exit-code -- k8s/server/deployment.yaml k8s/deployment.yaml` | ✅ existing files | ⬜ pending |
| 14-05 T1/T2 | 14-05 | 3 | WEB-01, WEB-02 | T-14-18 | Every path cited in each ADR's `## Code Locations` resolves on disk | CLI assertion | `test -e` sweep over every path extracted from that section | ❌ **W0** — the ADRs are created by this plan | ⬜ pending |
| 14-06 T2 | 14-06 | 2 | WEB-04 | T-14-22 | The Epic 27 correction is additive: zero original lines deleted | CLI assertion | `git diff --numstat -- .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md` reports 0 deletions | ✅ existing file | ⬜ pending |
| 14-07 T1/T2 | 14-07 | 4 | WEB-01…04 | T-14-26, T-14-29 | Every indexed ADR slug resolves, and the ledger's requirement-keyed row count is unchanged | CLI assertion | `test -e` sweep over the three indexed slugs plus a recomputed row count | ✅ existing files | ⬜ pending |
| 14-08 T2 | 14-08 | 5 | WEB-01 (D-18) | T-14-32, T-14-33 | The published contract advertises the bumped version and the drift guard is green in **checking** mode (second baseline move) | integration (drift guard) | `cargo test -p paladin-web --lib openapi_matches_committed_baseline` with the update variable **unset** | ✅ existing | ⬜ pending |
| 14-08 T1/T2 | 14-08 | 5 | WEB-01 (D-17) | T-14-31 | No version tag is created and nothing is published | CLI assertion | `git tag --list 'v0.8.0'` returns nothing | ✅ n/a | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

**Sampling continuity check:** no three consecutive tasks lack an automated verify — every task in all
eight plans carries an `<automated>` block. The three Wave 0 gaps below are each created by the task
that consumes them, so no task depends on a test that does not exist by the time it runs.

---

## Wave 0 Requirements

All three gaps are closed by the plan task that consumes them, so no task runs against a missing test.

- [ ] Unit test asserting `build_auth_config` emits its WARN when the in-process store is wired — covers WEB-02 / D-07. **Owner: plan 14-04, Task 1** (wave 2). Creates this binary's first `#[cfg(test)] mod tests`; the binary logs through the `log` crate, not `tracing`, so the test installs a capturing `log::Log` and must call `log::set_max_level` or the capture is vacuous.
- [ ] Unit test driving `build_auth_config`'s `Err` branch (enabled, empty API-key list, verifier disabled) — covers `REQ-fail-closed-auth-posture` / D-15b. **Owner: plan 14-04, Task 1** (wave 2), same new test module.
- [ ] `crates/doc-examples` test asserting the sidecar route literal against `API_V1_PREFIX` — covers D-15a. **Owner: plan 14-02, Task 2** (wave 1). **Resolved during planning:** `crates/doc-examples/Cargo.toml` depends on the root facade with the `web-server` feature, and `src/infrastructure/web/mod.rs` re-exports the web crate's public items including its `agent_controller` module — so the constant is reachable through the same import path the sibling `http_service_host.rs` example already uses, and **no dependency needs adding**.
- [ ] Framework install: **none needed** — `cargo test` is already the workspace test runner

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| The three ADRs (0040, 0041, 0042) exist with the mandated section shape and correct sequential numbering | WEB-01, WEB-02, WEB-04 | Document-shape conformance is a review judgement, not a runtime assertion | Read each ADR against D-00a's file shape (`Status / Context / Decision / Considered Options / Code Locations / Code Conformance / Downstream Consumers`, no frontmatter); confirm `PROMOTION.md:63` is updated past 0042 |
| The dated correction banner on the Deferred-QA Epic 27 source annotates rather than rewrites | WEB-04 / D-00c | Annotation-vs-rewrite is a diff-reading judgement | Confirm original text retained and marked superseded, banner names what was wrong and points at the ADR |
| Lockstep 0.8.0 bump is present in all twelve manifests | Release bookkeeping / D-18 | Manifest enumeration is a checklist, not a single assertion | `grep -rn '^version' Cargo.toml crates/*/Cargo.toml` and confirm every entry reads `0.8.0` |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] Both machine baselines regenerated in the same commits that move them (OpenAPI baseline moves **twice** — security-scheme rename and 0.8.0 bump)
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
