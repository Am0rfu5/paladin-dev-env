# Configuration

Paladin is configured via a YAML file (`config.yml` by default) and environment
variables. Environment variables take precedence over file values and use the
`APP_` prefix format shown throughout this guide.

## Loading Configuration

```rust,ignore
// Load from the default config.yml in the current directory
let settings = paladin_ai_core::config::ApplicationSettings::load()?;

// Or specify a path
let settings = paladin_ai_core::config::ApplicationSettings::from_file("config.yml")?;
```

## LLM Provider

```yaml
llm:
  default_provider: "openai"   # openai | deepseek | anthropic

  openai:
    base_url: "https://api.openai.com/v1"
    default_model: "gpt-4"
    default_temperature: 0.7
    timeout_seconds: 300
    max_retries: 3

  deepseek:
    base_url: "https://api.deepseek.com/v1"
    default_model: "deepseek-chat"
    default_temperature: 0.7
    timeout_seconds: 300
    max_retries: 3

  anthropic:
    base_url: "https://api.anthropic.com/v1"
    default_model: "claude-3-5-sonnet-20241022"
    default_temperature: 0.7
    timeout_seconds: 300
    max_retries: 3
```

**API keys** are read exclusively from environment variables:

| Variable | Provider |
|----------|----------|
| `OPENAI_API_KEY` | OpenAI |
| `DEEPSEEK_API_KEY` | DeepSeek |
| `ANTHROPIC_API_KEY` | Anthropic |
| `APP_LLM_DEFAULT_PROVIDER` | Override default provider at runtime |

> **Security:** Never put API keys in `config.yml`. Use environment variables or
> a secrets manager (AWS Secrets Manager, HashiCorp Vault, Kubernetes Secrets).

## Garrison (Short-term Memory)

The Garrison stores conversation context between Paladin turns.

```yaml
garrison:
  garrison_type: "in_memory"       # in_memory | sqlite
  # path: "./garrison.db"          # Required when garrison_type = "sqlite"
  max_entries: 100                  # Max conversation turns to retain
  max_tokens: 4000                  # Context-window token budget
  tokenizer: "gpt-4"               # Model name for token counting
  eviction_strategy: "importance_based"  # importance_based | fifo | sliding_window
  preserve_recent_count: 10        # Always keep at least N recent entries
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `garrison_type` | string | `in_memory` | Storage backend |
| `path` | string | - | SQLite file path (sqlite only) |
| `max_entries` | int | 100 | Maximum entries before eviction |
| `max_tokens` | int | 4000 | Token budget for context window |
| `eviction_strategy` | string | `importance_based` | Eviction algorithm |
| `preserve_recent_count` | int | 10 | Minimum recent entries to keep |

**Env vars:** `APP_GARRISON_TYPE`, `APP_GARRISON_PATH`, `APP_GARRISON_MAX_ENTRIES`,
`APP_GARRISON_MAX_TOKENS`, `APP_GARRISON_EVICTION_STRATEGY`, `APP_GARRISON_PRESERVE_RECENT_COUNT`

## Sanctum (Long-term Vector Memory)

Sanctum stores semantic memories in a vector database for RAG.

```yaml
sanctum:
  enabled: false
  adapter_type: "in_memory"        # in_memory | qdrant

  qdrant:                          # Required when adapter_type = "qdrant"
    url: "http://localhost:6334"
    collection_name: "paladin_memories"
    vector_dimension: 1536         # Must match your embedding model

rag:
  top_k: 5                         # Results to retrieve
  min_similarity: 0.7              # Score threshold (0.0-1.0)
  max_tokens: 2000                 # Max tokens to inject from RAG
  timeout_seconds: 5

memory_extraction:
  enabled: true
  strategy: "on_completion"        # every_turn | on_completion | manual
```

**Env vars:** `APP_SANCTUM_ENABLED`, `APP_SANCTUM_ADAPTER_TYPE`,
`APP_SANCTUM_QDRANT_URL`, `APP_SANCTUM_QDRANT_COLLECTION_NAME`,
`APP_SANCTUM_QDRANT_VECTOR_DIMENSION`

See [Sanctum Vector Memory](../user-guides/sanctum-vector-memory.md) for detail.

## Arsenal (Tool System / MCP)

The Arsenal connects Paladins to external tools via the Model Context Protocol.

```yaml
arsenal:
  default_timeout_seconds: 30
  max_concurrent_tools: 5
  mcp_servers:
    # STDIO server (command-line process)
    - name: "web_search"
      server_type: "stdio"
      command: "uvx"
      args: ["mcp-web-search"]

    # SSE server (HTTP-based)
    - name: "code_analyzer"
      server_type: "sse"
      endpoint: "http://localhost:8080/mcp"
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `default_timeout_seconds` | int | 30 | Per-tool execution timeout |
| `max_concurrent_tools` | int | 5 | Parallel tool invocations |
| `mcp_servers[].name` | string | - | Unique server identifier |
| `mcp_servers[].server_type` | string | - | `stdio` or `sse` |
| `mcp_servers[].command` | string | - | Executable (stdio only) |
| `mcp_servers[].endpoint` | string | - | URL (sse only) |

**Env vars:** `APP_ARSENAL_DEFAULT_TIMEOUT_SECONDS`, `APP_ARSENAL_MAX_CONCURRENT_TOOLS`

See [Arsenal & Tools](../user-guides/arsenal-tools.md) for full integration guide.

## Citadel (State Persistence)

