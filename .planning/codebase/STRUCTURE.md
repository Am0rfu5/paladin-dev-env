<!-- refreshed: 2026-07-30 -->
# Codebase Structure

**Analysis Date:** 2026-07-30

## Directory Layout

```
paladin/
├── crates/                          # Workspace member crates
│   ├── paladin-core/                # Domain entities (zero dependencies)
│   │   └── src/
│   │       ├── base/                # Foundation types (Node, Field, Message, etc.)
│   │       └── platform/
│   │           ├── container/       # Domain entities (Paladin, Battalion, Garrison, etc.)
│   │           └── manager/         # Core services (UserService, EventManager)
│   ├── paladin-ports/               # Port trait definitions (hexagonal contracts)
│   │   └── src/
│   │       ├── input/               # Input port traits
│   │       └── output/              # Output port traits (LlmPort, GarrisonPort, etc.)
│   ├── paladin-battalion/           # Multi-agent orchestration services
│   │   └── src/
│   │       ├── *_service.rs         # Formation, Phalanx, Campaign, ChainOfCommand, etc.
│   │       ├── commander.rs         # Dynamic routing strategy
│   │       ├── maneuver/            # Flow DSL for Maneuver patterns
│   │       │   ├── parser/          # Lexer, AST, parser for DSL
│   │       │   └── service.rs       # Maneuver execution
│   │       └── retry.rs             # Exponential backoff
│   ├── paladin-llm/                 # LLM provider adapters
│   │   └── src/
│   │       ├── openai/              # OpenAI adapter + vision
│   │       ├── anthropic/           # Anthropic adapter
│   │       ├── deepseek/            # DeepSeek adapter
│   │       ├── mock.rs              # Mock adapter for testing
│   │       ├── provider_factory.rs  # Factory for adapter selection
│   │       └── config/              # LLM config types
│   ├── paladin-memory/              # Memory adapters (Garrison, Sanctum, Citadel)
│   │   └── src/
│   │       ├── garrison/            # Conversation memory (SQLite, in-memory)
│   │       ├── sanctum/             # Vector search (Qdrant, in-memory)
│   │       ├── citadel/             # State persistence
│   │       └── config/              # Memory config types
│   ├── paladin-storage/             # Repository & persistence adapters
│   │   └── src/
│   │       ├── sqlite/              # SQLite repository adapters
│   │       ├── mysql/               # MySQL repository adapters
│   │       ├── redis/               # Redis queue adapter
│   │       ├── minio/               # MinIO/S3 file storage adapter
│   │       ├── scheduler/           # Tokio-cron scheduler adapter
│   │       └── migrations/          # Database schema migrations
│   ├── paladin-content/             # Content ingestion & processing
│   │   └── src/
│   │       ├── ingestion/           # Document parsing
│   │       ├── processors/          # Content transformation
│   │       └── config/              # Content config types
│   ├── paladin-notifications/       # Notification adapters
│   │   └── src/
│   │       ├── email/               # Email notifications
│   │       ├── push/                # Push notifications
│   │       └── system/              # System notifications
│   ├── paladin-herald/              # Output formatting adapters
│   │   └── src/                     # JSON, Markdown, Table formatters
│   ├── paladin-web/                 # Web API adapter (Axum)
│   │   └── src/
│   │       ├── app.rs               # Router setup
│   │       ├── agent_controller.rs  # Agent endpoints
│   │       ├── auth_middleware.rs   # Authentication
│   │       ├── http_layers.rs       # CORS, rate limiting, timeout
│   │       ├── openapi.rs           # Swagger/OpenAPI schema
│   │       └── adapters/            # API-specific adapters
│   └── doc-examples/                # Documentation examples
│
├── src/                             # Facade crate (root paladin)
│   ├── lib.rs                       # Library root + public API
│   ├── main.rs                      # Default binary entry point
│   ├── prelude.rs                   # Curated re-exports (stable API)
│   ├── bin/
│   │   ├── paladin-cli.rs           # CLI tool (requires `cli` feature)
│   │   └── paladin-server.rs        # Web server binary (requires `web-server`)
│   ├── application/                 # Application services (composition root)
│   │   ├── services/
│   │   │   ├── paladin/             # Paladin execution (builder, executor, planning)
│   │   │   ├── battalion/           # Battalion orchestration services
│   │   │   ├── arsenal/             # Arsenal registry & execution
│   │   │   ├── garrison/            # Garrison memory coordination
│   │   │   ├── sanctum/             # Sanctum search & RAG services
│   │   │   ├── orchestration/       # Classic job orchestrator
│   │   │   ├── content/             # Content ingestion coordination
│   │   │   ├── analysis/            # Analysis services
│   │   │   ├── herald/              # Herald registry
│   │   │   ├── notification_orchestrator/  # Notification coordination
│   │   │   ├── queue_orchestrator/  # Queue coordination
│   │   │   ├── log_orchestrator/    # Log coordination
│   │   │   └── mod.rs               # Services module root
│   │   ├── errors/                  # Application-level error types
│   │   │   ├── planning_error.rs
│   │   │   ├── prompt_error.rs
│   │   │   ├── handoff_error.rs
│   │   │   └── mod.rs
│   │   ├── cli/                     # CLI (requires `cli` feature)
│   │   │   ├── commands/            # Command implementations
│   │   │   ├── config/              # Config loading & validation
│   │   │   ├── formatters/          # Output formatting (table, progress, JSON)
│   │   │   ├── interactive/         # REPL & wizard
│   │   │   ├── templates/           # Agent/Battalion templates
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── config/                      # Configuration types & loading
│   │   ├── settings.rs              # Top-level Settings struct
│   │   ├── agents.rs                # Agent configuration
│   │   ├── arsenal.rs               # Arsenal/MCP configuration
│   │   ├── citadel.rs               # Citadel configuration
│   │   ├── file_storage.rs          # File storage configuration
│   │   ├── garrison.rs              # Garrison configuration (re-exported from paladin-memory)
│   │   ├── herald.rs                # Herald formatter configuration
│   │   ├── notifications.rs         # Notification channel config
│   │   ├── queue.rs                 # Queue configuration
│   │   ├── scheduler.rs             # Scheduler configuration
│   │   ├── user_config.rs           # User service configuration
│   │   ├── web_server.rs            # Web server configuration
│   │   ├── setup/
│   │   │   ├── mod.rs               # setup_and_run() entry point
│   │   │   └── service_runner.rs    # ServiceRunner composition root
│   │   ├── env_utils.rs             # Environment variable parsing
│   │   └── mod.rs
│   ├── core/                        # Re-exports from paladin-core crate
│   │   ├── platform/
│   │   │   ├── container/           # Re-export domain entities
│   │   │   │   └── battalion/       # Extend with maneuver/parser shims
│   │   │   ├── manager/             # Core services
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── infrastructure/              # Infrastructure adapters
│   │   ├── adapters/
│   │   │   ├── llm/                 # LLM config bridge
│   │   │   ├── arsenal/             # MCP protocol (STDIO, HTTP, resource controls)
│   │   │   │   ├── mcp_protocol.rs  # MCP message handling
│   │   │   │   ├── mcp_stdio_adapter.rs
│   │   │   │   ├── mcp_streamable_http_adapter.rs
│   │   │   │   ├── resource_controls.rs
│   │   │   │   └── tool_result_formatter.rs
│   │   │   ├── auth/                # Token authentication adapter
│   │   │   ├── garrison/            # Re-export paladin-memory adapters
│   │   │   ├── sanctum/             # Re-export vector search adapters
│   │   │   ├── citadel/             # Re-export state persistence adapters
│   │   │   ├── file_storage/        # Re-export MinIO adapter
│   │   │   ├── queue/               # Re-export Redis adapter
│   │   │   ├── logs/                # System log adapter
│   │   │   ├── scheduling/          # Tokio-cron scheduler adapter
│   │   │   ├── notifications/       # Re-export notification adapters
│   │   │   ├── content/             # Re-export content adapters
│   │   │   ├── herald/              # Re-export formatter adapters
│   │   │   ├── input/               # Input port adapters
│   │   │   ├── output/              # Output port adapters
│   │   │   └── mod.rs
│   │   ├── repositories/            # Re-export SQL repositories
│   │   ├── resilience/              # Circuit breaker, fault tolerance
│   │   ├── security/                # Encryption, audit, TLS
│   │   ├── web/                     # Web-specific adapters (requires `web-server`)
│   │   │   ├── agent_host.rs        # Agent HTTP hosting
│   │   │   ├── facade_provisioner.rs # Adapter provisioning
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── mod.rs
│
├── tests/                           # Integration & functional tests
│   ├── unit/                        # Unit test collection
│   │   └── mod.rs
│   ├── integration/                 # Integration tests (require services)
│   │   ├── paladin_garrison_integration_test.rs
│   │   ├── citadel_integration_test.rs
│   │   ├── openai_embedding_tests.rs
│   │   ├── qdrant_sanctum_integration_test.rs
│   │   ├── rag_integration_tests.rs
│   │   ├── vision_integration_test.rs
│   │   ├── arsenal_execution_integration_test.rs
│   │   └── context_injection_test.rs
│   ├── functional/                  # Functional end-to-end tests
│   │   ├── paladin_server_smoke.rs
│   │   ├── web_server_e2e.rs
│   │   └── lib.rs
│   ├── cli/                         # CLI tests (requires `cli` feature)
│   │   └── mod.rs
│   ├── helpers/                     # Test fixtures & mocks
│   │   ├── mock_llm_adapter.rs
│   │   ├── mock_arsenal_adapter.rs
│   │   ├── mock_paladin_port.rs
│   │   └── mod.rs
│   └── lib.rs                       # Test utilities
│
├── examples/                        # Runnable examples
│   ├── vision_analysis.rs           # Vision capabilities demo
│   ├── vision_battalion.rs          # Multi-agent vision analysis
│   ├── document_processing.rs       # Content processing pipeline
│   ├── http_service_host.rs         # Web server hosting
│   └── cli_configs/                 # Example config files
│
├── benches/                         # Benchmarks (criterion)
│   └── config_benchmarks.rs
│
├── migrations/                      # Database schema migrations
│   └── *.sql
│
├── k8s/                             # Kubernetes manifests
│   └── server/
│
├── docker/                          # Docker configuration
│   ├── Dockerfile
│   ├── Dockerfile.server
│   ├── conf/                        # Service config
│   ├── redis/                       # Redis container setup
│   └── testserver/                  # Test server setup
│
├── docs/                            # Documentation
│   ├── src/
│   │   ├── architecture/            # Architecture guides
│   │   ├── deployment/              # Deployment docs
│   │   ├── getting-started/         # Tutorials
│   │   ├── operations/              # Operations & monitoring
│   │   ├── user-guides/             # Feature guides
│   │   ├── api-reference/           # API documentation
│   │   └── appendix/                # Reference material
│   └── book/
│
├── notes/                           # Design notes & research
│   ├── agentic-orchestration-research/
│   └── chats/                       # Design discussion archives
│
├── project/                         # PRDs & task lists
│   ├── Milestone_1-MVP/             # Milestone breakdown
│   └── Milestone_10-CI-Hardening-Release-Automation/
│
├── .planning/                       # GSD planning artifacts
│   ├── codebase/                    # Codebase maps (this file)
│   └── phases/                      # Phase planning documents
│
├── Cargo.toml                       # Workspace root manifest
├── Cargo.lock                       # Dependency lock file
├── config.example.yml               # Example application config
├── Makefile                         # Development commands
├── Dockerfile                       # Default container image
├── CHANGELOG.md                     # Version history
├── CLAUDE.md                        # Project instructions
├── README.md                        # Project overview
└── LICENSE                          # MIT license
```

