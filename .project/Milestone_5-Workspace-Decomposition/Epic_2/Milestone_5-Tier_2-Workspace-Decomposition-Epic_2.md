## Epic 2: Extract `paladin-ports` Crate

> **See [ADR-0014](../../../.planning/decisions/0014-milestone-4-6-tier-numbering.md)** (dated
> 2026-08-06) for the corrected Milestone/Tier numbering this document's Milestone-numbering
> references predate. This document is a byte-equivalent copy of
> `.project/Milestone_5-Workspace-Decomposition/overview/Milestone_5-Tier_2-Workspace-Decomposition.md`,
> carrying no independent content beyond that source, which is corrected there. Not corrected
> inline here.

**Epic Owner:** TBD
**Priority:** Critical
**Estimated Effort:** Medium
**Dependencies:** Epic 1 (paladin-core must exist)

### Objective

Extract the port trait definitions from `src/application/ports/` (~6k LOC, ~20 traits) into a dedicated `paladin-ports` crate. This crate defines the architectural contracts that all adapter crates implement. It depends only on `paladin-core` (for domain types referenced in trait signatures) and serves as the stable API boundary between the application layer and infrastructure.

### Background & Rationale

Port traits are the architectural backbone of the hexagonal design. They were hardened and documented as the stable public API in Milestone 1 (Epic 2). Extracting them into their own crate enforces the contract: infrastructure adapters depend on `paladin-ports` to implement traits, and application services depend on `paladin-ports` to declare their requirements. Neither can bypass the other.

The port traits reference domain types in their signatures (e.g., `GarrisonPort` references `GarrisonEntry`, `SanctumPort` references `SanctumEntry` and `Memory`). These types now live in `paladin-core`, so `paladin-ports` depends on `paladin-core`.

### Acceptance Criteria

1. `crates/paladin-ports/` exists with its own `Cargo.toml`.
2. `paladin-ports` depends only on `paladin-core`, `async-trait`, `serde`, `thiserror`, `uuid`, `chrono`, and `tokio` (for `mpsc` in streaming traits).
3. All ~20 port traits are defined in `paladin-ports` with their associated error types, request/response types, and configuration types.
4. `cargo build -p paladin-ports` succeeds in isolation.
5. The main `paladin` crate re-exports port traits at the existing `application::ports::` path.
6. All existing tests pass with updated import paths.

### Tasks

#### Task 2.1: Extract Output Port Traits

**Description:** Move all output port trait definitions to `crates/paladin-ports/src/output/`. This includes: `LlmPort` (with `LlmRequest`, `LlmResponse`, `LlmError`, `TokenUsage`, `FinishReason`, `StreamingResponse`), `GarrisonPort` (with `GarrisonError`, `GarrisonStats`), `SanctumPort` (with `SanctumError`, `SanctumQuery`, `SanctumFilter`, `SanctumSearchResult`), `EmbeddingPort` (with `Embedding`, `EmbeddingError`), `ArsenalPort`/`ArsenalRegistry`, `CitadelPort`, `QueuePort`, `NotificationPort`, `LogPort`, `SearchPort`, `FileStoragePort`, `PaladinPort` (with `PaladinResult`, `StopReason`), `BattalionPort`, and `PaladinRegistry` (with `RegistryError`).

**Deliverables:**
- `crates/paladin-ports/src/output/` containing all output port modules.
- All associated types (errors, requests, responses, configs) co-located with their port trait.
- `cargo build -p paladin-ports` succeeds.

**Estimated Effort:** Medium

#### Task 2.2: Extract Input Port Traits

**Description:** Move input port trait definitions to `crates/paladin-ports/src/input/`. This includes `ContentIngestionPort`, `RpcGatewayPort`, `MlPort`, and `NlpPort`.

**Deliverables:**
- `crates/paladin-ports/src/input/` containing all input port modules.
- `cargo build -p paladin-ports` succeeds.

**Estimated Effort:** Small

#### Task 2.3: Wire `paladin-ports` into Dependent Crates

**Description:** Update the main `paladin` crate and any modules that reference port traits to depend on `paladin-ports`. Remove duplicated source files from `src/application/ports/`. Ensure re-exports preserve backward compatibility.

**Deliverables:**
- `paladin` crate depends on `paladin-ports`.
- `src/application/ports/` removed or reduced to re-export shim.
- All adapter implementations updated to import from `paladin_ports::`.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

---
