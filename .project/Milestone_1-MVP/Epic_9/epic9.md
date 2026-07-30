## Epic 9: Armory CLI Tools

### Overview

**Priority:** Medium  
**Effort:** 2-3 weeks  
**Dependencies:** Epics 1-4  
**Team:** 1 developer

**Objective:** Provide the Armory CLI for rapid Paladin development, testing, and deployment.

### Technical Design

**src/bin/paladin-cli.rs**

```rust
#[derive(Parser)]
#[command(name = "paladin")]
#[command(about = "Paladin Multi-Agent Orchestration CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Paladin operations
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },
    /// Battalion operations
    Battalion {
        #[command(subcommand)]
        action: BattalionCommands,
    },
    /// Arsenal tool management
    Arsenal {
        #[command(subcommand)]
        action: ArsenalCommands,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Run a Paladin from configuration
    Run {
        #[arg(short, long)]
        config: PathBuf,
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Create a new Paladin configuration
    New {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        output: PathBuf,
    },
}
```

### CLI Examples

```bash
# Run a Paladin from config
paladin agent run --config analyst.yaml --input "Analyze Q4 revenue"

# Execute a Formation
paladin battalion run --type formation --config workflow.yaml

# List available tools
paladin arsenal list

# Test an MCP server
paladin arsenal test --mcp-stdio "uvx mcp-hn"
```

### Acceptance Criteria

- [ ] Paladins can be defined and run via CLI
- [ ] Battalions can be orchestrated via CLI
- [ ] Configuration validation with helpful error messages
- [ ] Interactive mode for testing

---
