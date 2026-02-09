# Paladin

## General Overview

Paladin is a Rust-based, modular content processing platform built with Hexagonal Architecture principles. It provides configurable pipelines to ingest, normalize, analyze, enrich, store, and deliver diverse content types (web pages, documents, feeds, and streams) at scale.

Key capabilities include configurable content ingestion, NLP/LLM analysis and summarization, metadata extraction and subject tagging, content filtering and lifecycle management, and flexible delivery channels (HTTP/webhooks, email, push, and queue-based adapters). Clear ports-and-adapters boundaries make integrations, testing, and replacement of components straightforward. Paladin is designed for content aggregation, monitoring, enrichment, and automated delivery workflows where extensibility, observability, and reliable storage across SQL/NoSQL/file backends are important.

## Quick Start (CLI)

Paladin provides a powerful command-line interface for quick agent creation and orchestration without writing code:

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/paladin.git
cd paladin

# Build the CLI
cargo build --release

# The binary will be available at target/release/paladin
```

### First-Time Setup

Run the interactive onboarding wizard to configure your environment:

```bash
paladin onboarding
```

This wizard will:
- Guide you through provider selection (OpenAI, DeepSeek, Anthropic)
- Help you set up API keys securely
- Validate your configuration
- Create a `.env` file with your settings
- Generate sample configurations to get started

### Quick Commands

```bash
# Verify your setup
paladin setup-check

# List available features
paladin features

# Generate an AI workflow from description
paladin muster "Create a 3-step data analysis pipeline" -o pipeline.yaml

# Quick multi-agent discussion
paladin council "Should we adopt microservices?" -n 5

# Run a configuration
paladin run -c examples/cli_configs/basic_paladin.yaml -i "Hello, explain Rust ownership"

