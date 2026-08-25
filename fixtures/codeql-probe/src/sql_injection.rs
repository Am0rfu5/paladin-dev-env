//! Deliberately vulnerable CodeQL evaluation fixture — planted class 1 (SQL injection).
//!
//! **DIAGNOSTIC VARIANT 2026-08-25 (18-03 continuation, second checkpoint).** The
//! remote-source-wired redesign (`## Re-Probe Result`,
//! `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`) did not
//! fire, with two compounding diagnosed causes: a `format!`-macro-expansion failure at the
//! exact taint-carrying line, and an unconfirmed `?`-operator taint-loss hypothesis (see
//! `## Diagnostic Iteration (pre-registered)` in that document). This variant isolates the
//! `?`-operator hypothesis from the `format!` defect by changing BOTH in one step, matching
//! CodeQL's own upstream test idiom exactly: `.unwrap()`/`.unwrap_or_default()` instead of
//! `?`, and string concatenation (the `unsafe_query_3` shape from
//! `rust/ql/test/query-tests/security/CWE-089/sqlx.rs`) instead of `format!`.
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST finds
//! a SQL injection reachable from a remote HTTP response, using the tool's own passing test
//! idiom rather than early-return error propagation. It must NEVER be referenced from real
//! code — `.unwrap()`/`.unwrap_or_default()` on network I/O is deliberately diagnostic, not a
//! pattern this repository's own `CLAUDE.md` conventions endorse (see that file's "Avoid
//! `unwrap()`/`expect()` ... in library code" guidance, which this fixture intentionally
//! violates for measurement purposes only).

use sqlx::SqlitePool;

/// Fetches a caller-supplied URL, treats the response body as a raw username, and
/// concatenates it, unsanitised, into a SQL query string, then passes that string to
/// `sqlx::query_as` against a `SqlitePool` — the classic SQL injection sink shape. The taint
/// source is the `reqwest::blocking::get(...).text()` response body, unwrapped via
/// `.unwrap()`/`.unwrap_or_default()` rather than `?`, matching CodeQL's own upstream test
/// idiom for this query.
pub async fn find_user_from_remote_lookup(
    pool: &SqlitePool,
    lookup_url: &str,
) -> Vec<(i64, String)> {
    let untrusted_username = reqwest::blocking::get(lookup_url)
        .unwrap()
        .text()
        .unwrap_or_default();
    let unsanitised_query =
        String::from("SELECT id, name FROM users WHERE name = '") + &untrusted_username + "'";
    sqlx::query_as(&unsanitised_query)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}
