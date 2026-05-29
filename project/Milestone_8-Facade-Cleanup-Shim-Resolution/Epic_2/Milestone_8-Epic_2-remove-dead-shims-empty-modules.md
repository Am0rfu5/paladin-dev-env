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
