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
