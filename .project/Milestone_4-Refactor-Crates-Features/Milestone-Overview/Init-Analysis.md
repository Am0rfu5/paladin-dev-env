# Refactoring into Separate Crates and Feature - Initial Analysis

## Current state — what the numbers say

The project is already well-structured internally. With the architecture already enforced at the *module* level (core → application → infrastructure), the codebase isn't in bad shape. But it is a single Cargo crate with roughly 102,000 lines spread across ~200+ source files, and that creates a specific set of problems that grow proportionally with the project.

| Area | Approx. LOC | Notes |
|---|---|---|
| `core/platform` | 25k | Domain entities + manager services |
| `application/use_cases` | 24k | Orchestration, planning, PaladinBuilder |
| `infrastructure/adapters` | 23k | LLM, MCP, storage, notifications |
| `application/cli` | 12k | Commands, formatters, wizards |
| `application/ports` | 6k | Port traits — the architectural backbone |
| `core/base` | 4k | Node, Collection, Field primitives |
| `config` | 4k | Settings, service runner |
| `infrastructure/repositories` | 3k | SQL repos |

---

## The core tension: monolith vs workspace

The honest answer is that the current structure is not wrong, and splitting too eagerly is a real danger in Rust. Cargo workspaces carry meaningful costs: longer cold compile times due to linker overhead between crates, more complex `Cargo.toml` maintenance, and circular dependency risks if boundaries aren't thought through carefully. The benefit has to exceed that friction.

The project is currently at the point where the benefits are real but the split needs to be surgical. Here are the specific areas where the friction is felt:

**1. Every change rebuilds everything.** Because all 200+ files are in one crate, a change in an infrastructure adapter recompiles the CLI, domain entities, and everything else. In a workspace, only the affected crate and its dependents recompile.

