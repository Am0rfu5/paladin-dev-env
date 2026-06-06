//! Compiled examples for the **Deployment Topologies** book section
//! (`docs/src/deployment-topologies/`, Epic 6 of Milestone 11).
//!
//! Each `// ANCHOR: name` region is pulled into a topology page via mdBook
//! `{{#include}}`, so every example here is compiled by
//! `cargo check -p paladin-doc-examples` against the current public API. The
//! single-agent embedded example is reused from `readme.rs` (the `quickstart`
//! anchor) rather than duplicated here.
#![allow(unused_variables, unused_imports, dead_code)]

// ANCHOR: embedded_registry
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use paladin::MockLlmAdapter;
use paladin::application::services::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::infrastructure::resilience::circuit_breaker::CircuitBreaker;
use paladin::prelude::*; // PaladinBuilder, LlmPort, Paladin, PaladinResult

/// Several *distinct* agents, each with its own execution service, all resident
/// in one process. Build the registry once, then route each request to an agent
/// by name. This is the in-process foundation the HTTP-host topology serves.
pub struct AgentRegistry {
    agents: HashMap<String, (Paladin, Arc<PaladinExecutionService>)>,
}

impl AgentRegistry {
    /// Construct a registry of agents that differ by system prompt (and could
    /// differ by model, tools, or memory). One shared LLM port and circuit
    /// breaker are reused across them here; give each its own if they diverge.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let llm: Arc<dyn LlmPort> = Arc::new(MockLlmAdapter::new());
        let breaker = Arc::new(CircuitBreaker::new(5, 2, Duration::from_secs(30)));

        let mut agents = HashMap::new();
        for (name, prompt) in [
            (
                "researcher",
                "You research topics thoroughly and cite sources.",
            ),
            ("summarizer", "You write concise, faithful summaries."),
        ] {
            let agent = PaladinBuilder::new(llm.clone())
                .name(name)
                .system_prompt(prompt)
                .build()
                .await?;
            let service = Arc::new(PaladinExecutionService::new(
                llm.clone(),
                breaker.clone(),
                None, // garrison (memory) — none in this minimal example
                None, // arsenal (tools)  — none in this minimal example
            ));
            agents.insert(name.to_string(), (agent, service));
        }
        Ok(Self { agents })
    }

    /// Route an input to a named agent and run it in-process.
    pub async fn run(
        &self,
        agent: &str,
        input: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (paladin, service) = self
            .agents
            .get(agent)
            .ok_or_else(|| format!("no agent named '{agent}'"))?;
        let result: PaladinResult = service.execute(paladin, input).await?;
        Ok(result.output)
    }
}
// ANCHOR_END: embedded_registry
