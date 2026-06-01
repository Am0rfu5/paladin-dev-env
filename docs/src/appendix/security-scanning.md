# Dependency Security & License Compliance

This document describes Paladin's supply-chain security tooling: vulnerability
scanning, license compliance, the exception process, and Software Bill of
Materials (SBOM) generation. It is part of **Milestone 10 — CI Hardening and
Release Automation, Epic 2**.

## Tooling Overview

| Concern | Tool | Where it runs | Config / source of truth |
|---------|------|---------------|--------------------------|
| Known vulnerabilities (RustSec) | `cargo audit` | CI (`security-audit` job) + local | `.cargo/audit.toml` |
| Known vulnerabilities (OSV DB) | OSV-Scanner | CI (`osv-scanner` job, PR annotations) | `Cargo.lock` |
| License compliance + bans + duplicates | `cargo deny` | CI (`cargo-deny` job) + local | `deny.toml` |
| Software Bill of Materials | `cargo cyclonedx` | Release pipeline | `Cargo.lock` |

## Running the Checks Locally

```bash
# Vulnerability advisories (reads exceptions from .cargo/audit.toml)
cargo audit

# License policy, bans, duplicate versions, advisories (reads deny.toml)
cargo deny check

# Both at once
make security

# Generate a CycloneDX SBOM for the workspace
make sbom
```

Install the tools once with:

```bash
cargo install --locked cargo-audit cargo-deny cargo-cyclonedx
```

## License Policy

`deny.toml` enforces a **permissive-only** allow-list:

- Allowed (core): `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`.
- Allowed (additional permissive, each justified in `deny.toml`): `Unicode-3.0`,
  `0BSD`, `CC0-1.0`, `CDLA-Permissive-2.0`.
- Strong copyleft licenses (`GPL-*`, `AGPL-*`, `LGPL-*`) are **not** allowed.
- Weak/file-level copyleft (`MPL-2.0`) is **not** in the global allow-list; it is
  granted only via narrowly-scoped per-crate `[[licenses.exceptions]]` entries so
  the global policy stays permissive-only.

If a required dependency uses a license outside this set, do **not** disable the
license check. Instead, either:

1. Add the specific SPDX license id to `deny.toml`'s `[licenses].allow` list with
   a comment justifying it (for genuinely permissive licenses), or
2. Add a narrowly-scoped `[[licenses.exceptions]]` entry granting a specific
   license to a specific crate (preferred for weak copyleft like `MPL-2.0`), or
3. Add a `[[licenses.clarify]]` entry for a specific crate when its license
   metadata is ambiguous.

## Advisory Exception Process

Some advisories cannot be remediated immediately (typically transitive or
dev/test-only dependencies with no upstream fix). Exceptions are recorded in
**two synchronized files**:

- `.cargo/audit.toml` — auto-discovered by `cargo audit`.
- `deny.toml` (`[advisories].ignore`) — used by `cargo deny`.

Each exception **must** include a comment stating:

1. The advisory ID (e.g. `RUSTSEC-2023-0071`).
2. The affected crate and why it is in the tree (e.g. transitive dev dependency
   of `sqlx-mysql`).
3. Why it is not yet fixable (no upstream patch available).
4. A revisit condition (e.g. "revisit when sqlx upgrades rsa").

When adding or removing an exception, update **both** files so the two scanners
do not contradict each other.

Current tracked exceptions:

- `RUSTSEC-2023-0071` — RSA timing side-channel via `rsa 0.9.x` (transitive
  dev/test dep of `sqlx-mysql`; no upstream fix).
- `RUSTSEC-2025-0111` — `tokio-tar` path traversal (transitive dev/test dep of
  `testcontainers`; no upstream fix).

## OSV-Scanner Policy

OSV-Scanner runs on pull requests and reports findings as **PR annotations**
(via SARIF upload). It is currently **annotate-only (non-blocking)** to avoid
contradicting the `cargo audit` gate while the annotation signal level is
assessed. It may be promoted to a blocking gate later (see PRD Open Question 1).

## Snyk Evaluation & Decision

**Decision: Deferred.**

Snyk's free tier was evaluated against the combined coverage of `cargo audit`
(RustSec), OSV-Scanner (OSV database), and `cargo deny` (licenses + bans +
duplicates):

| Capability | cargo audit + OSV + cargo deny | Snyk free tier |
|------------|--------------------------------|----------------|
| RustSec advisories | Yes (`cargo audit`) | Yes |
| Broad OSV coverage | Yes (OSV-Scanner) | Partial |
| License compliance | Yes (`cargo deny`) | Limited on free tier |
| Dependency bans / duplicates | Yes (`cargo deny`) | No |
| Reachability analysis | No | Yes (added value) |
| Automated fix PRs | No | Yes (added value) |
| Requires external account/secret | No | Yes (`SNYK_TOKEN`) |
| Maintenance cost | Low (all in-repo config) | Medium (account + secret rotation) |

**Rationale:** The existing three tools already cover advisories and license
compliance with no external account, no secret management, and fully
version-controlled policy (`.cargo/audit.toml`, `deny.toml`). Snyk's incremental value
(reachability analysis, automated fix PRs) does not currently justify the added
account/secret-management overhead.

**Revisit when:** the project needs reachability-based prioritization of
advisories, wants automated dependency-bump PRs beyond Dependabot, or an
enterprise compliance requirement mandates Snyk specifically.

## SBOM

Every GitHub release attaches a CycloneDX SBOM
(`paladin-<version>.cdx.json`) generated from the locked dependency graph by the
`sbom` job in `.github/workflows/release.yml`. Generate the SBOMs locally with
`make sbom`, which runs `cargo cyclonedx --all --format json` and writes one
`<crate>.cdx.json` next to each workspace crate's manifest (the root package's
`paladin-ai.cdx.json` is the primary deliverable). These generated files are
git-ignored.