## Directory Purposes

**Corrected 2026-08-06 (Phase 7, D-05):** this section previously described only 6 of the workspace's
10 library crates (`paladin-core`, `paladin-ports`, `paladin-battalion`, `paladin-llm`,
`paladin-memory`, `paladin-storage`), even though this map sits third in the project's precedence
order — above `intel/` and every PRD — and the top-level directory tree above already listed all ten
correctly. The five entries below (`paladin-herald`, `paladin-notifications`, `paladin-content`,
`paladin-web`, `doc-examples`) close that gap; the six original entries are unchanged.

**`crates/`:**
- Purpose: Workspace member crates (leaf crates)
- Contains: Domain, ports, adapters, orchestration, content processing
- Key files: Each crate has `Cargo.toml`, `src/lib.rs`, `README.md`

**`crates/paladin-core/`:**
- Purpose: Pure domain logic with zero external dependencies
- Contains: Paladin, Battalion, Garrison, Arsenal, Citadel, Sanctum, Herald domain types
- Key files: `src/platform/container/*.rs` (domain entities)

**`crates/paladin-ports/`:**
- Purpose: Port trait definitions (hexagonal architecture contracts)
- Contains: Output ports (LlmPort, GarrisonPort, ArsenalPort, etc.) and input ports
- Key files: `src/output/*.rs` (port traits)

