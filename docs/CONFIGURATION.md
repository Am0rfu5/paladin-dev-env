# Paladin Configuration Guide

This guide explains how Paladin's configuration system works, best practices for different environments, and the clear separation of concerns between YAML files and environment variables.

## Table of Contents

- [Configuration Philosophy](#configuration-philosophy)
- [Quick Start](#quick-start)
- [Configuration Sources](#configuration-sources)
- [Environment Variables Reference](#environment-variables-reference)
- [Environment-Specific Setup](#environment-specific-setup)
- [Security Best Practices](#security-best-practices)
- [Advanced Topics](#advanced-topics)

## Configuration Philosophy

Paladin uses a **dual-path configuration system** with clear separation of concerns:

| What | Where | Purpose | Example |
|------|-------|---------|---------|
| **Behavioral Config** | YAML files | Define *how* the system behaves | Timeouts, model names, strategies |
| **Secrets** | Environment variables | Credentials and sensitive data | API keys, passwords |
| **Overrides** | `APP_*` env vars | Deployment-time tuning | `APP_GARRISON_MAX_ENTRIES=500` |

### Why Both?

- **YAML files** are version-controlled, reviewed in PRs, and define the system's structure
- **Environment variables** are injected at deployment time and never committed to source control
- **This separation** enables security (secrets stay out of repos), flexibility (same code works in dev/staging/prod), and auditability (config changes are tracked in git)

## Quick Start

### Development (DevContainer)

1. **Copy the example environment file:**
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env` and add your API keys:**
   ```bash
   # LLM Provider API Keys (choose one or more)
   OPENAI_API_KEY=sk-your-key-here
   DEEPSEEK_API_KEY=your-deepseek-key
   ANTHROPIC_API_KEY=your-anthropic-key
   ```

3. **Load the environment file** (automatic in DevContainer):
   ```bash
   # Manual loading if needed:
   set -a
   . /workspace/.env
   set +a
   ```

4. **Run Paladin:**
   ```bash
   cargo run
   ```

The `.env` file is automatically loaded by the application in debug builds.

### CI/CD

Set secrets as environment variables in your CI system:

**GitHub Actions:**
```yaml
env:
  OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
  DEEPSEEK_API_KEY: ${{ secrets.DEEPSEEK_API_KEY }}
```

**GitLab CI:**
```yaml
variables:
  CONFIG_FILE: "config.test.yml"
script:
  - cargo test --features live-api-tests
```

### Production

Use a secrets manager:

**AWS Secrets Manager + ECS:**
```json
"secrets": [
  {
    "name": "OPENAI_API_KEY",
    "valueFrom": "arn:aws:secretsmanager:region:account:secret:paladin/openai"
  }
]
```

**Kubernetes Secrets:**
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: paladin-secrets
type: Opaque
data:
  OPENAI_API_KEY: <base64-encoded-key>
---
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: paladin
        envFrom:
        - secretRef:
            name: paladin-secrets
```

## Configuration Sources

Paladin loads configuration in this **priority order** (later sources override earlier ones):

1. **`config.yml`** (or specified via `--config` flag) - Base configuration
2. **Environment-specific file** - `config.{APP_ENV}.yml` if `APP_ENV` is set
3. **`APP_*` environment variables** - Override any YAML value
4. **Direct environment variables** - LLM API keys bypass the config system

### Example: Loading Sequence

Given this setup:

**config.yml:**
```yaml
garrison:
  garrison_type: "in_memory"
  max_entries: 100
```

**Environment:**
```bash
APP_GARRISON_MAX_ENTRIES=500
OPENAI_API_KEY=sk-real-key
```

**Result:**
- Garrison type: `in_memory` (from config.yml)
- Max entries: `500` (overridden by `APP_*` env var)
- OpenAI key: `sk-real-key` (from direct env var, never in YAML)

## Environment Variables Reference

### LLM Provider API Keys (Direct Read)

These are **NOT** in `config.yml` — adapters read them directly from the environment:

| Variable | Provider | Required When |
|----------|----------|---------------|
| `OPENAI_API_KEY` | OpenAI GPT models | Using default_provider: "openai" |
| `DEEPSEEK_API_KEY` | DeepSeek models | Using default_provider: "deepseek" |
| `ANTHROPIC_API_KEY` | Anthropic Claude | Using default_provider: "anthropic" |

### APP_* Overrides (Settings System)

Override any YAML value using the `APP_` prefix + uppercase path with underscores:

**YAML path** → **Environment variable**

```yaml
garrison:
  max_entries: 100
```
→ `APP_GARRISON_MAX_ENTRIES=500`

```yaml
llm:
  openai:
    default_model: "gpt-4"
```
→ `APP_LLM_OPENAI_DEFAULT_MODEL="gpt-4-turbo"`

### Common Overrides

#### Garrison (Memory System)
```bash
APP_GARRISON_TYPE=sqlite
APP_GARRISON_PATH=./custom_garrison.db
APP_GARRISON_MAX_ENTRIES=200
APP_GARRISON_MAX_TOKENS=8000
APP_GARRISON_TOKENIZER=gpt-4
APP_GARRISON_EVICTION_STRATEGY=fifo
APP_GARRISON_PRESERVE_RECENT_COUNT=20
```

#### Sanctum (Long-term Memory)
```bash
APP_SANCTUM_ENABLED=true
APP_SANCTUM_ADAPTER_TYPE=qdrant
APP_SANCTUM_QDRANT_URL=http://qdrant-server:6334
APP_SANCTUM_QDRANT_COLLECTION_NAME=custom_memories
APP_SANCTUM_QDRANT_VECTOR_DIMENSION=3072
```

#### Arsenal (Tool System)
```bash
APP_ARSENAL_DEFAULT_TIMEOUT_SECONDS=60
APP_ARSENAL_MAX_CONCURRENT_TOOLS=10
```

#### Citadel (State Persistence)
```bash
APP_CITADEL_ENABLED=true
APP_CITADEL_STATE_DIR=./custom-states
APP_CITADEL_AUTOSAVE_ENABLED=true
APP_CITADEL_CLEANUP_ENABLED=true
APP_CITADEL_MAX_STATE_AGE_DAYS=60
```

#### Redis Queue
```bash
APP_REDIS_HOST=redis-prod.example.com
APP_REDIS_PORT=6379
APP_REDIS_PASSWORD=secure-password
APP_REDIS_DB=2
APP_REDIS_POOL_SIZE=20
```

#### MinIO File Storage
```bash
APP_MINIO_ENDPOINT=https://s3.amazonaws.com
APP_MINIO_BUCKET=paladin-prod
APP_MINIO_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE
APP_MINIO_SECRET_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
APP_MINIO_REGION=us-west-2
```

## Environment-Specific Setup

### Development (DevContainer)

**Config file:** `config.yml`

**Secrets source:** `.env` file (auto-loaded in debug builds)

**Setup:**
```bash
# 1. Copy template
cp .env.example .env

# 2. Edit .env with your keys
vim .env

# 3. The DevContainer post-start.sh loads it automatically
# Or manually in new terminals:
set -a && . /workspace/.env && set +a

# 4. Run
cargo run
```

**Benefits:**
- ✅ Fast iteration with hot-reload
- ✅ No need to export vars in every terminal
- ✅ `.env` is gitignored, so secrets stay local

### CI/CD (GitHub Actions, GitLab, etc.)

**Config file:** `config.test.yml`

**Secrets source:** CI secrets store

**Setup (GitHub Actions example):**
```yaml
name: Test
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    env:
      # Use shorter timeouts and smaller limits for tests
      CONFIG_FILE: config.test.yml
    steps:
      - uses: actions/checkout@v4

      - name: Run tests with mocks
        run: cargo test

      - name: Run live API tests
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
        run: cargo test --features live-api-tests -- --ignored
```

**Benefits:**
- ✅ Different config for test environment (faster timeouts, smaller limits)
- ✅ Secrets managed by CI platform (encrypted, audited, rotated)
- ✅ Mock tests run without API keys, live tests only with secrets present

### Staging

**Config file:** `config.staging.yml` (set `APP_ENV=staging`)

**Secrets source:** Vault, AWS Secrets Manager, or K8s Secrets

**Setup (Kubernetes example):**
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: paladin-config
data:
  config.staging.yml: |
    llm:
      default_provider: "deepseek"  # Use cheaper model in staging
    garrison:
      garrison_type: "sqlite"
      max_entries: 200
---
apiVersion: v1
kind: Secret
metadata:
  name: paladin-secrets
type: Opaque
stringData:
  OPENAI_API_KEY: "sk-staging-key"
  DEEPSEEK_API_KEY: "staging-key"
---
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: paladin
        env:
        - name: APP_ENV
          value: "staging"
        envFrom:
        - secretRef:
            name: paladin-secrets
        volumeMounts:
        - name: config
          mountPath: /etc/paladin/config.staging.yml
          subPath: config.staging.yml
      volumes:
      - name: config
        configMap:
          name: paladin-config
```

### Production

**Config file:** `config.production.yml` (set `APP_ENV=production`)

**Secrets source:** Enterprise secrets manager (Vault, AWS SM, Azure Key Vault)

**Setup (AWS ECS + Secrets Manager):**

1. **Store secrets:**
   ```bash
   aws secretsmanager create-secret \
     --name paladin/prod/openai \
     --secret-string "sk-prod-key-..."
   ```

2. **Task definition:**
   ```json
   {
     "family": "paladin-prod",
     "containerDefinitions": [{
       "name": "paladin",
       "image": "paladin:1.0.0",
       "command": ["--config", "/etc/paladin/config.production.yml"],
       "environment": [
         {"name": "APP_ENV", "value": "production"}
       ],
       "secrets": [
         {
           "name": "OPENAI_API_KEY",
           "valueFrom": "arn:aws:secretsmanager:region:account:secret:paladin/prod/openai"
         }
       ]
     }]
   }
   ```

**Benefits:**
- ✅ Secrets never touch disk or config files
- ✅ Automatic rotation with Secrets Manager
- ✅ Audit trail of all secret access
- ✅ Fine-grained IAM permissions

## Security Best Practices

### ✅ DO

1. **Keep secrets in environment variables only**
   ```bash
   export OPENAI_API_KEY="sk-..."
   ```

2. **Use `.env` files for local development**
   ```bash
   # .env (gitignored)
   OPENAI_API_KEY=sk-dev-key
   ```

3. **Use secrets managers in production**
   - AWS Secrets Manager
   - HashiCorp Vault
   - Kubernetes Secrets (with encryption at rest)
   - Azure Key Vault
   - GCP Secret Manager

4. **Set restrictive file permissions on .env**
   ```bash
   chmod 600 .env
   ```

5. **Rotate API keys regularly**

6. **Use different keys per environment**
   - Dev key: Limited quota, separate account
   - Staging key: Separate from prod
   - Prod key: High quota, monitored

### ❌ DON'T

1. **Never commit secrets to git**
   ```yaml
   # ❌ BAD - Don't do this!
   api_key: "sk-real-key-here"
   ```

2. **Never use production keys in development**

3. **Never share .env files via Slack/email**

4. **Never log API keys**
   ```rust
   // ❌ BAD
   println!("API key: {}", api_key);
   ```

5. **Never put secrets in Docker images**
   ```dockerfile
   # ❌ BAD
   ENV OPENAI_API_KEY=sk-...
   ```

## Advanced Topics

### Custom Configuration Files

Specify a different config file:

```bash
cargo run -- --config my-custom-config.yml
```

### Environment-Specific Configs

Set `APP_ENV` to automatically load environment-specific files:

```bash
export APP_ENV=staging
cargo run
# Loads config.yml first, then overrides with config.staging.yml
```

### Configuration Validation

The application validates configuration on startup:

```rust
let settings = Settings::new()?; // Returns error if invalid
```

Common validation errors:
- Missing required fields
- Invalid enum values
- Out-of-range numbers
- Unreachable URLs (for live validation)

### Programmatic Configuration

For tests or embedded usage:

```rust
use paladin::config::application_settings::Settings;

// Load from specific file
let settings = Settings::load_from_file("config.test.yml")?;

// Access config values
let garrison_config = settings.get_garrison_config()?;
assert_eq!(garrison_config.max_entries, 100);
```

### MCP Server Configuration

MCP servers are defined in YAML but may reference env vars:

```yaml
arsenal:
  mcp_servers:
    - name: "github"
      type: "stdio"
      command: "npx"
      args: ["-y", "@modelcontextprotocol/server-github"]
      env:
        GITHUB_TOKEN: "${GITHUB_TOKEN}"  # ❌ Won't interpolate!

    - name: "web-search"
      type: "sse"
      url: "http://localhost:8080/mcp"
```

**Note:** The `${VAR}` syntax in YAML is **not interpolated** by the config crate. Set env vars directly:

```bash
export GITHUB_TOKEN="ghp_..."
cargo run
```

### Debugging Configuration

Enable debug logging to see config loading:

```bash
RUST_LOG=debug cargo run
```

Check what config values are loaded:

```rust
use log::info;

let settings = Settings::new()?;
info!("Loaded config: {:?}", settings);
```

### Configuration Schema

For IDE autocomplete and validation, generate a JSON schema:

```bash
# Future feature - not yet implemented
cargo run -- config schema > config-schema.json
```

## Troubleshooting

### "Missing API key" errors

**Symptom:** `Error: Missing OPENAI_API_KEY environment variable`

**Solutions:**
1. Check the variable is set: `echo $OPENAI_API_KEY`
2. Load .env file: `set -a && . .env && set +a`
3. Export manually: `export OPENAI_API_KEY="sk-..."`
4. In DevContainer, restart terminal or source ~/.bashrc

### Config file not found

**Symptom:** `Failed to load configuration: config.yml not found`

**Solutions:**
1. Check current directory: `pwd`
2. Verify file exists: `ls -la config.yml`
3. Specify absolute path: `--config /workspace/config.yml`
4. Use correct filename: `config.yml` not `config.yaml`

### APP_* overrides not working

**Symptom:** Environment variable set but value not changing

**Solutions:**
1. Check variable name matches YAML structure: `garrison.max_entries` → `APP_GARRISON_MAX_ENTRIES`
2. Use uppercase and underscores
3. Verify with: `env | grep APP_`
4. Check the getter method exists in `application_settings.rs`

### Permissions errors on .env

**Symptom:** `.env` file readable by others

**Solution:**
```bash
chmod 600 .env
ls -la .env
# Should show: -rw------- (owner read/write only)
```

## Further Reading

- [Garrison (Memory) Documentation](GARRISON.md)
- [Sanctum (Long-term Memory) Documentation](SANCTUM.md)
- [Arsenal (Tool System) Documentation](ARSENAL.md)
- [CLI Usage Guide](CLI_USAGE.md)
- [Deployment Guide](deployment/README.md)
- [Contributing Guide](../CONTRIBUTING.md)

## Support

For configuration issues:
1. Check this guide first
2. Search [existing issues](https://github.com/DF3NDR/paladin-dev-env/issues)
3. Ask in [Discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)
4. Open a new issue with:
   - Your config.yml (redact secrets!)
   - Environment variables (redact secrets!)
   - Error messages
   - Rust version and OS
