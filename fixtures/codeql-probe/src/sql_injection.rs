//! Deliberately vulnerable CodeQL evaluation fixture — planted defect class 4
//! (SQL injection).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate
//! excluded from the Paladin workspace build graph. It exists ONLY to
//! measure whether CodeQL's Rust SAST finds a SQL injection, matching the
//! fourth class the Snyk evaluation was measured against
//! (`.github/instructions/security.instructions.md`). It must NEVER be
//! referenced from real code.

use sqlx::SqlitePool;

/// Takes caller-supplied input and interpolates it, unsanitised, into a SQL
/// query string via `format!`, then passes that string to `sqlx::query`
/// against a `SqlitePool` — the classic SQL injection sink shape (a
/// parameterised `sqlx::query!`/bind call is the fix this deliberately
/// avoids).
pub async fn find_user_by_name(
    pool: &SqlitePool,
    caller_input: &str,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let unsanitised_query = format!("SELECT id, name FROM users WHERE name = '{caller_input}'");
    let rows: Vec<(i64, String)> = sqlx::query_as(&unsanitised_query)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
