# External Integrations

**Analysis Date:** 2026-07-30

## APIs & External Services

**LLM Providers:**
- **OpenAI** - GPT-3.5-Turbo, GPT-4, GPT-4o family models
  - SDK/Client: `reqwest` 0.12.4, wrapped by adapter in `crates/paladin-llm/src/openai/adapter.rs`
  - Auth: `OPENAI_API_KEY` environment variable
  - Base URL: `https://api.openai.com/v1` (configurable via `OPENAI_BASE_URL`)
  - Organization: Optional via `OPENAI_ORGANIZATION` env var
  - Features: Vision support (multimodal), embeddings API, streaming responses
  - Config struct: `OpenAIConfig` in `crates/paladin-llm/src/openai/adapter.rs`
  - Adapter location: `crates/paladin-llm/src/openai/adapter.rs`

- **Anthropic** - Claude 3/3.5 models
  - SDK/Client: `reqwest` 0.12.4, wrapped by adapter in `crates/paladin-llm/src/anthropic/adapter.rs`
  - Auth: `ANTHROPIC_API_KEY` environment variable
  - Base URL: `https://api.anthropic.com/v1` (configurable via `ANTHROPIC_BASE_URL`)
  - Features: Vision support (multimodal), thinking blocks support, streaming responses
  - Config struct: `AnthropicConfig` in `crates/paladin-llm/src/anthropic/adapter.rs`
  - Adapter location: `crates/paladin-llm/src/anthropic/adapter.rs`

- **DeepSeek** - DeepSeek large language models
  - SDK/Client: `reqwest` 0.12.4, wrapped by adapter in `crates/paladin-llm/src/deepseek/adapter.rs`
  - Auth: `DEEPSEEK_API_KEY` environment variable
  - Base URL: Configurable via `DEEPSEEK_BASE_URL` env var
  - Features: Streaming responses, cost-effective inference
  - Config struct: `DeepSeekConfig` in `crates/paladin-llm/src/deepseek/adapter.rs`
  - Adapter location: `crates/paladin-llm/src/deepseek/adapter.rs`

**Content & External Data APIs:**
- **News API** - News article aggregation (optional, feature: `news-api`)
  - Implementation: `crates/paladin-content/src/` (gated by feature flag)
  - Purpose: News source integration for content ingestion

- **Web Content Extraction** - HTTP-based web scraping (optional, feature: `web-scraping`)
  - SDK/Client: Scraper crate (CSS selectors) + reqwest for HTTP
  - Location: `crates/paladin-content/src/`

- **RSS Feed Ingestion** (optional, feature: `rss`)
  - SDK/Client: RSS crate for feed parsing
  - Location: `crates/paladin-content/src/`

**Etherscan Blockchain API** - Via MCP server
- **Service**: Etherscan official MCP server (remote Streamable-HTTP endpoint)
- **URL**: `https://mcp.etherscan.io/mcp`
- **Auth**: Bearer token via `ETHERSCAN_API_KEY` environment variable
- **Purpose**: Blockchain data queries through Model Context Protocol
- **Configuration**: `.mcp.json` (MCP server registry for Claude Code sessions only)

## Data Storage

**Databases:**

**SQLite** (built-in, default)
- Type: Embedded SQL database
- Client: SQLx 0.8 with async Tokio runtime
- Connection: File-based (URI in config or migration handling)
- Migrations: Via SQLx migrations in `migrations/` directory
- Features enabled: `sqlite` (always available)
- Use cases: Development, testing, small deployments
- Adapters: `crates/paladin-storage/src/` (SQLite repository implementations)
- Memory storage: `crates/paladin-memory/src/` (in-memory Garrison)

**MySQL** (optional, feature: `storage-mysql`)
- Type: Relational database
- Client: SQLx 0.8 with async Tokio runtime and rustls TLS
- Connection: Configured via connection string in `config.yml`
- Features enabled: `mysql` (opt-in via feature flag)
- Use cases: Production deployments requiring ACID compliance and scalability
- Adapters: `crates/paladin-storage/src/mysql/`
- Configuration: Passed via settings from `config.yml`

