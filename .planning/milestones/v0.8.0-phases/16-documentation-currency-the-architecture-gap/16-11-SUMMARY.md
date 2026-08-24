---
phase: 16-documentation-currency-the-architecture-gap
plan: 11
subsystem: docs
tags: [rustdoc, doctests, paladin-llm, paladin-storage, paladin-web, paladin-content, paladin-notifications, facade, DOCS-03]

# Dependency graph
requires:
  - phase: 16-07
    provides: "Workspace-wide cargo doc gate held at zero warnings"
  - phase: 16-08
    provides: "16-DOCS-03-ENTRY-POINTS.md (D-05 enumeration) and scripts/check-public-api-examples.sh (the gate)"
  - phase: 16-10
    provides: "38/38 D-05 entry points OK across paladin-core, paladin-memory, paladin-battalion,
       paladin-herald; confirmed the Arc<dyn Port>-parameter and generic-repository-parameter
       example shapes; left 8 MISSING + 6 SINGULAR under src/ (plus 1 MISSING in paladin-llm) for
       16-11/16-12"
provides:
  - "Zero MISSING D-05 entry points tree-wide (76/76 have an example block) — the last 8 MISSING
     rows closed: 1 in crates/paladin-llm/ (LlmAnalysisService), 7 under src/ (ProgressBarBuilder,
     PromptBuilder, DefaultContentIngestionService, TemperatureService, ContentItemService,
     EventService, UserService)"
  - "Confirmed structural fact: paladin-storage, paladin-web, paladin-content and
     paladin-notifications contribute zero D-05 entry points (adapter-only crates); src/config/
     also contributes zero"
  - "6 remaining SINGULAR rows in src/ recorded and left untouched for plan 16-12's D-06 sweep,
     per this plan's explicit instruction"
