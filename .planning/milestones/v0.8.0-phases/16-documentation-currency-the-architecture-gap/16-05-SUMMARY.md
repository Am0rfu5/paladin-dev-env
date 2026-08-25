---
phase: 16-documentation-currency-the-architecture-gap
plan: 05
subsystem: docs
tags: [mdbook, docs-currency, logging, monitoring, performance, troubleshooting, linkcheck]

# Dependency graph
requires:
  - phase: 16-01
    provides: "Pinned mdbook toolchain, the D-09 verdict record seeded with all fourteen files, the eight-class signal battery proven on cicd.md, and the opening linkcheck report"
  - phase: 16-04
    provides: "Ten of fourteen D-09 files settled by content (user-guides + deployment groups), the reusable verdict-row shape"
provides:
  - "The last four of fourteen D-09 files settled by content: logging.md, monitoring.md, performance-tuning.md, troubleshooting.md — closing the docs/src/operations/ group and completing DOCS-01's fourteen-file scope"
  - "logging.md's entire premise corrected: the page described a tracing-crate ecosystem with zero call sites anywhere in the tree; the real facade is log + env_logger wrapped by LogOrchestrator/LogPort/LogDestination"
  - "monitoring.md's Overview corrected: no /metrics endpoint exists (prometheus/opentelemetry are not dependencies, no route registered); real GET /health and GET /ready shapes substituted for a fabricated HealthStatus struct"
  - "performance-tuning.md's last stale v0.4.3 tag cleared; fabricated Garrison/Concurrency/Compression config YAML and a PostgreSQL-only GIN/to_tsvector SQL example (this project's Garrison store is SQLite, with a real FTS5 virtual table) corrected"
  - "troubleshooting.md's fabricated `paladin config validate` CLI subcommand removed (no config subcommand exists), all eight fabricated /metrics curl commands removed or replaced with the real GET /ready, and a fabricated garrison_entries.session_id column corrected to the real paladin_id"
  - "The DOCS-01 verdict record completed: all fourteen rows carry a content verdict, zero pending, with a closure statement and an out-of-scope observations section (README.md's stale version figure, recorded not fixed)"
  - "The linkcheck report reviewed at the phase's end state (Run 4) as well as its start, with an honest comparison of the link-count delta against the opening run"
  - "REQUIREMENTS.md's Hand-off to Phase 16 item 4 and DOCS-01 entry amended in place, dated, superseded text retained"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Reuse the 16-01 D-09 verdict-row shape and eight-class signal battery verbatim across all five plans in this phase"
    - "When a page's entire technical premise is wrong (not just individual fields), add one prominent scope note naming the real system with citations, then correct only the independently-checkable claims (env vars, config keys, endpoint shapes, file paths) in place — leaving the rest as illustrative code fences covered by the standing note, per D-12's no-restructure guard"
    - "For dated benchmark/measurement claims that can't be reproduced against the current tree, add a one-clause dating attribution rather than inventing new numbers or deleting the claim"
    - "Cross-check SQL/config examples against the actual shipped schema and struct fields, not just against the mechanical eight-signal grep battery — several of this plan's most severe fabrications (PostgreSQL syntax on a SQLite project, a fabricated tracing ecosystem) were prose-level findings the mechanical signals alone would have missed"

key-files:
  created: []
  modified:
    - docs/src/operations/logging.md
    - docs/src/operations/monitoring.md
    - docs/src/operations/performance-tuning.md
    - docs/src/operations/troubleshooting.md
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md
    - .planning/phases/16-documentation-currency-the-architecture-gap/16-LINKCHECK-REPORT.md
    - .planning/REQUIREMENTS.md

