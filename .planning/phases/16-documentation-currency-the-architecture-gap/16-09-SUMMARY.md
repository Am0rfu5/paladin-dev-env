---
phase: 16-documentation-currency-the-architecture-gap
plan: 09
subsystem: docs
tags: [rustdoc, doctests, paladin-ports, hexagonal-architecture, DOCS-03]

# Dependency graph
requires:
  - phase: 16-07
    provides: "Workspace-wide cargo doc gate held at zero warnings"
  - phase: 16-08
    provides: "16-DOCS-03-ENTRY-POINTS.md (D-05 enumeration) and scripts/check-public-api-examples.sh (the gate)"
provides:
  - "Executable # Examples blocks on all 19 previously-MISSING crates/paladin-ports/ *Port traits"
  - "Plural heading normalization (D-06) on the 2 SINGULAR paladin-ports *Port traits"
  - "All 35 *Port entry points in crates/paladin-ports/ now report OK under scripts/check-public-api-examples.sh"
affects: ["16-10", "16-11", "16-12"]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Trait-doc # Examples take an already-implementing &dyn Trait (or a generic bound, for
       traits with generic methods that are not object-safe) as a parameter and call its
       methods with ? propagation — never constructing a full local mock struct. This matches
       the crate's established shape (embedding_port.rs/arsenal_port.rs) and lets the doctest
       compile-and-run without needing a live adapter, since the function is never invoked from
       a `main`; rustdoc only needs the body to type-check and execute without panicking."
    - "Combinator/marker traits (FullFileStoragePort, FullQueuePort) are documented by writing a
       generic/dyn function that calls one method from a constituent supertrait, with the # Examples
       prose explicitly noting the combinator adds no methods of its own and each constituent
       documents its own call pattern separately."

key-files:
  created: []
  modified:
    - crates/paladin-ports/src/output/auth_port.rs
    - crates/paladin-ports/src/output/file_storage_port.rs
    - crates/paladin-ports/src/output/log_port.rs
    - crates/paladin-ports/src/output/orchestrator_port.rs
    - crates/paladin-ports/src/output/paladin_executor_port.rs
    - crates/paladin-ports/src/output/queue_port.rs
    - crates/paladin-ports/src/output/scheduler_port.rs
    - crates/paladin-ports/src/output/streaming_executor_port.rs
    - crates/paladin-ports/src/output/user_repository_port.rs
    - crates/paladin-ports/src/output/vision_port.rs
    - crates/paladin-ports/src/output/workflow_repository_port.rs
    - crates/paladin-ports/src/input/content_input_port.rs
    - crates/paladin-ports/src/input/document_port.rs
    - crates/paladin-ports/src/input/ml_port.rs

key-decisions:
  - "Every new example is compile-and-run (plain ```rust fence), zero no_run: the
     &dyn-Trait-parameter shape means the example body never actually executes at doctest
     runtime (it defines a function and never calls it), so there is no live-I/O dependency
     that would force no_run — the plan's own discretion decision permits no_run only where
     I/O is genuinely unavoidable, and here it never is."
  - "UserRepositoryPort and WorkflowRepositoryPort, initially assessed as no_run candidates
     (Node<UserData> / Workflow domain-object construction looked complex), turned out to have
     simple public constructors (Node::new, OrchestrationContext::new) once inspected — resolved
     as compile-and-run instead of taking the easier no_run path."

requirements-completed: [DOCS-03]

