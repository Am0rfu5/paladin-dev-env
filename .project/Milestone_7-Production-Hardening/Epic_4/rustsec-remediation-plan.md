# RustSec Remediation Plan (Epic 4)

> **SUPERSEDED AS THE EXCEPTION REGISTER — 2026-08-08, per [ADR-0024](../../../.planning/decisions/0024-rustsec-exception-governance.md).**
> This document was one of the **four divergent answers** ROADMAP Phase 9 criterion 1 named to the
> question "which RustSec advisories does this project suppress, and why?" — it formally risk-accepted
> exactly two advisories while `.cargo/audit.toml` suppressed five, `deny.toml` suppressed fourteen,
> and `.github/workflows/ci.yml` ran two independently-configured `cargo audit` jobs. The
> authoritative register is now **[`SECURITY-EXCEPTIONS.md`](../../../SECURITY-EXCEPTIONS.md)** at
> the repository root, which carries all ten live suppressions with owner, review date, affected
> scope and compensating control, and is mechanically enforced by
> `scripts/check-advisory-register.sh`.
>
> **Three specific claims below are now false and are corrected here rather than rewritten in place:**
>
> 1. **"Exception owner: Platform Security (Milestone 7)"** (below) — that is a team label attached to
>    a milestone that has ended, so the acceptance had no reachable owner. The owner is now the
>    repository owner, **`DF3NDR`**, on every one of the ten register rows.
> 2. **"Exception review/expiry target: 2026-09-30"** (below) — **this was the only dated item in the
>    entire 199-document planning corpus.** It has been **renewed**, not allowed to lapse: every
>    suppression now carries a per-advisory review date of **2026-12-31**. Neither original advisory
>    (`RUSTSEC-2023-0071` via `rsa`/`sqlx-mysql`, `RUSTSEC-2025-0111` via `tokio-tar`/`testcontainers`)
>    has an upstream fix, so closing was not available; renewal with a named owner was.
> 3. **"Local audit target (`make audit`) runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore
>    RUSTSEC-2025-0111`"** and **"CI security job enforces the same command"** (below) — both describe
>    a configuration that no longer exists. `make audit` runs a bare `cargo audit`, sourcing its
>    exceptions from `.cargo/audit.toml`; the second, inline-`--ignore` CI job was **deleted** by Phase 9
>    plan 09-06, leaving exactly one `cargo audit` invocation in `ci.yml`. Two jobs both displaying the
>    name "Security Audit" could previously reach different verdicts on the same `Cargo.lock`.
>
> The original text below is retained unmodified as the historical risk-acceptance record; nothing
> below this banner was rewritten.

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

## Progress Update (2026-05-28)

Completed initial hardening actions:
- Moved `testcontainers-modules` from normal dependencies to `dev-dependencies` in the root manifest.
- Tightened MySQL repository compilation in `src/infrastructure/repositories/mod.rs` so MySQL module paths are only present when `storage-mysql` is enabled.
- Disabled `sqlx` default features at workspace level and explicitly listed required features (`sqlite`, `migrate`, etc.) to reduce implicit backend activation.

Validation snapshot:
- `cargo check` passes after the changes.
- `cargo tree -i tokio-tar` now shows only dev-dependency paths through `testcontainers`.
- `cargo audit` still reports both RustSec advisories because Cargo.lock includes dev/optional dependency graphs and no fixed upstream versions are available.

Exception governance now enforced:
- Local audit target (`make audit`) runs `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111`.
- CI security job enforces the same command to block newly introduced vulnerabilities while allowing only approved exceptions.
- Exception owner: Platform Security (Milestone 7).
- Exception review/expiry target: 2026-09-30 (or earlier if upstream fixes become available).

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
- CI workflow now enforces RustSec checks with explicit exception IDs.
