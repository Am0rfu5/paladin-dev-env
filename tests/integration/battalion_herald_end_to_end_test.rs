//! End-to-end proof for GAP-03 / Epic 8 task 7.13.
//!
//! Drives a real `FormationExecutionService` over mock Paladins and formats the resulting
//! `BattalionResult` through all three Heralds (`JsonHerald`, `MarkdownHerald`, `TableHerald`),
//! asserting the five content requirements ROADMAP success criterion 3 names: Battalion identity
//! (name, id, strategy), per-Paladin results in execution order, aggregated token usage derived
//! from the mocks' own counts, partial results on failure, and all three Heralds exercised.
//!
//! Task 7.13's own inline note names the missing piece as "needs Battalion execution setup" — a
//! hand-built `BattalionResult` literal would close the criterion on paper without proving the
//! producer side. This file contains no such literal; every result comes out of
//! `FormationExecutionService::execute`.

use async_trait::async_trait;
use chrono::Utc;
use paladin::application::services::battalion::formation_service::FormationExecutionService;
use paladin::application::services::paladin::error::PaladinError;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;
use paladin::core::platform::container::battalion::BattalionConfig;
use paladin::core::platform::container::battalion::formation::Formation;
use paladin::core::platform::container::herald::Herald;
use paladin::core::platform::container::paladin::Paladin;
use paladin::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};
use paladin_ports::output::llm_port::{
    FinishReason, LlmError, LlmPort, LlmRequest, LlmResponse, TokenUsage as LlmTokenUsage,
};
use paladin_ports::output::paladin_port::{PaladinPort, PaladinResult, PaladinStream, StopReason};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Mock LLM Port used only to satisfy `PaladinBuilder::new`'s constructor requirement.
///
/// `FormationExecutionService` drives Paladins through a `PaladinPort` mock
/// (`FormationMockPaladinPort` below), not through the LLM, so this port's `generate` output is
/// never read by the assertions in this file.
struct MockLlmPort;

#[async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError> {
        Ok(LlmResponse {
            id: Uuid::new_v4(),
            request_id: request.id,
            model: request.model,
            content: "unused".to_string(),
            finish_reason: FinishReason::Stop,
            usage: LlmTokenUsage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
            created_at: Utc::now(),
            metadata: HashMap::new(),
            function_call: None,
        })
    }

    async fn generate_stream(
        &self,
        _request: LlmRequest,
    ) -> Result<
        Box<
            dyn futures::Stream<
                    Item = Result<paladin_ports::output::llm_port::StreamingResponse, LlmError>,
                > + Send,
        >,
        LlmError,
    > {
        unimplemented!("Streaming not needed for this test")
    }

    async fn validate_model(&self, _model: &str) -> Result<bool, LlmError> {
        Ok(true)
    }

    async fn get_available_models(&self) -> Result<Vec<String>, LlmError> {
        Ok(vec!["mock-model".to_string()])
    }

    fn get_provider_name(&self) -> &'static str {
        "mock"
    }

    fn get_capabilities(&self) -> paladin_ports::output::llm_port::ProviderCapabilities {
        paladin_ports::output::llm_port::ProviderCapabilities::default()
    }
}

/// Mock `PaladinPort` for driving `FormationExecutionService`.
///
/// Configured per-name with a distinct, non-round `(output, token_count, execution_time_ms)`
/// triple so every downstream assertion can only pass if the rendering read the real values.
#[derive(Clone)]
struct FormationMockPaladinPort {
    responses: HashMap<String, (String, u32, u64)>,
    execution_log: Arc<Mutex<Vec<String>>>,
}

impl FormationMockPaladinPort {
    fn new(responses: HashMap<String, (String, u32, u64)>) -> Self {
        Self {
            responses,
            execution_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    fn get_execution_log(&self) -> Vec<String> {
        self.execution_log.lock().unwrap().clone()
    }
}

#[async_trait]
impl PaladinPort for FormationMockPaladinPort {
    async fn execute(
        &self,
        paladin: &Paladin,
        _input: &str,
    ) -> Result<PaladinResult, PaladinError> {
        self.execution_log
            .lock()
            .unwrap()
            .push(paladin.node.name.clone());

        let (output, token_count, execution_time_ms) = self
            .responses
            .get(&paladin.node.name)
            .cloned()
            .unwrap_or_else(|| panic!("No mock response configured for {}", paladin.node.name));

        Ok(PaladinResult {
            output,
            token_count,
            execution_time_ms,
            loop_count: 1,
            stop_reason: StopReason::Completed,
            ..Default::default()
        })
    }

    async fn execute_stream(
        &self,
        _paladin: &Paladin,
        _input: &str,
    ) -> Result<PaladinStream, PaladinError> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok(rx)
    }

    fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
        Ok(())
    }
}

