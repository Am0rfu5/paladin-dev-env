// src/infrastructure/adapters/llm/mod.rs
//
// LLM adapters module

#[cfg(feature = "llm-anthropic")]
pub mod anthropic_adapter;
#[cfg(all(feature = "vision", feature = "llm-anthropic"))]
pub mod anthropic_vision;
#[cfg(feature = "llm-deepseek")]
pub mod deepseek_adapter;
pub mod mock_llm_adapter;
#[cfg(feature = "llm-openai")]
pub mod openai_adapter;
#[cfg(feature = "openai-embeddings")]
pub mod openai_embedding_adapter;
#[cfg(all(feature = "vision", feature = "llm-openai"))]
pub mod openai_vision;
pub mod provider_factory;
