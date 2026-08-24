---
phase: 16-documentation-currency-the-architecture-gap
plan: 08
subsystem: docs
tags: [rustdoc, shellcheck, D-05, D-06, DOCS-03, public-api]

# Dependency graph
requires:
  - phase: 16-01
    provides: phase-wide artifact table and CONTEXT.md D-05/D-06 decisions this plan implements
provides:
  - The D-05 enumeration of all D-05-defined public API entry points, with file:line, example
    status, and heading spelling, backed by re-runnable commands
  - scripts/check-public-api-examples.sh — the phase-authored gate for "every entry point has an
    `# Examples` block", since no stable-Rust lint performs this check
  - The D-06 heading rule recorded in .planning/codebase/CONVENTIONS.md Sec Comments
affects: [16-09, 16-10, 16-11, 16-12]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Phase-authored shell gate as the honest fallback for a check with no stable-Rust lint
       equivalent (rustdoc::missing_doc_code_examples is nightly-only)"
    - "Crate-level 'not actually exported' exclusion via publish = false in the crate's own
       Cargo.toml, applied generically rather than hardcoding a single excluded item"

key-files:
  created:
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-ENTRY-POINTS.md
    - scripts/check-public-api-examples.sh
  modified:
    - .planning/codebase/CONVENTIONS.md

key-decisions:
  - "Excluded MockListService (crates/doc-examples/src/support.rs:273) from the entry-point count
     as 'not actually exported' — paladin-doc-examples carries publish = false and its own
     Cargo.toml describes itself as not published"
  - "Recorded a 2-item unexplained delta between D-05's stated 33 Service structs and this run's
     30 legitimate matches (after the MockListService exclusion), rather than adjusting the
     definition to fit 33 — no additional pub struct *Service declaration exists anywhere under
     crates/*/src or src/ per an exhaustive re-run of the derivation command"
  - "Implemented the #[cfg(test)] exclusion as 'first #[cfg(test)] immediately followed by a mod
     declaration' rather than a naive 'first #[cfg(test)] line in the file', because
     phalanx_service.rs has #[cfg(test)]-gated use statements before its real struct declaration —
     a naive heuristic would have wrongly excluded a legitimate entry point"

# DOCS-03 spans plans 16-08..16-12 (all five carry it in their own PLAN.md frontmatter). This plan
# only builds the enumeration and gate; the baseline gate exits 1 (55 of 76 entry points still lack
# a plural `# Examples` heading), so REQUIREMENTS.md is intentionally left untouched here. See
# "Decisions Made" below.
requirements-completed: []

coverage:
  - id: D1
    description: "D-05 enumeration of 76 public API entry points (11 Builders + 35 *Port traits +
      30 *Service structs) with file:line, example status, and heading spelling, plus a recorded
      delta against D-05's stated 11/35/33"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "for p in $(grep -ohE '(crates|src)/[A-Za-z0-9_/.-]+\\.rs' 16-DOCS-03-ENTRY-POINTS.md | sort -u); do test -f \"$p\"; done — all 62 referenced files resolve"
        status: pass
      - kind: other
        ref: "grep -cE '^\\| (Builder|Port|Service) ' 16-DOCS-03-ENTRY-POINTS.md == 76 (matches record's own stated total)"
        status: pass
    human_judgment: false
  - id: D2
    description: "scripts/check-public-api-examples.sh — shellcheck-clean, executable gate script;
      default mode fails on any MISSING/SINGULAR entry point or a degenerate zero-entry
      derivation; --list is a report that always exits 0 and says so in its own first line"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "shellcheck --severity=warning scripts/check-public-api-examples.sh"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list | grep -q 'Port'"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh (default/gate mode) exits 1"
        status: pass
    human_judgment: false
  - id: D3
    description: "D-06 heading rule recorded in .planning/codebase/CONVENTIONS.md Sec Comments,
      scoped to the 76 enumerated entry points, with the section's own worked example
      (PaladinBuilder::new) updated to the plural spelling"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "grep -q 'check-public-api-examples.sh' .planning/codebase/CONVENTIONS.md"
        status: pass
      - kind: other
        ref: "git diff .planning/codebase/CONVENTIONS.md — all hunks fall between '## Comments' and '## Function Design'"
        status: pass
    human_judgment: false

# Metrics
duration: ~35min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 08: D-05 Entry-Point Enumeration and the `# Examples` Gate Summary

**Enumerated 76 D-05 public API entry points with a re-runnable derivation, authored the
phase-owned `scripts/check-public-api-examples.sh` gate (shellcheck-clean, baseline exit 1: 38
MISSING + 17 SINGULAR of 76), and recorded the D-06 heading rule in `CONVENTIONS.md`.**

## Performance

- **Duration:** ~35 min
- **Started:** 2026-08-24T12:18:00Z (approx.)
- **Completed:** 2026-08-24T12:53:36Z
- **Tasks:** 2
- **Files modified:** 3 (2 created, 1 modified)

## Accomplishments

- Derived the D-05 entry-point set with three independently re-runnable `grep -rnE` commands
  (Builders, `*Port` traits, `*Service` structs) across `crates/*/src` and `src/`, verified each
  of the 77 raw matches against its file's `#[cfg(test)]` boundary, and excluded one
  (`MockListService`) as not-actually-exported (`paladin-doc-examples` is `publish = false`) —
  leaving **76 legitimate entry points**.