**`crates/paladin-battalion/`:**
- Purpose: Multi-agent orchestration patterns and services
- Contains: Formation, Phalanx, Campaign, ChainOfCommand, Commander, Maneuver DSL
- Key files: `src/*_service.rs`, `src/maneuver/` (DSL implementation)

**`crates/paladin-llm/`:**
- Purpose: LLM provider adapters
- Contains: OpenAI, Anthropic, DeepSeek, mock adapters
- Key files: `src/{openai,anthropic,deepseek}/adapter.rs`

**`crates/paladin-memory/`:**
- Purpose: Memory adapters (conversation & semantic)
- Contains: Garrison (conversation), Sanctum (vectors), Citadel (state)
- Key files: `src/garrison/*.rs`, `src/sanctum/*.rs`

**`crates/paladin-storage/`:**
- Purpose: Repository and persistence adapters
- Contains: SQLite, MySQL, Redis queue, MinIO/S3, scheduler
- Key files: `src/{sqlite,mysql,redis,minio,scheduler}/*.rs`

**`crates/paladin-herald/`:**
- Purpose: Herald output-formatter adapters (JSON, Markdown, Table)
- Contains: JSON, Markdown (via `colored`) and Table (via `comfy-table`) Herald implementations
- Key files: `src/{json_herald,markdown_herald,table_herald}.rs`

