---
phase: 14-api-contract-truthfulness
plan: 01
subsystem: auth
tags: [rust, axum, openapi, utoipa, bearer-token, config-rename, breaking-change]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth
    provides: "verified defect record naming WEB-01 — the tree carries the Milestone 12 vocabulary (JWT) and the Milestone 9 mechanism (opaque in-process token) at once"
provides:
  - "One token vocabulary (opaque server-issued bearer token) across config, binary wiring, web middleware, published OpenAPI contract, example config, Kubernetes ConfigMap, deployment-topology docs and the codebase map"
  - "paladin::config::agents::BearerTokenAuthConfig (renamed from JwtAuthConfig, clean break, no serde alias)"
  - "paladin_web::AgentAuthConfig::token_verifier (renamed from .jwt)"
  - "paladin_web::openapi::SEC_BEARER_TOKEN = \"bearer_token\" (renamed from SEC_JWT = \"jwt\", .bearer_format(\"JWT\") hint dropped)"
  - "Regenerated crates/paladin-web/openapi.json and .project/current-exports.txt baselines, both verified idempotent"
  - "BREAKING entries in CHANGELOG.md and crates/paladin-web/CHANGELOG.md pointing at ADR-0040"
affects: [14-02, 14-03, 14-04, 14-05, 14-06, 14-07, 14-08]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Clean-break rename with no compatibility alias (D-02/D-03/D-04) — a superseded config key fails to deserialize rather than being silently accepted"
    - "Regenerate machine baselines in the same commit that moves them (D-20)"

key-files:
  created: []
  modified:
    - src/config/agents.rs
    - src/config/mod.rs
    - src/bin/paladin-server.rs
    - crates/paladin-web/src/agent_auth.rs
    - crates/paladin-web/src/openapi.rs
    - crates/paladin-web/src/agent_controller.rs
    - crates/paladin-web/openapi.json
    - .project/current-exports.txt
    - examples/http_service_host.rs
    - tests/paladin_server_smoke.rs
    - tests/web_server_e2e.rs
    - config.example.yml
    - k8s/server/configmap.yaml
    - docs/src/deployment-topologies/http-service-host.md
    - .planning/codebase/ARCHITECTURE.md
    - crates/paladin-web/CHANGELOG.md
    - CHANGELOG.md

key-decisions:
  - "Checkpoint Task 1 (replacement identifier vocabulary) resolved before this agent was dispatched: the human operator selected option-a (the plan's proposed descriptive vocabulary table) via the orchestrator's interactive AskUserQuestion checkpoint at the start of Wave 1, 2026-08-12, before any code was dispatched. Applied verbatim, all eleven rows including removal of the .bearer_format(\"JWT\") hint."
  - "src/config/mod.rs's public re-export of JwtAuthConfig, and three downstream construction sites (examples/http_service_host.rs, tests/paladin_server_smoke.rs, tests/web_server_e2e.rs) were not in the plan's files_modified list but were required for the rename to compile; fixed under deviation Rule 3 (blocking compile fix)."

patterns-established:
  - "AgentAuthConfig field and OpenAPI scheme-id renames trace through the whole stack (config -> binary wiring -> middleware -> published contract) in one atomic commit, since a partial rename does not compile"

requirements-completed: [WEB-01]

coverage:
  - id: D1
    description: "One token vocabulary (opaque server-issued bearer token) spans config, binary wiring, web middleware and the published OpenAPI contract, with no residue of the superseded acronym"
    requirement: "WEB-01"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-ai --lib config::agents (7 tests)"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-web (117 unit + 5 integration tests, including openapi::tests::openapi_matches_committed_baseline)"
        status: pass
      - kind: other
        ref: "grep -rci 'jwt' across all nine plan-owned Rust and non-Rust surfaces returns 0 for every file"
        status: pass
    human_judgment: false
  - id: D2
    description: "A config file naming the superseded jwt key fails to load rather than being silently accepted (no serde alias)"
    requirement: "WEB-01"
    verification:
      - kind: unit
        ref: "cargo test -p paladin-ai --lib config::agents::tests::auth_config_defaults_to_enabled_with_no_credentials, auth_config_parses_api_keys_with_roles (both updated to bearer_token key)"
        status: pass
      - kind: other
        ref: "grep -c 'serde(alias' src/config/agents.rs == 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "Both machine baselines (openapi.json, current-exports.txt) regenerated in the commits that moved them and verified idempotent on a second regeneration"
    requirement: "WEB-01"
    verification:
      - kind: other
        ref: "UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline + ./scripts/extract-public-api.sh, run twice; byte-identical openapi.json and current-exports.txt content (excluding the timestamp header line) across both runs"
        status: pass
      - kind: other
        ref: "./scripts/check-api-surface.sh"
        status: pass
    human_judgment: false
  - id: D4
    description: "Both CHANGELOG.md and crates/paladin-web/CHANGELOG.md carry a BREAKING entry under the existing ## [Unreleased] heading naming both consumer breaks and pointing at ADR-0040"
    requirement: "WEB-01"
    verification:
      - kind: other
        ref: "awk '/^## \\[Unreleased\\]/,/^## \\[0\\./' on both files: grep -c BREAKING/BearerTokenAuthConfig/token_verifier/ADR-0040 all >= 1; grep -c '^## \\[0.8.0\\]' == 0 on both"
        status: pass
    human_judgment: false
  - id: D5
    description: "Full workspace cargo test --workspace run to completion"
    verification: []
    human_judgment: true
    rationale: "Blocked by system-wide disk exhaustion (830G/875G used, 0 avail on the /workspace mount) unrelated to this plan's code changes — recorded in .planning/WINDOWS.md entry #5 (unrun-verify). The plan's own targeted <verify> commands (paladin-ai lib config::agents, full paladin-web suite, paladin-server binary build with web-server feature, the openapi drift guard, and check-api-surface.sh) all ran to completion and passed."

