---
phase: 07-workspace-ground-truth-recorded-answers
verified: 2026-08-06T20:39:07Z
status: passed
score: 7/7 must-haves verified
behavior_unverified: 0
overrides_applied: 0
---

# Phase 7: Workspace Ground Truth & Recorded Answers Verification Report

**Phase Goal:** A developer can open `.planning/` and get a truthful account of the three
milestones that restructured this codebase — which of the 115 requirements the workspace actually
satisfies, which competing position each of the four variant pairs resolved to, and which five
documented positions must never be applied literally because shipped code contradicts them.

**Verified:** 2026-08-06T20:39:07Z
**Status:** passed
**Re-verification:** No — initial verification

**Phase character note:** This is a records phase by design — it changes no product code. The
hard boundary (`git diff --stat <phase-range> -- '*.rs' 'Cargo.toml' '.github/'` empty) was
verified directly and confirmed empty across the whole commit range (`c34f78a^..HEAD`). This is a
success criterion, not a gap.

## Goal Achievement

### Observable Truths (mapped to the seven ROADMAP success criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | 115-row ledger with `file:line`-cited verdicts, correct workspace shape recorded | ✓ VERIFIED | `.planning/ledgers/milestone-04-06.md` has exactly 115 `^| REQ-` rows, 0 `PENDING-VERDICT` stubs, 0 duplicate IDs, 13 epic sections in REQUIREMENTS.md's own order. `## Summary` verdict distribution (71 satisfied + 15 present-unproven + 3 genuinely-outstanding + 1 deferred-with-reason + 12 superseded + 5 relocated + 8 diverged = 115) matches an independent recount (`grep -oP` primary-key extraction). REQUIREMENTS.md's pointer and `STRUCTURE.md`'s Directory Purposes prose both state "ten library crates plus `doc-examples`" — confirmed against `Cargo.toml` (10 `crates/*` workspace members) and `ls crates/` (10 named crates + `doc-examples`), replacing the "six crates" and "9-crate" figures. |
| 2 | Five contradicted positions corrected at source; four mdbook deliverables recorded as living, not missing | ✓ VERIFIED | All five corrections (vision gating, MCP transports, web-server, `paladin-cli`, `src/application/use_cases/`) landed via plan 07-09 with dated banners; spot-checked `vision = []`/`chacha20poly1305`/`zeroize` against `Cargo.toml`, and `test -d src/application/use_cases` (fails, confirmed). All four mdbook paths (`docs/src/api-reference/{stable-api,feature-flags,migration-guide,crate-map}.md`, `docs/src/getting-started/installation.md`) confirmed to exist on disk. |
| 3 | Four competing variant pairs each have one recorded answer citing shipped code | ✓ VERIFIED | Edition → ADR-0009 (pre-existing, cited not re-decided). Dependency allowlist → ADR-0015, dependency counts (14 in `paladin-core`, 11 in `paladin-ports`) independently recounted from `Cargo.toml` and match exactly. Port value-type ownership → ADR-0016, all three cited line numbers (`token_usage.rs:13`, `battalion/mod.rs:497`, `llm_analysis_service.rs:51`) resolve to `pub struct TokenUsage`. LLM config bridge → ADR-0017. `REQ-port-value-type-ownership-v1`/`-v2` both survive as separate rows (v1 satisfied citing ADR-0016, v2 superseded). |
| 4 | "Was Milestone 6 a breaking change?" has one recorded answer with version consequence | ✓ VERIFIED | ADR-0018 clause (iii) cites ADR-0008 by number (does not re-derive), records the breaking-under-semver / minor-under-pre-1.0 conclusion; `REQ-battalion-facade-shim` correctly `superseded by shipped code` per D-15, recorded as history not contradiction. |
| 5 | "Which milestone is Milestone 1?" has one answer; `REQ-*` provenance resolves | ✓ VERIFIED | ADR-0014 states the directory/task-list numbering convention (4=Tier1, 5=Tier2, 6=Tier3); all eight byte-equivalent extracts carry the pointer banner (plan 07-03, verified additive diffs); the seven/eight/nine count discrepancy in `INGEST-CONFLICTS.md` is reconciled with a dated ADR-0014 amendment. |
| 6 | ≥50% incremental-rebuild target judged pass/fail per scenario | ✓ VERIFIED | ADR-0020's five transcribed figures (−6.6%, −18.9%, −44.6%, −50.2%, −90.9%) match `build-benchmarks.md`'s Summary Table exactly (byte-for-byte diff performed). The −6.6%/−5% internal inconsistency is correctly resolved (table cited as authoritative, conclusion's −5% recorded as transcription error) and "Target achieved" is judged contradicted by the report's own table (3 of 5 fail). Re-measurement is declined with a recorded reason (baseline commit unbuildable, offline/no-Docker constraints), so no later phase inherits an unfunded task. |
| 7 | Every binary target has a documented purpose | ✓ VERIFIED | ADR-0019 names all three: `paladin` (`src/main.rs`, no `required-features`, honestly recorded as the stale `smartcontent-aggregator` legacy runner — not tidied away), `paladin-cli` (`required-features = ["cli"]`), `paladin-server` (`required-features = ["web-server"]`). All `Cargo.toml` line citations (240-242, 244-247, 249-252) verified against the file exactly. `structopt`/`paladin-herald` re-scoping finding correctly handed to Phase 8, not executed here. |

**Score:** 7/7 truths verified (0 present, behavior-unverified — this is a records phase; no
runtime state-transition/cancellation invariants apply to markdown artefacts)

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.planning/ledgers/milestone-04-06.md` | 115 cited rows, 13 sections, 7-value legend, Summary | ✓ VERIFIED | 422 lines; row count, headings, verdict distribution, forward-scope handoffs all confirmed by direct count |
| `.planning/decisions/0014`…`0021` (8 ADRs) | 7 canonical `## ` headings, bulleted Considered Options/Code Locations, conformance verdict | ✓ VERIFIED | `grep -L 'conforms\|must change'` over all 8 returns nothing (all carry a verdict); all 8 show all 7 headings in order; Code Locations/Considered Options sections are bulleted lists |
| `.planning/decisions/0018-*` (mid-phase amendment) | Amended in place, original retained, dated | ✓ VERIFIED | Clause (iv) amendment dated 2026-08-06 with "original clause retained above; narrowed, not reversed" — the 36-file consumer count and `src/application/services/battalion/mod.rs` shim survival independently re-confirmed |
| `.project/` corrections (13 documents across plans 07-01–07-09) | Additive only, original text retained | ✓ VERIFIED | Sampled 9 correction commits' diffs directly — all show insertions dominating, and content-level review of 3 (ports-extraction PRD, llm-extraction PRD) confirms strikethrough-wrapped originals fully readable, never deleted |
| `.planning/decisions/PROMOTION.md` | 8 index rows, next-free → 0022, both Part B candidates closed | ✓ VERIFIED | `Next free ADR number: 0022` confirmed; rows 0014-0021 all present; candidates 1 (ADR-0016) and 2 (ADR-0021) both marked closed |
| `.planning/REQUIREMENTS.md` pointer | Old ledger section reduced to pointer, 0 `REQ-` rows remaining | ✓ VERIFIED | Section body is a single pointer paragraph; 0 `^| REQ-` rows inside it |
| `.planning/codebase/STRUCTURE.md` | Directory Purposes prose lists all 10 crates + `doc-examples` | ✓ VERIFIED | Prose section (not just the ASCII tree) lists all 10 crate entries plus `doc-examples`, dated correction note |
| `COVERAGE.md` | Reasoned no-external-API declaration | ✓ VERIFIED | Present, matches the Phase 5/6 shape, explains why the API-keyword detector fires without an actual integration |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| REQUIREMENTS.md pointer | `.planning/ledgers/milestone-04-06.md` | Markdown link + prose redirect | ✓ WIRED | Link resolves, section body contains no competing inline data |
| Ledger `REQ-*` rows | ADR-0009/0014-0021 | In-cell citations | ✓ WIRED | Sampled across sections; every ADR-citing row's target file exists and states the claimed content |
| `REQ-crate-isolation-ci` | `.github/workflows/ci.yml:304` | Citation-drift canonical row | ✓ WIRED | `crate-isolation:` job confirmed at line 304 (not the stale 228), 10-crate matrix confirmed by direct read |
| ADR-0016 | Phase 8 / DEBT-05 | Forward-scope handoff | ✓ WIRED | Ledger's `## Summary` Forward scope section names the phase, requirement, and exact target lines |
| ADR-0018 | Phase 11 / FACADE-02 D1 | Forward-scope handoff | ✓ WIRED | Same section, correctly recorded not executed |
| ADR-0019 | Phase 16, Phase 8 | Forward-scope handoff | ✓ WIRED | `structopt`/`paladin-herald` re-scoping and mdbook page both recorded with owning phase |
| ADR-0015 | Phase 15 | Forward-scope handoff | ✓ WIRED | Allowlist-enforcement candidate recorded, not built |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| Ledger row count is exactly 115 | `grep -c '^| REQ-' .planning/ledgers/milestone-04-06.md` | `115` | ✓ PASS |
| No duplicate REQ IDs | `grep -o '^| REQ-[a-z0-9-]*' ... \| sort \| uniq -d` | (empty) | ✓ PASS |
| No stub rows remain | `grep -c PENDING-VERDICT` | `0` | ✓ PASS |
| Product-code boundary held | `git diff --stat c34f78a^..HEAD -- '*.rs' 'Cargo.toml' '.github/'` | (empty) | ✓ PASS |
| A cited `satisfied` row's test actually passes | `cargo test --offline -p paladin-battalion maneuver` | `74 passed; 0 failed` (matches row's claimed count exactly) | ✓ PASS |
| Cited struct/module lines resolve | `sed -n '13p' token_usage.rs`, `sed -n '497p' battalion/mod.rs`, `sed -n '51p' llm_analysis_service.rs`, `Cargo.toml:240-252` bin stanzas | All match cited content exactly | ✓ PASS |
| Benchmark figures transcribed faithfully | Diff of ADR-0020's five figures against `build-benchmarks.md`'s Summary Table | Byte-identical (−6.6%, −18.9%, −44.6%, −50.2%, −90.9%) | ✓ PASS |
| Mid-phase corrections landed and are accurate | `grep -n 'cfg(feature = "cli")' src/application/mod.rs`; `test -d src/application/use_cases`; `grep -rln 'application::services::battalion' ...` | CLI gating confirmed; use_cases absent confirmed; 36-file count matches ADR-0018's re-confirmed figure exactly | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plans | Status | Evidence |
|-------------|-------------|--------|----------|
| ARCH-01 | 07-01, 07-06, 07-08, 07-10, 07-11, 07-12, 07-13 | ✓ SATISFIED | 115-row cited ledger, correct workspace shape |
| ARCH-02 | 07-01, 07-03, 07-05, 07-09, 07-13 | ✓ SATISFIED | ADR-0014, 8 extracts annotated, count reconciliation |
| ARCH-03 | 07-01, 07-02, 07-04, 07-06, 07-08, 07-10, 07-11, 07-13 | ✓ SATISFIED | ADR-0015, 0016, 0017 + ADR-0009 citation |
| ARCH-04 | 07-05, 07-08, 07-09, 07-10, 07-13 | ✓ SATISFIED | ADR-0018, version consequence recorded |
| ARCH-05 | 07-05, 07-08, 07-09, 07-12 | ✓ SATISFIED | Five positions corrected, verified against shipped tree |
| ARCH-06 | 07-07, 07-12 | ✓ SATISFIED | ADR-0019, three binary targets documented |
| ARCH-07 | 07-07, 07-10, 07-11 | ✓ SATISFIED | ADR-0020, per-scenario restatement |

