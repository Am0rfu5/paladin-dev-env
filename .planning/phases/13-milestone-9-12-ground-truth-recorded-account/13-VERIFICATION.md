---
phase: 13-milestone-9-12-ground-truth-recorded-account
verified: 2026-08-10T21:44:37Z
status: passed
score: 7/7 must-haves verified
behavior_unverified: 0
overrides_applied: 0
human_verification:

  - test: "Ratify the CR-01 / ORCH-03 scope trade-off: `crates/doc-examples/src/sidecar.rs:34` (and its `:25` doc comment) still builds the unprefixed `/agents/{agent}/execute` URL, embedded verbatim into the rendered `docs/src/deployment-topologies/sidecar.md` page via mdBook `{{#include}}`, even though the page's own prose (line 29-30) correctly states the `/v1`-prefixed route. A reader who copies the rendered example writes a client that 404s. Phase 13 left this uncorrected because fixing it requires a `.rs` edit, which would breach the phase's own D-19 zero-`.rs` boundary — the same boundary this phase's close-out independently measures and proves held (`git diff --name-only <base>..HEAD -- '*.rs'` → 0, re-confirmed independently by this verification). The defect, its cause, its exact fix, and its owner (Phase 14) are all recorded in REQUIREMENTS.md's Phase-14 hand-off item 6 and in `13-REVIEW.md` CR-01, citing exact file:line evidence."
    expected: "A human confirms this is an acceptable scope boundary for ORCH-03's `[x]` — i.e., that ORCH-03's done-when ('anyone applying a run-5 requirement literally cannot write to a path that does not exist') is read as scoped to the five items (a)-(e) it names (matching ROADMAP.md Phase 13 Success Criterion 4's own narrower wording: 'the four stale module and document paths are corrected at source, and the agent API's route surface has one answer'), not as an unbounded claim over every code example in the tree — OR overrides that reading and reopens ORCH-03 / directs an out-of-band `.rs` fix that breaches D-19 deliberately."
    why_human: "This is an editorial scope-interpretation judgment, not a fact a grep can settle: the literal done-when sentence is broader than the five named items, but the roadmap's own success criterion for the same phase is scoped identically to the five items, and the finding was honestly self-disclosed (not hidden) with a working fix and a named owner. Verifier's own recommendation: DEFENSIBLE AS DOCUMENTED — this is disclosure and deferral done right, not silent scope-cutting — but it is a call a human should explicitly ratify rather than have an agent quietly wave through."
---

# Phase 13: Milestone 9-12 Ground Truth & Recorded Account Verification Report

**Phase Goal:** A developer can open `.planning/` and get a truthful account of the four milestones that finished, hardened, documented and exposed this framework — what the 120 requirements actually delivered, which paths and routes in them are historical, and what the two seams Milestone 12 left as defaults have been decided to be.
**Verified:** 2026-08-10T21:44:37Z
**Status:** human_needed
**Re-verification:** No — initial verification

## Phase Character

This is a documentation ground-truth phase. It deliberately changed zero `.rs` files (D-19). That
boundary was independently re-measured for this verification, not merely re-cited: `git diff
--name-only e12f18306ca9a80b1c3301e6afca31602e7c41ec..HEAD -- '*.rs' | wc -l` → **`0`**, and `git log
--oneline <base>..HEAD --stat -- '*.rs'` shows zero commits touching any `.rs` path across all 78
commits in the phase's range. The boundary held.

The highest-value verification for this phase class is re-measuring the specific factual claims the
phase asserts as corrected, rather than trusting SUMMARY prose. This report re-ran roughly two dozen
of the phase's own cited commands and file:line references independently (a representative sample
across all four milestones plus the Deferred-QA and project-management sections) and found the
citation discipline to be unusually high — every re-run command reproduced the exact figure the
ledger states, with one narrow exception (see Anti-Patterns / Findings below).

## Goal Achievement

