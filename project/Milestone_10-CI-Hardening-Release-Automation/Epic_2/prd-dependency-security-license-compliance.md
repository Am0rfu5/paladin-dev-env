# PRD: Milestone 10 — Epic 2: Dependency Security and License Compliance

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 2 of 4
**Version Target:** v0.4.0
**Source Epic:** `Milestone_10-Epic_2-dependency-security-license-compliance.md`
**Status:** Ready for Implementation

---

## 1. Introduction/Overview

Paladin already runs a `cargo audit` step in CI (`.github/workflows/ci.yml` → `security-audit`
job) with two documented RustSec exceptions tracked in `audit.toml`. However, the project has **no
license-compliance enforcement, no supplementary vulnerability scanner, no Software Bill of
Materials (SBOM) for releases, and no formalized exception process**. The current `cargo audit`
invocation also hard-codes the ignored advisory IDs inline (`--ignore RUSTSEC-...`) instead of
reading them from the version-controlled `audit.toml`, so the CI command and the config file can
drift apart.

This Epic hardens the supply chain by:

1. Formalizing `cargo audit` so it reads its ignore-list from `audit.toml` (single source of truth)
   and documenting the exception process.
2. Adding **OSV-Scanner** as a supplementary advisory source with broader coverage than RustSec
   alone.
3. Evaluating **Snyk** and recording an explicit integrate-or-defer decision.
4. Adding **`cargo deny`** with a version-controlled `deny.toml` to enforce an allowed-license
   policy, ban undesirable crates, and flag duplicate dependency versions.
5. Generating a **CycloneDX SBOM** for every release and attaching it as a release artifact.

The goal: every dependency change is automatically checked for known vulnerabilities and license
compatibility before merge, and every release ships a verifiable bill of materials.

## 2. Goals

1. `cargo audit` runs in CI on every pull request and on pushes to the primary branches, reads its
   ignore-list from `audit.toml`, and fails the build on any non-excepted advisory.
2. A documented, auditable exception process exists for advisories that cannot yet be remediated
   (transitive/dev-only dependencies without an upstream fix).
3. OSV-Scanner runs in CI against `Cargo.lock` and surfaces findings as PR annotations without
   duplicating or contradicting the `cargo audit` gate.
4. A reasoned Snyk evaluation is recorded, with either an integration or a documented deferral.
5. A version-controlled `deny.toml` enforces the approved license allow-list, and
   `cargo deny check` passes against the current dependency tree in CI.
6. Each GitHub release carries an attached CycloneDX SBOM artifact generated from the locked
   dependency graph.
7. All new CI checks pass against the current `main`/`develop` state (no pre-existing violations are
   left unaddressed or are explicitly, visibly excepted).

## 3. User Stories

- **As a maintainer**, I want license compliance enforced automatically, so that a transitively
  pulled-in GPL/AGPL crate is caught at PR time rather than discovered during a release audit.
- **As a security reviewer**, I want two independent vulnerability scanners (RustSec via `cargo
  audit` and the broader OSV database via OSV-Scanner), so that coverage gaps in one source are
  caught by the other.
- **As a release engineer**, I want an SBOM attached to every release, so that downstream consumers
  and compliance teams can inventory exactly which dependency versions shipped.
- **As a contributor**, I want a single documented place (`audit.toml` / `deny.toml`) and process for
  recording an approved exception, so that suppressing a false-positive or un-patchable advisory is
  transparent and reviewable instead of an undocumented inline flag.
- **As a downstream integrator**, I want assurance that Paladin's dependency licenses are all
  permissive (MIT/Apache-style), so that I can embed it without copyleft obligations.

## 4. Functional Requirements

### `cargo audit` Integration & Exception Process (FR 1–4)

1. The CI `security-audit` job must invoke `cargo audit` such that its ignore-list is sourced from
   the version-controlled `audit.toml` (single source of truth) rather than inline `--ignore`
   flags, so the workflow and the config cannot drift.
2. `cargo audit` must run on every pull request and on every push to the primary branches
   (`main`, `develop`), and must fail the build on any advisory not listed in `audit.toml`.
