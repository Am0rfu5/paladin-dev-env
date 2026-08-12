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
| TBD | TBD | TBD | WEB-01 | — | Renamed config key is not silently aliased to the old `jwt` key | unit | `cargo test -p paladin --lib config::agents` | ✅ existing (`src/config/agents.rs:306-334`) | ⬜ pending |
| TBD | TBD | TBD | WEB-01 | — | OpenAPI security scheme matches the committed baseline after the rename | integration (drift guard) | `cargo test -p paladin-web openapi_matches_committed_baseline` | ✅ existing (`crates/paladin-web/src/openapi.rs:120-125`) | ⬜ pending |
| TBD | TBD | TBD | WEB-01 | — | Public export surface matches `.project/current-exports.txt` after the rename | integration (CI script) | `./scripts/check-api-surface.sh` | ✅ existing (`scripts/check-api-surface.sh`) | ⬜ pending |
| TBD | TBD | TBD | WEB-02 | — | Startup warning fires when the in-process token store is wired | unit | new test around `build_auth_config` asserting the WARN line | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | REQ-fail-closed-auth-posture (D-15b) | T-13-20 adjacent | `build_auth_config` returns `Err` when auth is enabled with no credentials | unit | new test: `AuthConfig { enabled: true, api_keys: vec![], jwt: JwtAuthConfig { enabled: false } }` → `is_err()` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | WEB-03 | — | `supports_function_calling` matches actual request-surface reachability across all shipped adapters | unit | `cargo test -p paladin-llm test_capabilities_tool_calling_matches_request_surface` (extended) | ✅ existing (`crates/paladin-llm/src/lib.rs:98-136`) | ⬜ pending |
| TBD | TBD | TBD | D-15a | T-13-20 | Sidecar doc-example route literal matches `paladin_web::agent_controller::API_V1_PREFIX` | unit | `cargo test -p doc-examples sidecar_example_route_matches_api_v1_prefix` (new) | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] Unit test asserting `build_auth_config` emits its WARN when the in-process store is wired (`cfg.jwt.enabled` or its renamed equivalent is `true`) — covers WEB-02 / D-07
- [ ] Unit test driving `build_auth_config`'s `Err` branch (enabled, no credentials) — covers `REQ-fail-closed-auth-posture` / D-15b
- [ ] `crates/doc-examples` test asserting the sidecar route literal against `API_V1_PREFIX` — covers D-15a. Confirm `crates/doc-examples/Cargo.toml` already depends on `paladin-web` before adding the import
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