**File Storage:**

**MinIO / S3-Compatible Storage** (optional, feature: `s3-storage`)
- Service: MinIO S3 API or AWS S3
- Client: rust-s3 0.35.1
- Credentials: Configured via `config.yml` or environment
- Development setup: MinIO container via Docker Compose in `docker/docker-compose.dev.yml`
  - Default credentials (dev): `MINIO_ROOT_USER=devuser`, `MINIO_ROOT_PASSWORD=devpassword123`
  - Ports: 9000 (API), 9001 (Console)
- Use cases: Large document storage, state persistence, artifact management
- Adapter location: `crates/paladin-storage/src/` (S3 storage adapter)
- Environment variables (dev):
  - `APP_MINIO_ACCESS_KEY=devuser`
  - `APP_MINIO_SECRET_KEY=devpassword123`

**Vector/Semantic Search:**

**Qdrant** (optional, feature: `qdrant`)
- Type: Vector database for semantic search and RAG
- Client: qdrant-client 1.14
- Purpose: Store embeddings for similarity search, retrieval-augmented generation
- Configuration: `config.yml` with connection details
- Adapter location: `crates/paladin-memory/src/` (Sanctum/vector search implementation)
- Use cases: Semantic search across documents, RAG retrieval, similarity queries

**Caching:**

**Redis** (optional, feature: `redis-queue`)
- Type: In-memory cache and message queue
- Client: redis 0.32.2 with async Tokio and connection manager
  - Features: `aio`, `tokio-comp`, `connection-manager`, `script`
- Development setup: Redis container via Docker Compose in `docker/docker-compose.dev.yml`
  - Port: 6379
  - Config: `redis-dev.conf` (debug logging, AOF persistence)
- Use cases: Async job queuing, result caching, distributed locking
- Adapter location: `crates/paladin-storage/src/redis_adapter.rs`
- Production: Kubernetes manifests in `k8s/redis.yaml`

## Authentication & Identity

**Auth Provider:**
- **Custom In-Process**
  - Implementation: `src/infrastructure/adapters/auth/in_memory_token_auth_adapter.rs`
  - Approach: Static API key-based authentication for agent routes

**Authentication Methods:**

**API Key Authentication (HTTP Header)**
- Method: Static API keys sent via `X-API-Key` header
- Configuration: In `config.yml` under `http.auth.api_keys`
- Purpose: Service-to-service and client authentication
- Roles: `admin` (register/deregister agents), `user` (invoke agents)
- Key storage: Environment variables (never in VCS)
- Example env vars: `PALADIN_API_KEY_CI`, `PALADIN_API_KEY_APP`

**JWT Authentication**
- Method: Bearer token via Authorization header (optional, feature-gated)
- Configuration: In `config.yml` under `http.auth.jwt`
- Purpose: User identity assertion and role-based access control
- Status: Optional, not enabled by default (requires configuration)
- Token storage: In-process via `AuthPort` trait

**Authorization:**
- Role-based access control (RBAC) on agent invocation
- Agents specify allowed roles in configuration
- Default: Any authenticated caller if roles omitted

## Monitoring & Observability

**Error Tracking:**
- Not integrated - errors are logged locally via `log` crate
- Recommendation: Integrate with Sentry, Datadog, or similar via middleware

**Logs:**
- Approach: Structured logging via `env_logger` and `tracing-subscriber`
- Control: Environment variable `RUST_LOG` (e.g., `RUST_LOG=debug`)
- Output: Stdout/stderr (redirected by container runtime in production)
- Request correlation: Request ID middleware in HTTP layer
- Implementation: `src/infrastructure/adapters/logs/system_log_adapter.rs`

**Metrics:**
- Kubernetes annotations for Prometheus scraping:
  - `prometheus.io/scrape: "true"`
  - `prometheus.io/port: "9090"`
  - `prometheus.io/path: "/metrics"`
- Status: Scaffolding in place; requires metrics export implementation

**Health Checks:**
- Liveness probe: `/health/live` endpoint
- Readiness probe: `/health/ready` endpoint
- Location: `crates/paladin-web/src/health.rs`

