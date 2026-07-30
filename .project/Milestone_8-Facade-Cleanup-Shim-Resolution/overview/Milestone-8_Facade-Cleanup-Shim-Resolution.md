# Milestone 8: Facade Cleanup and Shim Resolution

**Project:** Paladin Framework
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Version Target:** v0.2.0
**Status:** Planning
**Created:** 2026-05-29
**Document Version:** 1.0

---

## Executive Summary

After seven milestones of feature flag expansion, workspace decomposition, and architectural refinement, the facade crate (`src/`) has accumulated layers of re-export shims, residual modules, and code that either belongs in an extracted crate or should be removed entirely. This Milestone audits every remaining file in `src/`, resolves each one, establishes the facade crate's permanent role as the application assembly point, and completes the `use_cases` → `services` rename.

### Success Criteria

- Every file in `src/` has an explicit disposition: stays (with justification), moves to a crate, or deleted.
- All dead re-export shims from Milestones 5–6 are removed.
- `src/application/ports/` is empty or deleted.
- `src/application/notifications/` channel services are moved to `paladin-notifications`.
- `src/application/use_cases/` renamed to `src/application/services/`.
- The facade crate compiles, all workspace tests pass, no new warnings.
- The facade crate's role is documented as "application assembly and composition root."

---

## Parallel Execution Context

This Milestone has **no dependencies on Milestones 9, 10, or 11** and can begin immediately. It is a prerequisite for:
- **Milestone 9** (Classic Orchestrator Completion) — needs a clean facade with stable directory structure.
- **Milestone 11** (Documentation Overhaul) — needs finalized paths and module names to document.

---

## Epic 1: Facade Crate Audit

**Priority:** Critical
**Estimated Effort:** Medium
**Dependencies:** None

### Objective

Produce a definitive file-by-file audit of every module in `src/`. Classify each as "stays," "moves to crate," or "delete." This is the decision document that gates all subsequent Epics.

### Tasks

#### Task 1.1: Inventory All `src/` Files

**Description:** Run `find src/ -name "*.rs" | wc -l` and produce a complete inventory. For each file, record: path, approximate LOC, what it contains (re-export shim, application service, infrastructure adapter, config, binary entry point), and what crate it references or re-exports from.

**Deliverables:**
- `facade-audit.md` with full file inventory and classification.

#### Task 1.2: Classify Each File

**Description:** Apply the disposition rules:

