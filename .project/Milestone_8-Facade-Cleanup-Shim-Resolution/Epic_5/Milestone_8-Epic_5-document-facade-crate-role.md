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
