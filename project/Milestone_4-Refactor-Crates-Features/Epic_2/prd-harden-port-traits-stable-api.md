# Product Requirements Document: Harden Port Traits as the Stable Public API Contract

**Project:** Paladin Framework Refactoring Initiative
**Epic:** Epic 2 - Milestone 4, Tier 1
**PRD Version:** 1.0
**Created:** 2026-04-15
**Target Completion:** 1-2 months
**Status:** Draft

---

## 1. Introduction/Overview

The Paladin framework currently exposes its entire internal module graph through glob re-exports in `src/lib.rs`, making every internal type, adapter implementation, and infrastructure detail part of the de facto public API. This creates a fragile contract where any internal refactoring constitutes a breaking change for consumers.

This PRD defines the work required to establish the ~20 port traits in `src/application/ports/` as the **explicit, documented, stable public API** of the Paladin framework, while restricting visibility of implementation details that should remain internal.

### Problem Statement

The current `src/lib.rs` performs indiscriminate glob re-exports:

```rust
pub use application::*;
pub use config::*;
pub use core::*;
pub use infrastructure::*;
```

This pattern:
- Exposes ~200+ internal types as public API
- Couples consumers to implementation details (adapter internals, repository specifics, manager services)
- Makes module reorganization and refactoring risky
- Lacks clear API contract documentation
- Prevents future multi-crate decomposition without breaking changes

### Goal

Transform the Paladin crate's public API surface from "everything exported" to a **curated, documented, stable contract** centered on port traits and essential domain types, enabling safe internal refactoring and establishing the foundation for workspace decomposition.

---

## 2. Goals

1. **Establish Port Traits as the Stable API Contract**: Explicitly designate the ~20 port traits (`LlmPort`, `GarrisonPort`, `SanctumPort`, `ArsenalPort`, etc.) as the public-facing abstraction layer.

2. **Restrict Internal Implementation Visibility**: Mark adapter implementations, repository details, CLI modules, and manager services as `pub(crate)` or remove from crate root exports.

3. **Comprehensive Documentation**: Create reference-grade rustdoc for all port traits with multiple examples, edge cases, thread-safety guarantees, and implementor guidance.

4. **API Surface Documentation**: Produce a `STABLE_API.md` reference document cataloging every public type, trait, and function with stability guarantees.

5. **Automated Stability Verification**: Implement CI tooling to detect and prevent unintended public API surface changes.

6. **Zero Test Regression**: Maintain 100% test pass rate (1,487+ tests) with minimal import path adjustments.

7. **Clean Documentation Build**: Achieve zero warnings in `cargo doc --no-deps` with no broken intra-doc links.

---

## 3. User Stories

### US-1: Library Consumer Using Agent Orchestration
**As a** Rust developer building an AI agent application
**I want to** depend on a stable, well-documented public API contract
**So that** internal Paladin refactors don't break my application and I understand exactly what interfaces to implement

**Acceptance Criteria:**
- Port traits are clearly documented as the primary integration points
- Examples show how to implement custom adapters
- Internal types I shouldn't depend on are not exported from the crate root
- Documentation includes error semantics and thread-safety guarantees

### US-2: Custom Adapter Developer
**As a** developer implementing a custom LLM provider adapter
**I want to** understand the exact contract of `LlmPort` and see multiple implementation examples
**So that** I can confidently build an adapter that integrates correctly with the framework

**Acceptance Criteria:**
- `LlmPort` rustdoc includes detailed trait purpose, all method contracts, and error conditions
- At least 2-3 examples showing different implementation patterns
- Guidance on thread-safety requirements and async execution model
- Links to reference implementations (OpenAI, Anthropic, DeepSeek adapters)

### US-3: Framework Maintainer Refactoring Infrastructure
**As a** Paladin framework maintainer
**I want to** refactor internal adapter implementations without fear of breaking downstream consumers
**So that** I can improve the codebase structure and performance over time

