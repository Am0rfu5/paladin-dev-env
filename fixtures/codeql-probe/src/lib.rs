//! CodeQL Rust SAST evaluation fixture (Phase 18, D-07..D-12).
//!
//! This crate is DELIBERATELY VULNERABLE. It exists solely to measure whether CodeQL's
//! Rust analysis finds a set of vulnerability classes.
//!
//! **REDESIGNED 2026-08-25 (18-03 continuation).** The original four-class fixture,
//! reused verbatim from the Snyk evaluation methodology
//! (`.github/instructions/security.instructions.md`) so its 0-in-Rust /
//! 3-in-JavaScript comparability was already established and recorded by the first probe
//! run (that history is preserved in
//! `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`, not
//! deleted), turned out to be instrument-invalid: every one of its planted classes was
//! structurally incapable of firing any wired CodeQL Rust `security-extended` query,
//! independent of whether CodeQL's actual Rust detection capability is good or bad. See
//! that evidence document's `## Verdict` section (per-class impossibility table, with
//! upstream `github/codeql` citations) for the full analysis.
//!
//! This redesign is **rule-aligned**: every scoreable class now starts its taint flow
//! from a source CodeQL actually recognizes under its default `remote` active threat
//! model (`reqwest::blocking::get(...).text()`), matching the shape CodeQL's own
//! upstream test suite uses for these exact queries, rather than treating a bare
//! function parameter as a source (the defect that invalidated the first probe). The
//! methodology is pre-registered in the same evidence document's
//! `## Re-Probe Criteria (pre-registered)` section, committed before this file changed.
//!
//! It is excluded from the workspace build graph (see this crate's `Cargo.toml` header
//! comment) and MUST NEVER be referenced from real code, published, or depended on by
//! any workspace crate.
//!
//! **DIAGNOSTIC VARIANT 2026-08-25 (18-03 continuation, second checkpoint).** The
//! rule-aligned redesign above fired on only 1 of 4 scoreable classes
//! (`## Re-Probe Result`). `sql_injection.rs`, `path_traversal.rs`, `regex_injection.rs`
//! and `feature_gated.rs` were further rewritten to replace every `?`-operator taint-path
//! unwrap with `.unwrap()`/`.unwrap_or_default()`, matching CodeQL's own upstream test
//! idiom, isolating the `?`-operator hypothesis pre-registered in
//! `## Diagnostic Iteration (pre-registered)`. `sql_injection.rs` and `feature_gated.rs`
//! additionally replace `format!` with string concatenation, isolating that class from
//! the separately-diagnosed `format!`-macro-expansion defect. `credential.rs` and
//! `command_injection.rs` are unchanged from the redesigned probe.
#![allow(dead_code)]

/// Class 1: SQL injection. `reqwest` response body (unwrapped, not `?`) -> string
/// concatenation -> `sqlx::query_as` (the already-modeled sink,
/// `sqlx_core::query_as::query_as, Argument[0]`). Scoreable.
pub mod sql_injection;

/// Class 2: Path traversal. `reqwest` response body (unwrapped, not `?`) ->
/// `PathBuf::join` -> `std::fs::read_to_string`. Scoreable.
pub mod path_traversal;

/// Class 3: Hardcoded credential. A string literal `const` passed as the argument to a
/// local function whose parameter is named `password` — the rule's heuristic sink
/// (`rust/hard-coded-cryptographic-value`). Scoreable. Unchanged from the redesigned
/// probe (no `?`/`format!` on its taint path to begin with).
pub mod credential;

/// Class 4: Regex injection (`rust/regex-injection`). `reqwest` response body
/// (unwrapped, not `?`) -> `regex::Regex::new(...)`. Replaces one of the original
/// fixture's two untestable command-injection slots with a rule that actually exists
/// upstream. Scoreable.
pub mod regex_injection;

/// Known-gap class, kept but NOT scored: shell command injection via `sh -c`. No CWE-078
/// (OS command injection) query exists in CodeQL's Rust query pack as of this
/// evaluation — see the module's own header comment and the evidence document's
/// "Known-gap register row". This module documents a real, plausible Rust
/// vulnerability shape that this evaluation cannot test, because CodeQL cannot test it,
/// not because the fixture is malformed.
pub mod command_injection;

// D-12's empirical probe, replanted: the SQL-injection class (now genuinely
// detectable, unlike the original's untestable command-injection shape) gated behind a
// non-default cargo feature, so whether buildless CodeQL extraction *and* semantic
// analysis reach feature-gated code is a measured result rather than an assumption. See
// feature_gated.rs's own header comment.
#[cfg(feature = "probe-feature-gated")]
pub mod feature_gated;
