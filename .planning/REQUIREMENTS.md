# Requirements: Paladin

**Milestone: v0.9.0 — Security Tooling** (started 2026-08-24)

*Fresh requirements file, opened at the v0.8.0 milestone close on 2026-08-24. Everything the
previous milestones defined — the 90 forward requirements across `RECON-*` … `PROV-*`, the 554
ingested `REQ-*` IDs, the five as-shipped ledgers, the 30 competing-variant groups and the full
traceability history — is archived and unchanged in
[`milestones/v0.8.0-REQUIREMENTS.md`](milestones/v0.8.0-REQUIREMENTS.md) and
[`milestones/v0.7.1-REQUIREMENTS.md`](milestones/v0.7.1-REQUIREMENTS.md). Consult those before
re-deriving anything; this file starts the new milestone rather than restating them.*

**`SAST-01` … `SAST-04` are carried forward, not re-minted.** They were written on 2026-08-24
(commit `48ac11a5`) alongside Phase 18 and appear in the v0.8.0 archive only because the archive is
a snapshot taken at close — they were never v0.8.0 work. Their text below is the text that shipped
with the roadmap entry, byte-identical. `.planning/ROADMAP.md`'s Phase 18 section references these
IDs directly, so they must resolve here.

---

## Requirements — Security Tooling (Phase 18)

*Added 2026-08-24. Forward work, not ingest-derived — opened by the v0.8.0 milestone audit, which
records the absence of first-party Rust static analysis as the milestone's one genuinely open item.
`SAST-*` is the nineteenth prefix; per Roadmap Extension Protocol item 3 no earlier prefix is
recycled. These IDs are minted at roadmap time rather than during execution, which is the lesson
from Phase 15.1 carrying `Requirements: TBD` into execution and settling it retroactively.*

### Rust static analysis (SAST)

- [ ] **SAST-01**: A candidate Rust SAST is **measured against a deliberate-vulnerability probe on
      this tree before any adoption decision**, and the finding count is recorded either way.
      The probe is the one that disqualified Snyk, reused verbatim so the results are comparable: a
      Rust fixture carrying a hardcoded credential, command injection via `sh -c`, path traversal
      and SQL injection. Snyk Code returned **0 findings** on that fixture while the identical four
      in JavaScript returned 3 (HIGH/MEDIUM/LOW), which is what proved the scanner and credentials
      worked and the Rust analysis did not — see `.github/instructions/security.instructions.md`.
      **A zero-finding result satisfies this requirement**: it disqualifies the tool, and the
      verdict plus its evidence is the deliverable. What does not satisfy it is adopting a scanner
      without running the probe.
      Primary candidate: **CodeQL**, whose Rust support left public preview and reached general
      availability in October 2025, is supported in both default and advanced setup, and carries
      real Rust queries rather than file ingestion alone. Secondary: **Semgrep**, which is pattern
      matching rather than interprocedural taint analysis and is therefore evaluated as a
      complement, not as the primary control.

- [ ] **SAST-02**: If a scanner qualifies under SAST-01, it **runs on every pull request and cannot
      be path-filtered into silence.**
      Its workflow triggers on `pull_request` with no path filter, plus `push` on `main` and a
      schedule. This is a hard constraint rather than a preference: `scripts/check-workflow-triggers.sh`
      Clause 4 exists because a required context living in a path-filtered workflow never reports
      on a PR touching no matching path, and that PR is then unmergeable forever with no failing
      check to point at. Cost note for planning: the repository is public, so GitHub code scanning
      and CodeQL carry no licence cost, and `github/codeql-action/upload-sarif@v3` is already wired
      into `ci.yml` for OSV results — code scanning is enabled today. Unlike Snyk, no token or
      vendor account is required.

- [ ] **SAST-03**: The scanner **runs non-blocking first, and is promoted on measured behaviour.**
      A recorded observation window reports its false-positive rate and wall-clock cost against
      this tree's real size (385 `.rs` files, ~141,717 lines). Only then may it become a required
      check — and promotion updates all four places the required set is written down in a single
      change: the context is added to `.github/rulesets/protect-main-branch.json` (44 → 45), the
      live ruleset `20868126` is re-applied, `docs/src/appendix/branch-protection.md`'s context
      table is brought to match, and `scripts/check-workflow-triggers.sh` passes. Pinning an
      unmeasured scanner as a 45th required check is how a gate ends up permanently red or
      routinely bypassed — the defect class Phase 12 deleted when it removed the duplicate audit
      job.

- [ ] **SAST-04**: `.github/instructions/security.instructions.md`'s **"Known gap: no Rust SAST"
      section is rewritten to match the measured outcome**, stating what the adopted tool does and
      does not cover and what the manual credential-handling review still owns.
      The section is narrowed or replaced by evidence, never deleted to imply coverage the probe
      did not establish. If SAST-01 disqualifies every candidate, this requirement is satisfied by
      updating the section to record which tools were measured, on what date, with what result —
      so the next person to ask does not repeat the evaluation blind.

---

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SAST-01 | Phase 18 | Pending |
| SAST-02 | Phase 18 | Pending |
| SAST-03 | Phase 18 | Pending |
| SAST-04 | Phase 18 | Pending |

**Coverage:**

- v0.9.0 requirements: **4 total** (4 Security Tooling, Phase 18)
- Mapped to phases: 4
- Unmapped: 0 ✓
- Duplicated across phases: 0 ✓
- Phases carrying no requirement ID: 0

**Prefix register.** Nineteen prefixes are spent and none may be recycled, per *Roadmap Extension
Protocol* item 3: `RECON-*`, `GAP-*`, `QUAL-*`, `REL-*` (Milestone 1); `VERIFY-*`, `CLOSE-*`
(Milestone 2-3); `ARCH-*`, `DEBT-*` (Milestone 4-6); `SEC-*`, `HARD-*`, `FACADE-*` (Milestone 7-8);
`SUPPLY-*`, `ORCH-*`, `WEB-*`, `PIPE-*`, `DEFER-*`, `DOCS-*` (Milestone 9-12 + Deferred-QA);
`PROV-*` (Provider Expansion); `SAST-*` (Security Tooling). Ingested `REQ-*` IDs remain the stable
merge keys.

**Phase 15.1 remains the one phase in this project's history with no requirement identifier**, by
its own recorded decision (plan `15.1-10`, D-00f, 2026-08-14) rather than by oversight. That record
lives in the v0.8.0 archive; it is noted here so the convention is not mistaken for an accident and
repeated by default. New phases mint their IDs at roadmap time.

---
*Requirements opened: 2026-08-24 at the v0.8.0 milestone close.*
