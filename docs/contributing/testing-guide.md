# Testing Guide

Comprehensive testing guide for Paladin development with TDD practices, coverage requirements, and testing patterns.

## Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Organization](#test-organization)
- [Unit Testing](#unit-testing)
- [Integration Testing](#integration-testing)
- [Functional Testing](#functional-testing)
- [Test Coverage](#test-coverage)
- [Mocking and Fixtures](#mocking-and-fixtures)
- [CI Integration](#ci-integration)

## Testing Philosophy

Paladin follows **Test-Driven Development (TDD)** with the Red-Green-Refactor cycle:

```
┌─────────────┐
│  1. RED     │  Write failing test first
│  ✗ Failing  │
└─────────────┘
       │
       ▼
┌─────────────┐
│  2. GREEN   │  Write minimal code to pass
│  ✓ Passing  │
└─────────────┘
       │
       ▼
┌─────────────┐
│ 3. REFACTOR │  Improve while keeping tests green
│  ✓ Passing  │
└─────────────┘
```

### Coverage Requirements

| Test Type | Target Coverage | Minimum Required |
|-----------|----------------|------------------|
| **Unit Tests** | ≥ 90% | ≥ 80% |
| **Integration Tests** | ≥ 80% | ≥ 70% |
| **Public APIs** | 100% | 100% (doc tests) |

## Test Organization

### Directory Structure

```
tests/
├── lib.rs                    # Test utilities and common setup
├── unit/                     # Unit tests (parallel execution)
│   ├── mod.rs
│   ├── paladin_tests.rs
│   ├── garrison_tests.rs
│   └── arsenal_tests.rs
├── integration/              # Integration tests (serial execution)
│   ├── mod.rs
│   ├── redis_queue_test.rs
│   ├── minio_storage_test.rs
│   └── llm_provider_test.rs
├── functional/               # End-to-end functional tests
│   ├── mod.rs
│   ├── content_lifecycle_test.rs
│   └── battalion_execution_test.rs
└── fixtures/                 # Test data and fixtures
    ├── config.test.yml
    └── sample_data.json
```

### Test Module Naming

```rust
// Unit tests inline with code
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_paladin_builder_validation() {
        // Test implementation
    }
}

// Integration tests in tests/ directory
// tests/integration/redis_queue_test.rs
#[tokio::test]
async fn test_redis_queue_operations() {
    // Test implementation
}
```

## Unit Testing

### Basic Unit Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_paladin_builder_creates_valid_paladin() {
        // Arrange
        let llm_port = Arc::new(MockLlmPort::new());
        let builder = PaladinBuilder::new(llm_port);
        
        // Act
        let result = builder
            .name("test-paladin")
            .system_prompt("You are a helpful assistant")
            .build();
        
        // Assert
        assert!(result.is_ok());
        let paladin = result.unwrap();
        assert_eq!(paladin.name(), "test-paladin");
    }
    
    #[test]
    fn test_paladin_builder_validates_empty_prompt() {
        // Arrange
        let llm_port = Arc::new(MockLlmPort::new());
        let builder = PaladinBuilder::new(llm_port);
        
        // Act
        let result = builder
            .name("test-paladin")
            .system_prompt("")  // Invalid: empty prompt
            .build();
        
        // Assert
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PaladinError::ConfigurationError(_)
        ));
    }
}
```

### Testing Async Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_paladin_execution() {
        // Arrange
        let mock_llm = Arc::new(MockLlmPort::with_response("Test response"));
        let paladin = create_test_paladin(mock_llm);
        
        // Act
        let result = paladin.execute("Test input").await;
        
        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.content, "Test response");
    }
}
```

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_garrison_always_respects_max_entries(
        entries in prop::collection::vec(any::<String>(), 0..1000)
    ) {
        let max_entries = 100;
        let garrison = InMemoryGarrison::new(max_entries);
        let session_id = Uuid::new_v4();
        
        // Add all entries
        for entry in entries {
            let _ = garrison.add_entry(session_id, entry);
        }
        
        // Verify max entries constraint
        let stored = garrison.get_entries(session_id, None).unwrap();
        prop_assert!(stored.len() <= max_entries);
    }
}
```

## Integration Testing

### Redis Integration Test

```rust
// tests/integration/redis_queue_test.rs

use paladin::infrastructure::adapters::queue::RedisQueueAdapter;
use testcontainers::{clients, images};

#[tokio::test]
#[serial]  // Run serially to avoid port conflicts
async fn test_redis_queue_enqueue_dequeue() {
    // Arrange: Start Redis container
    let docker = clients::Cli::default();
    let redis = docker.run(images::redis::Redis::default());
    let port = redis.get_host_port_ipv4(6379);
    
    let adapter = RedisQueueAdapter::new(&format!("redis://localhost:{}", port))
        .await
        .unwrap();
    
    // Act: Enqueue task
    let task = Task::new("test-task", serde_json::json!({"input": "test"}));
    adapter.enqueue(task.clone()).await.unwrap();
    
    // Assert: Dequeue task
    let dequeued = adapter.dequeue().await.unwrap();
    assert!(dequeued.is_some());
    assert_eq!(dequeued.unwrap().id, task.id);
}
```

### MinIO Integration Test

```rust
// tests/integration/minio_storage_test.rs

use paladin::infrastructure::adapters::file_storage::MinioAdapter;
use testcontainers::{clients, GenericImage};

#[tokio::test]
#[serial]
async fn test_minio_upload_download() {
    // Arrange: Start MinIO container
    let docker = clients::Cli::default();
    let minio = docker.run(
        GenericImage::new("minio/minio", "latest")
            .with_env_var("MINIO_ROOT_USER", "minioadmin")
            .with_env_var("MINIO_ROOT_PASSWORD", "minioadmin")
            .with_wait_for(WaitFor::message_on_stdout("API:"))
    );
    
    let adapter = MinioAdapter::new(
        "localhost:9000",
        "minioadmin",
        "minioadmin",
        "test-bucket",
    ).await.unwrap();
    
    // Act: Upload file
    let content = b"Test content";
    adapter.upload("test.txt", content).await.unwrap();
    
    // Assert: Download file
    let downloaded = adapter.download("test.txt").await.unwrap();
    assert_eq!(downloaded, content);
}
```

### LLM Provider Mock Test

```rust
// tests/integration/llm_provider_test.rs

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_openai_adapter_with_mock_server() {
    // Arrange: Start mock server
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "Mock response"
                    }
                }],
                "usage": {
                    "total_tokens": 10
                }
            })
        ))
        .mount(&mock_server)
        .await;
    
    // Act: Create adapter with mock URL
    let adapter = OpenAiAdapter::new(
        "test-key",
        &mock_server.uri(),
    );
    
    let messages = vec![Message::user("Test")];
    let response = adapter.generate(&messages, &LlmConfig::default()).await.unwrap();
    
    // Assert
    assert_eq!(response.content, "Mock response");
}
```

## Functional Testing

### End-to-End Content Lifecycle

```rust
// tests/functional/content_lifecycle_test.rs

