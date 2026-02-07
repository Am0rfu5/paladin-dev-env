//! Application Layer
//!
//! This module contains the application's use cases, port definitions (interfaces),
//! and orchestration logic following the **Hexagonal Architecture** pattern.
//!
//! # Architecture
//!
//! The application layer sits between the core domain and infrastructure:
//! - **Depends on**: Core domain layer only
//! - **Depended on by**: Infrastructure layer
//! - **Defines ports** (interfaces) implemented by infrastructure adapters
//! - **Implements use cases** that orchestrate domain entities
//!
//! # Dependency Flow
//!
//! ```text
//! Infrastructure ──implements──> Application ──uses──> Core
//!                                     │
//!                                     │ defines
//!                                     ▼
//!                                 Ports (traits)
//! ```
//!
//! # Modules
//!
//! ## [`ports`]
//!
//! Port definitions (trait interfaces) that define contracts for external systems:
//!
//! ### Output Ports (implemented by infrastructure adapters)
//!
//! - **[`LlmPort`](ports::output::llm_port::LlmPort)** - LLM provider abstraction
//!   - Implementations: OpenAI, DeepSeek, Anthropic adapters
//! - **[`GarrisonPort`](ports::output::garrison_port::GarrisonPort)** - Memory storage
//!   - Implementations: InMemory, SQLite, Redis adapters
//! - **[`ArsenalPort`](ports::output::arsenal_port::ArsenalPort)** - Tool execution
//!   - Implementations: MCP STDIO, MCP SSE adapters
//! - **[`CitadelPort`](ports::output::citadel_port::CitadelPort)** - State persistence
//!   - Implementations: File, S3 adapters
//! - **[`PaladinPort`](ports::output::paladin_port::PaladinPort)** - Paladin execution
//! - **[`BattalionPort`](ports::output::battalion_port::BattalionPort)** - Battalion orchestration
//!
//! ### Input Ports (API boundaries)
//!
//! - Content ingestion interfaces
//! - RPC service interfaces
//!
//! ## [`use_cases`]
//!
//! Application services that implement business workflows:
//!
//! - **Paladin Services**:
//!   - [`PaladinBuilder`](use_cases::paladin::paladin_builder) - Fluent builder for Paladin creation
//!   - [`PaladinExecutionService`](use_cases::paladin::paladin_execution_service) - Execution orchestration
//!
//! - **Battalion Services**:
//!   - [`FormationService`](use_cases::battalion::formation_service) - Sequential execution
//!   - [`PhalanxService`](use_cases::battalion::phalanx_service) - Parallel execution
//!   - [`CampaignService`](use_cases::battalion::campaign_service) - Graph orchestration
//!   - [`ChainOfCommandService`](use_cases::battalion::chain_of_command_service) - Hierarchical delegation
//!   - [`Commander`](use_cases::battalion::commander) - Automatic strategy selection
//!
//! ## [`storage`]
//!
//! Repository trait definitions for data persistence.
//!
//! ## [`errors`]
//!
//! Application-level error types.
//!
//! # Port/Adapter Pattern
//!
//! ## Port Definition (Application Layer)
//!
//! ```ignore
//! use async_trait::async_trait;
//!
//! #[async_trait]
//! pub trait LlmPort: Send + Sync {
//!     async fn generate(&self, messages: &[Message]) -> Result<Response>;
//! }
//! ```
//!
//! ## Adapter Implementation (Infrastructure Layer)
//!
//! ```ignore
//! use async_trait::async_trait;
//!
//! pub struct OpenAiAdapter {
//!     api_key: String,
//!     client: reqwest::Client,
//! }
//!
//! #[async_trait]
//! impl LlmPort for OpenAiAdapter {
//!     async fn generate(&self, messages: &[Message]) -> Result<Response> {
//!         // OpenAI-specific implementation
//!     }
//! }
//! ```
//!
//! ## Dependency Injection
//!
//! ```ignore
//! let llm_port: Arc<dyn LlmPort> = Arc::new(OpenAiAdapter::new(api_key));
//! let paladin = PaladinBuilder::new(llm_port).build()?;
//! ```
//!
//! # Examples
//!
//! ## Building a Paladin with Ports
//!
//! ```ignore
//! use paladin::application::use_cases::paladin::paladin_builder::PaladinBuilder;
//! use std::sync::Arc;
//!
//! let llm_port = Arc::new(OpenAiAdapter::new(api_key));
//! let garrison_port = Arc::new(SqliteGarrison::new("garrison.db"));
//!
//! let paladin = PaladinBuilder::new(llm_port)
//!     .name("ResearchPaladin")
//!     .system_prompt("You are a research assistant")
//!     .with_garrison(garrison_port)
//!     .build()?;
//! ```
//!
//! ## Executing a Battalion
//!
//! ```ignore
//! use paladin::application::use_cases::battalion::formation_service::FormationService;
//!
//! let service = FormationService::new(paladin_port);
//! let result = service.execute(paladins, "input").await?;
//! ```

pub mod cli;
pub mod errors;
pub mod ports;
pub mod storage;
pub mod use_cases;
