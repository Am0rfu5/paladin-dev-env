//! Shared OpenAI-compatible protocol engine (D-05, Phase 17).
//!
//! One engine owns the request/response types, streaming chunk assembly,
//! retry and error mapping shared by every OpenAI-compatible provider
//! preset (Kimi, and future presets under this feature gate). Presets supply
//! only `base_url`, credential, default model, curated model-list fallback
//! and a capabilities block.

/// Generalized OpenAI-compatible wire types (request/response/stream shapes).
pub mod types;

/// The shared engine: request building, HTTP transport, retry, error
/// mapping, credential redaction and memoized model-list resolution.
pub mod engine;

pub use engine::{CompatCapabilities, CompatEngine, CompatEngineConfig};
