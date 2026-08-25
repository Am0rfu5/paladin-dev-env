---
phase: 18-rust-sast-evaluate-and-adopt-codeql
plan: 03
subsystem: infra
tags: [codeql, sast, rust, security-evaluation, ci, code-scanning]

requires:
  - phase: 18-rust-sast-evaluate-and-adopt-codeql
    provides: .github/workflows/codeql.yml (proven end-to-end), scripts/codeql-analysed-files.sh, fixtures/codeql-probe/ (four-class probe, D-07..D-11)
provides:
  - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md — final SAST-01 verdict (disqualified, version-scoped CodeQL 2.26.3 / rust-queries 0.1.40), full four-measurement evidence chain, alert #28 correction, and the workspace-member confound test's complete execution-and-safety record
  - fixtures/codeql-probe/ — rewritten twice beyond its original D-07..D-11 four-class shape (rule-aligned remote-source redesign, then diagnostic unwrap/concatenation variant), now five classes (SQL injection, path traversal, hardcoded credential, regex injection, D-12 feature-gated SQL variant) plus one documented-but-unscored known gap (command injection, no upstream CWE-078 Rust query)
  - Empirical settlement of D-12 (feature-gated code coverage): buildless extraction always archives feature-gated files; semantic taint analysis of a `#[cfg(feature = "...")]`-gated module is skipped unless that feature is active for the scan — confirmed identically across three independent runs
  - Proof (workspace-member confound test) that CodeQL's null result on SQL injection/path traversal/regex injection is a genuine detection gap at this version, not an artifact of the probe fixture's workspace exclusion
affects: [18-04, 18-05, 18-06, 18-07]

tech-stack:
  added: []
  patterns:
    - "Pre-registered criteria before every measurement, at every iteration (T-18-13 discipline) — Promotion Criteria, Re-Probe Criteria, Diagnostic Iteration criteria, and Workspace-Member Confound Test criteria were each written and committed before the run that would produce the numbers they judge, so no threshold was ever retrofitted to a result"
    - "Throwaway-branch confound testing with a hard safety invariant: vulnerable experimental code committed on a disposable local branch (never the mergeable worktree branch), pushed only to the throwaway eval branch, then discarded — verified absent from the mergeable branch's history via an empty `git diff --stat` against a path-scoped baseline"
    - "Coverage-before-findings discipline applied recursively: every one of the five CodeQL runs in this plan re-verified `probe_fixture_entries > 0` (or the confound file's presence in the denominator-scoped set) before any finding count was interpreted, carrying forward 18-01's original 'analysed nothing vs. found nothing' distinction into every subsequent measurement"

key-files:
  created: []
  modified:
    - .planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md
    - fixtures/codeql-probe/Cargo.toml
    - fixtures/codeql-probe/Cargo.lock
    - fixtures/codeql-probe/src/lib.rs
    - fixtures/codeql-probe/src/sql_injection.rs
    - fixtures/codeql-probe/src/path_traversal.rs
    - fixtures/codeql-probe/src/credential.rs
    - fixtures/codeql-probe/src/command_injection.rs
    - fixtures/codeql-probe/src/feature_gated.rs
  created_then_kept:
    - fixtures/codeql-probe/src/regex_injection.rs

