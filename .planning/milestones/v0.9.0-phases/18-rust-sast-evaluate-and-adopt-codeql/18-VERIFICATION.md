---
phase: 18-rust-sast-evaluate-and-adopt-codeql
verified: 2026-08-25T23:30:00Z
status: passed
score: 8/8 must-haves verified (1 truth non-applicable by recorded decision, see note)
behavior_unverified: 0
overrides_applied: 0
overrides: []
---

# Phase 18: Rust SAST — Evaluate and Adopt CodeQL Verification Report

**Phase Goal:** This project stops having to say "there is no static taint analysis for
first-party Rust" — either because a scanner is running that provably finds real defects in
*this* tree, or because the evaluation proved it does not and that verdict is recorded with its
evidence. The failure mode this phase exists to prevent is the Snyk one: a scan that reads as
assurance while analysing nothing.

**Verified:** 2026-08-25
**Status:** passed
**Re-verification:** No — initial verification

## Verdict Context (read before the findings below)

This phase reached a **disqualified-advisory** outcome via a user-directed path with several
explicitly authorized deviations from the plan's literal branches. These are recorded decisions,
not shortfalls, and are verified as such below rather than flagged as failures:

- SAST-01 verdict: **disqualified**, version-scoped to CodeQL `2.26.3` / `rust-queries` `0.1.40`.
- `codeql.yml` retained, **advisory-only** — a user override of the plan's disqualified→remove
  branch.
- SAST-03 (promotion) is **satisfied-by-not-applicable**: the scanner was disqualified, so not
  promoting it is correct. 18-05 (observation window) and part of 18-06 (promotion decision) are
  correctly recorded not-applicable / hold-advisory, each with an explicit, evidenced rationale.
- SAST-04 is satisfied by 18-07's rewrite of every record that asserted the gap.

The evaluation's own record is unusually rigorous: it self-corrected an initial
"instrument-invalid" zero-finding read, re-designed the probe fixture around a `reqwest` remote
source, ran three further diagnostic/confound measurements to rule out artifact explanations, and
landed on a genuine, version-scoped detection gap (SQL injection / path traversal / regex
injection never fire; hardcoded-credential fires reliably with a 100% FP rate on its one
real-code sample). That is exactly the "evaluation proved it does not, and the verdict is
recorded with its evidence" branch the phase goal names as a pass condition.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A CodeQL run against this repository completes end-to-end (workflow fires, database built with `build-mode: none`, query suite runs, SARIF uploaded, alerts readable via the code-scanning API) | ✓ VERIFIED | `18-CODEQL-EVIDENCE.md` Run Log rows for run ids `32868842656`, `32877178870`, etc.; `results_count`/`rules_count` cross-checked against the `/code-scanning/analyses` endpoint |
| 2 | `codeql.yml` declares no `paths:`/`paths-ignore:` under any trigger — a PR touching zero `.rs` files still produces a check | ✓ VERIFIED | Read `.github/workflows/codeql.yml`: `on.pull_request` has only `branches:`, no path filter |
| 3 | `branching-model.md`'s push branch-filter cell and `codeql.yml`'s `on.push.branches` are byte-identical (`['**']`), `check-workflow-triggers.sh` exits 0 | ✓ VERIFIED | `bash scripts/check-workflow-triggers.sh` → exit 0, "7 workflow file(s) scanned... coverage, drift, context and reachability clauses all pass" |
| 4 | `codeql.yml` declares exactly one job named literally `CodeQL Analysis (Rust)`, no matrix, no expression | ✓ VERIFIED | `jobs.codeql.name: CodeQL Analysis (Rust)` — literal string, single job, no `strategy` key |
| 5 | The analysed-`.rs`-file count is obtainable as an integer from the run's own debug artifact, recorded beside the 385 denominator | ✓ VERIFIED | `scripts/codeql-analysed-files.sh` exists, reads `src.zip` inside the debug artifact; evidence doc records `analysed_rs_files=385`/`denominator=385` on every run, `386` on the confound run (`difference=-1`, explained) |
| 6 | The promotion-qualifying condition was written down before any measurement existed | ✓ VERIFIED | `## Promotion Criteria` section (line 25) precedes `## Run Log`/`## Probe Result` in the evidence doc's own structure and is dated before the 18-03/18-06 measurement runs |
| 7 | A probe result of exactly 0 across all four planted classes is a valid, published, non-retried outcome; per-class breakdown recorded either way | ✓ VERIFIED | `## Probe Result` and `## Verdict` per-class tables; the initial 0-finding read was investigated (not retried with a weakened probe) and reclassified `instrument-invalid`, then re-measured with a redesigned fixture to a final `1 of 4` result |
| 8 | The Rust-SAST record is rewritten to the measured outcome everywhere it was previously asserted (security instructions, CLAUDE.md, copilot-instructions.md, MILESTONES.md), Snyk prohibition intact | ✓ VERIFIED | All four files read directly; identical claim ("disqualified... `2.26.3`... advisory... does not gate a merge"), Snyk section byte-unchanged in `security.instructions.md` |
| 9 (backstop, N/A) | "A cancelled CodeQL run never satisfies the required-check gate... after promotion" | N/A — not applicable, consistent with 18-05/18-06's recorded promotion-not-pursued decision | This truth is scoped to *after promotion*; promotion was not pursued (disqualified verdict, hold-advisory, `## Open Item — Promotion Held`). Cancelled-run handling *is* empirically exercised as ordinary CI behavior — every cancelled run in the evidence doc's Run Log correctly shows `conclusion: cancelled` and contributes no metrics — but the specific claim ("merge button stays unavailable") cannot be exercised because there is no required check to gate a merge. Not a gap: it becomes testable only if/when a future re-evaluation promotes CodeQL, which the recorded `## Open Item` already conditions on a specific upstream changelog trigger. |