# Interactive mode
paladin run -c config.yaml --interactive
```

### Documentation

- **[CLI Usage Guide](docs/CLI_USAGE.md)** - Complete command reference
- **[Onboarding Guide](docs/cli/ONBOARDING.md)** - First-time setup walkthrough
- **[Setup Check Guide](docs/cli/SETUP_CHECK.md)** - Environment validation
- **[Muster Command](docs/cli/MUSTER.md)** - AI-powered Battalion generation
- **[Council Command](docs/cli/COUNCIL.md)** - Quick multi-agent discussions
- **[Example Configs](examples/cli_configs/)** - Sample configurations

See [docs/CLI_USAGE.md](docs/CLI_USAGE.md) for comprehensive CLI documentation.

## Architectural Overview

Paladin is an enterprise capable AI Orchestration system designed using hexagonal architecture principles to provide robust and flexible handling of a large range of functionality including processing any type of content (structured documents, test, audio, video, images, etc), notification management (push, email, sms, etc), machine learning integrations (LLM's, ML, etc), and content delivery mechanisms (web servers, streaming, apis, etc).

This project utilizes clearly defined Ports and Adapters, enabling seamless integration with external services such as email, SMS, push notifications, webhooks, machine learning models, and more. The design ensures high modularity, scalability, and ease of maintenance.

## Project Structure

* `src/application` – Application layer containing use cases, ports, and storage repositories.

  * `use_cases` – Business logic and services like content aggregation, filtering, summarization, and analysis.
  * `ports` – Interfaces for interaction with external systems.
  * `storage` – Abstracts various storage mechanisms (SQL, NoSQL, File storage).

## Features

### Ports and Adapters

The project clearly defines Ports as interfaces to external systems, enabling adapters to easily integrate specific implementations.

**Output Ports:**

* **Notification Publisher:** Abstracts notification services for channels like Email, SMS, Push, Slack, Discord, etc.
* **Content Delivery:** Manages content distribution methods, including HTTP, email, webhooks, and push notifications.
* **Logging:** Provides a standardized logging mechanism.
* **Search Engine:** Defines interactions with external search services.

**Input Ports:**

* **Content Ingestion:** Facilitates fetching and ingestion of external content.
* **RPC Gateway:** Exposes application functionalities via REST, GraphQL, gRPC, etc.
* **Machine Learning (ML):** Interfaces with ML models like TensorFlow or PyTorch.
* **LLM Port:** Provides integration with Low-Level Models for content analysis.
* **NLP Port:** Interfaces with Natural Language Processing services.

### Use Cases

* **Content Aggregation and Summarization:** Aggregates various content sources and generates concise summaries.
* **Content Filtering:** Implements filtering mechanisms based on keywords or criteria.
* **ML and NLP Analysis:** Leverages machine learning and NLP models to analyze and enrich content.
* **Subject Tagging and Searching:** Advanced tagging, indexing, and search capabilities.
* **Paladin:** Autonomous AI agents with memory and context management.
* **Garrison Memory System:** Persistent conversation history with windowing and search capabilities.
* **Arsenal Tool System:** External tool integration via Model Context Protocol (MCP).
* **Battalion Orchestration:** Multi-agent coordination with eight orchestration patterns.
* **Maneuver Flow DSL:** String-based workflow orchestration with declarative syntax.
* **Herald Output Formatting:** Pluggable formatters (JSON, Markdown, Table) with streaming support.

### AI Agent System (Paladin)

Paladin provides a sophisticated AI agent framework with memory management and tool capabilities:

* **Paladins**: Autonomous AI agents with configurable behaviors and tool access
* **Garrison Memory**: Context-aware conversation history storage
  * **InMemoryGarrison**: Fast, ephemeral storage for development and testing
  * **SqliteGarrison**: Persistent storage with full-text search for production
* **Arsenal Tool System**: External tool integration via MCP
  * **STDIO Transport**: Command-line tool execution (Python, Node.js, binaries)
  * **SSE Transport**: HTTP-based remote tool services
  * **Tool Registry**: Dynamic tool discovery and registration
  * **Resource Controls**: Timeout management and concurrency limiting
* **Sanctum Long-term Memory**: Persistent, searchable vector memory for agents
  * **InMemory Adapter**: Fast, ephemeral storage for development (<10,000 entries)
  * **Qdrant Adapter**: Production-ready vector database (millions of entries, <500ms search)
  * **Semantic Search**: Find relevant memories using vector similarity
  * **Rich Metadata**: Filter by paladin ID, memory type, importance, timestamps
  * **Memory Types**: Episodic (events), Semantic (facts), Procedural (skills)
* **Circuit Breaker**: Fault tolerance with automatic retry and backoff
* **Execution Service**: Orchestrates agent execution with memory integration

See [docs/GARRISON.md](docs/GARRISON.md) for detailed memory system documentation.
See [docs/ARSENAL.md](docs/ARSENAL.md) for comprehensive tool system documentation.
See [docs/SANCTUM.md](docs/SANCTUM.md) for long-term memory system documentation.

### RAG (Retrieval-Augmented Generation)

**New in Epic 12**: Paladin now supports automatic RAG integration, enabling agents to retrieve relevant context from long-term memory and extract new memories after execution.

**What is RAG?**
- **Automatic Context Retrieval**: Fetch relevant memories before generating responses
- **Smart Memory Extraction**: Automatically store important facts after conversations
- **Knowledge Building**: Accumulate wisdom across multiple sessions
- **Response Quality**: Improve accuracy with historical context

**Quick Start:**

```yaml
# config.yml
sanctum:
  provider: in_memory  # or 'qdrant' for production

rag:
  top_k: 5                    # Retrieve top 5 relevant memories
  min_similarity: 0.7          # 70% relevance threshold
  max_tokens: 2000             # Context budget
  timeout_seconds: 5           # Retrieval timeout

memory_extraction:
  enabled: true
  strategy: on_completion     # Extract after each conversation
```

**Example Usage:**

```rust
use paladin::application::use_cases::sanctum::{
    RagRetrievalService, MemoryExtractionService
};

// Create RAG services
let rag_service = Arc::new(RagRetrievalService::new(
    sanctum_port, embedding_port, rag_config
));

let extraction_service = Arc::new(MemoryExtractionService::new(
    llm_port, embedding_port, sanctum_port
));

// Configure Paladin with RAG
let execution_service = PaladinExecutionService::new(llm_port)
    .with_rag_retrieval(rag_service)
    .with_memory_extraction(extraction_service);