**Acceptance Criteria:**
- Internal adapter types are marked `pub(crate)` or not re-exported
- CI detects unintended public API surface changes
- Only types in `STABLE_API.md` are considered breaking-change-protected
- Deprecation warnings guide users away from internal types if accidentally exposed

### US-4: Documentation Reader
**As a** new Paladin user reading the generated docs
**I want to** see a clean, well-organized API reference with no broken links
**So that** I can quickly understand the framework's capabilities and integration points

**Acceptance Criteria:**
- `cargo doc --no-deps` builds with zero warnings
- All intra-doc links resolve correctly
- Port traits are prominently featured in the generated documentation
- Each port trait has "# Examples" section with working code

### US-5: API Stability Verifier (CI/CD Process)
**As a** CI/CD pipeline
**I want to** automatically detect when a PR changes the public API surface
**So that** maintainers are alerted to potential breaking changes before merge

**Acceptance Criteria:**
- `cargo-public-api` or equivalent tool runs on every PR
- API diff is generated and posted as PR comment if changes detected
- Breaking changes fail the CI check unless explicitly acknowledged
- `STABLE_API.md` is automatically updated or validated against actual surface

---

## 4. Functional Requirements

### FR-1: Curated Public API Exports (Priority: Critical)

**Description:** Replace glob re-exports in `src/lib.rs` with explicit, curated `pub use` statements.

**Requirements:**
1. Export all ~20 port traits from `application::ports::{input, output}`
2. Export essential domain entities required by port trait signatures:
   - `Paladin`, `PaladinData`, `PaladinConfig`, `PaladinResult`, `PaladinStatus`
   - `Battalion` types: `Formation`, `Phalanx`, `Campaign`, `ChainOfCommand`
   - `Garrison`, `Arsenal`, `Armament`, `Citadel` domain types
   - `Node<T>`, `Collection`, `Field`, `Message` base types
3. Export builder types: `PaladinBuilder`, `BattalionBuilder` (if applicable)
4. Export configuration types: `ApplicationSettings`, relevant subsystem configs
5. Export common error types: `PaladinError`, `BattalionError`, `GarrisonError`, etc.
6. **Do NOT export:**
   - Adapter implementations (`OpenAIAdapter`, `DeepSeekAdapter`, `RedisAdapter`, etc.)
   - Repository implementations (`MySQLRepository`, `SQLiteRepository`, etc.)
   - CLI modules (`application::cli::*`)
   - Manager services (`manager::scheduler`, `manager::queue_service`, etc.)
   - Internal infrastructure utilities

### FR-2: Visibility Modifier Hardening (Priority: Critical)

**Description:** Apply `pub(crate)` or `pub(super)` to internal modules and types.

**Requirements:**
1. Mark all adapter module internals as `pub(crate)`:
   - `infrastructure::adapters::llm::openai_adapter` internals
   - `infrastructure::adapters::garrison::*` internals
   - `infrastructure::adapters::queue::redis_adapter` internals
   - All other adapter module private types
2. Mark repository implementations as `pub(crate)`
3. Mark CLI modules as `pub(crate)` (handled primarily in Epic 3, but coordinate visibility)
4. Mark manager services as `pub(crate)` unless explicitly part of public API
5. Ensure port trait definitions remain `pub` and exported

### FR-3: Comprehensive Port Trait Documentation (Priority: Critical)

**Description:** Add reference-grade rustdoc to all port traits.

**Requirements for each port trait:**
1. **Module-level documentation** (if trait is sole member of module):
   - High-level purpose and use case
   - Link to architecture documentation
   - Overview of all traits in the module
2. **Trait-level documentation**:
   - Trait purpose (3-5 sentences)
   - When to implement this trait
   - Thread-safety guarantees (`Send + Sync` implications)
   - Async execution model
3. **Method documentation**:
   - Purpose and behavior contract for each method
   - Parameter descriptions with constraints
   - Return value semantics
   - Error conditions (all possible error variants)
   - Panic conditions (if any)
