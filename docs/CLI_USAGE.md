# Paladin CLI Usage Guide

Complete guide to using the Paladin command-line interface for running AI agents and multi-agent battalions.

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Environment Setup](#environment-setup)
- [Commands Reference](#commands-reference)
  - [paladin agent](#paladin-agent)
  - [paladin battalion](#paladin-battalion)
  - [paladin arsenal](#paladin-arsenal)
- [Configuration Files](#configuration-files)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Quick Start

```bash
# 1. Set your API key
export OPENAI_API_KEY="sk-..."

# 2. Generate a Paladin template
paladin agent new -n my-agent -o my-agent.yaml

# 3. Edit the template (customize system_prompt, etc.)
vim my-agent.yaml

# 4. Run your Paladin
paladin agent run -c my-agent.yaml -i "Hello, Paladin!"
```

## Installation

```bash
# Build from source
cargo build --release --bin paladin-cli

# Binary will be at: target/release/paladin-cli

# Add to PATH (optional)
sudo ln -s $(pwd)/target/release/paladin-cli /usr/local/bin/paladin
```

## Environment Setup

### Required: API Keys

Set the appropriate environment variable for your chosen LLM provider:

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# DeepSeek
export DEEPSEEK_API_KEY="sk-..."

# Anthropic
export ANTHROPIC_API_KEY="sk-..."
```

### Optional: MCP Servers

For external tool access (Arsenal), install MCP servers:

```bash
# Web search capability
pip install mcp-web-search

# Or use npx for Node-based servers
npx -y @modelcontextprotocol/server-filesystem /path/to/dir
```

---

## Commands Reference

### paladin agent

Manage and run individual Paladin agents.

#### `paladin agent new`

Generate a new Paladin configuration template.

**Syntax:**
```bash
paladin agent new -n <name> -o <output> [-p <provider>]
```

**Options:**
- `-n, --name <NAME>` - Paladin name (required)
- `-o, --output <PATH>` - Output file path (required)
- `-p, --provider <PROVIDER>` - LLM provider (optional, default: openai)
  - Valid values: `openai`, `deepseek`, `anthropic`

**Examples:**
```bash
# Basic template with OpenAI
paladin agent new -n MyAgent -o agent.yaml

# DeepSeek template
paladin agent new -n DeepAgent -o deepseek-agent.yaml -p deepseek

# Anthropic template
paladin agent new -n ClaudeAgent -o claude-agent.yaml -p anthropic
```

#### `paladin agent run`

Execute a Paladin from a configuration file.

**Syntax:**
```bash
paladin agent run -c <config> [-i <input>] [-o <output>] [-v]
```

**Options:**
- `-c, --config <PATH>` - Configuration file path (required)
- `-i, --input <TEXT>` - Input text (optional, prompts if omitted)
- `-o, --output <PATH>` - Save JSON output to file (optional)
- `-v, --verbose` - Show detailed execution logs (optional)

**Examples:**
```bash
# Run with command-line input
paladin agent run -c agent.yaml -i "What is Rust?"

# Interactive mode (prompts for input)
paladin agent run -c agent.yaml

# With verbose output
paladin agent run -c agent.yaml -i "Query" --verbose

# Save results to file
paladin agent run -c agent.yaml -i "Query" -o result.json
```

---

### paladin battalion

Manage and run multi-agent battalions.

#### `paladin battalion new`

Generate a new Battalion configuration template.

**Syntax:**
```bash
paladin battalion new -n <name> -t <type> -o <output>
```

**Options:**
- `-n, --name <NAME>` - Battalion name (required)
- `-t, --type <TYPE>` - Battalion type (required)
  - `formation` - Sequential execution (pipeline)
  - `phalanx` - Parallel execution (concurrent)
  - `campaign` - DAG workflow (complex dependencies)
  - `chain-of-command` - Hierarchical delegation
- `-o, --output <PATH>` - Output file path (required)

**Examples:**
```bash
# Formation (sequential)
paladin battalion new -n MyFormation -t formation -o formation.yaml

# Phalanx (parallel)
paladin battalion new -n MyPhalanx -t phalanx -o phalanx.yaml

# Campaign (DAG)
paladin battalion new -n MyCampaign -t campaign -o campaign.yaml

# Chain of Command (hierarchical)
paladin battalion new -n MyTeam -t chain-of-command -o team.yaml
```

#### `paladin battalion run`

Execute a Battalion from a configuration file.

**Syntax:**
```bash
paladin battalion run -c <config> [-i <input>] [-o <output>] [-v]
```

**Options:**
- `-c, --config <PATH>` - Configuration file path (required)
- `-i, --input <TEXT>` - Input text (optional, prompts if omitted)
- `-o, --output <PATH>` - Save JSON output to file (optional)
- `-v, --verbose` - Show detailed execution logs (optional)

**Examples:**
```bash
# Run formation
paladin battalion run -c formation.yaml -i "Process this text"

# Run phalanx with verbose output
paladin battalion run -c phalanx.yaml -i "Analyze this" --verbose

# Run campaign and save results
paladin battalion run -c campaign.yaml -i "Input" -o results.json
```

---

### paladin arsenal

Manage and test external tools (MCP servers).

#### `paladin arsenal list`

List all configured MCP servers and their tools.

**Syntax:**
```bash
paladin arsenal list
```

**Example:**
```bash
paladin arsenal list

# Output:
# Tool Name       | Description          | Type   | Status
# ────────────────┼──────────────────────┼────────┼─────────
# web_search      | Search the web       | stdio  | ✓ Connected
# filesystem      | File operations      | stdio  | ✓ Connected
```

#### `paladin arsenal test`

Test connection to an MCP server.

**Syntax:**
```bash
paladin arsenal test --mcp-stdio <command>
paladin arsenal test --mcp-sse <url>
```

**Options:**
- `--mcp-stdio <COMMAND>` - Test STDIO MCP server (mutually exclusive with --mcp-sse)
- `--mcp-sse <URL>` - Test SSE MCP server (mutually exclusive with --mcp-stdio)

**Examples:**
```bash
# Test STDIO server
paladin arsenal test --mcp-stdio "uvx mcp-web-search"

# Test SSE server
paladin arsenal test --mcp-sse "http://localhost:3000/mcp"

# With full command and args
paladin arsenal test --mcp-stdio "npx -y @modelcontextprotocol/server-filesystem /tmp"
```

---

## Configuration Files

### Paladin Configuration Schema

```yaml
# Identity
name: "PaladinName"
user_name: "UserName"

# System prompt (most important!)
system_prompt: |
  Define the Paladin's role, capabilities, and behavior here.

# LLM settings
model: "gpt-4"
temperature: 0.7
max_loops: 3
timeout_seconds: 300
stop_words: ["STOP"]

# Provider
provider:
  type: openai  # or deepseek, anthropic
  
# Optional: Memory
garrison:
  type: sqlite
  path: ./garrison.db
  max_entries: 1000

# Optional: Tools
arsenal:
  mcp_servers:
    - name: web_search
      type: stdio
      command: uvx
      args: [mcp-web-search]
```

### Battalion Configuration Schema

**Formation (Sequential):**
```yaml
type: formation
name: "FormationName"
pass_output_to_next: true
paladins:
  - inline: { ... paladin config ... }
  - inline: { ... paladin config ... }
```

**Phalanx (Parallel):**
```yaml
type: phalanx
name: "PhalanxName"
paladins:
  - inline: { ... paladin config ... }
  - inline: { ... paladin config ... }
inputs: []  # Optional: different input for each
```

**Campaign (DAG):**
```yaml
type: campaign
name: "CampaignName"
nodes:
  - id: node1
    paladin: { inline: { ... } }
  - id: node2
    paladin: { inline: { ... } }
edges:
  - from: node1
    to: node2
start_node: node1
```

**Chain of Command (Hierarchical):**
```yaml
type: chain_of_command
name: "TeamName"
commander:
  inline: { ... paladin config ... }
delegates:
  - inline: { ... paladin config ... }
  - inline: { ... paladin config ... }
```

---

## Examples

### Example 1: Simple Q&A Agent

```bash
# 1. Create config
cat > qa-agent.yaml << 'EOF'
name: "QAAgent"
system_prompt: "You are a helpful Q&A assistant."
model: "gpt-4"
temperature: 0.7
max_loops: 1
provider: { type: openai }
EOF

# 2. Run
export OPENAI_API_KEY="sk-..."
paladin agent run -c qa-agent.yaml -i "What is Rust?"
```

### Example 2: Multi-Stage Analysis

```bash
# 1. Generate formation template
paladin battalion new -n Analysis -t formation -o analysis.yaml

# 2. Edit to add analyzer → summarizer → validator stages

# 3. Run
paladin battalion run -c analysis.yaml -i "$(cat document.txt)"
```

### Example 3: Agent with Web Search

```bash
# 1. Install MCP web search
pip install mcp-web-search

# 2. Create config with arsenal
cat > web-agent.yaml << 'EOF'
name: "WebAgent"
system_prompt: "You can search the web for current information."
model: "gpt-4"
temperature: 0.7
max_loops: 3
provider: { type: openai }
arsenal:
  mcp_servers:
    - name: web_search
      type: stdio
      command: uvx
      args: [mcp-web-search]
EOF

# 3. Run
paladin agent run -c web-agent.yaml -i "Latest AI news"
```

---

## Troubleshooting

### Common Errors

#### Error: "Missing API key"

**Problem:** Required environment variable not set.

**Solution:**
```bash
export OPENAI_API_KEY="sk-..."
# Or for other providers:
export DEEPSEEK_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-..."
```

#### Error: "Config file not found"

**Problem:** Path to configuration file is incorrect.

**Solution:**
- Use absolute paths: `/full/path/to/config.yaml`
- Or relative from current directory: `./config.yaml`
- Check file exists: `ls -l config.yaml`

#### Error: "Invalid YAML"

**Problem:** Syntax error in configuration file.

**Solution:**
- Validate YAML online: https://www.yamllint.com/
- Check indentation (use spaces, not tabs)
- Ensure all strings with special characters are quoted
- Use `yamllint config.yaml` if available

#### Error: "Invalid provider"

**Problem:** Provider type not recognized.

**Solution:**
- Valid providers: `openai`, `deepseek`, `anthropic`
- Check spelling in config file
- Use `paladin agent new -p <provider>` to generate correct template

#### Error: "MCP server connection failed"

**Problem:** Cannot connect to MCP server.

**Solution:**
- Verify server is installed: `which uvx`, `which npx`
- Test server manually: `uvx mcp-web-search`
- Check command and args in config
- Ensure server supports MCP protocol
- Review server logs in stderr

#### Error: "Timeout"

**Problem:** Execution exceeded configured timeout.

**Solution:**
- Increase `timeout_seconds` in config
- Reduce `max_loops` for simpler tasks
- Check if LLM API is responding slowly
- Verify network connectivity

#### Error: "Rate limit exceeded"

**Problem:** Too many API requests to LLM provider.

**Solution:**
- Wait and retry
- Use `--verbose` to see which call failed
- Consider using cheaper model for testing
- Check provider's rate limits
- Add delays between requests

### Getting Help

- **Documentation:** See `examples/cli_configs/` for working examples
- **Issues:** Report bugs at https://github.com/DF3NDR/paladin-dev-env/issues
- **Verbose Mode:** Use `--verbose` flag to see detailed execution logs
- **Logs:** Check stderr output for detailed error messages

### Performance Tips

1. **Model Selection:**
   - Use `gpt-3.5-turbo` for simple tasks (faster, cheaper)
   - Use `gpt-4` for complex reasoning
   - Use `deepseek-chat` for cost-effective alternative

2. **Temperature:**
   - Lower (0.0-0.3) for factual, consistent outputs
   - Medium (0.4-0.7) for balanced responses
   - Higher (0.8-1.0) for creative, varied outputs

3. **Max Loops:**
   - 1-2: Simple single-response tasks
   - 3-5: Default for most tasks
   - 6+: Complex multi-step reasoning

4. **Timeouts:**
   - 60s: Simple queries
   - 180-300s: Standard tasks
   - 600s+: Complex multi-step operations

5. **Battalions:**
   - Use Phalanx for parallel speedup
   - Use Formation for sequential pipelines
   - Monitor costs with `--verbose`

---

## Advanced Topics

### External Configuration References

Instead of inline Paladin configs, reference external files:

```yaml
paladins:
  - file: ./agents/analyzer.yaml
  - file: ./agents/summarizer.yaml
```

### Environment Variable Substitution

Use environment variables in configs:

```yaml
provider:
  api_key_env: "${CUSTOM_API_KEY_VAR}"
```

### Custom MCP Servers

Create your own tools:
- Implement MCP protocol
- Register in arsenal configuration
- See MCP documentation: https://modelcontextprotocol.io/

### Streaming Responses

For real-time output (coming soon):
```bash
paladin agent run -c config.yaml -i "Query" --stream
```

---

## See Also

- [Basic Paladin Example](../examples/cli_configs/basic_paladin.yaml)
- [Advanced Paladin Example](../examples/cli_configs/advanced_paladin.yaml)
- [Formation Example](../examples/cli_configs/formation.yaml)
- [Phalanx Example](../examples/cli_configs/phalanx.yaml)
- [Campaign Example](../examples/cli_configs/campaign.yaml)
- [Chain of Command Example](../examples/cli_configs/chain_of_command.yaml)
- [Main README](../README.md)