| Classification | Rule | Action |
|---------------|------|--------|
| **Re-export shim** | File contains only `pub use other_crate::*` or `pub mod x { pub use ... }` with no logic | Evaluate: if consumers exist, keep; if no consumers, delete |
| **Application service** | Contains business logic, orchestration, or coordination | Stays in facade (it's the assembly crate) |
| **Infrastructure adapter** | Contains concrete adapter implementation | Evaluate for extraction to existing crate |
| **Config module** | Contains configuration types or loading logic | Stays (composition root needs config) |
| **Binary entry point** | `main.rs`, `paladin-cli.rs` | Stays |
| **Dead code** | Unused, unreachable, or superseded by crate extraction | Delete |

**Deliverables:**
- Updated `facade-audit.md` with disposition for every file.
- List of files to move, grouped by target crate.
- List of files to delete.

#### Task 1.3: Validate Shim Consumer References

**Description:** For every re-export shim identified, run `grep -r` across the workspace to confirm whether any consumer still uses the re-exported path. A shim with zero consumers is dead code.

**Deliverables:**
- Consumer reference matrix: shim path → list of consumers (or "none").
- Final delete list for zero-consumer shims.

---

## Epic 2: Remove Dead Shims and Empty Modules

**Priority:** High
**Estimated Effort:** Small
**Dependencies:** Epic 1

### Objective

Delete all re-export shims, empty modules, and dead code identified in the audit. This reduces the facade crate to only the files that carry real logic or serve active consumers.

### Tasks

#### Task 2.1: Delete Zero-Consumer Shims

**Description:** Remove all files identified in Task 1.3 as having zero consumers. Update parent `mod.rs` declarations. Run `cargo build --workspace` after each batch of deletions.

**Deliverables:**
- All dead shims deleted.
- `cargo build --workspace` succeeds.
- `cargo test --workspace` passes.

#### Task 2.2: Verify and Clean `src/application/ports/`

**Description:** Port traits were extracted to `paladin-ports` in Milestone 5. If `src/application/ports/` still exists, verify it contains only re-exports. If those re-exports have consumers (e.g., facade-internal code using `crate::application::ports::*`), update those consumers to import from `paladin_ports::` directly. Then delete the directory.

**Deliverables:**
- `src/application/ports/` deleted or confirmed empty.
- All internal consumers updated to `paladin_ports::` imports.

#### Task 2.3: Verify and Clean `src/core/`

**Description:** Core domain types were extracted to `paladin-core` in Milestone 5. `src/core/` should contain only the facade re-export structure (including the `platform/mod.rs` with injected battalion paths from Milestone 6 Epic 3). Verify no real code remains. Remove any dead sub-modules.

**Deliverables:**
- `src/core/` reduced to the minimum re-export structure needed by facade consumers.
- Any dead sub-modules deleted.

---

## Epic 3: Relocate Remaining Misplaced Modules

**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 1

### Objective

Move files that the audit identifies as belonging in extracted crates to their correct locations.

### Tasks

#### Task 3.1: Move Notification Channel Services to `paladin-notifications`

**Description:** `src/application/notifications/email_notifications.rs`, `push_notifications.rs`, and `system_notifications.rs` are channel-specific application services that belong with the notification adapters in `paladin-notifications`. Move them and update imports.

**Deliverables:**
- Three files moved to `crates/paladin-notifications/src/`.
- Facade re-exports added if needed for backward compatibility.
- `cargo test -p paladin-notifications` passes.

#### Task 3.2: Resolve `src/application/storage/` Modules

**Description:** `src/application/storage/` contains `sql_store.rs`, `file_store.rs`, `user_store.rs`. Determine whether these define port traits (→ move to `paladin-ports`) or contain implementations (→ move to `paladin-storage`). If they define repository traits, they are ports. If they contain `SqliteStore` or `MigrationManager` logic, they are implementations.

**Deliverables:**
- Each file moved to its correct crate or confirmed as staying.
- Imports updated.
- Tests passing.

#### Task 3.3: Evaluate Remaining Infrastructure Adapters

**Description:** `src/infrastructure/` may still contain adapters not extracted in Milestone 7 (e.g., `tensorflow_adapter.rs`, citadel adapter, MCP adapters, log adapters). For each, decide: move to an existing crate, gate behind a feature flag and leave in facade, or flag for Milestone 9+.

**Deliverables:**
- Disposition documented for each remaining adapter.
- Extractions completed where warranted.
- Feature flags added where needed.

---

## Epic 4: `use_cases` → `services` Rename

**Priority:** Medium
**Estimated Effort:** Small
**Dependencies:** Epics 2, 3 (directory stabilized after cleanup)

### Objective

Rename `src/application/use_cases/` to `src/application/services/` to correct the DDD terminology mismatch. The contents are application services, not use cases. Actual use cases will be the AI agents and orchestration workflows users create.

### Tasks

#### Task 4.1: Rename Directory and Update Module Declarations

**Description:** `git mv src/application/use_cases src/application/services`. Update `src/application/mod.rs` to declare `pub mod services;` instead of `pub mod use_cases;`.

**Deliverables:**
- Directory renamed.
- `mod.rs` updated.

#### Task 4.2: Update All Import Paths

**Description:** Find and replace all `use_cases` references across the workspace: `grep -r "use_cases" src/ tests/ examples/ benches/`. Update each to `services`.

**Deliverables:**
- All import paths updated.
- `cargo build --workspace` succeeds.
- `cargo test --workspace` passes.

#### Task 4.3: Add Backward-Compatible Re-Export (Optional)

**Description:** If any external consumer or example uses `paladin::application::use_cases::*`, add a temporary `pub use services as use_cases;` in `application/mod.rs` with a `#[deprecated]` attribute. Evaluate whether this is needed based on the consumer audit in Epic 1.

**Deliverables:**
- Deprecated re-export added if consumers exist.
- Otherwise, no re-export (clean break).

---

## Epic 5: Document Facade Crate Role and Finalize

**Priority:** Medium
**Estimated Effort:** Small
**Dependencies:** Epics 1–4

### Objective

Document the facade crate's permanent role, update `STABLE_API.md`, and verify the workspace is clean.

### Tasks

#### Task 5.1: Document Facade Crate Architecture

**Description:** Write a `src/README.md` or crate-level doc comment in `src/lib.rs` explaining: the facade crate is the application assembly point; it contains `ServiceRunner` (composition root), application-layer coordination services, CLI modules, and binary entry points. Leaf crates provide capabilities; the facade assembles them.

**Deliverables:**
- Facade crate role documented.

#### Task 5.2: Update STABLE_API.md

**Description:** Reflect the rename (`use_cases` → `services`), removed shims, and relocated modules.

**Deliverables:**
- `STABLE_API.md` updated.
- `CHANGELOG.md` entry for v0.2.0.

#### Task 5.3: Final Verification

**Description:** Run the full quality gate: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`, `cargo doc --workspace --no-deps`.

**Deliverables:**
- All gates green.
- v0.2.0 release candidate ready.

---

## Schedule Overview

| Phase | Epic | Duration | Predecessors |
|-------|------|----------|-------------|
| Phase 1 | Epic 1: Audit | 0.5–1 sprint | None |
| Phase 2 | Epic 2: Shim Removal (parallel) | 1 sprint | Epic 1 |
| Phase 2 | Epic 3: Module Relocation (parallel) | 1–2 sprints | Epic 1 |
| Phase 3 | Epic 4: Rename | 0.5 sprint | Epics 2, 3 |
| Phase 4 | Epic 5: Documentation + Finalize | 0.5 sprint | Epic 4 |

**Total: 3–4 sprints**
