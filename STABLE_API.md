# Stable Public API Contract

**Version:** 0.1.0
**Last Updated:** 2026-04-15
**Epic:** Milestone 4, Epic 2 - Harden Port Traits as Stable Public API

## Overview

This document defines the **stable public API contract** for the Paladin framework. These types, traits, and functions are guaranteed to follow semantic versioning for breaking changes.

## API Stability Guarantee

The types and traits listed in this document follow these rules:

1. **Backwards Compatibility**: Breaking changes will only occur in major version bumps (0.x.0 → 1.0.0, 1.x.0 → 2.0.0)
2. **Deprecation Process**: Types/methods being removed will be deprecated for at least one minor version before removal
3. **Addition Safety**: New methods can be added to traits only if they have default implementations
4. **Documentation**: All public API items must have comprehensive rustdoc with examples

## Tracking API Changes

### Automated Tracking

We use `cargo-public-api` to track changes to the public API surface:

```bash
# Generate current API surface snapshot
./scripts/extract-public-api.sh project/current-exports.txt

# Check for API changes (used in CI)
./scripts/check-api-surface.sh project/current-exports.txt

# Check deprecation warnings compile correctly
./scripts/check-deprecations.sh
```

### CI Integration

API surface changes are automatically detected in CI. If the API changes:
- The CI build will fail with a diff showing changes
- Review the changes carefully for breaking changes
- Update `CHANGELOG.md` with breaking change details
- Update the baseline: `./scripts/extract-public-api.sh project/current-exports.txt`
- Increment the version according to semver

## Stable Public API Surface

### 1. Port Traits (Primary Stable API)

Port traits define the framework's extension points and are the most stable part of the API.

#### Output Ports (Adapters)

Located in `src/application/ports/output/`:

- **`LlmPort`** - LLM provider abstraction
- **`GarrisonPort`** - Memory/context storage abstraction
- **`SanctumPort`** - Vector store abstraction
- **`ArsenalPort`** - Tool/capability registry abstraction
- **`BattalionPort`** - Multi-agent orchestration abstraction
- **`CitadelPort`** - State persistence abstraction
- **`QueuePort`** - Message queue abstraction
- **`NotificationPort`** - Notification delivery abstraction
- **`FileStoragePort`** - Object storage abstraction
- **`EmbeddingPort`** - Text embedding abstraction
- **`DocumentPort`** - Document processing abstraction
- **`SearchPort`** - Search capabilities abstraction
- **`CachePort`** - Caching abstraction
- **`MetricsPort`** - Observability metrics abstraction
- **`TracingPort`** - Distributed tracing abstraction

#### Input Ports (Use Cases)

Located in `src/application/ports/input/`:

- **`ContentInputPort`** - Content ingestion use cases
- **`MlPort`** - Machine learning use cases
- **`NlpPort`** - Natural language processing use cases
- **`RpcPort`** - RPC service use cases

### 2. Domain Entities

Core domain types from `src/core/platform/container/`:

- **`Paladin`** - Autonomous AI agent entity
- **`PaladinData`** - Paladin configuration data
- **`PaladinConfig`** - Runtime configuration
- **`PaladinStatus`** - Execution status enum
- **`PaladinResult`** - Execution result type

#### Battalion Types

- **`Battalion`** - Multi-agent coordination entity
- **`Formation`** - Sequential orchestration pattern
- **`Phalanx`** - Parallel orchestration pattern
- **`Campaign`** - Graph/DAG orchestration pattern
- **`ChainOfCommand`** - Hierarchical delegation pattern

#### Memory Types

- **`Garrison`** - Memory storage entity
- **`GarrisonConfig`** - Memory configuration
- **`MemoryEntry`** - Individual memory record

#### Tool Types

- **`Arsenal`** - Tool registry entity
- **`Armament`** - Individual tool/capability
- **`ToolCall`** - Tool invocation type
- **`ToolResult`** - Tool execution result

#### Base Types