- Recorded the delta against D-05's stated 11/35/33: Builders and `*Port` traits match exactly
  (11 and 35); `*Service` structs are 3 short (30 vs 33) — 1 explained (`MockListService`), 2
  unattributed after an exhaustive re-run of the grep command, recorded rather than papered over.
- Authored `scripts/check-public-api-examples.sh`: derives the same 76-item set by the same rule
  (crate-level `publish = false` exclusion applied generically via a Cargo.toml walk-up, not a
  hardcoded item name; `#[cfg(test)]` exclusion via "first `#[cfg(test)]` immediately followed by
  a `mod` declaration" rather than "first `#[cfg(test)]` line in the file" — the naive version
  would have wrongly excluded `PhalanxExecutionService`, which follows two `#[cfg(test)]`-gated
  `use` statements earlier in its file).
- Ran the baseline: `--list` shows 76 entry points, 21 OK, 38 MISSING, 17 SINGULAR; default
  (gate) mode exits 1 as expected, naming every violation with its `file:line`. Full `--list`
  output recorded below.
- Recorded the D-06 rule in `CONVENTIONS.md` Sec Comments — scoped to the 76 enumerated entry
  points, both spellings tolerated elsewhere, `scripts/check-public-api-examples.sh` named as
  enforcer — and updated the section's own worked example (`PaladinBuilder::new`) from `# Example`
  to `# Examples`, the spelling the new rule requires.
- No `.rs` file was touched by either task (`git diff --name-only -- '*.rs'` empty throughout).

## Task Commits

Each task was committed atomically:

1. **Task 1: Enumerate the D-05 public API entry points into the phase record** - `bc6b3eb` (docs)
2. **Task 2: Author the examples gate script, run the baseline, and record the D-06 heading rule** - `cdf72c9` (feat)

## Files Created/Modified

- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-ENTRY-POINTS.md` -
  The D-05 enumeration: selection rule, rejected readings, re-runnable derivation commands, the
  76-row table, delta accounting, and closing totals.
- `scripts/check-public-api-examples.sh` - The D-05/D-06 gate script (executable, shellcheck-clean
  at `--severity=warning`).
- `.planning/codebase/CONVENTIONS.md` - Sec Comments gets the D-06 rule and its worked example is
  updated to the plural spelling; a staleness note on the map's 2026-07-30 date is added inline.

## D-00e evidence: derivation commands and counts

```bash
# 1. Builders — raw count 11 (matches D-05's stated 11 exactly)
grep -rnE '^\s*pub struct [A-Za-z0-9_]*Builder\b' crates/*/src src/ --include='*.rs'

# 2. *Port traits — raw count 35 (matches D-05's stated 35 exactly)
grep -rnE '^\s*pub trait [A-Za-z0-9_]*Port\b' crates/*/src src/ --include='*.rs'