4. **Examples section** (`/// # Examples`):
   - At least 2 examples per trait showing different usage patterns
   - One "basic usage" example
   - One "advanced usage" or "custom implementation" example
   - All examples must compile and run (use `/// # use paladin::*;` preamble)
5. **Implementor guidance** (`/// # Implementation Notes`):
   - Best practices for implementing the trait
   - Common pitfalls to avoid
   - Performance considerations
   - Links to reference implementations
6. **Safety section** (if applicable):
   - Thread-safety requirements
   - Concurrent call semantics

**Traits requiring documentation (minimum list):**
- `LlmPort`
- `GarrisonPort`
- `SanctumPort`
- `EmbeddingPort`
- `ArsenalPort`, `ArsenalRegistry`
- `PaladinPort` (if exists as abstraction)
- `BattalionPort` (if exists as abstraction)
- `CitadelPort`
- `QueuePort`
- `NotificationPort`
- `FileStoragePort`
- All input ports in `application::ports::input::*`

### FR-4: STABLE_API.md Reference Document (Priority: High)

**Description:** Create a developer-facing catalog of the public API surface with stability guarantees.

**Requirements:**
1. **Document structure:**
   - Introduction: Purpose and scope of the stable API
   - Versioning policy: What constitutes a breaking change
   - Stability tiers: Stable, Unstable/Experimental, Deprecated
   - Public types catalog (organized by category)
   - Change process: How API changes are proposed and reviewed
2. **Public types catalog sections:**
   - Port Traits (output ports)
   - Input Ports (use case interfaces)
   - Domain Entities
   - Builders
   - Configuration Types
   - Error Types
   - Base Types (Node, Collection, Field, Message)
3. **For each type/trait:**
   - Fully qualified path
   - Stability tier (Stable / Unstable / Deprecated)
   - Short description (one sentence)
   - Link to rustdoc
   - Breaking change policy (what changes are allowed)
4. **Versioning policy section:**
   - Definition of breaking change per SemVer
   - Deprecation process (minimum 1 minor version notice)
   - Feature flag impact on stability
   - Crate split impact on versioning (for future workspace decomposition)
5. **Markdown format:**
   - Use tables for type catalogs
   - Include table of contents with links
   - Maintain alphabetical ordering within sections
   - Include last-updated date

### FR-5: Import Path Updates (Priority: High)

**Description:** Update all examples and integration tests to use new import paths.

**Requirements:**
1. Audit all files in `examples/` and `tests/` for direct imports of types that will no longer be re-exported
2. Update imports to use explicit paths (e.g., `paladin::infrastructure::adapters::llm::openai_adapter::OpenAIAdapter` becomes `paladin::OpenAIAdapter` if exported, or remains fully qualified if internal)
3. Add `#[allow(deprecated)]` to any code using deprecated paths during transition
4. Ensure all 193+ examples compile after changes
5. Ensure all 1,487+ tests pass after changes

### FR-6: Documentation Build Verification (Priority: High)

**Description:** Ensure clean documentation generation with zero warnings.

**Requirements:**
1. `cargo doc --no-deps` completes with zero warnings
2. All intra-doc links resolve (no `[broken link]` warnings)
3. All public items have rustdoc (no missing docs warnings if `-D missing_docs` is enabled)
4. Generated HTML documentation has logical organization:
   - Port traits are prominent in sidebar
   - Internal types (if any remain public) are clearly marked
5. `cargo doc --open` produces navigable, professional documentation

### FR-7: Automated API Surface Tracking (Priority: High)

**Description:** Implement CI tooling to detect public API surface changes.

**Requirements:**
1. Install and configure `cargo-public-api` (or equivalent):
   - Add to CI dependencies
   - Generate baseline API surface snapshot on `main` branch
   - Compare PR API surface against baseline
2. CI job configuration:
   - Run on every PR targeting `main`
   - Generate API diff report
   - Post diff as PR comment (or artifact)
   - Fail check if breaking changes detected (unless labeled `breaking-change`)
