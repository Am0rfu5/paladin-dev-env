//! Citadel Port - State Persistence Abstraction
//!
//! This module defines the port trait for state persistence operations in the
//! Citadel system. The trait abstracts file system, database, or cloud storage
//! implementations following the hexagonal architecture pattern.

use async_trait::async_trait;
use uuid::Uuid;

use crate::application::errors::citadel_error::CitadelError;
use crate::core::platform::container::citadel::{BattalionState, PaladinState, StateSummary};

/// Port trait for state persistence operations
///
/// Defines the contract for saving and loading Paladin and Battalion states.
/// Implementations must be thread-safe (`Send + Sync`) for async compatibility.
///
/// # Example
///
/// ```rust,no_run
/// use paladin::application::ports::output::citadel_port::CitadelPort;
/// use paladin::infrastructure::adapters::citadel::file_citadel::FileCitadel;
/// use std::sync::Arc;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a FileCitadel adapter
/// let citadel: Arc<dyn CitadelPort> = Arc::new(
///     FileCitadel::new("./paladin-states")?
/// );
///
/// // Save a Paladin state
/// // citadel.save_paladin(&paladin_state).await?;
///
/// // Load a Paladin state by ID
/// // let state = citadel.load_paladin(&paladin_id).await?;
///
/// // List all saved states
/// let states = citadel.list_saved().await?;
/// for summary in states {
///     println!("Saved: {} at {}", summary.id, summary.created_at);
/// }
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait CitadelPort: Send + Sync {
    /// Saves a Paladin state to persistent storage
    ///
    /// Overwrites any existing state with the same Paladin ID.
    async fn save_paladin(&self, state: &PaladinState) -> Result<(), CitadelError>;

    /// Loads a Paladin state from persistent storage
    ///
    /// Returns `None` if no state exists for the given ID.
    async fn load_paladin(&self, id: Uuid) -> Result<Option<PaladinState>, CitadelError>;

    /// Saves a Battalion state to persistent storage
    async fn save_battalion(&self, state: &BattalionState) -> Result<(), CitadelError>;

    /// Loads a Battalion state from persistent storage
    async fn load_battalion(&self, id: Uuid) -> Result<Option<BattalionState>, CitadelError>;

    /// Lists all saved states with summary information
    async fn list_saved(&self) -> Result<Vec<StateSummary>, CitadelError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::entity::node::Node;
    use crate::core::platform::container::citadel::{
        BattalionConfig, CheckpointData, PaladinData, PaladinStatus,
    };
    use crate::core::platform::container::paladin::MaxLoops;

    // Mock implementation for testing trait bounds
    struct MockCitadel;

    #[async_trait]
    impl CitadelPort for MockCitadel {
        async fn save_paladin(&self, _state: &PaladinState) -> Result<(), CitadelError> {
            Ok(())
        }

        async fn load_paladin(&self, _id: Uuid) -> Result<Option<PaladinState>, CitadelError> {
            Ok(None)
        }

        async fn save_battalion(&self, _state: &BattalionState) -> Result<(), CitadelError> {
            Ok(())
        }

        async fn load_battalion(&self, _id: Uuid) -> Result<Option<BattalionState>, CitadelError> {
            Ok(None)
        }

        async fn list_saved(&self) -> Result<Vec<StateSummary>, CitadelError> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_mock_citadel_implements_trait() {
        let citadel = MockCitadel;

        let paladin_data = PaladinData {
            system_prompt: "test".to_string(),
            name: "test".to_string(),
            user_name: "test".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_loops: MaxLoops::Fixed(3),
            stop_words: vec![],
            status: PaladinStatus::Idle,
            vision_enabled: false,
        };
        let paladin = Node::new(paladin_data, Some("test".to_string()));
        let state = PaladinState::new(paladin, vec![], vec![]);

        assert!(citadel.save_paladin(&state).await.is_ok());
        assert!(citadel.load_paladin(Uuid::new_v4()).await.is_ok());

        let battalion_state = BattalionState::new(
            "Formation",
            BattalionConfig::default(),
            vec![],
            Some(CheckpointData::new()),
        );

        assert!(citadel.save_battalion(&battalion_state).await.is_ok());
        assert!(citadel.load_battalion(Uuid::new_v4()).await.is_ok());
        assert!(citadel.list_saved().await.is_ok());
    }

    #[test]
    fn test_trait_is_object_safe() {
        // This test verifies that CitadelPort can be used as a trait object
        let _: Option<Box<dyn CitadelPort>> = None;
    }
}
