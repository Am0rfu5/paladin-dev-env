//! # Paladin - Enterprise Multi-Agent Orchestration Framework
//!
//! Paladin is a Rust-based enterprise multi-agent orchestration framework built with
//! **Hexagonal Architecture** (Ports & Adapters) and **Domain-Driven Design** principles.
//! It enables the creation, coordination, and scaling of intelligent AI agents (Paladins)
//! through configurable orchestration patterns with comprehensive LLM integration.
//!
//! ## Core Concepts
//!
//! - **Paladin**: Autonomous AI agent capable of reasoning and action
//! - **Battalion**: Multi-agent coordination patterns (Formation, Phalanx, Campaign, Chain of Command)
//! - **Garrison**: Conversation memory and context storage
//! - **Arsenal**: External tool execution via MCP (Model Context Protocol)
//! - **Sanctum**: Vector storage for semantic search and RAG
//! - **Citadel**: State persistence and recovery
//!
//! ## Architecture
//!
//! The framework follows hexagonal architecture with three layers:
//!
//! - **Core Layer** (`core`): Pure business logic with zero external dependencies
//! - **Application Layer** (`application`): Use cases and port trait definitions
//! - **Infrastructure Layer** (`infrastructure`): Adapter implementations for external systems
//!
//! ## Stable Public API
//!
//! This library exports a **curated stable public API** defined in [STABLE_API.md](https://github.com/DF3NDR/paladin-dev-env/blob/main/STABLE_API.md).
//! The API follows **semantic versioning** with strong backwards compatibility guarantees:
//!
//! - **Port Traits**: Primary extension points for integrating external systems
//! - **Domain Entities**: Core business types (Paladin, Battalion, etc.)
//! - **Builders**: Fluent construction patterns
//! - **Configuration**: Application settings types
//! - **Errors**: Comprehensive error enums
//! - **Base Types**: Generic framework primitives
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use paladin::{PaladinBuilder, LlmPort};
//! use std::sync::Arc;
//!
//! # async fn example(llm_port: Arc<dyn LlmPort>) -> Result<(), Box<dyn std::error::Error>> {
//! // Create a Paladin agent
//! let paladin = PaladinBuilder::new(llm_port)
//!     .name("ResearchAgent")
//!     .system_prompt("You are a helpful research assistant.")
//!     .max_loops(5)
//!     .build()?;
//!
//! // Execute the agent
//! let result = paladin_service.execute(&paladin, "Analyze the market trends").await?;
//! println!("Response: {}", result.output);
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! - `redis-queue` (default): Redis-based async queue support
//! - `s3-storage` (default): MinIO/S3 file storage support
//! - `mysql-backend`: MySQL database backend
//! - `sqlite-backend`: SQLite database backend
//!
//! ## Documentation
//!
//! - [API Reference](https://docs.rs/paladin)
//! - [User Guides](https://github.com/DF3NDR/paladin-dev-env/tree/main/docs)
//! - [Examples](https://github.com/DF3NDR/paladin-dev-env/tree/main/examples)
//! - [Stable API Contract](https://github.com/DF3NDR/paladin-dev-env/blob/main/STABLE_API.md)
//!
//! ## Stability Guarantee
//!
//! All types exported from this module are part of the stable public API and follow
//! semantic versioning. Breaking changes will only occur in major version bumps.
//! See [STABLE_API.md](https://github.com/DF3NDR/paladin-dev-env/blob/main/STABLE_API.md) for details.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/paladin/0.1.0")]

// ============================================================================
// Module Declarations
// ============================================================================
//
// Modules are public to support tests, examples, and advanced use cases,
// but the **curated exports below** define the stable public API.
// Users should prefer the root-level re-exports over direct module access.

/// Application layer: Use cases and port trait definitions
pub mod application;

/// Configuration management
pub mod config;

/// Core domain layer: Pure business logic
pub mod core;

/// Infrastructure layer: Adapter implementations
pub mod infrastructure;

// ============================================================================
// CLI Module (Internal/Testing Support)
// ============================================================================
//
// Re-export CLI module for testing and internal tooling.
// Only available when the `cli` feature is enabled.
// This is NOT part of the stable public API.

