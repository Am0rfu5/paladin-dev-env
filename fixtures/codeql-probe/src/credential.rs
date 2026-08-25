//! Deliberately vulnerable CodeQL evaluation fixture — planted defect class 1
//! (hardcoded credential).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate
//! excluded from the Paladin workspace build graph. It exists ONLY to
//! measure whether CodeQL's Rust SAST finds a hardcoded credential, matching
//! the first class the Snyk evaluation was measured against
//! (`.github/instructions/security.instructions.md`). It must NEVER be
//! referenced from real code.
//!
//! The value below is obviously synthetic, non-resolving and low-entropy by
//! design (D-10): it reads unmistakably as a planted test string rather than
//! a plausible real secret. Task 2 of this plan proves whether it survives
//! the repository's own secret-scanning gate as-is.

/// Planted hardcoded credential — synthetic, non-resolving, low-entropy.
/// NOT a real secret. See module header comment.
const PLANTED_TEST_CREDENTIAL: &str = "planted-fixture-credential-not-real-0000000000";

/// Builds an `Authorization` header value from the hardcoded credential
/// above — the sink shape CodeQL's `rust/hard-coded-cryptographic-value` and
/// related credential-detection queries look for.
pub fn build_authorization_header() -> String {
    format!("Bearer {PLANTED_TEST_CREDENTIAL}")
}