3. API surface snapshot management:
   - Store baseline in version control (e.g., `.public-api-baseline.txt`)
   - Update baseline when intentional API changes are merged
   - Document update process in `CONTRIBUTING.md`
4. Integration with deprecation process:
   - Detect when deprecated items are removed
   - Verify deprecation was present for required period

### FR-8: Deprecation Warnings for Transitional Types (Priority: Medium)

**Description:** Add deprecation warnings to types that will be removed from public API in future.

**Requirements:**
1. Identify types currently public but not intended for stable API (from audit in Task 2.1)
2. Add `#[deprecated(since = "X.Y.Z", note = "Use ... instead. This will be made private in version X+1.Y.Z")]` annotations
3. Provide migration guidance in deprecation note:
   - If type has public alternative, specify it
   - If no alternative, explain what pattern should be used instead
4. Ensure deprecated types are clearly marked in `STABLE_API.md` as "Deprecated"
5. Plan removal timeline (minimum 1 minor version deprecation period)

---

## 5. Non-Goals (Out of Scope)

The following are explicitly **not** part of this Epic:

1. **Workspace Decomposition**: Creating separate crates (`paladin-core`, `paladin-battalion`, etc.) is Tier 2 work.
2. **Feature Flag Expansion**: Adding new feature flags is Epic 1; this Epic only documents how feature flags impact API stability.
3. **Config File Splitting**: Decomposing `application_settings.rs` is Tier 3 work.
4. **Manager Layer Relocation**: Moving manager services to application layer is Tier 3 work.
5. **New Port Trait Addition**: This Epic hardens existing ports; new ports are separate feature work.
6. **Port Trait Refactoring**: Changing port trait signatures is out of scope; only documentation and visibility changes.
7. **Performance Optimization**: No performance tuning is required for this Epic.
8. **Binary Size Reduction**: Visibility changes may incidentally reduce binary size but it's not a goal.
9. **Runtime Behavior Changes**: All changes are compile-time only; no runtime behavior modifications.

---

## 6. Design Considerations

### Port Trait Organization

The `src/application/ports/` directory should maintain the current structure:
- `input/` - Use case entry point traits (if applicable)
- `output/` - Outbound adapter abstractions (primary port traits)

All port traits should follow the pattern:
```rust
#[async_trait]
pub trait LlmPort: Send + Sync {
    /// Method documentation here
    async fn generate(...) -> Result<...>;
}
```

### Documentation Style Guide

All port trait rustdoc should follow this template:

```rust
/// High-level trait purpose (2-3 sentences).
///
/// Detailed description explaining when and why to use this trait,
/// what problem it solves, and how it fits into the framework.
///
/// # Thread Safety
///
/// This trait requires `Send + Sync` because...
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use paladin::*;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Example code here
/// # Ok(())
/// # }
/// ```
///
/// Advanced usage:
///
/// ```no_run
/// # use paladin::*;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Advanced example here
/// # Ok(())
/// # }
/// ```
///
/// # Implementation Notes
///
/// When implementing this trait:
/// - Best practice 1
/// - Best practice 2
/// - Common pitfall to avoid
///
/// See [`OpenAIAdapter`](crate::infrastructure::adapters::llm::OpenAIAdapter)
/// for a reference implementation.
#[async_trait]
pub trait LlmPort: Send + Sync {
    // Methods...
}
```

### STABLE_API.md Structure

The document should use this structure:

```markdown
# Paladin Stable Public API Reference

**Version:** X.Y.Z
**Last Updated:** YYYY-MM-DD
**Status:** Pre-release

## Introduction

[Purpose and scope]

## Versioning Policy

[SemVer interpretation, breaking change definition]

## Stability Tiers

- **Stable**: [Definition and guarantees]
- **Unstable/Experimental**: [Definition and caveats]
- **Deprecated**: [Removal timeline]

## Public API Catalog

### Port Traits (Output Ports)