#[tokio::test]
async fn test_complete_content_processing_flow() {
    // Arrange: Set up full application stack
    let config = ApplicationSettings::test_config();
    let app = Application::build(&config).await.unwrap();
    
    // Act: Submit content for processing
    let content = ContentItem::new("Test article", "https://example.com");
    let result = app.ingest_content(content).await.unwrap();
    
    // Assert: Verify content processed through all stages
    assert_eq!(result.status, ContentStatus::Completed);
    
    // Verify analysis results exist
    let analysis = app.get_analysis(result.id).await.unwrap();
    assert!(analysis.is_some());
    
    // Verify stored in database
    let stored = app.get_content(result.id).await.unwrap();
    assert!(stored.is_some());
}
```

### Battalion Execution Flow

```rust
// tests/functional/battalion_execution_test.rs

#[tokio::test]
async fn test_formation_sequential_execution() {
    // Arrange
    let llm_port = Arc::new(MockLlmPort::sequential_responses(vec![
        "Response 1",
        "Response 2",
        "Response 3",
    ]));
    
    let paladin1 = create_test_paladin(llm_port.clone(), "paladin-1");
    let paladin2 = create_test_paladin(llm_port.clone(), "paladin-2");
    let paladin3 = create_test_paladin(llm_port.clone(), "paladin-3");
    
    let formation = Formation::new(vec![paladin1, paladin2, paladin3]);
    
    // Act
    let result = formation.execute("Initial input").await.unwrap();
    
    // Assert
    assert_eq!(result.steps.len(), 3);
    assert_eq!(result.steps[0].output, "Response 1");
    assert_eq!(result.steps[1].output, "Response 2");
    assert_eq!(result.steps[2].output, "Response 3");
}
```

## Test Coverage

### Measuring Coverage

```bash
# Install llvm-cov
cargo install cargo-llvm-cov

# Run tests with coverage
cargo llvm-cov --html

