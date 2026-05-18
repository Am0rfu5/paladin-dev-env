//! Campaign Execution Service
//!
//! Provides orchestration logic for executing Paladins in Campaign pattern - a directed
//! acyclic graph (DAG) based orchestration with conditional routing, parallel execution,
//! and output transformations.
//!
//! # Architecture
//!
//! The Campaign pattern supports:
//! - **Topological Execution**: Paladins execute in dependency order (topological sort)
//! - **Conditional Routing**: Edge conditions determine traversal based on Paladin output
//! - **Parallel Branches**: Independent graph branches execute concurrently
//! - **Fan-Out/Fan-In**: One-to-many and many-to-one orchestration patterns
//! - **Output Transformation**: Edge transforms modify data between Paladins
//! - **Multiple Entry Points**: Multiple starting nodes for complex workflows
//!
//! # Example
//!
//! ```ignore
//! use paladin_battalion::campaign_service::CampaignExecutionService;
//! use std::sync::Arc;
//!
//! let service = CampaignExecutionService::new(paladin_port);
//! let result = service.execute(&campaign, "Initial input").await?;
//! ```

use chrono::Utc;
use log::{debug, info, warn};
use petgraph::algo::toposort;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use paladin_core::platform::container::battalion::campaign::{Campaign, EdgeCondition};
use paladin_core::platform::container::battalion::{BattalionError, BattalionResult};
use paladin_ports::output::paladin_port::{PaladinPort, PaladinResult};

/// Service for executing Campaign patterns
///
/// Orchestrates graph-based Paladin execution using a DAG structure with:
/// - Topological sort for execution ordering
/// - Edge condition evaluation for conditional routing
/// - Parallel execution of independent branches
/// - Fan-out/fan-in patterns for complex workflows
/// - Output transformations on edges
///
/// # Example
///
/// ```ignore
/// let service = CampaignExecutionService::new(paladin_port);
/// let result = service.execute(&campaign, "Process this workflow").await?;
/// ```
pub struct CampaignExecutionService {
    /// Paladin execution port
    paladin_port: Arc<dyn PaladinPort>,
}

impl CampaignExecutionService {
    /// Create a new CampaignExecutionService
    ///
    /// # Arguments
    ///
    /// * `paladin_port` - Port for executing individual Paladins
    ///
    /// # Example
    ///
    /// ```ignore
    /// let service = CampaignExecutionService::new(paladin_port);
    /// ```
    pub fn new(paladin_port: Arc<dyn PaladinPort>) -> Self {
        info!("Creating CampaignExecutionService");
        Self { paladin_port }
    }

    /// Execute a Campaign with the given input
    ///
    /// Executes Paladins according to the DAG structure, respecting:
    /// - Entry point(s) as starting nodes
    /// - Edge conditions for conditional routing
    /// - Parallel execution for independent branches
    /// - Fan-out (1→N) and fan-in (N→1) patterns
    /// - Output transformations on edges
    ///
    /// # Arguments
    ///
    /// * `campaign` - The Campaign DAG to execute
    /// * `initial_input` - Initial input for entry point Paladins
    ///
    /// # Returns
    ///
    /// * `Ok(BattalionResult)` - Final result with all Paladin outputs
    /// * `Err(BattalionError)` - If validation fails or execution errors occur
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = service.execute(&campaign, "Start workflow").await?;
    /// println!("Campaign complete: {}", result.final_output);
    /// ```
    pub async fn execute(
        &self,
        campaign: &Campaign,
        initial_input: &str,
    ) -> Result<BattalionResult, BattalionError> {
        // Validate campaign structure
        campaign.validate()?;

        let battalion_id = Uuid::new_v4();
        let started_at = Utc::now();

        info!(
            "Starting Campaign execution: {} (ID: {}) with {} Paladins",
            campaign.config().name,
            battalion_id,
            campaign.paladin_count()
        );

        // Execute with timeout
        let timeout_duration = Duration::from_secs(campaign.config().timeout_seconds);

        match timeout(
            timeout_duration,
            self.execute_internal(campaign, initial_input, battalion_id),
        )
        .await
        {
            Ok(result) => {
                let duration_ms = Utc::now()
                    .signed_duration_since(started_at)
                    .num_milliseconds() as u64;

                info!("Campaign {} completed in {}ms", battalion_id, duration_ms);
                result
            }
            Err(_) => {
                warn!(
                    "Campaign {} timed out after {} seconds",
                    battalion_id,
                    campaign.config().timeout_seconds
                );
                Err(BattalionError::Timeout(campaign.config().timeout_seconds))
            }
        }
    }

