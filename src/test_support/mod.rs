//! Shared, `Send + Sync` test infrastructure for co-located `#[cfg(test)]` modules in `src/`.
//!
//! This module exists because a `#[cfg(test)] mod tests` block co-located inside `src/` cannot
//! import from the separate `tests/` crate — Rust integration tests under the top-level
//! `tests/` directory compile as an independent crate that *depends on* this library, not the
//! other way around. The mock helpers living under that separate crate's `helpers` module
//! therefore cannot serve tests that live inside `src/` and need private-path access, which
//! matters for reaching the module coverage bar. `src/test_support/` fills that gap instead: it
//! is declared `#[cfg(test)]` on the module declaration itself in `src/lib.rs` (not merely on
//! individual items), so none of it reaches a release build, while still being importable by any
//! co-located test module in this crate.
//!
//! `src/test_support/` and the top-level `tests` crate's `helpers` module are deliberately
//! disjoint and coexist: neither re-exports from the other, and no double is defined in both
//! places. The `tests`-crate helpers keep serving the integration suites under that directory;
//! this module serves `src/`-side unit tests only.
//!
//! # Provided doubles
//!
//! - [`failing_channel_handler`] — a `NotificationChannelHandler` implementation whose
//!   `handle_notification` always errors, for exercising notification-failure paths without
//!   changing any production signature.
//!
//! No new dependency is added to any manifest to build these doubles; `mockall` is not adopted.

pub mod failing_channel_handler;

pub use failing_channel_handler::{FailingChannelHandler, FailingChannelInvocation};
