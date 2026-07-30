# Redis Queue Integration Test Fix

## Problem Summary

The 16 Redis queue integration tests were failing with:
```
SocketNotFoundError(/var/run/docker.sock)
```

## Root Cause

1. The tests use `testcontainers` library to spin up isolated Redis instances
2. `testcontainers` requires Docker socket access (`/var/run/docker.sock`)  
3. The DevContainer is configured with standalone Dockerfile, not docker-compose
4. No Docker socket mounted → testcontainers cannot start containers
5. No `REDIS_URL` environment variable → tests cannot use existing Redis service

## Solution Implemented ✅

Modified [`tests/integration/redis_queue_integration_test.rs`](tests/integration/redis_queue_integration_test.rs):

1. **Environment-first approach**: Check for `REDIS_URL` environment variable
   - If set: Parse `redis://host:port` and use existing Redis service
   - If not set: Fall back to testcontainers (requires Docker socket)

2. **Clippy fixes**:
   - Boxed large enum variant `RedisSource::Testcontainer`
   - Fixed rust-analyzer setting in devcontainer.json

## Code Changes

```rust
enum RedisSource {
    Existing {
        host: String,
        port: u16,
    },
    Testcontainer {
        #[allow(dead_code)]
        container: Box<testcontainers::ContainerAsync<GenericImage>>,
        port: u16,
    },
}

// In TestContext::new():
let source = if let Ok(redis_url) = std::env::var("REDIS_URL") {
    // Parse and use existing Redis service
    RedisSource::Existing { host, port }
} else {
    // Fall back to testcontainers (requires Docker socket)
    RedisSource::Testcontainer { container: Box::new(container), port }
};
```

## Current Status

- ✅ Code changes committed: [`c4a1df7`](https://github.com/DF3NDR/paladin-dev-env/commit/c4a1df7)
- ✅ Tests compile cleanly
- ✅ No clippy warnings
- ✅ **DevContainer configured to use docker-compose.yml**
- ✅ **REDIS_URL automatically set via docker-compose environment**
- 🔧 **Rebuild DevContainer to apply changes**

## How to Apply the Fix

**Simply rebuild the DevContainer:**

1. Press `F1` (or `Ctrl+Shift+P` / `Cmd+Shift+P`)
2. Type: "Dev Containers: Rebuild Container"
3. Select the command and wait for rebuild
4. After rebuild, Redis will be available at `redis:6379`
5. `REDIS_URL=redis://redis:6379` will be set automatically

**Verify Redis connectivity:**
```bash
# Check environment variable
echo $REDIS_URL

# Test Redis connection
nc -zv redis 6379

# Or use redis-cli if installed
redis-cli -h redis ping
```

**Run the tests:**
```bash
# Test just Redis queue tests
cargo test --test integration redis_queue

# Test all integration tests  
cargo test --tests

# Run with verbose output
cargo test redis_queue -- --nocapture
```

## What Changed in DevContainer Configuration

The DevContainer now uses `docker-compose.yml` instead of standalone `Dockerfile.dev`:

**Before:**
```json
{
  "dockerFile": "Dockerfile.dev",
  "runArgs": [...],
  "containerEnv": {...}
}
```

**After:**
```json
{
  "dockerComposeFile": "docker-compose.yml",
  "service": "paladin-dev",
  "workspaceFolder": "/workspace"
}
```

This gives the DevContainer access to:
- ✅ Redis service (`redis:6379`)
- ✅ MinIO object storage (`minio:9000`)
- ✅ MySQL database (`mysql:3306`)
- ✅ Shared network for all services
- ✅ Environment variables from docker-compose

## Alternative: Use Make Targets (No Rebuild Needed)

If you prefer not to rebuild the DevContainer right now, you can still run tests using:

```bash
# Runs integration tests with docker-compose Redis
make test-integration-docker
```

## Related Files

- [`tests/integration/redis_queue_integration_test.rs`](tests/integration/redis_queue_integration_test.rs)
- [`.devcontainer/devcontainer.json`](.devcontainer/devcontainer.json)
- [`.devcontainer/docker-compose.yml`](.devcontainer/docker-compose.yml)
- [`Makefile`](Makefile) - see `test-integration-docker` target

## Commits

All changes on branch `bugs/epic-24-post-fixes`:

- `c4a1df7` - fix: suppress dead_code warning for Redis container field
- `22951e8` - fix: redis queue tests to support external Redis service
- `fdf016e` - fix: remove duplicate cfg attribute in llm_live_api_tests  
- `3ddcfc8` - fix: modernize integration test APIs for LLM providers

**DevContainer configuration** (pending commit):
- Modified `.devcontainer/devcontainer.json` to use docker-compose.yml
- Enables automatic Redis, MinIO, MySQL service connectivity
