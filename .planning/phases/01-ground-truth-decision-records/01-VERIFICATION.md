---
phase: 01-ground-truth-decision-records
verified: 2026-07-31T13:31:33Z
status: gaps_found
score: 3/5 must-haves verified
behavior_unverified: 0
overrides_applied: 0
gaps:
  - truth: "Six ADRs exist, one per competing variant pair (BattalionConfig, BattalionResult, Formation minimum Paladin count, temperature range, Herald trait signature, coverage gate), each naming the chosen variant and the shipped code it was checked against."
    status: failed
    reason: "Only five ADRs exist (0001-0005). ADR-0006 (coverage gate, RECON-07) was never authored. Plan 01-04, the only plan that produces it, halted at its own precondition — cargo-llvm-cov cannot be installed (crates.io returns HTTP 403 in this environment), Docker is unavailable so the --features integration-tests scope cannot run, and the repo's stale root lcov.info predates the workspace migration. No 01-04-SUMMARY.md exists; ROADMAP.md itself still shows `- [ ] 01-04-PLAN.md` unchecked."
    artifacts:
      - path: ".planning/decisions/0006-coverage-gate.md"
        issue: "Does not exist"
      - path: ".planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md"
        issue: "Does not exist — no raw measurement evidence was ever produced"
      - path: ".planning/PROJECT.md"
        issue: "Key Decisions table row for ADR-0006 is an honest 'Pending' placeholder, not a real decision row (line 1036)"
    missing:
      - "A workspace coverage measurement produced by actually running `cargo llvm-cov` against the current tree (via a network-enabled environment, or by reading the number off the existing `.github/workflows/integration-tests.yml` CI run), recorded with its exact command, flags, toolchain versions, date and commit"
      - "`.planning/decisions/0006-coverage-gate.md` authored from that measurement per plan 01-04 Task 2's full D-07..D-10 specification (floor, target, ratchet trigger, module-scoped gates handed to VERIFY-05)"
      - "The blocking human-verify checkpoint in plan 01-04 Task 3, run to confirm the number and gate"
  - truth: "The coverage question has one number and one scope, so Phase 3 can objectively pass or fail against it instead of choosing between 80% and 85%."
    status: failed
    reason: "This truth is downstream of ADR-0006, which does not exist, so it cannot be true. Confirmed directly: ROADMAP.md's Phase 3 success criterion 1 still reads '... coverage at or above the gate recorded in Phase 1 (baseline 60.88%) and integration coverage at or above 70% (baseline 67.79%)' — the exact two-baseline, unresolved-scope statement RECON-07 was created to eliminate. Plan 01-08 Task 3, which would have amended this criterion, was explicitly not executed (per its own SUMMARY) because it depends on ADR-0006's number."
    artifacts:
      - path: ".planning/ROADMAP.md"
        issue: "Phase 3 success criterion 1 (line 219) names unit and integration coverage separately with the two stale Milestone-1 baselines, and does not cite any ADR"
    missing:
      - "ADR-0006 (see gap above), which this amendment is entirely blocked on"
      - "The scoped ROADMAP.md Edit specified in plan 01-08 Task 3, run after ADR-0006 exists"
deferred: []
---

# Phase 1: Ground Truth & Decision Records Verification Report

