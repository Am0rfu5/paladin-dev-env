//! Deliberately vulnerable CodeQL evaluation fixture — KNOWN-GAP class, kept but NOT
//! SCORED (shell command injection).
//!
//! **Status corrected 2026-08-25 (18-03 continuation).** This class is reused verbatim
//! from the Snyk-era methodology (`.github/instructions/security.instructions.md`), but
//! **no CWE-078 (OS command injection) query exists in CodeQL's Rust `security-extended`
//! query pack as of this evaluation** — confirmed against `rust/ql/src/queries/security`
//! upstream, which has no `CWE-078` directory. This class is untestable by construction:
//! no fixture shape, however faithfully wired to a `remote` taint source, could ever
//! trigger a rule that does not exist. It is kept in the crate as documentation of a
//! real, plausible Rust vulnerability shape CodeQL cannot currently detect, and is
//! **excluded from the pre-registered Re-Probe Criteria scoring** in
//! `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`
//! ("## Re-Probe Criteria (pre-registered)" -> "Known-gap register row"). Its continued
//! 0-finding result on any future scan is expected and uninformative, not evidence of
//! anything.
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It must NEVER be referenced from real code.

use std::process::{Command, Output};

/// Takes caller-supplied input and interpolates it, unsanitised, into a
/// shell command string executed via `sh -c` — the classic command
/// injection sink shape. Untestable by CodeQL as of this evaluation (see module header).
pub fn run_with_caller_input(caller_input: &str) -> std::io::Result<Output> {
    let shell_command = format!("echo {caller_input}");
    Command::new("sh").arg("-c").arg(shell_command).output()
}
