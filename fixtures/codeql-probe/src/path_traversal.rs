//! Deliberately vulnerable CodeQL evaluation fixture — planted class 2 (path traversal).
//!
//! **REDESIGNED 2026-08-25 (18-03 continuation).** The original shape took an
//! already-tainted `&str` function parameter as its source. `rust/path-injection`'s
//! `Source` is `ActiveThreatModelSource` only — a bare function parameter is never
//! itself a source under the default `remote` active threat model. This redesign starts
//! the taint flow from a `reqwest::blocking::get(...).text()` response body, a source
//! CodeQL actually recognizes.
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST
//! finds a path traversal reachable from a remote HTTP response. It must NEVER be
//! referenced from real code.

use std::path::PathBuf;

/// Fetches a caller-supplied URL and treats the response body as a relative path
/// segment, joins it onto a fixed base directory with no normalisation or containment
/// check, then reads the resulting path — the classic path-traversal sink shape
/// (`../../etc/passwd`-style escape). The taint source is the
/// `reqwest::blocking::get(...).text()` response body.
pub fn read_from_remote_path_lookup(
    lookup_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let untrusted_relative_path = reqwest::blocking::get(lookup_url)?.text()?;
    let base_dir = PathBuf::from("/var/lib/paladin/codeql-probe-fixture-data");
    let unsanitised_path = base_dir.join(untrusted_relative_path);
    Ok(std::fs::read_to_string(unsanitised_path)?)
}