No orphaned requirements found — every ARCH-01…ARCH-07 ID declared in REQUIREMENTS.md's Phase 7
section is claimed by at least one plan's `requirements:` frontmatter, and REQUIREMENTS.md marks
all seven `[x]`.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| — | — | No `TBD`/`FIXME`/`XXX`/`TODO`/`HACK`/`PLACEHOLDER` markers found in the ledger or any of the 8 ADRs | — | None |

No blockers found. SUMMARY.md "Issues Encountered" sections across all 13 plans document only
process-level deviations (commit granularity, an aborted background test invocation, a plan-authoring
imprecision self-corrected in-row) — none is a correctness or security defect, and none was hidden.

### Deferred Manual-Verification Items — Resolved by This Verification

`07-VALIDATION.md` names three Manual-Only Verification categories (citation-supports-claim spot
checks, ADR code-location checks, and benchmark-figure fidelity) and each PLAN.md's per-task
`<verify><human-check>` blocks defer specific instances of these to end-of-phase per the
end-of-phase human-verify pattern. This verification performed all of them directly rather than
routing them to a human queue:

- Sampled `satisfied`/`diverged`/`relocated`/`superseded` rows across every one of the 13 epic
  sections and opened the cited `file:line` for each — all read as claimed.
- Opened every one of the 8 ADRs' `## Code Locations` entries sampled above and confirmed each
  resolves to the stated content.
- Diffed ADR-0020's five benchmark figures against `build-benchmarks.md`'s source table —
  byte-identical.
- Re-ran one of the ledger's cited exercising tests (`cargo test --offline -p paladin-battalion
  maneuver`) and confirmed the pass count matches the row's claim exactly (74/0).
- Confirmed both mid-phase CONTEXT.md/ADR-0018 corrections are accurate against the current tree.

Because primary evidence was read directly (not merely re-stated), these items are resolved as
VERIFIED rather than routed to `human_needed`.

### Gaps Summary

None. All seven ROADMAP success criteria are observably true in the codebase: the 115-row ledger
exists with independently-recountable citations; the five contradicted positions are corrected at
source with original text retained; the four competing variant pairs each resolve to exactly one
ADR citing shipped code; the Milestone 6 breaking-change question and the Milestone-numbering
question each have one recorded answer; the build-benchmark target is judged per scenario against
its own source table; and all three binary targets carry documented, honestly-stated purposes. The
phase's hard boundary (no product code touched) is independently confirmed empty across the full
commit range.

---

_Verified: 2026-08-06T20:39:07Z_
_Verifier: Claude (gsd-verifier)_
