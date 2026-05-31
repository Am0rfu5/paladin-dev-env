## Epic 2: Dependency Security and License Compliance

**Project:** Paladin Framework
**Milestone:** 10 — CI Hardening and Release Automation
**Epic:** 2 of 4
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** None

### Objective

Add automated scanning for known vulnerabilities in dependencies and verify license compatibility across the dependency tree.

### Tasks

#### Task 2.1: Integrate `cargo audit`

**Description:** `cargo audit` checks the RustSec Advisory Database for known vulnerabilities in `Cargo.lock` dependencies. Add it to the CI pipeline as a required check.

**Deliverables:**
- `cargo audit` added to CI pipeline.
- Runs on every PR and on `main` branch pushes.
- Fails the build on known vulnerabilities (with a documented exception process for false positives or unpatched advisories).

#### Task 2.2: Integrate OSV-Scanner

**Description:** OSV-Scanner (Google's Open Source Vulnerability scanner) provides broader coverage than RustSec alone. Add it as a supplementary scanner.

**Deliverables:**
- OSV-Scanner added to CI pipeline.
- Configured to scan `Cargo.lock`.
- Results reported as PR annotations.

#### Task 2.3: Evaluate and Optionally Integrate Snyk

**Description:** Snyk provides commercial-grade vulnerability scanning with deeper analysis. Evaluate whether the free tier provides value beyond `cargo audit` + OSV-Scanner. If so, integrate; if not, document the decision and skip.

**Deliverables:**
- Evaluation document.
- Integration if warranted, or documented deferral.

#### Task 2.4: Configure `cargo deny`

**Description:** `cargo deny` enforces license compliance, bans specific crates, and detects duplicate dependency versions. Configure a `deny.toml` with:
- **Licenses:** Allow MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib. Deny copyleft (GPL, AGPL, LGPL) unless explicitly approved.
- **Bans:** No banned crates initially; add as needed.
- **Duplicates:** Warn on duplicate crate versions (different major versions of the same crate in the tree).

**Deliverables:**
- `deny.toml` in repository root.
- `cargo deny check` added to CI pipeline.
- All current dependencies pass.

#### Task 2.5: SBOM Generation

**Description:** Generate a Software Bill of Materials for each release. Use `cargo cyclonedx` or `cargo sbom` to produce CycloneDX or SPDX format.

**Deliverables:**
- SBOM generation added to the release pipeline.
- SBOM artifact attached to GitHub releases.

---