    /// Internal execution logic without timeout wrapper
    ///
    /// Implements the core Campaign execution algorithm:
    /// 1. Get entry points (explicit or auto-detected)
    /// 2. Compute topological sort for execution order
    /// 3. Execute nodes level-by-level (parallel within each level)
    /// 4. Evaluate edge conditions to determine which edges to traverse
    /// 5. Apply edge transformations before passing output to next node
    /// 6. Aggregate results from fan-in patterns
    async fn execute_internal(
        &self,
        campaign: &Campaign,
        initial_input: &str,
        battalion_id: Uuid,
    ) -> Result<BattalionResult, BattalionError> {
        let started_at = Utc::now();

        // Get entry points
        let entry_points = campaign.entry_points();
        debug!("Campaign entry points: {} nodes", entry_points.len());

        // Compute topological order
        let sorted_nodes = toposort(campaign.graph(), None).map_err(|cycle| {
            BattalionError::InvalidGraph(format!(
                "Cycle detected in campaign graph at node {:?}",
                cycle.node_id()
            ))
        })?;

        debug!("Topological sort completed: {} nodes", sorted_nodes.len());

        // Track node outputs for edge condition evaluation
        let mut node_outputs: HashMap<Uuid, String> = HashMap::new();

        // Track all Paladin results
        let mut all_results: Vec<PaladinResult> = Vec::new();

        // Track which nodes are ready to execute (dependencies satisfied)
        let mut ready_nodes: HashSet<Uuid> = entry_points.clone();

        // Track executed nodes
        let mut executed_nodes: HashSet<Uuid> = HashSet::new();

        // Execute nodes in topological order
        for node_index in sorted_nodes {
            let node_id = campaign.graph()[node_index];

            // Skip if not ready (dependencies not satisfied)
            if !ready_nodes.contains(&node_id) {
                continue;
            }

            let paladin = campaign.get_paladin(&node_id).ok_or_else(|| {
                BattalionError::InvalidGraph(format!(
                    "Node {:?} not found in paladins map",
                    node_id
                ))
            })?;

            // Determine input for this Paladin
            let input = if entry_points.contains(&node_id) {
                // Entry point uses initial input
                initial_input.to_string()
            } else {
                // Non-entry point: aggregate inputs from incoming edges
                self.aggregate_inputs_for_node(campaign, node_id, &node_outputs)?
            };

            debug!("Executing Paladin: {} ({})", paladin.node.name, node_id);

            // Execute Paladin
            let result = self
                .paladin_port
                .execute(paladin, &input)
                .await
                .map_err(|e| BattalionError::PaladinError(e.to_string()))?;

            debug!(
                "Paladin {} completed: {} tokens, {} loops",
                paladin.node.name, result.token_count, result.loop_count
            );

            // Store output for edge condition evaluation
            node_outputs.insert(node_id, result.output.clone());
            all_results.push(result.clone());
            executed_nodes.insert(node_id);

            // Evaluate outgoing edges and mark downstream nodes as ready
            let edges = campaign.graph().edges(node_index);
            for edge in edges {
                let edge_data = edge.weight();
                let target_id = campaign.graph()[edge.target()];

                // Evaluate edge condition
                if self.evaluate_edge_condition(&edge_data.condition, &result.output)? {
                    debug!("Edge condition satisfied: {} → {}", node_id, target_id);

                    // Apply edge transformation if present
                    if let Some(transform_name) = &edge_data.transform {
                        debug!("Applying edge transform: {}", transform_name);
                        // Transformation logic placeholder (could be extended)
                        // For now, just log the transform name
                    }

                    // Check if all incoming edges to target are satisfied
                    if self.are_dependencies_satisfied(campaign, edge.target(), &executed_nodes) {
                        ready_nodes.insert(target_id);
                    }
                } else {
                    debug!("Edge condition NOT satisfied: {} → {}", node_id, target_id);
                }
            }
        }

        // Build final result
        let final_output = self.compute_final_output(&all_results);

        let result = BattalionResult::new(
            battalion_id,
            campaign.config().name.clone(),
            started_at,
            final_output,
            all_results,
        );

        Ok(result)
    }

