---
phase: 03-verification-depth
plan: 03
subsystem: testing
tags: [rmcp, axum, mcp, streamable-http, tokio, arsenal]

# Dependency graph
requires:
  - phase: 03-01
    provides: reproduced ADR-0006 coverage pipeline baseline this plan's test additions are measured against
provides:
  - "MCPClient::connect_streamable_http_with_timeout — additive test seam for the handshake bound"
  - "Five named, passing MCP tool-invocation failure-mode tests (QUAL-04 / ROADMAP criterion 4)"
  - "Two deliberately non-spec-compliant hermetic fixture servers (malformed response, silent/never-responding)"
affects: [03-07]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Additive _with_timeout sibling method + one-line delegation from the default-bound public fn, for injecting a test-only bound without changing any existing caller's signature or behaviour"
    - "Raw axum handler (not rmcp::ServerHandler) for MCP failure modes a spec-compliant server cannot produce by construction (malformed response, no response)"
    - "Factor input-validation into a pure fn (extract_echo_message) so wire-unreachable shapes (absent arguments) can still be asserted directly"

key-files:
  created: []
  modified:
    - src/infrastructure/adapters/arsenal/mcp_protocol.rs
    - tests/integration/mcp_streamable_http_test.rs

key-decisions:
  - "Extended the existing hermetic rmcp+axum FixtureServer instead of building a parallel wiremock harness (supersedes CONTEXT.md D-11's wiremock framing per this plan's recorded decision and Research Pattern 3)"
  - "The malformed-response fixture returns its truncated tools/list body as text/plain (axum's default for a raw String), not application/json — verified against the vendored rmcp-2.1.0 client source that a malformed 200 body labelled application/json is silently treated as an accepted no-op, which would hang the test instead of failing loud"
  - "The 'arguments absent entirely' bad-arguments shape is asserted directly against the fixture's extract_echo_message helper rather than through MCPClient::invoke_tool: rmcp's CallToolRequestParams::with_arguments always wraps its map in Some(..), even when empty, so the public client API cannot construct that wire shape"

patterns-established:
  - "Pattern: _with_timeout sibling + delegation for injecting bounded-wait seams into async connect methods without touching existing signatures"

requirements-completed: [QUAL-04]

coverage:
  - id: D1
    description: "MCPClient::connect_streamable_http_with_timeout added as an additive pub async fn; connect_streamable_http reduced to a one-line delegation with the unchanged 30s constant"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_handshake_timeout_returns_timeout_error"
        status: pass
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_round_trip_with_correct_bearer_token_succeeds"
        status: pass
    human_judgment: false
  - id: D2
    description: "Rejected/expired bearer token failure mode: connecting with an issued-but-expired token returns Err(ArsenalError::AuthFailed)"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_rejects_expired_bearer_token"
        status: pass
    human_judgment: false
  - id: D3
    description: "Malformed-response failure mode: a truncated, unparseable tools/list response returns Err from discover_tools without panicking or hanging"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_malformed_tools_list_response_returns_error"
        status: pass
    human_judgment: false
  - id: D4
    description: "Handshake-timeout failure mode: a never-responding server returns Err(ArsenalError::Timeout) via the new 200ms-bound seam, sub-second wall clock"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_handshake_timeout_returns_timeout_error"
        status: pass
    human_judgment: false
  - id: D5
    description: "Unknown-tool failure mode: invoking a non-existent tool maps to an Err naming the requested tool"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_invoke_unknown_tool_maps_to_error"
        status: pass
    human_judgment: false
  - id: D6
    description: "Bad-arguments failure mode, all three shapes (absent arguments, empty map, non-string message) rejected rather than silently defaulted"
    requirement: "QUAL-04"
    verification:
      - kind: integration
        ref: "tests/integration/mcp_streamable_http_test.rs#streamable_http_invoke_with_missing_message_argument_maps_to_error"
        status: pass
    human_judgment: false

# Metrics
duration: ~15min
completed: 2026-08-02
status: complete
---

# Phase 3 Plan 03: MCP Failure-Mode Depth Summary

