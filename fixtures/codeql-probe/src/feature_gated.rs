//! Deliberately vulnerable CodeQL evaluation fixture — planted defect class 5
//! (feature-gated, D-12's empirical coverage probe).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate
//! excluded from the Paladin workspace build graph. It exists ONLY to
//! measure whether CodeQL's buildless Rust extraction reaches code gated
//! behind a non-default cargo feature — an open question with no official
//! documentation either way (RESEARCH.md, "Open Questions" item 1, Pitfall
//! 1). It must NEVER be referenced from real code.
//!
//! This module is compiled only when the `probe-feature-gated` feature is
//! enabled (`fixtures/codeql-probe/Cargo.toml`'s `[features]` table; see
//! `src/lib.rs`'s `#[cfg(feature = "probe-feature-gated")]` guard). It
//! deliberately reuses the exact command-injection shape from
//! `command_injection.rs` so the two probes differ ONLY in whether a
//! feature gates them — isolating "does extraction reach feature-gated
//! code" as the single variable under test:
//!
//! - `command_injection.rs`'s class is reported and this class is also
//!   reported: feature-gated code IS reached.
//! - `command_injection.rs`'s class is reported and this class is NOT:
//!   feature-gated code is NOT reached.

use std::process::{Command, Output};

/// Identical sink shape to `command_injection::run_with_caller_input`,
/// gated behind the `probe-feature-gated` cargo feature — the fifth planted
/// defect class, and the only one whose presence in a scan result answers
/// D-12 empirically.
pub fn run_with_caller_input_feature_gated(caller_input: &str) -> std::io::Result<Output> {
    let shell_command = format!("echo {caller_input}");
    Command::new("sh").arg("-c").arg(shell_command).output()
}
