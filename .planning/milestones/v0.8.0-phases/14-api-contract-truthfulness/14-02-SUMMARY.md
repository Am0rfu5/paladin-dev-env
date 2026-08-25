---
phase: 14-api-contract-truthfulness
plan: 02
subsystem: api
tags: [llm-adapters, provider-capabilities, correspondence-test, mdbook-docs, drift-guard, tdd]

# Dependency graph
requires:
  - phase: 13-milestone-9-12-ground-truth-recorded-account
    provides: ADR-0037 (agent route surface is /v1), 13-SECURITY.md's AR-13-01 accepted-risk entry naming this plan as the closure owner for T-13-20
provides:
  - OpenAIAdapter::get_capabilities().supports_function_calling now returns false, matching that generate() never returns a populated function_call
  - test_capabilities_tool_calling_matches_request_surface extended in place to pin both supports_tool_calling and supports_function_calling against two separate named surface constants, in one correspondence test
  - crates/doc-examples/src/sidecar.rs's compiled example now calls the /v1-prefixed agent route, with sidecar_example_route_matches_api_v1_prefix guarding it against paladin::infrastructure::web::agent_controller::API_V1_PREFIX
affects: [phase-15-pipe, gsd-secure-phase-13-rerun]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Correspondence test extended in place (not forked) to pin a second capability flag against a second named surface constant, inside the same for-loop/assert_eq! shape"
    - "Route literal + drift guard: the compiled mdBook-included example repeats its own route literal in a #[cfg(test)] const outside the ANCHOR region and asserts it against the framework's named route constant (API_V1_PREFIX), following openapi.rs's spec_paths_are_versioned_under_v1 precedent"

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/lib.rs
    - crates/paladin-llm/src/openai/adapter.rs
    - crates/doc-examples/src/sidecar.rs

key-decisions:
  - "supports_function_calling flipped to false on OpenAIAdapter only; mock.rs left untouched since it already declared both flags false (per CONTEXT.md's Claude's-Discretion note, answered no this phase; deferred to ADR-0042/plan 14-06)"
  - "Sidecar route drift guard implemented as a repeated literal in the test module (not a shared const), because the acceptance criteria require the production format! literal to appear exactly once in the ANCHOR region verbatim -- a shared const would split the literal and fail that criterion"
  - "/gsd-secure-phase 13 re-run (to move T-13-20 from accept/AR-13-01 to closed) is NOT run by this worktree-isolated executor -- it would modify .planning/phases/13-.../13-SECURITY.md, a file outside this plan's files_modified list and outside worktree isolation scope. Recorded here as the outcome for the orchestrator/user to trigger post-merge."

requirements-completed: [WEB-03]

coverage:
  - id: D1
    description: "OpenAIAdapter::get_capabilities().supports_function_calling returns false, matching that generate() always hard-codes function_call: None in its response"
    requirement: "WEB-03"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/openai/adapter.rs#tests::test_get_capabilities"
        status: pass
      - kind: unit
        ref: "crates/paladin-llm/src/lib.rs#capability_invariants::test_capabilities_tool_calling_matches_request_surface"
        status: pass
    human_judgment: false
  - id: D2
    description: "The correspondence test pins both supports_tool_calling and supports_function_calling for all three shipped adapters (OpenAI, Anthropic, DeepSeek) against two separate named constants inside the same test, not a forked parallel test"
    requirement: "WEB-03"
    verification:
      - kind: unit
        ref: "crates/paladin-llm/src/lib.rs#capability_invariants::test_capabilities_tool_calling_matches_request_surface"
        status: pass
    human_judgment: false
  - id: D3
    description: "crates/doc-examples/src/sidecar.rs's compiled example calls the versioned /v1/agents/{agent}/execute route instead of the unversioned form, closing threat T-13-20 / AR-13-01"
    verification:
      - kind: unit
        ref: "crates/doc-examples/src/sidecar.rs#tests::sidecar_example_route_matches_api_v1_prefix"
        status: pass
    human_judgment: false
  - id: D4
    description: "The route drift guard (sidecar_example_route_matches_api_v1_prefix) lives outside the mdBook ANCHOR region so docs/src/deployment-topologies/sidecar.md's rendered content is unchanged in structure by the added test"
    verification:
      - kind: unit
        ref: "crates/doc-examples/src/sidecar.rs#tests::sidecar_example_route_matches_api_v1_prefix"
        status: pass
    human_judgment: false

duration: 55min
completed: 2026-08-12
status: complete
---

# Phase 14 Plan 02: OpenAI Function-Calling Flag + Sidecar Route Drift Guard Summary