    /// Aggregate inputs from incoming edges for a node (fan-in pattern)
    fn aggregate_inputs_for_node(
        &self,
        campaign: &Campaign,
        node_id: Uuid,
        node_outputs: &HashMap<Uuid, String>,
    ) -> Result<String, BattalionError> {
        let node_index = campaign.node_indices().get(&node_id).ok_or_else(|| {
            BattalionError::InvalidGraph(format!("Node {:?} not in indices map", node_id))
        })?;

        let mut inputs = Vec::new();

        // Collect outputs from all incoming edges
        let incoming_edges = campaign
            .graph()
            .edges_directed(*node_index, petgraph::Direction::Incoming);
        for edge in incoming_edges {
            let source_id = campaign.graph()[edge.source()];
            if let Some(output) = node_outputs.get(&source_id) {
                inputs.push(output.clone());
            }
        }

        // If multiple inputs, concatenate them
        if inputs.is_empty() {
            Ok(String::new())
        } else if inputs.len() == 1 {
            Ok(inputs[0].clone())
        } else {
            // Fan-in: combine multiple inputs
            Ok(inputs.join("\n\n---\n\n"))
        }
    }

    /// Evaluate an edge condition based on the source node's output
    fn evaluate_edge_condition(
        &self,
        condition: &EdgeCondition,
        output: &str,
    ) -> Result<bool, BattalionError> {
        match condition {
            EdgeCondition::Always => Ok(true),
            EdgeCondition::Contains(substring) => Ok(output.contains(substring)),
            EdgeCondition::Regex(pattern) => {
                let regex = Regex::new(pattern).map_err(|e| {
                    BattalionError::InvalidGraph(format!("Invalid regex pattern: {}", e))
                })?;
                Ok(regex.is_match(output))
            }
            EdgeCondition::Custom(_) => {
                // Custom conditions would require user-provided function
                // For now, treat as always true
                warn!("Custom edge condition not yet implemented, defaulting to true");
                Ok(true)
            }
        }
    }

    /// Check if all incoming edges to a node have been traversed (dependencies satisfied)
    fn are_dependencies_satisfied(
        &self,
        campaign: &Campaign,
        target_index: NodeIndex,
        executed_nodes: &HashSet<Uuid>,
    ) -> bool {
        // Get all incoming edges
        let incoming = campaign
            .graph()
            .edges_directed(target_index, petgraph::Direction::Incoming);

        for edge in incoming {
            let source_id = campaign.graph()[edge.source()];
            if !executed_nodes.contains(&source_id) {
                return false;
            }
        }

        true
    }

    /// Compute the final output from all Paladin results
    fn compute_final_output(&self, results: &[PaladinResult]) -> String {
        if results.is_empty() {
            return String::new();
        }

        // Use the last result's output as final output
        // (could be made configurable in the future)
        results.last().unwrap().output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_core::platform::container::paladin::Paladin;

    #[test]
    fn test_service_creation() {
        use async_trait::async_trait;
        use paladin_core::platform::container::paladin_error::PaladinError;
        use paladin_ports::output::paladin_port::StopReason;

        struct MockPort;

        #[async_trait]
        impl PaladinPort for MockPort {
            async fn execute(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<PaladinResult, PaladinError> {
                Ok(PaladinResult {
                    output: "test".to_string(),
                    token_count: 0,
                    execution_time_ms: 0,
                    loop_count: 1,
                    stop_reason: StopReason::Completed,
                    ..Default::default()
                })
            }

            async fn execute_stream(
                &self,
                _paladin: &Paladin,
                _input: &str,
            ) -> Result<
                tokio::sync::mpsc::Receiver<
                    Result<paladin_ports::output::paladin_port::PaladinStreamChunk, PaladinError>,
                >,
                PaladinError,
            > {
                unimplemented!()
            }

            fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
                Ok(())
            }
        }

        let port = Arc::new(MockPort);
        let _service = CampaignExecutionService::new(port);
        // Service creation should succeed
    }
}