// Execute with automatic RAG
let result = execution_service.execute(&paladin, "user input").await?;
// ✓ Context automatically retrieved from Sanctum
// ✓ Response generated with historical context
// ✓ New memories extracted and stored
```

**Benefits:**
- 🧠 **Contextual Awareness**: Agents remember previous conversations
- 📈 **Improved Accuracy**: Responses reference historical knowledge
- 🔄 **Automatic**: No manual memory management needed
- ⚙️ **Configurable**: Tune retrieval and extraction parameters
- 🚀 **Scalable**: Works with both in-memory and Qdrant storage

**Learn More:**
- [docs/SANCTUM.md#rag-integration](docs/SANCTUM.md#rag-integration-retrieval-augmented-generation) - Complete RAG documentation
- `examples/paladin_with_rag.rs` - Configuration guide and workflow
- `examples/cli_configs/paladin_rag.yaml` - Full configuration example

### Multi-Provider LLM Support

Paladin supports multiple LLM providers with a consistent interface, allowing you to choose the best provider for your needs:

* **OpenAI** (GPT-4, GPT-3.5-turbo): Mature ecosystem, vision support, production-ready
* **DeepSeek**: Cost-effective, strong reasoning capabilities, high throughput
* **Anthropic Claude**: Safety-focused, long context (200K tokens), complex analysis

**Key Features**:
* Unified `LlmPort` trait across all providers
* Hot-swappable providers without code changes
* Provider-specific capabilities detection
* Automatic retry with exponential backoff
* Comprehensive error handling and rate limiting

**Configuration Example**:
```yaml
llm:
  default_provider: "openai"  # or "deepseek", "anthropic"
  
  openai:
    api_key: "${OPENAI_API_KEY}"
    model: "gpt-4"
  
  deepseek:
    api_key: "${DEEPSEEK_API_KEY}"
    model: "deepseek-chat"
  
  anthropic:
    api_key: "${ANTHROPIC_API_KEY}"
    model: "claude-3-5-sonnet-20241022"
```

**Programmatic Usage**:
```rust
// Create adapter
let config = DeepSeekConfig::from_env()?;
let llm_port = Arc::new(DeepSeekAdapter::new(config)?);

// Use with Paladin
let paladin = PaladinBuilder::new(llm_port)
    .system_prompt("You are a helpful assistant")
    .build()?;
```

See [docs/PROVIDER_EXPANSION.md](docs/PROVIDER_EXPANSION.md) for detailed comparison and migration guide.
See [docs/CONTRIBUTING_PROVIDERS.md](docs/CONTRIBUTING_PROVIDERS.md) to add new providers.

### Vision & Multi-Modal Processing (Sentinel)

**New in Epic 13**: Paladin now supports vision capabilities, enabling agents to analyze images and documents alongside text. The **Sentinel Vision System** provides seamless integration with vision-capable LLM providers.

**Key Features:**
* 🖼️ **Image Analysis**: Process images from URLs, files, or base64-encoded data
* 📄 **Document Processing**: Extract text from PDFs with intelligent chunking for RAG
* 🎚️ **Quality Control**: Auto/Low/High detail levels for cost/accuracy trade-offs
* 🔐 **Secure**: Built-in encryption (ChaCha20-Poly1305) and audit logging
* 🏰 **Battalion-Ready**: Vision works seamlessly with all orchestration patterns
* 🔌 **Multi-Provider**: OpenAI GPT-4o and Anthropic Claude 3 support

**Quick Example:**

```rust
use paladin::core::platform::container::vision::{VisionContent, ImageDetail};

// Create vision-enabled Paladin
let paladin = PaladinBuilder::new(llm_port)
    .system_prompt("You are a professional image analyst")
    .enable_vision(true)
    .build()?;

// Analyze an image
let image = VisionContent::ImageUrl {
    url: "https://example.com/photo.jpg".to_string(),
    detail: ImageDetail::High,
};

let result = execution_service
    .execute_with_vision(&paladin, "What's in this image?", vec![image])
    .await?;
```

**Supported Vision Content:**
- **ImageUrl**: Reference publicly accessible images
- **ImageFile**: Load images from local filesystem  
- **ImageBase64**: Embed encoded image data directly

**Document Processing:**

```rust
use paladin::application::ports::input::document_port::{DocumentPort, ChunkConfig};

