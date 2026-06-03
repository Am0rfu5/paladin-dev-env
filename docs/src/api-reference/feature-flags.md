# Paladin Feature Flags

Paladin uses Cargo feature flags to enable fine-grained control over compiled dependencies and functionality. This allows you to build minimal, focused binaries for specific use cases while reducing compile times and binary sizes.

> See also the [Crate Map & Feature Flags](crate-map.md) reference for per-crate flag tables, the crate dependency graph, and copy-paste consumer profiles.

## Table of Contents

- [Overview](#overview)
- [Available Feature Flags](#available-feature-flags)
- [Default Configuration](#default-configuration)
- [Usage Examples](#usage-examples)
- [Build Comparison](#build-comparison)
- [Feature Dependencies](#feature-dependencies)
- [Best Practices](#best-practices)

## Overview

### Philosophy

Feature flags in Paladin follow these principles:

1. **Core Framework Always Available** - Paladin agents, Battalion orchestration, Garrison memory, Arsenal tools, and Herald formatters are always compiled
2. **Provider Choice** - Choose which LLM providers to support (OpenAI, Anthropic, DeepSeek)
3. **Subsystem Opt-In** - Enable only the subsystems you need (web servers, content processing, notifications)
4. **Infrastructure Selection** - Pick storage/queue adapters (Redis, S3/MinIO, Qdrant)
5. **Testing Flexibility** - Enable integration tests only when needed

### Default vs. Full

| Configuration | Features Enabled | Use Case |
|--------------|------------------|----------|
| **Default** | `llm-openai` only | Production orchestration with OpenAI |
| **Full** | All optional features | Development, testing, full functionality |
| **No Default** | Core framework only | Library usage, custom integrations |

## Available Feature Flags

### LLM Provider Flags

| Flag | Dependencies | Modules Gated | Description |
|------|--------------|---------------|-------------|
| `llm-openai` | None (uses `reqwest`) | `infrastructure::adapters::llm::openai_adapter` | OpenAI GPT models (GPT-3.5, GPT-4, GPT-4-turbo, GPT-4o) |
| `llm-anthropic` | None (uses `reqwest`) | `infrastructure::adapters::llm::anthropic_adapter` | Anthropic Claude models (Claude 3 Opus, Sonnet, Haiku) |
| `llm-deepseek` | None (uses `reqwest`) | `infrastructure::adapters::llm::deepseek_adapter` | DeepSeek models (DeepSeek-V3, DeepSeek-Chat) |
| `llm-all` | `llm-openai`, `llm-anthropic`, `llm-deepseek` | All LLM adapters | All supported LLM providers |

### Subsystem Flags

| Flag | Dependencies | Modules Gated | Description |
|------|--------------|---------------|-------------|
| `vision` | None | Vision-related types, prompt builders | Enable vision capabilities for multimodal LLM interactions |
| `content-processing` | `pdf-extract`, `scraper`, `tiktoken-rs`, `rss` | Content extraction, tokenization | PDF parsing, web scraping, RSS feeds, token counting |
| `web-server` | `actix-web`, `axum` | REST API controllers, server setup | HTTP/REST API servers for user management and content delivery |
| `notifications` | `lettre`, `handlebars` | Email adapter, templating | Email notifications with template rendering |

### Storage & Queue Flags

| Flag | Dependencies | Modules Gated | Description |
|------|--------------|---------------|-------------|
| `redis-queue` | `redis` | `infrastructure::adapters::queue::redis` | Redis-based async queue adapter |
| `s3-storage` | `rust-s3` | `infrastructure::adapters::file_storage::minio` | S3/MinIO file storage adapter |
| `openai-embeddings` | None | Embedding generation utilities | OpenAI embedding model support |
| `qdrant` | `qdrant-client` | Qdrant vector database adapter | Vector database for semantic search |
| `storage-sqlite` | `sqlx` (sqlite) | `paladin-storage` SQLite adapters | SQLite-based persistent repository |
| `storage-mysql` | `sqlx` (mysql) | `paladin-storage` MySQL adapters | MySQL-based persistent repository |
| `storage` | `storage-sqlite`, `storage-mysql` | Both storage adapters | Convenience flag enabling both DB backends |

### Special Build Flags

| Flag | Description |
|------|-------------|
| `vendored-openssl` | Statically compile OpenSSL from source. Used for cross-compiled release binaries that lack a target-arch system libssl. |

### CLI Flags

| Flag | Dependencies | Modules Gated | Description |
|------|--------------|---------------|-------------|
| `cli` | `clap`, `dialoguer`, `indicatif`, `console`, `serde_yaml` | `application::cli` | Command-line tooling for the `paladin-cli` binary |

Build the `paladin-cli` binary with:
```bash
cargo build --bin paladin-cli --features cli
```

### Testing Flags

| Flag | Dependencies | Modules Gated | Description |
|------|--------------|---------------|-------------|
| `integration-tests` | None | Integration test modules | Enable integration tests (Docker services required) |
| `live-api-tests` | None | Live API test modules | Tests requiring real API keys (OpenAI, Anthropic, DeepSeek) |

### Convenience Flags

| Flag | Enables | Description |
|------|---------|-------------|
| `full` | `llm-all`, `content-processing`, `web-server`, `notifications`, `vision`, `redis-queue`, `s3-storage`, `openai-embeddings`, `qdrant`, `cli` | All optional features for development/testing |

## Default Configuration

**Current Default** (as of v0.5.0):

```toml
[dependencies]
paladin-ai = "0.5"
```

This enables **only**:
- ✅ `llm-openai` - OpenAI LLM provider
- ✅ Core framework (always available)

**Previous Default** (before v0.5.0):

```toml
# Old default - no longer applies
default = ["redis-queue", "s3-storage", "openai-embeddings"]
```

See [migration-guide.md](migration-guide.md) for migration guidance.

## Usage Examples

### Minimal Build (Core Only)

No external LLM providers, storage, or queues:

```toml
[dependencies]
paladin-ai = { version = "0.5", default-features = false }
```

**Use case**: Custom LLM integrations, library embedding, edge deployments

### Single Provider Builds

**OpenAI Only** (default):
```toml
[dependencies]
paladin-ai = "0.5"
# Or explicitly:
paladin-ai = { version = "0.5", features = ["llm-openai"] }
```

**Anthropic Only**:
```toml
[dependencies]
paladin-ai = { version = "0.5", default-features = false, features = ["llm-anthropic"] }
```

**DeepSeek Only**:
```toml
[dependencies]
paladin-ai = { version = "0.5", default-features = false, features = ["llm-deepseek"] }
```

### Multi-Provider Builds

**All LLM Providers**:
```toml
[dependencies]
paladin-ai = { version = "0.5", default-features = false, features = ["llm-all"] }
```

**OpenAI + Anthropic**:
```toml
[dependencies]
paladin-ai = { version = "0.5", default-features = false, features = ["llm-openai", "llm-anthropic"] }
```

### Orchestration Platform Build

Agents + web API + Redis queue + S3 storage:

```toml
[dependencies]
paladin-ai = { version = "0.5", features = ["web-server", "redis-queue", "s3-storage"] }
```

### Content Processing Build

Content ingestion + processing + all providers:

```toml
[dependencies]
paladin-ai = { version = "0.5", features = ["llm-all", "content-processing", "qdrant", "s3-storage"] }
```

### Full Development Build

All features enabled:

```toml
[dependencies]
paladin-ai = { version = "0.5", features = ["full"] }
```

Or use the CLI:

```bash
cargo build --features full
cargo test --features full
```

### Production API Server

Web server + notifications + OpenAI + storage:

```toml
[dependencies]
paladin-ai = { version = "0.5", features = ["web-server", "notifications", "redis-queue", "s3-storage"] }
```

## Build Comparison

### Binary Size Comparison

| Configuration | Features | Dependencies | Approx. Binary Size* | Compile Time* |
|---------------|----------|--------------|---------------------|---------------|
| Core Only | None | ~50 crates | 8-12 MB | 30-45s |
| Default | `llm-openai` | ~55 crates | 10-14 MB | 40-60s |
| Full | All | ~120 crates | 25-35 MB | 3-5 min |

*Approximate values for release builds on x86_64 Linux. Actual values vary by system.

### Compile Time Optimization

**Fast iteration** (core only):
```bash
cargo build --no-default-features
cargo test --lib --no-default-features
```

**Full testing** (all features):
```bash
cargo test --features full
```

## Feature Dependencies

### Dependency Tree

```
full
├── llm-all
│   ├── llm-openai
│   ├── llm-anthropic
│   └── llm-deepseek
├── content-processing
│   ├── pdf-extract
│   ├── scraper
│   ├── tiktoken-rs
│   └── rss
├── web-server
│   ├── actix-web
│   └── axum
├── notifications
│   ├── lettre
│   └── handlebars
├── vision
├── redis-queue
│   └── redis
├── s3-storage
│   └── rust-s3
├── openai-embeddings
└── qdrant
    └── qdrant-client
```

### Conditional Compilation Examples

**In Your Code:**

```rust,ignore
// Always available (core framework)
use paladin::core::platform::container::paladin::Paladin;
use paladin::application::services::paladin::paladin_builder::PaladinBuilder;

// Conditionally compiled
#[cfg(feature = "llm-openai")]
use paladin::infrastructure::adapters::llm::openai_adapter::OpenAIAdapter;

#[cfg(feature = "redis-queue")]
use paladin::infrastructure::adapters::queue::redis::RedisQueueAdapter;

#[cfg(feature = "web-server")]
use paladin::infrastructure::web::server::start_web_server;
```

## Best Practices

### 1. Start Minimal, Add as Needed

Begin with default features, add others only when required:

```toml
# Start here
[dependencies]
paladin-ai = "0.5"

# Add features as needed
paladin-ai = { version = "0.5", features = ["redis-queue"] }
```

### 2. Use `full` for Development Only

Enable all features during development, but specify exact features for production:

```toml
[dependencies]
# Production - explicit features
paladin-ai = { version = "0.5", features = ["llm-anthropic", "s3-storage"] }

[dev-dependencies]
# Development - all features
paladin-ai = { version = "0.5", features = ["full"] }
```

### 3. Document Feature Requirements

If your application requires specific features, document them:

```rust,ignore
//! # Example Application
//!
//! **Required Features:**
//! ```toml
//! paladin-ai = { version = "0.5", features = ["llm-openai", "redis-queue", "s3-storage"] }
//! ```
```

### 4. Test with Multiple Feature Combinations

Use CI to test critical combinations:

```yaml
# .github/workflows/ci.yml
strategy:
  matrix:
    features:
      - "--no-default-features"
      - ""  # default
      - "--features full"
```

See [.github/workflows/](https://github.com/DF3NDR/paladin-dev-env/tree/main/.github/workflows) for Paladin's complete feature matrix testing.

### 5. Feature-Gate Examples

Add feature requirements to example documentation:

```rust,ignore
//! # Redis Queue Example
//!
//! **Required Cargo Features:**
//! ```toml
//! paladin-ai = { version = "0.5", features = ["redis-queue"] }
//! ```
//!
//! Run with: `cargo run --example redis_queue --features redis-queue`
```

## Migration Guide

If you're upgrading from a version before the feature flag reorganization, see [migration-guide.md](migration-guide.md) for detailed migration instructions.

## CI/CD Integration

### GitHub Actions

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        features:
          - ""                              # default
          - "--no-default-features"         # core only
          - "--features full"               # all features
          - "--features llm-anthropic"      # specific provider
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Test
        run: cargo test ${{ matrix.features }}
```

### Docker Multi-Stage Builds

```dockerfile
# Builder with only needed features
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features "llm-openai,redis-queue,s3-storage"

# Runtime image
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/paladin /usr/local/bin/
CMD ["paladin"]
```

## Support

For issues or questions about feature flags:
- **Documentation**: [Configuration Guide](../getting-started/configuration.md)
- **Migration**: [Migration Guide](migration-guide.md)
- **Issues**: [GitHub Issues](https://github.com/DF3NDR/paladin-dev-env/issues)
- **Discussions**: [GitHub Discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)
