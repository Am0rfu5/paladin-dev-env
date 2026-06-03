# Installation

This guide covers adding Paladin to an existing Rust project or setting up the
Paladin workspace for development.

## Prerequisites

### Required

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| **Rust** | 1.85.0 | Latest stable (1.95+) |
| **Cargo** | Included with Rust | - |
| **Edition** | 2024 | 2024 |
| **LLM API Key** | At least one | - |

> **Why Rust >= 1.85?** Paladin uses edition 2024 features. Verify your toolchain:
> ```bash
> rustc --version   # should print >= 1.85.0
> ```
> Update with `rustup update stable`.

### Optional (for Docker-based services)

- **Docker + Docker Compose** v2 -- required for the built-in Redis, MinIO, and
  MySQL services (see [Docker Guide](../deployment/docker.md))

## Installing Rust

```bash
# Install rustup and the stable toolchain
curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build tools -- Linux (Ubuntu/Debian)
sudo apt-get install -y build-essential pkg-config libssl-dev

# Build tools -- macOS (via Homebrew)
brew install openssl pkg-config
```

Windows users should use [rustup-init.exe](https://rustup.rs/) or WSL 2.

## Adding Paladin to a Rust Project

### Cargo.toml -- choose your crates

Paladin v0.5.0 is published as a workspace of focused crates. Add only what you need:

```toml
[dependencies]
# Core framework -- always required
paladin-ai-core   = "0.5.0"
paladin-ports     = "0.5.0"

# LLM providers (pick one or more)
paladin-llm       = { version = "0.5.0", features = ["llm-openai"] }

# Multi-agent orchestration (optional)
paladin-battalion = "0.5.0"

# Memory / Garrison (optional)
paladin-memory    = "0.5.0"

# Storage adapters (optional)
paladin-storage   = "0.5.0"

# Async runtime (required)
tokio = { version = "1", features = ["full"] }
```

### Umbrella crate

The `paladin-ai` umbrella crate (v0.5.0) re-exports everything and accepts workspace feature flags:

```toml
[dependencies]
paladin-ai = { version = "0.5.0", features = ["redis-queue", "s3-storage"] }
tokio      = { version = "1", features = ["full"] }
```

### Feature Flag Profiles

| Flag | Default | Description |
|------|---------|-------------|
| `llm-openai` | yes | OpenAI GPT adapter |
| `redis-queue` | no | Redis async task queue |
| `s3-storage` | no | MinIO / AWS S3 file storage |
| `openai-embeddings` | no | OpenAI embedding API |
| `qdrant` | no | Qdrant vector database for Sanctum |

### Verification

```bash
cargo check
```

No errors means all selected features resolved correctly.

## Cloning the Source for Development

```bash
# 1. Clone
git clone https://github.com/DF3NDR/paladin-dev-env.git
cd paladin-dev-env

# 2. Build the workspace
cargo build

# 3. Run unit tests
cargo test --workspace --lib

# 4. (Optional) Start backing services
make services-up   # Redis, MinIO, MySQL via Docker Compose
```

See [Development Setup](../contributing/development-setup.md) for the full contributor workflow.

## Environment Variables for LLM Keys

Paladin reads API keys exclusively from environment variables -- **never put keys in config files**.

```bash
# Set at least one provider key before running
export OPENAI_API_KEY="sk-..."       # OpenAI
export DEEPSEEK_API_KEY="sk-..."     # DeepSeek
export ANTHROPIC_API_KEY="sk-..."    # Anthropic
```

Copy `.env.example` to `.env` for local development (`.env` is git-ignored).

## Next Steps

- **[Quickstart](quickstart.md)** -- write your first Paladin agent in minutes
- **[Configuration](configuration.md)** -- full `config.yml` schema reference
- **[User Guides](../user-guides/paladin-agents.md)** -- in-depth agent patterns
