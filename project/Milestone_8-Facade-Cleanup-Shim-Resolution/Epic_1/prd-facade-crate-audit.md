# PRD: Facade Crate Audit

**Epic:** 1 — Facade Crate Audit
**Milestone:** 8 — Facade Cleanup, Shim Resolution, and Directory Stabilization
**Version Target:** v0.2.0
**Priority:** Critical
**Status:** Ready for Implementation
**Created:** 2026-05-29
**Document Version:** 1.1

---

## 1. Introduction / Overview

The Paladin facade crate (`src/`) is the application assembly point — it wires together all extracted leaf crates into a running system. Over seven milestones of workspace decomposition, `src/` has accumulated re-export shims, residual modules, and files that may belong in a specific leaf crate or should be deleted entirely.

**This Epic produces `facade-audit.md`**: a definitive, file-by-file decision document classifying all 189 `.rs` files in `src/` as "stays," "moves to crate," or "delete." This document directly gates Epics 2–5; no cleanup or rename work begins until the audit is complete and approved.

**Problem being solved:** Without a complete inventory and classification, cleanup work in later Epics risks breaking imports, leaving dead code behind, or moving files to the wrong crate. The audit makes every subsequent action intentional and verifiable.

---

## 2. Goals

1. Produce a complete inventory of all `.rs` files in `src/` with path, approximate LOC, and content classification.
2. Assign a disposition (stays / moves to crate / delete) to every file using explicit, documented rules.
3. Identify every re-export shim and determine whether it has active consumers anywhere in the workspace.
4. Produce a zero-consumer shim delete list that Epic 2 can execute directly.
5. Produce a "move" list, grouped by target crate, that Epic 3 can execute directly.
6. Deliver the audit as a combined markdown document (prose summary + structured table appendix) that is both human-readable and easy to reference programmatically.

---

## 3. User Stories

- **As a developer working on Epic 2**, I need to know exactly which files are dead shims with zero consumers so I can delete them confidently without breaking the build.
- **As a developer working on Epic 3**, I need a grouped list of files to move to each target crate, with their current paths and intended destinations, so I can execute `git mv` and update imports without guesswork.
- **As a developer working on Epic 4**, I need to know whether any external consumer uses `paladin::application::use_cases::*` so I can decide whether a backward-compat re-export is needed after the rename.
- **As a reviewer**, I need to see the reasoning behind each disposition decision so I can challenge or approve them before any destructive changes are made.

---

## 4. Functional Requirements

### Task 1.1 — File Inventory

1. The developer **must** enumerate every `.rs` file under `src/` using `find src/ -name "*.rs" | sort`.
2. For each file, the developer **must** record:
   - **Path** — relative to the workspace root (e.g., `src/application/notifications/email_notifications.rs`)
   - **LOC** — approximate line count (use `wc -l` or any tool; exact precision not required)
   - **Content type** — one of: `re-export shim`, `application service`, `infrastructure adapter`, `config module`, `binary entry point`, `test module`, `dead code`
   - **References** — which external crate(s) the file imports from or re-exports (e.g., `paladin_notifications`, `paladin_ports`)
3. The inventory **must** cover all 189 files. No file may be omitted.
4. The inventory **must** be saved in the structured table (Appendix A) of `facade-audit.md`.

### Task 1.2 — File Classification

5. Every file **must** be assigned a disposition using the rules in the table below. No file may have an empty or ambiguous disposition.

   | Content Type | Classification Rule | Default Disposition |
   |---|---|---|
   | **Re-export shim** | File contains only `pub use other_crate::*` or `pub mod x { pub use ... }` with no logic | Evaluate consumers (Task 1.3); delete if zero consumers |
   | **Application service** | Contains business logic, orchestration, or coordination belonging to the facade assembly layer | **Stays** |
   | **Infrastructure adapter** | Contains a concrete adapter implementation (struct + `impl Trait`) | Evaluate: move to existing crate if one owns the domain, else flag for Milestone 9+ |
   | **Config module** | Contains configuration struct definitions or config-loading logic | **Stays** (composition root needs config) |
   | **Binary entry point** | `main.rs` or `paladin-cli.rs` | **Stays** |
   | **Test module** | `#[cfg(test)]` only, or integration test helper | Stays with its source module |
   | **Dead code** | Unused, unreachable, or fully superseded by an extracted crate | **Delete** |

6. When a file's disposition is "moves to crate," the developer **must** specify the target crate (e.g., `paladin-notifications`, `paladin-storage`, `paladin-ports`).
7. When a file's disposition is "stays," the developer **must** provide a one-line justification (e.g., "composition root service," "CLI entry point").
8. The classified list **must** be saved in Appendix A of `facade-audit.md`, adding a `Disposition` column and a `Target Crate / Justification` column to the inventory table.

   **Key files explicitly requiring classification** (known from Milestone 8 planning):
   - `src/application/notifications/email_notifications.rs`
   - `src/application/notifications/push_notifications.rs`
   - `src/application/notifications/system_notifications.rs`
   - `src/application/storage/sql_store.rs`
   - `src/application/storage/file_store.rs`
   - `src/application/storage/user_store.rs`
   - All files under `src/application/ports/` (expected: re-export shims pointing to `paladin-ports`)
   - All files under `src/application/errors/` (confirmed: re-export shims — see Section 7)
   - All files under `src/core/` (expected: re-export shims pointing to `paladin-core`)
   - All files under `src/infrastructure/` (expected mix: some adapters not yet extracted in Milestone 7)

