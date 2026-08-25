//! Deliberately vulnerable CodeQL evaluation fixture — planted defect class 2
//! (shell command injection).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate
//! excluded from the Paladin workspace build graph. It exists ONLY to
//! measure whether CodeQL's Rust SAST finds a shell command injection,
//! matching the second class the Snyk evaluation was measured against
//! (`.github/instructions/security.instructions.md`). It must NEVER be
//! referenced from real code.

use std::process::{Command, Output};

/// Takes caller-supplied input and interpolates it, unsanitised, into a
/// shell command string executed via `sh -c` — the classic command
/// injection sink shape.
pub fn run_with_caller_input(caller_input: &str) -> std::io::Result<Output> {
    let shell_command = format!("echo {caller_input}");
    Command::new("sh").arg("-c").arg(shell_command).output()
}
