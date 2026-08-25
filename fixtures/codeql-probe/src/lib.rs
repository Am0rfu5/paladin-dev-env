//! CodeQL Rust SAST evaluation fixture (Phase 18, D-07..D-12).
//!
//! This crate is DELIBERATELY VULNERABLE. It exists solely to measure
//! whether CodeQL's Rust analysis finds a fixed, ordered set of vulnerability
//! classes reused verbatim from the Snyk evaluation (see
//! `.github/instructions/security.instructions.md`), so the resulting finding
//! count is directly comparable to the recorded 0-in-Rust / 3-in-JavaScript
//! baseline.
//!
//! It is excluded from the workspace build graph (see this crate's
//! `Cargo.toml` header comment) and MUST NEVER be referenced from real code,
//! published, or depended on by any workspace crate.
//!
//! Modules are declared in the exact order the Snyk-era probe was measured
//! in — reused verbatim, not improved, so the methodology stays comparable
//! (D-08). Do not reorder.
#![allow(dead_code)]

pub mod credential;
pub mod command_injection;
pub mod path_traversal;
pub mod sql_injection;

// D-12's empirical answer to "does buildless Rust extraction reach code
// gated behind a non-default cargo feature?" — off by default, so it is
// observable rather than assumed. See feature_gated.rs's own header comment.
#[cfg(feature = "probe-feature-gated")]
pub mod feature_gated;