**`crates/paladin-notifications/`:**
- Purpose: Notification adapter implementations (email, push, system)
- Contains: Email (via `lettre`/`handlebars`, feature-gated), push, and system notification adapters
- Key files: `src/{email_notification_adapter,push_notification_adapter,system_notification_adapter}.rs`

**`crates/paladin-content/`:**
- Purpose: Content processing adapters and use-case services
- Contains: Web scraping, RSS, PDF extraction, tokenization, and LLM-backed content analysis
  (optional `paladin-llm` dependency behind the `llm` feature)
- Key files: `src/adapters/`, `src/services/`

**`crates/paladin-web/`:**
- Purpose: Web server adapters (Axum) exposing the agent orchestration API over HTTP
- Contains: Agent/job/user controllers, auth middleware, rate limiting, OpenAPI/Swagger UI, SSE
  streaming
- Key files: `src/{app,agent_controller,agent_auth,auth_middleware,openapi}.rs`

**`crates/doc-examples/`:**
- Purpose: Compile-verified documentation examples for the Paladin book (`publish = false`, not
  published to crates.io)
- Contains: Deployment-topology examples (HTTP service host, queue worker, sidecar), orchestration
  and content-bridge examples, README quick-example verification
- Key files: `src/{deployment_topologies,http_service_host,queue_worker,sidecar,bridge,orchestration,content,readme}.rs`

**`src/`:**
- Purpose: Facade crate (composition root and assembly point)
- Contains: Application services, CLI, config, infrastructure re-exports
- Key files: `lib.rs` (stable API), `main.rs`, `application/services/`

**`src/application/services/`:**
- Purpose: Application-level coordination services
- Contains: Paladin execution, battalion orchestration, registries
- Key files: `paladin/paladin_execution_service.rs`, `battalion/mod.rs`

**`src/config/`:**
- Purpose: Configuration types and multi-source loading
- Contains: Settings, agent config, arsenal config, memory config
- Key files: `settings.rs` (top-level), `setup/service_runner.rs` (composition root)

**`src/infrastructure/adapters/`:**
- Purpose: Infrastructure adapter implementations
- Contains: MCP protocol, auth, logs, scheduling; re-exports from leaf crates
- Key files: `arsenal/mcp_*.rs`, `auth/`, `logs/`

**`tests/`:**
- Purpose: Integration and functional tests
- Contains: Service-level tests, E2E tests, mocks, fixtures
- Key files: `integration/`, `functional/`, `helpers/`

**`examples/`:**
- Purpose: Runnable example applications
- Contains: Vision analysis, document processing, HTTP service hosting
- Key files: Feature-gated examples with `required-features`

## Key File Locations

**Entry Points:**
- `src/main.rs` — Default binary (minimal setup)
- `src/bin/paladin-cli.rs` — CLI tool (requires `cli` feature)
- `src/bin/paladin-server.rs` — Web server (requires `web-server` feature)
- `src/config/setup/mod.rs` — `setup_and_run()` orchestrates services

**Configuration:**
- `config.example.yml` — Example YAML configuration
- `src/config/settings.rs` — Settings struct and loading logic
- `src/config/setup/service_runner.rs` — ServiceRunner (composition root)

**Core Logic:**
- `crates/paladin-core/src/platform/container/paladin.rs` — Paladin entity
- `crates/paladin-core/src/platform/container/battalion/` — Battalion types
- `crates/paladin-core/src/platform/container/garrison.rs` — Memory types

**Execution:**
- `src/application/services/paladin/paladin_execution_service.rs` — Core reasoning loop
- `crates/paladin-battalion/src/formation_service.rs` — Sequential execution
- `crates/paladin-battalion/src/phalanx_service.rs` — Concurrent execution
- `crates/paladin-battalion/src/campaign_service.rs` — DAG execution