| Trait | Path | Stability | Description |
|-------|------|-----------|-------------|
| `LlmPort` | `paladin::LlmPort` | Stable | LLM provider abstraction |
| ... | ... | ... | ... |

### Domain Entities

[Table]

### Builders

[Table]

### Configuration Types

[Table]

### Error Types

[Table]

## Change Process

[How to propose API changes]

## FAQ

[Common questions about API stability]
```

---

## 7. Technical Considerations

### Hexagonal Architecture Preservation

All changes must maintain the hexagonal architecture boundaries:
- **Core layer** (`src/core/`): Domain entities, pure business logic - already has no external dependencies
- **Application layer** (`src/application/`): Use cases, port trait definitions - must remain independent of infrastructure
- **Infrastructure layer** (`src/infrastructure/`): Adapter implementations - can depend on core and application

The port traits live in the application layer and define the contract between layers. This Epic makes that contract explicit and stable.

### Module Organization Impact

The current module structure is:
```
src/
├── lib.rs                   # Crate root with glob re-exports (TO CHANGE)
├── core/                    # Domain layer
├── application/
│   ├── ports/
│   │   ├── input/          # Use case traits
│   │   └── output/         # Adapter abstractions (PRIMARY API)
│   ├── use_cases/          # Service implementations
│   └── cli/                # CLI modules (TO RESTRICT)
├── infrastructure/
│   ├── adapters/           # Adapter implementations (TO RESTRICT)
│   └── repositories/       # Repository implementations (TO RESTRICT)
├── config/                 # Configuration
└── manager/                # Manager services (TO RESTRICT)
```

Only types from `core/`, `application/ports/`, builders, and config should be exported from `lib.rs`.

### Backward Compatibility During Transition

Since this is pre-release software, we can be more aggressive with changes, but we'll still use deprecation warnings as a best practice:

1. **Phase 1** (This Epic): Add deprecation warnings to types that will be restricted
2. **Phase 2** (Future minor version): Actually restrict visibility
3. **Phase 3** (After workspace decomposition): Full API stabilization with SemVer 1.0.0

### Testing Strategy

1. **Unit tests**: Should already be using internal paths; minimal impact expected
2. **Integration tests**: May use crate root re-exports; will need import updates
3. **Examples**: Definitely use crate root re-exports; will need import updates
4. **Benchmarks**: Check for any direct adapter imports

All test runs should use the pattern:
```bash
cargo test --all-features          # All tests
cargo test --lib                   # Unit tests only
cargo test --test '*'              # Integration tests only
cargo run --example <name>         # Verify each example
```

### CI Pipeline Integration

Extend existing CI with new jobs:

```yaml
# Pseudo-CI config
jobs:
  api-surface-check:
    - name: Check Public API Surface
      run: |
        cargo install cargo-public-api
        cargo public-api --simplified > current-api.txt
        diff .public-api-baseline.txt current-api.txt || echo "API changed"

  doc-check:
    - name: Build Documentation
      run: |
        cargo doc --no-deps --all-features
        # Check for warnings in output

  deprecation-check:
    - name: Verify Deprecation Policy
      run: |
        # Custom script to check deprecated items have proper annotations
        ./scripts/check-deprecations.sh
