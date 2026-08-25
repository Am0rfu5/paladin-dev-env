---
phase: 09
slug: release-security-gate-integrity
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (the blocking gate)
threats_open: 0
asvs_level: 1
created: 2026-08-15
---

# Phase 09 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

Register origin: `register_authored_at_plan_time: true` — all seven plans
(09-01…09-07) carried a parseable `<threat_model>` block. This audit verifies the
authored mitigations exist; it does not scan for new threats.

Verification mode: ASVS L1 (grep-depth), short-circuit rule satisfied
(`threats_open: 0` AND `register_authored_at_plan_time: true` AND `asvs_level == 1`).

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| contributor commit → `crates/` tree | A new crate can be added by anyone with commit rights; nothing asserted it carried the release artifacts a published crate family requires | crate manifests, changelogs |
| repo `scripts/` → CI verdict | A guard's exit code is what CI turns into a pass or fail; the script's own error paths sit inside the trust boundary of that verdict | exit codes |
| contributor commit → suppression surfaces | Anyone with commit rights can add an ID to `deny.toml` or `.cargo/audit.toml`; nothing required a corresponding accountability record | advisory identifiers |
| register → downstream consumers of the published crate family | The register is a public repo-root document; whatever it claims about reachability is what a consumer will believe | advisory IDs, reachability claims |
| a closed milestone's team label → present-day accountability | An owner field naming an organisational unit that no longer exists reads as governed while being unowned | ownership metadata |
| repository crate set → Docker build inputs | The planner stage decides which manifests reach `cargo chef prepare`; a hand-maintained list drifts silently as crates are added | crate manifests |
| tree crate names → crates.io namespace | An unowned package name reaching a publish attempt is an unrecoverable namespace collision | package names |
| licence declaration sites → downstream consumers | Manifests, licence files, README badge and OCI label each answer "what licence is this?" independently | licence expressions |

---

## Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-09-01 | Repudiation | `scripts/check-changelogs.sh` | high | mitigate | Exactly one `exit 0` branch, no fall-through; four demonstrated non-zero exits. Verified: `grep -c '^\s*exit 0'` = 1; guard exits 0 on clean tree | closed |
| T-09-02 | Spoofing | changelog guard crate-discovery glob | high | mitigate | Zero-crates-discovered is a distinct non-zero exit, proven against an empty `crates/` tree | closed |
| T-09-03 | Tampering | `crates/doc-examples/Cargo.toml` `publish` field | medium | mitigate | Exemption read from the manifest `publish` field, not a hard-coded directory name. Verified: 15 `publish` references in the guard | closed |
| T-09-04 | Elevation of privilege | `.github/workflows/ci.yml` job placement | medium | mitigate | Step lives in the `cargo-deny` job whose display name `License & Dependency Policy` is a required context in `.github/rulesets/protect-main-branch.json`. Verified: required-contexts list + green step on CI run 31898200704 | closed |
| T-09-05 | Repudiation | `owner` field, all register rows | high | mitigate | Every row owned by the named human approver `DF3NDR`. Verified: `owners = {'DF3NDR'}` across all 11 rows | closed |
| T-09-06 | Tampering | governance fields as free text | high | mitigate | Every `compensating_control` exceeds 40 chars and is distinct. Verified: min length 145, 11 distinct across 11 rows | closed |
| T-09-07 | Spoofing | register rows for dead suppressions | medium | mitigate | Dead rows not authored; plan 09-06's liveness clause makes the state unreachable. Verified: guard clause 3 green — 11 rows all present in `Cargo.lock` | closed |
| T-09-08 | Information disclosure | publishing advisory IDs + reachability at repo root | low | accept | IDs are already public in `deny.toml`, `.cargo/audit.toml` and the RustSec database; adds nothing derivable-only-from-here. See Accepted Risks (R-09-01) | closed |
| T-09-09 | Elevation of privilege | stricter-than-PRD schema without recorded supersession | medium | mitigate | ADR-0024 records the FR-3 supersession under D-00b precedence | closed |
| T-09-10 | Tampering | planner-stage manifest enumeration | high | mitigate | Enumeration deleted; coverage now structural. Verified: `grep -c 'COPY crates/paladin' Dockerfile.chef` = 0, `cargo chef prepare` = 1 | closed |
| T-09-11 | Repudiation | ADR-0027's evidence claim | high | mitigate | ADR states measured-vs-established explicitly and carries an "Outstanding, CI-only measurement (not performed here)" block. Verified: ADR-0027 lines 58–66, 146–149 | closed |
| T-09-12 | Denial of service | Docker image build | medium | mitigate | Subtraction-only change; zero added `COPY`/`RUN`. Verified: "Docker Build" job green on CI run 31898200704 including compose test and size budget | closed |
| T-09-13 | Tampering | OCI licence `LABEL` in `Dockerfile.chef` | medium | mitigate | Label left byte-unchanged by 09-03; owned by 09-05 in wave 2. Verified: label reads `MIT OR Apache-2.0` at `Dockerfile.chef:87` | closed |
| T-09-14 | Spoofing | crates.io package name | high | mitigate | Forward set-equality fails any tree name absent from the committed allow-list, on every PR. Verified: guard exits 0; step green in the required job | closed |
| T-09-15 | Repudiation | a one-directional guard | high | mitigate | Both directions asserted; each demonstrated failing separately | closed |
| T-09-16 | Tampering | `.crate-names.txt` provenance | medium | mitigate | Allow-list committed and hand-edited; guard reads the file rather than deriving it. Verified: 4 references to `.crate-names.txt`, no auto-generation | closed |
| T-09-17 | Spoofing | near-miss name matching | medium | mitigate | Exact byte comparison; case-only variant proven to fail. Verified: zero case-folding operations in the guard | closed |
| T-09-18 | Elevation of privilege | guard placed in a non-required CI job | medium | mitigate | Step in the required `License & Dependency Policy` context. Verified on CI run 31898200704 | closed |
| T-09-19 | Repudiation | the licence decision itself | high | mitigate | Blocking `checkpoint:decision` plus a halting `<precondition>`; ADR-0025 names approver and date. Verified: 6 approver/`DF3NDR` references in ADR-0025 | closed |
| T-09-20 | Elevation of privilege | published permission set | high | mitigate | Narrowing forbidden outright; only the additive branch executable. Verified: single distinct expression `MIT OR Apache-2.0` across all 11 manifests | closed |
| T-09-21 | Tampering | `LICENSE-APACHE` text | high | mitigate | Verbatim canonical text required, reconstruction forbidden. Verified: 201 lines, consistent with the full Apache-2.0 text | closed |
| T-09-22 | Spoofing | divergent licence statements across sites | medium | mitigate | All four sites (manifests, licence files, README, OCI label) updated together. Verified: each site reads `MIT OR Apache-2.0` | closed |
| T-09-23 | Information disclosure | `git mv` versus delete-and-create | low | mitigate | `git mv` required, `git log --follow` asserted. Verified: `LICENSE-MIT` and `LICENSE-APACHE` present, legacy `LICENSE` absent | closed |
| T-09-24 | Spoofing | two CI jobs sharing one display name | high | mitigate | Duplicate deleted. Verified: exactly one `Security Audit` job in `ci.yml` and in CI run 31898200704's job list | closed |
| T-09-25 | Denial of service | the surviving audit gate | high | mitigate | Exactly one audit invocation reading the authoritative file. Verified: 1 invocation in `ci.yml`; "Run cargo-audit (exceptions from .cargo/audit.toml)" green | closed |
| T-09-26 | Elevation of privilege | self-granted suppressions | high | mitigate | Clause 2 fails any identifier without a fully-populated register row, inside a required-context job. Verified: step green on CI run 31898200704 | closed |
| T-09-27 | Tampering | a suppression outliving its dependency | high | mitigate | Clause 3 asserts every register row's crate is present in `Cargo.lock`. Verified live: "11 register row(s) checked … all clauses satisfied", exit 0 | closed |
| T-09-28 | Repudiation | the guard's own error paths | high | mitigate | Missing-input and marker-extraction failures are explicit non-zero exits; nine failure modes demonstrated. Verified: `MISSING_INPUT` path plus `exit 1` branches | closed |
| T-09-29 | Tampering | class information scraped from comments | medium | mitigate | Class read only from the register's `class` field via `tomllib` (`row.get("class")`), partitions discovered structurally, no hard-coded class literal. Verified by reading the guard | closed |
| T-09-30 | Repudiation | closure claimed on tools that cannot run here | high | mitigate | Both tool-level passes recorded as `backstop`/CI-only, never inferred. Subsequently discharged: both tools green on CI run 31898200704 | closed |
| T-09-31 | Repudiation | closure claimed on unexecuted evidence | high | mitigate | Every closure note carries a verbatim command or `file:line`; CI-only claims listed explicitly. Verified: 5 SEC rows read `Phase 9 | Complete` | closed |
| T-09-32 | Tampering | silent overwriting of stale figures | medium | mitigate | Corrections made as annotations with original text retained; zero deleted lines asserted | closed |
| T-09-33 | Denial of service | Phase 12 re-planning executed work | medium | mitigate | SUPPLY-01/SUPPLY-02 rows and the ROADMAP Phase 12 section both amended with closure notes | closed |
| T-09-34 | Spoofing | a coverage floor reported without measurement | medium | mitigate | Floor recorded as confirmed-unmoved by the no-source-change argument; unmeasured percentages forbidden in the SUMMARY | closed |
| T-09-35 | Elevation of privilege | the ADR numbering line | low | mitigate | Counter advanced exactly once, in the last wave, with a dated note. Verified: all four 0024–0027 index rows present in `PROMOTION.md` | closed |
| T-09-SC | Tampering | package-manager installs (all 7 plans) | high | accept | No plan in this phase installed anything or changed a dependency; guards use `bash` + stdlib `tomllib` only. Package Legitimacy Gate did not fire. See Accepted Risks (R-09-02) | closed |

