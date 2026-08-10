# Milestone 9-12 cited status ledger

This file **supersedes** `REQUIREMENTS.md`'s `## Milestone 9-12 as-shipped ledger` section (D-01,
Phase 13 plan 13-01). That section becomes a pointer to this file, exactly as plan 13-10 executes at
close-out. This is the **fifth and final sibling** in the series — `milestone-01.md`,
`milestone-02-03.md`, `milestone-04-06.md` and `milestone-07-08.md` (this last one Phase 10's
deliverable, and the closest analogue and structural template this file copies) each already name
`milestone-09-12.md` as the sibling that completes the series. The series is now complete: there is
no sixth name to forward.

**Primary key: the `REQ-*` requirement ID.** Outstanding task items are nested under the requirement
they belong to, not given their own identifiers — nesting them keeps this ledger joinable to
`REQUIREMENTS.md` and `ROADMAP.md` without inventing new IDs (D-00f). The same `file:line` citation
may legitimately appear in more than one row: two requirements describing the same shipped artefact
keep separate rows and separate verdicts, because the `REQ-*` ID is the primary key, not the
citation. Two rows are never merged because they cite the same artefact.

**Evidence bar.** No row gets a `Shipped` verdict without a `file:line` citation **plus** something
that exercises it — a passing test, example, or command. This bar applies to **all 120 rows below
without exception**, including the 53 rows an earlier ingest already marked with the bare status word
`Shipped` and the 35 it left as a bare `Verify`. An ingest status word **is** the bare "the code
exists" claim this bar exists to reject — it is re-derived, not carried forward, for every one of the
120 rows (D-03, D-00e). Milestone 9 Epic 1's six rows below are derived to this bar in full this
session; every other row is transcribed from the existing `REQUIREMENTS.md:3607-3931` draft, prefixed
`run-5 input (not yet re-derived):` for the 53+32 rows that already carry content, or a
`pending` marker naming its owning plan for the 35 rows that do not, per the interim-state contract
below.

## Verdict legend

The vocabulary is the run-5 eleven-class status key already written at `REQUIREMENTS.md:3632-3636`,
not the series' earlier seven-class vocabulary — re-keying all 120 rows onto the earlier set would be
churn with no reader benefit (D-02). `Verify` is retired below rather than mapped: it was never a
verdict, only a marker meaning "ORCH-01 owes one here," and no row in this ledger ships carrying it.

| Verdict | Meaning |
| --- | --- |
| `Shipped` | `file:line` citation **and** a named passing test, example, or command exercising it |
| `Shipped (relocated)` | The deliverable exists, but at a different path than the requirement names |
| `Shipped, superseded` | Shipped, and a later milestone deliberately replaced the behaviour |
| `Shipped, one acceptance criterion false` | Every artefact ships, and the requirement's own acceptance criterion is nonetheless false — both halves recorded, dated (D-05) |
| `Superseded by outcome` | Shipped code answers the requirement differently than the ingested document specified; implementing the requirement as written would undo shipped work — do not plan as written |
| `Verified open` | Confirmed absent from the tree, checked directly rather than inferred |
| `Variant` | The row is cross-referenced in the corpus's variant register; two or more requirements describe the same underlying implementation choice from different angles |
| `Contract diverges` | The shipped implementation's public contract differs from what the requirement specifies, as a deliberate, standing divergence |
| `Open defect → X` | A defect confirmed present, handed to downstream requirement `X` |
| `Provenance only` | The document's value is as an origin/provenance record; its scope was already ingested and verified elsewhere — do not double-count |
| `Verify` (retired) | Never a verdict — a marker meaning ORCH-01 owed a real verdict here. No row below carries it |

**Mapping onto the series' seven-class vocabulary** (`satisfied`, `present, unproven`,
`genuinely outstanding`, `relocated`, `superseded by outcome`, `deferred with register`, `diverged` —
`milestone-07-08.md`'s own legend), the way Phase 10's D-02 mapped four HARD-01 dispositions onto
seven:

| This ledger's class | Series equivalent |
| --- | --- |
| `Shipped` | `satisfied` |
| `Shipped (relocated)` | `relocated` |
| `Shipped, superseded` | `superseded by outcome` |
| `Shipped, one acceptance criterion false` | Dual-classed: `satisfied` for the deliverables, `diverged` for the false criterion — see the tie-break rule below |
| `Superseded by outcome` | `superseded by outcome` |
| `Verified open` | `genuinely outstanding` |
| `Variant` | No clean single equivalent — an annotation naming corpus-wide register membership, not a verdict on its own; folds into `diverged` when the variant reflects a deliberate implementation choice, or rides alongside whatever primary verdict class the row also carries |
| `Contract diverges` | `diverged` |
| `Open defect → X` | `genuinely outstanding` |
| `Provenance only` | No equivalent in the series' seven — the earlier ledgers never inherited a document whose entire evidentiary value is being the origin record for content ingested and verified elsewhere. Recorded here as a genuine gap in the mapping rather than forced into a poor fit |
| `Verify` | Not mapped — retired, never shipped as a verdict |

**Tie-break rule.** A row that qualifies for two classes at once states the split explicitly, following
`milestone-07-08.md`'s own precedent that `relocated` wins over `superseded by outcome` when both
apply (the mdbook relocations are this corpus's single largest false-gap generator, and collapsing the
class would destroy the moved-not-missing signal). This ledger has one recurring case beyond that:
`Shipped, one acceptance criterion false` is not resolved by picking one side — D-05 requires **both
halves**, dated, so the row's Verdict cell carries both classes rather than a single winner.

**Manifest carve-out.** Milestones 10 and 12 are structural/infrastructural in large part (CI
hardening and release automation; Web API deployment artefacts), so for those rows a manifest line, a
workflow job or a `Makefile` target **plus** a named consumer is the exercising artefact — a bare
`Cargo.toml` feature declaration or an unconsumed manifest line is `Verified open` or a `pending`
placeholder, never `Shipped` on its own (Phase 7's D-01, carried by D-03). The canonical job-name
reference every fan-out plan cites by name, rather than asserting "CI runs it," is the corrected
15-job `ci.yml` list in the next paragraph.

**Path-caveat / factual-anchor paragraphs**, each re-grepped fresh this session, not trusted from an
earlier document:

1. **The agent route surface is `/v1`.** `grep -n '"/v1/agents' crates/paladin-web/openapi.json`
   returns six paths, all prefixed: `/v1/agents` (`:17`), `/v1/agents/{id}` (`:148`),
   `/v1/agents/{id}/execute` (`:271`), `/v1/agents/{id}/execute/stream` (`:382`),
   `/v1/agents/{id}/jobs` (`:489`), `/v1/agents/{id}/jobs/{job_id}` (`:580`) (D-11).
2. **`.github/workflows/ci.yml` has 15 job ids, not the 14 `intel/code-verification.md:539-540`
   records.** `grep -nE '^  [a-z][a-z0-9-]*:$' .github/workflows/ci.yml`, re-run this session, returns
   `lint`(:21), `security-audit`(:61), `cargo-deny`(:81), `osv-scanner`(:126), `api-surface`(:155),
   `test`(:206), **`examples`**(:245), `crate-isolation`(:319), `integration-tests`(:374),
   `docker`(:494), **`kubernetes-smoke`**(:611), `e2e-tests`(:718), `benchmark`(:779),
   `benchmark-regression-signal`(:812), `publish-dry-run`(:898) — fifteen entries. `security` is gone
   (Phase 9, D-05) and `examples`/`kubernetes-smoke` are present without a corresponding run-5 entry.
   Handed to Phase 15 / PIPE-01 (D-08).
3. **The `.project/current-exports.txt` baseline exists and the script reads the dotted path.**
   `sed -n '1,10p' scripts/check-api-surface.sh` confirms `check-api-surface.sh:6`:
   `BASELINE="${1:-.project/current-exports.txt}"`, and `test -f .project/current-exports.txt` — 446 KB,
   present. Run-5 finding 8's "fails on every run" consequence clause is closed in the script; the
   *documentation* half (four Milestone 12 requirement texts still naming the undotted path) stays open
   and is recorded in those four rows below, not as a sixth ORCH-03 item (D-09).
4. **The tree is at `v0.7.0`/`v0.7.1`, not the `0.6.0`/`v0.5.1` ORCH-05's own text states.**
   `Cargo.toml:34` — `version = "0.7.0"`; `git tag --sort=-v:refname | head -8`, re-run this session,
   returns `v0.7.1, v0.7.0, v0.5.1, v0.5.0, v0.4.3, v0.4.2, v0.4.1, v0.4.0`; branch `release/v0.7.0`.
   The same defect class Phase 10's D-11 already corrected once in HARD-03, recurring one requirement
   later (D-18).

**Corrected arithmetic.** ORCH-01's own text ("sixteen entries already carry `settled-by` pointers…
the remaining 104 need the same treatment") counts two different populations. Re-run this session:
`grep -c "settled-by" .planning/REQUIREMENTS.md` → **10**, none inside the ledger region
(`sed -n '3607,3931p' .planning/REQUIREMENTS.md | grep -c "settled-by"` → **0**) — the sixteen are
variant-register entries (`intel/SYNTHESIS.md:335`, `:546`), not ledger rows. **All 120 rows need a
verdict.** The measured split, re-run this session against `REQUIREMENTS.md:3607-3931`:
`grep -c '^| REQ-'` → **120**; bare `Verify` rows → **35**; bare `Shipped` rows (`Shipped — ` plus the
`Shipped → ` arrow form) → **51 + 2 = 53**; the remainder, already carrying a richer verdict → **32**
(120 − 35 − 53). D-04's headline 35/53/32 figures are reconciled exactly (D-04).

**Per-milestone checkbox corroboration (D-10, ORCH-02).** The five verdicts, once, here, in milestone
order, none converted into a task, per `intel/task-completion-state.md` and
`code-verification.md:622-659`: **M9** — 0 open, corroborated, every Epic 1-5 deliverable present.
**M10** — 0 open, corroborated in artefacts, contradicted in one acceptance criterion (the row above).
**M11** — 26 open, the only genuinely open count in run 5 and the only one of all 542 items across 75
task lists that survives verification — all fourteen target files exist, but whether their *content*
is current is settleable only by reading them, carried to DOCS-01 in Phase 16. **M12** — 3 open, all
three Task 0.0 feature-branch scaffolding ("Create feature branch," checkout/create
`feature/m12-epic5-api-security-authorization`, "Confirm a clean baseline") while the Epic 5 code ships
as `crates/paladin-web/src/agent_auth.rs` — vacuous. **project-management** — 1 open, nonexistent: a
`- [ ] 1.1 Create template → - [x] 1.1 Create template` formatting example inside a template file.
Across five runs, the pattern is **understated → accurate → overstated → contradicted → vacuous**, and
the corpus position (`code-verification.md:647-659`) is that checkbox arithmetic is not a backlog.
This paragraph is the only home for the pattern in the corpus, so a sixth rediscovery has somewhere to
land.

**Precedence.** Every verdict in this file is resolved under one order: **ADR → shipped tree →
`.planning/codebase/` map → `intel/code-verification.md` → PRD → DOC → task-list checkbox** (D-00b). An
ADR that contradicts shipped code is an instruction to change the code. This phase's own material adds
one note the order otherwise leaves implicit: a published page under `docs/src/` sits between the tree
and the maps, because a reader treats it as the contract — which is why the `sidecar.md` and
`http-service-host.md` defects this phase's D-19 boundary corrects are **corrections**, not mere
annotations, unlike a `.project/` PRD nobody executes against.

**Inherited stale-citation inventory (D-07).** Phase 12's plan 12-01 measured **87 stale-citation hits
across 25 files** for the `ci.yml:389-406` citation, corrected **8 sites across four canonical
governance documents** (`REQUIREMENTS.md`, `PROJECT.md`, `ROADMAP.md`, `STATE.md`), and excluded the
rest by a stated class-by-class scoping rule recorded in `12-01-SUMMARY.md` §Grep Inventory (frozen
milestone snapshots, prior-phase records, closed ingest outputs, a closed ledger row, ADR-0024's own
self-annotated citation, Phase 9's own correction banners, and Phase 12's own files). Those exclusions
are deliberate historical record; re-running the grep and "fixing" them would undo a decision this
ledger inherits rather than re-derives.

**Provenance.** Plan 13-01 is the scaffold — wave 1, dated 2026-08-10. It writes every head-note
element above, all 29 section headers and all 120 row stubs, and fully derives Milestone 9 Epic 1's
six rows end-to-end, the only section no fan-out plan owns. Every other row's Verdict cell is replaced
**in place** by the fan-out plan named in the ledger-file contention table below — never inserted,
deleted, or reordered.

## Shipped, one acceptance criterion false — the single instance

Placed here, at the head of the file, so a reader never has to scan 120 rows to find it (D-02, D-05).

Milestone 10 is recorded 100% complete and ships every artefact, job, target and ruleset it promised —
**and** it failed one of its own acceptance criteria, M10 Epic 2 §8's "`audit.toml` and `deny.toml`
are the only places policy/exceptions are defined; no inline advisory-ignore flags remain in CI." Both
halves, dated: the criterion was false when ingested, and **as of 2026-08-08 it no longer is.** Phase 9
made it true (plan 09-06, commit `cb75b2b`, deleting the duplicate `security:` job at pre-deletion
`ci.yml:465-482`); Phase 12 promoted the fix to ADR-0036 and put `scripts/check-workflow-suppressions.sh`
behind it so it stays true. Re-verified independently this session: `grep -n "cargo audit --ignore"
.github/workflows/ci.yml` returns **nothing** — the deletion held. A row recording only the failure, or
only the fix, is wrong; this ledger's own `REQ-audit-toml-single-source` row (Milestone 10 Epic 2,
below) carries both.

