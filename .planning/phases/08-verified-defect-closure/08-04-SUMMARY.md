---
phase: 08-verified-defect-closure
plan: 04
subsystem: decisions
tags: [adr, deprecation, cli-isolation, records-only]
dependency-graph:
  requires: [ADR-0016, ADR-0019, ADR-0021, ADR-0008]
  provides: [ADR-0022, ADR-0023]
  affects: ["Phase 8 / plan 08-06 (DEBT-02 reconciliation)", "Phase 8 / plan 08-07 (DEBT-04 src/main.rs + Herald manifest changes)", "Phase 8 / plan 08-08 (DEBT-04 build-surface sweep + criterion-4 proof)", "Phase 8 / plan 08-09 (PROMOTION.md advance to 0024, ledger/REQUIREMENTS.md/PROJECT.md close-out)"]
tech-stack:
  added: []
  patterns: ["ADR file shape (no frontmatter, 7 fixed ## headings)", "must change with the executing phase named as its own executor (D-22)"]
key-files:
  created:
    - .planning/decisions/0022-deprecation-requirement-withdrawal.md
    - .planning/decisions/0023-cli-dependency-isolation.md
  modified: []
decisions:
  - "ADR-0022: withdraw Milestone 4 Epic 2 FR-8, citing DEPRECATIONS.md's own zero-candidate IMMEDIATE DEPRECATION section as evidence, restating the stale v0.2.0->v1.0.0 timeline against 0.7.0 per ADR-0008"
  - "ADR-0023: record D-13 (src/main.rs clap migration + paladin [[bin]] gate) and D-14 (paladin-herald feature split) as one CLI-dependency-isolation decision with two sites plus three root-facade consumer sites"
metrics:
  duration: "~45min"
  completed: 2026-08-07
status: complete
---

# Phase 8 Plan 04: Author ADR-0022 and ADR-0023 Summary

Authored the two ADRs Phase 8 allocates before the code that executes them lands: ADR-0022
withdraws Milestone 4 Epic 2's deprecation requirement using the epic's own tracking document as
evidence, and ADR-0023 records CLI dependency isolation across the `src/main.rs` binary gate and
`paladin-herald`'s formatter feature split, naming all three root-facade consumer sites that must
gate in lockstep.

## What Was Built

- **`.planning/decisions/0022-deprecation-requirement-withdrawal.md`** — withdraws Milestone 4
  Epic 2 FR-8. Evidence: `.project/Milestone_4-Refactor-Crates-Features/Epic_2/DEPRECATIONS.md:81`'s
  IMMEDIATE DEPRECATION section lists no candidates ("None identified yet..."); its SOFT DEPRECATION
  half resolved to `#[doc(hidden)]` (38 occurrences, confirmed executed); its INTERNAL-ONLY category
  resolves to `pub(crate)` by design. Restates the stale v0.2.0→v0.3.0→v1.0.0 timeline against the
  shipped 0.7.0 tree (`Cargo.toml:34`) per D-08, citing ADR-0008's pre-1.0 minor-bump posture. Names
  **Phase 8 itself** as the `must change` executor (plan 08-06 performs the three-way
  reconciliation).
- **`.planning/decisions/0023-cli-dependency-isolation.md`** — records D-13 and D-14 as one
  decision with two sites: (1) migrate `src/main.rs` from `structopt` to `clap` v4 derive and add
  `required-features = ["cli"]` to the `paladin` `[[bin]]`; (2) give `paladin-herald` its first
  `[features]` section gating `comfy-table`/`colored`. Names all **three** root-facade consumer
  sites that must gate to match — `src/infrastructure/adapters/herald/mod.rs:9-10`,
  `src/application/services/herald/herald_registry.rs:248-250`, and (found during planning, absent
  from `08-RESEARCH.md`) `src/config/settings.rs:214,235,244`'s `create_default_herald()`. Preserves
  retiring `src/main.rs` as an explicitly rejected option. Records the four downstream build
  surfaces (`Dockerfile:33`, `Dockerfile.chef:74`, `.github/workflows/feature-flags.yml:144`,
  `docs/src/deployment/docker.md:135,146,156`) that assume the `paladin` binary builds unconditionally
  today. Leaves the criterion-4 `cargo tree` proof output unfilled, for plan 08-08 to complete. Names
  **Phase 8 itself** as the `must change` executor (plans 08-07, 08-08).

Both ADRs match the corpus shape exactly: no YAML frontmatter, exactly seven `##` headings in the
fixed order (`Status` / `Context` / `Decision` / `Considered Options` / `Code Locations` /
`Code Conformance` / `Downstream Consumers`), a `**Date:** 2026-08-06` line beneath `Status`, and a
single `must change` line each.

No `.rs`, `Cargo.toml`, or `.github/workflows/` file was touched — this plan writes records only, as
its prohibitions require. `.planning/decisions/PROMOTION.md` is unmodified (verified via
`git diff --stat .planning/decisions/PROMOTION.md`, empty output); plan 08-09 owns that bookkeeping.

## Evidence — Verbatim Per D-00e / D-21

**Heading listing, ADR-0022** (`grep -n '^## ' .planning/decisions/0022-deprecation-requirement-withdrawal.md`):
```
3:## Status
9:## Context
37:## Decision
64:## Considered Options
77:## Code Locations
98:## Code Conformance
110:## Downstream Consumers
```

**Heading listing, ADR-0023** (`grep -n '^## ' .planning/decisions/0023-cli-dependency-isolation.md`):
```
3:## Status
9:## Context
42:## Decision
99:## Considered Options
122:## Code Locations
198:## Code Conformance
211:## Downstream Consumers
```