9. The audit **must** produce three derived lists in the prose section of `facade-audit.md`:
   - **List A — Files to Delete:** All files with disposition "delete."
   - **List B — Files to Move:** All files with disposition "moves to crate," grouped by target crate.
   - **List C — Files That Stay:** All files with disposition "stays," with their justification.

### Task 1.3 — Consumer Reference Validation

10. For every file classified as a `re-export shim`, the developer **must** search the entire workspace for usages of the re-exported path using `grep -r` (or equivalent) across `src/`, `crates/`, `tests/`, `examples/`, and `benches/`.
11. The search **must** look for the *re-exported path* as it would appear at the call site. For shims inside modules (e.g., `src/application/errors/citadel_error.rs` which does `pub use paladin_core::...::CitadelError;`), search for the module-qualified path: `crate::application::errors::citadel_error::CitadelError` and `paladin::application::errors::citadel_error::CitadelError`. For re-exports in `src/lib.rs` that elevate types to the crate root (e.g., `pub use ...::CitadelError;`), also search for `paladin::CitadelError` as a top-level import — consumers may use the short crate-root path rather than the fully-qualified module path.
12. The developer **must** record the result in Appendix B of `facade-audit.md` as a consumer reference matrix:

    | Shim File | Re-exported Path | Consumers (file:line) | Has Consumers? |
    |---|---|---|---|
    | `src/application/ports/llm_port.rs` | `crate::application::ports::LlmPort` | `src/foo.rs:42` | Yes |
    | `src/core/some_shim.rs` | `crate::core::SomeType` | — | **No** |

13. Any shim with `Has Consumers? = No` **must** be added to List A (Files to Delete) in the prose section.
14. Any shim with `Has Consumers? = Yes` **must** remain on List C (Files That Stay) with justification "active re-export shim — consumers exist."

---

## 5. Non-Goals (Out of Scope)

- **Do not delete, move, or modify any file** during this Epic. The audit is read-only. All changes happen in Epics 2–5.
- **Do not refactor or rewrite** any file's content.
- **Do not update import paths** in any consumer file.
- **Do not run `cargo build`** as a validation step in this Epic. The codebase is unchanged; build validation is only needed after modifications in later Epics.
- **Do not audit files inside `crates/`** — only `src/` is in scope.
- **Do not audit non-`.rs` files** (e.g., `Cargo.toml`, `README.md`).

---

## 6. Design Considerations

### Output Document: `facade-audit.md`

The document **must** be saved at:
```
project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md
```

It **must** follow this structure:

```
# Facade Crate Audit

## Summary
- Total files audited: N
- Files staying: N
- Files to move: N (grouped by target crate)
- Files to delete: N

## List A — Files to Delete
(one file per line, with reason)

## List B — Files to Move
### → paladin-notifications
### → paladin-storage
### → paladin-ports
(etc.)

## List C — Files That Stay
(file path + one-line justification)

## Appendix A — Full Inventory Table
| Path | LOC | Content Type | Disposition | Target Crate / Justification |

## Appendix B — Consumer Reference Matrix
| Shim File | Re-exported Path | Consumers (file:line) | Has Consumers? |
```

The **prose sections** (Lists A, B, C and Summary) serve as the human-readable decision record. The **appendices** serve as the structured reference for Epics 2–5 developers.

---

## 7. Technical Considerations

### Useful Commands

```bash
# Count all .rs files
find src/ -name "*.rs" | wc -l

# List all .rs files sorted
find src/ -name "*.rs" | sort

# LOC per file (sorted by size)
find src/ -name "*.rs" -exec wc -l {} + | sort -n

# Find all re-export-only files (files that are mostly `pub use`)
grep -rl "^pub use\|^    pub use" src/ | sort

# Find consumers of a specific re-exported path (workspace-wide)
grep -r "application::ports" src/ crates/ tests/ examples/ benches/

# Find consumers of a crate-root re-export (e.g., paladin::CitadelError)
grep -r "paladin::CitadelError" src/ crates/ tests/ examples/ benches/

# Find all use_cases references (used in Epic 4 planning)
grep -r "use_cases" src/ crates/ tests/ examples/ benches/
```

### `src/lib.rs` — Special Case

`src/lib.rs` is the facade crate root and must stay. However, it contains 30+ individual `pub use` re-export lines (e.g., `pub use core::platform::container::arsenal::ArsenalError;`, `pub use paladin_ports::output::citadel_port::CitadelPort;`, `pub use application::use_cases::queue_orchestrator::QueueError;`). During Task 1.3, **each re-export line must be individually checked for consumers**, not just the file as a whole. Dead re-export lines within `lib.rs` should be flagged for removal in Epic 2 even though the file itself stays.

