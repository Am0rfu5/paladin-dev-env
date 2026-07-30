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