key-decisions:
  - "SAST-01 verdict: disqualified, version-scoped to CodeQL 2.26.3 / rust-queries 0.1.40 — decided by the user at the fourth of four checkpoints on this plan, after four independent measurements (original fixture, redesigned fixture, diagnostic iteration, workspace-member confound) all produced the same null result for SQL injection, path traversal and regex injection"
  - "codeql.yml is retained as a non-required advisory scan, not deleted — it reliably detects the hardcoded-credential class, proves the extraction-to-alert pipeline end-to-end, and delivers 385/385 first-party coverage on every run without exception, closing off any recurrence of the Snyk 'analysed nothing' failure shape even though it does not qualify for merge-gating"
  - "The original four-class probe fixture (D-08, reused verbatim from the Snyk evaluation) was itself found instrument-invalid before any finding count could be trusted: every planted class was structurally incapable of firing any wired CodeQL Rust security-extended query — a fact established from the query source code itself (with upstream github/codeql citations), not inferred from the zero result. This is a user-authorized deviation from this plan's original three enumerated checkpoint options (qualified / qualified-with-coverage-gap / disqualified) to a fourth path: instrument-invalid, redesign, re-probe"
  - "Alert #28 (rust/hard-coded-cryptographic-value, user_service.rs:1582), recorded by 18-01's tracer run and initially characterised as a 'genuine first-party finding,' was corrected on direct source inspection: it fires on a literal test string inside a #[cfg(test)] mod tests block — a test-code false positive, not a leaked production credential. This does not retract the pipeline-proof it provided; it does retract the 'proves detection on real code' framing"
  - "Two further user-authorized diagnostic iterations beyond the original re-probe were run to rule out confounds before accepting the null result: (1) a diagnostic fixture variant isolating the `?`-operator and `format!`-macro-expansion defect (ruled out `?` as a cause; confirmed `format!` as compounding-not-causal for SQL injection); (2) a workspace-member confound test planting the identical classes inside the real, already-scanned `paladin` crate using its own resolved reqwest/sqlx/regex dependencies (0/3 fired, ruling out workspace exclusion as the explanation)"
  - "The workspace-member confound's vulnerable code was never committed to this plan's mergeable branch history at any point — created and committed on a separate throwaway local branch, pushed only to the disposable eval/codeql-probe branch, then discarded. Verified via `git diff --stat 04328647 HEAD -- ':!.planning' ':!fixtures/codeql-probe'` returning empty, confirming zero changes outside the evidence document and the fixture crate across this plan's entire mergeable history"
  - "18-06's required-check promotion work does not occur — its held-verdict branch applies. No ruleset write, no branch-protection.md count update, no new required-check context. 18-06's remaining scope is limited to its other named deliverable (correcting the ruleset re-application procedure documentation)"
  - "D-20's Semgrep contingency trigger condition (no qualifying Rust SAST promoted) is now met — recorded as a forward pointer for a future plan, not acted on here"

patterns-established:
  - "Query-source-level impossibility analysis as a first-class disqualifying-evidence category, distinct from and prior to a zero finding count: the per-class impossibility table (citing exact .qll/.model.yml paths and CodeQL's own upstream test files) is evidence that existed before the probe ever ran, and is what separates 'the tool doesn't work' from 'the probe couldn't have worked regardless'"
  - "Confound isolation via single-variable fixture rewrites: each iteration (redesign -> diagnostic -> workspace-member) changed exactly one candidate explanation while holding every other variable constant against the prior run, so each run's delta is directly attributable"

requirements-completed: [SAST-01]