key-decisions:
  - "logging.md's fabrication was total, not partial: the tracing crate has zero call sites anywhere in src/ or crates/, despite tracing-subscriber being a listed dependency, and tracing-loki/tracing-elastic/tracing-appender are not dependencies at all. Rather than rewrite every tracing-shaped code fence (disproportionate to D-12), added one prominent scope note naming the real log + env_logger + LogOrchestrator/LogPort/LogDestination facade with citations, then fixed the independently-checkable claims (env vars, config.yml, one fabricated file path) in place."
  - "monitoring.md's Overview claim of a /metrics endpoint on port 9090 is provably false (no prometheus/opentelemetry dependency, no route registered anywhere) — corrected with a scope note documenting the real GET /health / GET /ready shapes, matching the docker.md/production.md precedent from 16-04 for infrastructure that is reserved (port exposed) but not wired up."
  - "performance-tuning.md's 'Benchmark Results (Measured - January 2026)' figures for Garrison/Battalion/Herald cannot be reproduced against the current benches/ (only config_benchmarks.rs exists; the other four benchmark files were drafted per benches/BENCHMARK_FIXES.md but never fixed to compile and are absent from the tree). Per the plan's own guard against inventing new numbers, added a one-clause dating attribution rather than deleting or re-deriving the figures."
  - "troubleshooting.md's eight /metrics curl commands and one CLI subcommand were treated differently from logging.md/monitoring.md's illustrative code fences: because troubleshooting.md's commands are direct, actionable reader instructions (not example patterns), leaving them uncorrected would send an on-call reader down a dead end — matching the plan's own stated bar ('worse than no page'). All eight were individually corrected or removed rather than covered by a single scope note."
  - "A PostgreSQL-only 'USING gin(to_tsvector(...))' index and a garrison_entries.session_id column appeared in both performance-tuning.md and troubleshooting.md — this project's Garrison store is SQLite (garrison_type: \"sqlite\"), which has no GIN index type, and the real scoping column is paladin_id. A real SQLite FTS5 virtual table (garrison_search) already ships for full-text search; pointed both pages to it instead of fabricated Postgres syntax."
  - "Per the plan's own explicit instruction (task 3 action text) and its acceptance criteria, the DOCS-01 checkbox in REQUIREMENTS.md was left unticked even though this plan settles the fourteenth and final file — requirement-ticking is documented as the phase-close step's job, not a plan's, and an acceptance criterion (`grep -c '^- \\[x\\] **DOCS-01**' .planning/REQUIREMENTS.md` = 0) enforces this. A dated pointer to the completed verdict record was added instead of inlining the fourteen rows or ticking the box."

requirements-completed: []

coverage:
  - id: D1
    description: "logging.md settled by content: the page's tracing-ecosystem premise corrected with a scope note (zero tracing call sites in the tree; real facade is log + env_logger + LogOrchestrator), fabricated config.yml logging: section removed, fabricated RUST_LOG_FORMAT env var removed and real SYSTEM_LOG_LEVEL fallback added, one fabricated file path (src/infrastructure/logging/loki.rs) fixed, tracing-loki/tracing-elastic/tracing-appender flagged as non-dependencies"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'pending — not yet checked' 16-DOCS-01-VERDICTS.md == 2 after Task 1; mdbook build docs/ exit 0; git diff --name-only lists only logging.md, monitoring.md, and the verdict record"
        status: pass
    human_judgment: false
  - id: D2
    description: "monitoring.md settled by content: Overview's fabricated /metrics-on-9090 claim corrected with a scope note naming the real GET /health (200 {status:ok}) and GET /ready (200 {status:ready,agents:N}) endpoints against crates/paladin-web/src/health.rs; fabricated HealthStatus/ComponentHealth struct replaced with the real handlers verbatim"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'pending — not yet checked' 16-DOCS-01-VERDICTS.md == 2 after Task 1; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D3
    description: "performance-tuning.md settled by content: last stale v0.4.3 tag (2 sites) replaced with v0.8.0; Benchmark Results section dated in place rather than invented/deleted; Garrison/Concurrency/Compression fabricated config YAML corrected against real struct fields; PostgreSQL-only GIN/to_tsvector SQL and a fabricated session_id column corrected to the real SQLite FTS5 table and paladin_id column; moka/sysinfo flagged as non-dependencies"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -c 'v0\\.4\\.3' docs/src/operations/performance-tuning.md == 0; grep -rc 'v0\\.4\\.3' docs/src/user-guides/ docs/src/deployment/ docs/src/operations/ == 0 for all fourteen files; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D4
    description: "troubleshooting.md settled by content: PaladinError::ExecutionError proven real; fabricated `paladin config validate` CLI subcommand removed (no config subcommand exists on the shipped CLI); all eight fabricated /metrics curl commands removed or replaced with the real GET /ready; fabricated config.yml logging:/queue.url/paladin: top-level/llm.rate_limit-providers blocks corrected against real config fields; fabricated garrison_entries.session_id column and a stale libssl1.1 Dockerfile dependency corrected"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -noE '[A-Z][A-Za-z]*Error(::[A-Za-z]+)?' docs/src/operations/troubleshooting.md resolves via grep -rn in crates/ (PaladinError::ExecutionError, paladin_error.rs:26); grep -c 'pending — not yet checked' 16-DOCS-01-VERDICTS.md == 0; grep -c '^| docs/src/' 16-DOCS-01-VERDICTS.md == 14; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false
  - id: D5
    description: "Linkcheck report re-reviewed at the phase's end state (Run 4, verbatim mdbook build docs/ output) with a prose comparison to the opening run (895→905 links, fully explained by five other plans' cumulative edits landing in between, zero broken links in both); DOCS-01 verdict record completed with a closure statement and an out-of-scope section; REQUIREMENTS.md's Hand-off item 4 and DOCS-01 entry amended in place with dated notes, superseded text retained, checkbox intentionally left unticked per the plan's own instruction"
    requirement: "DOCS-01"
    verification:
      - kind: other
        ref: "grep -q 'Out of scope' 16-DOCS-01-VERDICTS.md; grep -q 'd87d11e' REQUIREMENTS.md; grep -q '16-DOCS-01-VERDICTS.md' REQUIREMENTS.md; test $(grep -c '^- \\[x\\] **DOCS-01**' REQUIREMENTS.md) = 0; mdbook build docs/ exit 0"
        status: pass
    human_judgment: false