**Phase Goal:** `.planning/` describes the v0.7.0 code as it actually is, and each of the six contested definitions has exactly one recorded, evidence-cited answer that later milestones can build on.
**Verified:** 2026-07-31T13:31:33Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths (ROADMAP.md Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A developer can open one status ledger and see, for every outstanding Milestone-1 task item, whether shipped code already satisfies it — each verdict carrying a `file:line` citation rather than a task-list checkbox. | ✓ VERIFIED | `.planning/ledgers/milestone-01.md` exists (~80KB), holds sections for all 10 epics + unit-test-improvements workstream, and its own `## Outstanding item reconciliation` section states 39/39 nested outstanding items match `intel/task-completion-state.md`'s deterministic per-file breakdown exactly (independently re-counted: 19+4+4+3+3+2+2+2 = 39, confirmed against `intel/task-completion-state.md:15-23`). Spot-checked citations (`battalion/mod.rs:37`, `:549`; `formation.rs:109`; `commander.rs:1912`) all resolve against the live tree at the cited lines. Verdict distribution table (100 satisfied / 23 present,unproven / 11 genuinely outstanding / 19 superseded / 1 deferred) is stated as a count, not implied. |
| 2 | Six ADRs exist, one per competing variant pair (`BattalionConfig`, `BattalionResult`, Formation minimum Paladin count, temperature range, `Herald` trait signature, coverage gate), each naming the chosen variant and the shipped code it was checked against. | ✗ FAILED | Only 5 of 6 exist: `0001-battalion-config.md`, `0002-battalion-result.md`, `0003-formation-min-paladins.md`, `0004-temperature-validation.md`, `0005-herald-trait.md` — all parse cleanly via `adr-parser.cjs` (`status: accepted`, non-empty `decisions`/`options_considered`/`key_files`) and their citations were re-verified against the live source (`battalion/mod.rs:37,549`, `formation.rs:109-111`, `commander.rs:1912`, `llm_port.rs:754` `ProviderCapabilities` confirmed to carry no temperature field). `0006-coverage-gate.md` does not exist. See gaps. |
| 3 | The ledger records the places where shipped code has already superseded an ingested requirement — MCP Streamable-HTTP in place of SSE, Sanctum/Qdrant in place of `sqlite-vss`, the interactive REPL that Epic 9 declared a non-goal — so no later phase mistakes divergence for a defect. | ✓ VERIFIED | All three rows present in `## Divergences` section of `milestone-01.md`, each classed `superseded by shipped code` with resolving `file:line` citations (`mcp_streamable_http_adapter.rs:76`, `qdrant_adapter.rs:59` / `in_memory_adapter.rs:73`) and named passing exercisers (`streamable_http_round_trip_with_correct_bearer_token_succeeds`, `test_store_and_retrieve`). REPL row carries the bold "documented non-goal that shipped anyway" callout as required. |
| 4 | The coverage question has one number and one scope, so Phase 3 can objectively pass or fail against it instead of choosing between 80% and 85%. | ✗ FAILED | No measurement was ever produced (no `01-coverage-measurement.md`, no ADR-0006). `ROADMAP.md`'s Phase 3 success criterion 1 is confirmed unamended: it still names unit coverage and integration coverage as two separate figures with the two original stale baselines (60.88% / 67.79%). See gaps. |
| 5 | The Epic 10 Task 7.0 dispute is answered — either the Final Documentation Review is outstanding work with an owner, or the validation report is recorded as wrong — and the 102-vs-103 subtask discrepancy is explained. | ✓ VERIFIED | `## Epic 10 Task 7.0 — dispute resolution (RECON-08)` section in `milestone-01.md` states both documents' claims side by side with citations, records the search commands run (`ls`, `grep -rn`, `grep -c`) and their empty/positive results, reaches one of the two permitted verdicts ("the validation report is recorded as wrong"), and explains the 102-vs-103 arithmetic (task list's own deterministic count is 103; the validation report's 102 does not reconcile against 103 under either reading). |

**Score:** 3/5 ROADMAP success criteria verified (0 present-but-behavior-unverified)

### Requirement IDs Cross-Reference (RECON-01..08)

| Requirement | REQUIREMENTS.md checkbox | Status | Evidence |
|---|---|---|---|
| RECON-01 | `[x]` | ✓ SATISFIED | Ledger + divergences + bookkeeping corrections (Battalion module path, requirement-count discrepancy) all present and re-verified. One incompleteness noted below (not a RECON-01 breach on its literal wording, but a completeness gap): `REQ-battalion-result-v1` has no row anywhere in the ledger. |
| RECON-02 | `[x]` | ✓ SATISFIED | ADR-0001 exists, parses, citations verified. |
| RECON-03 | `[x]` | ✓ SATISFIED | ADR-0002 exists, parses, citations verified. |
| RECON-04 | `[x]` | ✓ SATISFIED | ADR-0003 exists, parses, citations verified (both halves of the live contradiction re-confirmed in the tree). |
| RECON-05 | `[x]` | ✓ SATISFIED | ADR-0004 exists, parses; `ProviderCapabilities` re-confirmed to have no temperature field, both contradicting sites (`paladin_builder.rs:1112`, `llm/config/llm.rs:14`) re-confirmed. |
| RECON-06 | `[x]` | ✓ SATISFIED | ADR-0005 exists, parses, `herald.rs:49` trait method set reproduced exactly. |
| RECON-07 | `[ ]` (still unchecked in REQUIREMENTS.md) | ✗ BLOCKED | No ADR-0006, no measurement file. REQUIREMENTS.md itself already reflects this honestly (checkbox left open). |
| RECON-08 | `[x]` | ✓ SATISFIED | Epic 10 dispute resolved with recorded search and verdict. |

7 of 8 RECON requirement IDs are satisfied; RECON-07 is genuinely blocked, not silently dropped — REQUIREMENTS.md's own checkbox for it is correctly left unchecked.

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.planning/decisions/PROMOTION.md` | ADR conventions, numbering index, supersession mechanism, 11-candidate promotion inventory | ✓ VERIFIED | Contains `Next free ADR number: 0007`, `Superseded`, 12 `Owner phase` occurrences (≥11 required), `2026-09-30`, `AgentProvisioner`, `rustsec-remediation-plan.md`. |
| `.planning/decisions/0001-battalion-config.md` | RECON-02 ADR | ✓ VERIFIED | Parses; `must change` + GAP-07 named; citations resolve. |
| `.planning/decisions/0002-battalion-result.md` | RECON-03 ADR | ✓ VERIFIED | Parses; `conforms`; field substitutions (`per_paladin_times`, `node_errors`) confirmed present in shipped struct. |
| `.planning/decisions/0003-formation-min-paladins.md` | RECON-04 ADR | ✓ VERIFIED | Parses; both halves of the live contradiction re-confirmed against the tree. |
| `.planning/decisions/0004-temperature-validation.md` | RECON-05 ADR | ✓ VERIFIED | Parses; `ProviderCapabilities` field-absence claim re-confirmed. |
| `.planning/decisions/0005-herald-trait.md` | RECON-06 ADR | ✓ VERIFIED | Parses; trait method set matches `herald.rs`. |
| `.planning/decisions/0006-coverage-gate.md` | RECON-07 ADR | ✗ MISSING | Never authored — plan 01-04 halted at precondition. |
| `.planning/phases/01-ground-truth-decision-records/01-coverage-measurement.md` | Raw coverage evidence | ✗ MISSING | Never produced. |
| `.planning/ledgers/milestone-01.md` | Milestone 1 cited status ledger | ✓ VERIFIED | All epic sections present, reconciliation section present and internally consistent, but see key-link gap below for `REQ-battalion-result-v1`. |
| `.planning/PROJECT.md` | Precedence order + Key Decisions table | ⚠️ PARTIAL | Precedence order fully updated (`ADR → shipped tree` at all 3 sites). Key Decisions table has 5 real rows + 1 explicit, non-fabricated "Pending" placeholder row for ADR-0006 — honestly incomplete rather than silently wrong. |
| `.planning/REQUIREMENTS.md` | Milestone 1 ledger body reduced to pointer | ✗ NOT DONE (correctly withheld) | Body is unreduced (still the full 2361-2700 legacy table, byte-identical to pre-phase state) because plan 01-08's own mandatory safety check found `REQ-battalion-result-v1` missing from the destination ledger and correctly HALTed rather than reducing to a pointer at an incomplete destination. Tracked as an open item in `.planning/WINDOWS.md` (id 1, `unmet-truth`, `open`). |
| `.planning/ROADMAP.md` | Phase 3 criterion amended to one coverage number | ✗ NOT DONE | Untouched, per explicit dependency on the missing ADR-0006. |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `.planning/PROJECT.md` Key Decisions | `.planning/decisions/0001-...0005-*.md` | Markdown link per row | ✓ WIRED | All 5 links present and resolve; Outcome cells match each ADR's `## Code Conformance` value. |
| `.planning/decisions/0005-herald-trait.md` | `crates/paladin-core/src/platform/container/herald.rs:49` | Code Locations citation | ✓ WIRED | `pub struct` / trait re-confirmed at cited region. |
| `.planning/decisions/0001-battalion-config.md` | `crates/paladin-core/src/platform/container/citadel.rs:280` | Code Locations citation for the duplicate | ✓ WIRED | Placeholder struct with 3 named fields confirmed present at that location. |
| `.planning/ledgers/milestone-01.md` | `.planning/REQUIREMENTS.md` (subset of `REQ-*` IDs) | "supersedes" relationship (D-17) | ⚠️ PARTIAL | Not a strict superset: `REQ-battalion-result-v1` exists in REQUIREMENTS.md's Milestone 1 body (`REQUIREMENTS.md:2578`, "Variant (group 4)") and is discussed by name in ADR-0002's Considered Options, but has zero rows in `milestone-01.md`'s Epic 4 or Epic 5 tables. Independently confirmed: `grep -c "REQ-battalion-result-v1" .planning/ledgers/milestone-01.md` = 0. This is exactly why plan 01-08 Task 2 correctly refused to reduce REQUIREMENTS.md's body to a pointer. |
| `.planning/ROADMAP.md` Phase 3 SC1 | `.planning/decisions/0006-coverage-gate.md` | Amendment citing the ADR | ✗ NOT_WIRED | ADR doesn't exist; amendment never made. |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| RECON-01 | 01-01, 01-05, 01-06, 01-07 | Cited status ledger + divergences + bookkeeping | ✓ SATISFIED | See above; one open completeness item (`REQ-battalion-result-v1`) tracked in WINDOWS.md, not silently dropped. |
| RECON-02 | 01-02 | BattalionConfig ADR | ✓ SATISFIED | ADR-0001. |
| RECON-03 | 01-02 | BattalionResult ADR | ✓ SATISFIED | ADR-0002. |
| RECON-04 | 01-03 | Formation minimum Paladin count ADR | ✓ SATISFIED | ADR-0003. |
| RECON-05 | 01-03 | Temperature validation ADR | ✓ SATISFIED | ADR-0004. |
| RECON-06 | 01-01 | Herald trait ADR | ✓ SATISFIED | ADR-0005. |
| RECON-07 | 01-04 | Coverage gate ADR | ✗ BLOCKED | Plan 01-04 never executed past its precondition; environment cannot run `cargo-llvm-cov` (crates.io HTTP 403) or Docker-backed integration tests. |
| RECON-08 | 01-05 | Epic 10 Task 7.0 dispute | ✓ SATISFIED | Resolved with recorded search + verdict. |

No orphaned requirements found — all 8 RECON-* IDs declared in plan frontmatter map onto the 8 IDs REQUIREMENTS.md assigns to Phase 1.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| — | — | — | — | No `TBD`/`FIXME`/`XXX`/`TODO`/`HACK`/unreferenced-placeholder markers found in any Phase 1 document (`.planning/decisions/*.md`, `.planning/ledgers/milestone-01.md`, `.planning/PROJECT.md`). The only "placeholder" occurrences are accurate descriptions of the `citadel.rs:280` struct itself (the ADR's actual subject matter), not authoring debt. |

No blockers found via anti-pattern scan. The phase's incompleteness (ADR-0006, ROADMAP amendment, `REQ-battalion-result-v1` row, REQUIREMENTS.md reduction) is all honestly disclosed in SUMMARY.md files and `.planning/WINDOWS.md` rather than hidden or fabricated — this is a meaningfully different failure mode than a stub or fabricated pass, but the ROADMAP success criteria are still unmet as written.

### Probe Execution

No `scripts/*/tests/probe-*.sh` files exist and none are declared in any Phase 1 PLAN/SUMMARY. SKIPPED (no probes).

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| All 5 existing ADRs parse via GSD's own ADR parser | `node .claude/gsd-core/bin/lib/adr-parser.cjs --input <file>` for each of 0001-0005 | All 5 exit with `"status": "accepted"` and non-empty `decisions`/`options_considered`/`key_files` | ✓ PASS |
| `formation.rs:109-111` still rejects <2 Paladins as ADR-0003 describes | `sed -n '100,115p' formation.rs` | `"Formation requires at least 2 Paladins, got {}"` confirmed at cited location | ✓ PASS |
| `test_auto_selects_formation_for_single_paladin` exists and is the test ADR-0003/ledger cite | `sed -n '1900,1920p' commander.rs` | Test found at expected location, non-`#[ignore]` | ✓ PASS |
| `ProviderCapabilities` (ADR-0004's central claim) carries no temperature field | `grep -n "struct ProviderCapabilities" -A 20 llm_port.rs` | 7 fields listed, none temperature-related | ✓ PASS |
| Ledger's 39-item reconciliation matches the deterministic source | Manual sum of `intel/task-completion-state.md`'s 8-file breakdown | 19+4+4+3+3+2+2+2 = 39, matches ledger's stated total exactly | ✓ PASS |
| `REQ-battalion-result-v1` presence in ledger | `grep -c "REQ-battalion-result-v1" .planning/ledgers/milestone-01.md` | 0 (confirmed absent) | ✗ FAIL (documented gap, see above) |

## Gaps Summary

Two of five ROADMAP.md success criteria are unmet, both stemming from the same root cause: **RECON-07 (the coverage gate) never executed.** Plan 01-04 is the only plan that produces ADR-0006 and the coverage measurement, and it halted correctly at its own stated precondition — `cargo-llvm-cov` is not installable in this environment (crates.io returns HTTP 403), Docker is unavailable so the CI's `--features integration-tests` scope cannot be reproduced, and the repository's pre-existing `lcov.info` predates the workspace migration and cannot substitute. No coverage number was fabricated or estimated in its place — the plan's own halt-and-report protocol worked as designed, and REQUIREMENTS.md's own RECON-07 checkbox is correctly left unchecked rather than falsely marked done.

Downstream of that single blocker: PROJECT.md's Key Decisions table carries an honest "Pending" placeholder for ADR-0006 rather than a fabricated row, and ROADMAP.md's Phase 3 success criterion 1 is untouched (still names two separate, stale coverage baselines) because its amendment (plan 01-08 Task 3) explicitly declined to run without the ADR-0006 number to cite.

A third, independent finding: `.planning/ledgers/milestone-01.md` is missing a row for `REQ-battalion-result-v1` (an Epic 4 competing-variant ID that ADR-0002 discusses by name but the ledger never turned into a row). This was caught by plan 01-08 Task 2's own mandatory safety check, which correctly HALTed rather than reducing REQUIREMENTS.md's Milestone 1 ledger body to a pointer at an incomplete destination — so REQUIREMENTS.md's full legacy ledger body is still intact and no data was lost. This gap is already tracked in `.planning/WINDOWS.md` (id 1, open, blocks `/gsd-ship`).

None of these three gaps is deferred to a later phase in the roadmap: Phases 3, 5 and 15 (which consume RECON-07's answer) all assume Phase 1 already produced it — none of them re-run the measurement or author the ADR themselves.

**What closes this phase:**
1. Obtain a real workspace coverage figure — either by running `cargo llvm-cov --workspace ...` in a network-enabled environment, or by reading the number off an existing/triggered `.github/workflows/integration-tests.yml` CI run — and complete plan 01-04's Task 1 and Task 2 (author `01-coverage-measurement.md` and `.planning/decisions/0006-coverage-gate.md`), then its Task 3 human checkpoint.
2. Add the missing `REQ-battalion-result-v1` row to `milestone-01.md`'s Epic 4 table (content is already available verbatim from ADR-0002's Considered Options), then re-run plan 01-08 Task 2's subset check and complete the REQUIREMENTS.md pointer reduction.
3. Complete plan 01-08 Task 1 (fill the ADR-0006 row) and Task 3 (amend ROADMAP.md Phase 3's criterion) once step 1 is done.

---

*Verified: 2026-07-31T13:31:33Z*
*Verifier: Claude (gsd-verifier)*
