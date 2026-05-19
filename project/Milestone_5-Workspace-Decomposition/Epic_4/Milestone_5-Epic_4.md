
## Epic 4: Extract `paladin-llm` Crate

**Epic Owner:** TBD
**Priority:** High
**Estimated Effort:** Medium
**Dependencies:** Epic 2 (paladin-ports must define `LlmPort`)

### Objective

Extract all LLM provider adapters into a single `paladin-llm` crate with per-provider feature flags (`openai`, `anthropic`, `deepseek`, `mock`). This consolidates all `LlmPort` implementations in one coherent location, allowing provider maintainers to work independently and downstream users to pay only for the providers they use.

### Background & Rationale

The current LLM adapters in `src/infrastructure/adapters/llm/` include `openai_adapter.rs`, `anthropic_adapter.rs`, `deepseek_adapter.rs`, `openai_embedding_adapter.rs`, and mock adapters. They share only the `LlmPort` trait (now in `paladin-ports`) and each brings its own HTTP client configuration and provider-specific logic.

A single `paladin-llm` crate with feature flags is preferred over one crate per provider because it avoids over-fragmentation, keeps the `LlmPort` trait implementations discoverable in one place, and simplifies version coordination across providers. The Milestone 1 feature flags (`llm-openai`, `llm-anthropic`, `llm-deepseek`) translate directly to this crate's internal feature flags.

### Acceptance Criteria

1. `crates/paladin-llm/` exists with its own `Cargo.toml`.
2. `paladin-llm` depends on `paladin-core` and `paladin-ports`.
3. Each provider is gated behind a feature flag: `openai`, `anthropic`, `deepseek`.
4. A `mock` feature provides `MockLlmPort` and `MultiStepMockLlmPort` for testing.
5. `cargo build -p paladin-llm --no-default-features` compiles (producing an empty crate with no provider).
6. `cargo build -p paladin-llm --features openai` compiles only the OpenAI adapter.
7. The OpenAI embedding adapter is included under the `openai` feature alongside the chat adapter.
8. All LLM adapter tests pass when their respective features are enabled.
9. The `paladin` facade crate re-exports provider adapters at existing paths.

### Tasks

#### Task 4.1: Scaffold `paladin-llm` Crate with Feature Flags

**Description:** Create `crates/paladin-llm/` with a `Cargo.toml` that defines provider feature flags and optional dependencies. Map `reqwest` and provider-specific dependencies to their respective flags.

**Deliverables:**
- `crates/paladin-llm/Cargo.toml` with `openai`, `anthropic`, `deepseek`, `mock` feature flags.
- `crates/paladin-llm/src/lib.rs` with `#[cfg(feature = "...")]` module declarations.
- `cargo build -p paladin-llm --no-default-features` succeeds.

**Estimated Effort:** Small

#### Task 4.2: Extract OpenAI Adapter

**Description:** Move `openai_adapter.rs` and `openai_embedding_adapter.rs` from `src/infrastructure/adapters/llm/` to `crates/paladin-llm/src/openai/`. Update imports to reference `paladin_core` and `paladin_ports`.

**Deliverables:**
- `crates/paladin-llm/src/openai/` containing both adapters.
- `cargo build -p paladin-llm --features openai` succeeds.
- OpenAI adapter unit and integration tests pass.

**Estimated Effort:** Small

#### Task 4.3: Extract Anthropic and DeepSeek Adapters

**Description:** Move `anthropic_adapter.rs` and `deepseek_adapter.rs` to their respective modules in `paladin-llm`.

**Deliverables:**
- `crates/paladin-llm/src/anthropic.rs` and `crates/paladin-llm/src/deepseek.rs`.
- Each compiles independently behind its feature flag.
- Provider-specific tests pass.

**Estimated Effort:** Small

#### Task 4.4: Extract Mock Adapters and Wire into Facade

**Description:** Move mock LLM adapters (`MockLlmPort`, `MultiStepMockLlmPort`) behind a `mock` feature flag. Update the `paladin` facade crate to depend on `paladin-llm` and re-export adapters. Remove duplicated adapter source from the main crate.

**Deliverables:**
- Mock adapters available under `paladin-llm`'s `mock` feature.
- `paladin` facade depends on `paladin-llm` with appropriate default features.
- All existing tests that use mock adapters compile and pass.
- `cargo test --workspace` passes.

**Estimated Effort:** Medium

---
