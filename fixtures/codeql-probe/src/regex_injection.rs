//! Deliberately vulnerable CodeQL evaluation fixture — planted class 4 (regex
//! injection).
//!
//! **NEW 2026-08-25 (18-03 continuation).** Replaces one of the original fixture's two
//! untestable `sh -c` command-injection slots (CWE-078 has no upstream CodeQL Rust query
//! — see this crate's `command_injection.rs`, kept as a documented known gap, not
//! scored) with `rust/regex-injection`, a rule that actually exists in CodeQL's Rust
//! `security-extended` query pack.
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST
//! finds an attacker-controlled regular expression compiled from a remote HTTP
//! response. It must NEVER be referenced from real code.

/// Fetches a caller-supplied URL and compiles the response body directly as a regular
/// expression pattern, unsanitised — the classic regex-injection sink shape (a
/// caller-controlled pattern can express catastrophic backtracking, a denial-of-service
/// vector `rust/regex-injection` is designed to catch). The taint source is the
/// `reqwest::blocking::get(...).text()` response body.
pub fn compile_pattern_from_remote_lookup(
    lookup_url: &str,
) -> Result<regex::Regex, Box<dyn std::error::Error>> {
    let untrusted_pattern = reqwest::blocking::get(lookup_url)?.text()?;
    Ok(regex::Regex::new(&untrusted_pattern)?)
}
