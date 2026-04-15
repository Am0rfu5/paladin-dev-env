# DevContainer Rebuild Instructions

## Context: OpenSSL 3.x Upgrade for cargo-public-api

As part of **Epic 2 (Milestone 4)** - Hardening Port Traits as the Stable Public API, we've upgraded the development container from Debian Bullseye to Debian Bookworm to support OpenSSL 3.x.

**Why this upgrade?**
- `cargo-public-api` requires OpenSSL 3.0.0+ for API surface tracking
- Debian Bullseye ships with OpenSSL 1.1.1 (too old)
- Debian Bookworm ships with OpenSSL 3.x (compatible)

## What Changed

### Dockerfiles Updated:
1. `.devcontainer/Dockerfile.dev` - Base image: `rust:1.93-slim-bullseye` → `rust:1.93-slim-bookworm`
2. `Dockerfile` - Production runtime: `libssl1.1` → `libssl3`
3. `Dockerfile.chef` - Build image upgraded to Bookworm

### Impact:
- ✅ Enables `cargo-public-api` installation
- ✅ Modern OpenSSL 3.x with better security
- ✅ Maintains compatibility with existing dependencies
- ⚠️ Requires devcontainer rebuild (one-time)

## How to Rebuild the DevContainer

### Option 1: VS Code Command Palette (Recommended)

1. Press `Ctrl+Shift+P` (Windows/Linux) or `Cmd+Shift+P` (Mac)
2. Type: **"Dev Containers: Rebuild Container"**
3. Select the command
4. Wait for rebuild (5-10 minutes for first build)
5. Verify installation: `cargo public-api --version`

### Option 2: Command Line

```bash
# From your host machine (not inside the container)
cd /path/to/paladin-dev-env

# Stop and remove current container
docker-compose -f .devcontainer/docker-compose.yml down

# Rebuild with no cache to ensure clean build
docker-compose -f .devcontainer/docker-compose.yml build --no-cache paladin-dev

# Start the new container
docker-compose -f .devcontainer/docker-compose.yml up -d

# Reopen in VS Code
code .
```

### Option 3: Just Rebuild the Image

```bash
# From inside the current container (temporary fix for current session)
sudo apt-get update
sudo apt-get install -y wget build-essential

# Download and install OpenSSL 3.0.x manually
cd /tmp
wget https://www.openssl.org/source/openssl-3.0.13.tar.gz
tar -xzf openssl-3.0.13.tar.gz
cd openssl-3.0.13
./config --prefix=/usr/local/ssl --openssldir=/usr/local/ssl shared zlib
make -j$(nproc)
sudo make install

# Update library path
echo "/usr/local/ssl/lib64" | sudo tee -a /etc/ld.so.conf.d/openssl-3.0.13.conf
sudo ldconfig -v

# Set environment variables
export PKG_CONFIG_PATH=/usr/local/ssl/lib64/pkgconfig
export LD_LIBRARY_PATH=/usr/local/ssl/lib64:$LD_LIBRARY_PATH

# Now try installing cargo-public-api
cargo install cargo-public-api

# Verify
cargo public-api --version
```

**Note:** Option 3 is temporary and will be lost when container restarts. Use Option 1 or 2 for permanent solution.

## Verification Steps

After rebuild, verify the installation:

```bash
# Check OpenSSL version (should be 3.x)
openssl version

# Check cargo-public-api is installed
cargo public-api --version

# Test API extraction
cd /workspace
./scripts/extract-public-api.sh project/current-exports.txt

# You should see: "✅ API surface extracted to project/current-exports.txt (XXX items)"
```

## Troubleshooting

### cargo-public-api still fails
```bash
# Check pkg-config can find OpenSSL 3
pkg-config --modversion openssl

# Should output: 3.x.x
# If not, rebuild container
```

### Container won't start
```bash
# Check Docker logs
docker-compose -f .devcontainer/docker-compose.yml logs paladin-dev

# Remove all volumes and rebuild
docker-compose -f .devcontainer/docker-compose.yml down -v
docker-compose -f .devcontainer/docker-compose.yml build --no-cache
docker-compose -f .devcontainer/docker-compose.yml up -d
```

## Next Steps After Rebuild

Once the devcontainer is rebuilt and cargo-public-api is working:

1. ✅ Run `./scripts/extract-public-api.sh` to generate baseline
2. ✅ Continue with Task 2.0 (Install and Configure API Tracking Tools)
3. ✅ Proceed with Epic 2 implementation

## References

- Epic 2 PRD: `project/prd-harden-port-traits-stable-api.md`
- Task List: `project/tasks-harden-port-traits-stable-api.md`
- API Audit: `project/api-audit.md`
- cargo-public-api: https://github.com/Enselic/cargo-public-api
