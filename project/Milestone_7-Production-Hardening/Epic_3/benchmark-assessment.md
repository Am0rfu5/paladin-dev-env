# Benchmark Assessment

**Milestone:** 7 — Production Hardening (Tier 4)
**Epic:** 3 — Benchmark Suite Migration and Performance Baseline
**Created:** 2026-05-27
**Purpose:** Initial audit of the current benchmark suite, benchmark ownership, and implementation anchors before migration work begins.

---

## Functional Requirement Map

| Requirement Area | PRD Reference | Current Implementation Anchor | Expected Target |
|---|---|---|---|
| Active benchmark migration | FR-01 to FR-05 | Root `benches/sanctum_benchmarks.rs`, root `Cargo.toml` benchmark section | `crates/paladin-memory/benches/` + `crates/paladin-memory/Cargo.toml` |
| Disabled benchmark review | FR-06 to FR-10 | Root `benches/` directory, commented bench entries in root `Cargo.toml`, `.disabled` files | Owning crate `benches/` directories or documented removal |
| New battalion benchmarks | FR-11, FR-12 | No crate-local battalion bench directory yet | `crates/paladin-battalion/benches/` |
| New LLM serialization benchmark | FR-13 | No crate-local LLM bench directory yet | `crates/paladin-llm/benches/` |
| New garrison benchmarks | FR-14 | Root `benches/garrison_benchmarks.rs` candidate only | `crates/paladin-memory/benches/` |
| New config benchmark | FR-15 | `src/config/settings.rs` in root `paladin` crate | Root crate benchmark unless config ownership changes later |
| Workspace execution | FR-17 to FR-19 | Root `Cargo.toml` only, with commented benchmark entries | Per-crate manifest ownership plus `cargo bench --workspace` |
| Baseline documentation | FR-20 to FR-23 | No current Epic 3 baseline artifact | `docs/PERFORMANCE_BASELINE.md` |
| Optional CI signaling | FR-24 to FR-26 | Existing `.github/workflows/ci.yml` to review later | Non-blocking reporting only if added |

---

## Current Benchmark Inventory

| File | Current State | Basis for Classification | Initial Disposition |
|---|---|---|---|
| `benches/sanctum_benchmarks.rs` | Active candidate | Present as a normal `.rs` file and explicitly called out in the PRD as the active benchmark to migrate | Migrate to `paladin-memory` |
| `benches/battalion_benchmarks.rs` | Disabled candidate | Present as a normal `.rs` file, but corresponding `[[bench]]` entry is commented out in root `Cargo.toml` | Review for restore or rewrite in `paladin-battalion` |
| `benches/garrison_benchmarks.rs` | Disabled candidate | Present as a normal `.rs` file, but corresponding `[[bench]]` entry is commented out in root `Cargo.toml` | Review for restore or rewrite in `paladin-memory` |
| `benches/herald_benchmarks.rs` | Disabled candidate | Present as a normal `.rs` file, but corresponding `[[bench]]` entry is commented out in root `Cargo.toml` | Review for continued relevance or deprecation |
| `benches/paladin_benchmarks.rs.disabled` | Disabled candidate | File extension marks it disabled; root manifest comment says it needs `LlmPort` trait implementation | Review for deprecation or redesign |
| `benches/arsenal_benchmarks.rs.disabled` | Disabled candidate | File extension marks it disabled; root manifest comment says it needs rewrite with Armament domain types | Review for deprecation or redesign |

There are no unclear benchmark files in the current root inventory. Every existing benchmark artifact can be classified as either active-to-migrate or disabled-to-review.

---

## Root Manifest State

The current benchmark state is still controlled by the root `Cargo.toml`.

- Root `dev-dependencies` already include `criterion = { version = "0.5", features = ["async_tokio"] }`.
- Root benchmark entries for `battalion_benchmarks`, `herald_benchmarks`, `garrison_benchmarks`, and `sanctum_benchmarks` are present only as commented `[[bench]]` stanzas.
- `paladin_benchmarks` and `arsenal_benchmarks` are documented as disabled benchmarks in comments rather than as active manifest entries.
- No crate-local `benches/` directories exist yet in `paladin-memory`, `paladin-battalion`, or `paladin-llm`.

This confirms the Epic starts from a fully root-owned benchmark layout and that migration work will require both file moves and per-crate manifest registration.

---

## Target Ownership Map

