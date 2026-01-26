# Paladin

## General Overview

Paladin is a Rust-based, modular content processing platform built with Hexagonal Architecture principles. It provides configurable pipelines to ingest, normalize, analyze, enrich, store, and deliver diverse content types (web pages, documents, feeds, and streams) at scale.

Key capabilities include configurable content ingestion, NLP/LLM analysis and summarization, metadata extraction and subject tagging, content filtering and lifecycle management, and flexible delivery channels (HTTP/webhooks, email, push, and queue-based adapters). Clear ports-and-adapters boundaries make integrations, testing, and replacement of components straightforward. Paladin is designed for content aggregation, monitoring, enrichment, and automated delivery workflows where extensibility, observability, and reliable storage across SQL/NoSQL/file backends are important.

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
* **Paladin Agents:** Autonomous AI agents with memory and context management.
* **Garrison Memory System:** Persistent conversation history with windowing and search capabilities.
* **Arsenal Tool System:** External tool integration via Model Context Protocol (MCP).
* **Battalion Orchestration:** Multi-agent coordination with four orchestration patterns.

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
* **Circuit Breaker**: Fault tolerance with automatic retry and backoff
* **Execution Service**: Orchestrates agent execution with memory integration

See [docs/GARRISON.md](docs/GARRISON.md) for detailed memory system documentation.
See [docs/ARSENAL.md](docs/ARSENAL.md) for comprehensive tool system documentation.

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

### Battalion Orchestration System

Battalion provides powerful multi-agent coordination capabilities with four distinct orchestration patterns:

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

**Performance**: Handles 100+ concurrent Battalions with <10ms orchestration overhead

**Error Resilience**: Three strategies (FailFast, ContinueOnError, RetryThenContinue) with exponential backoff

**Testing**: 218 comprehensive tests (85 unit + 133 integration) ensuring reliability

See [docs/BATTALION.md](docs/BATTALION.md) for comprehensive orchestration documentation.

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

## Contributing

Contributions are welcome! Please open issues and submit pull requests for new features, enhancements, or bug fixes.

## License

Distributed under the MIT License. See `LICENSE` for more information.