duration: ~40min
completed: 2026-08-24
status: complete
---

# Phase 16 Plan 05: Operations Currency Sweep (logging.md + monitoring.md + performance-tuning.md + troubleshooting.md) and DOCS-01 Closure Summary

**Closed the last four of DOCS-01's fourteen files against the live 0.8.0 tree — finding the phase's most severe fabrication yet, an entire logging.md page built around a `tracing` crate ecosystem with zero call sites anywhere in the codebase (real facade: `log` + `env_logger` + a custom `LogOrchestrator`), plus a fabricated `/metrics` endpoint spanning monitoring.md and eight actionable troubleshooting.md commands, PostgreSQL-only SQL syntax on a SQLite project, and a fabricated CLI subcommand — then completed the DOCS-01 verdict record (14/14, zero pending) and re-reviewed the linkcheck report at the phase's end state.**

## Performance

- **Duration:** ~40 min
- **Started:** 2026-08-24 (approx, first tool call after reading the 16-05 plan and verdict record)
- **Completed:** 2026-08-24T14:28:17Z
- **Tasks:** 3
- **Files modified:** 7 (4 doc pages, 1 verdict record, 1 linkcheck report, 1 requirements ledger)

## Accomplishments

- Ran the full eight-class D-09 signal battery plus an end-to-end prose review against `logging.md`, `monitoring.md`, `performance-tuning.md`, and `troubleshooting.md`, recording every producing command and result
- **`logging.md`:** discovered the page's entire premise — the `tracing` crate ecosystem (spans, `#[instrument]`, `tracing-subscriber` layers, `tracing-loki`, `tracing-elastic`, `tracing-appender`) — has zero call sites anywhere in `src/` or `crates/`, despite `tracing-subscriber` being a listed (unused) dependency; the real facade is `log` + `env_logger` wrapped by an application-layer `LogOrchestrator`/`LogPort`/`LogDestination` system. Added a prominent scope note with citations, then corrected the independently-checkable claims: removed the fabricated `config.yml` `logging:` section (no such key exists in `Settings`), fixed the Environment Variables section (removed fabricated `RUST_LOG_FORMAT`, added the real `SYSTEM_LOG_LEVEL` fallback), fixed one fabricated file path (`src/infrastructure/logging/loki.rs`, doesn't exist), and flagged `tracing-loki`/`tracing-elastic`/`tracing-appender` as non-dependencies
- **`monitoring.md`:** proved the Overview's claimed `/metrics` endpoint (port 9090, Prometheus/Grafana/Alertmanager/Jaeger stack) does not exist — `prometheus` and `opentelemetry` are not dependencies anywhere, no `/metrics` route is registered. Corrected with a scope note documenting the real unauthenticated `GET /health` (`200 {"status":"ok"}`) and `GET /ready` (`200 {"status":"ready","agents":N}`) endpoints, and replaced a fabricated `HealthStatus`/`ComponentHealth` struct with the real handlers verbatim from `crates/paladin-web/src/health.rs`
- **`performance-tuning.md`:** replaced the corpus's last stale `v0.4.3` tag (2 sites, `cargo bench --save-baseline`/`--baseline`) with `v0.8.0`; dated the January 2026 Garrison/Battalion/Herald benchmark figures in place (only `config_benchmarks.rs` exists in `benches/` today; the other four benchmark files were drafted per `benches/BENCHMARK_FIXES.md` but never fixed to compile and are absent from the tree) rather than inventing new numbers; corrected fabricated Garrison Configuration (`type:`→`garrison_type`, removed fabricated `windowing:`/`cleanup:` blocks), Concurrency Limits (removed fabricated `paladin:`/`battalion:` top-level sections), and Network Optimization Compression (`ServerConfig` has no such field) YAML blocks against the real config surface; fixed a PostgreSQL-only `USING gin(to_tsvector(...))` SQL index example — this project's Garrison store is SQLite and already ships a real FTS5 virtual table (`garrison_search`) for full-text search — and a fabricated `session_id` column to the real `paladin_id` scoping column; flagged `moka`/`sysinfo` as non-dependencies
- **`troubleshooting.md`:** proved `PaladinError::ExecutionError` (the page's one error-type citation, signal class 7) is real; found and removed a fabricated `paladin config validate` CLI subcommand (the shipped `paladin` CLI has no `config` subcommand at all); found and corrected all eight instances of a fabricated `curl http://localhost:8081/metrics` command threaded throughout the page (no such route exists; port `8081` doesn't match the real reserved-but-unused `9090`), replacing the two most actionable occurrences with the real `GET /ready`; corrected fabricated `config.yml` `logging:`, `queue.url`, top-level `paladin:` (two places), and `llm.rate_limit`/`llm.providers` blocks against the real config surface; corrected the same `session_id`→`paladin_id` fabrication found in `performance-tuning.md`; fixed a stale `libssl1.1` Dockerfile dependency to the shipped `libssl3`
- Completed the DOCS-01 verdict record: wrote the final two evidence-bearing rows (bringing the count to 14/14, zero `pending`), added a closure statement naming all fourteen files and the method, and added an "Out of scope — observed, not fixed" section recording `README.md`'s stale Project Status version (`0.6.0` vs. shipped `0.8.0`, `README.md:181`) without editing `README.md` (reserved for plan 16-14's single-line D-15 change)
- Re-ran `mdbook build docs/` after the last content edit and appended its complete verbatim output to `16-LINKCHECK-REPORT.md` as Run 4, followed by a prose review: 905 links found (up from Run 3's 895), zero broken links in both, `warning-policy = "error"` unescalated throughout. Traced the link-count delta honestly to five other plans (16-02, 16-03, 16-04, 16-09, 16-10) whose commits landed in this worktree's history between the opening and closing runs — a `git diff` of every 16-05 commit against the four operations pages contains zero added or removed markdown link syntax, confirming this plan's own edits contributed nothing to the delta
- Amended `REQUIREMENTS.md` twice, in place, per D-00d: the "Hand-off to Phase 16" item 4 now carries a dated note that both broken links it named were already repaired by Phase 15.1 (commit `d87d11e`) before Phase 16 began, with the original text retained above it; the `DOCS-01` entry now carries a dated pointer to the completed verdict record and linkcheck report rather than inlining the fourteen rows — the checkbox itself is intentionally left unticked, per the plan's own explicit instruction and enforced acceptance criterion

## Task Commits

1. **Task 1: Sweep logging.md and monitoring.md against the 0.8.0 tree** - `103249e3` (fix)
2. **Task 2: Sweep performance-tuning.md and troubleshooting.md, clearing the last stale release tag** - `059523f0` (fix)
3. **Task 3: Re-review the linkcheck report at the end state and amend the two stale ledger statements in place** - `e950b699` (docs)

**Plan metadata:** (this commit)

## Files Created/Modified

- `docs/src/operations/logging.md` - Corrected the page's fabricated tracing-ecosystem premise, config.yml, env vars, one fabricated file path
- `docs/src/operations/monitoring.md` - Corrected the fabricated /metrics endpoint claim and Health Endpoint response shape
- `docs/src/operations/performance-tuning.md` - Cleared the last stale v0.4.3 tag; corrected fabricated Garrison/Concurrency/Compression config, a PostgreSQL-only SQL example, and a fabricated session_id column
- `docs/src/operations/troubleshooting.md` - Removed a fabricated CLI subcommand and eight fabricated /metrics commands; corrected fabricated config YAML and a fabricated session_id column
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md` - Appended the final two verdict rows, a closure statement, and an out-of-scope section (modified)
- `.planning/phases/16-documentation-currency-the-architecture-gap/16-LINKCHECK-REPORT.md` - Appended the closing Run 4 and its prose review (modified)
- `.planning/REQUIREMENTS.md` - Amended Hand-off item 4 and the DOCS-01 entry in place, dated, superseded text retained (modified)

## Decisions Made

See `key-decisions` in frontmatter above for the full rationale on each. In summary: `logging.md` and `monitoring.md` received one prominent scope note each (rather than a full rewrite) because their fabrication was total-premise rather than per-field, matching the 16-04 precedent for disproportionate full rewrites; `troubleshooting.md`'s fabricated commands were individually corrected rather than covered by a single note because they are direct reader instructions, not illustrative patterns; benchmark figures were dated in place rather than invented or deleted, per the plan's explicit guard; and the DOCS-01 checkbox was left unticked per the plan's own explicit instruction, even though this plan settles the fourteenth file.

## Deviations from Plan

None beyond the plan's own explicitly-scoped auto-fix mandate — every correction above is a Rule 1 (bug: doc content that doesn't match the live tree) fix within the plan's own `<action>` instructions to run the signal battery, extract and prove the page's claim surface, and check every command against the Makefile/shipped CLI. No architectural changes, no new dependencies, no files touched outside the four named pages and the three ledger/report files the plan names.

## Issues Encountered

- The sandboxed Bash tool rejected several multi-statement commands as "too complex to verify [they stay] inside the worktree," consistent with prior plans' notes in this phase — worked around by splitting into individual single-purpose commands (e.g. per-path `test -f` calls instead of a loop).
- `mdbook-mermaid install docs/` was needed once before the first build (fresh worktree, gitignored mermaid assets not yet present); confirmed `docs/book.toml` byte-identical before and after via `git diff --exit-code`.
- The closing linkcheck run's link count (905) differed from the opening run's (895) captured during 16-01. Investigated rather than assumed: confirmed via `git log` and per-commit `git diff` that the delta is fully attributable to five other plans (16-02/16-03/16-04/16-09/16-10) whose content landed in this worktree's history between the two captures, not to this plan's own content-only corrections.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- DOCS-01's fourteen-file scope is fully settled by content: `docs/src/user-guides/` (six files, 16-02/16-03), `docs/src/deployment/` (`cicd.md` from 16-01, three files from 16-04), and `docs/src/operations/` (all four files, this plan). The verdict record at `.planning/phases/16-documentation-currency-the-architecture-gap/16-DOCS-01-VERDICTS.md` carries 14/14 rows with zero `pending`, a closure statement, and an out-of-scope section.
- The linkcheck report at `.planning/phases/16-documentation-currency-the-architecture-gap/16-LINKCHECK-REPORT.md` is reviewed at both the phase's opening (Runs 1-3, 16-01) and closing (Run 4, this plan) state — Milestone 11's task 1.2 is satisfied.
- REQUIREMENTS.md's DOCS-01 checkbox remains unticked by design (this plan's own explicit instruction); the phase-close step should tick it, since the verdict record it points to is now complete. One out-of-scope observation is recorded for a later plan: `README.md`'s Project Status section states `0.6.0` against the shipped `0.8.0` (`README.md:181`) — plan 16-14 owns fixing this under D-15, not this plan.
- No blockers.

---
*Phase: 16-documentation-currency-the-architecture-gap*
*Completed: 2026-08-24*

## Self-Check: PASSED

All four claimed doc files verified present on disk (`docs/src/operations/logging.md`,
`docs/src/operations/monitoring.md`, `docs/src/operations/performance-tuning.md`,
`docs/src/operations/troubleshooting.md`). All three task commit hashes (`103249e3`, `059523f0`,
`e950b699`) verified present in `git log --oneline --all`.