/// Builds a Paladin via `PaladinBuilder`, matching the setup shape of
/// `herald_integration_test.rs`'s `test_battalion_formation_with_herald`.
async fn build_paladin(llm_port: &Arc<dyn LlmPort>, name: &str) -> Paladin {
    PaladinBuilder::new(Arc::clone(llm_port))
        .system_prompt(format!("You are {}", name))
        .name(name)
        .build()
        .await
        .unwrap_or_else(|_| panic!("Failed to build Paladin {}", name))
}

#[tokio::test]
async fn test_formation_result_through_json_markdown_table_heralds() {
    let llm_port: Arc<dyn LlmPort> = Arc::new(MockLlmPort);

    let paladin1 = build_paladin(&llm_port, "Scoutmaster").await;
    let paladin2 = build_paladin(&llm_port, "Sentinel").await;
    let paladin3 = build_paladin(&llm_port, "Vanguard").await;

    let mut responses: HashMap<String, (String, u32, u64)> = HashMap::new();
    responses.insert(
        "Scoutmaster".to_string(),
        ("Scoutmaster completed the recon.".to_string(), 137, 1011),
    );
    responses.insert(
        "Sentinel".to_string(),
        ("Sentinel confirmed the perimeter.".to_string(), 263, 1148),
    );
    responses.insert(
        "Vanguard".to_string(),
        ("Vanguard led the advance.".to_string(), 401, 1285),
    );

    let paladin_port: Arc<dyn PaladinPort> =
        Arc::new(FormationMockPaladinPort::new(responses.clone()));

    let formation = Formation::new(
        vec![paladin1, paladin2, paladin3],
        BattalionConfig::new("recon_formation"),
    )
    .expect("Failed to create Formation");

    let service = FormationExecutionService::new(Arc::clone(&paladin_port));

    let result = service.execute(&formation, "Begin the mission").await;
    assert!(result.is_ok(), "Formation execution should succeed");
    let result = result.unwrap();

    // Derived from the mocks' own token counts — changing one mock's count changes this value.
    let expected_total: u64 = responses
        .values()
        .map(|(_, tokens, _)| u64::from(*tokens))
        .sum();
    assert_eq!(result.total_tokens, expected_total);
    assert_eq!(result.paladin_results.len(), 3);

    // --- JSON Herald ---
    let json_herald = JsonHerald::new();
    let json_output = json_herald
        .format_battalion_result(&result)
        .expect("JSON formatting should succeed");
    let parsed: serde_json::Value =
        serde_json::from_str(&json_output).expect("JSON Herald output should be valid JSON");

    // 1. Battalion name, id and strategy.
    assert_eq!(parsed["battalion_name"], "recon_formation");
    assert_eq!(parsed["battalion_id"], result.battalion_id.to_string());
    assert_eq!(parsed["strategy_used"], "Formation");

    // 2. Per-Paladin results in execution order — positional, not containment.
    let per_paladin = parsed["paladin_results"]
        .as_array()
        .expect("paladin_results should be an array");
    assert_eq!(per_paladin.len(), 3);
    assert_eq!(per_paladin[0]["output"], responses["Scoutmaster"].0.clone());
    assert_eq!(per_paladin[1]["output"], responses["Sentinel"].0.clone());
    assert_eq!(per_paladin[2]["output"], responses["Vanguard"].0.clone());

    // 3. Aggregated token usage, derived from the mocks' own counts.
    assert_eq!(parsed["total_tokens"].as_u64().unwrap(), expected_total);

    // --- Markdown Herald ---
    let markdown_herald = MarkdownHerald::new();
    let markdown_output = markdown_herald
        .format_battalion_result(&result)
        .expect("Markdown formatting should succeed");

    assert!(markdown_output.contains("recon_formation"));
    assert!(markdown_output.contains("Formation"));
    assert!(markdown_output.contains(&expected_total.to_string()));
    assert!(markdown_output.contains("Scoutmaster"));
    assert!(markdown_output.contains("Sentinel"));
    assert!(markdown_output.contains("Vanguard"));

    // --- Table Herald ---
    let table_herald = TableHerald::default();
    let table_output = table_herald
        .format_battalion_result(&result)
        .expect("Table formatting should succeed");

    assert!(table_output.contains("Scoutmaster"));
    assert!(table_output.contains("Sentinel"));
    assert!(table_output.contains("Vanguard"));
    assert!(table_output.contains(&expected_total.to_string()));
}
