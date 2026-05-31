## Epic 19: Herald & Domain Type Consolidation

**Theme:** Replace placeholder domain types, harden Herald system  
**Duration:** 1–2 weeks  
**Priority:** Critical  
**Dependencies:** None  
**Origin:** Epic 8 (Herald Output Formatting) inline TODOs

### Description

During Epic 8, several domain types in `herald.rs` were defined as placeholder structs with `TODO` comments indicating they should be replaced with actual types from Epics 1 and 4. Additionally, the Herald registry has a `TODO` to auto-register built-in formatters. Now that all domain types exist (`PaladinResult`, `BattalionResult`, `PaladinError`), these placeholders must be consolidated and the formatter pipeline completed.

### User Stories

#### US-19.1: Consolidate Herald Domain Types

**As a** framework developer  
**I want** Herald to use the actual domain result and error types  
**So that** there are no duplicate or placeholder structs in the codebase

**Acceptance Criteria:**
- [ ] Remove placeholder `PaladinResult` from `src/core/platform/container/herald.rs` (line 147)
- [ ] Remove placeholder `BattalionResult` from `src/core/platform/container/herald.rs` (line 158)
- [ ] Remove placeholder `PaladinError` from `src/core/platform/container/herald.rs` (line 187)
- [ ] Replace with imports from actual domain types in `paladin.rs` and `battalion/`
- [ ] Complete `StreamChunk` structure with full streaming metadata (line 169)
- [ ] Complete `ExecutionMetadata` structure with full telemetry fields (line 178)
- [ ] Update all Herald traits and implementations to use real types
- [ ] All existing Herald tests continue to pass
- [ ] No duplicate type definitions remain in the codebase

**Source Files:**
- `src/core/platform/container/herald.rs` — lines 147, 158, 169, 178, 187

---

#### US-19.2: Register Built-in Herald Formatters

**As a** developer  
**I want** built-in formatters auto-registered in the Herald registry  
**So that** JSON, Markdown, and Table formatting work out of the box

**Acceptance Criteria:**
- [ ] `HeraldRegistry::default()` auto-registers `JsonHerald`, `MarkdownHerald`, and `TableHerald`
- [ ] Formatters are retrievable by name from the registry
- [ ] Unit tests verify all built-in formatters are present after construction
- [ ] Documentation updated for Herald usage

**Source Files:**
- `src/application/use_cases/herald/herald_registry.rs` — line 186