coverage:
  - id: D1
    description: "16 output-side crates/paladin-ports/ *Port traits (AuthPort, BatchFileStoragePort,
       AdvancedFileStoragePort, FileVersioningPort, FullFileStoragePort, LogPort, OrchestratorPort,
       TypedQueuePort, BatchQueuePort, PriorityQueuePort, QueueManagementPort, FullQueuePort,
       SchedulerPort, UserRepositoryPort, VisionPort, WorkflowRepositoryPort) gain executable
       # Examples blocks"
    requirement: "DOCS-03"
    verification:
      - kind: unit
        ref: "cargo test --doc -p paladin-ports"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (zero MISSING under crates/paladin-ports/src/output/)"
        status: pass
    human_judgment: false
  - id: D2
    description: "2 output-side SINGULAR heading sites (PaladinExecutorPort, StreamingExecutorPort)
       normalized to plural '# Examples' (D-06)"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (zero SINGULAR under crates/paladin-ports/src/output/)"
        status: pass
    human_judgment: false
  - id: D3
    description: "3 input-side crates/paladin-ports/ *Port traits (ContentIngestionPort,
       DocumentPort, MlPort) gain executable # Examples blocks, closing the crate's MISSING/SINGULAR
       count to zero across all 35 enumerated *Port traits"
    requirement: "DOCS-03"
    verification:
      - kind: unit
        ref: "cargo test --doc -p paladin-ports"
        status: pass
      - kind: other
        ref: "bash scripts/check-public-api-examples.sh --list (35/35 crates/paladin-ports/ Port rows OK)"
        status: pass
    human_judgment: false
  - id: D4
    description: "Workspace doc gate (cargo doc --workspace --no-deps, ci.yml:63) still reports
       zero warnings after all additions — 16-07's bar held"
    requirement: "DOCS-03"
    verification:
      - kind: other
        ref: "cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q 'warning:' /tmp/doc-output.txt"
        status: pass
    human_judgment: false

duration: 55min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 09: Executable Examples for paladin-ports *Port Traits Summary

**Added compile-and-run `# Examples` blocks to all 19 previously-MISSING `*Port` traits in `crates/paladin-ports/` and normalized the 2 remaining SINGULAR headings, taking the crate's D-05 `*Port` compliance from 19/35 OK to 35/35 OK.**

## Performance

- **Duration:** 55 min
- **Started:** 2026-08-24T12:31:00Z (approx.)
- **Completed:** 2026-08-24T13:26:21Z
- **Tasks:** 2 (Task 1: output-side ports; Task 2: input-side ports)
- **Files modified:** 14

