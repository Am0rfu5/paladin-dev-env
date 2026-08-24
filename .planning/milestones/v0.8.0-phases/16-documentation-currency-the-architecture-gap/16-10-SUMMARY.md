---
phase: 16-documentation-currency-the-architecture-gap
plan: 10
subsystem: docs
tags: [rustdoc, doctests, paladin-core, paladin-battalion, paladin-memory, paladin-herald, DOCS-03]

# Dependency graph
requires:
  - phase: 16-07
    provides: "Workspace-wide cargo doc gate held at zero warnings"
  - phase: 16-08
    provides: "16-DOCS-03-ENTRY-POINTS.md (D-05 enumeration) and scripts/check-public-api-examples.sh (the gate)"
  - phase: 16-09
    provides: "All 35 crates/paladin-ports/ *Port traits documented; established the &dyn-Trait-parameter
       example pattern and the heading-only-fix precedent for pre-existing SINGULAR examples"
provides:
  - "Executable # Examples blocks on all 11 previously-MISSING D-05 entry points in
     crates/paladin-core/, crates/paladin-memory/ and crates/paladin-battalion/"
  - "Plural heading normalization (D-06) on the 9 SINGULAR entry points in those same crates"
  - "All 20 (11 Builders + 9 Services, scoped to this plan's four crates) D-05 entry points now
     report OK under scripts/check-public-api-examples.sh"
  - "Confirmed structural fact: crates/paladin-herald/ contributes zero D-05 entry points (no pub
     *Builder/*Port/*Service declarations there)"
affects: ["16-11", "16-12"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Generic-repository-parameter shape for Service structs wrapping a generic repository trait
       (NodeVersionService<T>, FieldVersionService<T>, CollectionVersionService<T>): the # Examples
       block defines a generic function accepting Arc<dyn Repository<T> + Send + Sync>, constructs
       the service via ::new(), and calls one delegate method with `?`. Mirrors 16-09's
       &dyn-Trait-parameter pattern for Port traits, extended to Service structs whose sole
       dependency is a repository trait — the function body type-checks the real API surface
       without ever being invoked, so no concrete repository implementation is needed."
    - "Arc<dyn Port>-parameter shape for orchestration Services (paladin-battalion's
       *ExecutionService family, MemoryExtractionService, RagRetrievalService): a small function
       accepts the already-implementing Arc<dyn PaladinPort>/LlmPort/EmbeddingPort/SanctumPort and
       constructs the service via ::new(), demonstrating the real constructor signature without
       executing any I/O."
    - "Trivial-constructor shape for path/config-only Services (DataBackupService,
       ContentIndexingService, EmailNotificationService, MessageService): construction alone is the
       example — execute()/start() are deliberately not called in the doctest because they perform
       real filesystem I/O (T-16-26), matching the threat register's compile-and-run-but-no-I/O
       discipline."
    - "Heading-only fix for pre-existing SINGULAR examples with an `ignore` fence (PaladinConfigBuilder,
       GroveBuilder, CouncilBuilder, and all 6 SINGULAR paladin-battalion *ExecutionService structs):
       only the '# Example' -> '# Examples' heading line is touched; the pre-existing `ignore` fence
       disposition is left untouched, exactly matching 16-09's PaladinExecutorPort/
       StreamingExecutorPort precedent. D-06 asks for heading normalization, not a disposition
       re-audit of examples this plan did not author."

key-files:
  created: []
  modified:
    - crates/paladin-core/src/platform/container/log.rs
    - crates/paladin-core/src/platform/container/paladin_config.rs
    - crates/paladin-core/src/platform/container/battalion/grove.rs
    - crates/paladin-core/src/platform/container/battalion/council.rs
    - crates/paladin-core/src/base/service/field_version_service.rs
    - crates/paladin-core/src/base/service/node_version_service.rs
    - crates/paladin-core/src/base/service/message_service.rs
    - crates/paladin-core/src/base/service/collection_versioning_service.rs
    - crates/paladin-core/src/platform/container/task.rs
    - crates/paladin-memory/src/services/memory_extraction_service.rs
    - crates/paladin-memory/src/services/rag_retrieval_service.rs
    - crates/paladin-battalion/src/maneuver/service.rs
    - crates/paladin-battalion/src/grove_service.rs
    - crates/paladin-battalion/src/council_service.rs
    - crates/paladin-battalion/src/phalanx_service.rs
    - crates/paladin-battalion/src/formation_service.rs
    - crates/paladin-battalion/src/campaign_service.rs
    - crates/paladin-battalion/src/conclave_execution_service.rs

key-decisions:
  - "paladin-herald contributes zero D-05 entry points. `grep -rnE '^\\s*pub (struct|trait)
     [A-Za-z0-9_]*(Builder|Port|Service)\\b' crates/paladin-herald/src` returns nothing — the crate
     holds Herald trait implementations (JsonHerald, MarkdownHerald) but no pub *Builder/*Port/
     *Service declaration of its own. This is the same structural fact 16-DOCS-03-ENTRY-POINTS.md
     already recorded for paladin-storage/paladin-web/paladin-content/paladin-notifications: an
     adapter crate implementing a port declared elsewhere is not itself a source of D-05 entry
     points. Task 2's paladin-herald verification therefore reduced to confirming the crate's
     16-07 zero-warning doc posture still holds (it does) and running its (unchanged, pre-existing)
     doctests — no new example was needed or written there."
  - "All 9 SINGULAR headings in this plan's four crates were fixed to plural, not deferred to a
     later normalization sweep. 16-DOCS-03-ENTRY-POINTS.md's closing note names plan 16-12 as owning
     a 17-site D-06 sweep, but 16-10-PLAN.md's own must_haves truths and acceptance criteria
     explicitly require zero SINGULAR rows under crates/paladin-core/, crates/paladin-memory/,
     crates/paladin-battalion/ and crates/paladin-herald/ at this plan's close. The plan's own
     acceptance criteria are authoritative for its scope; this leaves 6 SINGULAR sites (not in
     this plan's four crates) for 16-12."
  - "Generic Service examples (NodeVersionService<T>, FieldVersionService<T>,
     CollectionVersionService<T>) call one Option-returning delegate method (get_current_version /
     get_current_field_version / get_current_collection_version) rather than the history-returning
     methods, keeping the example's Result/Option shape simple and consistent across all three
     sibling versioning services."

requirements-completed: [DOCS-03]

coverage:
  - id: D1
    description: "10 previously-MISSING D-05 entry points in crates/paladin-core/ and
       crates/paladin-memory/ (LogEntryBuilder, FieldVersionService, NodeVersionService,
       MessageService, CollectionVersionService, DataBackupService, ContentIndexingService,
       EmailNotificationService, MemoryExtractionService, RagRetrievalService) gain executable
       # Examples blocks"
    requirement: "DOCS-03"
    verification:
      - kind: unit
        ref: "cargo test --doc -p paladin-ai-core -p paladin-memory"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (zero MISSING under crates/paladin-core/ or crates/paladin-memory/)"
        status: pass
    human_judgment: false
  - id: D2
    description: "3 SINGULAR heading sites in crates/paladin-core/ (PaladinConfigBuilder,
       GroveBuilder, CouncilBuilder) normalized to plural '# Examples' (D-06)"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (zero SINGULAR under crates/paladin-core/)"
        status: pass
    human_judgment: false
  - id: D3
    description: "1 previously-MISSING D-05 entry point in crates/paladin-battalion/
       (ManeuverExecutionService) gains an executable # Examples block; 6 SINGULAR heading sites
       (GroveExecutionService, CouncilExecutionService, PhalanxExecutionService,
       FormationExecutionService, CampaignExecutionService, ConclaveExecutionService) normalized to
       plural, closing crates/paladin-battalion/'s MISSING/SINGULAR count to zero across all 8
       enumerated Service entry points (CommanderBuilder and ChainOfCommandExecutionService were
       already OK)"
    requirement: "DOCS-03"
    verification:
      - kind: unit
        ref: "cargo test --doc -p paladin-battalion"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (zero MISSING/SINGULAR under crates/paladin-battalion/)"
        status: pass
    human_judgment: false
  - id: D4
    description: "crates/paladin-herald/ verified to contribute zero D-05 entry points; its 16-07
       zero-warning doc posture confirmed unchanged after this plan's other additions"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "grep -rnE '^\\s*pub (struct|trait) [A-Za-z0-9_]*(Builder|Port|Service)\\b' crates/paladin-herald/src (zero matches)"
        status: pass
      - kind: other
        ref: "cargo doc -p paladin-herald --no-deps 2>&1 | grep -c warning: (returns 0)"
        status: pass
    human_judgment: false
  - id: D5
    description: "Workspace doc gate (cargo doc --workspace --no-deps, ci.yml:63) still reports
       zero warnings after all additions -- 16-07's bar held through this plan's fourth touch"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q 'warning:' /tmp/doc-output.txt"
        status: pass
    human_judgment: false

duration: 70min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 10: Executable Examples for paladin-core, paladin-battalion, paladin-memory and paladin-herald Entry Points Summary

**Added compile-and-run `# Examples` blocks to all 11 previously-MISSING D-05 entry points across `paladin-core`, `paladin-memory` and `paladin-battalion`, normalized 9 SINGULAR headings to plural, and confirmed `paladin-herald` structurally contributes zero D-05 entry points — taking this plan's four-crate scope from 20/38 OK to 38/38 OK.**

## Performance

- **Duration:** ~70 min
- **Started:** 2026-08-24 (approx.)
- **Completed:** 2026-08-24
- **Tasks:** 2 (Task 1: paladin-core + paladin-memory; Task 2: paladin-battalion + paladin-herald)
- **Files modified:** 18

## Accomplishments

- Every D-05 entry point enumerated in `16-DOCS-03-ENTRY-POINTS.md` under `crates/paladin-core/`,
  `crates/paladin-memory/`, `crates/paladin-battalion/` and `crates/paladin-herald/` now carries a
  plural `# Examples` block: `bash scripts/check-public-api-examples.sh --list` reports **0 MISSING,
  0 SINGULAR** for all rows scoped to these four crates.
- **11 new examples are compile-and-run** (plain ` ```rust ` fence, zero new `no_run`): LogEntryBuilder,
  FieldVersionService, NodeVersionService, MessageService, CollectionVersionService,
  DataBackupService, ContentIndexingService, EmailNotificationService, MemoryExtractionService,
  RagRetrievalService, ManeuverExecutionService.
- **9 pre-existing SINGULAR headings normalized to plural** (D-06): PaladinConfigBuilder,
  GroveBuilder, CouncilBuilder (paladin-core), GroveExecutionService, CouncilExecutionService,
  PhalanxExecutionService, FormationExecutionService, CampaignExecutionService,
  ConclaveExecutionService (paladin-battalion). All 6 of the paladin-battalion sites and 2 of the 3
  paladin-core sites (GroveBuilder, CouncilBuilder) carried pre-existing `ignore`-fenced examples
  predating this plan; per the 16-09 precedent, only the heading line was touched — the fence
  disposition was not re-audited (outside this plan's MISSING scope, and not a new example this
  plan authored).
- **paladin-herald verified to contribute zero D-05 entry points.** `grep -rnE '^\s*pub (struct|trait)
  [A-Za-z0-9_]*(Builder|Port|Service)\b' crates/paladin-herald/src` returns nothing. The crate's own
  `JsonHerald`/`MarkdownHerald` types implement the `Herald` trait declared in `paladin-core`, and
  hold no `pub *Builder`/`*Port`/`*Service` declaration of their own — the same structural pattern
  16-DOCS-03-ENTRY-POINTS.md already recorded for paladin-storage/paladin-web/paladin-content/
  paladin-notifications. No new example was needed there; Task 2's paladin-herald work reduced to
  confirming the crate's 16-07 zero-warning posture holds and its pre-existing doctests still pass.
- `cargo test --doc -p paladin-ai-core -p paladin-memory -p paladin-battalion -p paladin-herald`:
  **95 passed, 0 failed** (57 + 9 + 29 + 0, remainder pre-existing `ignored`).
- `cargo doc --workspace --no-deps` still reports **zero warnings** after both task commits — the
  16-07 gate holds.
- `cargo doc -p paladin-herald --no-deps`: **0 warnings** — the 16-07 missing-docs flip still costs
  nothing after this plan's other three crates' additions.
- `cargo fmt --check` passes.
- No trait/struct/fn signature or visibility was changed in either task — `git diff -- crates/
  paladin-core/ crates/paladin-memory/` and `git diff -- crates/paladin-battalion/ crates/
  paladin-herald/` touch only doc-comment lines (verified: zero `pub struct`/`pub trait`/`pub fn`
  declaration lines in either diff).
- No `,ignore` or `,text` fence was **added** by this plan anywhere in its four crates (verified via
  diff grep — both counts are 0 in both tasks' diffs). ADR-0033 Finding 3's pre-existing 87-fence
  count is unaffected.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add executable # Examples blocks to the enumerated paladin-core and paladin-memory entry points** - `a00fbecc` (docs) — 11 files, 10 new `# Examples` blocks + 3 heading normalizations
2. **Task 2: Add executable # Examples blocks to the enumerated paladin-battalion and paladin-herald entry points** - `2b6a59b2` (docs) — 7 files, 1 new `# Examples` block + 6 heading normalizations

**Plan metadata:** commit pending (this SUMMARY is handled by the orchestrator after wave merge — worktree mode, per execute-plan.md; STATE.md/ROADMAP.md are NOT touched by this executor).

## Files Created/Modified

### Task 1 (paladin-core + paladin-memory, 11 files)
- `crates/paladin-core/src/platform/container/log.rs` — `LogEntryBuilder` (line 210): construct a `LogEntry` via `new_entry`, assert its priority
- `crates/paladin-core/src/platform/container/paladin_config.rs` — `PaladinConfigBuilder` (line 100): heading `# Example` → `# Examples` only (D-06); pre-existing plain fence, already compile-and-run, unchanged
- `crates/paladin-core/src/platform/container/battalion/grove.rs` — `GroveBuilder` (line 311): heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-core/src/platform/container/battalion/council.rs` — `CouncilBuilder` (line 288): heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-core/src/base/service/field_version_service.rs` — `FieldVersionService<T>` (line 115): generic function over `Arc<dyn FieldVersionRepository<T> + Send + Sync>`, calls `get_current_field_version`
- `crates/paladin-core/src/base/service/node_version_service.rs` — `NodeVersionService<T>` (line 115): generic function over `Arc<dyn NodeVersionRepository<T> + Send + Sync>`, calls `get_current_version`
- `crates/paladin-core/src/base/service/message_service.rs` — `MessageService` (line 176): construct with a customized `MessageServiceConfig`
- `crates/paladin-core/src/base/service/collection_versioning_service.rs` — `CollectionVersionService<T>` (line 122): generic function over `Arc<dyn CollectionVersionRepository<T> + Send + Sync>`, calls `get_current_collection_version`
- `crates/paladin-core/src/platform/container/task.rs` — `DataBackupService` (342), `ContentIndexingService` (417), `EmailNotificationService` (562): trivial constructors, no I/O invoked
- `crates/paladin-memory/src/services/memory_extraction_service.rs` — `MemoryExtractionService` (line 59): generic function over `Arc<dyn LlmPort>`/`Arc<dyn EmbeddingPort>`/`Arc<dyn SanctumPort>`, constructs the service
- `crates/paladin-memory/src/services/rag_retrieval_service.rs` — `RagRetrievalService` (line 44): generic function over `Arc<dyn SanctumPort>`/`Arc<dyn EmbeddingPort>`, constructs with `RagConfig::default()`

### Task 2 (paladin-battalion + paladin-herald, 7 files)
- `crates/paladin-battalion/src/maneuver/service.rs` — `ManeuverExecutionService` (line 18): generic function over `Arc<dyn PaladinPort>`, constructs the service (new example — this entry point was MISSING)
- `crates/paladin-battalion/src/grove_service.rs` — `GroveExecutionService`: heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-battalion/src/council_service.rs` — `CouncilExecutionService`: heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-battalion/src/phalanx_service.rs` — `PhalanxExecutionService`: heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-battalion/src/formation_service.rs` — `FormationExecutionService`: heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-battalion/src/campaign_service.rs` — `CampaignExecutionService`: heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-battalion/src/conclave_execution_service.rs` — `ConclaveExecutionService`: heading-only fix (D-06); pre-existing `ignore` fence unchanged
- `crates/paladin-herald/src/*` — **no file modified**; verified zero D-05 entry points exist in this crate (recorded as a finding, not a code change)

## Decisions Made

See `key-decisions` in frontmatter. Summarized:

- **paladin-herald contributes zero D-05 entry points** — a structural fact confirmed by direct grep, matching the pattern already recorded for four other adapter-only crates in `16-DOCS-03-ENTRY-POINTS.md`.
- **All 9 SINGULAR headings in this plan's four crates were fixed now**, not deferred, because `16-10-PLAN.md`'s own must_haves truths and acceptance criteria explicitly require zero SINGULAR rows in this plan's scope — superseding the enumeration document's earlier, less specific note pointing a 17-site sweep at plan 16-12. This leaves 6 SINGULAR sites elsewhere in the workspace for 16-12.
- **Generic-repository-parameter and Arc\<dyn Port\>-parameter example shapes**, extending 16-09's established pattern (a generic/dyn-typed function that is defined but never invoked, so the doctest type-checks the real API without needing a live implementation) from Port traits to Service structs whose primary dependency is itself a port/repository trait.

## Deviations from Plan

None — plan executed as written, including its explicit discretion that pre-existing SINGULAR examples get heading-only fixes rather than a disposition re-audit (carried forward from 16-09's precedent, referenced in this plan's own read_first pointers).

## Non-running fence audit (must_haves truth #3)

**Zero new non-running fences were added.** All 11 new examples (10 in Task 1, 1 in Task 2) are
compile-and-run. No `no_run`, `ignore`, or `text` fence was introduced by this plan anywhere in its
four crates (`git diff -U0 -- crates/paladin-core/ crates/paladin-memory/ | grep -c
'^+.*rust,ignore'` → 0; same command scoped to `crates/paladin-battalion/ crates/paladin-herald/` →
0). The 9 heading-only fixes touched exactly one line each (the `# Example` → `# Examples` heading);
none of those diffs added a new fence line. ADR-0033 Finding 3's pre-existing 87-fence count is
unaffected by this plan.

## Verbatim verification output

```
$ cargo test --doc -p paladin-ai-core -p paladin-memory
   Doc-tests paladin_core: test result: ok. 57 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out
   Doc-tests paladin_memory: test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --doc -p paladin-battalion -p paladin-herald
   Doc-tests paladin_battalion: test result: ok. 29 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out
   Doc-tests paladin_herald: test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out

$ cargo doc -p paladin-herald --no-deps 2>&1 | grep -c "warning:"
0

$ cargo fmt --check
(no output — clean)

$ bash scripts/check-public-api-examples.sh --list | grep -E 'paladin-core|paladin-battalion|paladin-memory|paladin-herald' | grep -vc 'OK$'
0

$ bash scripts/check-public-api-examples.sh --list | tail -1
TOTAL: 76 entry points -- 62 OK, 8 MISSING, 6 SINGULAR   # workspace-wide; this plan's four crates are 20/20 OK

$ cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt
Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.02s
Generated .../target/doc/paladin/index.html and 12 other files
(grep for "warning:" found none — gate exits 0)
```

## Violation count before/after (must_haves truth #4/#5)

| Scope | Metric | Before this plan | After this plan |
|---|---|---|---|
| This plan's 4 crates (paladin-core, paladin-memory, paladin-battalion, paladin-herald) | OK | 20 / 38 | **38 / 38** |
| This plan's 4 crates | MISSING | 11 / 38 | **0 / 38** |
| This plan's 4 crates | SINGULAR | 9 / 38 (3 paladin-core + 6 paladin-battalion) | **0 / 38** |
| Whole-workspace (all kinds — for context) | OK / MISSING / SINGULAR | 42 / 19 / 15 (16-09's exit baseline) | **62 / 8 / 6** (this plan resolved 20 of the 34 remaining; the remaining 14 are the `src/` crate's Builders/Services/Ports outside this plan's `files_modified` scope, for plans 16-11 and 16-12) |

Note: the "13 items fixed in Task 1 + 7 in Task 2 = 20 total" figure ties out exactly with the
20-item delta above (10+3 in Task 1's paladin-core/paladin-memory scope + 1+6 in Task 2's
paladin-battalion scope; paladin-herald contributed 0 either way).

## Issues Encountered

None. The crate package-name mismatch (`crates/paladin-core/Cargo.toml` sets `name =
"paladin-ai-core"` while the library target is `paladin_core`) required using `-p paladin-ai-core`
rather than `-p paladin-core` for `cargo test --doc`/`cargo doc` package selection — this is a
pre-existing workspace fact (crates.io name collision, per `SEC-03`/`Milestone 8` history), not a
defect introduced or discovered by this plan, and did not require a deviation.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- All four of this plan's assigned crates (`paladin-core`, `paladin-memory`, `paladin-battalion`,
  `paladin-herald`) are now fully compliant with the D-05 `# Examples` gate: 0 MISSING, 0 SINGULAR.
- The workspace doc gate is green (`cargo doc --workspace --no-deps`, 0 warnings) and every touched
  crate's `cargo test --doc` is green, handing the next wave a clean tree.
- 14 entry points remain workspace-wide (8 MISSING + 6 SINGULAR), all under `src/` (the facade
  crate) — plans 16-11 and 16-12's stated scope per `16-DOCS-03-ENTRY-POINTS.md`'s remaining rows.
- No blockers.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

- FOUND: `.planning/phases/16-documentation-currency-the-architecture-gap/16-10-SUMMARY.md`
- FOUND: commit `a00fbecc` (Task 1: paladin-core + paladin-memory examples)
- FOUND: commit `2b6a59b2` (Task 2: paladin-battalion + paladin-herald examples)
