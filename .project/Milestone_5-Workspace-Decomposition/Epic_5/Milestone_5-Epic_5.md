
## Epic 5: Extract `paladin-memory` Crate

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 2 (paladin-ports must define `GarrisonPort` and `SanctumPort`)

### Objective

Extract all memory and vector storage adapter implementations into a `paladin-memory` crate with feature-flagged backends. This consolidates `GarrisonPort` implementations (in-memory, SQLite) and `SanctumPort` implementations (in-memory, Qdrant) alongside the token counting utilities and RAG retrieval service.

### Background & Rationale

The memory subsystem spans multiple directories:
- `src/infrastructure/adapters/garrison/` — `InMemoryGarrison`, `SqliteGarrison`, `TokenCounter` implementations.
- `src/infrastructure/adapters/sanctum/` — `InMemorySanctum`, `QdrantSanctumAdapter`.
- `src/application/use_cases/sanctum/` — `MemoryExtractionService`, `RagRetrievalService`.

These components share the `GarrisonPort` and `SanctumPort` traits (now in `paladin-ports`) and the domain types `GarrisonEntry`, `SanctumEntry`, `Memory`, `MemoryBuilder` (now in `paladin-core`). The SQLite garrison depends on `sqlx`, and the Qdrant sanctum depends on `qdrant-client` — both are heavy dependencies that should be optional.

### Acceptance Criteria

1. `crates/paladin-memory/` exists with its own `Cargo.toml`.
2. `paladin-memory` depends on `paladin-core` and `paladin-ports`.
3. Feature flags: `sqlite` (gates `sqlx`), `qdrant` (gates `qdrant-client`). In-memory implementations are always available.
4. `MemoryExtractionService` and `RagRetrievalService` are included (they depend on port traits, not concrete adapters).
5. `TokenCounter` and `TiktokenCounter` are included (with `tiktoken` behind the existing `content-processing` or a new `tokenizer` feature).
6. `cargo build -p paladin-memory --no-default-features` compiles with only in-memory implementations.
7. All memory-related tests pass when appropriate features are enabled.
8. The `paladin` facade crate re-exports memory types at existing paths.

### Tasks

#### Task 5.1: Scaffold `paladin-memory` Crate

**Description:** Create the crate structure with feature flags for storage backends.

**Deliverables:**
- `crates/paladin-memory/Cargo.toml` with `sqlite`, `qdrant` feature flags.
- Module structure: `garrison/`, `sanctum/`, `services/`.
- `cargo build -p paladin-memory --no-default-features` succeeds.

**Estimated Effort:** Small

#### Task 5.2: Extract Garrison Adapters

**Description:** Move `InMemoryGarrison`, `SqliteGarrison`, `TokenCounter`, and `TiktokenCounter` to `crates/paladin-memory/src/garrison/`. Gate `SqliteGarrison` behind the `sqlite` feature.

**Deliverables:**
- Garrison adapter modules relocated.
- `SqliteGarrison` compiles only with `--features sqlite`.
- `InMemoryGarrison` compiles unconditionally.
- Garrison unit tests pass.

**Estimated Effort:** Medium

#### Task 5.3: Extract Sanctum Adapters

**Description:** Move `InMemorySanctum` and `QdrantSanctumAdapter` to `crates/paladin-memory/src/sanctum/`. Gate `QdrantSanctumAdapter` behind the `qdrant` feature.

**Deliverables:**
- Sanctum adapter modules relocated.
- `QdrantSanctumAdapter` compiles only with `--features qdrant`.
- `InMemorySanctum` compiles unconditionally.
- Sanctum unit tests pass.

**Estimated Effort:** Medium

#### Task 5.4: Extract Memory Services and Wire into Facade

**Description:** Move `MemoryExtractionService` and `RagRetrievalService` to `crates/paladin-memory/src/services/`. Update the `paladin` facade crate to depend on `paladin-memory` and re-export. Remove duplicated source.

**Deliverables:**
- Memory services relocated.
- `paladin` facade depends on `paladin-memory`.
- All memory and RAG integration tests pass.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

---
