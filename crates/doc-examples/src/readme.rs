//! The Quick Example shown in the root `README.md`.
//!
//! Kept here (and compiled as part of the workspace) so the README's headline
//! snippet can never drift from the real API.
#![allow(unused_variables, unused_imports, dead_code)]

// ANCHOR: quickstart
use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin::prelude::*; // PaladinBuilder, LlmPort, Paladin, ...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An offline mock LLM so this runs without an API key.
    // For real use: `Arc::new(OpenAIAdapter::from_env()?)`.
    let llm: Arc<dyn LlmPort> =
        Arc::new(MockLlmAdapter::new().with_response("Hello from Paladin!"));

    // Build an agent with the fluent builder.
    let agent = PaladinBuilder::new(llm.clone())
        .name("Greeter")
        .system_prompt("You are a friendly assistant.")
        .build()
        .await?;

    // Execute it and print the result.
    let breaker = Arc::new(CircuitBreaker::new(5, 2, Duration::from_secs(30)));
    let service = PaladinExecutionService::new(llm, breaker, None, None);
    let result = service
        .execute(&agent, "Say hello in one sentence.")
        .await?;

    println!("{}", result.output);
    Ok(())
}
// ANCHOR_END: quickstart