# Open coverage report
open target/llvm-cov/html/index.html

# Generate lcov format for CI
cargo llvm-cov --lcov --output-path lcov.info
```

### Coverage Configuration

```toml
# .cargo/config.toml
[target.'cfg(all())']
rustflags = ["-C", "instrument-coverage"]

[build]
target-dir = "target/llvm-cov-target"
```

### Exclude from Coverage

```rust
// Exclude test utilities from coverage
#[cfg(not(tarpaulin_include))]
pub fn test_helper() {
    // Helper code
}
```

## Mocking and Fixtures

### Mock LLM Port

```rust
// tests/lib.rs

pub struct MockLlmPort {
    responses: Vec<String>,
    call_count: Arc<Mutex<usize>>,
}

impl MockLlmPort {
    pub fn new() -> Self {
        Self {
            responses: vec!["Mock response".into()],
            call_count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn with_response(response: impl Into<String>) -> Self {
        Self {
            responses: vec![response.into()],
            call_count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn sequential_responses(responses: Vec<impl Into<String>>) -> Self {
        Self {
            responses: responses.into_iter().map(Into::into).collect(),
            call_count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait]
impl LlmPort for MockLlmPort {
    async fn generate(
        &self,
        _messages: &[Message],
        _config: &LlmConfig,
    ) -> Result<LlmResponse, PaladinError> {
        let mut count = self.call_count.lock().unwrap();
        let index = *count % self.responses.len();
        *count += 1;
        
        Ok(LlmResponse {
            content: self.responses[index].clone(),
            model: "mock".into(),
            usage: Usage::default(),
            tool_calls: vec![],
        })
    }
    
    async fn generate_stream(
        &self,
        _messages: &[Message],
        _config: &LlmConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<LlmChunk>>>>, PaladinError> {
        unimplemented!("Stream not implemented in mock")
    }
    
    fn validate_model(&self, _model: &str) -> Result<(), PaladinError> {
        Ok(())
    }
}
```

### Test Fixtures

```rust
// tests/lib.rs

pub fn create_test_paladin(llm_port: Arc<dyn LlmPort>, name: &str) -> Paladin {
    PaladinBuilder::new(llm_port)
        .name(name)
        .system_prompt("Test system prompt")
        .model("test-model")
        .temperature(0.7)
        .max_loops(3)
        .build()
        .unwrap()
}

pub fn test_config() -> ApplicationSettings {
    ApplicationSettings {
        llm: LlmConfig {
            provider: "mock".into(),
            ..Default::default()
        },
        garrison: GarrisonConfig {
            r#type: "in_memory".into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
```

## CI Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    strategy:
      matrix:
        rust: [stable, beta]
    
    services:
      redis:
        image: redis:7
        ports:
          - 6379:6379
      
      minio:
        image: minio/minio
        env:
          MINIO_ROOT_USER: minioadmin
          MINIO_ROOT_PASSWORD: minioadmin
        ports:
          - 9000:9000
    
    steps:
      - uses: actions/checkout@v3
      
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          override: true
      
      - name: Run unit tests
        run: cargo test --lib
      
      - name: Run integration tests
        run: cargo test --test '*' -- --test-threads=1
      
      - name: Run doc tests
        run: cargo test --doc
      
      - name: Generate coverage
        run: |
          cargo install cargo-llvm-cov
          cargo llvm-cov --lcov --output-path lcov.info
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
```

### Pre-commit Hooks

```bash
# .git/hooks/pre-commit
#!/bin/bash

echo "Running tests..."
cargo test --quiet || exit 1

echo "Checking formatting..."
cargo fmt --check || exit 1

echo "Running clippy..."
cargo clippy -- -D warnings || exit 1

echo "All checks passed!"
```

## Testing Best Practices

### Do's ✅

- Write tests first (TDD)
- Use descriptive test names
- Test one thing per test
- Use arrange-act-assert pattern
- Mock external dependencies
- Test error cases
- Use property-based testing for algorithms
- Maintain high coverage

### Don'ts ❌

- Don't test implementation details
- Don't ignore failing tests
- Don't skip integration tests
- Don't hardcode test data
- Don't make tests dependent on order
- Don't test framework code
- Don't ignore performance tests

## Next Steps

- **[Adapter Development](adapter-development.md)** - Create custom adapters
- **[CONTRIBUTING](CONTRIBUTING.md)** - Contribution workflow
- **[CI/CD](../deployment/cicd.md)** - Continuous integration setup