## CI/CD & Deployment

**Hosting:**

**Kubernetes** (recommended for production)
- Manifests: `k8s/` directory
  - `k8s/namespace.yaml` - Paladin namespace
  - `k8s/deployment.yaml` - Agent deployment (3 replicas, rolling update)
  - `k8s/service.yaml` - Service exposure
  - `k8s/redis.yaml` - Redis StatefulSet
  - `k8s/minio.yaml` - MinIO deployment
  - `k8s/configmap.yaml` - Configuration map

**Docker** (containerized deployments)
- Multi-stage Dockerfile: `Dockerfile`
  - Builder stage: Rust 1.93 on Debian Bookworm with build dependencies
  - Runtime stage: Debian 12-slim with minimal footprint
  - Binary stripping: Debug symbols removed for size optimization
- Docker Compose configurations:
  - Development: `docker/docker-compose.dev.yml` (with hot-reload volumes)
  - Test: `docker/docker-compose.test.yml`
  - Server: `docker/docker-compose.server.yml`
  - Integration: `docker/docker-compose.yml` (base)

**CI Pipeline:**
- Not integrated in this codebase (GitHub Actions workflow not included in this scan)
- Recommendation: Use GitHub Actions or equivalent for:
  - Cargo build/test on Rust toolchain
  - Security audit via `cargo-audit`
  - Clippy linting via `cargo clippy`
  - Container image builds and registry push
  - Deployment to Kubernetes clusters

## Environment Configuration

**Required env vars:**
- `OPENAI_API_KEY` - For OpenAI LLM provider (if used)
- `ANTHROPIC_API_KEY` - For Anthropic LLM provider (if used)
- `DEEPSEEK_API_KEY` - For DeepSeek LLM provider (if used)

**Optional env vars:**
- `RUST_LOG` - Logging level (default: depends on build mode)
- `RUST_BACKTRACE` - Backtrace verbosity (0/1/full)
- `APP_ENV` - Environment stage (development/production)
- `ETHERSCAN_API_KEY` - Etherscan MCP server authentication

**Secrets location:**
- Development: `.env` file (gitignored, never committed)
- Production: Kubernetes Secrets, Docker secrets, or secure secret manager (Vault, AWS Secrets Manager, etc.)
- Never: Commit API keys, passwords, or secrets to VCS

**Configuration files:**
- `config.yml` - Application configuration (gitignored, provided at runtime)
- `config.example.yml` - Template for configuration (includes schema documentation)
- `config.test.yml` - Test-specific configuration
- YAML schema: Defines agents, timeouts, HTTP layers, LLM settings, storage, etc.

## Webhooks & Callbacks

**Incoming Webhooks:**
- Not implemented - agent invocation is request-response only via HTTP REST

**Outgoing Webhooks:**
- Not implemented - no built-in event callbacks to external systems
- Potential: Could be added via notification adapters (email/push/system)

**Job Status Polling:**
- Async execution via `/v1/agents/<id>/jobs` endpoint
- Poll `/v1/agents/<id>/jobs/<job_id>` for status
- Job state stored in in-memory `JobStore` (location: `crates/paladin-web/src/job_store.rs`)

## MCP (Model Context Protocol) Integration

**MCP Server Discovery:**
- Client-side: rmcp 2.1.0 SDK with Streamable-HTTP transport
- Configuration: `.mcp.json` (project-scoped MCP server registry)
- Supported transports:
  - STDIO: Subprocess-based servers (e.g., local MCP tools)
  - Streamable-HTTP: Remote HTTP-based servers (e.g., Etherscan official MCP)

**Arsenal (Tool Integration):**
- Adapters for MCP tool discovery and invocation:
  - STDIO adapter: `src/infrastructure/adapters/arsenal/mcp_stdio_adapter.rs`
  - Streamable-HTTP adapter: `src/infrastructure/adapters/arsenal/mcp_streamable_http_adapter.rs`
- Pattern: Arsenal registry manages available tools
- Purpose: Extend Paladin agent capabilities with external tools

---

*Integration audit: 2026-07-30*