coverage:
  - id: D1
    description: "SAST-01 settled with a recorded, version-scoped disqualified verdict resting on four independent measurements, each pre-registered before its own run, with the full per-class impossibility/diagnosis chain and the alert #28 correction all committed to 18-CODEQL-EVIDENCE.md"
    requirement: "SAST-01"
    verification:
      - kind: manual_procedural
        ref: "grep-based schema check: 18-CODEQL-EVIDENCE.md contains ## Probe Result, ## Baseline Comparison, ## Steady-State Exclusion, and a settled ## Verdict; python3 split-based check confirms the Verdict section names 'disqualified' and cites numeric evidence"
        status: pass
      - kind: integration
        ref: "bash scripts/check-workflow-triggers.sh (7 workflow files, 7 policy-table rows, all clauses pass)"
        status: pass
    human_judgment: false
  - id: D2
    description: "Workspace-member confound test proves the SQL-injection/path-traversal/regex-injection null result is a genuine CodeQL detection gap at this version, not an artifact of the probe fixture's workspace exclusion, while keeping the vulnerable experimental code fully out of this plan's mergeable branch history"
    requirement: "SAST-01"
    verification:
      - kind: integration
        ref: "git diff --stat 04328647 HEAD -- ':!.planning' ':!fixtures/codeql-probe' (empty output, confirming zero src/ changes in this plan's mergeable history)"
        status: pass
      - kind: manual_procedural
        ref: "grep -rn for the three confound function names across the repository tree (no match, exit 1); git log --all --oneline | grep -i THROWAWAY (no match, exit 1)"
        status: pass
    human_judgment: false
  - id: D3
    description: "codeql.yml's disposition (non-required, advisory-only, not pinned in any ruleset) matches the final verdict, and downstream plan 18-06's scope is correctly narrowed to its non-promotion deliverable"
    requirement: "SAST-01"
    verification: []
    human_judgment: true
    rationale: "Whether the advisory-only disposition and the 18-06 scope note are the correct downstream framing is a judgment call about how this verdict should propagate into later plans, not something a script can verify — the evidence document states the reasoning plainly for a human to confirm."

duration: ~3h46m elapsed (2026-08-25T16:49:10Z-2026-08-25T20:35:36Z); the large majority was waiting on GitHub Actions CodeQL runs (five dispatched scans, ~3-5 min each) and the pre-push hook's full workspace build/test/clippy suite (multiple pushes, ~10-15 min each on this environment's cold local cache) — active editing/analysis time was substantially less
completed: 2026-08-25
status: complete
---

# Phase 18 Plan 03: Settle the SAST-01 Verdict Summary

**CodeQL disqualified as a Rust SAST for SQL injection, path traversal and regex injection (version-scoped: CodeQL 2.26.3 / rust-queries 0.1.40) after four independent measurements all returned the same null result; retained as a non-required advisory scan for its one reliably-working class (hardcoded credentials) and its proven end-to-end pipeline.**

## Performance

- **Duration:** ~3h46m elapsed (2026-08-25T16:49:10Z – 2026-08-25T20:35:36Z). The plan's original three tasks (scan the probe, prove steady-state exclusion, settle the verdict) expanded into four checkpoint continuations at the user's explicit direction, each authorizing exactly one more diagnostic before the final decision. The majority of elapsed time was GitHub Actions run wall-clock and pre-push-hook build time, not active analysis.
- **Started:** 2026-08-25T16:49:10Z
- **Completed:** 2026-08-25T20:35:36Z
- **Tasks:** 3 planned tasks (Task 1, Task 2 executed as planned; Task 3 — settle the verdict — executed across four checkpoint rounds: original three-option checkpoint, then three user-authorized deviations)
- **Files modified:** 10 (1 evidence document across 8 commits; 9 fixture crate files across 2 commits — plus one throwaway commit on a discarded branch, never merged)

## Accomplishments

