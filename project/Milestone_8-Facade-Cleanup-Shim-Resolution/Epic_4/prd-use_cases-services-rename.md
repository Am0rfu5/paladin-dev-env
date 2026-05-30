# PRD: `use_cases` → `services` Rename

**Milestone:** 8 — Facade Cleanup & Shim Resolution
**Epic:** 4
**Status:** Draft
**Date:** 2026-05-30

---

## 1. Introduction / Overview

The directory `src/application/use_cases/` is misnamed. Its contents are **application services** — stateless orchestrators that coordinate domain logic and port adapters (e.g., `PaladinExecutionService`, `FormationExecutionService`, `ContentIngestionService`). In Domain-Driven Design, a "use case" is an **AI agent or orchestration workflow** that a user composes from those services. The name therefore inverts the DDD vocabulary that the rest of the codebase follows.

This Epic renames the directory to `src/application/services/`, updates every import path that references it, corrects all documentation, and records it as a **breaking change** in the public API surface.

---

## 2. Goals

1. Rename `src/application/use_cases/` to `src/application/services/` using `git mv` to preserve full history.
2. Update the single module declaration in `src/application/mod.rs` from `pub mod use_cases;` to `pub mod services;`.
3. Replace all **286 Rust-file references** (`src/`, `tests/`, `examples/`, `benches/`) to `use_cases` with `services`.
4. Replace all **57 markdown references** in `docs/`, `README.md`, `CHANGELOG.md`, and `CONTRIBUTING.md` (excluding files under `project/`).
5. Record the path change as a **breaking change** in `CHANGELOG.md` with a full migration table.
6. Make a **clean break** — no backward-compatible re-export shim. All consumers are updated in the same commit.
7. Pass the full quality gate: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy`, `cargo fmt --check`.

---

## 3. User Stories

- **As a developer using the Paladin facade**, I want import paths to use the term `services` so that they match the DDD vocabulary consistently used throughout the codebase.
- **As a contributor reading the source tree**, I want the application layer directory name to reflect what the code actually is (application services) so I can find things quickly without confusion.
- **As a developer upgrading from a previous release**, I want a clear migration table in `CHANGELOG.md` so I know exactly which import paths changed and what to replace them with.

---

## 4. Functional Requirements

### 4.1 Directory Rename

1. The system must rename `src/application/use_cases/` to `src/application/services/` using `git mv` (not a copy-delete, to preserve git history).
2. The module declaration in `src/application/mod.rs` must be changed from `pub mod use_cases;` to `pub mod services;`.
3. No `pub use services as use_cases;` re-export must be added — this is a clean break.

### 4.2 Rust Import Path Updates

4. Every occurrence of `use_cases` in `src/`, `tests/`, `examples/`, and `benches/` **Rust files** (`.rs`) must be replaced with `services`. This covers:
   - `use crate::application::use_cases::` → `use crate::application::services::`
   - `use paladin::application::use_cases::` → `use paladin::application::services::`
   - Internal cross-references within `mod.rs` doc comments (`crate::application::use_cases::...`)
   - Any struct fields, type aliases, or identifiers whose name contains `use_cases`
5. After all replacements, `cargo build --workspace` must exit 0 with zero errors.

### 4.3 Documentation Updates

6. Every occurrence of `use_cases` in the following files must be replaced with `services`:
   - All `.md` files under `docs/`
   - `README.md` (repo root)
   - `CHANGELOG.md` (repo root)
   - `CONTRIBUTING.md` (repo root)
   - `STABLE_API.md` (repo root)
   - Files under `project/` are **explicitly excluded** from this pass.

### 4.4 CHANGELOG Breaking Change Entry

7. `CHANGELOG.md` must gain a `### Breaking Changes` entry under `[Unreleased]` that includes:
   - A one-line description: "`src/application/use_cases/` renamed to `src/application/services/`"
   - A migration table with the old path pattern, the new path pattern, and a one-line description for each major sub-module:

| Old path | New path | Module |
|----------|----------|--------|
| `paladin::application::use_cases::paladin::*` | `paladin::application::services::paladin::*` | Paladin builder & execution |
| `paladin::application::use_cases::battalion::*` | `paladin::application::services::battalion::*` | Battalion orchestration |
| `paladin::application::use_cases::arsenal::*` | `paladin::application::services::arsenal::*` | Arsenal / tool execution |
| `paladin::application::use_cases::content::*` | `paladin::application::services::content::*` | Content pipeline services |
| `paladin::application::use_cases::herald::*` | `paladin::application::services::herald::*` | Herald registry |
| `paladin::application::use_cases::orchestration::*` | `paladin::application::services::orchestration::*` | Scheduler & listener |
| `paladin::application::use_cases::log_orchestrator::*` | `paladin::application::services::log_orchestrator::*` | Log orchestration |
| `paladin::application::use_cases::notification_orchestrator::*` | `paladin::application::services::notification_orchestrator::*` | Notification orchestration |
| `paladin::application::use_cases::queue_orchestrator::*` | `paladin::application::services::queue_orchestrator::*` | Queue orchestration |
| `paladin::application::use_cases::sanctum::*` | `paladin::application::services::sanctum::*` | Sanctum auth bridge |
| `paladin::application::use_cases::analysis::*` | `paladin::application::services::analysis::*` | LLM analysis service |