/// CLI module for internal tooling (not part of stable API).
/// Requires the `cli` feature flag.
#[cfg(feature = "cli")]
pub use application::cli;

// ============================================================================
// Port Traits - Primary Stable API
// ============================================================================

// Output Ports (External System Integration)

pub use application::ports::output::llm_port::{
    LlmError, LlmPort, LlmRequest, LlmResponse, ProviderCapabilities, TokenUsage,
};

// Re-export PromptItem from core for LLM usage
pub use core::platform::container::prompt::PromptItem;

pub use application::ports::output::garrison_port::{
    GarrisonError, GarrisonPort, GarrisonStats, LongTermGarrisonPort,
};

pub use application::ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery, SanctumSearchResult,
};

// Re-export SanctumEntry from core for Sanctum usage
pub use core::platform::container::sanctum::SanctumEntry;

pub use application::ports::output::embedding_port::{Embedding, EmbeddingError, EmbeddingPort};

pub use application::ports::output::arsenal_port::{ArsenalPort, ArsenalRegistry};

// Re-export ArsenalError from core for Arsenal usage
pub use core::platform::container::arsenal::ArsenalError;

pub use application::ports::output::citadel_port::CitadelPort;

// Re-export CitadelError from application errors
pub use application::errors::citadel_error::CitadelError;

pub use application::ports::output::queue_port::QueuePort;

// Re-export QueueError from core for Queue usage
pub use core::platform::manager::queue_service::QueueError;

pub use application::ports::output::notification_port::{
    Notification, NotificationChannel, NotificationDeliveryPort, NotificationPortError,
    NotificationPriority, NotificationStatus, NotificationTemplate, NotificationTemplatePort,
};

pub use application::ports::output::file_storage_port::{FileStorageError, FileStoragePort};

pub use application::ports::output::paladin_port::{PaladinPort, PaladinResult, StopReason};

pub use application::ports::output::battalion_port::BattalionPort;

// Re-export Battalion types from core
pub use core::platform::container::battalion::{BattalionResult, BattalionStatus};

// Input Ports (Use Case Interfaces)

pub use application::ports::input::content_input_port::ContentIngestionPort;
pub use application::ports::input::document_port::DocumentPort;
pub use application::ports::input::ml_port::MlPort;

// ============================================================================
// Domain Entities
// ============================================================================

// Paladin (Agent) Types
pub use core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};

pub use core::platform::container::paladin_config::PaladinConfig;

// Battalion (Multi-Agent) Types
pub use core::platform::container::battalion::{BattalionConfig, BattalionError};

pub use core::platform::container::battalion::campaign::Campaign;
pub use core::platform::container::battalion::chain_of_command::ChainOfCommand;
pub use core::platform::container::battalion::formation::Formation;
pub use core::platform::container::battalion::phalanx::Phalanx;

// Memory (Garrison) Types - Not exported as domain entities yet, use ports

// Tool (Arsenal) Types
pub use core::platform::container::arsenal::{Armament, ArmamentCall, ArmamentResult};

// State (Citadel) Types - Not exported as domain entities yet, use ports

// ============================================================================
// Builder Types
// ============================================================================

pub use application::use_cases::battalion::commander::CommanderBuilder;
pub use application::use_cases::paladin::paladin_builder::PaladinBuilder;
pub use core::platform::container::battalion::council::CouncilBuilder;
pub use core::platform::container::battalion::grove::GroveBuilder;

// ============================================================================
// Configuration Types
// ============================================================================

// Configuration types are currently not exported as stable API
// Users should use builder patterns and port traits instead

// ============================================================================
// Error Types
// ============================================================================

pub use application::errors::citadel_error::CitadelError as CitadelServiceError;
pub use application::use_cases::paladin::error::PaladinError;

// ============================================================================
// Base Types (Framework Primitives)
// ============================================================================

pub use core::base::entity::collection::CollectionType;
pub use core::base::entity::field::Field;
pub use core::base::entity::message::Message;
pub use core::base::entity::node::Node;