Citadel saves Paladin state to disk for crash recovery and resumption.

```yaml
citadel:
  enabled: false
  state_dir: "./paladin-states"
  autosave_enabled: false          # Save state after each execution
  cleanup_enabled: false           # Delete old state files automatically
  max_state_age_days: 30
```

**Env vars:** `APP_CITADEL_ENABLED`, `APP_CITADEL_STATE_DIR`,
`APP_CITADEL_AUTOSAVE_ENABLED`, `APP_CITADEL_CLEANUP_ENABLED`,
`APP_CITADEL_MAX_STATE_AGE_DAYS`

## Battalion (Multi-agent Orchestration)

```yaml
battalion:
  default_timeout_seconds: 300     # Per-battalion execution timeout
  error_strategy: "fail_fast"      # fail_fast | continue_on_error | retry_then_continue
  max_concurrent_paladins: 10      # Phalanx concurrency limit
  metadata_output_enabled: false   # Write execution metadata to files

  retry:                           # Used when error_strategy = retry_then_continue
    max_attempts: 3
    exponential_backoff: true
    jitter: true
    base_delay_ms: 100
    max_delay_seconds: 10

  maneuver:                        # Flow DSL (Maneuver pattern)
    error_strategy: "fail_fast"    # fail_fast | continue_parallel | ignore_errors
    output_format: "combined_text" # combined_text | structured_json
    pass_output_as_input: true
    timeout_seconds: 300
    collect_timing_metrics: true
    max_agents: 30
    max_depth: 5
```

**Env vars:** `APP_BATTALION_DEFAULT_TIMEOUT_SECONDS`, `APP_BATTALION_ERROR_STRATEGY`,
`APP_BATTALION_MAX_CONCURRENT_PALADINS`, `APP_BATTALION_RETRY_MAX_ATTEMPTS`, etc.

See [Battalion Patterns](../user-guides/battalion-patterns.md) for Formation,
Phalanx, Campaign, and Chain of Command details.

## Herald (Output Formatting)

```yaml
herald:
  default_formatter: "json"        # json | markdown | table

  json:
    pretty: true
    include_metadata: true

  markdown:
    include_colors: true
    heading_level: 2

  table:
    max_column_width: 60
    border_style: "rounded"        # ascii | rounded | modern | sharp | none
```

**Env vars:** `APP_HERALD_DEFAULT_FORMATTER`, `APP_HERALD_JSON_PRETTY`,
`APP_HERALD_MARKDOWN_INCLUDE_COLORS`, `APP_HERALD_TABLE_BORDER_STYLE`

## Autonomous Features

All autonomous features are opt-in (disabled by default). Uncomment sections in
`config.yml` to enable:

```yaml
autonomous:
  planning:
    enabled: false          # Decompose complex tasks into subtasks
    max_subtasks: 10

  prompt_generation:
    enabled: false          # Auto-generate system prompts from description
    description: null       # e.g. "Expert data analyst"

  dynamic_temperature:
    enabled: false          # Adjust temperature per task type
    min: 0.1
    max: 0.9

  handoffs:
    enabled: false          # Delegate to specialist Paladins
    strategy: "automatic"   # automatic | explicit | {threshold: 0.8}
    max_depth: 5
```

**Env vars:** `APP_AUTONOMOUS_PLANNING_ENABLED`, `APP_AUTONOMOUS_PLANNING_MAX_SUBTASKS`,
`APP_AUTONOMOUS_PROMPT_GENERATION_ENABLED`, `APP_AUTONOMOUS_DYNAMIC_TEMPERATURE_ENABLED`,
`APP_AUTONOMOUS_HANDOFFS_ENABLED`, `APP_AUTONOMOUS_HANDOFFS_STRATEGY`

## Multi-Environment Pattern

Keep a `config.yml` for defaults and override per environment:

```bash
# Development
export APP_LLM_DEFAULT_PROVIDER=openai
export APP_GARRISON_TYPE=in_memory

# Staging
export APP_GARRISON_TYPE=sqlite
export APP_GARRISON_PATH=/data/garrison.db
export APP_SANCTUM_ENABLED=true

# Production
export APP_GARRISON_TYPE=sqlite
export APP_SANCTUM_ENABLED=true
export APP_SANCTUM_ADAPTER_TYPE=qdrant
export APP_CITADEL_ENABLED=true
export APP_CITADEL_AUTOSAVE_ENABLED=true
```

## Complete Example (`config.yml`)

```yaml
llm:
  default_provider: "openai"
  openai:
    default_model: "gpt-4"
    default_temperature: 0.7

garrison:
  garrison_type: "sqlite"
  path: "./garrison.db"
  max_entries: 200
  max_tokens: 8000

arsenal:
  default_timeout_seconds: 30
  max_concurrent_tools: 5
  mcp_servers:
    - name: "web_search"
      server_type: "stdio"
      command: "uvx"
      args: ["mcp-web-search"]

battalion:
  error_strategy: "retry_then_continue"
  max_concurrent_paladins: 10
  retry:
    max_attempts: 3
    exponential_backoff: true

herald:
  default_formatter: "markdown"
```

## See Also

- [Installation](installation.md)
- [Quickstart](quickstart.md)
- [Garrison Memory](../user-guides/garrison-memory.md)
- [Arsenal & Tools](../user-guides/arsenal-tools.md)
- [Sanctum Vector Memory](../user-guides/sanctum-vector-memory.md)
- [Battalion Patterns](../user-guides/battalion-patterns.md)