// Extract text from PDF
let document = document_adapter
    .ingest(DocumentSource::File("paper.pdf".into()))
    .await?;

// Chunk for RAG (500 chars, 100 overlap)
let chunks = document_adapter.chunk(&document, ChunkConfig {
    chunk_size: 500,
    chunk_overlap: 100,
    separator: "\n\n".to_string(),
}).await?;
```

**CLI Usage:**

```bash
# Analyze single image
paladin vision analyze \
  --image photo.jpg \
  --prompt "Describe this image" \
  --detail high

# Process PDF document
paladin vision document \
  --file report.pdf \
  --chunk-size 1000 \
  --output chunks.json
```

**Battalion Integration:**
Vision capabilities work with all Battalion patterns:
- **Formation**: Sequential vision analysis pipeline
- **Phalanx**: Parallel image processing (~3x speedup)
- **Campaign**: Complex vision workflows with branching
- **Chain of Command**: Hierarchical image review processes

**Learn More:**
- [docs/SENTINEL.md](docs/SENTINEL.md) - Complete vision system documentation
- [docs/BATTALION_VISION_SUPPORT.md](docs/BATTALION_VISION_SUPPORT.md) - Multi-agent vision patterns
- `examples/vision_analysis.rs` - Single-image analysis walkthrough
- `examples/document_processing.rs` - PDF extraction and chunking
- `examples/vision_battalion.rs` - Formation and Phalanx patterns

### Battalion Orchestration System

Battalion provides powerful multi-agent coordination capabilities with **eight distinct orchestration patterns**:

* **Formation (Sequential)**: Execute Paladins in sequence, passing output from one to the next
  * Perfect for multi-step pipelines and data transformation workflows
  * Linear execution with output chaining
* **Phalanx (Concurrent)**: Execute all Paladins simultaneously with result aggregation
  * Strategies: CollectAll, FirstSuccess, Majority, Custom
  * Ideal for parallel analysis and consensus building
* **Campaign (Graph/DAG)**: Conditional routing through a directed acyclic graph
  * Edge conditions: Always, Contains, Regex, Custom
  * Complex workflows with branching logic and fan-out/fan-in patterns
* **Chain of Command (Hierarchical)**: Commander analyzes input and delegates to specialists
  * Delegation strategies: Automatic (LLM-based), Broadcast, RoundRobin, Custom
  * Intelligent task routing and load distribution
* **Conclave (Multi-Expert Synthesis)**: Multiple experts analyze in parallel, aggregator synthesizes perspectives
  * Implements Mixture-of-Agents pattern for higher quality outputs
  * Perfect for expert panel decisions requiring multiple perspectives (technical, business, security)
  * Resilience: Continues even if some experts fail (partial success)
  * Configurable retry logic with exponential backoff
* **Council (Iterative Discussion)**: Iterative multi-round discussion pattern with turn-based dialogue
  * Enables debate, consensus building, and deliberative decision-making
  * Configurable turn limits, participation rules, and discussion strategies
  * Built-in conclusion detection and summary generation
* **Grove (Contextual Routing)**: Advanced routing with contextual analysis and LLM-based decision making
  * Semantic routing based on input content and historical performance
  * Fallback chains and multi-stage routing pipelines
  * Dynamic scoring and performance-based optimization
* **Maneuver (Flow DSL)**: String-based workflow orchestration with declarative syntax
  * Compact syntax: `"a -> (b, c) -> d"` for sequential and parallel flows
  * Error handling strategies: FailFast, ContinueParallel, IgnoreErrors
  * Built-in visualization (ASCII tree, Mermaid flowcharts) and CLI tools
  * See [Flow DSL Guide](docs/guides/flow-dsl.md) for complete syntax and examples

**Performance**: Handles 100+ concurrent Battalions with <10ms orchestration overhead

**Error Resilience**: Three strategies (FailFast, ContinueOnError, RetryThenContinue) with exponential backoff

**Testing**: 218 comprehensive tests (85 unit + 133 integration) ensuring reliability

See [docs/BATTALION.md](docs/BATTALION.md) for comprehensive orchestration documentation.

### Herald Output Formatting System

Herald provides a powerful, pluggable output formatting system that transforms Paladin and Battalion execution results into human-readable formats:

* **JSON Herald**: Structured JSON output for APIs and machine parsing
  * Pretty-printed or compact format
  * Optional timestamps
  * NDJSON streaming support
* **Markdown Herald**: Beautiful, human-readable output with colors
  * Color-coded status badges (✅ ❌ ⏱️)
  * Configurable heading levels
  * Progressive text streaming
* **Table Herald**: Compact ASCII/Unicode tables for dashboards
  * Multiple border styles (rounded, ascii, modern, none)
  * Automatic text truncation
  * Buffered rendering for consistency

**Key Features**:
* ⚡ **High Performance**: <1ms for 10KB results (tested at 0.0095ms - 105x faster than target)
* 🔌 **Pluggable**: Easy to implement custom formatters (XML, CSV, etc.)
* 📡 **Streaming Support**: Three strategies (NDJSON, Progressive, Buffered)
* ⚙️ **Configurable**: YAML-based config with runtime overrides
* 🏗️ **Type-Safe**: Strong typing with comprehensive error handling

**Quick Example**:
```rust
use paladin::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald};
use std::sync::Arc;

