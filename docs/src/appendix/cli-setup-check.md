# Paladin Setup Check

Comprehensive environment validation to ensure your Paladin installation is correctly configured.

## Overview

The `paladin setup-check` command validates your entire Paladin environment:
- System requirements (CLI version, Rust toolchain)
- Environment configuration (.env file, API keys)
- LLM provider connectivity (OpenAI, Anthropic, DeepSeek)
- Optional services (Redis, Qdrant, MinIO)

## Quick Start

```bash
# Basic validation
paladin setup-check

# Detailed output with timing
paladin setup-check --verbose

# Minimal output (CI-friendly)
paladin setup-check --quiet
```

## Command Options

```bash
paladin setup-check [OPTIONS]
```

**Options:**
- `-v, --verbose` - Show detailed version strings, response times, and diagnostic info
- `-q, --quiet` - Minimal output, only show failures (exit code indicates status)
- `--json` - Output results in JSON format (for scripting)

## Check Categories

### 1. System Checks

Validates core system requirements:

```
System:
  ✓ Paladin CLI: v0.1.0
  ✓ Rust Toolchain: 1.75.0 (stable)
```

**What's checked:**
- Paladin CLI version (from `Cargo.toml`)
- Rust compiler version (`rustc --version`)
- Binary build date and features

**Verbose output:**
```
System:
  ✓ Paladin CLI: v0.1.0
    Build: 2026-02-09 10:30:00 UTC
    Features: redis-queue, s3-storage, qdrant-vector
  ✓ Rust Toolchain: rustc 1.75.0 (82e1608df 2023-12-21)
    Host: x86_64-unknown-linux-gnu
```

### 2. Environment Checks

Validates configuration files and environment variables:

```
Environment:
  ✓ .env file: Found (12 variables loaded)
  ✓ OPENAI_API_KEY: Configured (sk-...xyz)
  ⚠ ANTHROPIC_API_KEY: Not configured
  ⚠ DEEPSEEK_API_KEY: Not configured
```

**What's checked:**
- `.env` file existence and parsability
- Required environment variables
- API key format validation (prefix, length)
- Configuration completeness

**Status Indicators:**
- ✓ **Pass**: Configured and valid format
- ⚠ **Warn**: Not configured (optional)
- ✗ **Fail**: Configured but invalid format

### 3. Provider Checks

Tests connectivity to configured LLM providers:

```
Providers:
  ✓ OpenAI: Connected [342ms]
    Models: gpt-4, gpt-3.5-turbo, gpt-4-32k
  ✗ Anthropic: Authentication failed
    Error: Invalid API key format
  - DeepSeek: Not configured (skipped)
```

**What's checked:**
- **OpenAI** (`GET /v1/models`)
  - Authentication
  - Available models
  - Response time

- **Anthropic** (`POST /v1/messages` minimal request)
  - Authentication
  - API version compatibility
  - Response time

- **DeepSeek** (`GET /models`)
  - Authentication
  - Available models
  - Response time

**Verbose output includes:**
- Full model lists
- API endpoint URLs
- Request/response times
- Quota/rate limit info (if available)

### 4. Service Checks (Optional)

Tests connectivity to optional external services:

```
Services (Optional):
  ✓ Redis: Connected [15ms]
    Version: 7.0.11
    Memory: 1.2MB / 512MB used
  ✓ Qdrant: Connected [28ms]
    Version: 1.7.4
    Collections: 2 (paladin_memory, documents)
  - MinIO: Not configured (skipped)
```

**What's checked:**

**Redis** (if `REDIS_URL` configured):
- Connection test
- PING command
- Server version
- Memory usage stats

**Qdrant** (if `QDRANT_URL` configured):
- Connection test
- Version check
- Collection list
- Health status

**MinIO** (if `MINIO_ENDPOINT` configured):
- Connection test
- Bucket list
- Credentials validation

**Status Indicators:**
- ✓ **Pass**: Connected and operational
- ⚠ **Warn**: Connected but issues detected
- ✗ **Fail**: Cannot connect or authentication failed
- - **Skip**: Not configured (not an error)

## Exit Codes

The command returns different exit codes based on results:

| Exit Code | Meaning | Description |
|-----------|---------|-------------|
| `0` | Success | All checks passed |
| `1` | Critical Failure | One or more critical checks failed |
| `2` | Warnings | All critical checks passed, but warnings present |

**Usage in scripts:**
```bash
#!/bin/bash

paladin setup-check --quiet
status=$?

case $status in
  0)
    echo "✓ Environment ready"
    ./run-deployment.sh
    ;;
  1)
    echo "✗ Critical failures detected"
    exit 1
    ;;
  2)
    echo "⚠ Warnings present, proceeding anyway"
    ./run-deployment.sh
    ;;
esac
```

## Output Formats

### Standard Format (Human-Readable)

Default terminal-friendly output with colors and Unicode symbols:

```
=== Paladin Setup Check ===

System:
  ✓ Paladin CLI: v0.1.0
  ✓ Rust Toolchain: 1.75.0

Environment:
  ✓ .env file: Found
  ✓ OPENAI_API_KEY: Configured

Providers:
  ✓ OpenAI: Connected [342ms]

Services (Optional):
  ✓ Redis: Connected [15ms]
  - Qdrant: Not configured

=== Summary ===
✓ 5 passed
⚠ 1 warning
✗ 0 failed

All critical checks passed!
```

### Verbose Format

Includes additional diagnostic information:

```bash
paladin setup-check --verbose
```