**All five QUAL-04 MCP tool-invocation failure modes (expired token, malformed response, handshake timeout, unknown tool, bad arguments) now have real, passing, fast tests against the shipped Streamable-HTTP transport, plus an additive `connect_streamable_http_with_timeout` test seam.**

## Performance

- **Duration:** ~15 min
- **Completed:** 2026-08-02
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments
- Added `MCPClient::connect_streamable_http_with_timeout`, an additive public method that takes an explicit `Duration` bound; `connect_streamable_http` now delegates to it with the unchanged 30-second default, so every existing caller is byte-identical
- Hardened `FixtureServer::call_tool`'s argument validation (`extract_echo_message`) to reject absent, empty, and non-string `message` arguments with `McpError::invalid_params` instead of silently defaulting to an empty echo
- Extended the bearer-token middleware to recognise a third, distinctly-named rejection case (issued-but-expired) alongside the two shipped missing/incorrect cases
- Added two deliberately non-spec-compliant hermetic fixture servers (`spawn_malformed_fixture_server`, `spawn_silent_fixture_server`) for the two failure modes a real `rmcp::ServerHandler` cannot produce by construction
- All five named failure-mode tests plus the two pre-existing auth-rejection tests pass together, back to back with no cleanup, in ~0.2s total wall clock

## Task Commits

Each task was committed atomically:

1. **Task 1: Add the additive connect_streamable_http_with_timeout seam** - `4cffe01` (feat)
2. **Task 2: Harden FixtureServer argument validation and cover three spec-compliant failure modes** - `a395ddd` (feat)
3. **Task 3: Add non-compliant fixtures and cover malformed-response and handshake-timeout** - `d2a6e9f` (feat)

_Note: this plan was type="auto" throughout — no separate plan-metadata commit is created by the sequential executor path; STATE.md/ROADMAP.md updates are captured in the final docs commit._

## Files Created/Modified
- `src/infrastructure/adapters/arsenal/mcp_protocol.rs` - Added `connect_streamable_http_with_timeout`; `connect_streamable_http` now delegates to it with the unchanged 30s constant
- `tests/integration/mcp_streamable_http_test.rs` - Hardened `FixtureServer::call_tool` argument validation; added `EXPIRED_BEARER_TOKEN`; added two non-compliant fixture servers (`spawn_malformed_fixture_server`, `spawn_silent_fixture_server`); added five new failure-mode tests

## Decisions Made
- Kept all five failure-mode tests inside the one existing `mcp_streamable_http_test.rs` file — no new file, no `tests/integration/mod.rs` edit, no `Cargo.toml` target — so the compiled test-binary object count stays stable for plan 03-07's exit re-measurement
- Chose the hand-rolled axum route over `wiremock` for the malformed-response fixture, per the plan's recorded supersession of CONTEXT.md D-11 and Research Pattern 3 (the shipped `FixtureServer` is the stronger, more realistic analog, and `wiremock` has zero prior usages in the repo)
- The `initialize` response JSON in the malformed fixture uses field names and the `protocolVersion` literal (`"2025-11-25"`) cross-checked directly against the vendored `rmcp-2.1.0` `InitializeResult`/`ProtocolVersion::LATEST` source rather than assumed from memory

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Malformed-response fixture must not label its truncated body `application/json`**
- **Found during:** Task 3 (malformed-response and handshake-timeout fixtures)
- **Issue:** The plan's action text specifies "HTTP 200 with a deliberately truncated JSON body that cannot parse" for the non-`initialize`/non-`notifications/initialized` branch. Direct verification against the vendored `rmcp-2.1.0` client source (`transport/common/reqwest/streamable_http_client.rs::post_message`) showed that when a 200 response body fails to deserialize as `ServerJsonRpcMessage` **and** its `Content-Type` header starts with `application/json`, rmcp's own client silently treats it as `StreamableHttpPostResponse::Accepted` (a no-op ack) rather than surfacing an error — a documented leniency for real notification/response edge cases. Following the plan's literal wording (200 + truncated + `application/json`) would have made `discover_tools()` hang waiting for a response that never arrives, instead of returning the `Err` the plan's own `<behavior>` block requires.
- **Fix:** The truncated body is returned as a bare axum `String`, which axum labels `text/plain; charset=utf-8` by default (not manually overridden). A non-JSON content type routes the same malformed body through rmcp's `UnexpectedContentType` error path instead, which the client surfaces immediately as an `Err` — satisfying the plan's actual behavioral contract ("returns an Err ... does not panic, hang") using an HTTP-200 truncated body exactly as specified, just without the `application/json` label that would have triggered rmcp's swallow-as-accepted leniency.
- **Files modified:** tests/integration/mcp_streamable_http_test.rs
- **Verification:** `streamable_http_malformed_tools_list_response_returns_error` passes in ~0.2s (not a hang); the file passes twice back to back
- **Committed in:** d2a6e9f (Task 3 commit)

