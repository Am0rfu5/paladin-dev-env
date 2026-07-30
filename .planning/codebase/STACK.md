# Technology Stack

**Analysis Date:** 2026-07-30

## Languages

**Primary:**
- Rust 1.97.1 (pinned in `rust-toolchain.toml`) - Enterprise multi-agent orchestration framework
- Edition: 2021 (stated in crate Cargo.toml files)

**Build scripting:**
- Makefile for development workflow
- Cargo build system (Rust's standard)

## Runtime

**Environment:**
- Tokio async runtime (version 1.x) with full features enabled
- Linux-based deployment (Debian 12-slim for production)

**Package Manager:**
- Cargo (Rust's official package manager)
- Lockfile: `Cargo.lock` (committed to repo for reproducible builds)

## Frameworks

**Core:**
- **Axum** 0.8.4 - HTTP web framework for the `paladin-server` binary (optional, gated by `web-server` feature)
  - Routing and request handling for agent execution endpoints
  - Location: `crates/paladin-web/`

**Async/Concurrency:**
- **Tokio** 1.x - Async runtime for all async operations
- **Futures** 0.3 - Future combinators and utilities

**Testing:**
- **Mockito** 1.7.0 - HTTP mocking for adapter integration tests
- **Testcontainers** 0.24.0 - Docker container management for integration tests
- **Criterion** 0.5 - Benchmarking framework (async support via `criterion-async_tokio`)
- **Proptest** 1.4 - Property-based testing
- **Serial_test** 3.2.0 - Test serialization for thread-safety
- **Wiremock** 0.6 - Standalone HTTP mocking (dev-only)

**Build/Dev:**
- **cargo-audit** - Security vulnerability scanning (run via `make audit`)
- **cargo-clippy** - Linting and code quality (run via `make clean-code`)
- **cargo-fmt** - Code formatting (run via `make clean-code`)

## Key Dependencies

**Critical Core:**
- **Serde** 1.0 (with derive feature) - Serialization/deserialization for all data types
- **Serde_json** 1.0 - JSON serialization
- **Tokio** 1.x - Async runtime for all async operations and concurrency
- **Async-trait** 0.1.88 - Async trait support for port definitions
- **Thiserror** 2.0 - Custom error type derivation and handling

**Data & Persistence:**
- **SQLx** 0.8 - Async SQL toolkit with compile-time query verification
  - Features: `runtime-tokio-rustls`, `sqlite`, `chrono`, `uuid`, `json`, `migrate`
  - Supports both SQLite and MySQL (optional, feature-gated)
- **Qdrant-client** 1.14 (optional) - Vector database client for semantic search/RAG (gated by `qdrant` feature)

**LLM Integration:**
- **Reqwest** 0.12.4 (and 0.13.x aliased as `reqwest_mcp`) - HTTP client for LLM APIs and MCP
- **OpenAI adapter** - GPT-3.5, GPT-4, GPT-4o support (feature: `openai`)
- **Anthropic adapter** - Claude model support (feature: `anthropic`)
- **DeepSeek adapter** - DeepSeek model support (feature: `deepseek`)
- **Mock adapter** - Testing and deterministic workflows (feature: `mock`, enabled by default)

**MCP (Model Context Protocol):**
- **rmcp** 2.1.0 (pinned exact version) - Official MCP SDK for Rust
  - Features: `client`, `transport-child-process`, `transport-streamable-http-client`, `transport-streamable-http-client-reqwest`, `reqwest`
  - Supports both STDIO (subprocess) and Streamable-HTTP (remote) MCP servers

**Cache & Queue:**
- **Redis** 0.32.2 (optional, feature: `redis-queue`) - Async job queuing and caching
  - Features: `aio`, `tokio-comp`, `connection-manager`, `script`

**File Storage:**
- **Rust-s3** 0.35.1 (optional, feature: `s3-storage`) - MinIO/S3 compatible file storage

**Content Processing:**
- **Pdf-extract** 0.7 - PDF document extraction
- **Scraper** 0.23.1 (optional, feature: `web-scraping`) - HTML/CSS scraping
- **RSS** 2.0 (optional, feature: `rss`) - RSS feed parsing
- **Tiktoken-rs** 0.6.0 (optional, feature: `content-processing`) - Token counting for LLM context

**Authentication & Encryption:**
- **Argon2** 0.5.3 - Password hashing
- **ChaCha20Poly1305** 0.10 - AEAD encryption for sensitive data
- **Zeroize** 1.8 - Secure memory wiping for secrets

**Notifications:**
- **Lettre** 0.11.17 (optional, feature: `email`) - SMTP email sending
  - Features: `smtp-transport`, `pool`, `hostname`, `builder`
- **Handlebars** 6.3.2 (optional, feature: `email`) - Email template rendering

**Scheduling:**
- **Tokio-cron-scheduler** 0.13 (optional, feature: `scheduler`) - Async cron-based task scheduling

**CLI:**
- **Structopt** 0.3 - Ergonomic argument parsing (legacy)
- **Clap** 4.5.40 (optional, feature: `cli`) - Argument parsing with derive macros
  - Features: `derive`, `cargo`, `env`
- **Dialoguer** 0.11 (optional, feature: `cli`) - Interactive terminal dialogs
- **Indicatif** 0.17 (optional, feature: `cli`) - Progress bar rendering
- **Console** 0.15 (optional, feature: `cli`) - Terminal color and formatting
- **Serde_yaml** 0.9 (optional, feature: `cli`) - YAML serialization for CLI config

**Logging & Observability:**
- **Log** 0.4.21 - Logging facade (used throughout)
- **Env_logger** 0.11.3 - Environment-based logging configuration
- **Tracing-subscriber** 0.3 - Structured tracing and filtering

**Utilities:**
- **UUID** 1.8.0 - UUID generation (v4, serde support)
- **Chrono** 0.4.38 - Date/time handling with serde support
- **URL** 2.5.2 - URL parsing and manipulation
- **Regex** 1.11.1 - Regular expression support
- **Base64** 0.22 - Base64 encoding/decoding
- **Blake3** 1.8.2 - Cryptographic hashing
- **SHA2** 0.10.9 - SHA-256 hashing
- **MD5** 0.8.0 - MD5 hashing (legacy support)
- **Murmur3** 0.5.2 - Murmur3 hashing for deduplication
- **Petgraph** 0.6 - Graph data structures (DAG orchestration)
- **Bytes** 1.10.1 - Efficient byte manipulation
- **Mime_guess** 2.0.5 - MIME type detection
- **Colored** 2.1 - Terminal color output
- **Comfy-table** 7.1 - ASCII table formatting
- **Once_cell** 1.19.0 - Lazy statics and once-initialized values
- **Lazy_static** 1.4.0 - Lazy static initialization
- **Lock_api** 0.4.12 - Synchronization primitives
- **Rand** 0.8 - Random number generation
- **Urlencoding** 2.1.3 - URL encoding/decoding
- **Tempfile** 3.10.1 - Temporary file creation
- **Toml** 0.8.23 - TOML parsing

**Web Framework Support:**
- **Tower** 0.5 - Modular middleware and utility types for HTTP services
- **Tower-http** 0.6 - Pre-built HTTP middleware (CORS, body limit, timeout)
  - Features: `cors`, `limit`, `timeout`
- **Tower_governor** 0.8 - Rate limiting middleware
  - Features: `axum`
- **Async-stream** 0.3 - Async generator syntax
- **Utoipa** 5.0 - OpenAPI specification generation
  - Features: `axum_extras`, `chrono`, `uuid`
- **Utoipa-axum** 0.2 - Axum integration for utoipa
- **Utoipa-swagger-ui** 9.0 - Swagger UI serving
  - Features: `axum`

**Configuration Management:**
- **Config** 0.15.11 - Multi-source configuration loading (YAML files, environment)
- **Dotenv** 0.15.0 - Load environment variables from `.env` files (dev-only in debug builds)

**Optional TLS:**
- **OpenSSL** 0.10 (optional, feature: `vendored-openssl`) - Static OpenSSL linking for cross-compilation
  - Used only for release binaries via `cross` targeting non-host architectures

## Configuration

**Environment:**
- Configuration loaded from YAML files (`config.yml`, `config.example.yml`)
- Environment variables for LLM API keys:
  - `OPENAI_API_KEY` - OpenAI API authentication
  - `ANTHROPIC_API_KEY` - Anthropic API authentication
  - `DEEPSEEK_API_KEY` - DeepSeek API authentication
  - `ETHERSCAN_API_KEY` - Etherscan MCP server authentication
- Runtime environment variables for services:
  - `RUST_LOG` - Logging level control
  - `RUST_BACKTRACE` - Backtrace verbosity
  - `APP_ENV` - Environment stage (development/production)
  - `APP_MINIO_ACCESS_KEY`, `APP_MINIO_SECRET_KEY` - MinIO credentials

**Build:**
- `Cargo.toml` - Workspace root with member crates and workspace dependencies
- Feature flags control optional subsystems:
  - LLM providers: `llm-openai`, `llm-anthropic`, `llm-deepseek`, `llm-all`
  - Subsystems: `vision`, `content-processing`, `web-server`, `notifications`
  - Storage: `redis-queue`, `s3-storage`, `storage-mysql`
  - Memory: `qdrant` (vector database)
  - CLI: `cli` (binary tools only)
  - Convenience: `full` (enables all optional features)

## Platform Requirements

**Development:**
- Rust 1.97.1 (via `rustup`, configured in `rust-toolchain.toml`)
- Docker or Docker Desktop (for integration tests via testcontainers)
- Make (for development workflow commands)
- Linux or Linux-compatible shell environment (Git Bash on Windows)
- For dev services: Docker Compose

**Production:**
- Deployment target: Kubernetes (reference manifests in `k8s/`) or Docker
- Linux-based OS (Debian 12-slim in production Docker image)
- External dependencies:
  - Redis (for queuing, optional)
  - MinIO or AWS S3-compatible object storage (optional)
  - Qdrant vector database (optional)
  - MySQL or SQLite database backend
  - LLM provider API access (OpenAI, Anthropic, or DeepSeek)
- Port binding: 8080 (default HTTP server port, configurable via `config.yml`)

**Build Artifacts:**
- Multi-stage Docker build with optimizations:
  - Builder stage: Rust 1.93 on Debian Bookworm
  - Runtime stage: Debian 12-slim with minimal dependencies
  - Binary stripped of debug symbols
  - Release profile: LTO enabled, panic=abort, opt-level=3

---

*Stack analysis: 2026-07-30*
