# RustSec Remediation Plan (Epic 4)

Date: 2026-05-28
Scope: Findings from `cargo audit` during Task 5.6

## Objective

Address or formally risk-accept the two blocking RustSec vulnerabilities before release candidate GO decision.

## Blocking Findings

1. `RUSTSEC-2023-0071` (`rsa 0.9.10`)
- Summary: Marvin timing side-channel risk.
- Current path: `rsa -> sqlx-mysql -> sqlx -> workspace crates`.
- Current status: No fixed upgrade available.

2. `RUSTSEC-2025-0111` (`tokio-tar 0.3.1`)
- Summary: PAX header parsing issue enabling file smuggling.
- Current path: `tokio-tar -> testcontainers -> testcontainers-modules`.
- Current status: No fixed upgrade available.

## Remediation Strategy

### Track A: Immediate Risk Reduction

1. Constrain feature/build surface for release artifacts.
- For production release profile, avoid enabling components that pull vulnerable paths unless explicitly required.
- Confirm whether `sqlx-mysql` is required in default release path; prefer SQLite-only release profile where acceptable.

2. Harden usage boundaries.
- `RUSTSEC-2023-0071`: ensure no direct RSA private-key decrypt/sign operations are exposed in runtime paths; document dependency-only exposure if true.
- `RUSTSEC-2025-0111`: ensure untrusted tar extraction is not performed in production runtime paths; limit `testcontainers` usage to test-only contexts.

3. Add temporary denylist/allowlist policy notes.
- Record explicit risk acceptance rationale with scope limitations and compensating controls.

### Track B: Dependency Replacement / Upgrade Feasibility

1. Investigate `sqlx-mysql` alternatives.
- Evaluate whether MySQL support can be feature-gated off by default for release candidate.
- Evaluate migration path to dependency versions that remove `rsa` when upstream fixes land.

2. Investigate test dependency isolation.
- Move `testcontainers` usage to dev/test-only crates or test targets if not already isolated.
- Ensure release build and published crates do not require `tokio-tar` transitive chain.

3. Monitor upstream advisories and releases.
- Subscribe to upstream issue/advisory trackers for `rsa`, `sqlx`, `tokio-tar`, `testcontainers`.

## Action Plan

1. Create issue: "Epic 4 Security: RUSTSEC-2023-0071 impact analysis and mitigation".
2. Create issue: "Epic 4 Security: RUSTSEC-2025-0111 testcontainers/tokio-tar mitigation".
3. Add CI step to fail on new RustSec vulnerabilities while allowing explicit, documented exceptions for these two IDs.
4. Add `audit.toml` exception entries only if approved, each with expiry date and owner.
5. Re-run `cargo audit` after mitigation changes and attach evidence to Epic 4 report.

## Acceptance Criteria

- Either vulnerabilities are eliminated from release dependency graph, or
- Formal risk acceptance is documented with:
  - owner
  - expiry date
  - affected scope
  - compensating controls
  - tracked follow-up issue.

## Exit Evidence

- Updated audit output.
- Updated Epic 4 release readiness report.
- Task list status updated for Task 5.6.
