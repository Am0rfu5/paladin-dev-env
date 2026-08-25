//! Deliberately vulnerable CodeQL evaluation fixture — planted class 3 (hardcoded
//! credential).
//!
//! **REDESIGNED 2026-08-25 (18-03 continuation).** The original shape built an
//! `Authorization` header string via `format!("Bearer {CREDENTIAL}")`. That is not a
//! sink `rust/hard-coded-cryptographic-value` recognizes: its sinks are modeled
//! crypto-API arguments (`credentials-{password,key,iv,nonce,salt}`) plus a heuristic
//! for call arguments whose **parameter name** is literally `password`/`iv`/`nonce`/
//! `salt`. This redesign passes the hardcoded value directly as the argument to a local
//! function whose parameter is named `password` — the rule's documented heuristic sink
//! shape.
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate excluded from the
//! Paladin workspace build graph. It exists ONLY to measure whether CodeQL's Rust SAST
//! finds a hardcoded credential passed to a password-named parameter. It must NEVER be
//! referenced from real code.
//!
//! The value below is obviously synthetic, non-resolving and low-entropy by design
//! (D-10): it reads unmistakably as a planted test string rather than a plausible real
//! secret. The 18-02 plan proved this property survives the repository's own
//! secret-scanning gate as-is; that property is unchanged by this redesign.

/// Planted hardcoded credential — synthetic, non-resolving, low-entropy. NOT a real
/// secret. See module header comment.
const PLANTED_TEST_CREDENTIAL: &str = "planted-fixture-credential-not-real-0000000000";

/// The heuristic sink: any call argument bound to a parameter literally named
/// `password` is a sink for `rust/hard-coded-cryptographic-value`, independent of what
/// the function actually does with it.
fn authenticate_with_password(username: &str, password: &str) -> bool {
    // Deliberately a stub — this fixture measures whether the hardcoded *argument* is
    // flagged, not whether authentication logic is correct.
    !username.is_empty() && !password.is_empty()
}

/// Calls the password-named sink above with the hardcoded credential above — the sink
/// shape CodeQL's `rust/hard-coded-cryptographic-value` heuristic looks for.
pub fn authenticate_with_planted_credential(username: &str) -> bool {
    authenticate_with_password(username, PLANTED_TEST_CREDENTIAL)
}
