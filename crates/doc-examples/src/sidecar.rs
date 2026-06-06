//! Compiled example for `docs/src/deployment-topologies/sidecar.md`
//! (Epic 6 of Milestone 11).
//!
//! Pulled into the page via mdBook `{{#include}}`. Paladin ships no IPC/RPC
//! transport, so the sidecar pattern is composed from the HTTP service host
//! (server side) plus a plain HTTP client (this caller side). The `reqwest`
//! call compiles but is never executed by the check gate.
#![allow(unused_variables, unused_imports, dead_code)]

// ANCHOR: sidecar_client
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ExecuteRequest {
    input: String,
}

#[derive(Deserialize)]
struct ExecuteResponse {
    output: String,
}

/// Call an agent that runs in a *separate process* (a sidecar) over HTTP. The
/// wire contract is the one the [HTTP service host] defines —
/// `POST /agents/{id}/execute` — because Paladin provides no first-class sidecar
/// transport. The contract (URL shape, request/response types) is yours to own.
pub async fn call_sidecar_agent(
    base_url: &str,
    agent: &str,
    input: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp: ExecuteResponse = client
        .post(format!("{base_url}/agents/{agent}/execute"))
        .json(&ExecuteRequest {
            input: input.to_string(),
        })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(resp.output)
}
// ANCHOR_END: sidecar_client
