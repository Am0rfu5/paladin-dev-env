//! Infrastructure Layer
//!
//! This module contains all adapter implementations and external system integrations
//! following the **Hexagonal Architecture** (Ports and Adapters) pattern.
//!
//! # Architecture
//!
//! The infrastructure layer is the outermost layer:
//! - **Depends on**: Application ports and core domain
//! - **Implements**: Application port traits (interfaces)
//! - **Integrates**: External systems (LLMs, databases, file storage, queues)
//!
//! # Dependency Flow
//!
//! ```text
//! External Systems
//!        │
//!        │ HTTP/TCP/Protocol
//!        ▼
//! Infrastructure (Adapters) ──implements──> Application (Ports) ──uses──> Core
//! ```
//!
//! # Modules
//!
//! ## [adapters]
//!
//! Concrete implementations of application ports for external systems:
//!
//! ### LLM Provider Adapters ([adapters::llm])
//!
//! - **OpenAiAdapter** - OpenAI GPT models (hidden from docs, use via [`LlmPort`](paladin_ports::output::llm_port::LlmPort))
//!   - Models: gpt-4, gpt-3.5-turbo, etc.
//!   - Features: Streaming, function calling, vision
//! - **DeepSeekAdapter** - DeepSeek models (hidden from docs, use via [`LlmPort`](paladin_ports::output::llm_port::LlmPort))
//!   - Cost-effective alternative with competitive performance
//! - **AnthropicAdapter** - Claude models (hidden from docs, use via [`LlmPort`](paladin_ports::output::llm_port::LlmPort))
//!   - Models: claude-3-opus, claude-3-sonnet
//!
//! ### Memory Storage Adapters ([adapters::garrison])
//!
//! - **InMemoryGarrison** - In-process memory storage (hidden from docs, use via [`GarrisonPort`](paladin_ports::output::garrison_port::GarrisonPort))
//!   - Fast, ephemeral, suitable for development
//! - **SqliteGarrison** - SQLite-backed persistent memory (hidden from docs, use via [`GarrisonPort`](paladin_ports::output::garrison_port::GarrisonPort))
//!   - Local file storage, full-text search support
//!
//! ### Tool System Adapters ([adapters::arsenal])
//!
//! - **McpStdioAdapter** - MCP (Model Context Protocol) STDIO tools (hidden from docs, use via [`ArsenalPort`](paladin_ports::output::arsenal_port::ArsenalPort))
//!   - Command-line tool integration
//! - **McpSseAdapter** - MCP SSE (Server-Sent Events) tools (hidden from docs, use via [`ArsenalPort`](paladin_ports::output::arsenal_port::ArsenalPort))
//!   - Web-based tool integration
//!
//! ### State Persistence Adapters ([adapters::citadel])
//!
//! - **FileCitadel** - File-based state persistence (hidden from docs, use via [`CitadelPort`](paladin_ports::output::citadel_port::CitadelPort))
//!   - Local filesystem storage
//!
//! ### Queue Adapters ([adapters::queue])
//!
//! - **RedisQueueAdapter** - Redis-backed queue (hidden from docs, use via port trait)
//!   - Distributed task queue
//!   - Feature flag: `redis-queue`
//!
//! ### File Storage Adapters ([adapters::file_storage])
//!
//! - **MinioAdapter** - S3-compatible storage (hidden from docs, use via port trait)
//!   - Object storage for large files
//!   - Feature flag: `s3-storage`
//!
//! ## [repositories]
//!
//! Database repository implementations for data persistence:
//!
//! - **MySQL repositories** - Production database storage (hidden from docs, use via repository ports)
//! - **SQLite repositories** - Development and testing (hidden from docs, use via repository ports)
//!
//! ## [web]
//!
//! Web server and API implementations:
//!
//! - REST API endpoints
//! - WebSocket handlers
//! - Middleware and routing
//!
//! # Adapter Pattern
//!
//! Each adapter translates between the application's port interface and
//! an external system's API:
//!
//! ```text
//! Application Port (Interface)         External System
//!         │                                  │
//!         │ trait LlmPort                    │
//!         │   fn generate(...)               │
//!         │                                  │
//!         ▼                                  │
//!   ┌─────────────────┐                     │
//!   │  OpenAiAdapter  │─────────────────────┤
//!   └─────────────────┘   HTTPS/REST API    │
//!         │                                  │
//!         │ implements LlmPort               │
//!         │                                  │
//!         └─────► translate request          │
//!                 call OpenAI API ───────────►
//!                 ◄──────── response
//!                 translate response
//!                 return Result<Response>
//! ```
//!
//! # Feature Flags
//!
//! Infrastructure adapters use Cargo feature flags for optional dependencies:
//!
//! - `redis-queue` - Enable Redis queue adapter
//! - `s3-storage` - Enable MinIO/S3 storage adapter
//! - `integration-tests` - Enable integration test infrastructure
//!
//! # Configuration
//!
//! Adapters are configured via:
//! - Environment variables (e.g., `OPENAI_API_KEY`)
//! - Configuration files (`config.yml`)
//! - Direct instantiation in code
//!
//! # Examples
//!
//! ## Creating and Using an LLM Adapter
//!
//! ```ignore
//! use paladin::infrastructure::adapters::llm::openai_adapter::OpenAiAdapter;
//! use paladin_ports::output::llm_port::LlmPort;
//!
//! let adapter = OpenAiAdapter::new(
//!     std::env::var("OPENAI_API_KEY").unwrap(),
//!     "https://api.openai.com/v1".to_string(),
//! );
//!
//! let response = adapter.generate(&messages, &config).await?;
//! ```
//!
//! ## Using Redis Queue
//!
//! ```ignore
//! #[cfg(feature = "redis-queue")]
//! use paladin::infrastructure::adapters::queue::redis_adapter::RedisQueueAdapter;
//!
//! #[cfg(feature = "redis-queue")]
//! let queue = RedisQueueAdapter::new("redis://localhost:6379").await?;
//! ```
//!
//! ## Testing with Mock Adapters
//!
//! ```ignore
//! use paladin_ports::output::llm_port::LlmPort;
//!
//! struct MockLlmAdapter;
//!
//! #[async_trait]
//! impl LlmPort for MockLlmAdapter {
//!     async fn generate(&self, _messages: &[Message]) -> Result<Response> {
//!         Ok(Response {
//!             content: "Mock response".to_string(),
//!             // ...
//!         })
//!     }
//! }
//! ```

// Internal modules (public for testing, not part of stable API)
#[allow(missing_docs)]
pub mod adapters;
#[allow(missing_docs)]
pub mod repositories;
pub mod resilience;
#[allow(missing_docs)]
pub mod security;
#[cfg(feature = "web-server")]
#[allow(missing_docs)]
pub mod web;