// Create Herald from config
let herald = settings.create_default_herald()?;

// Or create directly
let herald: Arc<dyn Herald> = Arc::new(MarkdownHerald::new());

// Use with Paladin
let service = PaladinExecutionService::new(llm_port, cb, None, None)
    .with_herald(herald);

let result = service.execute(&paladin, "input").await?;
if let Some(formatted) = service.format_result(&result, &paladin)? {
    println!("{}", formatted);
}
```

**Custom Formatters**: Implement the `Herald` trait to create custom output formats (XML, CSV, YAML, etc.)

See [docs/HERALD.md](docs/HERALD.md) for comprehensive formatting documentation.

### Storage Solutions

* **SQL Store:** Interfaces with relational databases for structured data storage and transactions.
* **NoSQL Store:** Manages unstructured data with NoSQL databases.
* **File Store:** Handles storage and retrieval of files.
* **Key and Key-Value Stores:** Efficient storage and retrieval mechanisms for keys and values.

## Getting Started

### Prerequisites

* Rust (latest stable version)
* Cargo package manager

### Installation

Clone the repository:

```sh
git clone <repository-url>
cd Paladin
```

### Building

To build the project, run:

```sh
cargo build
```

### Running Tests

Run unit tests to ensure functionality:

```sh
cargo test
```

### CLI Tools (Armory)

Paladin includes a powerful command-line interface for managing agents, battalions, and tools without writing code.

**Quick Start:**

```bash
# Build the CLI
cargo build --release

# Create a new Paladin agent from template
./target/release/paladin agent new my-assistant --output agent.yaml

# Edit the configuration file (agent.yaml) with your settings
# Then execute the agent
./target/release/paladin agent run agent.yaml "What is Rust?"

# Create and run a Battalion (multi-agent workflow)
./target/release/paladin battalion new formation --output pipeline.yaml
./target/release/paladin battalion run pipeline.yaml "Analyze this text"

