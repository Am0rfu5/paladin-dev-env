//! Deliberately vulnerable CodeQL evaluation fixture — planted class 2 (path traversal).
//!
//! **DIAGNOSTIC VARIANT 2026-08-25 (18-03 continuation, second checkpoint).** Only the
//! `?`-operator unwrapping changes here — the sink shape (`PathBuf::join` ->
//! `std::fs::read_to_string`) is unchanged, since it involves no `format!` call and the
//! orchestrator independently verified the source/propagation/sink models are all present at
//! the deployed CodeQL CLI version. If this variant still misses, `?` is ruled out as the
//! cause for this class and the miss implicates external-crate call resolution or a deeper,
//! undiagnosed extraction failure (see `## Diagnostic Iteration (pre-registered)` in
//! `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST finds
//! a path traversal reachable from a remote HTTP response, using the tool's own passing test
//! idiom (`.unwrap()`/`.unwrap_or_default()`) rather than early-return error propagation. It
//! must NEVER be referenced from real code — this unwrap-heavy shape is deliberately
//! diagnostic, not a pattern this repository's own `CLAUDE.md` conventions endorse.

use std::path::PathBuf;

/// Fetches a caller-supplied URL and treats the response body as a relative path segment,
/// joins it onto a fixed base directory with no normalisation or containment check, then
/// reads the resulting path — the classic path-traversal sink shape
/// (`../../etc/passwd`-style escape). The taint source is the
/// `reqwest::blocking::get(...).text()` response body, unwrapped via
/// `.unwrap()`/`.unwrap_or_default()` rather than `?`.
pub fn read_from_remote_path_lookup(lookup_url: &str) -> String {
    let untrusted_relative_path = reqwest::blocking::get(lookup_url)
        .unwrap()
        .text()
        .unwrap_or_default();
    let base_dir = PathBuf::from("/var/lib/paladin/codeql-probe-fixture-data");
    let unsanitised_path = base_dir.join(untrusted_relative_path);
    std::fs::read_to_string(unsanitised_path).unwrap_or_default()
}