- **Task 1** (`9dbea4df`): Dispatched the original four-class probe (D-08's Snyk-era classes plus D-12's feature-gated fifth) against `eval/codeql-probe`, recorded the per-class result (zero findings across all five), and established coverage (`probe_fixture_entries=6`) before interpreting that zero.
- **Task 2** (`c65f2346`): Proved the steady-state scan path excludes the probe fixture on two independent signals (`probe_fixture_entries=0`, zero alerts under `fixtures/codeql-probe/`) — the exclusion held with no alert requiring the 18-04 governed-disposition path.
- **First checkpoint deviation** (`c08d3b4f`): The literal zero-finding result was not accepted as a disqualifying verdict without checking whether the probe itself could ever have worked. A per-class impossibility analysis against CodeQL's actual Rust query source (with upstream `github/codeql` citations) showed all five original classes were structurally incapable of firing — three lacked a recognised taint source under the default `remote` threat model, two targeted CWE-078, which has no upstream Rust query at all. Verdict recorded as `instrument-invalid`, not `disqualified`; a `## Re-Probe Criteria` section pre-registered the redesign before any new fixture code existed. Alert #28 (18-01's tracer finding) was corrected in the same commit: it is a test-code false positive, not a genuine credential leak, though it still proves the pipeline runs end-to-end.
- **Fixture redesign** (`f9bc44cb`): Five classes rewritten to start taint from `reqwest::blocking::get(...).text()`, a source CodeQL's own upstream tests actually recognise, replacing the original bare-function-parameter shape.
- **Re-probe result** (`b3b53a03`): 1 of 4 scoreable classes fired (hardcoded credential). SQL injection had a diagnosed extraction-level cause (`format!`-macro-expansion failure, 889 occurrences checkout-wide); path traversal and regex injection extracted cleanly with no diagnosed cause. D-12 refined: extraction reaches feature-gated files; semantic analysis of a `#[cfg(feature=...)]`-gated module does not, unless that feature is active for the scan.
- **Second checkpoint deviation — diagnostic iteration** (`c7e3bc84`, `5fb5b76a`): Every `?` on the taint path replaced with `.unwrap()`/`.unwrap_or_default()`, and `sql_injection.rs`'s `format!` replaced with string concatenation, isolating the `?`-operator hypothesis from the `format!` defect. Result: still 1/4. Hypothesis ruled out — `?` was not the (sole) cause. `format!` confirmed compounding, not solely causal, for SQL injection specifically.
- **Third checkpoint deviation — workspace-member confound test** (`d27d4c33`, `ccdefde7`): The three still-missing classes replanted inside the real, already-scanned `paladin` crate using its own resolved `reqwest`/`sqlx`/`regex` dependencies — no new dependency, identical shapes to the diagnostic variant. Result: 0/3 fired. The confound file was confirmed semantically analysed (unlike the fixture's `feature_gated.rs`), ruling out workspace exclusion as the explanation. **This experiment's vulnerable code was committed only on a throwaway local branch, pushed only to the disposable `eval/codeql-probe` branch, and never entered this plan's mergeable branch history** — verified via an empty `git diff --stat` against a path-scoped baseline.
- **Final verdict** (`cb3d85f9`): SAST-01 settled as `disqualified`, version-scoped to CodeQL `2.26.3` / `rust-queries` `0.1.40`. `codeql.yml` retained, non-required, advisory-only. What CodeQL delivered is recorded honestly alongside the disqualification: reliable credential-class detection, a proven pipeline, and unbroken 385/385 coverage — but the one real-code hit was itself a false positive.

## Task Commits

Each task/checkpoint round was committed atomically:

1. **Task 1: Scan the probe and record the per-class result** — `9dbea4df` (docs)
2. **Task 2: Prove the steady-state scan path excludes the fixture** — `c65f2346` (docs)
3. **Checkpoint deviation 1a: instrument-invalid verdict, alert #28 correction, Re-Probe Criteria** — `c08d3b4f` (docs)
4. **Checkpoint deviation 1b: fixture redesign (rule-aligned, remote-source)** — `f9bc44cb` (feat)
5. **Checkpoint deviation 1c: re-probe result** — `b3b53a03` (docs)
6. **Checkpoint deviation 2a: Diagnostic Iteration criteria + fixture rewrite** — `c7e3bc84` (docs)
7. **Checkpoint deviation 2b: diagnostic-iteration result** — `5fb5b76a` (docs)
8. **Checkpoint deviation 3a: Workspace-Member Confound Test pre-registration** — `d27d4c33` (docs)
9. **Checkpoint deviation 3b: confound result** — `ccdefde7` (docs)
10. **Final settling: SAST-01 verdict** — `cb3d85f9` (docs)

_No TDD tasks in this plan — all commits are `type="auto"` (Tasks 1–2) or checkpoint-deviation continuations, all evidence/documentation or fixture-crate work._

**Not in this list, by design:** the workspace-member confound experiment's vulnerable-code commit (`70efbe23` on the discarded `scratch-eval-workspace-member-throwaway` local branch). It was never part of this plan's mergeable history — see "Deviations from Plan" below for the full safety record.

## Files Created/Modified

- `.planning/phases/18-rust-sast-evaluate-and-adopt-codeql/18-CODEQL-EVIDENCE.md` — grew from a seeded schema (18-01) to the full evidence record: five scan runs' raw data, the per-class impossibility table with upstream citations, the alert #28 correction, three pre-registered criteria sections (Re-Probe, Diagnostic Iteration, Workspace-Member Confound), their corresponding result sections, and the final settled verdict
- `fixtures/codeql-probe/Cargo.toml` — added `reqwest` (blocking, rustls-tls) and `regex` at the same version lines the root workspace already declares
- `fixtures/codeql-probe/Cargo.lock` — regenerated for the new dependencies (standalone crate's own lockfile)
- `fixtures/codeql-probe/src/lib.rs` — module list and documentation updated through both redesigns
- `fixtures/codeql-probe/src/sql_injection.rs` — rewritten twice: bare-parameter → `reqwest` source + `format!`, then → `reqwest` source + string concatenation + `.unwrap()`
- `fixtures/codeql-probe/src/path_traversal.rs` — rewritten twice: bare-parameter → `reqwest` source + `?`, then → `reqwest` source + `.unwrap()`
- `fixtures/codeql-probe/src/regex_injection.rs` — new in the redesign, replacing one of the original's two untestable command-injection slots; later updated to the `.unwrap()` idiom
- `fixtures/codeql-probe/src/credential.rs` — rewritten once: `format!`-built header string → literal argument to a locally-defined `password`-named parameter (the rule's actual heuristic sink)
- `fixtures/codeql-probe/src/command_injection.rs` — header corrected to document its known-gap, unscored status (no upstream CWE-078 Rust query); sink shape unchanged
- `fixtures/codeql-probe/src/feature_gated.rs` — rewritten twice, mirroring `sql_injection.rs`'s two redesigns, keeping the D-12 feature gate unchanged

## Decisions Made

See `key-decisions` in the frontmatter above for the full list. The single most consequential decision: **the user, at four separate checkpoints, chose paths outside this plan's originally enumerated options** — `instrument-invalid → redesign` (not one of the plan's three: `qualified` / `qualified-with-coverage-gap` / `disqualified`), then two further authorized diagnostic iterations (the `?`/`format!`-isolating rewrite, and the workspace-member confound), before landing on the final `disqualified` verdict. Every deviation was presented neutrally at its checkpoint and selected by the user, never self-selected by the executor.

## Deviations from Plan

### User-Authorized Checkpoint Deviations (not auto-fixed — explicit user decisions)

**1. Fourth-path verdict: `instrument-invalid → redesign` instead of one of the plan's three enumerated options**
- **Found during:** Task 3's original `checkpoint:decision`
- **Issue:** The plan's three options (`qualified`, `qualified-with-coverage-gap`, `disqualified`) all assumed the probe itself was a valid instrument. Query-source-level analysis showed it was not — every planted class was structurally incapable of firing any wired query, a fact true before the probe ever ran.
- **Resolution:** User selected a fourth path at the checkpoint: record `instrument-invalid`, redesign the fixture, re-probe. Not self-selected by the executor.
- **Files affected:** `18-CODEQL-EVIDENCE.md` (`c08d3b4f`), `fixtures/codeql-probe/*` (`f9bc44cb`)
- **Verification:** Re-probe run (`32877178870`) confirmed coverage (`probe_fixture_entries=6`) before any finding count was interpreted.

**2. Diagnostic iteration: `?`-operator and `format!`-macro-expansion isolation**
- **Found during:** Second checkpoint, after the redesigned fixture still returned 1/4
- **Issue:** Two candidate confounds (the `?` operator, a diagnosed `format!`-macro-expansion defect) could have explained the null result as an artifact of fixture authoring rather than a genuine gap.
- **Resolution:** User authorized exactly one diagnostic iteration. Both defects removed in one change-set (isolating them per-class); result unchanged (still 1/4), ruling out `?` and confirming `format!` as compounding-not-causal for SQL injection.
- **Files affected:** `18-CODEQL-EVIDENCE.md` (`c7e3bc84`, `5fb5b76a`), `fixtures/codeql-probe/{sql_injection,path_traversal,regex_injection,feature_gated,lib}.rs` (`c7e3bc84`)
- **Verification:** Diagnostic run (`32890183115`) cross-checked against raw SARIF; extraction logs confirmed the `format!` warning no longer appeared for `sql_injection.rs`.

**3. Workspace-member confound test — vulnerable code kept fully out of the mergeable branch**
- **Found during:** Third checkpoint, after the diagnostic iteration also returned 1/4
- **Issue:** The remaining candidate explanation was that the probe fixture's workspace exclusion (a required design property, D-07/D-11) itself prevented external-crate path resolution from matching how steady-state code resolves the same dependencies.
- **Resolution:** User explicitly lifted the prior "no workspace-membership variant" scope guard for one final diagnostic. The three classes were planted inside the real `paladin` crate. **Hard safety invariant, followed exactly:** the vulnerable code was created and committed on a separate throwaway local branch (`scratch-eval-workspace-member-throwaway`), pushed only to the disposable `eval/codeql-probe` branch, then discarded — `git checkout` back to the mergeable branch, local scratch branch deleted, remote eval branch deleted. **Never committed to this plan's mergeable history at any point, not even transiently.**
- **Files affected (throwaway branch only, never merged):** `src/codeql_workspace_probe.rs` (created), `src/lib.rs` (temporary `mod` declaration) — both existed only on commit `70efbe23`, which is unreachable from any ref in this worktree's local repository after cleanup.
- **Verification (all run and recorded in `18-CODEQL-EVIDENCE.md`):**
  - `git diff --stat 04328647 HEAD -- ':!.planning' ':!fixtures/codeql-probe'` → empty output.
  - `test -f src/codeql_workspace_probe.rs` → absent.
  - `grep -rn "codeql_workspace_probe" src/lib.rs` → no match (exit 1).
  - `grep -rn` for all three confound function names across the repository tree → no match (exit 1).
  - `git log --all --oneline | grep -i THROWAWAY` → no match (exit 1).
  - Result: 0/3 classes fired as workspace members, ruling out the workspace-exclusion hypothesis and supporting the final "genuine tool gap" verdict.

**4. `fixtures/codeql-probe/*` as a deviation from this plan's `files_modified` frontmatter**
- **Found during:** Every checkpoint round from the first redesign onward
- **Issue:** The plan's frontmatter listed only `18-CODEQL-EVIDENCE.md` and `.github/codeql/codeql-config.yml` under `files_modified`. The fixture crate itself required rewriting twice as the checkpoint chain progressed.
- **Resolution:** User-approved at each checkpoint as part of authorizing the redesign/diagnostic/confound iterations. `.github/codeql/codeql-config.yml` was never touched (Task 2's steady-state exclusion held without adjustment).

---

**Total deviations:** 4 user-authorized checkpoint deviations (0 auto-fixed via Rules 1-3 — every deviation here required an explicit human decision at a `checkpoint:decision`, none was a bug fix, missing-critical addition, or blocking-issue resolution the executor could resolve unilaterally).
**Impact on plan:** Substantial expansion in scope and duration beyond the original three-task plan, entirely at the user's direction, in service of the plan's own stated purpose: producing an assurance record trustworthy enough that "the next person to ask does not repeat the evaluation blind" (REQUIREMENTS.md, SAST-01). No scope crept in without an explicit checkpoint authorizing it.

## Issues Encountered

- **A real, reproducible Rust compile bug was found and fixed during the workspace-member confound test**: `String::from(...) + &untrusted_username` (a `String + &String` expression) compiles cleanly in the fixture crate's isolated dependency graph but fails to resolve under this workspace's full `cargo clippy --workspace --all-targets --all-features` invocation. Root cause not fully isolated (likely a competing `Add` impl surfaced under the full feature union, possibly from the `smartstring` dependency mentioned in the compiler's own diagnostic), but the fix (`.as_str()` for an unambiguous `&str` conversion) is unambiguous and correct regardless of the exact cause. This was caught by the pre-push hook's `cargo clippy` step on the throwaway branch — the same verification discipline this repository's own hooks apply to every push — before it ever reached CodeQL, let alone any mergeable branch.
- **A brittleness in this evidence document's own inline cross-references broke the plan's own Task 3 automated `<verify>` command**: the verify script splits the document on the literal substring `` `## Verdict` `` to isolate the Verdict section, but five earlier passages in the document used that exact literal string as a backtick-wrapped cross-reference (e.g., "see `` `## Verdict` `` below"), causing the split to land on the first inline reference rather than the actual heading. Fixed by rewording those five instances to "the Verdict section" (prose, not a literal heading match) — re-verified against the plan's own `<verify>` command, which now correctly reports `verdict recorded: disqualified`.

## User Setup Required

None — no external service configuration required. `GH_TOKEN` (a temporary full-access PAT, per the orchestrator's environment notes) was already exported and sufficient for every `gh api`/`gh workflow run`/`git push` operation this plan needed.

## Next Phase Readiness

- **18-04** (governed alert-dismissal register): still fully in scope. Alert #28's correction (test-code false positive) is exactly the kind of triage this register exists to record with an owner, review date, scope and compensating control.
- **18-05** (observation window): still in scope for whatever value it provides, though the SAST-01 disqualification means its numbers will not feed a promotion decision — they may still inform the D-14/D-15 false-positive-rate tracking this plan's `## Alert #28 correction` already opened (1 triaged, 1 false positive).
- **18-06** (ruleset promotion): **does not occur.** Its held-verdict branch applies. No `.github/rulesets/protect-main-branch.json` write (44 stays 44), no `docs/src/appendix/branch-protection.md` count update, no new required-check context for `scripts/check-workflow-triggers.sh` to resolve. 18-06's remaining scope is narrowed to its other named deliverable: correcting the ruleset re-application procedure documentation, independent of this plan's outcome.
- **18-07** (SAST-04 close): the rewrite of `.github/instructions/security.instructions.md`'s "Known gap: no Rust SAST" section now has its complete, honest input — CodeQL was measured (not merely evaluated on vendor claims), found to reliably catch one class while missing three others across four independent confirmatory measurements, and is retained advisory-only rather than adopted as a gate. The manual credential-handling review remains the primary control, exactly as before this phase, but now with a real, if narrow, automated supplement.
- **Semgrep contingency (D-20):** its trigger condition ("no qualifying Rust SAST promoted") is now met. Recorded here as a forward pointer only — no Semgrep evaluation work was performed as part of this plan, and none was authorized.

## Threat Flags

None found beyond what this phase's own `<threat_model>` already registered (T-18-11 through T-18-15, T-18-SC) — every new trust-boundary-relevant surface this plan touched (the evidence document's own claims, the probe/steady-state scan split, the code-scanning alert store reads) was already covered by those threat entries, and every named mitigation (coverage-before-findings, two-signal D-12 answers, T-18-13's pre-registration discipline, T-18-14's governed-disposition-only posture) was followed throughout, including through all three checkpoint-authorized deviations. The workspace-member confound test's own novel surface (deliberately vulnerable code temporarily present in a real, shipped crate) was mitigated by the hard safety invariant recorded above and verified with concrete, reproducible commands, not merely asserted.

---
*Phase: 18-rust-sast-evaluate-and-adopt-codeql*
*Completed: 2026-08-25*