# List available tools
./target/release/paladin arsenal list
```

**Features:**
- 🤖 **Agent Management**: Create, configure, and execute Paladins
- 🏢 **Battalion Operations**: Orchestrate multi-agent workflows (Formation, Phalanx, Campaign, Chain of Command)
- 🛠️ **Arsenal Tools**: List and test MCP-compatible tools
- 📝 **Template System**: Quick-start YAML templates for all agent types
- 🔧 **Interactive Mode**: Prompts for missing configuration
- 📊 **Multiple Output Formats**: JSON, Markdown, or save to file

**Comprehensive Documentation**: See [docs/CLI_USAGE.md](docs/CLI_USAGE.md) for detailed usage guide, including:
- Complete command reference with all flags and options
- Configuration file schemas and examples
- Troubleshooting guide for common errors
- Environment variable setup
- Advanced topics (streaming, custom formatters, arsenal integration)

## Examples

### Paladin Agent with Memory

```rust
use paladin::application::use_cases::paladin::{PaladinBuilder, PaladinExecutionService, CircuitBreaker};
use paladin::infrastructure::adapters::garrison::InMemoryGarrison;
use paladin::core::platform::container::garrison::GarrisonConfig;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create memory system
    let garrison = Arc::new(InMemoryGarrison::new(GarrisonConfig::default()));
    
    // Build agent
    let paladin = PaladinBuilder::new(llm_port)
        .name("Assistant")
        .system_prompt("You are a helpful AI assistant.")
        .with_garrison(garrison.clone())
        .build()?;
    
    // Execute with memory
    let circuit_breaker = Arc::new(CircuitBreaker::new(3, 2, 30000));
    let service = PaladinExecutionService::new(llm_port, circuit_breaker, Some(garrison));
    
    let result = service.execute(&paladin, "What is Rust?").await?;
    println!("Response: {}", result.content);
    
    Ok(())
}
```

See `examples/` directory for more examples:
- `garrison_in_memory.rs` - In-memory conversation history
- `garrison_persistent.rs` - SQLite persistence example
- `garrison_semantic_search.rs` - Future vector search demo
- `arsenal_stdio_tools.rs` - STDIO MCP tool integration
- `arsenal_sse_tools.rs` - SSE MCP tool integration
- `formation_sequential.rs` - Sequential Paladin pipeline
- `phalanx_parallel.rs` - Concurrent Paladin execution
- `campaign_workflow.rs` - Graph-based conditional routing
- `chain_of_command_delegation.rs` - Hierarchical task delegation

### Battalion Formation Example

```rust
use paladin::application::use_cases::battalion::formation_service::FormationExecutionService;
use paladin::core::platform::container::battalion::formation::Formation;
use paladin::core::platform::container::battalion::BattalionConfig;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sequential pipeline of Paladins
    let paladins = vec![
        create_paladin("analyzer", "Analyze the input data"),
        create_paladin("processor", "Process the analyzed data"),
        create_paladin("summarizer", "Create a summary"),
    ];
    
    let config = BattalionConfig::default();
    let formation = Formation::new(paladins, config)?;
    
    // Execute: output from each Paladin flows to the next
    let service = FormationExecutionService::new(llm_port);
    let result = service.execute(&formation, "Process this data").await?;
    
    println!("Final result: {:?}", result);
    Ok(())
}
```

See [docs/BATTALION.md](docs/BATTALION.md) for comprehensive orchestration documentation.

### Arsenal Tool System Example

```rust
use paladin::application::use_cases::arsenal::ArsenalRegistryService;
use paladin::application::ports::output::arsenal_port::ArsenalRegistry;
use paladin::infrastructure::adapters::arsenal::Armament;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create tool registry
    let registry = Arc::new(ArsenalRegistryService::new());

    // Register a calculator tool
    let calculator = Armament {
        name: "calculator".to_string(),
        description: "Performs basic arithmetic operations".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {"type": "string", "enum": ["add", "subtract", "multiply", "divide"]},
                "a": {"type": "number"},
                "b": {"type": "number"}
            },
            "required": ["operation", "a", "b"]
        }),
        required_params: vec!["operation".to_string(), "a".to_string(), "b".to_string()],
    };

    registry.register(calculator).await;

    // Paladin agents can now use the calculator tool via function calling
    Ok(())
}
```

See [docs/ARSENAL.md](docs/ARSENAL.md) for comprehensive tool system documentation.

### Notification Example

```rust
let notification_request = NotificationRequest {
    recipient: NotificationRecipient::Email("user@example.com".to_string()),
    content: NotificationContent {
        title: "Welcome!".to_string(),
        body: "Thank you for joining Paladin.".to_string(),
        category: "info".to_string(),
        action_url: None,
        attachments: None,
        template_id: None,
        template_variables: None,
    },
    channel: NotificationChannel::Email,
    priority: NotificationPriority::Normal,
    scheduled_time: None,
    expiry_time: None,
    metadata: None,
};

let response = notification_service.send_notification(notification_request)?;
```

### Content Delivery Example

```rust
let delivery_request = DeliveryRequest {
    recipient_id: "user123".to_string(),
    delivery_method: DeliveryMethod::Http {
        endpoint: "https://example.com/webhook".to_string(),
        headers: None,
    },
    content_payload: ContentPayload::Notification(NotificationContent {
        title: "Notification Title".to_string(),
        body: "Content body".to_string(),
        category: "update".to_string(),
        action_url: None,
        expires_at: None,
    }),
    priority: DeliveryPriority::Normal,
    scheduled_time: None,
    metadata: None,
};