**Score:** 8/8 applicable truths verified. Truth 9 is a backstop truth whose precondition (promotion) the phase's own recorded decision correctly never reaches — treated as non-applicable rather than as a gap, on the same basis 18-05/18-06 already established for the rest of the promotion-dependent measurement track.

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.github/workflows/codeql.yml` | Advanced-setup CodeQL Rust job, advisory | ✓ VERIFIED | `build-mode: none`, single literal job name, no `continue-on-error`/error-suppressing flags, header comment states the measured disqualification plainly |
| `docs/src/contributing/branching-model.md` | Trigger-policy register row for `codeql.yml` | ✓ VERIFIED | Row present, states disqualified/advisory outcome, matches workflow's actual trigger surface |
| `scripts/codeql-analysed-files.sh` | Analysed-file-count extraction from `src.zip` | ✓ VERIFIED | Present, executable, documented mechanism (nested `db-rust.zip` → `src.zip`) |
| `.planning/phases/.../18-CODEQL-EVIDENCE.md` | Run log, coverage, promotion criteria, verdict | ✓ VERIFIED | 91KB document; contains `## Promotion Criteria`, `## Probe Result`, `## Verdict`, `## Open Item — Promotion Held`, `## Observation Window` (not-applicable), four independent measurement rounds |
| `fixtures/codeql-probe/` (5 defect files + `lib.rs`) | Standalone, workspace-excluded probe fixture | ✓ VERIFIED | `Cargo.toml` has `publish = false`; root `Cargo.toml` `exclude = ["fixtures/codeql-probe"]`; `cargo metadata --no-deps` does not list it |
| `.github/codeql/codeql-config.yml` / `codeql-config-probe.yml` | Steady-state vs. probe-mode analysis scope | ✓ VERIFIED | Steady-state excludes fixture via `paths-ignore`; probe-mode has `paths-ignore: []`; selected via `scan_probe_fixture` dispatch input |
| `CODEQL-DISMISSALS.md` | Governed dismissal register | ✓ VERIFIED | 1 structured `[[dismissal]]` block, all required fields present, declared count matches row count |
| `scripts/check-codeql-dismissals.sh` + `tests/scripts/check-codeql-dismissals_test.sh` | Offline guard + fail-first regression harness | ✓ VERIFIED | Both run: guard exits 0 against the real register; test harness passes all 11 assertions including 6 fire-cases (missing owner, past review date, count mismatch, duplicate alert, missing file, orphaned path) |
| `.github/rulesets/protect-main-branch.json` | 44 required checks, no CodeQL entry | ✓ VERIFIED | Parsed JSON: exactly 44 contexts, none named `CodeQL Analysis (Rust)`; live ruleset `20868126` confirmed via `gh api` also reports 44 |
| `.github/instructions/security.instructions.md` | Rewritten "Known gap" section | ✓ VERIFIED | New dated section states measured outcome, cites evidence doc, Snyk section untouched |
| `.planning/MILESTONES.md` | Known Gap entry updated | ✓ VERIFIED | "No merge-gating Rust SAST — settled by Phase 18" entry present with owner/revisit date |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `codeql.yml` | `branching-model.md` | trigger-policy register row | ✓ WIRED | `check-workflow-triggers.sh` Clause 1/2 pass (exit 0) |
| `codeql-analysed-files.sh` | `codeql.yml` | reads `debug: true` artifact | ✓ WIRED | `init` step has `debug: true`; script's documented extraction path matches |
| `Cargo.toml` | `fixtures/codeql-probe/Cargo.toml` | workspace `exclude` | ✓ WIRED | `exclude = ["fixtures/codeql-probe"]` present, `cargo metadata --no-deps` confirms exclusion |
| `Makefile`/`ci.yml` | `check-codeql-dismissals.sh` | `check-gates` target / License & Dependency Policy job | ✓ WIRED | Both reference the script; `make check-codeql-dismissals` and the standalone script both exit 0 |
| `security.instructions.md` | `18-CODEQL-EVIDENCE.md` | coverage claims cite the evidence doc | ✓ WIRED | Direct path reference present in the rewritten section |
| `CLAUDE.md` / `copilot-instructions.md` | `security.instructions.md` | restated security bullet | ✓ WIRED | Both state the identical disqualified/advisory/does-not-gate claim |
| `.github/rulesets/protect-main-branch.json` | `codeql.yml` | pinned context ↔ job display name | N/A (correctly unwired) | No pin exists because promotion was not pursued — this is the correct state, not a broken link |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| Dismissal register guard is internally consistent | `bash scripts/check-codeql-dismissals.sh` | exit 0, "1 governed dismissal(s) checked... all pass" | ✓ PASS |
| Dismissal guard is fail-first proven | `bash tests/scripts/check-codeql-dismissals_test.sh` | exit 0, 11/11 assertions pass (6 fire-cases + 5 silent/idempotent cases) | ✓ PASS |
| Workflow trigger-policy guard passes with the new `codeql.yml` row | `bash scripts/check-workflow-triggers.sh` | exit 0, "7 workflow file(s) scanned... all pass" | ✓ PASS |
| Live ruleset matches committed JSON (44, no CodeQL) | `gh api /repos/DF3NDR/paladin-dev-env/rulesets/20868126 --jq '...length'` | `44` | ✓ PASS |
| Probe fixture excluded from workspace build graph | `cargo metadata --no-deps` (grep for `codeql`) | no match | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan(s) | Description | Status | Evidence |
|-------------|-----------------|--------------|--------|----------|
| SAST-01 | 18-01, 18-02, 18-03 | Candidate SAST measured against a deliberate-vulnerability probe, verdict recorded either way | ✓ SATISFIED | `18-CODEQL-EVIDENCE.md` `## Verdict`: disqualified, version-scoped, four independent measurements |
| SAST-02 | 18-01 | If qualified, runs on every PR with no path filter | ✓ SATISFIED (by construction, independent of qualification) | `codeql.yml`'s `pull_request` trigger has no `paths:`/`paths-ignore:`; built in 18-01 before the verdict was known and left unchanged after |
| SAST-03 | 18-04, 18-05, 18-06 | Runs non-blocking first, promoted only on measured behaviour | ✓ SATISFIED-BY-NOT-APPLICABLE | Scanner disqualified → not promoting is the correct action; `18-05` records the observation window as not-applicable, `18-06` records the promotion decision as hold-advisory with a named, owned, dated open item; ruleset stays at 44 checks |
| SAST-04 | 18-07 | "Known gap: no Rust SAST" section rewritten to the measured outcome, narrowed not deleted | ✓ SATISFIED | `security.instructions.md`, `CLAUDE.md`, `.github/copilot-instructions.md`, `.planning/MILESTONES.md` all rewritten in agreement; Snyk prohibition preserved verbatim |

