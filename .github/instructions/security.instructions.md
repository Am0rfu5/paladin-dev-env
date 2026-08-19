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

No tool above performs taint analysis of first-party Rust (see the gap below), so
credential-handling code is reviewed by hand. For any code touching an API key or
an external response body, confirm:

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

## Known gap: no Rust SAST

There is no static taint analysis for first-party Rust. `cargo-audit` and
`cargo-deny` scan dependencies; `clippy` is a lint. Evaluating a Rust-capable SAST
(CodeQL's Rust support, or Semgrep) is open work. Until then the manual review
above is the control — state that plainly rather than implying automated coverage.