3. The exception process must be documented: each ignored advisory in `audit.toml` must carry a
   comment stating (a) the advisory ID, (b) the affected crate and why it is present (e.g.
   transitive/dev-only), (c) why it is not yet fixable, and (d) a revisit condition. (The two
   existing exceptions already follow this shape and must be preserved.)
4. The `cargo audit` step must be reproducible locally (`cargo audit` from the repo root must honor
   the same `audit.toml`), and the equivalent command must be documented.

### OSV-Scanner Integration (FR 5–7)

5. OSV-Scanner must be added to CI, configured to scan `Cargo.lock`.
6. OSV-Scanner findings must be reported as PR annotations (e.g. via the official OSV-Scanner
   GitHub Action's SARIF upload / reviewdog output) without failing the build on advisories already
   excepted via the `cargo audit` process, to avoid contradictory gates. The OSV step's
   failure/annotation policy must be explicitly chosen and documented.
7. The OSV-Scanner step must run on pull requests (and may additionally run on a schedule for the
   primary branch) so new advisories in already-merged dependencies are surfaced over time.

### Snyk Evaluation (FR 8–9)

8. A short evaluation must be produced comparing Snyk's free tier against the combined
   `cargo audit` + OSV-Scanner + `cargo deny` coverage, considering: added value, required account
   secrets, and maintenance cost.
9. Based on the evaluation, either integrate Snyk into CI **or** record a documented deferral with
   the rationale and the conditions under which it would be reconsidered. (No silent skip.)

### `cargo deny` / License Compliance (FR 10–14)

10. A version-controlled `deny.toml` must exist in the repository root.
11. The `[licenses]` policy must **allow**: `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`,
    `ISC`, and `Zlib`. Copyleft licenses (`GPL-*`, `AGPL-*`, `LGPL-*`) must **not** be allowed
    unless explicitly added with a recorded justification.
12. The `[bans]` section must start with no banned crates (extensible later) and must surface
    **duplicate** crate versions (the same crate present at multiple incompatible versions) at least
    as a warning.
13. `cargo deny check` must be added to CI as a required gate running on every pull request and on
    pushes to the primary branches, and must pass against the current dependency tree.
14. If the current tree contains a crate with a license not on the allow-list (or whose license is
    unknown/ambiguous), the situation must be resolved by either: (a) adding the specific license to
    the allow-list with justification, or (b) adding a narrowly-scoped per-crate `clarify`/exception
    in `deny.toml` with a comment. Blanket disabling of the license check is not acceptable.

### SBOM Generation (FR 15–17)

15. The release pipeline (`.github/workflows/release.yml`) must generate a Software Bill of Materials
    in **CycloneDX** format from the locked dependency graph (e.g. via `cargo cyclonedx`).
16. The generated SBOM must be attached as an asset to the corresponding GitHub release.
17. SBOM generation must be reproducible locally via a documented command (and/or a Makefile target)
    so it can be produced on demand outside of CI.

### Documentation & Conformance (FR 18–20)

18. `CONTRIBUTING.md` (and/or `docs/`) must document the dependency-security tooling: how to run
    `cargo audit` and `cargo deny` locally, how to add an approved exception to `audit.toml` /
    `deny.toml`, and where SBOMs are published.
19. A `make` target (e.g. `make security` or `make deny`) should wrap the local security checks
    (`cargo audit` + `cargo deny check`) for one-command local verification.
20. All newly added CI gates must pass against the current dependency tree at the time of
    implementation; any unavoidable exception must be explicit and commented in the relevant config
    file.

## 5. Non-Goals (Out of Scope)

- **Release automation, changelog generation, version bumping, and the publish/tagging flow** —
  covered by Epic 3 (`Milestone_10-Epic_3-release-automation.md`). This Epic only *adds* SBOM
  generation to the existing release workflow; it does not redesign the release flow.
- **The pre-commit / pre-push hook framework** — covered by Epic 1 (already implemented).
- **Remediating the two existing, documented RustSec exceptions** (`RUSTSEC-2023-0071`,
  `RUSTSEC-2025-0111`) — these remain tracked until an upstream fix exists; this Epic preserves and
  documents them, it does not force an upgrade of `sqlx`/`testcontainers`.
- **Runtime/SBOM signing, provenance attestation (SLSA), and container image scanning** — valuable
  future hardening, but out of scope for this Epic.
- **Adding new application functionality** — this Epic is supply-chain/CI/config only.

## 6. Design Considerations

- Keep the new CI checks as **separate, clearly named jobs** (mirroring the existing
  `security-audit` job style) so a failing gate is immediately identifiable in the PR checks list.
- Prefer the **official, pinned GitHub Actions** for OSV-Scanner and (if adopted) Snyk, matching the
  pinned-revision discipline established in Epic 1.
- `deny.toml` and `audit.toml` are the version-controlled single sources of truth; CI must read from
  them rather than re-specifying policy inline.
- Favor `cargo install --locked <tool>` for CI tool installation to keep tool versions reproducible,
  consistent with the existing `cargo install cargo-audit --locked` usage.

## 7. Technical Considerations

- **Existing state:** `.github/workflows/ci.yml` already contains a `security-audit` job using
  `cargo audit` with inline `--ignore` flags; `audit.toml` already exists with the two exceptions.
  FR 1 changes the invocation to rely on `audit.toml`.
- **`cargo-deny`** is a Rust binary installable via `cargo install --locked cargo-deny`; `deny.toml`
  uses the current `cargo-deny` schema (advisories/licenses/bans/sources sections). The licenses
  section should use SPDX identifiers and the modern `allow = [...]` form.
- **OSV-Scanner** is distributed as a Go binary and an official GitHub Action
  (`google/osv-scanner-action`) that supports SARIF output for PR annotations via
  `github/codeql-action/upload-sarif`.
- **SBOM:** `cargo cyclonedx` (CycloneDX community tool) produces CycloneDX XML/JSON from the cargo
  metadata graph; it integrates into the release job after checkout + toolchain install.
- The workspace is a multi-crate Cargo workspace (`members = [".", "crates/*"]`); `cargo deny` and
  the SBOM tool operate on the whole workspace graph from the root.
- Some checks (OSV-Scanner action, Snyk action, SBOM upload-to-release) are **CI-only** and cannot
  be fully exercised inside the dev container; they will be validated by config correctness
  (YAML/schema) plus the locally-runnable equivalents (`cargo audit`, `cargo deny check`,
  `cargo cyclonedx`).

## 8. Success Metrics

- CI shows distinct passing checks for: `cargo audit` (config-driven), `cargo deny check`, and
  OSV-Scanner, on pull requests.
- `cargo deny check` reports zero license violations (or only explicitly-justified exceptions) on
  the current tree.
- A test/dry-run release produces a CycloneDX SBOM artifact attached to the release.
- `audit.toml` and `deny.toml` are the only places policy/exceptions are defined; no inline
  advisory-ignore flags remain in CI.
- Local `make security` (or equivalent) reproduces the audit + deny checks with one command.

## 9. Open Questions

1. **OSV-Scanner failure policy:** Should OSV-Scanner *fail* the PR build, or only *annotate*?
   Recommendation: annotate-only initially (non-blocking) to avoid duplicate/contradictory gates
   with `cargo audit`, then tighten to blocking once the annotation noise level is understood.
2. **Snyk:** Default recommendation is to **defer** (document the deferral) because `cargo audit` +
   OSV-Scanner + `cargo deny` already cover advisories + licenses without requiring an external
   account secret; revisit if a need for reachability analysis or fix-PR automation arises.
3. **SBOM format:** CycloneDX is the chosen format per the Epic; SPDX is not required. Confirm
   whether both formats are ever needed downstream (assume CycloneDX-only for now).
4. **Duplicate-version policy:** Start `multiple-versions` as `warn` (not `deny`) to avoid blocking
   on transitive duplicates outside our control; revisit promoting to `deny` once the tree is
   de-duplicated.
