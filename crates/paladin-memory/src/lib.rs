//! # paladin-memory
//!
//! Memory adapters for the Paladin framework.
//!
//! This crate provides two categories of memory storage:
//!
//! - **Garrison** (`garrison` module): Conversation history storage adapters.
//!   - [`garrison::InMemoryGarrison`] — always available, zero-dependency in-process store.
//!   - [`garrison::SqliteGarrison`] — persistent SQLite-backed store (requires feature `sqlite`).
//!   - [`garrison::TiktokenCounter`] / [`garrison::TokenCounter`] — token counting utilities
//!     (requires feature `content-processing`).
//!
//! - **Sanctum** (`sanctum` module): Vector / semantic memory adapters.
//!   - [`sanctum::InMemorySanctum`] — always available, in-process vector store.
//!   - [`sanctum::QdrantSanctumAdapter`] — production-grade Qdrant-backed store (requires feature `qdrant`).
//!
//! - **Services** (`services` module): Application-layer memory orchestration.
//!   - [`services::MemoryExtractionService`] — extracts and persists memories from conversations.
//!   - [`services::RagRetrievalService`] — retrieves context for RAG pipelines.
//!
//! ## Feature flags
//!
//! | Feature              | Enables                                          |
//! |----------------------|--------------------------------------------------|
//! | `sqlite`             | `SqliteGarrison` (depends on `sqlx`)             |
//! | `qdrant`             | `QdrantSanctumAdapter` (depends on `qdrant-client`) |
//! | `content-processing` | `TiktokenCounter`, `TokenCounter`, `TokenCounterFactory` (depends on `tiktoken-rs`) |
//!
//! No features are enabled by default.

#![deny(unsafe_code)]

pub mod garrison;
pub mod prelude;
pub mod sanctum;
pub mod services;
