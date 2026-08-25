//! Deliberately vulnerable CodeQL evaluation fixture — planted defect class 3
//! (path traversal).
//!
//! This file is part of `fixtures/codeql-probe/`, a standalone crate
//! excluded from the Paladin workspace build graph. It exists ONLY to
//! measure whether CodeQL's Rust SAST finds a path traversal, matching the
//! third class the Snyk evaluation was measured against
//! (`.github/instructions/security.instructions.md`). It must NEVER be
//! referenced from real code.

use std::path::PathBuf;

/// Joins caller-supplied input onto a fixed base directory with no
/// normalisation or containment check, then reads the resulting path — the
/// classic path-traversal sink shape (`../../etc/passwd`-style escape).
pub fn read_from_base_dir(caller_input: &str) -> std::io::Result<String> {
    let base_dir = PathBuf::from("/var/lib/paladin/codeql-probe-fixture-data");
    let unsanitised_path = base_dir.join(caller_input);
    std::fs::read_to_string(unsanitised_path)
}