affects: ["16-12"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Extended the Arc<dyn Port>-parameter shape (16-09/16-10) to LlmAnalysisService,
       TemperatureService and UserService: a function accepting the already-implementing
       Arc<dyn LlmPort>/Arc<dyn UserRepositoryPort>+Arc<dyn LogPort> constructs the service via
       ::new(), never invoked."
    - "Extended the generic-repository-parameter shape (16-10) to ContentItemService: a function
       accepting Arc<dyn NodeVersionRepository<ContentData> + Send + Sync> constructs the service,
       matching the NodeVersionService<T> precedent exactly (ContentItemService wraps that same
       generic service internally)."
    - "New: concrete-collaborator-construction shape, where a service's constructor needs another
       concrete facade type (not just a port) that is itself cheap and I/O-free to build inline —
       used for DefaultContentIngestionService (Arc<dyn ContentRepository> + a real
       Orchestrator::new()) and UserService (Arc<dyn Port> params + a real, in-memory
       NotificationService::new(NotificationServiceConfig::default(), message_service)). Verified
       by reading each constructed type's ::new() body to confirm it performs no network,
       filesystem or blocking I/O before writing the example — construction only allocates
       Arc<Mutex/RwLock<...>>-backed in-memory state."
    - "New: async-but-offline example shape for EventService — EventService::new is
       `pub async fn`, but its only await point is MessageService::register_handler, which writes
       to an in-memory RwLock<HashMap<..>> with no I/O. Written as a #[tokio::main] doctest that
       actually calls and awaits the constructor (not just defines an uncalled fn), since doing so
       introduces no I/O risk and demonstrates the real async signature end-to-end."
    - "New: no-invoke-of-blocking-call shape for PromptBuilder — the builder itself
       (`PromptBuilder::input(...)`, `PromptBuilder::confirm(...)`) is pure construction, but the
       terminal `.prompt()` call on the returned type blocks on stdin (dialoguer). The example
       constructs two prompt objects and stops there, documented inline as intentional (T-16-26)
       rather than reached for `no_run` to dodge a TTY dependency."
    - "cli-feature entry points (ProgressBarBuilder, PromptBuilder) rely on the crate's existing
       #[cfg(feature = \"cli\")] gate at the module-declaration site (src/application/mod.rs) —
       no new cfg attribute was needed on the doc comment itself, since the entire enclosing
       module is already gated. Verified separately with `cargo test --doc -p paladin-ai
       --features cli` (119 passed) since default-feature `cargo test --doc -p paladin-ai`
       (101 passed) never compiles this module at all."

key-files:
  created: []
  modified:
    - crates/paladin-llm/src/llm_analysis_service.rs
    - src/application/cli/formatters/progress.rs
    - src/application/cli/interactive/prompts.rs
    - src/application/services/content/content_ingestion_service.rs
    - src/application/services/paladin/temperature_service.rs
    - src/core/platform/manager/content_service.rs
    - src/core/platform/manager/event_manager.rs
    - src/core/platform/manager/user_service.rs

key-decisions:
  - "Task 1's real scope reduced to a single entry point. The plan's read_first and action text
     describe extensive work across paladin-llm, paladin-storage, paladin-web, paladin-content and
     paladin-notifications, but 16-DOCS-03-ENTRY-POINTS.md's own 'Crate coverage note' (and this
     plan's prior-wave context) already recorded that four of those five crates contribute zero
     D-05 entry points — they hold adapter implementations of *Port traits declared elsewhere, not
     their own *Builder/*Port/*Service declarations. Re-verified by direct grep
     (`grep -rnE '^\\s*pub (struct [A-Za-z0-9_]*(Builder|Service)\\b|trait
     [A-Za-z0-9_]*Port\\b)' crates/paladin-storage/src crates/paladin-web/src
     crates/paladin-content/src crates/paladin-notifications/src` — zero matches) and by running
     `cargo test --doc` on all four (0 tests each, confirming no doctest-bearing example exists
     or is needed). Task 1's actual work was therefore one # Examples block on
     LlmAnalysisService."
  - "src/config/ also contributes zero D-05 entry points, confirmed by the same grep pattern
     applied to that directory. Not previously stated explicitly in
     16-DOCS-03-ENTRY-POINTS.md's enumeration table (which predates this plan and only lists
     src/application/ and src/core/ facade rows) — recorded here as a fresh finding."
  - "Followed the plan's explicit instruction to leave the 6 pre-existing SINGULAR rows in src/
     (PaladinBuilder, ArsenalRegistryService, ArsenalExecutionService, HandoffService,
     PaladinExecutionService, EncryptionService) untouched rather than sweeping them, even though
     the plan's own prose also says the gating-mode script 'must exit 0.' These two instructions
     are in tension: `bash scripts/check-public-api-examples.sh` (no --list) exits 1 on any
     non-OK row, SINGULAR included (confirmed by reading the script: `status=\"SINGULAR\"` still
     reaches the `exit 1` path used when `${#violations[@]} -gt 0`). The plan's own
     acceptance_criteria/<verify> block for Task 2 is authoritative and does not invoke the
     gating-mode exit code — it uses `--list` (which always exits 0) plus a grep for `MISSING`.
     Zero MISSING is satisfied; the 6 SINGULAR rows are left for 16-12 per the explicit
     'leave them for that plan rather than sweeping them here' instruction. Recorded as the
     literal ground truth rather than silently reconciling the prose."
  - "The plan's own <verify> command for Task 2, `! bash scripts/check-public-api-examples.sh
     --list | grep -q 'MISSING'`, has a false-negative-prone substring match: the script's own
     TOTAL line (`TOTAL: 76 entry points -- 70 OK, 0 MISSING, 6 SINGULAR`) itself contains the
     literal substring `MISSING`, so this exact command reports failure even when the true
     MISSING count is zero. Verified independently with a per-row check
     (`... --list | grep -v '^TOTAL' | awk -F'\\t' '\\$4 ~ /^MISSING/'` returns nothing) —
     zero true MISSING rows tree-wide, confirming the plan's underlying intent is met even though
     its literal verify command as written would report otherwise."
  - "EventService's example calls and awaits the constructor inside a #[tokio::main] doctest
     (compile-and-run), rather than only defining an uncalled async fn as some Port-trait examples
     do — because EventService::new is a concrete constructor (not a trait method against an
     unknown implementor) and its only I/O-shaped operation (register_handler) is a provably
     in-memory RwLock write, verified by reading the implementation before writing the example."

requirements-completed: [DOCS-03]

coverage:
  - id: T1
    description: "LlmAnalysisService (crates/paladin-llm/src/llm_analysis_service.rs:54), the sole
       D-05 entry point among the five previously example-free crates, gains an executable
       # Examples block using the Arc<dyn LlmPort>-parameter shape"
    requirement: "DOCS-03"
    verification:
      - kind: unit
        ref: "cargo test --doc -p paladin-llm"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (zero MISSING/SINGULAR under crates/paladin-llm/)"
        status: pass
    human_judgment: false
  - id: T2
    description: "paladin-storage, paladin-web, paladin-content and paladin-notifications
       independently re-confirmed to contribute zero D-05 entry points; no code changed in any of
       the four, their existing doctests (0 each) still pass"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "grep -rnE '^\\s*pub (struct [A-Za-z0-9_]*(Builder|Service)\\b|trait [A-Za-z0-9_]*Port\\b)' crates/paladin-storage/src crates/paladin-web/src crates/paladin-content/src crates/paladin-notifications/src (zero matches)"
        status: pass
      - kind: unit
        ref: "cargo test --doc -p paladin-storage -p paladin-web -p paladin-content -p paladin-notifications"
        status: pass
    human_judgment: false
  - id: T3
    description: "7 remaining MISSING D-05 entry points under src/ (ProgressBarBuilder,
       PromptBuilder, DefaultContentIngestionService, TemperatureService, ContentItemService,
       EventService, UserService) gain executable # Examples blocks"
    requirement: "DOCS-03"
    verification:
      - kind: unit
        ref: "cargo test --doc -p paladin-ai (default features, 101 passed) and cargo test --doc -p paladin-ai --features cli (119 passed, covers ProgressBarBuilder/PromptBuilder)"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (0 MISSING tree-wide, per-row check excluding the TOTAL line)"
        status: pass
    human_judgment: false
  - id: T4
    description: "src/config/ confirmed to contribute zero D-05 entry points"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "grep -rnE '^\\s*pub (struct [A-Za-z0-9_]*(Builder|Service)\\b|trait [A-Za-z0-9_]*Port\\b)' src/config/ (zero matches)"
        status: pass
    human_judgment: false
  - id: T5
    description: "No credential-shaped literal, no ignore/text fence, no pub signature/visibility
       change, and no singular heading anywhere in this plan's diff"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "git diff -U0 -- crates/ src/ | grep -cE '(^\\+.*# Example$|^\\+.*rust,ignore|^\\+.*,text|^\\+.*(sk-[A-Za-z0-9]|api_key *= *\"[A-Za-z0-9]))' -> 0; git diff -- crates/ src/ | grep '^\\+.*pub (struct|trait|fn) ' -> none"
        status: pass
    human_judgment: false
  - id: T6
    description: "Workspace doc gate (cargo doc --workspace --no-deps) still zero warnings; paladin-web's OpenAPI baseline test (untouched by this plan) still passes"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q 'warning:' /tmp/doc-output.txt"
        status: pass
      - kind: unit
        ref: "cargo test -p paladin-web openapi (6 passed, incl. openapi_matches_committed_baseline)"
        status: pass
    human_judgment: false
  - id: T7
    description: "cargo fmt --check and bash scripts/check-doc-examples.sh both pass"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo fmt --check (clean); bash scripts/check-doc-examples.sh (All included examples compile; README Quick Example is in sync; 0 checked/578 skipped/0 failed for docs/src)"
        status: pass
    human_judgment: false

duration: 65min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 11: Executable Examples for the Five Zero-Example Crates and the Facade's Entry Points Summary

**Closed the last 8 MISSING D-05 entry points tree-wide — 1 in `paladin-llm` (`LlmAnalysisService`) and 7 in the facade's own `src/` (`ProgressBarBuilder`, `PromptBuilder`, `DefaultContentIngestionService`, `TemperatureService`, `ContentItemService`, `EventService`, `UserService`) — taking the D-05 gate to 0 MISSING / 6 SINGULAR (deferred to 16-12) across all 76 enumerated entry points.**

## Performance

- **Duration:** ~65 min
- **Started:** 2026-08-24 (approx.)
- **Completed:** 2026-08-24
- **Tasks:** 2 (Task 1: the five zero-example crates; Task 2: the facade's `src/` entry points)
- **Files modified:** 8

## Accomplishments

- **`LlmAnalysisService` (crates/paladin-llm/src/llm_analysis_service.rs:54)** gains a compile-and-run
  `# Examples` block: a function accepting `Arc<dyn LlmPort>` constructs the service via `::new()`,
  never invoked — the Arc-dyn-Port-parameter shape established by 16-09/16-10.
- **Re-confirmed, not re-derived:** `paladin-storage`, `paladin-web`, `paladin-content` and
  `paladin-notifications` contribute **zero** D-05 entry points. This was already recorded in
  `16-DOCS-03-ENTRY-POINTS.md`'s "Crate coverage note" and the prior-wave context, and this plan's
  own fresh `grep -rnE '^\s*pub (struct [A-Za-z0-9_]*(Builder|Service)\b|trait
  [A-Za-z0-9_]*Port\b)'` across all four crate `src/` trees returns zero matches — confirming the
  structural fact independently rather than trusting the prior record blindly. `cargo test --doc`
  on all four returns `0 passed; 0 failed` in every case (no doctest-bearing content exists, none
  was needed).
- **`src/config/` also contributes zero D-05 entry points** — a fresh finding this plan makes
  explicit (not stated in the pre-existing enumeration table, which only lists `src/application/`
  and `src/core/` rows), confirmed by the same grep pattern.
- **7 new compile-and-run `# Examples` blocks in the facade's `src/`:**
  - `ProgressBarBuilder` (`src/application/cli/formatters/progress.rs:69`) — constructs and
    finishes a progress bar entirely in-memory; offline, instant.
  - `PromptBuilder` (`src/application/cli/interactive/prompts.rs:14`) — constructs two prompt
    objects (`input`, `confirm`) but does not call `.prompt()`, which blocks on a TTY; documented
    inline as intentional (T-16-26).
  - `DefaultContentIngestionService` (`src/application/services/content/content_ingestion_service.rs:240`)
    — `Arc<dyn ContentRepository>`-parameter shape plus a real, in-memory `Orchestrator::new()`.
  - `TemperatureService` (`src/application/services/paladin/temperature_service.rs:51`) —
    `Arc<dyn LlmPort>`-parameter shape.
  - `ContentItemService` (`src/core/platform/manager/content_service.rs:20`) — generic
    `Arc<dyn NodeVersionRepository<ContentData> + Send + Sync>`-parameter shape, the same pattern
    16-10 used for `NodeVersionService<T>` itself (this service wraps that generic service).
  - `EventService` (`src/core/platform/manager/event_manager.rs:69`) — the one example in this
    plan that actually **calls and awaits** its constructor inside a `#[tokio::main]` doctest
    (rather than only defining an uncalled fn), because `EventService::new`'s only await point
    (`MessageService::register_handler`) writes to an in-memory `RwLock<HashMap<..>>` with
    verified-by-reading zero I/O.
  - `UserService` (`src/core/platform/manager/user_service.rs:29`) — `Arc<dyn Port>`-parameter
    shape plus a real, in-memory `NotificationService::new(NotificationServiceConfig::default(),
    message_service)`.
- **`ProgressBarBuilder`/`PromptBuilder` are gated by the crate's pre-existing
  `#[cfg(feature = "cli")]`** at the module-declaration site (`src/application/mod.rs`) — no new
  cfg attribute was needed on the doc comment. Verified separately with `cargo test --doc -p
  paladin-ai --features cli` (119 passed) since the default-feature build (101 passed) never
  compiles this module.
- **`cargo test --doc -p paladin-llm -p paladin-storage -p paladin-web -p paladin-content -p
  paladin-notifications`: all `ok`** (5 + 0 + 0 + 0 + 0 = 5 passed, 0 failed).
- **`cargo test --doc -p paladin-ai`: 101 passed, 0 failed, 17 ignored** (default features).
- **`cargo test --doc -p paladin-ai --features cli`: 119 passed, 0 failed, 17 ignored** (confirms
  the two cli-gated examples run).
- **`bash scripts/check-public-api-examples.sh --list` final totals: `76 entry points -- 70 OK, 0
  MISSING, 6 SINGULAR`** — zero MISSING tree-wide for the first time this phase.
- **`cargo doc --workspace --no-deps`: zero warnings** — the 16-07 gate holds through this plan's
  additions.
- **`cargo test -p paladin-web openapi`: 6 passed**, including `openapi_matches_committed_baseline`
  — confirms the OpenAPI-coupling risk flagged in this plan's prior-wave context did not manifest,
  since this plan made no changes to `paladin-web`.
- **`cargo fmt --check`: clean.**
- **`bash scripts/check-doc-examples.sh`: all included examples compile; README Quick Example is
  in sync; 0 checked/578 skipped/0 failed for `docs/src`.**
- No trait/struct/fn signature or visibility was changed in either task — verified via `git diff
  -- crates/ src/ | grep '^\+.*pub (struct|trait|fn) '` returning no matches in either diff.
- No `,ignore` or `,text` fence, no singular `# Example` heading, and no credential-shaped literal
  was added by this plan anywhere (all four greps return 0).

## Task Commits

Each task was committed atomically:

1. **Task 1: Add # Examples block to the five zero-example crates' sole D-05 entry point** -
   `707061d1` (docs) — 1 file, 1 new `# Examples` block (`LlmAnalysisService`); the other four
   crates were re-verified to need no change.
2. **Task 2: Add # Examples blocks to the facade's own entry points and close the MISSING gate** -
   `11c1aa8d` (docs) — 7 files, 7 new `# Examples` blocks.

**Plan metadata:** commit pending (this SUMMARY is handled by the orchestrator after wave merge —
worktree mode, per execute-plan.md; STATE.md/ROADMAP.md are NOT touched by this executor).

## Files Created/Modified

### Task 1 (1 file)
- `crates/paladin-llm/src/llm_analysis_service.rs` — `LlmAnalysisService` (struct doc block, line
  53): `Arc<dyn LlmPort>`-parameter shape, constructs via `::new()`, never invoked.

### Task 2 (7 files)
- `src/application/cli/formatters/progress.rs` — `ProgressBarBuilder` (line 58): construct,
  configure and finish a progress bar, entirely offline.
- `src/application/cli/interactive/prompts.rs` — `PromptBuilder` (line 6): construct two prompt
  objects; `.prompt()` (blocks on TTY) intentionally not called.
- `src/application/services/content/content_ingestion_service.rs` —
  `DefaultContentIngestionService` (line 240): `Arc<dyn ContentRepository>`-parameter shape +
  real `Orchestrator::new()`.
- `src/application/services/paladin/temperature_service.rs` — `TemperatureService` (line 51):
  `Arc<dyn LlmPort>`-parameter shape.
- `src/core/platform/manager/content_service.rs` — `ContentItemService` (line 20): generic
  `Arc<dyn NodeVersionRepository<ContentData> + Send + Sync>`-parameter shape.
- `src/core/platform/manager/event_manager.rs` — `EventService` (line 69): async example,
  constructor called and awaited under `#[tokio::main]`.
- `src/core/platform/manager/user_service.rs` — `UserService` (line 29): `Arc<dyn Port>`-parameter
  shape + real, in-memory `NotificationService` construction.

## Decisions Made

See `key-decisions` in frontmatter. Summarized:

- **Task 1's real work reduced to one entry point.** The plan's prose describes work across five
  crates, but four of them structurally contribute zero D-05 entry points (adapter
  implementations of ports declared in `paladin-ports`, not their own declarations) — re-verified
  by fresh grep and doctest runs rather than trusted from the prior record alone.
- **`src/config/` also contributes zero D-05 entry points** — recorded here for the first time.
- **The 6 pre-existing SINGULAR rows in `src/` were left untouched**, per the plan's explicit
  "leave them for that plan [16-12]" instruction — even though this creates tension with the same
  task's "the gate must exit 0" prose (the gating-mode script exits 1 on any non-OK row, SINGULAR
  included). The plan's own machine-checkable `<verify>` block for Task 2 is authoritative and
  does not invoke the gating-mode exit code; it checks `--list` (always exit 0) plus a MISSING
  grep, which this plan's changes satisfy.
- **The plan's own literal Task 2 `<verify>` MISSING-grep command is imprecise** — it matches the
  gate script's own `TOTAL: ... 0 MISSING ...` summary line, so it reports failure even at zero
  true MISSING. Verified the true count independently with a per-row parse excluding the TOTAL
  line: zero.
- **EventService's example is the one in this plan that calls and awaits its constructor**, rather
  than only defining an uncalled fn — justified because its single await point is a provably
  in-memory operation, verified by reading the implementation first.

## Deviations from Plan

**1. [Rule 1/interpretation] Task 1 scope was materially smaller than the plan's prose implies.**
The plan's `<read_first>` and `<action>` blocks describe substantial work across
`paladin-storage`, `paladin-web`, `paladin-content` and `paladin-notifications` (mock adapters,
sqlite/in-memory repositories, in-process router construction). None of that work exists to do:
these four crates hold zero D-05 entry points under the enumeration's own selection rule (`pub
*Builder`/`*Port`/`*Service` declared in the crate itself). This was already recorded as a
structural fact in `16-DOCS-03-ENTRY-POINTS.md`'s "Crate coverage note" before this plan started,
and re-confirmed independently here via grep and `cargo test --doc`. No fix was needed; this is
recorded as a scope note, not a bug.

**2. [Rule 1/interpretation] Task 2's SINGULAR rows were left in place despite the plan's "gate
must exit 0" phrasing**, per the plan's own more specific instruction one sentence later ("If it
still reports SINGULAR rows ... leave them for that plan"). Followed the specific instruction and
the machine-checkable `<verify>` block over the general prose. No code change; documented as a
plan-text tension for 16-12's awareness.

No other deviations. No new dependency, no architectural change, no widened API.

## Non-running fence audit (must_haves truth #4)

**Zero `no_run`, `ignore` or `text` fences were added.** All 8 new examples (1 in Task 1, 7 in
Task 2) are plain ` ```rust ` / ` ``` ` compile-and-run fences. `git diff -U0 -- crates/ src/ |
grep -c '^+.*rust,ignore'` and the same for `,text` both return 0 in both tasks' diffs. This is
the highest compile-and-run ratio of any plan in this phase's `# Examples` wave (16-09 was 19/19,
16-10 was 11/11 new, this plan is 8/8 new) — no example in this plan needed a credentialed
provider, a live database, or a bound port, so `no_run` was never reached for. ADR-0033 Finding
3's pre-existing 87-fence count is unaffected.

## Violation count before/after (must_haves truth #5)

| Scope | Metric | 16-08 baseline | Start of 16-11 (post-16-10) | End of 16-11 |
|---|---|---|---|---|
| Whole workspace | OK | — | 62 / 76 | **70 / 76** |
| Whole workspace | MISSING | — | 8 / 76 | **0 / 76** |
| Whole workspace | SINGULAR | — | 6 / 76 | **6 / 76** (unchanged — 16-12's scope) |
| This plan's crates (paladin-llm) | OK | — | 34/35 | **35/35** |
| This plan's crates (src/) | MISSING | — | 7 | **0** |

The running total across 16-09 → 16-11: 16-09 closed 19 examples + 2 heading fixes in
`paladin-ports`; 16-10 closed 11 examples + 9 heading fixes in `paladin-core`/`paladin-memory`/
`paladin-battalion`/`paladin-herald`; this plan (16-11) closed the final 8 MISSING rows tree-wide.
0 MISSING remain anywhere; the 6 remaining SINGULAR rows are 16-12's entire remaining scope.

## Verbatim verification output

```
$ cargo test --doc -p paladin-llm
   Doc-tests paladin_llm
running 5 tests
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --doc -p paladin-storage -p paladin-web -p paladin-content -p paladin-notifications
   Doc-tests paladin_content:       ok. 0 passed; 0 failed
   Doc-tests paladin_notifications: ok. 0 passed; 0 failed
   Doc-tests paladin_storage:       ok. 0 passed; 0 failed
   Doc-tests paladin_web:           ok. 0 passed; 0 failed

$ cargo test --doc -p paladin-ai
test result: ok. 101 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.02s

$ cargo test --doc -p paladin-ai --features cli
test result: ok. 119 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.03s

$ cargo fmt --check
(no output -- clean)

$ bash scripts/check-doc-examples.sh
All included examples compile.
README Quick Example is in sync.
Results: 0 checked, 578 skipped, 0 failed
All doc code examples pass validation.

$ bash scripts/check-public-api-examples.sh --list | tail -1
TOTAL: 76 entry points -- 70 OK, 0 MISSING, 6 SINGULAR

$ cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt
Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.41s
Generated .../target/doc/paladin/index.html and 12 other files
(grep for "warning:" found none -- gate exits 0)

$ cargo test -p paladin-web openapi
test openapi::tests::openapi_matches_committed_baseline ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 111 filtered out; finished in 0.01s
```

## Issues Encountered

- **`NotificationServiceConfig` import path.** First attempt imported it from
  `paladin::application::services::notification_orchestrator`, matching where it is re-exported
  as `NotificationService` (a type alias for `NotificationOrchestrator`) — but
  `NotificationServiceConfig` itself is imported into that module with a private `use`, not
  `pub use`, so it is not reachable from that path. The type actually lives at
  `paladin_core::platform::container::notification::NotificationServiceConfig`, reachable through
  the facade as `paladin::core::platform::container::notification::NotificationServiceConfig`
  (confirmed via the existing test-only imports at `user_service.rs:588` and
  `config/user_config.rs:67`, and via the facade's curated `pub mod container { pub use
  paladin_core::platform::container::notification; ... }` re-export in
  `src/core/platform/mod.rs`). Fixed inline (Rule 3 — blocking import error), re-ran the doctest
  to confirm, and the fix is reflected in the final committed example. Not a signature or
  visibility change to any type — only the doctest's own `use` line.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- **Zero MISSING D-05 entry points remain anywhere in the tree** — this plan closes the entire
  MISSING half of the gate that 16-08 authored.
- **6 SINGULAR rows remain, all in `src/`**, explicitly deferred to plan 16-12 per this plan's own
  instruction: `PaladinBuilder` (`src/application/services/paladin/paladin_builder.rs:77`),
  `ArsenalRegistryService` (`src/application/services/arsenal/arsenal_registry_service.rs:42`),
  `ArsenalExecutionService` (`src/application/services/arsenal/arsenal_execution_service.rs:60`),
  `HandoffService` (`src/application/services/paladin/handoff_service.rs:42`),
  `PaladinExecutionService` (`src/application/services/paladin/paladin_execution_service.rs:105`),
  `EncryptionService` (`src/infrastructure/security/encryption.rs:161`).
- The workspace doc gate is green (`cargo doc --workspace --no-deps`, 0 warnings) and every
  touched crate's `cargo test --doc` is green (default features and, separately, `--features
  cli`), handing plan 16-12 a clean tree with a purely mechanical heading-only task remaining.
- No blockers.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

- FOUND: `.planning/phases/16-documentation-currency-the-architecture-gap/16-11-SUMMARY.md`
- FOUND: commit `707061d1` (Task 1: LlmAnalysisService example)
- FOUND: commit `11c1aa8d` (Task 2: facade entry-point examples)
