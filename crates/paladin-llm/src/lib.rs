//! # paladin-llm
//!
//! LLM provider adapters for the Paladin framework.
//!
//! This crate provides concrete adapter implementations for multiple LLM providers,
//! all implementing the [`paladin_ports::output::llm_port::LlmPort`] trait defined
//! in `paladin-ports`.
//!
//! ## Supported Providers
//!
//! | Feature flag | Provider | Types |
//! |---|---|---|
//! | `openai` (default) | OpenAI | [`openai::OpenAIAdapter`], [`openai::OpenAIConfig`] |
//! | `anthropic` | Anthropic | [`anthropic::AnthropicAdapter`], [`anthropic::AnthropicConfig`] |
//! | `deepseek` | DeepSeek | [`deepseek::DeepSeekAdapter`], [`deepseek::DeepSeekConfig`] |
//! | `mock` (default) | Testing | [`mock::MockLlmAdapter`], [`mock::MultiStepMockLlmPort`] |
//! | `openai-embeddings` | OpenAI Embeddings | [`openai::OpenAIEmbeddingAdapter`] |
//! | `vision` | Vision (multimodal) | Extends OpenAI and Anthropic adapters |
//!
//! ## Architecture
//!
//! Follows the Hexagonal Architecture pattern — this crate is a pure adapter
//! layer. It depends only on `paladin-core` (domain types) and `paladin-ports`
//! (port trait contracts). It has no dependency on the root `paladin` crate.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! # #[cfg(feature = "openai")]
//! # {
//! use paladin_llm::openai::{OpenAIAdapter, OpenAIConfig};
//! use paladin_llm::provider_factory::LlmProviderFactory;
//!
//! // From environment variables
//! let factory = LlmProviderFactory::new();
//! let provider = factory.create("openai").expect("OPENAI_API_KEY must be set");
//! # }
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(rustdoc::broken_intra_doc_links)]

/// Shared configuration types for LLM providers and request behavior.
#[allow(missing_docs)]
pub mod config;
/// Error types returned by provider adapters.
#[allow(missing_docs)]
pub mod error;
/// LLM-backed content analysis service orchestration.
#[allow(missing_docs)]
pub mod llm_analysis_service;
/// Factory for selecting provider adapters from runtime configuration.
#[allow(missing_docs)]
pub mod provider_factory;

#[cfg(feature = "openai")]
/// OpenAI provider adapter and related configuration.
#[allow(missing_docs)]
pub mod openai;

#[cfg(feature = "anthropic")]
/// Anthropic provider adapter and related configuration.
#[allow(missing_docs)]
pub mod anthropic;

#[cfg(feature = "deepseek")]
/// DeepSeek provider adapter and related configuration.
#[allow(missing_docs)]
pub mod deepseek;

#[cfg(feature = "mock")]
/// Mock provider adapters for tests and deterministic workflows.
#[allow(missing_docs)]
pub mod mock;
