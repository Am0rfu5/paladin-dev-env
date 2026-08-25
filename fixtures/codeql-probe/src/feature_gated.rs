//! Deliberately vulnerable CodeQL evaluation fixture — planted class 5 (feature-gated,
//! D-12's empirical coverage probe).
//!
//! **REDESIGNED 2026-08-25 (18-03 continuation).** The original shape reused the
//! untestable `sh -c` command-injection pattern (no upstream CWE-078 Rust query
//! exists), which made D-12's *finding-status* signal vacuous — only the file-reach
//! signal (extraction, not detection) was ever informative for that class. This
//! redesign replants the SQL-injection class (`sql_injection.rs`) — now genuinely
//! detectable — behind the same non-default cargo feature, so a finding here answers
//! D-12 from BOTH signals: file-reach (as before) AND detection (new). Any fixture
//! finding also proves the workspace-excluded, standalone `fixtures/codeql-probe` crate
//! is semantically analysed, not merely archived into the extraction database.
//!
//! This module is compiled only when the `probe-feature-gated` feature is enabled
//! (`fixtures/codeql-probe/Cargo.toml`'s `[features]` table; see `src/lib.rs`'s
//! `#[cfg(feature = "probe-feature-gated")]` guard). It deliberately reuses the exact
//! SQL-injection shape from `sql_injection.rs` so the two probes differ ONLY in whether
//! a feature gates them — isolating "does extraction (and analysis) reach feature-gated
//! code" as the single variable under test:
//!
//! - `sql_injection.rs`'s class is reported and this class is also reported:
//!   feature-gated code IS reached and analysed.
//! - `sql_injection.rs`'s class is reported and this class is NOT: feature-gated code
//!   is reached but not semantically analysed (or some other feature-specific gap).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It must NEVER be referenced from real code.

use sqlx::SqlitePool;

/// Identical sink shape to `sql_injection::find_user_from_remote_lookup`, gated behind
/// the `probe-feature-gated` cargo feature — the fifth planted class, and the only one
/// whose presence in a scan result answers D-12 empirically from both the file-reach
/// and the detection signal.
pub async fn find_user_from_remote_lookup_feature_gated(
    pool: &SqlitePool,
    lookup_url: &str,
) -> Result<Vec<(i64, String)>, Box<dyn std::error::Error>> {
    let untrusted_username = reqwest::blocking::get(lookup_url)?.text()?;
    let unsanitised_query =
        format!("SELECT id, name FROM users WHERE name = '{untrusted_username}'");
    let rows: Vec<(i64, String)> = sqlx::query_as(&unsanitised_query).fetch_all(pool).await?;
    Ok(rows)
}
