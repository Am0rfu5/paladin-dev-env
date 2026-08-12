---
phase: 14-api-contract-truthfulness
reviewed: 2026-08-12T00:00:00Z
depth: standard
files_reviewed: 34
files_reviewed_list:
  - .project/Deferred-QA-CICD-Completion/prd-deferred-qa-completion.md
  - .project/current-exports.txt
  - crates/doc-examples/Cargo.toml
  - crates/doc-examples/src/sidecar.rs
  - crates/paladin-battalion/Cargo.toml
  - crates/paladin-content/Cargo.toml
  - crates/paladin-core/Cargo.toml
  - crates/paladin-herald/Cargo.toml
  - crates/paladin-llm/Cargo.toml
  - crates/paladin-llm/src/lib.rs
  - crates/paladin-llm/src/openai/adapter.rs
  - crates/paladin-memory/Cargo.toml
  - crates/paladin-notifications/Cargo.toml
  - crates/paladin-ports/Cargo.toml
  - crates/paladin-ports/src/output/llm_port.rs
  - crates/paladin-storage/Cargo.toml
  - crates/paladin-web/CHANGELOG.md
  - crates/paladin-web/Cargo.toml
  - crates/paladin-web/openapi.json
  - crates/paladin-web/src/agent_auth.rs
  - crates/paladin-web/src/agent_controller.rs
  - crates/paladin-web/src/openapi.rs
  - docs/src/architecture/domain-model.md
  - docs/src/architecture/overview.md
  - docs/src/contributing/contributing-providers.md
  - docs/src/deployment-topologies/http-service-host.md
  - docs/src/user-guides/tool-integration.md
  - examples/http_service_host.rs
  - k8s/README.md
  - k8s/server/configmap.yaml
  - src/bin/paladin-server.rs
  - src/config/agents.rs
  - src/config/mod.rs
  - tests/paladin_server_smoke.rs
  - tests/web_server_e2e.rs
findings:
  critical: 1
  warning: 0
  info: 1
  total: 2
status: issues_found
---

# Phase 14: Code Review Report

**Reviewed:** 2026-08-12T00:00:00Z
**Depth:** standard
**Files Reviewed:** 34
**Status:** issues_found

## Summary

The phase's stated purpose — make published contracts match shipped mechanisms — was executed
thoroughly and correctly across almost every touched surface. I verified the two headline
changes directly rather than trusting the diff:

- **Token vocabulary rename (WEB-01)**: `JwtAuthConfig` → `BearerTokenAuthConfig`,
  `AuthConfig.jwt` → `.bearer_token`, `paladin_web::AgentAuthConfig.jwt` → `.token_verifier`,
  `SEC_JWT` → `SEC_BEARER_TOKEN`, the OpenAPI scheme id `"jwt"` → `"bearer_token"`, and the
  `bearer_format("JWT")` hint removal are all internally consistent across
  `src/config/agents.rs`, `src/config/mod.rs`, `src/bin/paladin-server.rs`, all three
  `paladin-web` source files, `.project/current-exports.txt`, and `crates/paladin-web/openapi.json`
  (the crate's own drift-guard test — `openapi_matches_committed_baseline` — passes, confirming
  the committed baseline is byte-identical to what `openapi.rs` generates).
- **Capability honesty (WEB-03)**: `OpenAIAdapter::get_capabilities().supports_function_calling`
  is now `false`, matching the adapter's actual hard-coded `function_call: None` in
  `generate()`. The correspondence test in `crates/paladin-llm/src/lib.rs` genuinely pins both
  `supports_tool_calling` and `supports_function_calling` for all three shipped adapters.
- **Fail-closed startup refusal** in `src/bin/paladin-server.rs`'s `build_auth_config`: the
  logic (`if !auth.has_credentials() { return Err(...) }`) is correct, the single-replica
  warning fires exactly when the in-process token store is wired, and both are covered by
  tests I ran and confirmed pass.
