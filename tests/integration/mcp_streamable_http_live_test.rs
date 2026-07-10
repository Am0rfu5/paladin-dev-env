//! Gated live probe against the OFFICIAL `mcp.etherscan.io` Streamable-HTTP
//! MCP server (Phase 12.1 Plan 04, D-06 / SC2 operator-run half).
//!
//! # `[ASSUMED]` — confirm before running
//!
//! The endpoint below (`https://mcp.etherscan.io/mcp`) and its Bearer-auth
//! scheme are `[ASSUMED]` per `12.1-RESEARCH.md`'s Assumptions Log (A1):
//! Etherscan's own primary docs page (`docs.etherscan.io/mcp`) returned an
//! HTTP 405 to a direct fetch during research, so this endpoint/scheme was
//! sourced by aggregating third-party pages, NOT Etherscan's own
//! authoritative documentation. Before running this probe:
//!
//! 1. Confirm the exact endpoint URL and auth scheme against Etherscan's own
//!    docs/dashboard (`docs.etherscan.io` or the Etherscan API dashboard) —
//!    this may have changed since research time.
//! 2. Confirm `mcp.etherscan.io` access is not still gated behind an
//!    early-access waitlist for your API key (RESEARCH.md Assumption A1) —
//!    if so, this probe will fail with an auth/access error unrelated to any
//!    code defect here.
//!
//! This test is `#[ignore]`'d and NEVER runs in CI or the default `cargo
//! test` suite — it requires a real, live `ETHERSCAN_API_KEY` and network
//! access. The phase's hermetic completion (SC2, proven in
//! `mcp_streamable_http_test.rs`) does NOT depend on this probe passing;
//! only the operator runs it, out-of-band, per D-06 (mirrors Phase 13's
//! `mcp-it`/`--ignored` convention).
//!
//! Run with:
//! ```text
//! ETHERSCAN_API_KEY=<your-key> cargo test --test lib -- --ignored mcp_streamable_http_live
//! ```
//!
//! The key is read from the environment ONLY — it is never logged, printed,
//! or included in any assertion/failure message below.

use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;

/// `[ASSUMED]` endpoint — see module doc. Confirm against Etherscan's own
/// docs/dashboard before relying on this outside a test context.
const ETHERSCAN_MCP_ENDPOINT: &str = "https://mcp.etherscan.io/mcp";

/// D-06 / SC2 operator-run half: a live, authenticated Streamable-HTTP
/// round-trip (`initialize` + `tools/list`) against the real hosted
/// Etherscan MCP server. `#[ignore]`'d — never executed by the default
/// suite or CI.
#[tokio::test]
#[ignore = "operator-run only: requires a live ETHERSCAN_API_KEY + confirmed mcp.etherscan.io access -- see module doc's [ASSUMED] caveat"]
async fn mcp_streamable_http_live() {
    // Fail LOUD (not silently pass) if the operator forgot to export the key
    // before running with --ignored -- never logs the key itself, only
    // whether the env var is present.
    let api_key = std::env::var("ETHERSCAN_API_KEY").unwrap_or_else(|_| {
        panic!(
            "ETHERSCAN_API_KEY is not set. This live probe requires a real Etherscan API key \
             (see this file's module doc for the [ASSUMED] endpoint/auth caveat). Set it and \
             re-run with: ETHERSCAN_API_KEY=<key> cargo test --test lib -- --ignored \
             mcp_streamable_http_live"
        )
    });

    let client = MCPClient::connect_streamable_http(ETHERSCAN_MCP_ENDPOINT, Some(&api_key), None)
        .await
        .expect(
            "authenticated Streamable-HTTP handshake against mcp.etherscan.io failed -- confirm \
             the endpoint/Bearer scheme against Etherscan's own docs (module doc [ASSUMED] \
             caveat) and that this API key has mcp.etherscan.io access approved (early-access \
             gating per RESEARCH.md Assumption A1)",
        );

    let tools = client
        .discover_tools()
        .await
        .expect("tools/list against the live Etherscan MCP server failed");
    assert!(
        !tools.is_empty(),
        "expected the live Etherscan MCP server to expose at least one tool"
    );
}