| `REQ-*` ID | Both halves |
|---|---|
| `REQ-audit-toml-single-source` | **Failed** — the duplicate `security` job at pre-deletion `ci.yml:465-482` still passed `cargo audit --ignore RUSTSEC-...` flags after `security-audit` (`ci.yml:62-77`) had already been corrected, so two jobs with different names covered the same `Cargo.lock` with different policies. **Fixed, dated 2026-08-08** — commit `cb75b2b` (Phase 9, plan 09-06) deleted the duplicate job; ADR-0036 (Phase 12) records the single-source topology and `scripts/check-workflow-suppressions.sh` regression-guards it. `grep -n "cargo audit --ignore" .github/workflows/ci.yml` → zero matches, re-confirmed this session |

## Verified open — the Deferred-QA block

Placed here for the same reason — this is the highest-confidence forward-work signal in the corpus and
the direct input to Phases 14, 15 and 16, and a planner must not have to scan 120 rows to find it
(D-02). Every ID below carries the verdict `Verified open` in its epic section; listed here with its
owning downstream requirement so a planner never scans for them.

| `REQ-*` ID | Owning downstream requirement |
|---|---|
| `REQ-ci-cli-snapshot-job` | PIPE-01 (Phase 15) |
| `REQ-ci-bench-check-job` | PIPE-01 (Phase 15) |
| `REQ-ci-combined-coverage-job` | PIPE-02 (Phase 15) |
| `REQ-codecov-config-thresholds` | PIPE-02 (Phase 15) |
| `REQ-makefile-coverage-targets` | PIPE-03 (Phase 15) |
| `REQ-modernize-github-actions` | PIPE-04 (Phase 15) — partially open, the one Epic 25 item found closed |
| `REQ-contributing-coverage-docs` | PIPE-05 (Phase 15) |
| `REQ-arch-doc-modernization` | DOCS-02 (Phase 16) — verified open and hidden by a relocation |
| `REQ-rustdoc-zero-warnings` | DOCS-03 (Phase 16) — open, and the bar itself is contested |
| `REQ-public-api-doc-audit` | DOCS-03 (Phase 16) |
| `REQ-asciinema-demos` | DOCS-04 (Phase 16) |
| `REQ-llm-tool-calling-port` | WEB-04 (Phase 14) |
| `REQ-llm-tool-calling-adapters` | WEB-03 (the flag) / WEB-04 (the scope) (Phase 14) |
| `REQ-mock-infrastructure` | DEFER-01 |
| `REQ-user-service-test-coverage` | DEFER-02 |
| `REQ-listener-service-test-coverage` | DEFER-03 |
| `REQ-deferred-coverage-register` | DEFER-01/02/03 |
| `REQ-user-guides-rewrite` | DOCS-01 (Phase 16) — verified open (content), not in Deferred-QA but the same M11 signal |
| `REQ-deployment-operations-docs-update` | DOCS-01 (Phase 16) — verified open (content), not in Deferred-QA but the same M11 signal |

## Row order and amendment convention

Epic sections appear in `REQUIREMENTS.md`'s own order and are **never re-sorted**; a later plan
replaces a row's Verdict cell **in place** and never inserts, deletes, or reorders a row. Amendments
follow D-00d: edit in place, retain superseded text, date every amendment, never a separate
corrections file (D-00d).

## Ledger-file contention

The rule every ledger-writing plan in this phase obeys — disjoint section ranges fixed before any
fan-out plan runs, so parallel wave-2 writes never collide (D-23, T-13-05).

| Plan | Wave | Owns | May |
|---|---|---|---|
| 13-01 | 1 | the whole file | create head notes, legend, both highlight tables, all 29 section headings, all 120 row stubs; fully derive Milestone 9 Epic 1 |
| 13-02 | 2 | Milestone 9 Epics 2-6 | replace Verdict cells inside its own sections; nothing else |
| 13-03 | 2 | Milestone 10 Epics 1-5 | replace Verdict cells inside its own sections; nothing else |
| 13-04 | 2 | Milestone 11 Epics 1-2, 3, 4, 6 and 5 & 7 | replace Verdict cells inside its own sections; nothing else |
| 13-05 | 2 | Milestone 12 Epics 1-4 | replace Verdict cells inside its own sections; nothing else |
| 13-06 | 2 | Milestone 12 Epics 5-7 | replace Verdict cells inside its own sections; nothing else |
| 13-07 | 2 | Deferred-QA Epics 25, 26, 27, 28-29 and project-management | replace Verdict cells inside its own sections; nothing else |
| 13-13 | 4 | a dated close-out amendment section appended at the foot | append only |

Milestone 9 Epic 1's six rows are derived by plan 13-01 and are owned by no fan-out plan. The six wave-2
plans run in parallel over **disjoint, contiguous** section ranges and perform **cell replacement
only** — never row insertion, deletion or reordering — so their diffs are non-adjacent hunks in one
file and merge without conflict. `grep -c '^| REQ-'` reads `120` before and after every one of them.

For every row this plan does not derive, the Verdict cell reads a `pending` marker naming the owning
fan-out plan from the table above, or the row's existing text prefixed
`run-5 input (not yet re-derived):` when it already carries content. No cell is ever left blank.

### Milestone 9 Epic 1 — Orchestrator End-to-End Workflow Execution (6 IDs)

**Epic note.** This section is the tracer: derived end-to-end this session at the same evidence bar
every fan-out plan (13-02 through 13-07) must meet for its own sections. All three previously-bare
`Verify` rows below became real verdicts by reading `.planning/intel/requirements.md:4939-4992` for
each ID's acceptance criteria, then locating the implementing symbol and its exerciser with `grep -rn`
over `src/` and `crates/`. All three previously-`Shipped` rows had their citations re-run rather than
transcribed. `crates/paladin-core/src/platform/container/job.rs`, `.../task.rs`,
`src/application/services/orchestration/mod.rs`,
`crates/paladin-ports/src/output/workflow_repository_port.rs`,
`crates/paladin-storage/src/sqlite_workflow_repository.rs` and
`tests/integration/orchestrator_workflow_lifecycle_test.rs` were all re-read directly this session.
One command was actually run, not merely cited: `cargo test --test lib orchestrator_workflow_lifecycle
-- --nocapture` → `test integration::orchestrator_workflow_lifecycle_test::full_lifecycle_sequential_workflow_executes_in_order_to_completion
... ok` (`1 passed; 0 failed`).

