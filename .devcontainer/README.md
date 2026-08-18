# Paladin DevContainer

Professional development container configuration for the Paladin multi-agent orchestration framework.

## Features

### 🛠️ Development Tools

- **Rust Toolchain**: Latest stable Rust with all components
- **Cargo Extensions**: watch, edit, audit, nextest, llvm-cov, and more
- **Debugging Tools**: gdb, lldb, valgrind, strace
- **Database Clients**: sqlite3, mysql-client, redis-tools
- **Modern CLI Tools**: bat, exa, ripgrep, fd, tokei
- **Security Scanning**: cargo-audit, cargo-deny, cargo-cyclonedx, and the [Snyk CLI](#snyk-credentials)

### 🔌 VS Code Extensions

Pre-configured with essential extensions:
- Rust Analyzer (LSP)
- Even Better TOML
- GitLens
- Docker
- Error Lens
- GitHub Copilot (if available)

### 🐳 Integrated Services

The DevContainer includes Docker Compose configuration for:
- **Redis**: Queue management and caching
- **MinIO**: S3-compatible object storage
- **MySQL**: Persistent database

### ⚙️ Configured Features

- **Formatting**: Auto-format on save with rustfmt
- **Linting**: Clippy enabled with strict warnings
- **Testing**: Integrated test explorer
- **Debugging**: Full LLDB debugging support
- **Git Hooks**: Pre-commit checks for format and lint

## Snyk credentials

The Snyk CLI is baked into the image (pinned in `Dockerfile.dev`), so it is present
on every rebuild. Credentials are **not** baked in — they come from the host.

### One-time host setup

The container's `/home/vscode` is part of the container filesystem and is destroyed
on every rebuild, so the key cannot live there. `docker-compose.yml` bind-mounts the
host's `~/.config/paladin` read-only at `/home/vscode/.config/paladin` instead, which
survives rebuilds and is shared with any other project using the same convention.

Run this **on the host**, before first launch:

```bash
mkdir -p ~/.config/paladin
printf '%s' '<your-snyk-token>' > ~/.config/paladin/snyk_api_key
chmod 600 ~/.config/paladin/snyk_api_key
```

Create the directory *before* starting the container — otherwise the Docker daemon
creates it for you, owned by root.

> **Note on the variable name.** The Snyk CLI authenticates from `SNYK_TOKEN` (and
> reads `SNYK_API` for a self-hosted or regional endpoint). It does **not** read
> `SNYK_API_KEY`. Paladin stores the secret under the name `SNYK_API_KEY`, and
> `.devcontainer/snyk-env.sh` is the single place that maps one to the other.

### How the key is resolved

`.devcontainer/snyk-env.sh` is sourced from `~/.bashrc` (wired up by `post-start.sh`)
and resolves the first of:

1. `SNYK_TOKEN` already exported — used as-is
2. `SNYK_API_KEY` already exported — mirrored into `SNYK_TOKEN`
3. `~/.config/paladin/snyk_api_key` — read, trimmed, exported as both

So an explicit environment variable always beats the file. `docker-compose.yml` also
passes `SNYK_API_KEY` / `SNYK_TOKEN` / `SNYK_API` through from the host shell when
they are set there. An empty or placeholder key file is ignored silently.

### Usage

```bash
make snyk-status   # version + whether credentials resolved (never prints the key)
make snyk-code     # static analysis (SAST) over first-party source
make snyk-deps     # dependency (SCA) scan of the Cargo workspace
make snyk          # both

snyk-status        # same check, available in any interactive shell
```

`make snyk-code` is the CLI equivalent of the `snyk_code_scan` step required by
`.github/instructions/snyk_rules.instructions.md` for new or modified first-party code.

## Quick Start

### Prerequisites

- [Visual Studio Code](https://code.visualstudio.com/)
- [Docker Desktop](https://www.docker.com/products/docker-desktop)
- [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) VS Code extension

### Opening the DevContainer

1. Open the Paladin project in VS Code
2. When prompted, click "Reopen in Container"
   - Or use Command Palette: `Remote-Containers: Reopen in Container`
3. Wait for the container to build (first time takes ~10-15 minutes)
4. The post-create script will automatically:
   - Fetch dependencies
   - Build the project
   - Set up git hooks
   - Configure aliases

### First Build

The initial build will:
1. Install all Rust components and tools
2. Fetch and compile all project dependencies
3. Set up the development environment
4. Configure shell aliases and helpers

**Note**: The first build can take 10-15 minutes depending on your machine and network speed. Subsequent starts are much faster (< 30 seconds).

## Usage

### Quick Commands (Aliases)

The DevContainer includes convenient aliases:

```bash
# Building and Testing
pd-build      # cargo build
pd-test       # cargo test
pd-run        # cargo run
pd-check      # cargo check
pd-watch      # cargo watch (auto-rebuild on changes)

# Code Quality
pd-fmt        # cargo fmt
pd-clippy     # cargo clippy with strict warnings
pd-clean-code # Format, lint, and check all at once

# Documentation and Analysis
pd-doc        # Generate and open documentation
pd-bench      # Run benchmarks
pd-audit      # Security audit
pd-outdated   # Check for outdated dependencies
pd-bloat      # Analyze binary size

# Services (via Makefile)
pd-dev        # Start all services
pd-services   # Start supporting services only
pd-test-all   # Run all tests including integration

# Navigation
pd-src        # cd to src/
pd-tests      # cd to tests/
pd-docs       # cd to docs/
pd-examples   # cd to examples/
```

### Starting Services

The DevContainer can run alongside Redis, MinIO, and MySQL:

```bash
# Start all services
pd-dev

# Or using docker-compose directly
docker-compose -f .devcontainer/docker-compose.yml up -d
```

### Service Endpoints

When services are running:
- **Paladin API**: http://localhost:8080
- **Metrics**: http://localhost:9090
- **Redis**: localhost:6379
- **MinIO API**: http://localhost:9000
- **MinIO Console**: http://localhost:9001 (minioadmin/minioadmin)
- **MySQL**: localhost:3306 (paladin/paladinpass)

### Running Tests

```bash
# Unit tests only
cargo test

# All tests
pd-test-all

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests
cargo test --test integration_test_name
```

### Debugging

1. Set breakpoints in your Rust code
2. Press `F5` or go to Run and Debug panel
3. Select "Debug Rust" configuration
4. The debugger will attach with full symbol support

### Code Quality Checks

Git pre-commit hooks automatically run:
- `cargo fmt --check` - Ensures code is formatted
- `cargo clippy -- -D warnings` - Ensures no clippy warnings

You can also run these manually:

```bash
pd-clean-code  # Runs fmt, clippy, and check
```

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:
```env
# LLM Providers
OPENAI_API_KEY=your_key_here
DEEPSEEK_API_KEY=your_key_here
ANTHROPIC_API_KEY=your_key_here

# Services
REDIS_URL=redis://redis:6379
MINIO_ENDPOINT=minio:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin

# Database
DATABASE_URL=mysql://paladin:paladinpass@mysql:3306/paladin
```

### Customizing the DevContainer

Edit `.devcontainer/devcontainer.json` to:
- Add VS Code extensions
- Change port forwarding
- Modify environment variables
- Add additional features

After changes, rebuild the container:
- Command Palette: `Remote-Containers: Rebuild Container`

## Troubleshooting

### Container Won't Start

1. Check Docker is running: `docker ps`
2. Check Docker logs: `docker logs <container_id>`
3. Rebuild container: `Remote-Containers: Rebuild Container`

### Slow Performance

1. Ensure Docker has sufficient resources (4+ GB RAM, 4+ CPU cores)
2. Check volume mounts are using `:cached` consistency
3. Target directory is mounted as a volume (not bind mount) for better performance

### Build Failures

1. Clean and rebuild:
   ```bash
   cargo clean
   cargo build
   ```

2. Check for dependency issues:
   ```bash
   cargo update
   cargo build
   ```

3. Verify network connectivity for crate downloads

### Port Conflicts

If ports are already in use:
1. Stop conflicting services
2. Or modify port mappings in `devcontainer.json`

### Git Issues

If git complains about safe directories:
```bash
git config --global --add safe.directory /workspace
```

## Advanced Usage

### Adding Custom Tools

Edit `.devcontainer/Dockerfile.dev` to add tools:

```dockerfile
RUN cargo install your-custom-tool
```

Rebuild the container after changes.

### Modifying Services

Edit `.devcontainer/docker-compose.yml` to:
- Add new services
- Change service configurations
- Modify resource limits

### Custom Shell Configuration

Add to `.devcontainer/post-create.sh` or `.devcontainer/post-start.sh`:
- Custom aliases
- Environment setup
- Auto-start commands

## CI/CD Integration

The same Dockerfile can be used for CI/CD pipelines:

```yaml
# .github/workflows/ci.yml
jobs:
  test:
    runs-on: ubuntu-latest
    container:
      image: ghcr.io/your-org/paladin-dev:latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo test --all-features
```

Build and push the dev image:
```bash
docker build -f .devcontainer/Dockerfile.dev -t paladin-dev .
docker tag paladin-dev ghcr.io/your-org/paladin-dev:latest
docker push ghcr.io/your-org/paladin-dev:latest
```

## Resources

- [VS Code DevContainers Documentation](https://code.visualstudio.com/docs/remote/containers)
- [Rust in DevContainers](https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust)
- [Docker Compose Documentation](https://docs.docker.com/compose/)

## Support

For issues or questions:
1. Check this README
2. Review [project documentation](../docs/)
3. Open an issue on GitHub
4. Check existing issues for solutions

## License

This DevContainer configuration is part of the Paladin project and follows the same MIT license.