let delivery_response = content_delivery_service.deliver_content(delivery_request)?;
```

## Documentation

Comprehensive documentation is available in the `docs/` directory:

### Getting Started

- **[Installation Guide](docs/INSTALLATION.md)** - Platform-specific installation instructions
- **[Quick Start](docs/QUICKSTART.md)** - 15-minute tutorial to get up and running
- **[Examples Gallery](examples/README.md)** - Complete catalog of working examples with learning path

### User Guides

- **[Paladin Configuration](docs/guides/paladin-configuration.md)** - Agent configuration, system prompts, and behavior tuning
- **[Battalion Patterns](docs/guides/battalion-patterns.md)** - Multi-agent orchestration strategies (Formation, Phalanx, Campaign, Chain of Command)
- **[Tool Integration](docs/guides/tool-integration.md)** - Arsenal system and MCP protocol integration for external tools
- **[Memory Management](docs/guides/memory-management.md)** - Garrison memory system with windowing and semantic search
- **[Output Formatting](docs/guides/output-formatting.md)** - Herald formatters for JSON, Markdown, HTML, and custom formats
- **[User System](docs/USER_SYSTEM.md)** - User authentication and authorization
- **[CLI Usage](docs/CLI_USAGE.md)** - Command-line interface (Armory) reference

### Architecture & Design

- **[Architecture Overview](docs/architecture/overview.md)** - Three-layer hexagonal architecture with diagrams
- **[Hexagonal Design](docs/architecture/hexagonal-design.md)** - Ports and adapters pattern with implementation details
- **[Domain Model](docs/architecture/domain-model.md)** - DDD entities, aggregates, and business rules
- **[Design Patterns](docs/architecture/design-patterns.md)** - Builder, Factory, Strategy, Repository patterns
- **[Design & Architecture](docs/Design/Design_and_Architecture.md)** - Comprehensive system design document

### Deployment

- **[Docker Deployment](docs/deployment/docker.md)** - Multi-arch images, docker-compose, and production configuration
- **[Kubernetes Deployment](docs/deployment/kubernetes.md)** - K8s manifests, Helm charts, auto-scaling, and HA setup
- **[CI/CD Pipeline](docs/deployment/cicd.md)** - GitHub Actions workflows for automated testing and deployment
- **[Production Best Practices](docs/deployment/production-best-practices.md)** - Security, performance, and reliability checklist

### Operations

- **[Logging](docs/operations/logging.md)** - Structured logging with tracing, log levels, and aggregation
- **[Monitoring](docs/operations/monitoring.md)** - Metrics collection with Prometheus and Grafana dashboards
- **[Troubleshooting](docs/operations/troubleshooting.md)** - Common issues and diagnostic procedures
- **[Performance Tuning](docs/operations/performance-tuning.md)** - Optimization strategies and benchmarking

### Component Documentation

- **[Battalion System](docs/BATTALION.md)** - Multi-agent orchestration patterns and benchmarks
- **[Garrison System](docs/GARRISON.md)** - Memory management and conversation history
- **[Arsenal System](docs/ARSENAL.md)** - Tool integration and MCP protocol
- **[Herald System](docs/HERALD.md)** - Output formatting and transformation
- **[Provider Expansion](docs/PROVIDER_EXPANSION.md)** - LLM provider comparison and selection guide
- **[Contributing Providers](docs/CONTRIBUTING_PROVIDERS.md)** - Adding new LLM provider adapters

### Contributing

- **[Contributing Guide](docs/contributing/CONTRIBUTING.md)** - How to contribute code, tests, and documentation
- **[Adapter Development](docs/contributing/adapter-development.md)** - Creating custom adapters for ports
- **[Testing Guide](docs/contributing/testing-guide.md)** - Testing requirements and best practices

### API Documentation

Generate and view the Rust API documentation:

```bash
cargo doc --no-deps --open
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](docs/contributing/CONTRIBUTING.md) before submitting pull requests.

## License

Distributed under the MIT License. See `LICENSE` for more information.