| Requirement | Verdict |
|---|---|
| REQ-workflow-execution-loop | Shipped — re-confirmed this session: `src/application/services/orchestration/mod.rs:382` `pub async fn execute_workflow`, with `execute_workflow_inner` at `:403`. The draft ledger's own caveat that per-variant behaviour was "not individually re-checked" is discharged with evidence, not carried forward: **Sequential** (`:420-452`) threads job N's output into `OrchestrationContext` for job N+1 at `:429-433`, exercised by `test_execute_sequential_workflow_orders_and_threads_output` (`mod.rs:1521`) and its fail-fast stop by `test_execute_sequential_workflow_fail_fast_stops` (`:1557`, third job never runs after the second fails); **Parallel** (`:453-461`) runs `run_jobs_concurrently` and aggregates every outcome regardless of individual failure, exercised by `test_execute_parallel_workflow_aggregates_all` (`:1581`); **Custom/staged** (`:462-486`) runs each stage's jobs concurrently and gates the next stage on the current stage's completion, exercised by `test_execute_staged_workflow_orders_stages` (`:1609`, asserts stage-2's job position exceeds both stage-1 jobs'); **EventDriven** (`:487-490`) registers listeners via `create_workflow_listener` (`:887`, called at workflow-creation time, `:329`) rather than executing synchronously, matching the PRD's own "firing/matching validation is Epic 2" scope note. Internal state tracked via `WorkflowExecutionResult`/`JobOutcome`; no new public state API introduced, matching decision 4C. |
| REQ-taskservice-dispatch | Shipped — `crates/paladin-core/src/platform/container/job.rs:477-487` `execute_single_task` resolves `task.service_name` against the caller-supplied `HashMap<String, Box<dyn TaskService>>` and returns a typed `JobError::ServiceNotFound` (no `panic!`/`unwrap()`) when the service is absent, exercised by `test_unregistered_service_surfaces_typed_error` (`src/application/services/orchestration/mod.rs:1658-1682`, asserts the error message names the missing service). **Fail-fast** (`JobExecutionMode::Sequential`, `job.rs:239-245` `execute_sequential`, returns on first `Err`) is exercised end-to-end by `test_execute_sequential_workflow_fail_fast_stops` (`mod.rs:1557`). **Continue-on-error** (`JobExecutionMode::SequentialContinueOnError`, `job.rs:250-`) is exercised by `test_continue_on_error_job_runs_all_tasks` (`mod.rs:1685-1726`, both tasks attempted, the registered task still runs, terminal state reflects partial failure). Both paths reuse the single `Job::execute(&services)` dispatch mechanism (`job.rs:178`) — no parallel mechanism was invented. Retry, backoff and dead-letter are genuinely absent from this path, matching the PRD's explicit out-of-scope clause for this Epic. |
| REQ-default-task-services-real-logic | Shipped — all three placeholder `TaskService` implementations in `crates/paladin-core/src/platform/container/task.rs` carry real, observable behaviour, re-confirmed this session: `DataBackupService` (`:333-395`) writes a real artifact under `backup_path` via `tokio::fs::write`, exercised by `test_data_backup_service_writes_artifact` (`:750-768`, asserts the written file exists and `bytes_written > 0`) and rejects path traversal via `safe_relative_name` (`:309`), exercised by `test_data_backup_service_rejects_path_traversal` (`:770-786`, `../escape.json` → `Err(TaskError::ExecutionFailed(_))`); `ContentIndexingService` (`:399-`) builds and persists a term-frequency index, exercised by `test_content_indexing_service_builds_index` (asserts `terms_indexed == 3` and the index file exists); `EmailNotificationService` (`:535-596`) dispatches through an injectable `EmailSink` seam, exercised by `test_email_service_delivers_to_sink` (delivery recorded in the sink) and `test_email_service_propagates_sink_failure` (a forced sink failure surfaces as a typed `TaskError::ServiceUnavailable` — the "forced failure" acceptance criterion). `grep -n 'tokio::time::sleep' task.rs` (and a second grep for `simulate`) finds exactly one remaining `sleep`, at `:180`, inside the unrelated `Task::execute_simple()` helper ("for testing or simple tasks" per its own doc comment) — not any of the three named services; the sleep/`println!` scaffolding this requirement targets for removal is confirmed gone from all three. Open Question 3 (which production email transport `EmailNotificationService` should default to) remains unrecorded — carried forward as a stated limitation of this verdict, not a gap in the shipped code. |
| REQ-workflow-repository-port | Shipped — re-confirmed this session: `crates/paladin-ports/src/output/workflow_repository_port.rs:109` `pub trait WorkflowRepositoryPort`; `crates/paladin-storage/src/sqlite_workflow_repository.rs:20` `pub struct SqliteWorkflowRepository`, `:119` `impl WorkflowRepositoryPort for SqliteWorkflowRepository`, its own three tests (`:205-256` — save/load round-trip, upsert-overwrites-existing, list-incomplete-excludes-terminal) confirm the SQLite adapter directly. All queries are parameterised (`sqlx::query(...).bind(...)`, e.g. `:126`, `:156`, `:172` — no string-interpolated SQL anywhere in the file). `Orchestrator` holds `Option<Arc<dyn WorkflowRepositoryPort>>` (`src/application/services/orchestration/mod.rs:65`); constructing without one still works (`Orchestrator::new()`, used by the majority of this file's own tests); `execute_workflow_inner` persists state on every job terminal transition and on workflow terminal transition (`:414-449`). **Consumer caveat, found this session:** `grep -rln SqliteWorkflowRepository src/ crates/` (excluding the adapter's own file) returns nothing — no production binary or facade wires the SQLite adapter into a running `Orchestrator`; in-tree, the port is exercised only via the `FakeWorkflowRepository` test double (`mod.rs:1733`). Both the port and its SQLite adapter ship and both work, proven by the adapter's own direct database tests; a production consumer wiring them together does not yet exist — recorded as a limitation of this `Shipped` verdict rather than downgraded. |
| REQ-workflow-crash-recovery | Shipped — `crates/paladin-storage/src/sqlite_workflow_repository.rs` re-confirmed present this session (see the previous row). Epic 1 **Open Question 4**'s default placement (`paladin-storage` rather than the facade) is what shipped, recorded as resolved by outcome. Crash recovery itself is directly exercised: `test_crash_recovery_resumes_remaining_jobs_to_completion` (`src/application/services/orchestration/mod.rs:1800-1839`) persists a workflow with two of three jobs already in `completed_job_ids`, constructs a **fresh** `Orchestrator` on the same repository, calls `start()` (which internally calls `resume_incomplete_workflows()` at `:101`), and asserts only the outstanding third job runs (`result.job_outcomes.len() == 1`, its id matching the un-run job) and the workflow reaches terminal `Completed` with all three job ids recorded. `resume_incomplete_workflows` (`:534-555`) loads every incomplete record via `repository.list_incomplete()` and resumes from the last persisted position, matching FR-22/23 exactly. |
| REQ-workflow-lifecycle-integration-test | Shipped — `tests/integration/orchestrator_workflow_lifecycle_test.rs` (158 lines, re-read this session) is a deterministic full-lifecycle integration test under `tests/`: `RecordingService` (`:20-52`) is a mock `TaskService` with an observable side effect (a shared `Arc<Mutex<Vec<String>>>` log, not stdout); the test builds a 3-sequential-job workflow (`:92-111`), creates it, starts the orchestrator, calls `execute_workflow`, and asserts ordered execution via the log (`:141-150`), terminal `Completed` (`:125`), and retrievable per-job results (`:129-138`, `:152-157`) — no wall-clock reliance, no log scraping. Registered at `tests/integration/mod.rs:68` (`pub mod orchestrator_workflow_lifecycle_test;`), itself included from `tests/lib.rs:61` (`pub mod integration;`) — the `lib` integration-test binary that `cargo test --workspace` compiles and runs by default, no feature flag required. **Command actually run this session**, not merely cited: `cargo test --test lib orchestrator_workflow_lifecycle -- --nocapture` → `test integration::orchestrator_workflow_lifecycle_test::full_lifecycle_sequential_workflow_executes_in_order_to_completion ... ok` (`test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 696 filtered out`). |

### Milestone 9 Epic 2 — Scheduler, Queue & Event Operational Validation (5 IDs)

Owned by plan 13-02, fully derived this session — all five rows carry a fresh `file:line` citation
plus a command actually run against this tree, none transcribed from an earlier ingest.

| Requirement | Verdict |
|---|---|
| REQ-scheduler-tick-validation | Shipped — re-confirmed this session: `src/application/services/orchestration/scheduler.rs:34` `pub struct SchedulerOrchestrator`, `:193` `async fn check_and_execute_jobs`, `:366` `fn calculate_next_run`. Every acceptance clause is exercised by a named test in the same file: variant coverage by `test_calculate_next_run_all_variants` (`:916-947`, Interval/Daily/Weekly/Monthly → `Some`, future `Once` → `Some(exact)`, past `Once` → `None`, `OnStartup` → `None`); dispatch-and-bookkeeping by `test_tick_dispatches_due_job_and_updates_bookkeeping` (`:950-973`, one dispatch, `last_run` set, `run_count` +1, `next_run` recomputed); disabled-job skip by `test_tick_skips_disabled_job` (`:976-1001`, zero dispatches, `run_count` unchanged); `Once`-fires-once by `test_once_job_fires_once_then_not_again` (`:1004-1033`); near-future firing by `test_scheduler_fires_job_scheduled_in_near_future` (`:1036-`). **Command actually run this session:** `cargo test --lib scheduler::` → `test result: ok. 22 passed; 0 failed`. No clock abstraction or scheduler refactor was introduced — the tests force due-ness via mutable field assignment on `scheduled_jobs`, matching the requirement's own constraint |
| REQ-cron-adapter-validation | Shipped — re-confirmed this session: `src/infrastructure/adapters/scheduling/tokio_cron_adapter.rs:71` `pub struct TokioCronSchedulerAdapter`, `:103` `pub async fn new`. Live firing observed via a shared `AtomicUsize` counter incremented by the job closure: `test_cron_job_fires_on_schedule` (`:428-`), whose doc comment at `:426` states the UTC-evaluation assumption verbatim ("`tokio-cron-scheduler` evaluates cron expressions in **UTC**"), satisfying the DST-non-attempt clause. Invalid expression → `test_schedule_job_invalid_cron_returns_error` (`:319-`, asserts `SchedulerError::InvalidCronExpression`). Scheduling while not running → `test_schedule_job_when_not_running_returns_error` (`:311-`, asserts `SchedulerError::NotRunning`). Full `start → schedule → cancel/shutdown` lifecycle exercised across `test_start_sets_running` (`:273`), `test_schedule_job_returns_job_id` (`:297`), `test_cancel_job` (`:334`), `test_shutdown_when_not_running_returns_error` (`:290`). **Command actually run this session:** `cargo test --lib tokio_cron_adapter::` → `test result: ok. 14 passed; 0 failed` |
| REQ-queueport-contract-parity | Shipped — `tests/queue_port_contract.rs:81` `async fn assert_queue_port_contract<Q: QueuePortHarness>` is one reusable contract exercising `create_queue`/`enqueue`/`dequeue`/`start_processing`/`complete_processing`/`queue_length`/`health_check` (`:83-148`), run against the in-memory `QueueOrchestrator` unconditionally (`in_memory_queue_satisfies_queue_port_contract`, `:214-217`, always-on) and against `RedisQueueAdapter` behind `#[cfg(feature = "redis-queue")]` (`redis_contract` module, `:258-370`), whose `try_connect` (`:325-359`) tries `PALADIN_TEST_REDIS_PORT`, then `6380`, then `6379`, then skips gracefully with `eprintln!("skipping: …")` rather than failing — resolving Open Question 1 exactly as noted. No `testcontainers` dependency is used by this harness. **Command actually run this session:** `cargo test --test queue_port_contract --features redis-queue` → `test result: ok. 3 passed; 0 failed` (Redis unreachable in this environment, so the Redis test logged its skip line and passed, exercising the skip path itself; the in-memory contract ran for real) |
| REQ-queue-retry-dead-letter | Shipped — the same `assert_queue_port_contract` (`tests/queue_port_contract.rs:111-145`) drives `max_retries = 1` through the shared harness for both backends: first failure reports re-queue (`retry_results.first() == Some(&true)`), the second reports exhaustion (`retry_results.last() == Some(&false)`), and `queue_length` returns to `0` after dead-letter — for whichever backend the harness wraps. In-memory dead-letter-store parity is exercised directly by `in_memory_queue_preserves_dead_lettered_items` (`:222-252`), asserting `stats.failed_items == 1` after a `max_retries == 0` item is permanently failed — the minimal parity addition the requirement permits, no broader redesign. **Command actually run this session:** `cargo test --test queue_port_contract --features redis-queue` → `test result: ok. 3 passed; 0 failed` |
| REQ-event-trigger-job-pipeline | Shipped, **and it invalidates a later baseline** — re-confirmed this session with a dedicated top-level test file, `tests/event_trigger_pipeline.rs` (322 lines), not merely cited: `matching_event_creates_exactly_one_trigger` (`:152-176`), `non_matching_event_creates_no_trigger` (`:177-204`), `multiple_matching_listeners_fan_out_one_trigger_each` (`:205-238`, one trigger per matching listener), `rate_limit_caps_trigger_creation_within_window` (`:239-280`, throttled events create no excess triggers), `trigger_is_converted_to_job_and_executed` (`:281-`, drives the created trigger through the Epic 1 dispatch path via a `TaskService` test double). The glue is `Orchestrator::process_event()` (`src/application/services/orchestration/mod.rs:657`) draining `ListenerOrchestrator::get_next_trigger()` (`listener.rs:248`) into `execute_trigger()` (`mod.rs:680`) — no new listener subsystem was built, matching Open Question 2's resolution. **Command actually run this session:** `cargo test --test event_trigger_pipeline` → `test result: ok. 5 passed; 0 failed`. This is the work that makes Epic 29's 57.83% figure stale → DEFER-03 |