```
=== Paladin Setup Check (Verbose) ===

System:
  ✓ Paladin CLI
    Version: v0.1.0
    Build Date: 2026-02-09 10:30:00 UTC
    Git Commit: abc123f
    Features: redis-queue, s3-storage, qdrant-vector

  ✓ Rust Toolchain
    Version: rustc 1.75.0 (82e1608df 2023-12-21)
    Host: x86_64-unknown-linux-gnu
    LLVM: 17.0.6

Environment:
  ✓ .env file
    Path: /home/user/project/.env
    Size: 438 bytes
    Variables: 12
    Last Modified: 2026-02-09 09:15:23

  ✓ OPENAI_API_KEY
    Format: Valid (sk-...xyz)
    Length: 51 characters
    Status: Configured

Providers:
  ✓ OpenAI
    Endpoint: https://api.openai.com/v1
    Status: Connected
    Response Time: 342ms
    Models: 8 available
      - gpt-4 (context: 8192)
      - gpt-3.5-turbo (context: 4096)
      - gpt-4-32k (context: 32768)
    Organization: org-...

[... continues ...]
```

### JSON Format

Machine-readable output for scripting:

```bash
paladin setup-check --json
```

```json
{
  "version": "0.1.0",
  "timestamp": "2026-02-09T10:30:00Z",
  "checks": {
    "system": [
      {
        "name": "Paladin CLI",
        "status": "pass",
        "value": "v0.1.0",
        "details": {
          "build_date": "2026-02-09T10:30:00Z",
          "git_commit": "abc123f"
        }
      },
      {
        "name": "Rust Toolchain",
        "status": "pass",
        "value": "1.75.0"
      }
    ],
    "environment": [
      {
        "name": ".env file",
        "status": "pass",
        "value": "Found"
      },
      {
        "name": "OPENAI_API_KEY",
        "status": "pass",
        "value": "Configured"
      }
    ],
    "providers": [
      {
        "name": "OpenAI",
        "status": "pass",
        "response_time_ms": 342,
        "models": ["gpt-4", "gpt-3.5-turbo"]
      }
    ],
    "services": [
      {
        "name": "Redis",
        "status": "pass",
        "optional": true,
        "response_time_ms": 15,
        "version": "7.0.11"
      }
    ]
  },
  "summary": {
    "total": 10,
    "passed": 9,
    "warned": 1,
    "failed": 0,
    "skipped": 3
  },
  "exit_code": 0
}
```

## Troubleshooting

### System Checks Fail

**Problem:** CLI version check fails

```
System:
  ✗ Paladin CLI: Version not found
```

**Solutions:**
1. **Verify installation:**
   ```bash
   which paladin
   paladin --version
   ```

2. **Rebuild if needed:**
   ```bash
   cargo build --release --bin paladin-cli
   ```

3. **Check PATH:**
   ```bash
   echo $PATH
   export PATH="$PATH:/path/to/paladin/target/release"
   ```

### Provider Checks Fail

**Problem:** OpenAI authentication fails

```
Providers:
  ✗ OpenAI: Authentication failed (401)
    Error: Incorrect API key provided
```

**Solutions:**
1. **Verify API key:**
   ```bash
   echo $OPENAI_API_KEY
   # Should start with sk- and be 51+ characters
   ```

2. **Test directly:**
   ```bash
   curl https://api.openai.com/v1/models \
     -H "Authorization: Bearer $OPENAI_API_KEY"
   ```

3. **Re-run onboarding:**
   ```bash
   paladin onboarding
   ```

**Problem:** Connection timeout

```
Providers:
  ✗ Anthropic: Connection timeout (5000ms)
```

**Solutions:**
1. **Check network connectivity:**
   ```bash
   ping api.anthropic.com
   curl -I https://api.anthropic.com
   ```

2. **Check proxy settings:**
   ```bash
   env | grep -i proxy
   ```

3. **Increase timeout:**
   ```bash
   PALADIN_REQUEST_TIMEOUT=10000 paladin setup-check
   ```

### Service Checks Fail

**Problem:** Redis connection fails

```
Services (Optional):
  ✗ Redis: Connection refused
    Error: ECONNREFUSED 127.0.0.1:6379
```

**Solutions:**
1. **Start Redis:**
   ```bash
   # Docker
   docker run -d -p 6379:6379 redis:7-alpine

   # System service
   sudo systemctl start redis
   ```

2. **Check configuration:**
   ```bash
   echo $REDIS_URL
   # Should be: redis://localhost:6379
   ```

3. **Test connection:**
   ```bash
   redis-cli ping
   # Should return: PONG
   ```

## Continuous Integration

Use in CI/CD pipelines:

```yaml
# GitHub Actions
- name: Validate Paladin Environment
  run: |
    paladin setup-check --quiet --json > setup-check.json
    cat setup-check.json
  env:
    OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
```

```groovy
// Jenkins
stage('Validate Environment') {
  steps {
    sh '''
      paladin setup-check --quiet
      if [ $? -ne 0 ]; then
        echo "Environment validation failed"
        exit 1
      fi
    '''
  }
}
```

## Related Commands

- **[`paladin onboarding`](cli-onboarding.md)** - Set up environment from scratch
- **[`paladin features`](cli-usage.md)** - Check available features
- **[`paladin agent run`](cli-usage.md)** - Run agents after validation

## See Also

- [CLI Usage Guide](cli-usage.md)
- [Installation Guide](../getting-started/installation.md)
- [Troubleshooting Guide](../operations/troubleshooting.md)
