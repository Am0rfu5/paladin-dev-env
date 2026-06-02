# Installation Guide

This guide provides detailed installation instructions for Paladin on Linux, macOS, and Windows.

## Prerequisites

### Required

- **Rust 1.70 or later**: [https://rustup.rs/](https://rustup.rs/)
- **Cargo**: Included with Rust installation
- **LLM API Key**: OpenAI, DeepSeek, or Anthropic account

### Optional

- **Docker**: For containerized deployment (see [Docker Guide](../deployment/docker.md))
- **Redis**: For async queue functionality (see [Development Setup](#development-setup))
- **MinIO**: For file storage (see [Development Setup](#development-setup))

## Platform-Specific Setup

### Linux

#### Ubuntu/Debian

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Verify installation
rustc --version
cargo --version
```

#### Fedora/RHEL/CentOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies
sudo dnf install -y gcc pkg-config openssl-devel

# Verify installation
rustc --version
cargo --version
```

#### Arch Linux

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies
sudo pacman -S base-devel openssl pkg-config

# Verify installation
rustc --version
cargo --version
```

### macOS

#### Using Homebrew

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install OpenSSL (if needed)
brew install openssl pkg-config

# Verify installation
rustc --version
cargo --version
```

#### Apple Silicon (M1/M2/M3)

Rust supports Apple Silicon natively. No additional steps required:

```bash
# Verify architecture
rustc --version --verbose | grep host
# Should show: host: aarch64-apple-darwin
```

### Windows

#### Using rustup-init.exe

1. Download rustup-init.exe from [https://rustup.rs/](https://rustup.rs/)
2. Run the installer and follow prompts
3. Restart your terminal

```powershell
# Verify installation
rustc --version
cargo --version
```

#### Using WSL2 (Recommended for Development)

```bash
# Inside WSL2 Ubuntu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Verify installation
rustc --version
cargo --version
```

## Installing Paladin

### Option 1: From Crates.io (Stable)

```bash
# Add Paladin to your project
cargo add paladin

# Or manually edit Cargo.toml
```

```toml
[dependencies]
paladin = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Option 2: From Source (Latest)

```bash
# Clone the repository
git clone https://github.com/DF3NDR/paladin-dev-env.git
cd paladin-dev-env

# Build the project
cargo build --release

# Run tests to verify
cargo test

# Optionally install CLI tools
cargo install --path .
```

### Option 3: As a Dependency from Git

```toml
[dependencies]
paladin = { git = "https://github.com/DF3NDR/paladin-dev-env", branch = "main" }
tokio = { version = "1", features = ["full"] }
```

## Feature Flags

Paladin supports optional features that can be enabled in `Cargo.toml`:

```toml
[dependencies.paladin]
version = "0.1"
features = [
    "redis-queue",      # Enable Redis queue adapter (default)
    "s3-storage",       # Enable MinIO/S3 storage (default)
    "anthropic",        # Enable Anthropic LLM provider
    "deepseek",         # Enable DeepSeek LLM provider
    "mcp",              # Enable MCP tool protocol
]
```

### Default Features

Enabled by default:
- `redis-queue` - Redis-based async queue
- `s3-storage` - MinIO/S3 file storage

### Optional Features

Not enabled by default:
- `anthropic` - Anthropic Claude integration
- `deepseek` - DeepSeek LLM integration
- `mcp` - Model Context Protocol for tools

Disable default features:

```toml
[dependencies.paladin]
version = "0.1"
default-features = false
features = ["mcp"]  # Only enable MCP
```

## Environment Configuration

### API Keys

Create a `.env` file in your project root:

```bash
# OpenAI (default provider)
OPENAI_API_KEY=sk-your-api-key-here
OPENAI_BASE_URL=https://api.openai.com/v1  # Optional

# DeepSeek
DEEPSEEK_API_KEY=your-deepseek-key
DEEPSEEK_BASE_URL=https://api.deepseek.com/v1  # Optional

# Anthropic
ANTHROPIC_API_KEY=your-anthropic-key
ANTHROPIC_BASE_URL=https://api.anthropic.com/v1  # Optional
```

### Configuration File

Create `config.yml` (optional):

```yaml
paladin:
  default_model: "gpt-4"
  default_temperature: 0.7
  default_max_loops: 3
  timeout_seconds: 300

garrison:
  type: "sqlite"  # or "in_memory"
  path: "./garrison.db"
  max_entries: 1000

llm:
  openai:
    api_key: "${OPENAI_API_KEY}"
    base_url: "https://api.openai.com/v1"
```

## Development Setup

For local development with all features:

### 1. Clone the Repository

```bash
git clone https://github.com/DF3NDR/paladin-dev-env.git
cd paladin-dev-env
```

### 2. Install Development Dependencies

```bash
# Install additional cargo tools
cargo install cargo-watch      # Auto-rebuild on changes
cargo install cargo-edit        # cargo add/rm commands
cargo install cargo-audit       # Security vulnerability scanning
cargo install cargo-llvm-cov    # Code coverage
cargo install cargo-insta       # Snapshot testing (for CLI output tests)
```

**cargo-insta** is used for CLI snapshot testing. It allows you to capture and verify terminal output:

```bash
# Run snapshot tests
cargo test --test cli

# Review new snapshots
cargo insta review

# Accept all pending snapshots
cargo insta accept
```

See [`tests/cli/`](https://github.com/DF3NDR/paladin-dev-env/tree/main/tests/cli) for snapshot test examples.

### 3. Start Docker Services (Optional)

```bash
# Start Redis and MinIO
make dev

# Or manually with docker-compose
docker-compose -f docker/docker-compose.dev.yml up -d
```

### 4. Configure Environment

```bash
# Copy example environment
cp .env.example .env

# Edit with your API keys
vim .env
```

### 5. Build and Test

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with auto-reload
cargo watch -x run
```

## Verification

### Quick Test

Create `test.rs`:

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Paladin version: {}", env!("CARGO_PKG_VERSION"));
    println!("Installation successful!");
    Ok(())
}
```

Run:

```bash
cargo run --example test
```

### Full System Test

```bash
# Run all tests
cargo test

# Run integration tests (requires Docker services)
make test-integration-docker

# Run benchmarks
cargo bench
```

## Troubleshooting

### OpenSSL Errors (Linux)

```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install pkgconfig openssl-devel

# Arch
sudo pacman -S openssl pkg-config
```

### Linking Errors (Windows)

Install Visual Studio Build Tools:
- Download from [https://visualstudio.microsoft.com/downloads/](https://visualstudio.microsoft.com/downloads/)
- Select "Desktop development with C++"

### Permission Errors (macOS)

```bash
# Fix cargo permissions
sudo chown -R $(whoami) ~/.cargo
```

### Slow Compilation

Enable parallel compilation:

```bash
# Add to ~/.cargo/config.toml
[build]
jobs = 8  # Adjust based on CPU cores
```

Use `sccache` for caching:

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### Network Issues

Use a proxy:

```bash
# Set in ~/.cargo/config.toml
[http]
proxy = "http://proxy.example.com:8080"

[https]
proxy = "http://proxy.example.com:8080"
```

Or use a mirror:

```bash
[source.crates-io]
replace-with = "ustc"

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

## Next Steps

- **[Quickstart Guide](quickstart.md)** - Build your first Paladin agent
- **[Configuration Guide](../user-guides/paladin-configuration.md)** - Advanced configuration
- **[Examples](https://github.com/DF3NDR/paladin-dev-env/tree/main/examples)** - Working code examples
- **[API Reference](https://docs.rs/paladin)** - Complete API documentation

## Platform Support

| Platform | Architecture | Status | Notes |
|----------|-------------|--------|-------|
| Linux | x86_64 | ✅ Tested | Primary development platform |
| Linux | aarch64 | ✅ Tested | ARM servers, Raspberry Pi |
| macOS | x86_64 | ✅ Tested | Intel Macs |
| macOS | aarch64 | ✅ Tested | Apple Silicon (M1/M2/M3) |
| Windows | x86_64 | ⚠️ Experimental | WSL2 recommended |
| Windows | aarch64 | ❌ Untested | May work with WSL2 |

## Minimum System Requirements

- **CPU**: 2 cores (4+ recommended for parallel operations)
- **RAM**: 4 GB (8+ GB recommended)
- **Disk**: 2 GB for dependencies and builds
- **Network**: Internet connection for LLM API calls

## Get Help

- **Installation Issues**: [GitHub Issues](https://github.com/DF3NDR/paladin-dev-env/issues)
- **General Questions**: [GitHub Discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)
- **Documentation**: [docs/](../introduction.md)