### Milestone 9 Epic 3 — Content Processing Pipeline (4 IDs)

Owned by plan 13-02.

| Requirement | Verdict |
|---|---|
| REQ-paladin-content-processor | Shipped — re-confirmed this session: `src/application/services/orchestration/processors/paladin_processor.rs:40` `pub struct PaladinContentProcessor`, `:97` `impl ContentProcessor for PaladinContentProcessor`, in the **root crate** beside the trait exactly as Epic 3 **Open Question 1** resolved (not `paladin-content`, which would create a circular dependency). Configurable `PromptTemplate` at `processors/mod.rs:62`. Two `OutputParsing` strategies: `RawText` (default) stores the response verbatim (`:123`, `json!({"enrichment": output})`), `Json` attempts a parse (`:124`). Malformed-JSON degradation is a unit test, never a panic: `json_strategy_malformed_response_yields_degraded_result` (`:240-`, asserts a degraded `Ok` result, not an `Err`). Depends only on `PaladinExecutionService`/`PaladinPort`/`LlmPort` — no concrete LLM adapter import in the file. **Command actually run this session:** `cargo test --lib processors::paladin_processor::` (part of `processors::`) → `test result: ok. 6 passed; 0 failed` (see the combined count on the sibling row below) |
| REQ-battalion-content-processor | Shipped — re-confirmed this session: `src/application/services/orchestration/processors/battalion_processor.rs:68` `pub struct BattalionContentProcessor`, `:116` `impl ContentProcessor for BattalionContentProcessor`. Formation (sequential) via `FormationExecutionService::execute()` at `:131-136`, exercised by `formation_runs_agents_sequentially_into_one_result` (`:290-323`, asserts agent 2 received agent 1's threaded output and the final output is the last stage's result). Phalanx (parallel) via `PhalanxExecutionService::execute()` at `:138-`, exercised by `phalanx_runs_agents_in_parallel_and_merges_outputs` (`:325-`, merges both analysts' outputs keyed by agent name into `result_data`). Pattern selection is a constructor-time enum (`BattalionPattern::Formation`/`Phalanx`, `:33-35`), metadata records both the pattern name and participating agents. Unit tests use a `MockPaladinPort` — no network. Open Question 5 (Maneuver-flow configuration) remains explicitly deferred, matching the requirement's own scope note. **Command actually run this session:** `cargo test --lib processors::` → `test result: ok. 6 passed; 0 failed` (4 paladin_processor + 2 battalion_processor tests) |
| REQ-content-processor-orchestrator-wiring | Shipped — re-confirmed this session: `src/application/services/orchestration/mod.rs:150` `pub async fn register_content_processor`, `:273` `pub async fn process_content`. Registration accepts `Box<dyn ContentProcessor>` by name; `process_content` dispatches within the existing session lifecycle and returns the processor's `ContentProcessingResult`, exercised by `test_content_processing` (`:1185-1206`, asserts `success`, populated `result_data`, matching `processor_name`). An unregistered name returns the typed error, exercised by `test_content_processing_with_nonexistent_processor` (`:1208-1228`, asserts `OrchestratorError::ProcessorNotFound("NonExistentProcessor")`). No new lifecycle machinery — the step reuses `Orchestrator::start_session`/session context threading already exercised by Epic 1. **Command actually run this session:** `cargo test --lib orchestration::mod::tests::test_content_processing` (both tests, part of the `orchestration::mod` suite) → both `ok` |
| REQ-content-ingestion-e2e-validation | Shipped — `tests/content_ingestion_pipeline.rs:19` gates the whole file behind `#![cfg(feature = "content-processing")]`. **Deterministic** path: `deterministic_local_fixture_ingestion_to_enrichment` (`:91-`) drives a local fixture through ingest → extract/aggregate → a `MockLlmAdapter`-backed agent (`:24`, `:67`) → enriched result, no network, asserting content id preserved, enrichment present, `success == true`. **Live** path: `live_http_fetch_and_real_llm_enrichment` (`:159-`) uses a real `HttpContentFetcher` (`:161`) and a real OpenAI-backed agent, marked `#[ignore]` with a doc comment stating the network/credential requirement (`:143-157`), so it never runs in default CI. **Command actually run this session:** `cargo test --test content_ingestion_pipeline --features content-processing` → `test result: ok. 1 passed; 0 failed; 1 ignored` (the ignored test is the live one, logged as `ignored, requires network access and OPENAI_API_KEY`) |

### Milestone 9 Epic 4 — Agent / Orchestrator Bridge (4 IDs)

Owned by plan 13-02. **Epic-level note (transcribed from the source ledger):** §6.1 is "the cleanest
ADR-shaped section anywhere in the corpus" — a four-criterion comparison table, a `(CHOSEN)` column
header, an explicit decision, and the rejected option preserved as a future non-breaking enhancement.
It is manifest-typed PRD, so it is an ADR candidate, not a locked decision. `PROMOTION.md`'s own Part B
inventory (entry 9) separately flags this PRD section — **not** covered by this phase's ADR-0037-0039
allocation per D-20 (see `13-RESEARCH.md` Common Pitfall 6); no fourth ADR is promoted by this plan.

| Requirement | Verdict |
|---|---|
| REQ-orchestrator-port | Shipped — re-confirmed this session: `crates/paladin-ports/src/output/orchestrator_port.rs:232` `pub trait OrchestratorPort: Send + Sync`, re-exported at `crates/paladin-ports/src/output/mod.rs:15`. Exactly four methods (`:235-251`): `schedule_job`, `queue_item`, `fire_event`, `send_notification`, each returning `Result<_, OrchestratorBridgeError>`. Request types `ScheduleJobRequest`/`QueueItemRequest`/`FireEventRequest`/`SendNotificationRequest` (`:71,82,91,102`) are plain serializable structs with no root-crate `Orchestrator` import in the file. `OrchestratorBridgeError` (`:124`) carries `ActionNotAllowed(String)`, `QuotaExceeded { .. }`, plus the other required variants, all root-crate errors stringified at the boundary |
| REQ-bridge-policy-guardrails | Shipped — re-confirmed this session: `crates/paladin-ports/src/output/orchestrator_port.rs:43` `pub enum BridgeAction`, `:159` `pub struct BridgePolicy` with `max_jobs_scheduled`/`max_items_queued`/`max_events_fired`/`max_notifications_sent` (`:161-164`), `:207` `impl Default for BridgePolicy` (all four caps at `3`, a conservative low-single-digit default). Enforcement is in the adapter, not the policy struct itself: `src/application/services/orchestration/orchestrator_bridge.rs:92` rejects a disallowed action with `ActionNotAllowed` and `:100` rejects a cap-exceeding action with `QuotaExceeded`, both **before** any underlying `Orchestrator` call. Counters are per-adapter-instance `AtomicU32` (`:41-44`, thread-safe), reset by constructing a fresh adapter per execution rather than a process-global — the adapter's own module doc (`:12-14`) states this explicitly. This is an allow-list plus caps, not RBAC, matching the requirement's own minimality clause. **Command actually run this session:** `cargo test --lib orchestrator_bridge::` → `test result: ok. 10 passed; 0 failed` (covers success, `ActionNotAllowed`, `QuotaExceeded` for all four actions). **Part B candidate 9 pointer:** `.project/Milestone_9-Classic-Orchestrator-Completion/Epic_4/prd-agent-orchestrator-bridge.md` §6.1 is `PROMOTION.md`'s own Part B inventory candidate 9 and is **not** promoted to a fourth ADR by this phase — D-20 allocates exactly ADR-0037-0039, and no ORCH requirement's `Derives` list reaches that PRD section. Plan 13-13's advancing note carries its final disposition; this pointer keeps the two documents from disagreeing silently in the meantime |
| REQ-orchestrator-bridge-adapter | Shipped — re-confirmed this session: `src/application/services/orchestration/orchestrator_bridge.rs:59` `pub struct OrchestratorBridgeAdapter` holding `orchestrator: Arc<Orchestrator>` (`:60`), `policy: BridgePolicy` (`:61`), and `notification: Option<Arc<dyn NotificationDeliveryPort>>` (`:62`) — its absence yields `ActionNotAllowed`/`InvalidRequest` for `send_notification`, exercised by `send_notification_without_port_fails` (`:448-`). Lives in the **root crate** exactly as FR-12 specifies, with the module doc (`:1-7`) stating the same lower-crate-cannot-depend-on-root-crate rationale as the Epic 3 processors. `:114` `impl OrchestratorPort for OrchestratorBridgeAdapter`: `schedule_job` (`:115`) builds a `Job` and calls `Orchestrator::schedule_job`; `queue_item` (`:132`) enqueues via the queue service; `fire_event` (`:155`) builds an `Event` and dispatches through `ListenerOrchestrator::process_event` (the `Orchestrator` itself exposes no public `fire_event`, matching the requirement's own note); `send_notification` (`:177`) delivers via `NotificationDeliveryPort`. Every method consults `BridgePolicy` first (see the sibling row's `:92`/`:100` citations). **Command actually run this session:** `cargo test --lib orchestrator_bridge::` → `test result: ok. 10 passed; 0 failed`. **Part B candidate 9 pointer:** see the identical note on the `REQ-bridge-policy-guardrails` row above — this adapter is the FR-12-18 half of the same PRD §6.1 that PROMOTION.md's Part B inventory names candidate 9, not promoted to a fourth ADR here (D-20) |
| REQ-execution-service-bridge-wiring | Shipped — re-confirmed this session: `src/application/services/paladin/paladin_execution_service.rs:145` `orchestrator_port: Option<Arc<dyn OrchestratorPort>>`, mirroring the existing optional `garrison`/`arsenal` fields; `:214` `pub fn with_orchestrator_port` builder setter, added backward-compatibly — the 4-arg `PaladinExecutionService::new(llm_port, circuit_breaker, garrison, arsenal)` still compiles. Unit test `test_orchestrator_port_wiring` (`:2192-`) covers all four bridge methods against a mock port scaffolded at `:2155-2189`. Integration test `tests/agent_orchestrator_bridge.rs` deterministically drives a real `PaladinExecutionService` with a scripted mock-LLM tool call to `schedule_job`: `agent_schedule_job_reaches_orchestrator_scheduler` asserts the job is observable in the real `Orchestrator`'s scheduler state, and `agent_schedule_job_rejected_when_action_disallowed` proves the byte-for-byte-unchanged-when-absent / policy-enforced path. **Commands actually run this session:** `cargo test --lib test_orchestrator_port_wiring` → `1 passed`; `cargo test --test agent_orchestrator_bridge` → `test result: ok. 2 passed; 0 failed`. Option B (an `OrchestratorArmament` Arsenal tool) remains an explicitly deferred non-breaking follow-up, matching the requirement's own note |

### Milestone 9 Epic 5 — User / Admin System Completion (5 IDs)

Owned by plan 13-02.

| Requirement | Verdict |
|---|---|
| REQ-user-role-rbac | Shipped — re-confirmed this session: `crates/paladin-core/src/platform/container/user.rs:72` `pub enum UserRole` (`Admin`/`User`), `:84-85` string forms `"admin"`/`"user"`, `:99` `impl std::str::FromStr for UserRole` plus a lossy fallback `from_str_lossy` (`:91`). `UserData` carries `role: UserRole` (`:126`) defaulting to `UserRole::default()` (`:181`, `User` per its own `impl Default`); `User` exposes `role()`/`set_role()` (`:220`, `:225`) consistent with the `Node<UserData>` accessor pattern. The `users` table migration is idempotent: `crates/paladin-storage/src/sqlite_user_repository.rs:72` adds `role TEXT NOT NULL DEFAULT 'user'` to the `CREATE TABLE IF NOT EXISTS`, and `:89` separately `ALTER TABLE users ADD COLUMN role …` for pre-existing tables, swallowing only the SQLite "duplicate column" error (`:93-94`) so re-running the migration on an already-upgraded database is a no-op rather than a failure. Row mapping reads/writes the column at `:172` (`try_get::<String, _>("role")`) and `:255`/`:289` (`.bind(user.node.role.as_str())`). **Commands actually run this session:** `cargo test -p paladin-ai-core --lib platform::container::user::tests::test_user_role_string_round_trip platform::container::user::tests::test_user_role_default_and_accessors` → both `ok` (2 passed); `cargo test -p paladin-storage --lib --features sqlite sqlite_user_repository::tests::test_role_persisted_and_read_back` → `1 passed` |
| REQ-auth-port | Shipped — re-confirmed this session: `crates/paladin-ports/src/output/auth_port.rs:57` `pub trait AuthPort: Send + Sync` with `:63` `issue_token(user_id: Uuid, role: UserRole) -> Result<AuthToken, AuthError>`, `:71` `verify_token(&str) -> Result<AuthClaims, AuthError>`, `:79` `revoke_token(&str) -> Result<(), AuthError>`; `:17` `pub struct AuthToken`, `:26` `pub struct AuthClaims`; `:37` `pub enum AuthError` with `MissingToken` (`:40`), `InvalidToken` (`:43`), `Expired` (`:46`), `Internal(String)` (`:49`). `argon2 = "0.5.3"` retained at root `Cargo.toml:120` per §7, re-confirmed this session. `AuthPort` lives in `paladin-ports` and `UserRole` in `paladin-core`, both always-compiled crates independent of any `web-server` feature gate |
| REQ-opaque-bearer-token-adapter-v1 | **Contract diverges → WEB-01.** Re-confirmed this session: `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs` is the **only** `AuthPort` implementation in the workspace (`grep -rln "impl AuthPort for" src/ crates/` returns exactly this one file); `grep -rn 'jsonwebtoken' Cargo.toml crates/*/Cargo.toml` returns **nothing**, re-run this session. This is Milestone 9's shipped mechanism: an opaque, in-process, hashed bearer-token store, chosen deliberately with JWT/OIDC/OAuth as an explicit non-goal (FR-8-12, §5). Meanwhile Milestone 12 Epic 5's `crates/paladin-web/src/agent_auth.rs` documents its verifier as JWT throughout — the two milestones describe the token mechanism differently, and no shipped code resolves which is authoritative. This is **variant group 29**, the only variant in five verification runs that shipped code itself cannot settle (both halves are equally "shipped," they just aren't the same thing). **Phase 14 / WEB-01 owns the resolution; it is not resolved here.** Its own §6.1 multi-process in-memory-store caveat → WEB-02 |
| REQ-auth-middleware-rbac-guards | Shipped — re-confirmed this session: `crates/paladin-web/src/auth_middleware.rs` — `:59` `authenticate()` reads `Authorization: Bearer <token>` (`:43` `bearer_token()`), calls `AuthPort::verify_token`, injects `AuthClaims`; `:30` `unauthorized()` and `:34` `forbidden()` return a unified `ApiError` JSON envelope without revealing which check failed; `:71` `check_admin()` returns `403` for non-admin, `401` if unauthenticated; `:88` `authorize_self_or_admin()` returns `403` for a non-admin accessing another user's record; `:101` `require_auth` and `:120` `require_admin` are the composable Axum middleware functions. Router composition is `paladin_web::app::create_app_router` (used directly by the integration tests below); `paladin-web` depends only on `paladin-ports` + `paladin-core` and the middleware never performs cryptography itself — it only calls `AuthPort`. Offline deterministic integration tests in `crates/paladin-web/tests/auth_rbac.rs`: `protected_route_without_token_is_unauthorized` (`:166`), `protected_route_with_valid_admin_token_succeeds` (`:179`), `admin_route_with_user_token_is_forbidden` (`:193`), `admin_route_with_admin_token_succeeds` (`:207`), `admin_route_with_invalid_token_is_unauthorized` (`:221`). **Command actually run this session:** `cargo test -p paladin-web --test auth_rbac` → `test result: ok. 5 passed; 0 failed` |
| REQ-user-crud-completeness | Shipped — re-confirmed this session: `crates/paladin-web/src/user_controller.rs:395` `delete_user` and `:406` `list_users`, both admin-only per `app.rs:34`'s router comment (`GET /users`, `DELETE /users/:id` restricted to admin). User-data responses omit the password hash (verified via the same `auth_rbac.rs` fixtures, which never populate or assert a hash field on any response body). Unit tests for the auth round-trip (issue → verify, expiry, revoke, invalid-token, role/string conversions) live alongside `AuthPort`'s concrete adapter and `UserRole`'s `FromStr`/`Display` impls, already cited on the `REQ-user-role-rbac` and `REQ-opaque-bearer-token-adapter-v1` rows above. The offline router-level `401`/`200`(admin-token)/`403`(user-token on admin route)/success matrix is the same `auth_rbac.rs` suite cited on `REQ-auth-middleware-rbac-guards` — **command actually run this session:** `cargo test -p paladin-web --test auth_rbac` → `test result: ok. 5 passed; 0 failed` |

### Milestone 9 Epic 6 — Finalization & Release (1 ID)

Owned by plan 13-02.

| Requirement | Verdict |
|---|---|
| REQ-m9-quality-gate-v030 | Shipped — re-confirmed this session: `CHANGELOG.md:596` `## [0.3.0] - 2026-05-31`, grouped by feature area exactly as the requirement specifies — Orchestration (Epic 1), Scheduler & Queue (Epic 2), Content Pipeline (Epic 3), Agent–Orchestrator Bridge (Epic 4), User/Admin System & Security (Epic 5) — describing user-visible changes, not commit-by-commit detail. `git tag --sort=-v:refname` re-run this session shows `v0.3.0-rc.1` present. `git show v0.3.0-rc.1:Cargo.toml` confirms lockstep at that tag: root `version = "0.3.0"` and `paladin-core = { package = "paladin-ai-core", version = "0.3.0", … }` — the root crate and the aliased member both at `0.3.0`. This is the first of four release gates run 5 supplies → ORCH-05 (D-16 appends this as the `v0.3.0` row to ADR-0029's `## Trajectory` table; **not appended here** — that append is plan 13-12's). Its explicit non-goal ("reconciling whether the previous published version *should* have been 0.2.0") is the reason the trajectory has a gap HARD-03 records |

### Milestone 10 Epic 1 — Pre-Commit & Pre-Push Hooks (4 IDs)

Owned by plan 13-03. Fully derived this session — all four rows carry a fresh `file:line` citation
plus a named consumer, re-run against this tree rather than transcribed.

| Requirement | Verdict |
|---|---|
| REQ-pre-commit-framework | Shipped — re-confirmed this session: `.pre-commit-config.yaml` (repo root, version-controlled) sets `default_install_hook_types: [pre-commit, pre-push]` (`:7-9`); consumed by `Makefile:304-305` `hooks` target (`pre-commit install` + `pre-commit install --hook-type pre-push`, corrected from the stale `:282` citation — line drift). The rejected alternative (`cargo-husky`) and rationale are recorded at `.project/Milestone_10-CI-Hardening-Release-Automation/Epic_1/tasks-pre-commit-pre-push-hooks.md:53-59`, not in the shipped config — satisfied by the framework's adoption and named consumer per the manifest carve-out (D-03) |
| REQ-pre-commit-hook-set | Shipped — re-derived this session (was bare `Verify`): the nine commit-stage hooks are present and named in `.pre-commit-config.yaml` — `cargo-fmt` (`:73-79`, `cargo fmt --all -- --check`, `pass_filenames: false`, `always_run: true`), `cargo-clippy` (`:81-87`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`), `gitleaks` (`:54`), `check-toml` (`:36`), `check-yaml` (`:37`, `--allow-multiple-documents`), `check-added-large-files` (`:39`, `--maxkb=1024` — the 1 MB limit), `check-merge-conflict` (`:41`), `trailing-whitespace` (`:42`), `end-of-file-fixer` (`:47`). Both Rust hooks are `local`/`system` with `pass_filenames: false` and `always_run: true`, satisfying the once-per-workspace (not once-per-file) constraint. `pre-commit run --all-files` was not re-run this session — it would cold-compile the whole workspace via `cargo-clippy`'s `always_run: true`, which this worktree's own execution contract explicitly avoids — so the whole-repo-pass claim rests on the CI gate row below and the hooks' own presence, not a fresh local invocation; recorded as a limitation |
| REQ-pre-push-hook-set | Shipped — re-derived this session (was bare `Verify`): `.pre-commit-config.yaml` `cargo-build-push` (`:90-95`, `cargo build --workspace`, `stages: [pre-push]`) and `cargo-test-lib-push` (`:98-103`, `cargo test --workspace --lib`, `stages: [pre-push]`), wired through the same framework via `default_install_hook_types: [pre-commit, pre-push]` (`:7-9`) and installed together by `Makefile:304-313`'s `hooks` target (both `pre-commit install` and `pre-commit install --hook-type pre-push`). `docs/src/contributing/development-setup.md` (the Milestone 11-relocated `CONTRIBUTING.md`) documents installing `pre-commit` (`:59-77`), running hooks manually (`:91-92`), and the emergency override (`:100-101`, `git commit --no-verify` / `git push --no-verify`) |
| REQ-pre-commit-ci-gate | Shipped, with a caveat on the push half — citation re-run this session: `.github/workflows/pre-commit.yml` job `pre-commit` (`:24-53`) runs `pre-commit run --all-files` via `pre-commit/action@v3.0.1` (`:52-53`), triggered `on: pull_request` (`:5,11`). The file's own comment (`:6-9`) states the `push:` trigger is deliberately commented out "to avoid duplicate runs... on a branch that already has an open PR" — so the acceptance clause "and on pushes to the primary branches" is not met literally for a branch without an open PR (`grep -rn 'pre-commit' .github/workflows/*.yml` returns only this one file — `ci.yml` carries no equivalent job). Recorded as a limitation of this `Shipped` verdict: the gate is required on every PR and enforces the identical local hook suite; the gap is narrower than the requirement's own text (push-without-PR only) |

### Milestone 10 Epic 2 — Dependency Security & Licence Compliance (8 IDs)

Owned by plan 13-03. **Epic-level note (transcribed from the source ledger):** this Epic is where the
one false acceptance criterion lives — every deliverable ships and §8's "no inline advisory-ignore
flags remain in CI" is false. See this ledger's own `Shipped, one acceptance criterion false` highlight
table above, which carries both halves dated, superseding the bare "false" half transcribed below.

| Requirement | Verdict |
|---|---|
| REQ-audit-toml-single-source | **Shipped, one acceptance criterion false** — both halves, dated (D-05; the full account is this ledger's own highlight table above, echoed here rather than restated). **Failed** — the duplicate `security` job at pre-deletion `ci.yml:465-482` ran `cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2025-0111` after `security-audit` (`ci.yml:61-78`, single-source comment at `:74-76`, re-confirmed this session) had already been corrected: two jobs, one display name ("Security Audit"), different verdicts on the same `Cargo.lock`. **Fixed, dated 2026-08-08** — commit `cb75b2b` (Phase 9, plan 09-06) deleted the duplicate job; ADR-0036 (Phase 12) records the single-source topology, with `scripts/check-workflow-suppressions.sh` (`Makefile:171-176`, `.github/workflows/ci.yml:103-104`) regression-guarding it. Re-run this session: `grep -n "cargo audit --ignore" .github/workflows/ci.yml` → zero matches — the deletion held. **SUPPLY-01's previously-pending trigger clause is now resolved with a live citation, re-run this session** (superseding D-06's "pending" framing, which named `30861568499` (2026-08-03) as the newest run before the deletion): `gh run list --workflow=ci.yml --limit 5 --branch release/v0.7.0` returns run **`31320378772`** (2026-08-09, `success`) as the newest entry, newer than `30861568499`; `gh run view 31320378772 --json jobs` shows the single `Security Audit` job with `conclusion: "success"` — one non-duplicated job, resolving the required-status-check trigger the first real CI run after the 2026-08-08 deletion |
| REQ-advisory-exception-process | **Closed → SUPPLY-02** (`REQUIREMENTS.md:1941-2046`), cited per D-06 rather than re-verified: ADR-0024 (Phase 9, plan 09-02, commits `a587e5a`/`7ee741c`) extended M10 Epic 2 FR-3's four-field schema with `owner`/`review_date` and ratified the three 2026 vulnerability ignores in `SECURITY-EXCEPTIONS.md`, which backfills all ten surviving suppressions (owner `DF3NDR`, `review_date` `2026-12-31` on every row); Phase 12 plan 12-01 (dated 2026-08-09) re-ran `cargo deny check` (exit `0`) and `./scripts/check-advisory-register.sh` twice, byte-identical output, against the corrected baseline of **ten** entries, not the original thirteen/fifteen. This session's own read of the tree is consistent with that closure: `.cargo/audit.toml`'s five vulnerability-class entries carry the four-field comment shape with `RUSTSEC-2023-0071`/`RUSTSEC-2025-0111` preserved verbatim as the PRD's two-advisory baseline, and `deny.toml:116-130` `[advisories].ignore` holds exactly ten entries (five vulnerability, five unmaintained) matching `SECURITY-EXCEPTIONS.md` |
| REQ-osv-scanner-supplementary | Shipped — re-confirmed this session: `.github/workflows/ci.yml:126-152` job `osv-scanner`, using `google/osv-scanner-action/osv-scanner-action@v1.9.1` (`:139`) against `Cargo.lock` (`--lockfile=Cargo.lock`, `:143`), `continue-on-error: true` (`:140`) and SARIF upload via `github/codeql-action/upload-sarif@v3` (`:149`) — annotate-only, non-blocking, exactly Epic 2 **Open Question 1**'s recommendation, stated in the job's own comment (`:123-125`) |
| REQ-snyk-evaluation-decision | Shipped — re-derived this session (was `pending`): `docs/src/appendix/security-scanning.md:95-122` (the relocated decision doc, Milestone 11 mdbook) records **Decision: Deferred** (`:97`) with a four-tool comparison table (`:103-112` — RustSec advisories, OSV coverage, licence compliance, dependency bans, reachability analysis, automated fix PRs, external-secret requirement, maintenance cost), the rationale (`:114-118`, no external account/secret and fully version-controlled policy already covers advisories+licences) and a stated revisit condition (`:120-122`, reachability analysis or fix-PR automation needed) — exactly **Open Question 2**'s default recommendation. No silent skip: the decision is explicit and dated |
| REQ-deny-license-allowlist | Shipped — re-confirmed this session: `deny.toml:23-46` `[licenses] allow` lists MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib plus four justified additions (Unicode-3.0, 0BSD, CC0-1.0, CDLA-Permissive-2.0), each carrying an inline FR-14(a) justification comment (`:31-45`); eight `[[licenses.exceptions]]` entries (`:55-78`) grant MPL-2.0 to eight named crates (`colored`, `attohttpc`, `cssparser`, `cssparser-macros`, `dtoa-short`, `minidom`, `selectors`, `smartstring`) rather than weakening the global allow-list — textbook FR-14(b) compliance. Consumed by `ci.yml:81-121` job `cargo-deny` (`cargo deny check`, `:120-121`). Licence *posture* remains contested elsewhere in the corpus → SEC-02, unaffected by this row |
| REQ-deny-bans-duplicates | Shipped — re-confirmed this session: `deny.toml:95-103` `[bans]` sets `multiple-versions = "warn"` (`:96`) and `wildcards = "warn"` (`:97`) — **Open Question 4**'s recommendation, not `deny` — with the only `deny` entry being the pre-existing Milestone 8 `actix-web` ban (`:99-102`), so `[bans]` started empty exactly as specified before that one addition. Same `cargo-deny` CI consumer as the row above |
| REQ-cyclonedx-sbom-release | Shipped — re-confirmed this session, with two stale line citations corrected: `.github/workflows/release.yml:315-336` job `sbom` installs `cargo-cyclonedx --locked` (`:328-329`) and runs `cargo cyclonedx --all --format json` (`:336`), attached as a release asset; `Makefile:286-294` `sbom` target wraps the identical command locally (not `:264`, which is now unrelated content — line drift, corrected here). CycloneDX-only, no SPDX, per **Open Question 3** |
| REQ-security-docs-make-target | Shipped — re-confirmed this session, with a stale line citation corrected: `Makefile:283-284` `security: audit deny` wraps `make audit` + `make deny` for one-command local verification (not `:261`, which is now `.PHONY: check`/`check` — line drift, corrected here); `docs/src/appendix/security-scanning.md` documents both commands, exception-adding, and where SBOMs publish, consumed by the `cargo-deny`/`security-audit` CI jobs cited on the sibling rows above |

**Transcribed source correction, retained per D-00d/D-00c** (`REQUIREMENTS.md:3723-3727`, dated
2026-08-09, Phase 12 plan 12-01, citing `ci.yml:465-482` and commit `cb75b2b`): `REQ-audit-toml-single-source`'s
row above cites `ci.yml:389-406` for the duplicate `security` job — that range never held it; it was
re-derived at `ci.yml:465-482` and deleted by Phase 9's plan 09-06 in commit `cb75b2b`, so the citation
was already stale before Phase 9 touched anything. SUPPLY-01 is closed; see this ledger's own
`Shipped, one acceptance criterion false` highlight table above for the fresh, independently-re-run
citation.

### Milestone 10 Epic 3 — Release Automation (6 IDs)

Owned by plan 13-03.

| Requirement | Verdict |
|---|---|
| REQ-release-tooling-selection | run-5 input (not yet re-derived): Shipped — `release.toml` |
| REQ-workspace-publish-order | pending — plan 13-03 |
| REQ-lockstep-versioning | run-5 input (not yet re-derived): Shipped — root and every member at one version; the mechanism that produced v0.3.0 → v0.6.0 → ORCH-05 |
| REQ-tag-triggered-publish-pipeline | run-5 input (not yet re-derived): Shipped — `release.yml:355` job `publish-crates` |
| REQ-make-release-target | run-5 input (not yet re-derived): Shipped — `Makefile:439` `release`, `:424` `publish-dry-run`, `:413` `release-check` |
| REQ-contributing-add-dependency-guide | pending — plan 13-03 |

### Milestone 10 Epic 4 — v0.4.0 Release (1 ID)

Owned by plan 13-03.

| Requirement | Verdict |
|---|---|
| REQ-m10-v040-release | run-5 input (not yet re-derived): Shipped → ORCH-05. Its **non-goals froze the security configs** ("No changes to `deny.toml` or `.cargo/audit.toml`", "No new CI jobs — the Epic 3 pipeline is complete"), which is why nothing in the milestone was positioned to catch SUPPLY-01. FR-1 step 5 is also what **authorises** the ten unmaintained `deny.toml` ignores → SUPPLY-02 |

### Milestone 10 Epic 5 — Tag-Source Enforcement (4 IDs)

Owned by plan 13-03. **Epic-level note (transcribed from the source ledger):** this Epic exists
because of an incident — the only Epic in the corpus created in response to one.

| Requirement | Verdict |
|---|---|
| REQ-verify-tag-source-guard | run-5 input (not yet re-derived): Shipped — `release.yml:29` job `verify-tag-source`; `:74` and `:97` both declare `needs: verify-tag-source`, which is the two roots FR-1.5 specifies |
| REQ-make-release-branch-guard | pending — plan 13-03 |
| REQ-github-rulesets | run-5 input (not yet re-derived): Shipped — `.github/rulesets/protect-main-branch.json`, `.github/rulesets/protect-release-tags.json`, committed rather than configured only in the UI |
| REQ-branch-protection-doc | pending — plan 13-03 |

### Milestone 11 Epics 1-2 — mdbook Scaffold & Chapter Hierarchy (4 IDs)

Owned by plan 13-04. Rows not yet re-derived carry a `pending` marker naming plan 13-04 (bare `Verify`
in the source) or the source's own text prefixed `run-5 input (not yet re-derived):` (everything else).

| Requirement | Verdict |
|---|---|
| REQ-mdbook-scaffold | run-5 input (not yet re-derived): Shipped — `docs/book.toml`, with `mdbook-mermaid` wired (`docs/mermaid.min.js`, `docs/mermaid-init.js`) |
| REQ-mdbook-chapter-hierarchy | run-5 input (not yet re-derived): Shipped — `docs/src/{getting-started,architecture,user-guides,deployment,deployment-topologies,operations,api-reference,contributing,appendix}` plus `SUMMARY.md` and `introduction.md` |
| REQ-docs-ci-pages-deploy | pending — plan 13-04 |
| REQ-docs-migration-log | run-5 input (not yet re-derived): Shipped — `docs/MIGRATION_LOG.md`. **This is the document that makes the run-3/run-4 "missing deliverables" findings explicable**: `STABLE_API.md`, `docs/FEATURE_FLAGS.md`, `docs/MIGRATION.md`, `docs/CONFIGURATION.md`, `docs/PERFORMANCE_BASELINE.md` and four more were **relocated here, not deleted** → ARCH-05, HARD-01 |

### Milestone 11 Epic 3 — Content Rewrite (7 IDs)

Owned by plan 13-04. **Epic-level note (transcribed from the source ledger):** the only genuinely open
checkbox block in run 5 lives here — 26 items, all in `tasks-content-rewrite.md`. And its §5 non-goals
are what froze the architecture appendix.

| Requirement | Verdict |
|---|---|
| REQ-doc-link-repair-linkcheck | run-5 input (not yet re-derived): Shipped — `docs/book.toml [output.linkcheck]` with `follow-web-links = false` and `warning-policy = "error"`, verbatim as FR-1 specifies. **Task 1.2 (review the full report) is one of the 26 open items** → DOCS-01 |
| REQ-doc-example-compile-gate | pending — plan 13-04 |
| REQ-getting-started-rewrite | pending — plan 13-04 |
| REQ-architecture-docs-update | run-5 input (not yet re-derived): **Shipped, with the gap intact → DOCS-02** — the architecture *chapter* was rewritten; §5 non-goals exempt the 35 appendix files, and `design-and-architecture.md` had been relocated into the appendix by Epic 2. It is still exactly 311 lines with zero of seven newer subsystems and zero Mermaid diagrams |
| REQ-user-guides-rewrite | run-5 input (not yet re-derived): **Verified open (content) → DOCS-01** — all six target files exist under `docs/src/user-guides/`; task 6.0's in-place updates are unchecked and cannot be settled by file existence |
| REQ-deployment-operations-docs-update | run-5 input (not yet re-derived): **Verified open (content) → DOCS-01** — all eight target files exist under `docs/src/deployment/` and `docs/src/operations/`; task 7.0 is unchecked |
| REQ-api-reference-contributing-rewrite | pending — plan 13-04 |

### Milestone 11 Epic 4 — New Subsystem Guides (4 IDs)

Owned by plan 13-04.

| Requirement | Verdict |
|---|---|
| REQ-orchestration-guide | pending — plan 13-04 |
| REQ-content-processing-guide | pending — plan 13-04 |
| REQ-agent-orchestrator-bridge-guide | pending — plan 13-04 |
| REQ-crate-map-feature-flag-reference | run-5 input (not yet re-derived): Shipped (relocated) — ships as `docs/src/api-reference/{crate-map,feature-flags}.md`, which is where run 3's four "missing" deliverables went |

### Milestone 11 Epic 6 — Deployment Topologies (1 ID)

Owned by plan 13-04. **Epic-level note (transcribed from the source ledger):** this Epic created
Milestone 12 — writing the topology documentation surfaced a capability gap (no HTTP service host)
instead of papering over it.

| Requirement | Verdict |
|---|---|
| REQ-deployment-topologies-section | run-5 input (not yet re-derived): Shipped — all six pages exist: `docs/src/deployment-topologies/{overview,embedded-library,battalion-orchestration,http-service-host,queue-worker,sidecar}.md`. **FR-8 makes `overview.md` "the single source of routing" between topologies**, which is what makes the Garrison/Arsenal asymmetry a documentation defect → ORCH-04(b) |

### Milestone 11 Epics 5 & 7 — README, Version Sync, Final Review, v0.5.0 (4 IDs)

Owned by plan 13-04.

| Requirement | Verdict |
|---|---|
| REQ-mdbook-final-review | pending — plan 13-04 |
| REQ-doc-version-sync | pending — plan 13-04 |
| REQ-readme-landing-page | run-5 input (not yet re-derived): Shipped, **and it supersedes a Deferred-QA clause** — the README became a concise landing page with no demos section, so FR-26.4's embedding requirement targets a document that changed shape → DOCS-04 |
| REQ-m11-v050-release | run-5 input (not yet re-derived): Shipped → ORCH-05 |

### Milestone 12 Epic 1 — Agent Registry & Execution API (6 IDs)

Owned by plan 13-05. Rows not yet re-derived carry the source's own text prefixed
`run-5 input (not yet re-derived):` — this section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-agent-registry | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/agent_registry.rs`. **§7 names `project/current-exports.txt`** → DEBT-01. **Corrected (Phase 8, dated 2026-08-06):** §7's "API surface" bullet is now annotated in place at source (`.project/Milestone_12-Web-API/Epic_1/prd-agent-registry-execution-api.md`) with a dated D-00c banner and inline struck-and-corrected text naming `.project/current-exports.txt`; original text retained, nothing deleted → DEBT-01 |
| REQ-agent-execute-endpoint | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/agent_controller.rs`. **Route prefix contested** (`/agents/...` here, `/v1` in Epic 6) → ORCH-03(a) |
| REQ-agent-discovery-endpoints | run-5 input (not yet re-derived): Shipped — same controller; the "no raw system prompt in discovery responses" clause is re-asserted by Epic 5 FR-12 |
| REQ-agent-runtime-registration | run-5 input (not yet re-derived): Shipped — admin-gated by Epic 5 FR-9 |
| REQ-agent-provisioner-port | run-5 input (not yet re-derived): **Shipped, placement undecided → ORCH-04(a)** — Open Question 2 recorded a *default* (`paladin-web`) rather than a decision, and two shipped deployment-topology pages describe would-be second consumers |
| REQ-paladin-web-no-facade-dep | run-5 input (not yet re-derived): Shipped — **the strongest architectural invariant in run 5**, stated three times across two Epics with a mechanical verification command (`cargo tree -p paladin-web` must show no facade dependency). The clearest SPEC candidate in the run |

### Milestone 12 Epic 2 — Configurable Web Host & Server Binary (4 IDs)

Owned by plan 13-05.

| Requirement | Verdict |
|---|---|
| REQ-host-agents-config-schema | pending — plan 13-05 |
| REQ-registry-from-config-builder | run-5 input (not yet re-derived): **Shipped, with a non-goal that needs surfacing → ORCH-04(b)** — "Garrison (memory) and Arsenal (tools/MCP) wiring for agents is a later enhancement; agents are LLM + prompt only here" |
| REQ-concrete-agent-provisioner | run-5 input (not yet re-derived): Shipped — in the facade, as specified |
| REQ-paladin-server-binary | run-5 input (not yet re-derived): Shipped — `src/bin/paladin-server.rs`; `Cargo.toml:249-251` `[[bin]] name = "paladin-server"` with `required-features = ["web-server"]` |

### Milestone 12 Epic 3 — Streaming & Async Jobs (4 IDs)

Owned by plan 13-05.

| Requirement | Verdict |
|---|---|
| REQ-execute-stream-service | pending — plan 13-05 |
| REQ-sse-streaming-endpoint | run-5 input (not yet re-derived): Shipped — SSE plus in-process jobs via `crates/paladin-web/src/job_store.rs` |
| REQ-execution-timeout-cancellation | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/timeout.rs` |
| REQ-async-jobs-api | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/job_store.rs` |

### Milestone 12 Epic 4 — Operational Hardening (5 IDs)

Owned by plan 13-05. This section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-api-error-envelope | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/error.rs` |
| REQ-health-ready-endpoints | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/health.rs`; deliberately outside the auth layer per Epic 5 FR-15 |
| REQ-request-logging-request-id | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/request_log.rs`; carries the header-redaction duty from Epic 5 FR-13 |
| REQ-cors-body-limit-timeout | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/http_layers.rs` |
| REQ-rate-limiting | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/Cargo.toml:33` `tower_governor = { version = "0.8", features = ["axum"] }` |

### Milestone 12 Epic 5 — API Security & Authorization (6 IDs)

Owned by plan 13-06. **Epic-level note (transcribed from the source ledger):** all three of Milestone
12's open checkboxes are in this Epic and all three are Task 0.0 feature-branch scaffolding — "Create
feature branch," "Update `main` … and create/checkout
`feature/m12-epic5-api-security-authorization`," "Confirm a clean baseline." The Epic 5 code ships.
Zero real work is represented by that count.

| Requirement | Verdict |
|---|---|
| REQ-api-key-auth | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/agent_auth.rs`, constant-time comparison, with a test asserting a key value does not leak |
| REQ-jwt-bearer-auth-v2 | run-5 input (not yet re-derived): **Contract diverges → WEB-01** — the v2 *shape* ships (bearer-first precedence, `jwt: Option<Arc<dyn AuthPort>>`, `MockJwt` test double) while the v1 *mechanism* is what executes. No `jsonwebtoken` anywhere. **Open Question 4 is unanswerable for the shipped adapter.** Variant group 29 |
| REQ-fail-closed-auth-posture | pending — plan 13-06 |
| REQ-per-agent-role-authorization | pending — plan 13-06 |
| REQ-admin-gated-registration | pending — plan 13-06 |
| REQ-secret-hygiene-redaction | run-5 input (not yet re-derived): Shipped (partially verified) — the redaction test exists in `agent_auth.rs`; the full nine-case test matrix was not re-run → ORCH-01 |

### Milestone 12 Epic 6 — OpenAPI & Interactive Docs (4 IDs)

Owned by plan 13-06. This section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-openapi-spec-generation | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/src/openapi.rs`; `utoipa = "5"`, `utoipa-axum = "0.2"`, `utoipa-swagger-ui = "9"` |
| REQ-swagger-ui-docs-endpoint | run-5 input (not yet re-derived): Shipped — `utoipa-swagger-ui` wired; unversioned, per §4.3 |
| REQ-api-v1-versioning | run-5 input (not yet re-derived): **Later position, contested → ORCH-03(a)** — §4.3 relocates the agent API under `/v1` after four Epics wrote acceptance criteria, tests and examples against unprefixed paths |
| REQ-openapi-drift-guard | run-5 input (not yet re-derived): Shipped — `crates/paladin-web/openapi.json` is the committed baseline, and is therefore **the artefact that settles the route-prefix question**. **`cross_refs` names `project/current-exports.txt`** → DEBT-01. **Corrected (Phase 8, dated 2026-08-06):** the source clause is in fact §7's "API surface" bullet, not a `cross_refs` field — that label drift is recorded at source. It is now annotated in place (`.project/Milestone_12-Web-API/Epic_6/prd-openapi-spec-interactive-docs.md`) with a dated D-00c banner and inline struck-and-corrected text naming `.project/current-exports.txt`; original text retained, nothing deleted → DEBT-01 |

### Milestone 12 Epic 7 — Deployment Artefacts, Examples & Docs (5 IDs)

Owned by plan 13-06.

| Requirement | Verdict |
|---|---|
| REQ-dockerfile-server-compose | run-5 input (not yet re-derived): Shipped — `Dockerfile.server`; `docker/docker-compose.yml` |
| REQ-k8s-manifests | run-5 input (not yet re-derived): **Shipped, correctness question open → WEB-02** — `k8s/{deployment,service,configmap,namespace,secret.yaml.example,redis,minio}.yaml` plus a `k8s/server/` directory, with liveness and readiness probes. Multi-replica serving against an in-process token store is unaddressed by any requirement in the corpus |
| REQ-deployment-topology-doc-update | run-5 input (not yet re-derived): Shipped — greps for the pre-M12 disclaimers ("ships no agent-execution", "yours to compose", "compose your own", "does not run agents") across `docs/src/` return **zero matches**; `http-service-host.md` references `paladin-server` four times |
| REQ-server-e2e-tests | pending — plan 13-06 |
| REQ-m12-v060-release | run-5 input (not yet re-derived): Shipped — root `Cargo.toml:34` `version = "0.6.0"`, the terminal release gate → ORCH-05. **FR-4.6 names `project/current-exports.txt`** → DEBT-01. Its non-goals are notable: "artifacts/docs/tests/release only; no behavior changes to the API". **Corrected (Phase 8, dated 2026-08-06):** both defective clauses (Success Metric 6 and §4.6 FR-13, the corpus's "FR-4.6") are now annotated in place at source (`.project/Milestone_12-Web-API/Epic_7/prd-deployment-artifacts-examples-docs.md`) with a dated D-00c banner and inline struck-and-corrected text naming `.project/current-exports.txt`; original text retained, nothing deleted → DEBT-01 |

### Deferred-QA Epic 25 — CI/CD Pipeline Enhancement (7 IDs)

Owned by plan 13-07. **Epic-level note (transcribed from the source ledger):** verified open item by
item — seven of eight substantive items are unbuilt. This is the largest concrete unbuilt scope in the
corpus and the register's own recommended first epic, because it "establishes quality gates that
validate all subsequent work." This section has zero bare-`Verify` rows — every row already carries a
rich, item-by-item-verified verdict from ingest run 5.

| Requirement | Verdict |
|---|---|
| REQ-ci-cli-snapshot-job | run-5 input (not yet re-derived): **Verified open → PIPE-01** — no `cli-tests` job; 43 CLI snapshot tests never run in CI |
| REQ-ci-bench-check-job | run-5 input (not yet re-derived): **Verified open → PIPE-01** — no `bench-check` job. Note the inversion: `benchmark-regression-signal`, which this Epic's own non-goals place out of scope, ships at `ci.yml:531` while the compile-check prerequisite does not |
| REQ-ci-combined-coverage-job | run-5 input (not yet re-derived): **Verified open → PIPE-02** — no `coverage` job and no `llvm-cov`/`codecov` reference in `ci.yml`. **But coverage tooling is partially built**: `integration-tests.yml:117-123` runs `cargo llvm-cov --features integration-tests --lcov` and `codecov/codecov-action@v3` — the integration-only path this requirement supersedes. **Open Question 3** (remove or retain it) is unanswered |
| REQ-codecov-config-thresholds | run-5 input (not yet re-derived): **Verified open → PIPE-02** — neither `.codecov.yml` nor `codecov.yml` exists at the root. **The entry threshold competes with the parent PRD**: variant group 30 |
| REQ-makefile-coverage-targets | run-5 input (not yet re-derived): **Verified open → PIPE-03** — none of `coverage`, `coverage-html`, `test-cli`, `bench-check` exists; the `Makefile` has no `llvm-cov` reference at all |
| REQ-modernize-github-actions | run-5 input (not yet re-derived): **Partially open → PIPE-04** — the dangling `on: schedule` block is **gone** (`ci.yml` has exactly one `on:` at line 3, no `schedule:`/`cron:`), the **only** Epic 25 item found closed. Eight deprecated references remain: `actions-rs/toolchain@v1` at `ci.yml:147,317,507` and `integration-tests.yml:71`; `actions/cache@v3` at `integration-tests.yml:78,84,90`; `codecov/codecov-action@v3` at `integration-tests.yml:123` |
| REQ-contributing-coverage-docs | run-5 input (not yet re-derived): **Verified open → PIPE-05** |

### Deferred-QA Epic 26 — Documentation & Rustdoc (4 IDs)

Owned by plan 13-07. This section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-arch-doc-modernization | run-5 input (not yet re-derived): **Verified open, and hidden by a relocation → DOCS-02** — `docs/src/appendix/design-and-architecture.md` is **exactly 311 lines**, the same figure the PRD cites as the pre-rewrite state. Commander 0, Council 0, Conclave 0, Grove 0, Maneuver 0, Sanctum 0, Sentinel 0; zero mermaid blocks. Milestone 11 moved the corpus's largest documentation gap into the one chapter its own Epic 3 non-goals exempt from rewriting |
| REQ-rustdoc-zero-warnings | run-5 input (not yet re-derived): **Open, and the bar is contested → DOCS-03** — three positions on one command across three milestones (M7 zero-warnings, M8 warnings-acceptable, Deferred-QA zero-warnings-enforced-in-CI). HARD-07 picks; DOCS-03 applies |
| REQ-public-api-doc-audit | run-5 input (not yet re-derived): **Verified open → DOCS-03** — couples to DEBT-03, which makes port-trait examples executable rather than merely present |
| REQ-asciinema-demos | run-5 input (not yet re-derived): **Verified open → DOCS-04** — `docs/assets/` exists and is **empty**; `docs/DEMOS.md` does not exist. Open Question 4 (asciinema versus VHS/Terminalizer/GIF) unanswered, and the README target changed shape |

### Deferred-QA Epic 27 — LLM Tool Calling (2 IDs)

Owned by plan 13-07. This section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-llm-tool-calling-port | run-5 input (not yet re-derived): **Verified open → WEB-04** — `crates/paladin-ports/src/output/llm_port.rs` has no `tools` field; `struct ToolDefinition`, `struct ToolCall` and `tool_calls` return zero matches across `paladin-ports` and `paladin-llm`. The requirement names a path deleted by M5 Epic 2 → ORCH-03(c). The PRD flags the change as **breaking to the port interface** |
| REQ-llm-tool-calling-adapters | run-5 input (not yet re-derived): **Verified open, with a separable defect → WEB-03 (the flag), WEB-04 (the scope)** — the problem statement stands unchanged: all three adapters declare tool-calling capability in `ProviderCapabilities` and hardcode `function_call: None`. **`ProviderCapabilities` over-reporting is a correctness defect independent of whether Epic 27 is ever built.** Open Questions 1 and 5 unanswered |

### Deferred-QA Epics 28-29 & the coverage register (4 IDs)

Owned by plan 13-07. This section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-mock-infrastructure | run-5 input (not yet re-derived): **Verified open in the specified shape → DEFER-01** — no `tests/common/` directory; mocks live at `tests/helpers/{mock_llm_adapter,mock_arsenal_adapter,mock_paladin_port}.rs`, a different location and a disjoint set; none of the five named mocks exists. **The shared prerequisite for both coverage epics**, ~6-10 of the 35-45 estimated hours |
| REQ-user-service-test-coverage | run-5 input (not yet re-derived): **Open, and collides with a run-4 register → DEFER-02** — the target still ships at `src/core/platform/manager/user_service.rs` (19,046 bytes), one of only four files left in that directory. M8 `deferred-items.md` D2 (FACADE-02) plans to **split** the same file Epic 28 plans to **test**. Sequence deliberately |
| REQ-listener-service-test-coverage | run-5 input (not yet re-derived): **Open, with a stale path and a stale number → DEFER-03** — the module ships as `src/application/services/orchestration/listener.rs` after the M6 relocation, and M9 Epic 2 added tests against it, so the 57.83% baseline dated 2026-02-14 no longer holds. Scope real, arithmetic not |
| REQ-deferred-coverage-register | run-5 input (not yet re-derived): **Open register → DEFER-01/02/03** — the third and last deferred register in the corpus. Sign-off "AI Coding Agent (Epic 24 execution), February 14, 2026"; **Next Review: "Epic 27 or Epic 28 planning"**, a trigger never reached. **Materially less reliable than Milestone 8's two registers**: both module paths are stale and both baselines predate Milestone 9. Its three unchecked prerequisites are DEFER-01's scope |

### project-management (1 ID)

Owned by plan 13-07. This section has zero bare-`Verify` rows.

| Requirement | Verdict |
|---|---|
| REQ-master-plan-epics-11-18 | run-5 input (not yet re-derived): **Provenance only — do not double-count.** The master expansion plan (Status Draft, v1.0, **2026-01-29** — the earliest document in run 5 and the highest-level planning document in the corpus) defining Epics 11-18 with the dependency graph 11 → 12 → {13, 14} → 15 → {16, 17} → 18. Every one of those eight epics was ingested in run 2, and Conclave, Sanctum/Qdrant, Council, Grove, Maneuver and Sentinel vision are all **verified shipped**. Its value is provenance: it is the only place the dependency graph and the epic-level risk assessment are recorded. Its classifier note observes the content is "strongly PRD-like with embedded SPEC fragments" against a DOC manifest type — retagging it would raise the precedence of positions that shipped a year ago, not add scope |