In Appendix B, each `lib.rs` re-export line should appear as a separate row in the consumer reference matrix, with the re-exported path being the crate-root form (e.g., `paladin::ArsenalError`, `paladin::CitadelPort`).

### Known Areas Confirmed to Contain Shims

- **`src/application/ports/`** — Port traits were extracted to `paladin-ports` in Milestone 5. This directory is expected to be all shims.
- **`src/application/errors/`** — Error types were moved to `paladin-core` during Milestones 5–6. Contains four files: `citadel_error.rs` (confirmed: single-line re-export of `paladin_core::...::CitadelError`), `handoff_error.rs` (confirmed: single-line re-export of `HandoffError` from `paladin-core`'s arsenal module), `planning_error.rs`, and `prompt_error.rs` (both expected to follow the same pattern). Task 1.3 must verify consumers for all four.
- **`src/core/`** — Core domain types were extracted to `paladin-core` in Milestone 5. This directory is expected to be mostly shims plus the `platform/mod.rs` re-export structure added in Milestone 6 Epic 3 (the `pub mod container` block that injects battalion Maneuver paths from `paladin-battalion`).

### Known Areas Likely to Contain Real Logic (should stay)

- `src/application/use_cases/` — Application coordination services (will be renamed in Epic 4 but are not shims).
- `src/application/cli/` — CLI command implementations (binary entry point support).
- `src/config/` — Configuration loading and settings types.
- `src/bin/` — Binary entry points (`main.rs`, `paladin-cli.rs`).

### Known Areas Likely to Contain Misplaced Adapters (Epic 3 candidates)

- `src/application/notifications/` — Channel services (email, push, system) that belong in `paladin-notifications`.
- `src/application/storage/` — May contain implementations that belong in `paladin-storage` or `paladin-ports`.
- `src/infrastructure/adapters/` — May contain adapters not yet extracted in Milestone 7 (e.g., `tensorflow_adapter.rs`, citadel adapters, MCP adapters).

---

## 8. Success Metrics

Epic 1 is complete when **all** of the following are true:

- [ ] `facade-audit.md` exists at `project/Milestone_8-Facade-Cleanup-Shim-Resolution/Epic_1/facade-audit.md`.
- [ ] Every `.rs` file in `src/` appears in Appendix A with a non-empty disposition.
- [ ] The total count in Appendix A matches the output of `find src/ -name "*.rs" | wc -l`.
- [ ] Every file classified as `re-export shim` appears in Appendix B with a consumer search result.
- [ ] Every individual `pub use` re-export line in `src/lib.rs` appears as a separate row in Appendix B with a consumer search result.
- [ ] List A (delete), List B (move), and List C (stay) are complete, non-overlapping, and together account for all files.
- [ ] The Summary section totals are arithmetically consistent with the three lists.
- [ ] No file has been deleted, moved, or modified as part of this Epic (verified with `git status` — should be clean or only show the new `facade-audit.md`).

---

## 9. Open Questions

1. **`src/infrastructure/` depth:** Several subdirectories exist under `src/infrastructure/adapters/` (arsenal, citadel, document, garrison, herald, llm, logs, notifications, queue, sanctum, scheduling, security, web). Some may have been partially extracted in Milestone 7. Confirm with the Milestone 7 commit history which adapters are confirmed-extracted vs. still live in facade.

2. **`src/infrastructure/resilience/` and `src/infrastructure/security/`:** Not mentioned as extraction candidates. Classify during Task 1.1; if they contain concrete adapters, flag for Epic 3 or Milestone 9. Note: `src/infrastructure/resilience/circuit_breaker.rs` was relocated here in Milestone 6 Epic 4 and is confirmed as a staying application-layer utility — not a shim.

3. **Benchmark files:** `benches/` exists at the workspace root. Verify whether any bench file imports from shim paths that would break after Epic 2 deletions.

---

## Task Checklist

### Task 1.1 — Inventory All `src/` Files
- [ ] Run `find src/ -name "*.rs" | sort` and capture output
- [ ] For each file: record path, LOC, content type, and external references
- [ ] Populate Appendix A (partial — disposition column left blank until Task 1.2)
- [ ] Write count to Summary section

### Task 1.2 — Classify Each File
- [ ] Apply disposition rules to every row in Appendix A
- [ ] Add `Disposition` and `Target Crate / Justification` columns
- [ ] Produce List A (delete), List B (move, grouped by crate), List C (stays)
- [ ] Update Summary section with counts

### Task 1.3 — Validate Shim Consumer References
- [ ] For each `re-export shim` in Appendix A, run workspace-wide `grep` for the re-exported path across `src/`, `crates/`, `tests/`, `examples/`, and `benches/`
- [ ] For each `pub use` re-export line in `src/lib.rs`, run workspace-wide `grep` for the crate-root path (e.g., `paladin::TypeName`)
- [ ] Populate Appendix B with results
- [ ] Move zero-consumer shims from List C to List A
- [ ] Flag zero-consumer `lib.rs` re-export lines for Epic 2 removal (note: the file stays, individual lines are flagged)
- [ ] Finalize `facade-audit.md`
- [ ] Verify `git status` shows no source file changes
