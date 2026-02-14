# Paladin CLI Configuration Guide

Comprehensive guide to configuring Paladin agents through YAML configuration files.

## Table of Contents

- [Overview](#overview)
- [Configuration File Structure](#configuration-file-structure)
- [Garrison Configuration (Memory)](#garrison-configuration-memory)
- [Arsenal Configuration (Tools)](#arsenal-configuration-tools)
- [Scheduler Configuration](#scheduler-configuration)
- [Complete Configuration Examples](#complete-configuration-examples)
- [Environment Variables](#environment-variables)
- [Troubleshooting](#troubleshooting)

## Overview

Paladin agents can be configured entirely through YAML files, enabling:

- **Reproducible deployments**: Version-control your agent configurations
- **Complex orchestration**: Configure multi-agent battalions with memory and tools
- **Environment-specific settings**: Use environment variables for sensitive data
- **Testing and CI/CD**: Run agents with mock providers and predictable configurations

## Configuration File Structure

Basic Paladin YAML configuration:

```yaml
name: "my-agent"
system_prompt: "You are a helpful AI assistant."
llm:
  provider: "openai"
  model: "gpt-4"
  temperature: 0.7
max_loops: 3
user_name: "User"
stop_words:
  - "TERMINATE"
  - "DONE"
```

## Garrison Configuration (Memory)

Garrison provides memory capabilities to Paladins, enabling context retention across interactions.

### In-Memory Garrison

Fast, non-persistent memory suitable for single-session use:

```yaml
garrison:
  type: "in_memory"
  max_entries: 1000
```

**Configuration Options:**
- `type`: Must be `"in_memory"`
- `max_entries`: Maximum number of memory entries (default: 1000)

**Use cases:**
- Development and testing
- Short-lived agent sessions
- When persistence is not required

### SQLite Garrison

Persistent memory backed by SQLite database:

```yaml
garrison:
  type: "sqlite"
  path: "./data/agent_memory.db"
  max_entries: 10000
  ttl_seconds: 86400  # 24 hours
```

**Configuration Options:**
- `type`: Must be `"sqlite"`
- `path`: Database file path (will be created if it doesn't exist)
- `max_entries`: Maximum number of entries before cleanup (default: 10000)
- `ttl_seconds`: Entry time-to-live in seconds (optional, default: no expiration)

**Use cases:**
- Production deployments
- Long-running agents with conversation history
- Multi-session context retention

### Memory Operations

When garrison is configured, Paladins automatically:

1. **Store interactions**: Each LLM call and response is recorded
2. **Retrieve context**: Recent interactions are included in prompts
3. **Semantic search**: Find relevant past interactions (future enhancement)

## Arsenal Configuration (Tools)

Arsenal enables Paladins to access external tools via the Model Context Protocol (MCP).

### MCP STDIO Servers

Connect to command-line MCP servers:

```yaml
arsenal:
  mcp_servers:
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args: 
        - "mcp-web-search"
    
    - name: "filesystem"
      type: "stdio"
      command: "node"
      args:
        - "/path/to/mcp-server-filesystem"
        - "--root"
        - "/workspace"
```

**Configuration Options:**
- `name`: Unique identifier for the tool server
- `type`: Must be `"stdio"`
- `command`: Executable command (e.g., `uvx`, `node`, `python`)
- `args`: Command-line arguments as a list

### MCP SSE Servers

Connect to HTTP-based MCP servers via Server-Sent Events:

```yaml
arsenal:
  mcp_servers:
    - name: "api_tools"
      type: "sse"
      url: "https://api.example.com/mcp"
      auth_token: "${MCP_API_TOKEN}"
```

**Configuration Options:**
- `name`: Unique identifier for the tool server
- `type`: Must be `"sse"`
- `url`: HTTP endpoint for the MCP server
- `auth_token`: Authentication token (use environment variables for secrets)

### Tool Discovery and Registration

When arsenal is configured:

1. **Auto-discovery**: All MCP servers are queried for available tools
2. **Registration**: Tools are registered in the arsenal registry
3. **LLM integration**: Tool schemas are included in LLM system prompts
4. **Invocation**: Paladins can call tools by name with JSON arguments

### Available MCP Servers

Popular MCP servers you can integrate:

- **mcp-web-search**: Web search capabilities (Brave, Google)
- **mcp-server-filesystem**: File system operations
- **mcp-server-git**: Git repository operations
- **mcp-server-brave-search**: Brave search API
- **mcp-server-slack**: Slack workspace integration
- **mcp-server-github**: GitHub API access

See [MCP Server Directory](https://github.com/modelcontextprotocol/servers) for more.

## Scheduler Configuration

Configure scheduled task execution for async operations:

```yaml
scheduler:
  enabled: true
  default_cron: "0 0 * * *"  # Daily at midnight
  channel_size: 100
```

**Configuration Options:**
- `enabled`: Enable/disable scheduler (default: `false`)
- `default_cron`: Default cron expression for scheduled tasks
- `channel_size`: Task queue channel size (default: 100)

**Cron Expression Examples:**
```
"0 * * * *"      # Every hour
"0 0 * * *"      # Daily at midnight
"0 0 * * 1"      # Weekly on Monday
"*/15 * * * *"   # Every 15 minutes
"0 9-17 * * *"   # Hourly between 9 AM and 5 PM
```

**Use cases:**
- Scheduled content delivery
- Periodic agent execution
- Batch processing workflows

## Complete Configuration Examples

### Example 1: Basic Paladin with Memory

```yaml
name: "research-assistant"
system_prompt: |
  You are a research assistant that helps users find and analyze information.
  You have access to web search tools and maintain conversation context.

llm:
  provider: "openai"
  model: "gpt-4"
  temperature: 0.7

max_loops: 5
user_name: "Researcher"

garrison:
  type: "sqlite"
  path: "./data/research_memory.db"
  max_entries: 5000
  ttl_seconds: 604800  # 7 days
```

### Example 2: Paladin with Tools and Memory

```yaml
name: "developer-assistant"
system_prompt: |
  You are a software development assistant with access to code search,
  file system operations, and Git commands. Use tools to help users
  with coding tasks.

llm:
  provider: "openai"
  model: "gpt-4"
  temperature: 0.5

max_loops: 10
user_name: "Developer"

garrison:
  type: "sqlite"
  path: "./data/dev_memory.db"
  max_entries: 10000

arsenal:
  mcp_servers:
    - name: "filesystem"
      type: "stdio"
      command: "node"
      args:
        - "/usr/local/lib/mcp-server-filesystem"
        - "--root"
        - "${WORKSPACE_DIR}"
    
    - name: "git"
      type: "stdio"
      command: "node"
      args:
        - "/usr/local/lib/mcp-server-git"
    
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args:
        - "mcp-web-search"
        - "--brave-api-key"
        - "${BRAVE_API_KEY}"
```

### Example 3: Full-Featured Configuration

```yaml
name: "production-agent"
system_prompt: |
  You are a production AI agent with full capabilities:
  - Persistent memory for conversation context
  - Tool access for external operations
  - Scheduled task execution
  
  Always maintain context across sessions and use tools when appropriate.

llm:
  provider: "openai"
  model: "gpt-4"
  temperature: 0.7

max_loops: 5
user_name: "User"
stop_words:
  - "TERMINATE"
  - "TASK_COMPLETE"

garrison:
  type: "sqlite"
  path: "/var/lib/paladin/memory/agent.db"
  max_entries: 50000
  ttl_seconds: 2592000  # 30 days

arsenal:
  mcp_servers:
    - name: "web_search"
      type: "stdio"
      command: "uvx"
      args:
        - "mcp-web-search"
    
    - name: "slack"
      type: "stdio"
      command: "node"
      args:
        - "/opt/mcp-server-slack"
        - "--workspace"
        - "${SLACK_WORKSPACE_ID}"
        - "--token"
        - "${SLACK_BOT_TOKEN}"
    
    - name: "api_tools"
      type: "sse"
      url: "https://api.company.com/mcp"
      auth_token: "${COMPANY_API_TOKEN}"

scheduler:
  enabled: true
  default_cron: "0 */6 * * *"  # Every 6 hours
  channel_size: 200
```

## Environment Variables

### LLM Provider Keys

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# DeepSeek
export DEEPSEEK_API_KEY="..."

# Anthropic
export ANTHROPIC_API_KEY="..."
```

### Tool Authentication

```bash
# Brave Search
export BRAVE_API_KEY="..."

# Slack
export SLACK_BOT_TOKEN="xoxb-..."
export SLACK_WORKSPACE_ID="T..."

# Custom APIs
export COMPANY_API_TOKEN="..."
```

### File Paths

```bash
# Use environment variables in configuration
export WORKSPACE_DIR="/home/user/workspace"
export GARRISON_DB_PATH="/var/lib/paladin/memory"
```

### Using Environment Variables in YAML

```yaml
garrison:
  path: "${GARRISON_DB_PATH}/agent.db"

arsenal:
  mcp_servers:
    - name: "api"
      type: "sse"
      url: "${API_SERVER_URL}"
      auth_token: "${API_TOKEN}"
```

## Troubleshooting

### Garrison Issues

#### SQLite Database Locked

**Symptom:** `SqliteError: database is locked`

**Solutions:**
- Ensure only one Paladin instance accesses the database
- Check file permissions on the database file
- Use WAL mode for concurrent reads (automatic in SQLite garrison)

#### Memory Not Persisting

**Symptom:** Agent doesn't remember previous interactions

**Solutions:**
- Verify garrison type is `"sqlite"`, not `"in_memory"`
- Check database file path is correct and writable
- Verify `ttl_seconds` hasn't expired old entries
- Check garrison is wired in agent command: verify no TODO at line 293

### Arsenal Issues

#### Tool Not Found

**Symptom:** `ArsenalError: Tool 'tool_name' not registered`

**Solutions:**
- Verify MCP server configuration is correct
- Check MCP server command is executable: `which <command>`
- Test MCP server independently: run command with `--list-tools` (if supported)
- Check arsenal registry logs for tool discovery errors
- Verify arsenal is wired in agent command: verify no TODO at line 296

#### MCP Server Connection Failed

**Symptom:** `ArsenalError: Failed to connect to MCP server`

**Solutions:**
- For STDIO: Verify command and args are correct
- For STDIO: Check executable is in PATH
- For SSE: Verify URL is reachable: `curl <url>`
- For SSE: Check auth token is valid
- Review MCP server logs for startup errors

#### Tool Invocation Timeout

**Symptom:** Tool call hangs or times out

**Solutions:**
- Increase timeout in PaladinConfig
- Check MCP server is responding (may be slow external API)
- Verify tool arguments are valid JSON
- Check MCP server logs for errors

### Scheduler Issues

#### Scheduled Tasks Not Executing

**Symptom:** Jobs scheduled but never run

**Solutions:**
- Verify `scheduler.enabled: true` in config
- Check cron expression is valid: use [crontab.guru](https://crontab.guru/)
- Ensure scheduler port is wired in application (no TODO at line 297)
- Review scheduler logs for errors
- Verify tokio-cron-scheduler is initialized

#### Invalid Cron Expression

**Symptom:** `SchedulerError: Invalid cron expression`

**Solutions:**
- Use standard cron format: `minute hour day month weekday`
- Test expression at [crontab.guru](https://crontab.guru/)
- Use quotes around cron expressions in YAML
- Common format: `"0 0 * * *"` (daily), `"*/15 * * * *"` (every 15 min)

### Configuration File Errors

#### YAML Parsing Failed

**Symptom:** `ConfigError: Failed to parse YAML`

**Solutions:**
- Validate YAML syntax: `yamllint config.yaml`
- Check indentation (use spaces, not tabs)
- Ensure strings with special characters are quoted
- Verify list syntax uses `- ` prefix

#### Required Field Missing

**Symptom:** `ConfigError: Missing required field 'name'`

**Solutions:**
- Review configuration file structure above
- Ensure all required fields are present:
  - `name`
  - `system_prompt`
  - `llm.provider`
  - `llm.model`

#### Environment Variable Not Resolved

**Symptom:** Configuration contains literal `"${VAR_NAME}"`

**Solutions:**
- Export environment variable before running: `export VAR_NAME=value`
- Check variable name matches exactly (case-sensitive)
- Use quotes in YAML: `auth_token: "${TOKEN}"`
- Verify environment variable is set: `echo $VAR_NAME`

### Common Error Messages

| Error | Cause | Solution |
|-------|-------|----------|
| `GarrisonConfigError: Unknown type 'postgres'` | Invalid garrison type | Use `"in_memory"` or `"sqlite"` |
| `ArsenalConfigError: Missing required field 'command'` | STDIO config incomplete | Add `command` and `args` fields |
| `ArsenalConfigError: Missing required field 'url'` | SSE config incomplete | Add `url` field for SSE type |
| `SchedulerError: Job not found` | Attempting to cancel non-existent job | Check JobId is valid before cancellation |
| `LlmError: API key not found` | Missing environment variable | Set provider API key: `export OPENAI_API_KEY=...` |

### Getting Help

Still having issues? Check:

1. **Logs**: Run with `-v` flag for verbose output
   ```bash
   paladin agent run -c config.yaml -i "test" -v
   ```

2. **Test Configuration**: Use `paladin setup-check` to verify environment

3. **GitHub Issues**: [github.com/DF3NDR/paladin-dev-env/issues](https://github.com/DF3NDR/paladin-dev-env/issues)

4. **Documentation**: 
   - [CLI Usage Guide](../CLI_USAGE.md)
   - [Testing Guide](TESTING.md)
   - [Architecture Documentation](../Design/Design_and_Architecture.md)

---

**Last updated:** February 14, 2026  
**Epic:** 23 - CLI, Config & Infrastructure Completion
