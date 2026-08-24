---
phase: 11
slug: facade-residue-deferred-register-disposition
status: verified
# threats_open = count of OPEN threats at or above workflow.security_block_on severity (the blocking gate)
threats_open: 0
asvs_level: 1
created: 2026-08-09
---

# Phase 11 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

This phase writes **records, not executable code**. No runtime surface, endpoint, queue,
scheduler or dependency manifest is touched by any of its five plans. The threats that matter
are therefore **provenance threats** — a claim crossing from a lower-trust source into a
higher-trust governing record, where later phases inherit it as settled without re-measuring.

The register below is the union of the plan-time `<threat_model>` blocks in `11-01-PLAN.md`
through `11-05-PLAN.md` (`register_authored_at_plan_time: true`). No SUMMARY raised a
`## Threat Flags` entry.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| working tree → `.planning/` record | A measured fact (a grep count, a `file:line`) becomes a durable assertion later phases treat as settled | Repository-local paths, line numbers, commit SHAs |
| `.project/` historical corpus → `.planning/` governing record | A Milestone-8-era claim is promoted, corrected or superseded; the corpus is known-fallible | Superseded verdicts, effort ratings |
| Phase 10 `--auto` output → this phase's ADR | ADR-0031 crosses in as the governing test for D3/D4 without human ratification | Unratified decision text |
| `.project/` DOC (low precedence) → `.planning/decisions/` ADR (highest) | A placement condition is promoted out of a class the next PRD can silently override | Crate-placement conditions |
| git history → a durable recovery instruction | A commit reference must still resolve to the same 1,065-LOC module years later | Immutable commit SHA |
| this phase → `PROMOTION.md` shared numbering state | One line every future ADR-writing phase reads and trusts without re-deriving | Next-free ADR number |
| this phase → its own governing criterion | ROADMAP §Phase 11 criterion 1 is the standard this phase is audited against, and it amends it | Success-criterion text |

---

## Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-11-01 | Spoofing | 17 `file:line` citations in the D5 register | high | mitigate | Re-measured at execution time; `.planning/registers/facade-01-rustdoc-stdout-disposition.md` records the 17-occurrence count with its producing command | closed |
| T-11-02 | Tampering | `deferred-items.md`, `.planning/ROADMAP.md` | medium | mitigate | Annotation only. Original `low / low` rating (line 157) and `Quick wins: D5` line (line 165) retained verbatim inside `~~…~~` strikethrough | closed |
| T-11-03 | Repudiation | Ledger Evidence cell and register claims | medium | mitigate | D-00e evidence bar — both D5 commands quoted verbatim in the register and amended ledger cell | closed |
| T-11-04 | Elevation of Privilege | ROADMAP §Phase 11 criterion 1 | medium | mitigate | Amended criterion stays falsifiable (names two runnable commands and expected outputs); original text retained in the dated amendment note | closed |
| T-11-05 | Information Disclosure | — | low | accept | Not applicable — repository-local paths/SHAs only | closed |
| T-11-06 | Denial of Service | — | low | accept | Not applicable — no runtime surface touched | closed |
| T-11-07 | Elevation of Privilege | ADR-0034's citation of ADR-0031 | high | mitigate | ADR-0034 lines 65-69 state ADR-0031's `⚠ HUMAN REVIEW` status and require human confirmation before any D3/D4 edge executes; repeated at line 252-254 | closed |
| T-11-08 | Tampering | `deferred-items.md` D1–D4 clauses | medium | mitigate | `Effort / risk` rating rows count exactly 5 (lines 68, 91, 115, 135, 157) — no original rating deleted | closed |
| T-11-09 | Spoofing | 6-file / 49-importer / three-manager measurements | medium | mitigate | All three commands re-run at execution time, verbatim output recorded in ADR-0034 `## Code Locations` | closed |
| T-11-10 | Repudiation | The four D1–D4 verdicts | medium | mitigate | Each verdict carries verb, named owner and (where deferred) a concrete trigger | closed |
| T-11-11 | Tampering | Scope of the D2 withdrawal | medium | mitigate | `content_service.rs` and `event_manager.rs` retained with their own verb and owner rather than dropped with the withdrawn split | closed |
| T-11-12 | Information Disclosure | — | low | accept | Not applicable — no secret or personal datum read or written | closed |
| T-11-13 | Denial of Service | — | low | accept | Not applicable — file reads, three greps, two writes | closed |
| T-11-14 | Spoofing | Recovery pointer for `user.rs` | high | mitigate | Immutable SHA `3d48768` recorded in runnable `git show` form (`deferred-features.md:12-20`); SHA re-verified to resolve at audit time | closed |
| T-11-15 | Tampering | `deferred-features.md` | medium | mitigate | `--numstat` confirms 26 added / **0 deleted** — provably additive; original branch reference survives | closed |
| T-11-16 | Repudiation | Both feature records | medium | mitigate | D-00e evidence bar — every claim carries its producing command verbatim | closed |
| T-11-17 | Elevation of Privilege | ADR-0035 vs PROJECT.md `### Out of Scope` | medium | mitigate | ADR authorises no crate; `ls crates/` is 11 and `test -d crates/paladin-ml` exits 1 — re-verified at audit time | closed |
| T-11-18 | Information Disclosure | Recorded CLI surface | low | mitigate | Subcommand names already public in the checkout; record adds an auth-adjacent note so reintroduction is not scoped as a mechanical restore | closed |
| T-11-19 | Denial of Service | — | low | accept | Not applicable — no runtime surface touched | closed |
| T-11-20 | Spoofing | 20 per-row live-tree status cells | high | mitigate | Every path re-checked with `test -e`/`ls` at execution time; `.planning/registers/facade-04-m9-candidate-triage.md` records 20 rows in source order, each with its evidence command | closed |
| T-11-21 | Spoofing | `paladin-arsenal` / `paladin-sanctum` as target crates | high | mitigate | Recorded as artefacts on three cited grounds; both absent from `crates/` — re-verified at audit time | closed |
| T-11-22 | Repudiation | The triage tally | medium | mitigate | Row-identity definition stated before counting; tally 14 done / 6 not a candidate / 0 still open = 20; both superseded RESEARCH.md figures named explicitly | closed |
| T-11-23 | Tampering | `infrastructure-adapter-disposition.md` | medium | mitigate | `--numstat` confirms 1 added / **0 deleted** — one banner sentence, additions confined to file head | closed |
| T-11-24 | Elevation of Privilege | Triage authority over the source record | medium | mitigate | No blanket supersession — rows 1, 5, 13, 19, 20 decided individually; rows 14 and 17 corrected in place with do-not-re-delete marker carried forward | closed |
| T-11-25 | Information Disclosure | — | low | accept | Not applicable — repository-local adapter paths and SHAs only | closed |
| T-11-26 | Denial of Service | — | low | accept | Not applicable — existence checks and two writes | closed |
| T-11-27 | Tampering | `.planning/ledgers/milestone-07-08.md` | high | mitigate | Cell replacement only — `^\| REQ-` count is **86** before and after; `--numstat` shows 5 added / 5 deleted (added == deleted) | closed |
| T-11-28 | Tampering | `PROMOTION.md` numbering index | high | mitigate | Append only. 35 ADR rows, `sort -c` ascending, **zero duplicate numbers**; no row 0001-0033 modified — the single replacement is the next-free line itself | closed |
| T-11-29 | Repudiation | Five amended ledger cells | medium | mitigate | Each amendment dated, names plan 11-05, cites artefact by path, retains superseded text including plan 11-01's earlier amendment | closed |
| T-11-30 | Spoofing | REQUIREMENTS.md FACADE-03 correction | high | mitigate | Absence asserted only where `git rev-parse --verify` actually failed; plan 11-03's measured branch state carried through verbatim | closed |
| T-11-31 | Elevation of Privilege | The advanced next-free line | medium | mitigate | Precondition satisfied — `0034-*.md` and `0035-*.md` both exist on disk before the 0034→0036 advance, which was the phase's last edit | closed |
| T-11-32 | Information Disclosure | — | low | accept | Not applicable — paths, requirement IDs, SHAs and ADR numbers already in the checkout | closed |
| T-11-33 | Denial of Service | — | low | accept | Not applicable — file reads, a handful of greps, four writes | closed |
| T-11-SC | Tampering | npm/pip/cargo installs (supply chain) | low | accept | Not applicable and truthfully so — all five plans install, upgrade and remove nothing. `git diff --numstat` over the phase range touches **no** `Cargo.toml`, `Cargo.lock` or crate manifest; all edits confined to `.planning/` and `.project/` documents | closed |