**Re-run deprecation counts cited by ADR-0022** (2026-08-07, this session):
- `grep -rn '#\[deprecated' src crates | wc -l` → **0**
- `grep -rn 'doc(hidden)' src crates | wc -l` → **38**

**Structural acceptance checks, both files** (all pass):
- `head -1` of each file is the exact H1 title, not `---` (no frontmatter).
- `grep -c '^must change$'` → `1` for each file.
- ADR-0022: `grep -c '0008'` → `1` (ADR-0008 cited).
- ADR-0023: `grep -c '0019'` → `3` (ADR-0019 cited as precondition); `grep -c 'src/config/settings.rs'`
  → `4` (the third facade consumer site named); `grep -c 'PENDING'` → `1` (exactly one
  explicitly-marked pending evidence slot, in `## Code Locations`).
- ADR-0023 `Considered Options`: 5 options — 1 accepted, 4 rejected, including "Retire `src/main.rs`"
  as an explicitly rejected option with its reason.

## Every `file:line` Citation and Its Verification

All citations below were re-verified in this checkout during this session (2026-08-07), by direct
`sed -n '<line>p'` / `grep -n` reads immediately before writing each ADR — not carried over from
CONTEXT.md/RESEARCH.md without re-checking.

**ADR-0022:**
| Citation | Verified content |
|---|---|
| `DEPRECATIONS.md:81` | `### ⚠️ IMMEDIATE DEPRECATION` |
| `DEPRECATIONS.md:171` | `## Current Status` |
| `DEPRECATIONS.md:190` | `## Deprecation Log` |
| `DEPRECATIONS.md:206-211` | `## Open Questions` through its 4 numbered questions |
| `docs/src/api-reference/stable-api.md:875` | `- **[Deprecations Tracking](...)** - Current and planned deprecations` |
| root `Cargo.toml:34` | `version = "0.7.0"` |
| `grep -rn '#\[deprecated' src crates` | 0 matches |
| `grep -rn 'doc(hidden)' src crates` | 38 matches |

**ADR-0023:**
| Citation | Verified content |
|---|---|
| `Cargo.toml:93` | `structopt = "0.3"` |
| `Cargo.toml:122` | `clap = { version = "4.5.40", ..., optional = true }` |
| `Cargo.toml:125` | `colored = "2.1"` |
| `Cargo.toml:126` | `comfy-table = "7.1"` |
| `Cargo.toml:22` | `paladin-herald = { version = "0.7.0", path = "crates/paladin-herald" }` |
| `Cargo.toml:54` | `paladin-herald = { workspace = true }` |
| `Cargo.toml:240-242` | `[[bin]] name = "paladin"`, no `required-features` |
| `Cargo.toml:244-247` | `[[bin]] name = "paladin-cli"`, `required-features = ["cli"]` |
| `Cargo.toml:249-252` | `[[bin]] name = "paladin-server"`, `required-features = ["web-server"]` |
| `Cargo.toml:284` | `cli = ["dep:clap", "dep:dialoguer", "dep:indicatif", "dep:console", "dep:serde_yaml"]` |
| `src/main.rs:5` | `use structopt::StructOpt;` |
| `src/main.rs:7-12` | the `#[derive(StructOpt, ...)] struct Opt { ... }` block |
| `src/main.rs:26` | `let opt = Opt::from_args();` |
| `src/main.rs:46,52,58` | the three `Opt::from_iter(&[...])` test calls |
| `crates/paladin-herald/Cargo.toml` | `[dependencies]` unconditional `comfy-table`/`colored`, no `[features]` section |
| `crates/paladin-herald/src/lib.rs:19-25` | the three `pub mod`/`pub use` formatter declarations |
| `crates/paladin-herald/src/table_herald.rs:31` | `use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table, presets};` |
| `crates/paladin-herald/src/markdown_herald.rs:17` | `use colored::*;` |
| `src/infrastructure/adapters/herald/mod.rs:9-10` | the two ungated `pub use paladin_herald::{...}` lines |
| `src/application/services/herald/herald_registry.rs:248-250` | the three `registry.register(...)` calls in `Default::default()` |
| `src/config/settings.rs:214` | `use crate::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};` |
| `src/config/settings.rs:235` | `let herald = MarkdownHerald::with_config(markdown_config);` |
| `src/config/settings.rs:244` | `let herald = TableHerald::new(table_config);` |
| `src/application/mod.rs:57-59` | `#[cfg(feature = "cli")] pub mod cli;` |
| `Dockerfile:33` | `RUN cargo build --release --workspace --bin paladin` |
| `Dockerfile.chef:74` | same `cargo build --release --workspace --bin paladin` |
| `.github/workflows/feature-flags.yml:144` | `run: cargo build --bin paladin` under "Verify paladin binary builds without cli feature" |
| `docs/src/deployment/docker.md:135,146,156` | the matching build/copy/entrypoint prose |

## Deviations from Plan

None — plan executed exactly as written. Both tasks completed without invoking any deviation rule.

## Threat Flags

None — this plan's threat model records `T-08-04-01` (repudiation) as the risk it exists to close,
mitigated by the ADRs' own dated, cited, rejected-options-preserved shape; no new surface was
introduced.

## Self-Check: PASSED

- `FOUND: .planning/decisions/0022-deprecation-requirement-withdrawal.md`
- `FOUND: .planning/decisions/0023-cli-dependency-isolation.md`
- Commit `2d9fdf6` found in `git log --oneline --all`.
- Commit `7e72922` found in `git log --oneline --all`.
