//! Example: Arsenal Tool System - Streamable-HTTP MCP (authenticated remote transport)
//!
//! Demonstrates connecting to a remote, authenticated MCP server over the
//! Streamable-HTTP transport (Phase 12.1 D-02/D-03), discovering its tools,
//! and invoking one -- the real, currently-implemented API. This replaces
//! the retired `MCPSseAdapter`, whose fluent
//! `.endpoint(...).api_key(...).build()` auth API never existed in code.
//!
//! # Configuration
//!
//! Both environment variables below are optional -- if
//! `MCP_STREAMABLE_HTTP_ENDPOINT` is unset, this example prints usage and
//! exits cleanly, so `cargo run --example arsenal_streamable_http_tools` is
//! always safe to run without any server configured:
//!
//! - `MCP_STREAMABLE_HTTP_ENDPOINT` -- the remote MCP server's endpoint URL
//!   (e.g. `https://mcp.etherscan.io/mcp`).
//! - `MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV` -- NAMES another environment
//!   variable that holds the bearer token. This is an env-var REFERENCE,
//!   never the literal token itself -- mirrors the CLI's
//!   `--mcp-auth-token-env` flag and the YAML config's `auth_token_env`
//!   field (D-03). Omit entirely for an unauthenticated server.
//!
//! ```bash
//! export MCP_STREAMABLE_HTTP_ENDPOINT="https://mcp.etherscan.io/mcp"
//! export ETHERSCAN_API_KEY="..."
//! export MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV="ETHERSCAN_API_KEY"
//! cargo run --example arsenal_streamable_http_tools
//! ```
//!
//! For the underlying implementation, see:
//! - `src/infrastructure/adapters/arsenal/mcp_protocol.rs` (`MCPClient::connect_streamable_http`)
//! - `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs` (fluent builder)
//! - `tests/integration/mcp_streamable_http_test.rs` (hermetic round-trip, SC2)
//! - `tests/integration/mcp_streamable_http_live_test.rs` (`#[ignore]`'d live probe, D-06)

use paladin::infrastructure::adapters::arsenal::mcp_protocol::MCPClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("══════════════════════════════════════════════════════");
    println!("  Arsenal Streamable-HTTP MCP - Authenticated Remote Transport");
    println!("══════════════════════════════════════════════════════\n");

    println!("Streamable-HTTP MCP Transport:");
    println!("  • Remote HTTP(S) endpoint, not a spawned subprocess");
    println!("  • Optional bearer-token auth (D-03) -- token resolved from an");
    println!("    env var NAMED by another env var, never hardcoded");
    println!("  • Full initialize -> notifications/initialized handshake (D-04)");
    println!("  • Examples: hosted MCP servers (e.g. mcp.etherscan.io)\n");

    let Some(endpoint) = std::env::var("MCP_STREAMABLE_HTTP_ENDPOINT").ok() else {
        println!("MCP_STREAMABLE_HTTP_ENDPOINT is not set -- nothing to connect to.\n");
        println!("Set it (and optionally MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV, naming the");
        println!("env var that holds the bearer token) to run this example against a real");
        println!("Streamable-HTTP MCP server, e.g.:\n");
        println!("  export MCP_STREAMABLE_HTTP_ENDPOINT=\"https://mcp.etherscan.io/mcp\"");
        println!("  export ETHERSCAN_API_KEY=\"...\"");
        println!("  export MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV=\"ETHERSCAN_API_KEY\"");
        println!("  cargo run --example arsenal_streamable_http_tools\n");
        return Ok(());
    };

    // The bearer token is resolved host-side from the NAMED env var --
    // MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV never holds the secret itself, only
    // a reference to where it lives (mirrors `auth_token_env` in
    // config.yml / `--mcp-auth-token-env` on the CLI, D-03).
    let bearer_token = match std::env::var("MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV").ok() {
        Some(token_env_var) => Some(std::env::var(&token_env_var).map_err(|_| {
            format!(
                "MCP_STREAMABLE_HTTP_AUTH_TOKEN_ENV references '{token_env_var}', but that \
                 environment variable is not set"
            )
        })?),
        None => None,
    };

    println!("Step 1: Connecting to {endpoint}...");
    let client =
        MCPClient::connect_streamable_http(&endpoint, bearer_token.as_deref(), None).await?;
    println!("  ✓ Connected (initialize -> notifications/initialized handshake complete)\n");

    if let Some(caps) = client.server_capabilities() {
        println!(
            "Step 2: Server info: {} v{}\n",
            caps.server_info.name, caps.server_info.version
        );
    }

    println!("Step 3: Discovering tools via tools/list...");
    let tools = client.discover_tools().await?;
    println!("  ✓ Discovered {} tool(s)\n", tools.len());
    for tool in &tools {
        println!("  📦 {} - {}", tool.name, tool.description);
    }

    if let Some(first_tool) = tools.first() {
        println!("\nStep 4: Invoking '{}' via tools/call...", first_tool.name);
        match client.invoke_tool(&first_tool.name, HashMap::new()).await {
            Ok(result) => println!("  ✓ Result: {result}"),
            Err(e) => println!(
                "  (skipping -- this tool needs arguments this example doesn't supply: {e})"
            ),
        }
    }

    println!("\n══════════════════════════════════════════════════════");
    println!("  Key Concepts:");
    println!("    • MCPClient::connect_streamable_http replaces the never-real,");
    println!("      retired MCPSseAdapter fluent auth API");
    println!("    • Bearer tokens are sourced from env vars only -- never hardcoded");
    println!("    • Same discover_tools()/invoke_tool() surface as connect_stdio");
    println!();
    println!("  See also:");
    println!("    • tests/integration/mcp_streamable_http_test.rs");
    println!("══════════════════════════════════════════════════════");

    Ok(())
}