### 4.5 Quality Gate

8. After all changes, the following must all pass with zero errors/warnings:
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo fmt --all -- --check`
9. The source-file count (`find src/ -name "*.rs" | wc -l`) must remain **160** (no files added or deleted — only moved).
10. `STABLE_API.md` must be updated to replace every `use_cases` path with the corresponding `services` path. `api_surface_current.txt` and `final-api.txt` are **not** updated in this Epic (deferred to release-gate).

---

## 5. Non-Goals (Out of Scope)

- **No semantic refactoring.** The internal logic of any service file must not change. This is a pure rename.
- **No re-exports or deprecation shims.** Task 4.3 from the Epic spec is explicitly rejected; there will be no `pub use services as use_cases;`.
- **No renaming of the service structs themselves.** Types like `PaladinExecutionService`, `FormationExecutionService`, etc., keep their current names — only the module path changes.
- **No changes to files under `project/`.** Task files and PRDs in `project/` are not updated.
- **No changes to `crates/` leaf crates.** The rename applies only to the facade crate (`src/`). If leaf crates happen to contain `use_cases` strings they must be audited and, if found, addressed, but none are expected.
- **No Milestone 9 extraction.** Moving services into dedicated crates is out of scope for this Epic.

---

## 6. Technical Considerations

### 6.1 Directory Move Strategy

Use `git mv` to preserve history:

```bash
git mv src/application/use_cases src/application/services
```

This is a single atomic rename at the git level. All 39 `.rs` files under the directory move in one command.

### 6.2 Reference Update Strategy

Run a targeted search-and-replace across Rust files first, then markdown files:

```bash
# Rust files
grep -rn "use_cases" src/ tests/ examples/ benches/ --include="*.rs" | wc -l
# expect ~286

# Markdown files (exclude project/)
grep -rn "use_cases" docs/ README.md CHANGELOG.md CONTRIBUTING.md | wc -l
# expect ~57
```

A `sed -i` or equivalent tool pass on each file set is acceptable. The final `cargo build` confirms correctness.

### 6.3 Known Hotspots

The following files contain the highest concentration of `use_cases` references and must be checked manually after the automated pass:

| File | Reason |
|------|--------|
| `src/application/mod.rs` | Module declaration + 12+ doc comment links |
| `src/lib.rs` | Top-level re-exports and doc examples |
| `src/config/setup/service_runner.rs` | Imports multiple orchestrator services |
| `docs/QUICKSTART.md` | User-facing import examples |
| `README.md` | User-facing import examples |
| `STABLE_API.md` | Stable public API path declarations |

### 6.4 Public API Surface

`paladin::application::use_cases::*` is part of the stable public API documented in `STABLE_API.md`. The rename is a **breaking change** (semver minor bump deferred to release). The CHANGELOG entry (Requirement 7) is mandatory.

### 6.5 No Cargo.toml Changes

The `use_cases` → `services` rename is entirely within Rust module paths. No `Cargo.toml` dependency, feature flag, or crate name changes are required.

---

## 7. Success Metrics

| Metric | Target |
|--------|--------|
| `grep -r "use_cases" src/ tests/ examples/ benches/` (Rust files) | **0 hits** |
| `grep -r "use_cases" docs/ README.md CHANGELOG.md CONTRIBUTING.md` | **0 hits** (excluding the CHANGELOG migration table itself, which documents the old name) |
| `grep "use_cases" STABLE_API.md` | **0 hits** |
| `cargo build --workspace` | Exit 0, zero errors |
| `cargo test --workspace` | All tests pass, zero failures |
| `cargo clippy --workspace -- -D warnings` | Zero warnings |
| `cargo fmt --all -- --check` | Exit 0 |
| `find src/ -name "*.rs" \| wc -l` | 160 (unchanged) |
| `CHANGELOG.md` breaking-change entry | Present with full migration table |

---

## 8. Open Questions

_All questions resolved._

- **`STABLE_API.md`:** Updated in this Epic alongside all other `use_cases` → `services` replacements.
- **`api_surface_current.txt` / `final-api.txt`:** Left unchanged; regeneration is deferred to the release-gate step.