**2. Optional features lack isolation.** The `[features]` section currently has only five flags: `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, and `integration-tests`. These are thin wrappers around single optional dependencies. A user who only wants agent orchestration still compiles the Redis queue, MinIO adapter, notification system, web server, and content processing pipeline. In a workspace, they'd pull only the crates they need.

**3. The CLI is embedded in the application layer.** The `src/application/cli` subtree is 12k lines of command handlers, formatters, and config loaders. It has no reason to be compiled when Paladin is used as a library. It also pulls in `clap`, `dialoguer`, `indicatif`, `comfy-table`, and `colored`, which are all CLI-only dependencies that currently cost every downstream user.

**4. `paladin-battalion` is a standalone subsystem.** The Formation, Phalanx, Campaign, ChainOfCommand, Maneuver DSL (with its lexer, AST, and parser), Grove, Council, and flow visualizer constitute a complete orchestration runtime with its own error types, ~9k LOC of domain logic, and ~4k LOC of execution services. This is the primary public surface of the framework and has no hard dependency on the content pipeline, notifications, or web infrastructure.

**5. LLM adapters are fully swappable.** OpenAI, Anthropic, and DeepSeek adapters share only the `LlmPort` trait. Each could be its own optional crate, meaning downstream users pay only for the provider they use and provider maintainers can work independently without touching shared code.

**6. The `src/lib.rs` does a glob re-export of everything.** `pub use application::*; pub use config::*; pub use core::*; pub use infrastructure::*;` — this is a useful ergonomic layer, but it means the entire internal module graph is public by default. Without explicit visibility control at the crate boundary, the API surface is effectively the entire codebase.

---

## What to actually do — tiered recommendations

### Tier 1 — High value, low risk (do these first)

**Promote features to cover more surface area.** Before splitting crates, expand the current feature flags to cover the full optional surface. LLM provider support, document processing, vision, the MCP arsenal, Qdrant, the web server, and the notification adapters should each be individually gatable. This gives you the compile-time isolation benefit without a workspace refactor and is completely backward compatible. The `Cargo.toml` already has the pattern right for `redis` and `rust-s3`.

**Harden the port traits as the stable API contract.** The `src/application/ports` subtree (6k LOC, about 20 port traits) is already the cleanest architectural boundary in the project. Marking these ports as the primary public API — and making internal types non-public where they cross layer boundaries — is a documentation and visibility discipline task, not a refactor. It protects downstream users from coupling to implementation details.

**Move the CLI into its own binary-only compilation unit.** `[[bin]] paladin-cli` already exists in `Cargo.toml`. Moving the `src/application/cli` subtree into `src/bin/cli/` (or a dedicated `paladin-cli/src/` workspace crate if you go workspace) means those 12k lines and their 8+ CLI-only dependencies aren't compiled when Paladin is used as a library dependency.

### Tier 2 — Workspace split, when compile times become a bottleneck

If the team is feeling the pain of slow incremental builds, the diagram above represents the logical crate split. The priority order for extraction should be:

**`paladin-core` first.** The `src/core/base` and `src/core/platform/container` modules are pure domain types with essentially no external dependencies beyond `serde`, `uuid`, `chrono`, and `thiserror`. Extracting these to a zero-dependency domain crate is the safest possible move and unlocks clean dependency inversion for everything else.

**`paladin-battalion` second.** The orchestration patterns are the primary public value of the framework. Keeping them in their own crate makes the dependency graph clear — users who only want multi-agent orchestration over their own LLM implementation can depend on just `paladin-core` + `paladin-battalion` + `paladin-ports`, without pulling in MinIO, Redis, or the web server.

**`paladin-llm` per-provider, behind features.** A single `paladin-llm` crate with feature-flagged providers (`openai`, `anthropic`, `deepseek`, `mock`) is cleaner than one crate per provider, avoids over-fragmentation, and keeps the `LlmPort` trait and all its implementations in one coherent place.

**`paladin-memory` for garrison + sanctum.** SQLite garrison, in-memory garrison, and Qdrant sanctum adapter share the `GarrisonPort`/`SanctumPort` traits and could be co-located with `qdrant` behind a feature flag.

### Tier 3 — Architectural refinements within the existing structure

**`application_settings.rs` is too large (3,172 LOC).** A single file carrying all configuration types for every subsystem is a common Rust anti-pattern that compounds over time. This should be split by domain: `config/agent.rs`, `config/garrison.rs`, `config/arsenal.rs`, `config/notifications.rs`, etc., with a root `config/mod.rs` composing them. No behavioral change, but a meaningful maintainability win.

**Domain services are leaking into the `core/platform/manager` layer.** The manager modules (`notification_service.rs`, `queue_service.rs`, `orchestrator.rs`, `log_service.rs`) contain orchestration logic that properly belongs in the application use-cases layer. In strict hexagonal architecture, the core layer should contain only entities and value objects; services that depend on ports belong in the application layer. This is the one genuine architectural inconsistency.

**The `maneuver` DSL (lexer + AST + parser) should be co-located with the battalion domain types**, not split across `core/platform/container/battalion/parser/` and `application/use_cases/battalion/maneuver_service.rs`. They are tightly coupled and should travel together.

**`CircuitBreaker` in `application/use_cases/paladin/`** is an infrastructure concern, not a domain use case. It belongs in the infrastructure layer alongside rate limiting and retry logic.

---

## Summary judgment

Paladin's current architecture is solid and the module discipline is genuinely good for a project of this size. The hexagonal boundaries hold up well on inspection. The most impactful improvements in order are:

1. Expand feature flags to gate the full optional surface — immediate win, zero risk.
2. Isolate the CLI from the library compilation path.
3. Break `application_settings.rs` into per-domain config files.
4. Move manager-layer orchestration services into the application use-cases layer.
5. When build times become a genuine friction point, extract `paladin-core` and `paladin-battalion` into a workspace — those two extractions alone account for most of the isolation value with the lowest coupling risk.

The full workspace decomposition shown in the diagram is the right long-term shape, but it's not urgent today. Get the feature flags and the CLI separation done first — those are small diffs with immediate payoff.
