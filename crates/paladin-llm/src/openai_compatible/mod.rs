//! Generic operator-configured OpenAI-compatible LLM provider module (D-03).

pub mod adapter;

pub use adapter::{
    OpenAiCompatibleAdapter, OpenAiCompatibleCapabilitiesConfig, OpenAiCompatibleConfig,
};