**2. [Rule 3 - Blocking] "Absent arguments" bad-arguments shape is unreachable through MCPClient's public API**
- **Found during:** Task 2 (bad-arguments failure mode)
- **Issue:** The plan's three named bad-arguments shapes are absent `arguments`, an empty argument map, and a non-string `message`. `MCPClient::invoke_tool`'s only entry point (`HashMap<String, Value>` -> `CallToolRequestParams::with_arguments(...)`) always calls rmcp's `with_arguments`, which unconditionally sets `arguments: Some(map)` even when the map is empty (verified against the vendored rmcp-2.1.0 source). There is no way to drive a genuinely `arguments: None` request over the wire through the public client API used by this test file.
- **Fix:** Extracted the fixture's `message`-argument resolution into a pure, directly-testable helper (`extract_echo_message(Option<&Map<String, Value>>) -> Result<&str, McpError>`). The "absent arguments" shape is asserted by calling `extract_echo_message(None)` directly (documented in the test's doc comment); the "empty map" and "non-string message" shapes are asserted through the real network round-trip via `MCPClient::invoke_tool`, matching the plan's intent for those two.
- **Files modified:** tests/integration/mcp_streamable_http_test.rs
- **Verification:** `streamable_http_invoke_with_missing_message_argument_maps_to_error` exercises and asserts an `Err`/error for all three shapes; passes
- **Committed in:** a395ddd (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1/3 — correctness adjustments discovered by verifying against the vendored `rmcp` source rather than assuming library behavior). No scope creep; both keep the plan's actual behavioral contract (`<behavior>` blocks) intact where the literal `<action>` wording was technically unreachable or would have produced a hang.

## Issues Encountered
- An early draft doc comment for the handshake-timeout test explained why `#[tokio::test(start_paused = true)]` would not help (Pitfall 4), which literally contained the substring `start_paused` and tripped the plan's own acceptance-criteria grep (`grep -c 'start_paused' ... returns 0`, meant to catch actual usage of paused-time testing). Reworded the comment to describe "tokio's paused virtual-time test mode" without the literal attribute string. No behavior change; purely a doc-comment wording fix.

## Next Phase Readiness
- QUAL-04's MCP-failure-mode half is complete: all five named modes have passing, non-skipped tests, plus the additive timeout seam other future MCP tests can reuse
- No blockers for 03-07's coverage exit re-measurement: this plan touched zero new test files/targets, so the compiled test-binary object count is unchanged from entry measurement
- `mcp_protocol.rs` already measured 95.73% line coverage before this plan and this plan does not target that number further (D-11) — the two changed methods (`connect_streamable_http`, `connect_streamable_http_with_timeout`) are both exercised by the existing and new tests

---
*Phase: 03-verification-depth*
*Completed: 2026-08-02*

## Self-Check: PASSED

All created/modified files verified present on disk (`src/infrastructure/adapters/arsenal/mcp_protocol.rs`, `tests/integration/mcp_streamable_http_test.rs`, this SUMMARY). All three task commit hashes (`4cffe01`, `a395ddd`, `d2a6e9f`) verified present in `git log`.