From `src/core/base/`:

- **`Node<T>`** - Generic entity wrapper with metadata
- **`Collection<T>`** - Generic collection type
- **`Field`** - Field definition type
- **`Message`** - Generic message type

### 3. Builders

Fluent builder patterns for complex object construction:

- **`PaladinBuilder`** - Build Paladin instances
- **`CommanderBuilder`** - Build Commander instances
- **`CouncilBuilder`** - Build Council instances
- **`GroveBuilder`** - Build Grove router instances

### 4. Configuration Types

From `src/config/`:

- **`ApplicationSettings`** - Application-wide configuration
- **`LlmConfig`** - LLM provider configuration
- **`ServerConfig`** - Server configuration
- **`DatabaseConfig`** - Database configuration

### 5. Error Types

All error enums follow `thiserror` patterns:

- **`PaladinError`** - Paladin execution errors
- **`BattalionError`** - Battalion orchestration errors
- **`GarrisonError`** - Memory system errors
- **`ArsenalError`** - Tool system errors
- **`CitadelError`** - State persistence errors
- **`LlmError`** - LLM provider errors
- **`ConfigError`** - Configuration errors

### 6. Result Types

Type aliases for common Result patterns:

```rust
pub type PaladinResult<T> = Result<T, PaladinError>;
pub type BattalionResult<T> = Result<T, BattalionError>;
// ... etc
```

## Internal Implementation Details (Not Stable)

The following are **internal implementation details** and NOT part of the stable public API:

### Adapters

All concrete adapter implementations in `src/infrastructure/adapters/`:
- `OpenAIAdapter`, `DeepSeekAdapter`, `AnthropicAdapter` (use via `LlmPort`)
- `InMemoryGarrison`, `SqliteGarrison` (use via `GarrisonPort`)
- `QdrantSanctum`, `InMemorySanctum` (use via `SanctumPort`)
- `RedisQueue` (use via `QueuePort`)
- `MinIOAdapter` (use via `FileStoragePort`)

**Migration Path**: Use port traits in library code. Adapters are fine in examples and application code.

### Repositories

All repository implementations in `src/infrastructure/repositories/`:
- MySQL repositories
- SQLite repositories

### Managers

Internal service managers in `src/core/manager/`:
- `Scheduler`
- `QueueService`
- `EventManager`

### CLI

All CLI-related modules in `src/application/cli/` are internal to the binary and not exposed as library API.

### Web Server

All web server modules in `src/infrastructure/web/` are internal to the binary.

## Migration Guide for Breaking Changes

When we make breaking changes in a major version bump, we will:

1. **Deprecation Warning**: Add `#[deprecated]` annotations at least one minor version before removal
2. **Migration Documentation**: Update `docs/MIGRATION.md` with detailed migration instructions
3. **Compatibility Shims**: Provide compatibility shims when possible
4. **Version Path**: Document the upgrade path (e.g., 0.1.x → 0.2.x → 1.0.0)

## Contributing API Changes

Before changing the public API:

1. **Check Current Surface**: Run `./scripts/extract-public-api.sh`
2. **Discuss Breaking Changes**: Open an issue for breaking changes
3. **Add Deprecation First**: For removals, add `#[deprecated]` first
4. **Update This Document**: Update `STABLE_API.md` with changes
5. **Update CHANGELOG**: Document breaking changes in `CHANGELOG.md`
6. **Add Migration Guide**: Update `docs/MIGRATION.md` if needed
7. **Update Baseline**: Regenerate baseline after approval

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed API change process.

## Questions?

For questions about API stability:
- Open an issue with the `api-stability` label
- Discuss in [GitHub Discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)
- Tag maintainers: @DF3NDR

---

**Related Documents:**
- [API Audit](project/api-audit.md) - Classification of current API surface
- [CHANGELOG.md](CHANGELOG.md) - Version history and breaking changes
- [MIGRATION.md](docs/MIGRATION.md) - Migration guides between versions
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
