//! Deliberately vulnerable CodeQL evaluation fixture — planted class 4 (regex injection).
//!
//! **DIAGNOSTIC VARIANT 2026-08-25 (18-03 continuation, second checkpoint).** Only the
//! `?`-operator unwrapping changes here — the sink shape (`regex::Regex::new`) is unchanged,
//! since it involves no `format!` call and the orchestrator independently verified the
//! source/propagation/sink models are all present at the deployed CodeQL CLI version. If this
//! variant still misses, `?` is ruled out as the cause for this class and the miss implicates
//! external-crate call resolution or a deeper, undiagnosed extraction failure (see
//! `## Diagnostic Iteration (pre-registered)` in
//! `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST finds
//! an attacker-controlled regular expression compiled from a remote HTTP response, using the
//! tool's own passing test idiom (`.unwrap()`/`.unwrap_or_default()`) rather than early-return
//! error propagation. It must NEVER be referenced from real code — this unwrap-heavy shape is
//! deliberately diagnostic, not a pattern this repository's own `CLAUDE.md` conventions
//! endorse.

/// Fetches a caller-supplied URL and compiles the response body directly as a regular
/// expression pattern, unsanitised — the classic regex-injection sink shape (a
/// caller-controlled pattern can express catastrophic backtracking, a denial-of-service vector
/// `rust/regex-injection` is designed to catch). The taint source is the
/// `reqwest::blocking::get(...).text()` response body, unwrapped via
/// `.unwrap()`/`.unwrap_or_default()` rather than `?`. Returns `None` if the untrusted pattern
/// fails to compile — the fixture measures whether the *attempted* compilation itself is
/// flagged, not the success path.
pub fn compile_pattern_from_remote_lookup(lookup_url: &str) -> Option<regex::Regex> {
    let untrusted_pattern = reqwest::blocking::get(lookup_url)
        .unwrap()
        .text()
        .unwrap_or_default();
    regex::Regex::new(&untrusted_pattern).ok()
}
