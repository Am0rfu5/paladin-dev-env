//! # paladin-core
//!
//! Pure domain types for the Paladin framework.
//!
//! This crate contains all domain entities and base primitives with zero
//! dependencies on infrastructure, LLM providers, databases, or HTTP clients.
//! It is the foundational crate that all other Paladin workspace crates depend on.
//!
//! ## Module Structure
//!
//! - [`base`] — Foundation primitives: `Node<T>`, `Collection`, `Field`, `Message`, `Action`, `Event`
//! - [`platform`] — Domain entities: `Paladin`, Battalion types, `Garrison`, `Arsenal`, `Citadel`, `Herald`, `Sanctum`

#![warn(missing_docs)]

// pub mod base;
/// Foundation primitives and framework base types.
#[allow(missing_docs)]
pub mod base;
/// Core platform domain entities and containers.
#[allow(missing_docs)]
pub mod platform;
