//! Deliberately vulnerable CodeQL evaluation fixture — planted class 5 (feature-gated,
//! D-12's empirical coverage probe).
//!
//! **DIAGNOSTIC VARIANT 2026-08-25 (18-03 continuation, second checkpoint).** Mirrors
//! `sql_injection.rs`'s diagnostic shape exactly (string concatenation instead of `format!`,
//! `.unwrap()`/`.unwrap_or_default()` instead of `?`), unchanged in its feature-gating. A
//! finding here, alongside `sql_injection.rs` firing, answers D-12 from both the file-reach
//! and the detection signal on the unwrap/concatenation idiom.
//!
//! This module is compiled only when the `probe-feature-gated` feature is enabled
//! (`fixtures/codeql-probe/Cargo.toml`'s `[features]` table; see `src/lib.rs`'s
//! `#[cfg(feature = "probe-feature-gated")]` guard). It deliberately reuses the exact
//! SQL-injection shape from `sql_injection.rs` so the two probes differ ONLY in whether a
//! feature gates them — isolating "does extraction (and analysis) reach feature-gated code" as
//! the single variable under test:
//!
//! - `sql_injection.rs`'s class is reported and this class is also reported: feature-gated
//!   code IS reached and analysed.
//! - `sql_injection.rs`'s class is reported and this class is NOT: feature-gated code is
//!   reached but not semantically analysed — consistent with the run-log evidence already
//!   recorded in `## Re-Probe Result` (`semantic analyzer unavailable (not included as a
//!   module)`).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It must NEVER be referenced from real code —
//! `.unwrap()`/`.unwrap_or_default()` on network I/O is deliberately diagnostic here, not a
//! pattern this repository's own `CLAUDE.md` conventions endorse.

use sqlx::SqlitePool;

/// Identical sink shape to `sql_injection::find_user_from_remote_lookup`, gated behind the
/// `probe-feature-gated` cargo feature — the fifth planted class, and the only one whose
/// presence in a scan result answers D-12 empirically from both the file-reach and the
/// detection signal.
pub async fn find_user_from_remote_lookup_feature_gated(
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