*Status: open · closed · open — below high threshold (non-blocking)*
*Severity: critical > high > medium > low — only open threats at or above workflow.security_block_on (high) count toward threats_open*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| R-09-01 | T-09-08 | Publishing exact unpatched advisory IDs and their reachability at repo root discloses nothing an attacker could not derive from `deny.toml`, `.cargo/audit.toml`, the RustSec advisory database, or the published crates themselves. D-01's argument is that the governance must be visible where the config is. Severity low — below the `high` block threshold. | DF3NDR (via ADR-0024) | 2026-08-08 |
| R-09-02 | T-09-SC (×7 plans) | Supply-chain risk from package-manager installs is accepted as not-applicable for this phase: no plan installed a package or modified `Cargo.toml`/`Cargo.lock`/`crates/*/Cargo.toml`. Both guards use `bash` plus Python stdlib `tomllib`, already present and already used by `scripts/check-doc-config.sh`. The Package Legitimacy Gate did not fire for any of the seven plans. | DF3NDR (per 09-RESEARCH.md § Package Legitimacy Audit) | 2026-08-08 |

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-08-15 | 36 | 36 | 0 | /gsd-secure-phase (orchestrator, ASVS L1 short-circuit) |

**Evidence base.** Mitigations were verified against the working tree (guard scripts
executed live, manifests and ADRs read) and against GitHub Actions run
**31898200704** on branch `chore/15.1-close-out`, workflow `ci.yml` — the first real
CI run to exercise every gate this phase authored. That run discharges the four
CI-only claims phase 09 recorded honestly as unverified-here per D-19:

- `Check per-crate changelogs` — success (required job)
- `Check crates.io package names` — success (required job)
- `Check advisory exception register` — success (required job)
- `Run cargo-deny check` — success; `Run cargo-audit` — success

**Note on register drift.** The register held 10 rows at phase 09 close (commit
`a587e5a`) and holds 11 today. The 11th row was added by phase 15.1 (commit
`d955998`, giving RUSTSEC-2026-0249 a governed disposition under D-11) — a later
phase correctly *using* the exception process this phase built. The guard's three
clauses pass against all 11 rows, so this is evidence the control works, not drift
against it.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-08-15