# 3. *Service structs — raw count 31 (D-05 states 33; see delta below)
grep -rnE '^\s*pub struct [A-Za-z0-9_]*Service\b' crates/*/src src/ --include='*.rs'
```

**Delta against D-05's 11 / 35 / 33:**

- Builders: 11 vs 11 — exact match, no delta.
- `*Port` traits: 35 vs 35 — exact match, no delta.
- `*Service` structs: 30 legitimate (31 raw − 1 excluded) vs 33 — a 3-item shortfall.
  - 1 accounted for: `MockListService` (`crates/doc-examples/src/support.rs:273`) — a mock in the
    unpublished (`publish = false`) `paladin-doc-examples` crate, matching D-05's own "a
    non-exported service" exclusion example verbatim.
  - 2 unaccounted for. The `*Service` grep command above, re-run exhaustively across all eleven
    crate directories under `crates/*/src` plus the facade `src/`, returns exactly 31 raw matches
    — no additional declaration exists. `16-CONTEXT.md`'s own prior "Measured baseline" note
    already records "77 resolvable entry-point files" against FR-26.3's 79 (a 2-item gap of
    identical size), recorded before this plan's command-backed re-derivation existed and without
    a `file:line` for its own 2 dropped items. Recorded conclusion: FR-26.3's original 33 figure
    was carried into D-05 without an independent grep-based re-derivation; this enumeration is
    the first one. The definition is **not** adjusted to fit 33.

**Full `--list` output (baseline, this session):**

```
REPORT MODE -- this is not a gate; --list always exits 0 regardless of findings.
KIND	NAME	FILE:LINE	STATUS
Builder	CommanderBuilder	crates/paladin-battalion/src/commander.rs:1328	OK
Builder	LogEntryBuilder	crates/paladin-core/src/platform/container/log.rs:195	MISSING
Builder	PaladinConfigBuilder	crates/paladin-core/src/platform/container/paladin_config.rs:100	SINGULAR
Builder	StreamChunkBuilder	crates/paladin-core/src/platform/container/herald.rs:292	OK
Builder	ExecutionMetadataBuilder	crates/paladin-core/src/platform/container/herald.rs:557	OK
Builder	MemoryBuilder	crates/paladin-core/src/platform/container/sanctum.rs:119	OK
Builder	GroveBuilder	crates/paladin-core/src/platform/container/battalion/grove.rs:311	SINGULAR
Builder	CouncilBuilder	crates/paladin-core/src/platform/container/battalion/council.rs:288	SINGULAR
Builder	PaladinBuilder	src/application/services/paladin/paladin_builder.rs:77	SINGULAR
Builder	PromptBuilder	src/application/cli/interactive/prompts.rs:7	MISSING
Builder	ProgressBarBuilder	src/application/cli/formatters/progress.rs:59	MISSING
Port	ArsenalPort	crates/paladin-ports/src/output/arsenal_port.rs:470	OK
Port	WorkflowRepositoryPort	crates/paladin-ports/src/output/workflow_repository_port.rs:109	MISSING
Port	VisionPort	crates/paladin-ports/src/output/vision_port.rs:63	MISSING
Port	StreamingExecutorPort	crates/paladin-ports/src/output/streaming_executor_port.rs:66	SINGULAR
Port	SanctumPort	crates/paladin-ports/src/output/sanctum_port.rs:585	OK
Port	NotificationDeliveryPort	crates/paladin-ports/src/output/notification_port.rs:796	OK
Port	NotificationTemplatePort	crates/paladin-ports/src/output/notification_port.rs:1120	OK
Port	BasicNotificationPort	crates/paladin-ports/src/output/notification_port.rs:1266	OK
Port	SchedulerPort	crates/paladin-ports/src/output/scheduler_port.rs:237	MISSING
Port	FileStoragePort	crates/paladin-ports/src/output/file_storage_port.rs:980	OK
Port	BatchFileStoragePort	crates/paladin-ports/src/output/file_storage_port.rs:1241	MISSING
Port	AdvancedFileStoragePort	crates/paladin-ports/src/output/file_storage_port.rs:1264	MISSING
Port	FileVersioningPort	crates/paladin-ports/src/output/file_storage_port.rs:1309	MISSING
Port	FullFileStoragePort	crates/paladin-ports/src/output/file_storage_port.rs:1341	MISSING
Port	GarrisonPort	crates/paladin-ports/src/output/garrison_port.rs:380	OK
Port	LongTermGarrisonPort	crates/paladin-ports/src/output/garrison_port.rs:656	OK
Port	OrchestratorPort	crates/paladin-ports/src/output/orchestrator_port.rs:232	MISSING
Port	PaladinExecutorPort	crates/paladin-ports/src/output/paladin_executor_port.rs:60	SINGULAR
Port	EmbeddingPort	crates/paladin-ports/src/output/embedding_port.rs:371	OK
Port	CitadelPort	crates/paladin-ports/src/output/citadel_port.rs:567	OK
Port	AuthPort	crates/paladin-ports/src/output/auth_port.rs:57	MISSING
Port	QueuePort	crates/paladin-ports/src/output/queue_port.rs:549	OK
Port	TypedQueuePort	crates/paladin-ports/src/output/queue_port.rs:617	MISSING
Port	BatchQueuePort	crates/paladin-ports/src/output/queue_port.rs:641	MISSING
Port	PriorityQueuePort	crates/paladin-ports/src/output/queue_port.rs:678	MISSING
Port	QueueManagementPort	crates/paladin-ports/src/output/queue_port.rs:705	MISSING
Port	FullQueuePort	crates/paladin-ports/src/output/queue_port.rs:744	MISSING
Port	PaladinPort	crates/paladin-ports/src/output/paladin_port.rs:631	OK
Port	BattalionPort	crates/paladin-ports/src/output/battalion_port.rs:622	OK
Port	LogPort	crates/paladin-ports/src/output/log_port.rs:219	MISSING
Port	UserRepositoryPort	crates/paladin-ports/src/output/user_repository_port.rs:12	MISSING
Port	LlmPort	crates/paladin-ports/src/output/llm_port.rs:962	OK
Port	DocumentPort	crates/paladin-ports/src/input/document_port.rs:114	MISSING
Port	ContentIngestionPort	crates/paladin-ports/src/input/content_input_port.rs:10	MISSING (empty doc block -- degenerate input)
Port	MlPort	crates/paladin-ports/src/input/ml_port.rs:97	MISSING
Service	GroveExecutionService	crates/paladin-battalion/src/grove_service.rs:103	SINGULAR
Service	ManeuverExecutionService	crates/paladin-battalion/src/maneuver/service.rs:15	MISSING
Service	CouncilExecutionService	crates/paladin-battalion/src/council_service.rs:57	SINGULAR
Service	PhalanxExecutionService	crates/paladin-battalion/src/phalanx_service.rs:41	SINGULAR
Service	FormationExecutionService	crates/paladin-battalion/src/formation_service.rs:37	SINGULAR
Service	CampaignExecutionService	crates/paladin-battalion/src/campaign_service.rs:58	SINGULAR
Service	ConclaveExecutionService	crates/paladin-battalion/src/conclave_execution_service.rs:38	SINGULAR
Service	ChainOfCommandExecutionService	crates/paladin-battalion/src/chain_of_command_service.rs:65	OK
Service	FieldVersionService	crates/paladin-core/src/base/service/field_version_service.rs:93	MISSING
Service	NodeVersionService	crates/paladin-core/src/base/service/node_version_service.rs:94	MISSING
Service	MessageService	crates/paladin-core/src/base/service/message_service.rs:163	MISSING
Service	CollectionVersionService	crates/paladin-core/src/base/service/collection_versioning_service.rs:100	MISSING
Service	DataBackupService	crates/paladin-core/src/platform/container/task.rs:333	MISSING
Service	ContentIndexingService	crates/paladin-core/src/platform/container/task.rs:399	MISSING
Service	EmailNotificationService	crates/paladin-core/src/platform/container/task.rs:535	MISSING
Service	LlmAnalysisService	crates/paladin-llm/src/llm_analysis_service.rs:54	MISSING (empty doc block -- degenerate input)
Service	MemoryExtractionService	crates/paladin-memory/src/services/memory_extraction_service.rs:41	MISSING
Service	RagRetrievalService	crates/paladin-memory/src/services/rag_retrieval_service.rs:28	MISSING
Service	DefaultContentIngestionService	src/application/services/content/content_ingestion_service.rs:240	MISSING
Service	ArsenalRegistryService	src/application/services/arsenal/arsenal_registry_service.rs:42	SINGULAR
Service	ArsenalExecutionService	src/application/services/arsenal/arsenal_execution_service.rs:60	SINGULAR
Service	HandoffService	src/application/services/paladin/handoff_service.rs:42	SINGULAR
Service	TemperatureService	src/application/services/paladin/temperature_service.rs:51	MISSING
Service	PlanningService	src/application/services/paladin/planning_service.rs:45	OK
Service	PaladinExecutionService	src/application/services/paladin/paladin_execution_service.rs:105	SINGULAR
Service	PromptGenerationService	src/application/services/paladin/prompt_generation_service.rs:48	OK
Service	EncryptionService	src/infrastructure/security/encryption.rs:161	SINGULAR
Service	ContentItemService	src/core/platform/manager/content_service.rs:20	MISSING (empty doc block -- degenerate input)
Service	UserService	src/core/platform/manager/user_service.rs:29	MISSING
Service	EventService	src/core/platform/manager/event_manager.rs:69	MISSING
TOTAL: 76 entry points -- 21 OK, 38 MISSING, 17 SINGULAR
```

**Default (gate) mode:** exits **1**. Prints all 55 violation rows (38 MISSING + 17 SINGULAR) to
stdout, then to stderr:
```
ERROR: 55 of 76 D-05 public API entry points lack a plural
'# Examples' heading (MISSING) or use the singular '# Example' spelling
(SINGULAR). Add or fix the heading directly above the item (or in the file's
leading //! module doc), following src/application/services/paladin/paladin_builder.rs
as the worked pattern. Run with --list for the full derived table.
```

This is the recorded **starting baseline** plans 16-09 through 16-12 close — not a defect in the
script. Three items (`ContentIngestionPort`, `LlmAnalysisService`, `ContentItemService`) have a
completely empty preceding doc block (no `///` at all, and no leading `//!` module doc either) —
flagged distinctly as `MISSING (empty doc block -- degenerate input)` per the plan's requirement
that the script "fail loud on a degenerate input" rather than silently treating an undocumented
item the same as a documented-but-example-less one.

## Decisions Made

- **`MockListService` excluded as non-exported**, matching D-05's own worked exclusion category
  ("a non-exported service") — `paladin-doc-examples` carries `publish = false` and describes
  itself as "not published" in its own `Cargo.toml`.
- **2-item Service delta recorded as unattributed**, not resolved by adjusting the definition —
  per D-05's explicit instruction to record deltas honestly rather than force a match to 33.
- **`#[cfg(test)]` exclusion implemented as "immediately followed by `mod`"**, not "any
  `#[cfg(test)]` in the file" — the latter would have wrongly excluded `PhalanxExecutionService`,
  which sits after two `#[cfg(test)]`-gated `use` statements earlier in its own file. Verified
  manually against `crates/paladin-battalion/src/phalanx_service.rs` before writing the script.
- **Crate-level "not actually exported" exclusion implemented generically** (walk up to the
  nearest `Cargo.toml`, check `publish = false`) rather than hardcoding `MockListService` by name
  — so the script stays correct if another unpublished crate is added later.
- **DOCS-03 NOT marked complete in `REQUIREMENTS.md` by this plan.** The standard executor
  protocol marks a plan's frontmatter `requirements:` complete on SUMMARY creation; here that
  would be premature and false. `requirements: [DOCS-03]` appears in all five of this phase's
  plans (16-08 through 16-12) — DOCS-03 ("One `cargo doc` bar, applied, with the public API
  documented to it") is only satisfied once the examples themselves are written and the gate
  exits 0, which is plans 16-09 through 16-12's work. `gsd-tools query requirements.mark-complete
  DOCS-03` was run, observed to flip the checkbox, then reverted via `git checkout --
  .planning/REQUIREMENTS.md` after confirming the other four plans' frontmatter. The requirement
  should be marked complete by whichever of 16-09..16-12 closes the loop (baseline gate exits 0).

## Deviations from Plan

None — plan executed exactly as written. Both tasks' acceptance criteria and `<verify>` commands
pass as specified; no `.rs` file was modified.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `16-DOCS-03-ENTRY-POINTS.md` and `scripts/check-public-api-examples.sh` are the inputs plans
  16-09 through 16-12 need to close the 55 recorded violations (38 MISSING + 17 SINGULAR).
- Plan 16-12's D-06 normalisation sweep is scoped to exactly the 17 SINGULAR rows listed above
  (and cross-referenced in `16-DOCS-03-ENTRY-POINTS.md`'s closing totals) — no other file in the
  tree needs touching for that sub-task.
- No blockers.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

- FOUND: `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-03-ENTRY-POINTS.md`
- FOUND: `scripts/check-public-api-examples.sh`
- FOUND: `.planning/phases/16-documentation-currency-the-architecture-gap/16-08-SUMMARY.md`
- FOUND: commit `bc6b3eb` (Task 1)
- FOUND: commit `cdf72c9` (Task 2)