```

### Dependency Considerations

This Epic has minimal dependency impact:
- **Add**: `cargo-public-api` (dev dependency or CI-only)
- **No removals**: Visibility changes don't affect dependencies
- **No version bumps required**: This is a structural change

### Feature Flag Interaction

This Epic coordinates with Epic 1 (Feature Flags):
- Port traits are **never** feature-gated - they're always compiled
- Adapter implementations **may be** feature-gated
- The stable API surface includes all port traits regardless of features
- Feature-gated adapters should be clearly marked in `STABLE_API.md`

Example in documentation:
```rust
/// # Feature Requirements
///
/// This adapter requires the `llm-openai` feature flag.
#[cfg(feature = "llm-openai")]
pub struct OpenAIAdapter { /* ... */ }
```

---

## 8. Success Metrics

### Quantitative Metrics

1. **API Surface Reduction**:
   - **Baseline**: ~200+ exported types (current glob re-exports)
   - **Target**: ≤50 explicitly exported types (port traits + essential domain types)
   - **Measurement**: `cargo public-api --simplified | wc -l`

2. **Documentation Coverage**:
   - **Baseline**: Unknown (audit needed)
   - **Target**: 100% rustdoc coverage for all public items
   - **Measurement**: `cargo doc --no-deps 2>&1 | grep "warning: missing documentation"`

3. **Documentation Link Health**:
   - **Baseline**: Unknown (audit needed)
   - **Target**: 0 broken intra-doc links
   - **Measurement**: `cargo doc --no-deps 2>&1 | grep "broken link"`

4. **Test Pass Rate**:
   - **Baseline**: 100% (1,487+ tests passing)
   - **Target**: 100% (no regression)
   - **Measurement**: `cargo test --all-features`

5. **Example Compilation**:
   - **Baseline**: All examples compile
   - **Target**: All examples compile with updated imports
   - **Measurement**: `./scripts/check-all-examples.sh` (to be created)

6. **CI Build Time Impact**:
   - **Baseline**: Current CI duration
   - **Target**: No increase (should slightly decrease due to fewer re-exports)
   - **Measurement**: CI job duration before/after

### Qualitative Metrics

1. **Documentation Quality**:
   - All port traits have clear purpose statements
   - Each trait has at least 2 working examples
   - Implementor guidance is actionable and specific
   - No jargon without definition

2. **API Clarity**:
   - New users can identify integration points within 5 minutes
   - Custom adapter implementation is achievable with docs alone
   - No confusion between public API and internal types

3. **Maintainer Confidence**:
   - Team can refactor adapter internals without fear
   - API change review process is clear
   - `STABLE_API.md` is the source of truth

### Success Criteria Checklist

- [ ] `src/lib.rs` has explicit exports only (no glob re-exports)
- [ ] All ~20 port traits have comprehensive rustdoc
- [ ] `STABLE_API.md` is complete and accurate
- [ ] `cargo doc --no-deps` builds with 0 warnings
- [ ] All 1,487+ tests pass
- [ ] All 193+ examples compile and run
- [ ] `cargo-public-api` CI job is operational
- [ ] API surface reduced to ≤50 exported items
- [ ] Deprecation warnings added for transitional types
- [ ] Integration tests updated with new import paths
- [ ] No broken intra-doc links in generated documentation

---

## 9. Open Questions

### Q1: Should we maintain a separate public-api-baseline for each feature flag combination?

**Context**: Different feature flags expose different adapters. Should we track API surface per-feature, or only the base `--no-default-features` surface plus `--all-features`?

**Options**:
- A. Track only `--no-default-features` and `--all-features`
- B. Track each individual feature flag's API surface
- C. Track common combinations (default, minimal, full)

**Decision needed by**: Task 2.1 audit phase

---

### Q2: How should we handle types that are "implementation details" but need to be public for technical reasons?

**Context**: Some types must be `pub` for Rust's trait system (e.g., associated types, generic parameters) but aren't intended for external use.

**Options**:
- A. Mark with `#[doc(hidden)]` to hide from generated docs
- B. Create a separate `paladin::internal` module with clear "do not use" warning
- C. Use sealed trait pattern where applicable
- D. Accept that some implementation types must be visible

**Decision needed by**: Task 2.2 implementation phase

---

### Q3: What is the exact versioning policy for the stable API?

**Context**: Need to define what constitutes a breaking change in the context of:
- Adding new methods to port traits (breaking for implementors)
- Adding new error variants (potentially breaking)
- Changing internal implementation while preserving trait contract
- Feature flag additions/removals

**Options**:
- A. Strict SemVer: any trait change is major version bump
- B. Relaxed: new methods with default impls are minor bumps
- C. Pre-1.0 rules: use 0.x.y versioning with more flexibility
- D. Define per-trait stability tiers (some traits are "experimental")