## Accomplishments
- Every `*Port` trait enumerated in `16-DOCS-03-ENTRY-POINTS.md` under `crates/paladin-ports/` now carries a plural `# Examples` block: `bash scripts/check-public-api-examples.sh --list` reports **0 MISSING, 0 SINGULAR** for all 35 `*Port` rows scoped to that crate (previously 19 MISSING, 2 SINGULAR — this plan's opening baseline).
- Every new example is **compile-and-run** — `cargo test --doc -p paladin-ports` passes with 116 doctests passing, 0 failed (94 `ignored`/`- compile` are pre-existing, untouched by this plan; ADR-0033 Finding 3's 87-fence baseline is unchanged, not grown).
- `cargo doc --workspace --no-deps` still reports **zero warnings** — the 16-07 gate holds after this plan's additions.
- `cargo fmt --check` passes.
- No trait signature, method, or visibility was changed — `git diff -- crates/paladin-ports/` touches only doc-comment lines (verified: zero `pub trait`/`pub fn`/`pub struct` lines in the diff).
- No `,ignore` or `,text` fence was added anywhere in `crates/paladin-ports/` (verified via diff grep — both counts are 0).

## Task Commits

Each task was committed atomically:

1. **Task 1: Add executable # Examples blocks to the output-side port traits** - `34b3583d` (docs) — 11 files, 16 new `# Examples` blocks + 2 heading normalizations
2. **Task 2: Add executable # Examples blocks to the input-side port traits and any remaining enumerated ports** - `507f9e3e` (docs) — 3 files, 3 new `# Examples` blocks

**Plan metadata:** commit pending (this SUMMARY + STATE.md/ROADMAP.md are handled by the orchestrator after wave merge — worktree mode, per execute-plan.md).

## Files Created/Modified

### Task 1 (output-side, 11 files)
- `crates/paladin-ports/src/output/auth_port.rs` — `AuthPort` (line 79): issue/verify/revoke token round-trip
- `crates/paladin-ports/src/output/file_storage_port.rs` — `BatchFileStoragePort` (1261), `AdvancedFileStoragePort` (1301), `FileVersioningPort` (1363), `FullFileStoragePort` (1419, combinator)
- `crates/paladin-ports/src/output/log_port.rs` — `LogPort` (239): write a `LogEntry`, then count entries
- `crates/paladin-ports/src/output/orchestrator_port.rs` — `OrchestratorPort` (253): queue an item through the agent→orchestrator bridge
- `crates/paladin-ports/src/output/paladin_executor_port.rs` — `PaladinExecutorPort` (60): heading `# Example` → `# Examples` only (D-06); disposition (`no_run`) unchanged, pre-existing
- `crates/paladin-ports/src/output/queue_port.rs` — `TypedQueuePort` (638), `BatchQueuePort` (693), `PriorityQueuePort` (754), `QueueManagementPort` (794), `FullQueuePort` (853, combinator)
- `crates/paladin-ports/src/output/scheduler_port.rs` — `SchedulerPort` (253): schedule a cron job, read back its status
- `crates/paladin-ports/src/output/streaming_executor_port.rs` — `StreamingExecutorPort` (66): heading `# Example` → `# Examples` only (D-06); disposition (`no_run`) unchanged, pre-existing
- `crates/paladin-ports/src/output/user_repository_port.rs` — `UserRepositoryPort` (41): construct a `User` (`Node<UserData>`) and save it
- `crates/paladin-ports/src/output/vision_port.rs` — `VisionPort` (91): check model support, analyze an image
- `crates/paladin-ports/src/output/workflow_repository_port.rs` — `WorkflowRepositoryPort` (145): construct a `Workflow`, persist a pending checkpoint

### Task 2 (input-side, 3 files)
- `crates/paladin-ports/src/input/content_input_port.rs` — `ContentIngestionPort` (23): fetch then ingest round-trip
- `crates/paladin-ports/src/input/document_port.rs` — `DocumentPort` (131): ingest a document, chunk it
- `crates/paladin-ports/src/input/ml_port.rs` — `MlPort` (119): check model availability, run a prediction

## Decisions Made

- **Example shape:** every new `# Examples` block is a small `async fn`/`fn` that accepts an
  already-implementing `&dyn Trait` (or, for traits made non-object-safe by a generic method —
  `TypedQueuePort<T>`, `BatchQueuePort`, `PriorityQueuePort`, `FullQueuePort` — a generic type
  parameter bound by the trait) and calls 1–3 of its methods with `?` error propagation. This
  matches the pattern the plan's `read_first` pointed at (`embedding_port.rs`/`arsenal_port.rs`'s
  own already-passing examples) rather than authoring a full local mock struct per trait. Because
  the function is defined but never invoked from a rustdoc-synthesized `main`, the doctest still
  fully type-checks the call against the trait's real signature (catching broken imports/wrong
  argument types) without needing any I/O, live adapter, or runtime.
- **Disposition: 100% compile-and-run, zero new `no_run`.** The shape above never needs live I/O,
  so every one of the 19 new examples is a plain ```rust``` fence. Two candidates that looked like
  plausible `no_run` cases at first read turned out not to be, once the domain types were actually
  inspected:
  - `UserRepositoryPort`'s `User` is `Node<UserData>` — `Node::new(data, None)` is a public,
    trivial constructor; no versioning-system ceremony was actually required.
  - `WorkflowRepositoryPort`'s `Workflow` looked like it might need a populated job/listener graph,
    but every field accepts an empty `Vec` and `OrchestrationContext::new(initiator, environment)`
    is a two-argument constructor — a syntactically valid, if semantically empty, `Workflow` value
    is cheap to build.