| Benchmark Area | Current Owner | Target Owner | Notes |
|---|---|---|---|
| Sanctum | Root workspace | `paladin-memory` | Explicit PRD migration target |
| Garrison | Root workspace | `paladin-memory` | Same subsystem ownership as sanctum |
| Battalion orchestration | Root workspace | `paladin-battalion` | Must isolate orchestration overhead with mock `PaladinPort` |
| LLM serialization | No current owner | `paladin-llm` | New benchmark required by PRD |
| Config loading (`Settings::new`) | Root `paladin` crate | Root `paladin` crate, unless config ownership moves later | `Settings` currently lives in `src/config/settings.rs` |
| Herald | Root workspace | Undetermined pending relevance review | May be deprecated if no longer aligned to Epic 3 critical paths |
| Paladin aggregate benchmark | Root workspace | Undetermined pending architecture review | May no longer map cleanly to a single publishable crate boundary |
| Arsenal benchmark | Root workspace | Undetermined pending armament API review | Existing note indicates rewrite would be required |

---

## Config Benchmark Ownership Finding

The current owner of `Settings::new()` is the root `paladin` crate.

Evidence:

- `Settings` is defined in `src/config/settings.rs`.
- `impl Settings` in that file contains the current `Settings::new()` config loading path.
- No extracted workspace crate currently owns this type.

Initial conclusion: the configuration-loading benchmark should be implemented in the root crate unless a later architectural change moves configuration ownership into a dedicated crate.

---

## Open Questions

1. Should `herald_benchmarks` survive this Epic if herald performance is not part of the confirmed critical-path benchmark set?
2. Is there any historical sanctum benchmark output already checked into docs or CI artifacts that can serve as the pre-migration comparison point?
3. If the team decides to keep a `paladin` aggregate benchmark, what is the correct architectural boundary now that most major subsystems have crate-level ownership?

---

## Migration Notes

### Sanctum Benchmark Migration

- `benches/sanctum_benchmarks.rs` was moved to `crates/paladin-memory/benches/sanctum_benchmarks.rs`.
- Benchmark imports were updated from the root `paladin` facade crate to the current crate-aligned paths:
	- `paladin_core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry}`
	- `paladin_memory::sanctum::InMemorySanctum`
	- `paladin_ports::output::sanctum_port::{SanctumFilter, SanctumPort, SanctumQuery}`
- `crates/paladin-memory/Cargo.toml` now owns the Criterion registration for `sanctum_benchmarks` and includes a local `criterion` dev-dependency.
- The stale root `sanctum_benchmarks` placeholder was removed from `Cargo.toml` so the workspace no longer treats sanctum as a root-owned benchmark target.
- Validation result: `cargo bench -p paladin-memory --bench sanctum_benchmarks --no-run` completed successfully and produced the benchmark executable.

### Disabled Benchmark Dispositions

| Benchmark | Final Disposition | Rationale |
|---|---|---|
| `battalion_benchmarks.rs` | Remove and replace in Task 4 | The file targets a broad root-level orchestration surface including Formation, Phalanx, Campaign, Chain of Command, Maneuver parsing, and visualization. Epic 3's accepted scope requires narrower crate-local battalion benchmarks focused on Formation, Phalanx, and Campaign execution overhead. A direct restore would preserve the wrong ownership and benchmark scope. |
| `garrison_benchmarks.rs` | Remove and replace in Task 4 | The file benchmarks a root-level `ConversationHistory` surface with broad mixed operations. Epic 3 requires new `paladin-memory` crate-local garrison benchmarks focused on in-memory read/write behavior at specific history sizes of 100, 1000, and 10000 entries. A direct restore would keep the old root ownership and the wrong measurement shape. |
| `herald_benchmarks.rs` | Deprecate and remove | Herald formatting is not part of the confirmed critical-path benchmark set for Epic 3. Keeping this benchmark would expand scope beyond the approved battalion, LLM serialization, garrison, and config-loading areas. |
| `paladin_benchmarks.rs.disabled` | Deprecate and remove | The file is already documented as requiring `LlmPort` trait implementation work and benchmarks the old aggregate execution boundary rather than a crate-local performance surface. That makes it a poor fit for the refactored workspace and for Epic 3's narrower benchmark plan. |
| `arsenal_benchmarks.rs.disabled` | Deprecate and remove | The file is already documented as needing a rewrite around current armament domain types. Arsenal performance is not part of the confirmed critical-path benchmark set for this Epic, so rewriting it now would be out of scope. |

No disabled benchmark file is being directly restored in Task 3. The battalion and garrison areas are intentionally being replaced with new crate-local benchmark files in Task 4 rather than reactivated from the legacy root-owned implementations.
