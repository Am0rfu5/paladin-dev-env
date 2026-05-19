// src/infrastructure/adapters/llm/mod.rs
//
// LLM adapters module
//
// NOTE: All LLM provider adapter implementations have been relocated to the
// `paladin-llm` workspace crate (`crates/paladin-llm`).  The root `paladin`
// crate depends on `paladin-llm` and re-exports the adapter types through
// `src/lib.rs`.  Only the configuration bridge remains here to keep the
// `ApplicationSettings` ↔ adapter-config conversions inside the root crate
// where `ApplicationSettings` is defined.

pub mod config_bridge;