# Metrics
duration: ~45min
completed: 2026-08-12
status: complete
---

# Phase 14 Plan 01: End-to-end token-vocabulary rename Summary

**Renamed the agent API's JWT vocabulary to opaque server-issued bearer token across five Rust files, four non-Rust surfaces and two regenerated machine baselines, closing WEB-01's five-run-standing variant with no compatibility alias.**

## Performance

- **Duration:** ~45 min
- **Started:** 2026-08-12 (approx. 16:05 UTC, inferred from first commit at 16:45:57 UTC minus read/edit/verify time; exact start timestamp was not captured at agent spawn)
- **Completed:** 2026-08-12T16:51Z
- **Tasks:** 3 (Task 1 checkpoint pre-resolved before dispatch; Task 2 tracer; Task 3 non-Rust sweep)
- **Files modified:** 17 (11 in Task 2's commit, 6 in Task 3's commit)

## Accomplishments

- Renamed `JwtAuthConfig` → `BearerTokenAuthConfig`, `AuthConfig.jwt` → `.bearer_token`, and the YAML key `http.auth.jwt.enabled` → `http.auth.bearer_token.enabled`, with **no** `#[serde(alias = ...)]` — a config file naming the superseded key now fails to deserialize (D-02).
- Renamed `AgentAuthConfig.jwt` → `.token_verifier` throughout `crates/paladin-web/src/agent_auth.rs`: the field, `Default`, `has_credentials()`, `authenticate()`'s destructuring, the test double (`MockJwt` → `MockTokenVerifier`), the test (`valid_jwt_authenticates` → `valid_bearer_token_authenticates`) and every construction site, while `ct_eq`'s byte-wise constant-time comparison (occurrence count unchanged: 2) and the bearer-then-API-key credential precedence were left untouched (D-04).
- Renamed `SEC_JWT` (`"jwt"`) → `SEC_BEARER_TOKEN` (`"bearer_token"`) in `crates/paladin-web/src/openapi.rs` and **dropped** `.bearer_format("JWT")` entirely — an opaque token carries no registered format (D-03).
- Updated all eight `#[utoipa::path]` `security(...)` annotations in `agent_controller.rs` from `("jwt" = [])` to `("bearer_token" = [])`, leaving `("api_key" = [])` and its ordering untouched (D-05).
- Updated `build_auth_config` in `src/bin/paladin-server.rs`: config field access, the wiring comment (now describing the in-process opaque-token store), the fail-closed `Err` message, and the enabled-path log suffix (`" + JWT"` → `" + bearer token"`).
- Regenerated both machine baselines in the same commit that moved them (D-20): `crates/paladin-web/openapi.json` via `UPDATE_OPENAPI=1 cargo test -p paladin-web openapi_matches_committed_baseline`, and `.project/current-exports.txt` via `./scripts/extract-public-api.sh`. Both verified idempotent by running the regeneration a second time and diffing byte-for-byte (current-exports.txt's generated-timestamp header line is the one permitted difference).
- Swept the four non-Rust surfaces the compiler cannot check (`config.example.yml`, `k8s/server/configmap.yaml`, the deployment-topology doc, `.planning/codebase/ARCHITECTURE.md`) and added BREAKING entries to both changelogs pointing at ADR-0040.

## Task Commits

1. **Task 1: Ratify the replacement identifier vocabulary** — resolved before dispatch (see Checkpoint Status below); no code commit, recorded here per the plan's output instruction.
2. **Task 2: End-to-end token-vocabulary rename** — `1f8a1d1` (feat)
3. **Task 3: Sweep non-Rust surfaces, record BREAKING change** — `51deeeb` (docs)

**Plan metadata:** commit pending (this SUMMARY + STATE.md/ROADMAP.md are updated by the orchestrator after all wave agents complete, per worktree-mode instructions — see `<parallel_execution>` in this agent's dispatch).

## Checkpoint Status

**Task 1 (`checkpoint:decision`, gate="blocking") — RESOLVED before this agent was dispatched.**

- **Selected:** option-a — "Adopt the proposed descriptive vocabulary (recommended)" — the plan's full eleven-row "Replacement identifier vocabulary" table applied verbatim, with no amendments, including removal of the `.bearer_format("JWT")` hint.
- **How obtained:** the `/gsd-execute-phase` orchestrator presented the plan's three options (option-a / option-b / option-c) verbatim to the human operator in an interactive `AskUserQuestion` checkpoint prompt at the start of Wave 1, on 2026-08-12, before any code was dispatched to this agent.
- **Who decided:** the human operator (the repository owner running this session), not Claude.
- **Applied as ratified:** every one of the eleven rows in the vocabulary table landed exactly as specified — YAML key, config struct, config field, web struct field, OpenAPI const, OpenAPI scheme-id value, handler annotation, test double, test fn, enabled-log suffix, and the OpenAPI format-hint removal.

## Files Created/Modified

- `src/config/agents.rs` — renamed `JwtAuthConfig` struct and `AuthConfig.jwt` field to `BearerTokenAuthConfig`/`.bearer_token`; updated three tests
- `src/config/mod.rs` — updated the crate-root re-export list (deviation; see below)
- `src/bin/paladin-server.rs` — renamed local bindings, wiring comment, fail-closed error message, log suffix
- `crates/paladin-web/src/agent_auth.rs` — renamed `AgentAuthConfig.jwt` field, mock, test, and four doc-comment sites
- `crates/paladin-web/src/openapi.rs` — renamed the security-scheme const/value, dropped the format hint, updated module doc and test assertion
- `crates/paladin-web/src/agent_controller.rs` — renamed the scheme id in all eight handler `security(...)` annotations
- `crates/paladin-web/openapi.json` — regenerated published-contract baseline
- `.project/current-exports.txt` — regenerated api-surface baseline
- `examples/http_service_host.rs`, `tests/paladin_server_smoke.rs`, `tests/web_server_e2e.rs` — updated `AgentAuthConfig` construction sites (deviation; see below)
- `config.example.yml` — renamed the `jwt:` key to `bearer_token:`, rewrote the preceding comment
- `k8s/server/configmap.yaml` — renamed the same key
- `docs/src/deployment-topologies/http-service-host.md` — rewrote the credential sentence in "Authentication & authorization"
- `.planning/codebase/ARCHITECTURE.md` — corrected the Data Flow auth-middleware step and the Authentication bullet
- `crates/paladin-web/CHANGELOG.md`, `CHANGELOG.md` — added BREAKING entries under `## [Unreleased]`

## Decisions Made

- Applied the human-ratified vocabulary table exactly as written (see Checkpoint Status above) — no substitutions or amendments.
- ADR-0040 does not yet exist in this worktree (it is produced by sibling plan 14-05 in this same phase's wave structure); both CHANGELOG entries point at it per the plan's explicit instruction, as a forward reference within the phase's own plan set.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed `src/config/mod.rs`'s stale re-export after the config struct rename**
- **Found during:** Task 2 (`cargo check -p paladin-ai --lib`)
- **Issue:** `src/config/mod.rs` re-exports `JwtAuthConfig` from `crate::config::agents` (`pub use crate::config::agents::{..., JwtAuthConfig, ...}`); this file was not in the plan's `files_modified` list but the compiler surfaced it as `error[E0432]: unresolved import` the moment the struct was renamed.
- **Fix:** Updated the re-export list to `BearerTokenAuthConfig`.
- **Files modified:** `src/config/mod.rs`
- **Verification:** `cargo check -p paladin-ai --lib` passes; `.project/current-exports.txt` regeneration confirms `paladin::config::BearerTokenAuthConfig` is now the public re-export path.
- **Committed in:** `1f8a1d1` (Task 2 commit)

**2. [Rule 3 - Blocking] Fixed three downstream `AgentAuthConfig { jwt: None, ... }` construction sites**
- **Found during:** Task 2 (broad grep sweep for remaining `jwt`/`JWT` usages across the whole repo, run proactively after the primary file edits)
- **Issue:** `examples/http_service_host.rs`, `tests/paladin_server_smoke.rs`, and `tests/web_server_e2e.rs` each construct an `AgentAuthConfig` struct literal with a `jwt: None` field — none of these three files were in the plan's `files_modified` list, but all three are consumers of the renamed field and would fail to compile once `AgentAuthConfig.jwt` became `.token_verifier`.
- **Fix:** Updated all three construction sites to `token_verifier: None`.
- **Files modified:** `examples/http_service_host.rs`, `tests/paladin_server_smoke.rs`, `tests/web_server_e2e.rs`
- **Verification:** `cargo test --no-run --test web_server_e2e --test paladin_server_smoke --features web-server` and `cargo build --example http_service_host --features web-server` both compile cleanly; `cargo clippy --all-targets --features web-server -- -D warnings` is clean.
- **Committed in:** `1f8a1d1` (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 3 — blocking compile fixes surfaced by the compiler as direct consumers of the renamed symbols, not scope creep).
**Impact on plan:** Both fixes were necessary for the tracer task's own stated constraint — "a partial rename does not compile" — and stayed strictly within the rename's mechanical surface (field/type names only, no behavior change).

## Issues Encountered

- **System-wide disk exhaustion during verification.** After Task 2's commit, `/workspace`'s underlying filesystem reported 830G/875G used with 0 bytes available (`df -h /workspace`), which is far larger than this worktree's own `target/` (6.8G) or even all three active worktrees' `target/` dirs combined (~15G) — the exhaustion originates outside this worktree's visibility, on the shared underlying block device. This first surfaced as cascading `cargo test --workspace` compile failures (`error: couldn't create a temp dir: No space left on device`), and then as `git add`/`git commit` failing outright with `fatal: unable to write loose object file: No space left on device` when staging Task 3's files.
  - **Resolution:** removed this worktree's own `target/debug/incremental` compiler cache (3.1G, purely a rebuild-speed optimization, not required for correctness) via `rm -rf`, which freed enough space (3.1G available afterward) to complete staging and committing. This is not a `git clean`/`git reset` operation and does not touch any git-tracked or other-worktree state — it is a local cargo build-cache deletion within this agent's own worktree.
  - **Residual scope:** the plan's own literal `<verify>` automated command block (both Task 2's and Task 3's) ran to completion and passed in full, as did every acceptance-criteria grep/python check. The broader top-level `<verification>` section's `cargo test` (full workspace) line item was not run to completion — recorded as an `unrun-verify` entry (#5) in `.planning/WINDOWS.md` with the disk-exhaustion root cause, so it stays visible at ship time rather than silently passing.
- **Snyk code scan unavailable.** Neither a `snyk` CLI nor an MCP `snyk_code_scan` tool was reachable in this environment. Per the plan's own verification note ("if the integration is unavailable in this environment, record that fact in the SUMMARY rather than claiming a clean scan"), this is recorded as unavailable rather than a claimed clean result. `make audit` (dependency-only, not code-scan) was not run separately in this plan since no `[dependencies]` entry was added (confirmed by the threat model's T-14-SC disposition, re-verified: `grep -rn "jsonwebtoken" Cargo.toml crates/*/Cargo.toml` returns nothing).

## User Setup Required

None — no external service configuration required.

## Known Stubs

None. No placeholder values, empty-data components, or unwired data sources were introduced by this rename.

## Threat Flags

None. This plan renames identifiers and prose only; T-14-01 through T-14-06 and T-14-SC in the plan's own threat model are all disposed as "mitigate" or "accept" by the rename's own structure (verified: `ct_eq` occurrence count unchanged, credential precedence unchanged, no new dependency, no serde alias) — no new network endpoint, auth path, file-access pattern, or schema change at a trust boundary was introduced.

## Next Phase Readiness

- WEB-01's five-run-standing variant (Milestone 12 vocabulary vs. Milestone 9 mechanism coexisting in the shipped tree) is closed: one token vocabulary now spans every named surface.
- Sibling plans in this phase's wave structure depend on artifacts this plan does **not** yet produce: `.planning/decisions/0040-opaque-bearer-token-mechanism.md` (plan 14-05) is referenced by both CHANGELOG BREAKING entries added in this plan's Task 3 as a forward pointer — 14-05 must land that ADR file before the phase's `<verification>` grep for a resolvable link would fully close.
- The `unrun-verify` ledger entry (`.planning/WINDOWS.md` #5) for `cargo test --workspace` should be re-run and resolved once the shared disk-exhaustion condition clears — likely by the orchestrator or a subsequent plan/phase with a fresh disk allocation, since it is outside this worktree's control to fix.
- No blockers for 14-02 through 14-08 specific to this plan's changes.

---
*Phase: 14-api-contract-truthfulness*
*Completed: 2026-08-12*

## Self-Check: PASSED

All 17 claimed modified files verified present on disk (`ls -la`, two batches). All three
commit hashes (`1f8a1d1`, `51deeeb`, `7d493e3`) verified present in `git log --oneline --all`.