- **Combinator traits (`FullFileStoragePort`, `FullQueuePort`) get a real example, not a `no_run`
  punt:** each is `Send + Sync` plus 3–4 supertraits with no methods of its own. Both `FullFileStoragePort`
  and `FullQueuePort` remain object-safe or generic-bindable respectively (the file-storage
  supertraits are all object-safe; the queue supertraits include two with generic methods, so
  `FullQueuePort`'s example uses a generic bound instead of `dyn`), so a real call through the
  combined trait is possible. The `# Examples` prose explicitly notes the combinator itself adds
  no methods and that each constituent documents its own pattern separately, to avoid the example
  reading as if `FullFileStoragePort`/`FullQueuePort` had unique behavior of their own.
- **Heading-only fix, disposition untouched, on `PaladinExecutorPort`/`StreamingExecutorPort`:**
  these 2 traits already carried a working `no_run` example predating this plan; D-06 only asks
  for heading normalization (`# Example` → `# Examples`), not a disposition re-audit of pre-existing
  examples outside this plan's MISSING scope. Their `no_run` fences are untouched and are **not**
  new examples this plan authored — they are excluded from the "19 new, 0 no_run" figure above.

## Deviations from Plan

None — plan executed exactly as written. No Rule 1–4 auto-fixes were needed: every port's
supporting types (`AuthClaims`, `Embedding`-family, `User`/`Node<UserData>`, `Workflow`,
`QueueItem`, `Message`, `LogEntry`, etc.) already had the public constructors the examples needed;
no missing functionality or bug was uncovered in first-party code during this plan.

## Non-running fence audit (must_haves truth #3)

**Zero new non-running fences were added.** All 19 new examples are compile-and-run. No `no_run`,
`ignore`, or `text` fence was introduced by this plan anywhere in `crates/paladin-ports/`
(`git diff -U0 -- crates/paladin-ports/ | grep -c '^+.*rust,ignore'` → 0;
`... | grep -c '^+.*```text'` → 0). ADR-0033 Finding 3's pre-existing count of 87
`ignore`/`no_run`/`text` fences elsewhere in the codebase is unaffected — this plan did not touch
any of those sites.

## Verbatim verification output

```
$ cargo test --doc -p paladin-ports
test result: ok. 116 passed; 0 failed; 94 ignored; 0 measured; 0 filtered out; finished in 0.03s
all doctests ran in 1.62s; merged doctests compilation took 1.59s

$ cargo fmt --check
(no output — clean)

$ bash scripts/check-public-api-examples.sh --list | grep "crates/paladin-ports" | grep -vE "OK$"
(no output — all 35 Port rows OK)

$ bash scripts/check-public-api-examples.sh --list | tail -1
TOTAL: 76 entry points -- 42 OK, 19 MISSING, 15 SINGULAR   # workspace-wide; paladin-ports-only is 35/35 OK

$ cargo doc --workspace --no-deps 2>&1 | tee /tmp/doc-output.txt && ! grep -q "warning:" /tmp/doc-output.txt
Finished `dev` profile [unoptimized + debuginfo] target(s) in 4m 02s
Generated .../target/doc/paladin/index.html and 12 other files
(grep for "warning:" found none — gate exits 0)
```

## Violation count before/after (must_haves truth #4)

| Scope | Metric | Before this plan | After this plan |
|---|---|---|---|
| `crates/paladin-ports/` `*Port` rows | OK | 16 / 35 | **35 / 35** |
| `crates/paladin-ports/` `*Port` rows | MISSING | 19 / 35 | **0 / 35** |
| `crates/paladin-ports/` `*Port` rows | SINGULAR | 2 / 35 (Paladin/StreamingExecutorPort) | **0 / 35** |
| Whole-workspace (all kinds — carried from 16-08 baseline for context) | MISSING+SINGULAR | 38 MISSING + 17 SINGULAR = 55 | 19 MISSING + 15 SINGULAR = 34 (this plan resolved 21 of them; remaining 34 are Builders/Services outside this plan's `crates/paladin-ports/` scope, for plans 16-10 through 16-12) |

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- The framework's "primary integration contract" (`crates/paladin-ports/`) is now fully documented
  with executable examples across all 35 `*Port` traits — DOCS-03's stated value for this crate is
  delivered.
- Plans 16-10 through 16-12 (per the D-05 enumeration's remaining scope: Builders and `*Service`
  structs, plus the wider D-06 heading-normalization sweep across 17 total SINGULAR sites, only 2
  of which were in `crates/paladin-ports/` and are now resolved) can proceed independently — this
  plan touched only `crates/paladin-ports/src/output/` and `crates/paladin-ports/src/input/`, per
  its declared `files_modified` scope, and made no changes to any other crate.
- No blockers. The workspace doc gate is green and `cargo test --doc -p paladin-ports` is green,
  handing the next wave a clean tree per the plan's stated concurrency-edge intent.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

- FOUND: `.planning/phases/16-documentation-currency-the-architecture-gap/16-09-SUMMARY.md`
- FOUND: commit `34b3583d` (Task 1: output-side port examples)
- FOUND: commit `507f9e3e` (Task 2: input-side port examples)
