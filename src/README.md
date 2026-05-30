# Paladin Facade Crate

The `paladin` crate is the **application assembly point and composition root** for the Paladin
workspace. It does not contain business logic — it wires together all leaf crates into a runnable
application.

## Role

The facade crate hosts the `ServiceRunner` — the composition root that instantiates, configures,
and connects all application services, infrastructure adapters, and coordination layers. When you
run the Paladin binary or embed Paladin as a library, the facade is what you interact with.

### What this crate contains

- **`ServiceRunner`** — bootstraps all services from configuration and wires them together
- **Application services** (`src/application/services/`) — coordination logic for agents,
  battalions, arsenals, orchestration, herald, sanctum, and more (11 sub-modules)
- **Configuration** (`src/config/`) — multi-source settings loading (YAML, environment, defaults)
- **CLI modules** (`src/application/cli/`) — command implementations; requires the `cli` feature
  flag
- **Binary entry points** — `main.rs` (default binary) and `bin/paladin-cli.rs` (requires `cli`
  feature flag)
- **Re-export bridge** (`src/core/`) — minimal re-export structure that surfaces `paladin-core`
  types through the facade's module tree; real logic lives in `paladin-core`

### What this crate does NOT contain

Business logic, port trait definitions, and infrastructure adapter implementations live
exclusively in the leaf crates. The facade only assembles them. If you are tempted to add a new
trait, adapter, or domain type here, it almost certainly belongs in a leaf crate instead.

### Dependency direction

```
paladin (facade)
  ├── paladin-core
  ├── paladin-ports
  ├── paladin-battalion
  ├── paladin-llm
  ├── paladin-memory
  ├── paladin-notifications
  ├── paladin-storage
  ├── paladin-content
  └── paladin-web
```

**One direction only.** Leaf crates must never import from the facade crate.

---

## What lives here

| Path | Purpose | Notes |
|------|---------|-------|
| `src/application/services/` | Application coordination services | 11 sub-modules, 39 `.rs` files |
| `src/application/cli/` | CLI command implementations | Feature-gated (`cli` flag) |
| `src/config/` | Configuration loading & settings types | Composition root needs config at assembly time |
| `src/infrastructure/` | Infrastructure adapter implementations | Adapters not yet extracted to a leaf crate |
| `src/core/` | Re-export bridge to `paladin-core` | Minimal structure; real logic in `paladin-core` |
| `src/bin/` | Binary entry points | `paladin-cli.rs` (feature-gated `cli`) |
| `src/main.rs` | Default binary entry point | Thin wrapper; bootstraps `ServiceRunner` |

---

## Leaf Crates

| Crate | Capability |
|-------|-----------|
| `paladin-core` | Domain entities, base types, pure business logic |
| `paladin-ports` | Port trait definitions (output and input ports) |
| `paladin-battalion` | Multi-agent coordination patterns (Formation, Phalanx, Campaign, Chain of Command) |
| `paladin-llm` | LLM provider adapters (OpenAI, Anthropic, DeepSeek) and provider factory |
| `paladin-memory` | Garrison memory adapters (in-memory, SQLite, semantic vector search) |
| `paladin-notifications` | Notification channel adapters (email, push, system) |
| `paladin-storage` | Storage adapters (SQLite, MySQL repositories; MinIO/S3 file storage) |
| `paladin-content` | Content processing, ingestion, and analysis pipelines |
| `paladin-web` | Web API adapter (Axum-based REST server for the Paladin HTTP API) |

**Leaf crates must not import from the `paladin` facade crate.**

---

See [STABLE_API.md](../STABLE_API.md) for the public API contract.