**Decision needed by**: Task 2.4 STABLE_API.md creation

---

### Q4: Should adapter implementations be completely hidden or available via feature-gated paths?

**Context**: Adapters like `OpenAIAdapter` might be useful for direct instantiation in tests or advanced scenarios, but exposing them increases the API surface.

**Options**:
- A. Completely hide all adapters; consumers use only builders/factories
- B. Export adapters from `paladin::adapters::*` submodule with "unstable" marking
- C. Export adapters but mark with `#[doc(hidden)]`
- D. Export adapters as stable API (increases surface significantly)

**Decision needed by**: Task 2.1 audit phase

---

### Q5: How do we handle the Battalion pattern types (Formation, Phalanx, Campaign, ChainOfCommand)?

**Context**: These are domain entities but also have associated execution services. Are they part of the stable API or implementation details?

**Options**:
- A. Export domain types, hide execution services
- B. Export both domain types and execution service traits
- C. Export only a high-level `Battalion` trait, hide specific patterns
- D. Keep all Battalion types internal, expose only via builders

**Decision needed by**: Task 2.1 audit phase

---

### Q6: Should the `Node<T>` pattern and base types be public?

**Context**: `Node<T>`, `Collection`, `Field`, `Message` are fundamental patterns used throughout. Consumers might want to create their own domain entities following the same pattern.

**Options**:
- A. Export as stable public API with comprehensive docs
- B. Export but mark as "advanced/unstable" API
- C. Keep internal; provide alternative builder patterns for extensions
- D. Export only specific concrete types, not the generic pattern

**Decision needed by**: Task 2.1 audit phase

---

## Appendices

### Appendix A: Estimated Task Breakdown

| Task | Description | Estimated Effort | Priority |
|------|-------------|------------------|----------|
| 2.1 | Audit Current Public API Surface | 3-5 days | Critical |
| 2.2 | Replace Glob Re-Exports with Curated Exports | 5-7 days | Critical |
| 2.3 | Document Port Traits as Stable API | 10-15 days | Critical |
| 2.4 | Create STABLE_API.md Reference Document | 3-5 days | High |
| 2.5 | CI Integration for API Surface Tracking | 2-3 days | High |
| 2.6 | Import Path Updates (Examples/Tests) | 3-5 days | High |
| **Total** | | **26-40 days** | |

### Appendix B: Reference Documentation

- **Hexagonal Architecture**: `notes/hexagonal-arch.md`
- **Design Document**: `docs/Design/Design_and_Architecture.md`
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **cargo-public-api**: https://github.com/Enselic/cargo-public-api
- **rustdoc best practices**: https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html

### Appendix C: Example Export Structure

Proposed `src/lib.rs` structure after this Epic:

```rust
// Paladin Framework Public API
// This module defines the stable public API contract.
// Internal implementation types are not re-exported.

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

// Port Traits (Primary Public API)
pub use application::ports::output::{
    LlmPort, GarrisonPort, SanctumPort, ArsenalPort,
    EmbeddingPort, QueuePort, NotificationPort,
    FileStoragePort, CitadelPort,
    // ... other port traits
};

// Domain Entities
pub use core::platform::container::{
    Paladin, PaladinData, PaladinConfig, PaladinResult, PaladinStatus,
    Garrison, Arsenal, Armament, Citadel,
    // ... other essential domain types
};

// Battalion Types
pub use core::platform::container::battalion::{
    Battalion, Formation, Phalanx, Campaign, ChainOfCommand,
};

// Builders
pub use application::use_cases::paladin::PaladinBuilder;

// Base Types (if deemed public)
pub use core::base::{Node, Collection, Field, Message};

// Error Types
pub use core::platform::container::{
    PaladinError, BattalionError, GarrisonError, ArsenalError,
    // ... other public error types
};

// Configuration
pub use config::{ApplicationSettings, /* select config types */};

// Everything else remains internal (pub(crate) or not re-exported)
```

---

**End of PRD**