**Flipped OpenAIAdapter's over-reporting `supports_function_calling` flag to `false` and pinned it alongside `supports_tool_calling` in one extended correspondence test; corrected the sidecar doc example's route to `/v1`-prefixed and added a drift-guard test tying it to `API_V1_PREFIX`.**

## Performance

- **Duration:** ~55 min (majority spent on cold Rust workspace compilation across two full builds — `paladin-llm` and the `paladin-doc-examples` umbrella crate — in a resource-shared worktree)
- **Completed:** 2026-08-12
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- `OpenAIAdapter::get_capabilities().supports_function_calling` now returns `false`, matching that `generate()` hard-codes `function_call: None` in every response it builds (D-12)
- `test_capabilities_tool_calling_matches_request_surface` extended in place — not forked — to pin both `supports_tool_calling` and `supports_function_calling` for all three shipped adapters against two separate named surface constants (`REQUEST_SURFACE_SUPPORTS_TOOL_CALLING`, `RESPONSE_SURFACE_SUPPORTS_FUNCTION_CALLING`), inside the same loop and with a per-flag `assert_eq!` naming the adapter and the mismatched value
- `crates/doc-examples/src/sidecar.rs`'s compiled example now calls `{base_url}/v1/agents/{agent}/execute` instead of the unversioned form, matching ADR-0037's fixed `/v1` route surface and closing the residue `13-SECURITY.md` recorded as `AR-13-01`/`T-13-20`
- New `sidecar_example_route_matches_api_v1_prefix` test, placed after `// ANCHOR_END: sidecar_client` so the mdBook-included region (and therefore `docs/src/deployment-topologies/sidecar.md`'s rendered content) is unchanged by the added test; asserts the example's route literal against `paladin::infrastructure::web::agent_controller::API_V1_PREFIX` through the existing `paladin::infrastructure::web` facade re-export (no new dependency)

## Task Commits

Each task followed RED (failing assertion against current tree) → GREEN (fix lands, assertion passes), then a single commit per task:

1. **Task 1: Flip the asymmetric capability flag and extend the correspondence test to pin both** - `3ccf2d0` (fix)
2. **Task 2: Version the sidecar example route and guard it against API_V1_PREFIX** - `8ad9908` (fix)

_TDD detail: both tasks are `tdd="true"`. RED was verified by running the target test against the pre-fix tree and confirming failure with the exact panic message before making any fix; only after RED was confirmed was the GREEN change made and re-verified, then committed as a single task-scoped commit (this repo's convention groups RED+GREEN into one commit per task rather than separate `test:`/`feat:` commits, matching the existing single-commit-per-task pattern already used elsewhere in this phase)._

**Plan metadata:** this commit (docs: complete plan) — see final commit below.

## Files Created/Modified
- `crates/paladin-llm/src/lib.rs` - Extended `capability_invariants::test_capabilities_tool_calling_matches_request_surface` with a second named constant and a second `assert_eq!` per adapter, pinning `supports_function_calling`
- `crates/paladin-llm/src/openai/adapter.rs` - Flipped `supports_function_calling: true` → `false` in `get_capabilities()`, extended the rationale comment with a `(WEB-03, D-12)` citation, and updated `test_get_capabilities` to assert the flag is `false`
- `crates/doc-examples/src/sidecar.rs` - Corrected `call_sidecar_agent`'s request builder and doc comment to the `/v1`-prefixed route; added a `#[cfg(test)] mod tests` block after `// ANCHOR_END: sidecar_client` with `sidecar_example_route_matches_api_v1_prefix`

## Decisions Made
- `mock.rs` is left unchanged: it already declares both capability flags `false` and never emits a populated `function_call`, so it needs no correction for WEB-03/WEB-04 truthfulness. CONTEXT.md's Claude's-Discretion note about giving the mock the ability to emit one is answered "no" for this phase — recorded in ADR-0042 (owned by plan 14-06) as a possible future step instead.
- The sidecar route drift guard is implemented as a repeated literal (`SIDECAR_EXAMPLE_ROUTE` const in the test module, manually kept in sync with the production `format!` call) rather than a shared const used by both production and test code. A shared const would split the literal `{base_url}/v1/agents/{agent}/execute` across two tokens in `sidecar.rs`, failing the plan's acceptance criterion requiring that exact literal to appear exactly once in the file. This trade-off is explicitly sanctioned by the plan's action text ("a shared const in the test module holding the path segment, **or the literal repeated with the doc comment pointing at both**").
- **`/gsd-secure-phase 13` re-run is deferred, not run by this executor.** `13-SECURITY.md`'s `AR-13-01` entry explicitly instructs: "Re-run `/gsd-secure-phase 13` once Phase 14 lands the fix to move T-13-20 from `accept` to `closed`." This worktree-isolated executor's scope is limited to `files_modified: [crates/paladin-llm/src/openai/adapter.rs, crates/paladin-llm/src/lib.rs, crates/doc-examples/src/sidecar.rs]`; re-running `/gsd-secure-phase 13` would write to a Phase 13 artifact outside that scope and outside this worktree's isolation contract. **Recorded outcome:** the fix this re-run would verify (the corrected `/v1` route literal plus the new `sidecar_example_route_matches_api_v1_prefix` drift guard) has landed in commit `8ad9908`. The orchestrator or user should run `/gsd-secure-phase 13` after this wave merges to record T-13-20 as `closed`.

## Deviations from Plan

None — plan executed exactly as written. One acceptance-criteria-driven correction was made and self-caught during Task 2's own verification loop (see below), not a deviation from the plan's intent.

### Auto-fixed Issues

**1. [Rule 1 - Bug] Doc comment on the sidecar test module accidentally duplicated the pinned route literal, breaking its own acceptance criterion**
- **Found during:** Task 2, immediately after committing the GREEN fix, while re-verifying acceptance criteria
- **Issue:** The first draft of the `SIDECAR_EXAMPLE_ROUTE` const's doc comment quoted the production `format!("{base_url}/v1/agents/{agent}/execute")` call verbatim for readability. That made `grep -c '{base_url}/v1/agents/{agent}/execute' crates/doc-examples/src/sidecar.rs` return `2` instead of the required `1` (one occurrence in the doc comment, one in the actual `format!` call).
- **Fix:** Reworded the doc comment to describe the `format!` call without repeating its exact literal text.
- **Files modified:** `crates/doc-examples/src/sidecar.rs`
- **Verification:** Re-ran the acceptance-criteria greps (all passed), `cargo fmt --check`, `cargo test -p paladin-doc-examples`, and `cargo clippy -p paladin-doc-examples --all-targets -- -D warnings` — all clean — before the commit was finalized.
- **Committed in:** `8ad9908` (part of Task 2's commit; caught before commit, not a separate fix commit)

---

**Total deviations:** 1 auto-fixed (self-caught pre-commit correction, not visible in the committed diff as a separate change)
**Impact on plan:** No scope creep; the fix landed inside Task 2's single commit exactly as the plan specifies.

## Issues Encountered
- Both tasks required cold, from-scratch Rust compilation in this worktree (per the memory note on worktree pre-commit cold builds): the first `paladin-llm` build took ~4m20s, and the `paladin-doc-examples` umbrella-crate build (which pulls in `paladin-ai` with the `web-server` feature, `axum`, `reqwest`, `rmcp`, `sqlx`, and most of the rest of the workspace) took ~5m44s for `cargo test` and a further ~2m for `cargo clippy`, plus a full `cargo test --workspace` (all crates, all integration tests, all doctests) and `cargo clippy --workspace --all-targets -- -D warnings` run at the end for final verification. All commands ran to completion with `EXIT_CODE=0` and zero failures; no test infrastructure issues were hit, only expected first-build latency exacerbated by concurrent sibling worktree builds (plans 14-01 and 14-03) sharing the same CPU.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- WEB-03's remaining residue (the last asymmetric capability flag) is closed; all three shipped LLM adapters now declare both `supports_tool_calling` and `supports_function_calling` as `false`, matching the request/response surface.
- Phase 13's hand-off item D-15(a) is closed in code: the sidecar example teaches the correct `/v1`-prefixed route and is guarded against future prefix drift.
- **Outstanding action for the orchestrator/user (not this executor):** re-run `/gsd-secure-phase 13` after this wave merges so `T-13-20`/`AR-13-01` in `13-SECURITY.md` is recorded `closed` rather than `accepted`.
- No blockers for sibling plans 14-01 or 14-03, or for downstream Phase 15 (PIPE) work — this plan touched only `crates/paladin-llm/src/lib.rs`, `crates/paladin-llm/src/openai/adapter.rs`, and `crates/doc-examples/src/sidecar.rs`, none of which are shared with those plans' scopes.

---
*Phase: 14-api-contract-truthfulness*
*Completed: 2026-08-12*

## Self-Check: PASSED

- FOUND: crates/paladin-llm/src/lib.rs
- FOUND: crates/paladin-llm/src/openai/adapter.rs
- FOUND: crates/doc-examples/src/sidecar.rs
- FOUND: .planning/phases/14-api-contract-truthfulness/14-02-SUMMARY.md
- FOUND commit: 3ccf2d0 (Task 1)
- FOUND commit: 8ad9908 (Task 2)
- FOUND commit: d2bf538 (plan metadata / this SUMMARY)
