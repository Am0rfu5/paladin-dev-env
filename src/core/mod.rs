//! Core Domain Layer
//!
//! This module contains the pure business logic and domain entities for Paladin,
//! following Domain-Driven Design (DDD) principles. The core layer is completely
//! independent of external concerns and has no dependencies on application or
//! infrastructure layers.
//!
//! # Architecture
//!
//! The core layer follows the **Hexagonal Architecture** (Ports and Adapters) pattern:
//! - **Zero dependencies** on outer layers (application, infrastructure)
//! - Pure domain entities and business rules
//! - Framework-agnostic and testable in isolation
//!
//! # Modules
//!
//! ## [`base`]
//!
//! Foundation types and patterns used throughout the core domain:
//! - [`Node<T>`](base::entity::node::Node) - Entity wrapper with UUID, timestamps, versioning
//! - Collection management utilities
//! - Field definitions and message types
//!
//! ## [`platform`]
//!
//! Domain entities representing the Paladin multi-agent system:
//!
//! ### [`platform::container`](platform::container)
//!
//! - **[`Paladin`](platform::container::paladin::Paladin)** - Autonomous AI agent entity
//! - **[`Battalion`](platform::container::battalion)** - Multi-Paladin orchestration patterns:
//!   - [`Formation`](platform::container::battalion::formation) - Sequential execution
//!   - [`Phalanx`](platform::container::battalion::phalanx) - Parallel execution
//!   - [`Campaign`](platform::container::battalion::campaign) - Graph-based workflows
//!   - [`ChainOfCommand`](platform::container::battalion::chain_of_command) - Hierarchical delegation
//! - **[`Garrison`](platform::container::garrison)** - Memory and conversation history
//! - **[`Arsenal`](platform::container::arsenal)** - Tool system (Armaments)
//! - **[`Citadel`](platform::container::citadel)** - State persistence
//!
//! # Design Principles
//!
//! ## Domain-Driven Design (DDD)
//!
//! - **Ubiquitous Language**: Medieval military terminology (Paladin, Battalion, Garrison, Arsenal)
//! - **Bounded Contexts**: Clear boundaries between domain concepts
//! - **Aggregates**: Paladin is an aggregate root
//! - **Value Objects**: Immutable configuration types
//! - **Domain Events**: Used for cross-context communication
//!
//! ## Hexagonal Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │         Core Domain Layer           │
//! │                                     │
//! │  ┌───────────────────────────────┐ │
//! │  │  Entities & Business Logic    │ │
//! │  │  - Paladin, Battalion, etc.   │ │
//! │  └───────────────────────────────┘ │
//! │                                     │
//! │  NO DEPENDENCIES ON OUTER LAYERS    │
//! └─────────────────────────────────────┘
//! ```
//!
//! # Examples
//!
//! ## Creating a Paladin Entity
//!
//! ```
//! use paladin::core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};
//! use paladin::core::base::entity::node::Node;
//!
//! let data = PaladinData {
//!     system_prompt: "You are a helpful assistant".to_string(),
//!     name: "Assistant".to_string(),
//!     user_name: "User".to_string(),
//!     model: "gpt-4".to_string(),
//!     temperature: 0.7,
//!     max_loops: 3,
//!     stop_words: vec![],
//!     status: PaladinStatus::Idle,
//!     vision_enabled: false,
//! };
//!
//! let paladin: Paladin = Node::new(data, Some("Assistant".to_string()));
//! ```
//!
//! ## Using Garrison Memory
//!
//! ```
//! use paladin::core::platform::container::garrison::{
//!     ConversationHistory, GarrisonConfig, GarrisonEntry, ConversationRole
//! };
//!
//! let config = GarrisonConfig::default();
//! let mut history = ConversationHistory::new(config);
//!
//! history.add(GarrisonEntry::new(
//!     ConversationRole::User,
//!     "Hello!".to_string()
//! ));
//! ```

pub mod base;
pub mod platform;