*Status: open · closed · open — below high threshold (non-blocking)*
*Severity: critical > high > medium > low — only open threats at or above workflow.security_block_on count toward threats_open*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

**Totals:** 34 unique threats — 24 `mitigate` (all verified closed), 10 `accept` (all documented below). 8 rated `high`, every one dispositioned `mitigate` and closed with command-backed evidence.

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-11-01 | T-11-05, T-11-12, T-11-25, T-11-32 | Information Disclosure not applicable — every record holds only repository-local file paths, line numbers, requirement IDs, ADR numbers and commit SHAs already present in the checkout. No secret, credential or personal datum is read or written. | /gsd-secure-phase (ASVS L1) | 2026-08-09 |
| AR-11-02 | T-11-06, T-11-13, T-11-19, T-11-26, T-11-33 | Denial of Service not applicable — no service, endpoint, queue or scheduler is touched. Total execution is file reads, existence checks, a handful of greps and file writes. | /gsd-secure-phase (ASVS L1) | 2026-08-09 |
| AR-11-03 | T-11-SC | Supply chain not applicable — the phase installs, upgrades and removes nothing. D-13 confines all five plans to `.planning/` and `.project/` documents; no manifest or lockfile is within reach, verified by `git diff --numstat` over the phase commit range. | /gsd-secure-phase (ASVS L1) | 2026-08-09 |

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-08-09 | 34 | 34 | 0 | /gsd-secure-phase (orchestrator, ASVS L1 short-circuit) |

**Method.** `register_authored_at_plan_time: true` — all five PLAN files carried a parseable
`<threat_model>` block, so this run verified existing mitigations rather than scanning for new
threats (no retroactive-STRIDE). With `threats_open: 0`, `asvs_level: 1` and `block_on: high`,
the workflow's short-circuit rule applied and no separate auditor subagent was required; L1
grep-depth verification was performed inline and is cited per row above.

**Note on apparent deletions.** `git diff --numstat` reports 4 deleted lines on
`deferred-items.md` and 2 on `PROMOTION.md`. Inspection confirms these are not content removal:
the `deferred-items.md` lines are markdown strikethrough (`~~…~~`) annotations that retain the
original text verbatim, and the `PROMOTION.md` line is the intended next-free-number advance
(0034 → 0036). The annotation-only prohibitions (T-11-02, T-11-08, T-11-15, T-11-23) hold.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-08-09