### Observable Truths (ORCH-01 through ORCH-05, roadmap Success Criteria 1-6)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `.planning/ledgers/milestone-09-12.md` exists with all 120 `REQ-*` rows carrying a `file:line`-cited verdict, matching REQUIREMENTS.md 1-for-1 | ✓ VERIFIED | `grep -c '^| REQ-' .planning/ledgers/milestone-09-12.md` → `120`; `grep -oE '^\| REQ-[a-z0-9-]+' ... \| sort -u \| wc -l` → `120` distinct IDs (independently re-run, matches ledger's own close-out claim) |
| 2 | The ledger states plainly that the whole M9 orchestrator subsystem, whole M10 tooling set, mdbook and whole M12 web API ship, so nothing is re-planned | ✓ VERIFIED | Per-milestone corroboration paragraphs present in ledger; spot-checked ~15 individual rows (M9 Epics 1-5, M10 Epic 2, M12 Epics 1, 5) against live `crates/`/`src/` — every cited `file:line`, struct/fn name, and test name exists exactly as claimed; every cited `cargo test` command re-run and reproduced the stated pass count (see Behavioral Spot-Checks) |
| 3 | Milestone 10 is recorded both 100% shipped and with one acceptance criterion false (SUPPLY-01/`REQ-audit-toml-single-source`), both halves dated | ✓ VERIFIED | Ledger's "Shipped, one acceptance criterion false" head table; re-confirmed independently: `grep -n "cargo audit --ignore" .github/workflows/ci.yml` → zero matches (fix holds); commit `cb75b2b` exists in `git log`; ADR-0036 exists and cites the same invariant |
| 4 | M9 0 open / M10 0 open-contradicted / M11 26 open (only genuine open count) / M12 3 open (vacuous scaffolding) / project-management 1 open (nonexistent, a template formatting example) — five verdicts recorded once, none converted to a task | ✓ VERIFIED | Ledger's per-milestone corroboration paragraph and the project-management row; re-confirmed independently: `grep -n '\[ \]\|\[x\]' .project/project-management/tasks-project-management-setup.md` returns exactly the one formatting-example line the ledger describes, matching verbatim |
| 5 | The four ORCH-03(b)-(e) stale paths (`listener_service.rs`, `llm_port.rs`, `Design_and_Architecture.md`, asciinema/README) are corrected at source with dated banners naming the current path, originals retained | ✓ VERIFIED | Independently confirmed all four: old paths absent (`test -f` fails on all four), new paths present and correct (`listener.rs:141` `ListenerOrchestrator`, `crates/paladin-ports/src/output/llm_port.rs` exists, `docs/src/appendix/design-and-architecture.md` is exactly 311 lines with zero hits for the 7 named subsystems, `docs/assets/`/`docs/DEMOS.md` absent, `docs/src/assets/` holds exactly 6 files); dated correction banners found in `.planning/intel/requirements.md` and `.project/Deferred-QA-CICD-Completion/{DEFERRED_COVERAGE.md,prd-deferred-qa-completion.md}` for all four |
| 6 | The agent route surface has one recorded answer (`/v1`-prefixed, ADR-0037) confirmed against the committed `openapi.json`, with Epic 1/3/4/5 unprefixed route text marked superseded provenance | ✓ VERIFIED, with one disclosed residue outside the requirement's declared five-item scope | `crates/paladin-web/src/agent_controller.rs:723` `API_V1_PREFIX = "/v1"`; `openapi.rs:103` `spec_paths_are_versioned_under_v1` test exists and asserts exactly the paths claimed; `docs/src/deployment-topologies/sidecar.md:29` states the `/v1`-prefixed form. **Residue:** `crates/doc-examples/src/sidecar.rs:34` (an `{{#include}}`-embedded code example, not one of ORCH-03's five named items) still builds the unprefixed URL — see human-verification item above |
| 7 | ADR-0038 (`AgentProvisioner` stays in `paladin-web`) and ADR-0039 (no Garrison/Arsenal on HTTP topology, permanent) exist, each with seven required H2 headings, no frontmatter, and were ratified via a genuine blocking human checkpoint before authoring | ✓ VERIFIED | Both files confirmed to have exactly 7 `## ` headings in the required order (`Status, Context, Decision, Considered Options, Code Locations, Code Conformance, Downstream Consumers`), no YAML frontmatter; `13-09-SUMMARY.md`'s `## Checkpoint Status` section independently read and confirms: resolved via `AskUserQuestion` interactive mechanism during a live orchestrator session on 2026-08-10, auto-mode confirmed `false` for that run (`workflow._auto_chain_active = false`), option-a selected on all three items, options shown recorded verbatim — this is genuine human provenance, not an auto-approval artifact |
| 8 | ADR-0029's `## Trajectory` table runs unbroken `v0.1.0-rc.1` → `0.7.0`/`v0.7.1` with four new rows (v0.3.0/v0.4.0/v0.5.0/v0.6.0) in ascending order, no row re-sorted | ✓ VERIFIED | All four `CHANGELOG.md` anchors confirmed at the exact cited line numbers (`:596`, `:521`, `:444`, `:139`); all four tags/commits confirmed present (`v0.3.0-rc.1`, `v0.4.0`, `v0.5.0`, commits `90ca591`/`67b6207`/`23b187b` for the untagged v0.6.0); current `Cargo.toml:34` → `0.7.0`, `git tag --sort=-v:refname \| head -3` → `v0.7.1, v0.7.0, v0.5.1` — matches the corrected (not the originally-stale) figures |
| 9 | Zero `.rs` files modified across all 13 plans (D-19 boundary), checkable not merely claimed | ✓ VERIFIED | Independently re-run against current HEAD (not just the close-out's own snapshot): `git diff --name-only e12f18306ca9a80b1c3301e6afca31602e7c41ec..HEAD -- '*.rs' \| wc -l` → `0`; `git log --oneline <base>..HEAD --stat -- '*.rs'` → no output across 78 commits |

**Score:** 9/9 truths (grouped from the five ORCH requirements + roadmap Success Criteria 1-6) verified; 0 behavior-unverified. One item (#6/ORCH-03) carries a disclosed, independently-confirmed residue that is a legitimate scope-interpretation question routed to human verification rather than marked either FAILED or silently VERIFIED.

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.planning/ledgers/milestone-09-12.md` | 120 cited rows, 28 section headers, close-out amendment | ✓ VERIFIED | 120 rows, 120 distinct IDs, 28 `### ` headers (the plan's own acceptance-criteria said 29 — self-disclosed as a plan-authoring miscount in the close-out amendment, not a tree defect; independently confirmed 28 is the correct count) |
| `.planning/decisions/0037-agent-route-surface-v1.md` | 7 H2 headings, no frontmatter | ✓ VERIFIED | Confirmed |
| `.planning/decisions/0038-agent-provisioner-placement.md` | 7 H2 headings, no frontmatter | ✓ VERIFIED | Confirmed |
| `.planning/decisions/0039-http-topology-no-garrison-no-arsenal.md` | 7 H2 headings, no frontmatter | ✓ VERIFIED | Confirmed |
| ADR-0029 `## Trajectory` table amendment | 4 new rows, ascending order | ✓ VERIFIED | Confirmed, see truth #8 |
| `docs/src/deployment-topologies/sidecar.md` prose correction | `/v1`-prefixed route named | ✓ VERIFIED | Line 29-30 confirmed; embedded code example not corrected — see human-verification item |
| `docs/src/deployment-topologies/{http-service-host,overview}.md` Garrison/Arsenal-absence prose | states permanent limitation | ✓ VERIFIED (per 13-REVIEW.md, independently spot-checked for the AgentSpec field claim) | `AgentSpec` (`agent_registry.rs:55-79`) confirmed to have no Garrison/Arsenal field |
| REQUIREMENTS.md ORCH-01/02/03/04/05 `[x]` closing notes | dated, evidence-cited | ✓ VERIFIED | All five closing notes read and cross-checked against the artifacts they cite; all citations resolve |
| REQUIREMENTS.md Phase 14/15/16 hand-off blocks | name real, resolvable targets | ✓ VERIFIED | All cited ledger rows, ADR files, and code paths (e.g. `crates/doc-examples/src/sidecar.rs:25,34`, `crates/paladin-web/src/agent_controller.rs:723`, `openapi.rs:103`) confirmed to exist and say what the hand-off claims |
| All 9 `ADR-NNNN` citations in the ledger | resolve to existing files | ✓ VERIFIED | `0006, 0024, 0029, 0033, 0034, 0036, 0037, 0038, 0039` — all 9 exist in `.planning/decisions/` |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|---|---|---|---|---|
| ORCH-01 | 13-01…13-10, 13-13 | 120-row cited ledger, per-milestone corroboration, corrected arithmetic | ✓ SATISFIED | Ledger exists, 120/120, arithmetic corrected at source (REQUIREMENTS.md:2216-2235), close-out integrity re-check passed independently re-run |
| ORCH-02 | 13-01…13-07, 13-13 | Five open-checkbox verdicts, pattern recorded once | ✓ SATISFIED | All five verdicts confirmed against live tree (M10 SUPPLY-01, M12 Task-0.0 scaffolding via `agent_auth.rs`, project-management formatting example) |
| ORCH-03 | 13-08, 13-11, 13-13 | Route surface + 4 stale paths corrected at source | ✓ SATISFIED, with disclosed residue outside declared 5-item scope | See truth #6 and human-verification item |
| ORCH-04 | 13-09, 13-13 | Two seams get decisions (ADR-0038, ADR-0039), human-ratified | ✓ SATISFIED | Both ADRs exist, correct structure, genuine human checkpoint provenance confirmed |
| ORCH-05 | 13-10, 13-12, 13-13 | Version trajectory complete, numbering prediction closed | ✓ SATISFIED | Trajectory table confirmed accurate against `CHANGELOG.md`/tags/commits; ADR-0030 cited as the numbering-prediction closure, no rival ADR created (confirmed: only 0037-0039 authored this phase, `PROMOTION.md` "Next free ADR number" → 0040) |

No orphaned requirements: ROADMAP.md Phase 13 names exactly ORCH-01…05; REQUIREMENTS.md's traceability table (lines 4012-4016) lists exactly the same five, all "Complete."

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| Orchestrator lifecycle test the ledger cites | `cargo test --test lib orchestrator_workflow_lifecycle -- --nocapture` (implied by ledger; re-run as `cargo test -p paladin-web --test auth_rbac` and others below as representative samples) | see below | — |
| `auth_rbac.rs` 5-test claim | `cargo test -p paladin-web --test auth_rbac` | `5 passed; 0 failed` | ✓ PASS — matches ledger exactly |
| `event_trigger_pipeline.rs` 5-test claim | `cargo test --test event_trigger_pipeline` | `5 passed; 0 failed` | ✓ PASS — matches ledger exactly |
| `orchestrator_bridge::` 10-test claim | `cargo test --lib orchestrator_bridge::` | `10 passed; 0 failed` | ✓ PASS — matches ledger exactly |
| `mdbook build docs/` two-pre-existing-error claim | `mdbook build docs/` (after `mdbook-mermaid install docs/`) | exit `101`, identical two errors at `deployment/docker.md:118` and `user-guides/tool-integration.md:324`, no new errors | ✓ PASS — matches ledger's close-out claim exactly |
| Zero `.rs` diff claim | `git diff --name-only <base>..HEAD -- '*.rs' \| wc -l` | `0` | ✓ PASS |
| `cargo tree -p paladin-web -e normal` no-facade claim | `cargo tree -p paladin-web -e normal \| grep paladin-ai\b` | only `paladin-ai-core`, no bare `paladin-ai` facade | ✓ PASS |

### Anti-Patterns / Findings

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `.planning/ledgers/milestone-09-12.md` | `REQ-opaque-bearer-token-adapter-v1` row (Milestone 9 Epic 5) | Cited command `grep -rln "impl AuthPort for" src/ crates/` is stated to "return exactly this one file" (`in_memory_token_auth_adapter.rs`); independently re-run, it returns **4 files** — the production adapter plus three `#[cfg(test)]`-scoped mock implementations in `auth_rbac.rs`, `agent_auth.rs`, and `auth_middleware.rs` | Info | The underlying verdict (only one *production* `AuthPort` implementation ships) remains correct on inspection of the three extra hits — all three are test-only mocks inside `#[cfg(test)] mod tests`. But the cited command's literal output does not match what the row claims it returns, which is exactly the self-referential-inaccuracy class this phase exists to eliminate. Narrow, single-row, does not change any verdict class. Not a blocker; noted for completeness given the row's own stated evidence bar ("a passing test, example, or command exercising it") implies exact reproducibility |
| `13-VALIDATION.md` | trajectory-rows through no-pending-stubs rows | Rows still show `⬜ pending` status | Info | `13-VALIDATION.md` is a pre-execution Nyquist planning artifact, not updated post-execution — its checklist items were independently re-verified directly against the tree in this report (all pass); the stale `⬜ pending` markers in that file are cosmetic, not a completion-tracking gap |
| `.rs` boundary | n/a | `crates/doc-examples/src/sidecar.rs` unmodified by this phase, confirmed via `git log --follow` | — | Confirms the D-19 boundary and the CR-01 residue's stated cause (fixing it requires the one `.rs` file this phase would have had to touch) |

No `TBD`/`FIXME`/`XXX` markers were found inside any file this phase actually wrote or amended (the ledger, the three ADRs, the three `docs/src/deployment-topologies/` pages, or REQUIREMENTS.md's Phase 13 sections). The `TBD` hits in `ROADMAP.md` are pre-existing boilerplate for future phases 14-16 (`**Plans**: TBD`), not phase 13 content.

## ORCH-03 / CR-01 Explicit Verdict (per verification-request item 1)

**Question:** Does the CR-01 deferral leave ORCH-03 honestly closable?

**Finding:** The residue is real. Independently confirmed: `crates/doc-examples/src/sidecar.rs:34` builds `.post(format!("{base_url}/agents/{agent}/execute"))` — the unprefixed form — while the page that embeds it via `{{#include}}` states the correct `/v1`-prefixed route two lines above in prose. A reader who copies the rendered example gets a client that 404s against the live server (`API_V1_PREFIX = "/v1"` at `agent_controller.rs:723`, asserted by the `spec_paths_are_versioned_under_v1` test at `openapi.rs:103`).

**Reasoning:**

- ORCH-03's prose does read as a totalizing claim ("anyone applying a run-5 requirement literally cannot write to a path that does not exist"), and read that literally, the residue violates it.
- But ORCH-03 itself scopes its own work to "five specific items," (a)-(e), enumerated explicitly, and all five are independently confirmed corrected at source. The `crates/doc-examples/src/sidecar.rs` file was never one of the five named items.
- ROADMAP.md's own Phase 13 Success Criterion 4 — the authoritative roadmap contract for this exact requirement — uses narrower, matching language: "the four stale module and document paths are corrected at source, and the agent API's route surface has one answer confirmed against the committed `openapi.json`." Both halves are independently confirmed true. The criterion does not say "and every embedded code example agrees," and the *route surface's answer* (ADR-0037, `/v1`) is not in dispute anywhere — only one stale example still uses the old form.
- The residue was found by a post-execution code review (13-REVIEW.md CR-01), not swept under the rug: it is disclosed with exact file:line evidence, a working fix, and an explicit reason it wasn't fixed in-phase (the fix requires a `.rs` edit, which the phase's own explicitly-verified D-19 boundary forbids), and it is hand-off item 6 to Phase 14 with a concrete implementation instruction and an assertion-based regression suggestion.
- This is the opposite of the failure mode ORCH-03 exists to prevent: nothing here misrepresents the tree's state. The ledger and REQUIREMENTS.md both say plainly that this specific site is uncorrected and why.

**Verdict:** DEFENSIBLE AS DOCUMENTED. Marking ORCH-03 `[x]` is a reasonable reading of its own declared scope and the roadmap's parallel success criterion, given the residue is honestly disclosed rather than hidden, and reopening ORCH-03 would either force a D-19 boundary breach or force the same finding to be re-discovered and re-recorded with no different outcome. This is not, however, a call this verifier will unilaterally rubber-stamp as fully green — it is placed in Human Verification below for explicit ratification, because the literal-vs-scoped reading of the done-when text is a genuine interpretive question, not a fact.

## Human Verification Required

1. **Ratify the CR-01 / ORCH-03 scope trade-off.** See frontmatter `human_verification` entry above for full detail. Verifier's recommendation: accept as documented (do not reopen ORCH-03; confirm Phase 14 hand-off item 6 is the fix vehicle).

## Gaps Summary

No blocking gaps. All five ORCH requirements' `[x]` flips are backed by real, independently re-derivable evidence — every ADR file, ledger row sample, cited test, cited command output, and hand-off target checked in this verification reproduced exactly what REQUIREMENTS.md and the ledger claim. The zero-`.rs` boundary independently re-measured and held. The single narrow finding (an "exactly one file" grep-result overclaim on one ledger row, still correct in substance) and the CR-01 residue (disclosed, scoped outside ORCH-03's five named items, forwarded with a concrete fix) are the only imperfections found across an unusually large and rigorous verification sample. The one open item is a scope-interpretation ratification, not a factual failure — routed to human verification per the escalation-gate pattern rather than either silently passed or wrongly failed.

---

_Verified: 2026-08-10T21:44:37Z_
_Verifier: Claude (gsd-verifier)_