**Port Definitions:**
- `crates/paladin-ports/src/output/llm_port.rs` — LLM contract
- `crates/paladin-ports/src/output/garrison_port.rs` — Memory contract
- `crates/paladin-ports/src/output/arsenal_port.rs` — Tool execution contract

**Adapters:**
- `crates/paladin-llm/src/openai/adapter.rs` — OpenAI LLM
- `crates/paladin-llm/src/anthropic/adapter.rs` — Anthropic LLM
- `crates/paladin-llm/src/deepseek/adapter.rs` — DeepSeek LLM
- `crates/paladin-memory/src/garrison/sqlite_garrison.rs` — Conversation memory
- `crates/paladin-memory/src/sanctum/qdrant_sanctum.rs` — Vector search
- `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs` — STDIO MCP transport
- `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs` — HTTP MCP transport

**Web API:**
- `crates/paladin-web/src/app.rs` — Route setup
- `crates/paladin-web/src/agent_controller.rs` — Agent endpoints

## Naming Conventions

**Files:**
- Module files: `lowercase_snake_case.rs`
- Service files: `*_service.rs` (e.g., `paladin_execution_service.rs`)
- Adapter files: `*_adapter.rs` (e.g., `openai_adapter.rs`)
- Port files: `*_port.rs` (e.g., `llm_port.rs`)
- Integration tests: `*_integration_test.rs`
- Functional tests: `*_test.rs`

**Directories:**
- Domain containers: `container/`
- Service layers: `services/`
- Adapter implementations: `adapters/`
- Configuration: `config/`
- Port definitions: `ports/` (in paladin-ports crate)
- Tests: `tests/`

## Where to Add New Code

**New Agent Feature:**
- Implementation: `src/application/services/paladin/paladin_execution_service.rs`
- Tests: `tests/integration/paladin_*_test.rs`
- Config: `src/config/agents.rs` (if configuration needed)

**New LLM Provider:**
- Adapter: `crates/paladin-llm/src/{provider_name}/adapter.rs`
- Feature flag: Add to `Cargo.toml` (`llm-{provider}`)
- Factory: `crates/paladin-llm/src/provider_factory.rs`
- Tests: `crates/paladin-llm/tests/`

**New Orchestration Pattern:**
- Service: `crates/paladin-battalion/src/{pattern_name}_service.rs`
- Domain type: `crates/paladin-core/src/platform/container/battalion/{pattern_name}.rs`
- Tests: `crates/paladin-battalion/tests/`

**New Memory Adapter:**
- Garrison adapter: `crates/paladin-memory/src/garrison/{adapter_name}.rs`
- Sanctum adapter: `crates/paladin-memory/src/sanctum/{adapter_name}.rs`
- Feature flag: Add to `crates/paladin-memory/Cargo.toml`
- Tests: `crates/paladin-memory/tests/`

**New Repository:**
- Implementation: `crates/paladin-storage/src/{db_type}/{repository_name}.rs`
- Feature flag: Add to `crates/paladin-storage/Cargo.toml`
- Migration: `migrations/*.sql`

**New CLI Command:**
- Command: `src/application/cli/commands/{command_name}.rs`
- Requires: `cli` feature flag
- Handler: `src/application/cli/commands/mod.rs` (register command)

**New Web Endpoint:**
- Controller: `crates/paladin-web/src/{controller_name}.rs`
- Route: `crates/paladin-web/src/app.rs`
- Requires: `web-server` feature flag

**Utilities & Helpers:**
- Shared helpers: `src/infrastructure/` or `crates/paladin-core/src/base/service/`
- Domain services: `crates/paladin-core/src/platform/manager/`

## Special Directories

**`migrations/`:**
- Purpose: Database schema migrations
- Generated: Manual SQL files
- Committed: Yes
- Pattern: YYYY-MM-DD_description.sql

**`benches/`:**
- Purpose: Performance benchmarks (Criterion)
- Generated: Results in `target/criterion/`
- Committed: Source files only

**`examples/`:**
- Purpose: Runnable examples demonstrating framework usage
- Generated: Binaries built from `examples/*.rs`
- Committed: Yes

**`.planning/`:**
- Purpose: GSD (Getting Stuff Done) planning artifacts
- Generated: Codebase maps, phase plans
- Committed: Yes

**`notes/`:**
- Purpose: Design research, decision archives
- Generated: Design discussion notes
- Committed: Yes

---

*Structure analysis: 2026-07-30*