No orphaned requirements: all four `SAST-*` IDs declared in `.planning/REQUIREMENTS.md` are claimed by exactly one or more plans (`SAST-01`: 18-01/02/03; `SAST-02`: 18-01; `SAST-03`: 18-04/05/06; `SAST-04`: 18-07), and every plan's `requirements:` frontmatter field maps to an ID that exists in `REQUIREMENTS.md`.

**Note — REQUIREMENTS.md ledger bookkeeping not yet updated.** `.planning/REQUIREMENTS.md`'s checkboxes for `SAST-01`..`04` are still `[ ]` and its Traceability table still reads "Pending" for all four, and `.planning/STATE.md`/`.planning/ROADMAP.md` still show Phase 18 as "EXECUTING"/"planned, execution not started." This is consistent with 18-07-SUMMARY.md's explicit note that STATE.md/ROADMAP.md updates were deliberately left to the orchestrator's seal step in this worktree-mode run ("execute-plan auto-skips STATE.md in worktree mode"), and REQUIREMENTS.md checkbox/traceability updates follow the same seal-time pattern observed in the archived `v0.8.0-REQUIREMENTS.md` (checkboxes flip to `[x]` and rows to a completion status at milestone/phase close, not mid-verification). Not treated as a gap against the phase's technical goal — it is a bookkeeping step that follows a passed verification, not a precondition for one. Flagging here so the orchestrator applies it when sealing the phase.

