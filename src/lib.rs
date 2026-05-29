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
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::invalid_html_tags)]
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

/// Prelude: convenient re-exports of the most commonly used types.
pub mod prelude;

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
// Stable Public API — Short-path Aliases
// ============================================================================
//
// Only exports with confirmed workspace consumers are listed here.
// All other types should be accessed via their full crate paths
// (e.g. `paladin_ports::`, `paladin_memory::`, `paladin_battalion::`, etc.).

// LLM adapter implementations (from paladin-llm crate)
pub use paladin_llm::provider_factory::LlmProviderFactory;

#[cfg(feature = "llm-openai")]
pub use paladin_llm::openai::{OpenAIAdapter, OpenAIConfig};

#[cfg(feature = "openai-embeddings")]
pub use paladin_llm::openai::embedding::{OpenAIEmbeddingAdapter, OpenAIEmbeddingConfig};

#[cfg(feature = "llm-anthropic")]
pub use paladin_llm::anthropic::{AnthropicAdapter, AnthropicConfig};

#[cfg(feature = "llm-deepseek")]
pub use paladin_llm::deepseek::{DeepSeekAdapter, DeepSeekConfig};

pub use paladin_llm::mock::{MockLlmAdapter, MultiStepMockLlmPort};

// Paladin (Agent) Types
pub use core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};
pub use core::platform::container::paladin_config::PaladinConfig;

// Battalion (Multi-Agent) Types
pub use core::platform::container::battalion::{BattalionConfig, BattalionError};
