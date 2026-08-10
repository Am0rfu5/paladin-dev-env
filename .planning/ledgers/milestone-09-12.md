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
`run-5 input (not yet re-derived):` for the 53+32 rows that already carry content, or
`pending — plan 13-NN` for the 35 rows that do not, per the interim-state contract below.

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

For every row this plan does not derive, the Verdict cell reads `pending — plan 13-NN` naming the
owning fan-out plan from the table above, or the row's existing text prefixed
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

<!-- gsd:write-continue -->
