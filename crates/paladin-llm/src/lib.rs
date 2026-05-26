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

pub mod config;
pub mod error;
pub mod llm_analysis_service;
pub mod provider_factory;

#[cfg(feature = "openai")]
pub mod openai;

#[cfg(feature = "anthropic")]
pub mod anthropic;

#[cfg(feature = "deepseek")]
pub mod deepseek;

#[cfg(feature = "mock")]
pub mod mock;
