//! Deliberately vulnerable CodeQL evaluation fixture — planted class 1 (SQL injection).
//!
//! **REDESIGNED 2026-08-25 (18-03 continuation).** The original shape took an
//! already-tainted `&str` function parameter as its source. CodeQL's own test for this
//! query (`rust/ql/test/query-tests/security/CWE-089/sqlx.rs`) fires only from a source
//! recognized under the default `remote` active threat model, such as
//! `reqwest::blocking::get(...).text()` — a bare function parameter is never itself a
//! source. This redesign starts the taint flow from exactly that shape.
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST
//! finds a SQL injection reachable from a remote HTTP response. It must NEVER be
//! referenced from real code.

use sqlx::SqlitePool;

/// Fetches a caller-supplied URL, treats the response body as a raw username, and
/// interpolates it, unsanitised, into a SQL query string via `format!`, then passes that
/// string to `sqlx::query_as` against a `SqlitePool` — the classic SQL injection sink
/// shape (a parameterised `sqlx::query!`/bind call is the fix this deliberately avoids).
/// The taint source is the `reqwest::blocking::get(...).text()` response body, matching
/// CodeQL's own upstream test shape for this query.
pub async fn find_user_from_remote_lookup(
    pool: &SqlitePool,
    lookup_url: &str,
) -> Result<Vec<(i64, String)>, Box<dyn std::error::Error>> {
    let untrusted_username = reqwest::blocking::get(lookup_url)?.text()?;
    let unsanitised_query =
        format!("SELECT id, name FROM users WHERE name = '{untrusted_username}'");
    let rows: Vec<(i64, String)> = sqlx::query_as(&unsanitised_query).fetch_all(pool).await?;
    Ok(rows)
}