### Anti-Patterns Found

None. Scanned all phase-modified/created files (`codeql.yml`, the five fixture defect files + `lib.rs`, `codeql-analysed-files.sh`, `check-codeql-dismissals.sh`, `CODEQL-DISMISSALS.md`, `security.instructions.md`, `protect-main-branch.json`, `branch-protection.md`) for `TBD`/`FIXME`/`XXX`/`TODO`/`HACK`/`PLACEHOLDER`/"not yet implemented" — zero matches. No `continue-on-error` or other error-suppressing step in `codeql.yml` (the evidence doc explicitly contrasts this with `ci.yml`'s `osv-scanner` job, which does suppress).

### Human Verification Required

None. All must-haves resolved to VERIFIED or SATISFIED-BY-NOT-APPLICABLE with direct codebase/API evidence; no visual, real-time, or external-service behavior needed human judgment beyond the judgment-tier prohibitions below (all independently corroborated by evidence, not taken on SUMMARY claims alone).

**Judgment-tier prohibitions reviewed (non-authoritative LLM judgment, corroborated by direct evidence):**

- "No mechanism may make a green CodeQL result mean less than it says" — confirmed: no `continue-on-error`/error-suppressing flag in `codeql.yml`; non-blocking achieved solely by ruleset omission.
- "The probe must not be weakened... to make a scanner pass" — confirmed: the probe was *strengthened* (redesigned around a `reqwest` remote source) after an initial read was judged instrument-invalid, and the final verdict is a disqualification, not a pass being engineered.
- "The probe fixture must never be reachable from a real build" — confirmed: `exclude` in root `Cargo.toml`, `publish = false`, `cargo metadata --no-deps` excludes it.
- "A zero-finding result must not be treated as a failure/retried with a weakened probe" — confirmed: the zero-finding read was investigated and reclassified via root-cause analysis of the query source, not silently retried.
- "No existing gate may be weakened to accommodate this one" — confirmed: ruleset stays at 44 checks, unchanged.
- "Documentation must not claim coverage the probe did not demonstrate" / "Snyk prohibition must not be softened" — confirmed: rewritten sections cite exact measured numbers; Snyk section byte-identical.

### Gaps Summary

No gaps found. The phase delivers exactly the outcome its goal names: a static-analysis
evaluation was run against real code in this tree, with primary evidence (run IDs, alert IDs,
per-class tables, four independent measurement rounds including a self-correction), and reached a
disqualifying, version-scoped verdict that is recorded rather than assumed. The Snyk failure mode
(a scan that reads as assurance while analysing nothing) is explicitly and measurably ruled out —
385/385 file coverage is proven independently of the finding count on every single run, which is
the exact distinction the Snyk evaluation blurred. The advisory-retained `codeql.yml` genuinely
fails when it fails (no suppression), the one class it does detect is governed through a
schema-checked, fail-first-tested dismissal register, and every document that previously asserted
"no Rust SAST" now states the measured, dated, cited outcome in agreement.

The only outstanding item is bookkeeping (REQUIREMENTS.md checkboxes/traceability, STATE.md,
ROADMAP.md), which the phase's own SUMMARY documents as intentionally deferred to the
orchestrator's seal step — not a gap in the phase's delivered goal.

---

_Verified: 2026-08-25_
_Verifier: Claude (gsd-verifier)_
