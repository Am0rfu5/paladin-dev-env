# Versioning Policy

## Purpose

This document defines how Paladin versions its workspace crates and what constitutes a breaking change.

## Initial Versioning Strategy

Paladin uses lockstep versioning for the initial release line.

- Scope: all public crates in this workspace.
- Current baseline: 0.1.0.
- Milestone 7 target: 0.2.0 lockstep for publishable crates.
- Rule: a single release version is applied to all public crates in the same release cycle.

Public crates:

- paladin
- paladin-core
- paladin-ports
- paladin-battalion
- paladin-llm
- paladin-memory
- paladin-web
- paladin-notifications
- paladin-content
- paladin-storage

## Breaking Change Policy

Breaking changes require a coordinated lockstep release increment.

Examples of breaking changes:

- Removing or renaming a public type, trait, function, enum variant, or module path.
- Changing function signatures in a way that breaks callers.
- Changing trait method signatures or required methods.
- Changing feature flag semantics in a way that breaks existing consumers.
- Tightening configuration requirements without backward-compatible defaults.

Non-breaking changes:

- Additive APIs (new types, functions, optional feature flags).
- Internal refactoring that preserves public API behavior and signatures.
- Documentation-only improvements.

## Crate-Family Guidance

- paladin-core: domain model compatibility is high impact; treat model shape changes as potentially breaking.
- paladin-ports: trait contracts are compatibility-critical; changes are usually breaking.
- paladin-battalion: orchestration runtime APIs and strategy entrypoints should remain stable.
- paladin-llm: provider additions are additive; request/response contract changes may be breaking.
- paladin-memory: storage adapter behavior and query API changes may be breaking.
- paladin-web: externally consumed handler/middleware APIs should preserve compatibility.
- paladin-notifications: adapter trait behavior and config contracts should remain stable.
- paladin-content: use-case and adapter public APIs should preserve call signatures.
- paladin-storage: repository and migration public APIs should preserve compatibility.
- paladin facade: re-export paths and top-level developer ergonomics are compatibility-critical.

## Transition Criteria for Independent Versioning

Paladin may transition from lockstep to independent crate versioning after all criteria below are met:

- Stable dependency graph with low cross-crate churn across at least 2-3 release cycles.
- Per-crate changelog discipline is consistently maintained.
- Public API stability tiers are fully documented and regularly reviewed.
- CI pipeline supports dependency-aware, per-crate release automation.
- Release owners agree that independent cadence adds value without excessive coordination cost.

Until then, lockstep versioning remains the default policy.

## Dependency-Aware Publish Order

Use dependency-first publishing in this order:

1. paladin-core
2. paladin-ports
3. Leaf crates (paladin-battalion, paladin-llm, paladin-memory, paladin-web, paladin-notifications, paladin-content, paladin-storage)
4. paladin facade crate

This order is required because dry-run and publish validation for dependent crates requires published upstream dependencies.