- **Version bump**: all twelve `Cargo.toml` manifests (root + 11 crates) are lockstep at
  `0.8.0` with no stale internal path-dependency pins left at `0.7.0`.

However, the rename was **not** propagated to one integration test at the workspace root,
and running it confirms the suite is red as a result — this is exactly the "half-applied
rename across an auth boundary" failure mode the phase was scoped to catch, just in the one
place the sweep missed.

## Critical Issues

### CR-01: `tests/paladin_server_smoke.rs` still asserts the removed `"jwt"` OpenAPI scheme id — the test fails

**File:** `tests/paladin_server_smoke.rs:331`
**Issue:** This file *was* touched by the WEB-01 rename commit — the `AgentAuthConfig` field
literal at line 213 was correctly updated from `jwt: None` to `token_verifier: None` — but the
OpenAPI security-scheme assertion two tests later was missed:

```rust
let schemes = &spec["components"]["securitySchemes"];
assert!(schemes.get("api_key").is_some(), "missing api_key scheme");
assert!(schemes.get("jwt").is_some(), "missing jwt scheme");   // <-- stale
```

The scheme id was renamed to `"bearer_token"` everywhere else in this phase (`openapi.rs`'s
`SEC_BEARER_TOKEN`, the committed `openapi.json` baseline, `crates/paladin-web/src/openapi.rs`'s
own `spec_has_info_and_security_schemes` test, which was updated correctly). This one
assertion was not, so `schemes.get("jwt")` now returns `None` against the real spec and the
test panics.

I confirmed this by running the test directly:

```
$ cargo test -p paladin-ai --test paladin_server_smoke --features web-server server_serves_openapi_spec_and_docs
thread 'server_serves_openapi_spec_and_docs' panicked at tests/paladin_server_smoke.rs:331:5:
missing jwt scheme
test result: FAILED. 0 passed; 1 failed
```

This means `cargo test` (and `make test-all`) is red on this branch — the exact class of
defect this phase's own success criteria (published contract matches shipped mechanism) exists
to prevent, just manifesting as a broken *test* rather than a broken *contract*. A reviewer or
CI run that doesn't execute this specific test (e.g. relying on `cargo check` or a partial
`--lib` run) would miss it, which is likely how it shipped.

**Fix:**
```rust
assert!(schemes.get("api_key").is_some(), "missing api_key scheme");
assert!(schemes.get("bearer_token").is_some(), "missing bearer_token scheme");
```

## Info

### IN-01: `IN_PROCESS_TOKEN_STORE_WARNING` duplicates the `AgentAuthConfig` module-doc's single-replica caveat, but the two are independently maintained

**File:** `src/bin/paladin-server.rs:148-151` (cf. `crates/paladin-web/src/agent_auth.rs:1-20`,
`docs/src/deployment-topologies/http-service-host.md`, `k8s/README.md`,
`k8s/server/configmap.yaml`)
**Issue:** The single-replica / in-process-token-store caveat is now stated in prose in five
independent places (the binary's warning string, the deployment-topology doc, the k8s README
twice, and the configmap comment) plus ADR-0041. None of this is wrong today — I checked each
copy and they agree — but nothing enforces that they stay in agreement the next time the
constraint's wording or scope changes (e.g. if a future `AuthPort` implementation partially
lifts the single-replica limit for one deployment shape but not another). This is a
maintainability observation, not a functional defect; no fix is required for this phase, but a
future change to ADR-0041's scope should grep for `IN_PROCESS_TOKEN_STORE_WARNING`,
`single replica`, and `bearer_token.enabled` across these five sites rather than patching only
the binary or only the docs.
**Fix:** No action required now. Consider, in a future phase, sourcing the warning text or a
short reference key from one place (e.g. a `const` re-exported from `paladin-web` or a doc
comment on `AgentAuthConfig`) so the five copies cannot drift independently.

---

_Reviewed: 2026-08-12T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
