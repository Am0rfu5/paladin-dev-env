---
alwaysApply: true
always_on: true
trigger: always_on
applyTo: "**"
description: Security practices for first-party Rust
---

# Project security best practices

Paladin is a Rust workspace. Run these against new or modified first-party code:

| Concern | Command | Tool |
|---|---|---|
| Vulnerable dependencies | `make audit` | `cargo-audit` (RustSec advisory DB) |
| Licenses, bans, sources, advisories | `make deny` | `cargo-deny` |
| Both of the above | `make security` | — |
| Lints, including correctness classes | `cargo clippy -- -D warnings` | `clippy` |
| Dependency inventory | `make sbom` | `cargo-cyclonedx` |

`make security` must pass before a phase is sealed. Treat a new advisory as a
blocker unless it is explicitly allowlisted in `.cargo/audit.toml` with a reason.

## Manual review still matters

No tool above performs merge-gating taint analysis of first-party Rust. CodeQL was
evaluated for this role and retained **advisory-only** — see "Known gap: no Rust SAST"
below — so credential-handling code is still reviewed by hand. For any code touching an
API key or an external response body, confirm:

- Response bodies are redacted **before** truncation when embedded in errors or
  logs — see `crates/paladin-llm/src/redaction.rs`. Bounding first can slice a
  secret across the truncation boundary and leak the tail.
- No log statement interpolates an API key, and no config type carrying one is
  `Debug`-formatted or serialised outward.
- HTTP clients sending a credential header do not follow redirects, so the header
  cannot be forwarded to an attacker-influenced host.

## Snyk was evaluated and removed (2026-08-18)

Do **not** reintroduce a Snyk scan step, and do not record a phase as blocked on
one. An untracked `snyk_rules.instructions.md` previously mandated `snyk_code_scan`
for "new first party code in a Snyk-supported language". Rust is not such a
language, so the mandate was unsatisfiable: it blocked verification in Phase 15.1
and in plans 17-12 through 17-16, where five SUMMARYs recorded the scan as not-run.

Measured, not assumed:

- **Snyk Code (SAST)** ingests `.rs` files but has no meaningful Rust rules. A probe
  carrying a hardcoded credential, command injection via `sh -c`, path traversal and
  SQL injection returned **0 findings**. The same four in JavaScript returned 3
  (HIGH/MEDIUM/LOW), confirming the scanner and credentials worked.
- **Snyk Open Source (SCA)** has no Cargo support; `snyk test` exits
  `SNYK-CLI-0008 — no supported target files` on this workspace.

A "clean" Snyk result here means *nothing was analysed*, not *the code is clean* —
worse than no scan, because it reads as assurance.

## Known gap: no Rust SAST (measured 2026-08-25)

CodeQL was evaluated as a Rust-capable SAST candidate and **disqualified** at the tested
version — CodeQL CLI `2.26.3`, `rust-queries` `0.1.40`, `security-extended` query suite,
evaluated 2026-08-25. `.github/workflows/codeql.yml` is **retained, advisory-only**: it
runs on every push/PR/schedule and reports findings in the code-scanning UI, but it is
not pinned in any ruleset and does not gate a merge. Full run-by-run evidence is in
`.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md`.

Measured, not assumed:

- Across four independent measurements — the original fixture, a redesigned fixture
  sourced from a `reqwest` remote response, a `.unwrap()`-idiom diagnostic variant, and a
  confound test planting identical shapes inside the real, already-scanned `paladin`
  workspace crate — SQL injection, path traversal and regex injection built from a
  `reqwest` remote source **never fired**, under any tested condition. Only
  `rust/hard-coded-cryptographic-value` fired reliably.
- Coverage is not the gap: `analysed_rs_files` read `385` of the `385`-file denominator
  (100%) on every run without exception — the "analysed nothing while reporting success"
  failure shape that disqualified Snyk cannot recur here. The gap is a measured detection
  gap in 3 of the 4 rule-aligned, source-wired classes at this CodeQL/`rust-queries`
  version, not a coverage or extraction failure.
- The one class that does fire carries a real false-positive cost on this codebase's own
  code: alert #28 (`rust/hard-coded-cryptographic-value` on a test-fixture literal, not a
  leaked secret) is a false positive on triage — 1 triaged, 1 false positive (100% FP
  rate on the only real-code sample available), governed in `CODEQL-DISMISSALS.md`.
- This verdict is **version-scoped**. A future CodeQL/`rust-queries` release that adds
  source recognition for `reqwest::blocking` responses to these query families would
  warrant re-running the fixture at `fixtures/codeql-probe/` (kept in the tree for
  exactly this reason) rather than assuming the disqualification still holds.
- The Semgrep contingency's trigger condition ("no qualifying Rust SAST promoted") is now
  met; no Semgrep evaluation has been performed as part of this work.

There is still no static taint analysis of first-party Rust that gates a merge.
`cargo-audit` and `cargo-deny` scan dependencies; `clippy` is a lint. The manual review
above remains the primary control for credential-handling code — state that plainly
rather than letting CodeQL's retained advisory scan read as coverage it does not provide.
