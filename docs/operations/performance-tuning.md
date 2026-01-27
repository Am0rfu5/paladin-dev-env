# Performance Tuning Guide

Comprehensive guide for optimizing Paladin performance across different workloads and deployment scenarios.

## Table of Contents

- [Performance Baselines](#performance-baselines)
- [Benchmarking](#benchmarking)
- [LLM Optimization](#llm-optimization)
- [Memory Optimization](#memory-optimization)
- [Concurrency Tuning](#concurrency-tuning)
- [Database Optimization](#database-optimization)
- [Network Optimization](#network-optimization)
- [Resource Allocation](#resource-allocation)

## Performance Baselines

### Expected Performance

| Metric | Target | Acceptable | Action Required |
|--------|--------|------------|-----------------|
| **Throughput** | ≥10 req/s | ≥5 req/s | <5 req/s |
| **P95 Latency** | <2s | <5s | >5s |
| **Memory per Paladin** | <50MB | <100MB | >100MB |
| **CPU per Paladin** | <100m | <200m | >200m |
| **Error Rate** | <0.1% | <1% | >1% |

### Benchmark Results

```bash
# Run benchmarks
cargo bench

# Example results:
Paladin execution (mock LLM): 15.2ms avg
Formation (3 Paladins): 47.8ms avg
Phalanx (3 Paladins concurrent): 18.9ms avg
Garrison add/retrieve: 82.3µs avg
Arsenal tool invocation: 125.6µs avg
```

## Benchmarking

### Running Benchmarks

```bash
# All benchmarks
cargo bench

# Specific benchmark
cargo bench paladin_execution

# With baseline comparison
cargo bench --bench paladin_benchmarks -- --save-baseline v0.1.0
cargo bench --bench paladin_benchmarks -- --baseline v0.1.0

# Generate HTML report
cargo bench --bench paladin_benchmarks -- --plotting-backend gnuplot
```

### Custom Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn paladin_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let paladin = create_test_paladin();
    
    c.bench_function("paladin execution", |b| {
        b.to_async(&rt).iter(|| async {
            let result = paladin.execute(black_box("test input")).await;
            black_box(result)
        })
    });
}

criterion_group!(benches, paladin_benchmark);
criterion_main!(benches);
```

### Load Testing

```bash
# Using Apache Bench
ab -n 1000 -c 10 -T 'application/json' \
  -p request.json \
  http://localhost:8080/api/paladin/execute

# Using k6
k6 run --vus 10 --duration 30s load-test.js
```

## LLM Optimization

### Model Selection

```yaml
# Use appropriate model for task complexity
llm:
  model_routing:
    simple_tasks:
      model: "gpt-3.5-turbo"  # 5-10x faster than GPT-4
      max_tokens: 500
    
    complex_tasks:
      model: "gpt-4"
      max_tokens: 2000
    
    classification:
      model: "gpt-3.5-turbo"  # Sufficient for most classification
      temperature: 0.1
```

### Request Batching

```rust
// Batch similar requests
pub struct LlmBatcher {
    pending: Vec<LlmRequest>,
    max_batch_size: usize,
    max_wait_time: Duration,
}

impl LlmBatcher {
    pub async fn add_request(&mut self, request: LlmRequest) -> Result<LlmResponse> {
        self.pending.push(request);
        
        if self.pending.len() >= self.max_batch_size {
            return self.flush().await;
        }
        
        // Wait for more requests or timeout
        tokio::select! {
            _ = tokio::time::sleep(self.max_wait_time) => {
                self.flush().await
            }
        }
    }
    
    async fn flush(&mut self) -> Result<Vec<LlmResponse>> {
        let batch = std::mem::take(&mut self.pending);
        self.llm_port.generate_batch(batch).await
    }
}
```

### Caching Responses

```rust
use moka::future::Cache;

pub struct CachedLlmPort {
    inner: Arc<dyn LlmPort>,
    cache: Cache<String, LlmResponse>,
}

impl CachedLlmPort {
    pub fn new(port: Arc<dyn LlmPort>, max_capacity: u64) -> Self {
        Self {
            inner: port,
            cache: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(Duration::from_secs(3600))
                .build(),
        }
    }
    
    async fn generate_cached(&self, messages: &[Message]) -> Result<LlmResponse> {
        let key = compute_cache_key(messages);
        
        if let Some(cached) = self.cache.get(&key).await {
            return Ok(cached);
        }
        
        let response = self.inner.generate(messages).await?;
        self.cache.insert(key, response.clone()).await;
        Ok(response)
    }
}
```

### Streaming for Long Responses

```rust
// Use streaming to reduce perceived latency
pub async fn execute_with_streaming(
    paladin: &Paladin,
    input: &str,
) -> Result<impl Stream<Item = String>> {
    let stream = paladin.execute_stream(input).await?;
    
    Ok(stream.map(|chunk| {
        // Process chunk immediately
        format!("Received: {}\n", chunk.content)
    }))
}
```

## Memory Optimization

### Garrison Configuration

```yaml
# Optimize memory usage
garrison:
  type: "sqlite"
  max_entries: 500        # Reduce from default 1000
  max_tokens: 4000        # Reduce from default 8000
  
  # Use sliding window for active conversations
  windowing:
    strategy: "sliding"
    window_size: 10       # Keep last 10 messages
    
  # Aggressive cleanup
  cleanup:
    enabled: true
    interval: "5m"
    max_age: "1h"
```

### Memory Pooling

```rust
use tokio::sync::RwLock;

pub struct MemoryPool<T> {
    pool: RwLock<Vec<T>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> MemoryPool<T> {
    pub async fn acquire(&self) -> T {
        let mut pool = self.pool.write().await;
        pool.pop().unwrap_or_else(|| (self.factory)())
    }
    
    pub async fn release(&self, item: T) {
        let mut pool = self.pool.write().await;
        if pool.len() < 100 {  // Max pool size
            pool.push(item);
        }
    }
}
```

### Lazy Loading

```rust
// Load garrison entries on-demand
pub struct LazyGarrison {
    session_id: Uuid,
    cache: RwLock<Option<Vec<GarrisonEntry>>>,
    repository: Arc<dyn GarrisonRepository>,
}

impl LazyGarrison {
    pub async fn get_entries(&self) -> Result<Vec<GarrisonEntry>> {
        let cache = self.cache.read().await;
        if let Some(entries) = cache.as_ref() {
            return Ok(entries.clone());
        }
        
        drop(cache);
        let entries = self.repository.load(self.session_id).await?;
        *self.cache.write().await = Some(entries.clone());
        Ok(entries)
    }
}
```

## Concurrency Tuning

### Thread Pool Configuration

```rust
use tokio::runtime::Builder;

pub fn create_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(8)              // Match CPU cores
        .max_blocking_threads(16)       // For blocking operations
        .thread_name("paladin-worker")
        .thread_stack_size(3 * 1024 * 1024)  // 3MB stack
        .build()
        .unwrap()
}
```

### Concurrency Limits

```yaml
# Control concurrent operations
paladin:
  max_concurrent_executions: 100
  
arsenal:
  max_concurrent_tools: 10
  tool_timeout: 30s
  
battalion:
  phalanx:
    max_concurrent_paladins: 5
```

### Backpressure Handling

```rust
use tokio::sync::Semaphore;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
    
    pub async fn acquire(&self) -> Result<()> {
        match self.semaphore.acquire().await {
            Ok(permit) => {
                permit.forget();  // Release on drop
                Ok(())
            }
            Err(_) => Err(Error::RateLimitExceeded),
        }
    }
}
```

## Database Optimization

### SQLite Configuration

```sql
-- Optimize SQLite for performance
PRAGMA journal_mode = WAL;           -- Write-Ahead Logging
PRAGMA synchronous = NORMAL;         -- Balance safety/speed
PRAGMA cache_size = -64000;          -- 64MB cache
PRAGMA temp_store = MEMORY;          -- In-memory temp tables
PRAGMA mmap_size = 268435456;        -- 256MB memory-mapped I/O
PRAGMA page_size = 4096;             -- Optimal page size

-- Add indexes for common queries
CREATE INDEX IF NOT EXISTS idx_garrison_session 
  ON garrison_entries(session_id, timestamp);

CREATE INDEX IF NOT EXISTS idx_garrison_search 
  ON garrison_entries(content) 
  USING gin(to_tsvector('english', content));
```

### Connection Pooling

```rust
use sqlx::sqlite::SqlitePoolOptions;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    SqlitePoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await?
}
```

### Query Optimization

```rust
// Use prepared statements
let stmt = sqlx::query!(
    "SELECT * FROM garrison_entries 
     WHERE session_id = ? AND timestamp > ?
     ORDER BY timestamp DESC
     LIMIT ?",
    session_id,
    cutoff_time,
    limit
);

// Batch inserts
let mut tx = pool.begin().await?;
for entry in entries {
    sqlx::query!(
        "INSERT INTO garrison_entries (session_id, content, timestamp) 
         VALUES (?, ?, ?)",
        entry.session_id, entry.content, entry.timestamp
    )
    .execute(&mut *tx)
    .await?;
}
tx.commit().await?;
```

## Network Optimization

### Connection Reuse

```rust
use reqwest::Client;

// Reuse HTTP client
lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();
}
```

### Compression

```yaml
# Enable response compression
server:
  compression:
    enabled: true
    level: 6              # Balance between size and CPU
    min_size: 1024        # Only compress responses > 1KB
```

### HTTP/2 and Keep-Alive

```rust
let client = reqwest::Client::builder()
    .http2_prior_knowledge()      // Use HTTP/2
    .tcp_keepalive(Duration::from_secs(60))
    .pool_max_idle_per_host(10)
    .build()?;
```

## Resource Allocation

### Kubernetes Resource Tuning

```yaml
resources:
  requests:
    cpu: "1000m"        # Guaranteed
    memory: "2Gi"
  limits:
    cpu: "4000m"        # Allow bursting
    memory: "4Gi"       # Hard limit

# Horizontal Pod Autoscaler
autoscaling:
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

### JVM-Style Tuning (for context)

```bash
# Rust doesn't need JVM tuning, but consider:

# 1. Release build optimizations
cargo build --release

# 2. Profile-guided optimization (PGO)
cargo build --profile production

# 3. Link-time optimization
[profile.release]
lto = "fat"
codegen-units = 1
```

### Monitoring Resource Usage

```rust
use sysinfo::{System, SystemExt};

pub fn log_resource_usage() {
    let mut system = System::new_all();
    system.refresh_all();
    
    info!(
        cpu_usage = system.global_cpu_info().cpu_usage(),
        memory_used = system.used_memory(),
        memory_total = system.total_memory(),
        "Resource usage"
    );
}
```

## Performance Checklist

Before production deployment:

- [ ] Run benchmarks and verify targets met
- [ ] Profile CPU and memory usage under load
- [ ] Test with expected concurrency levels
- [ ] Verify database indexes exist
- [ ] Enable connection pooling
- [ ] Configure resource limits
- [ ] Set up monitoring and alerts
- [ ] Test auto-scaling behavior
- [ ] Optimize LLM model selection
- [ ] Enable response caching where appropriate

## Next Steps

- **[Monitoring](monitoring.md)** - Set up performance monitoring
- **[Troubleshooting](troubleshooting.md)** - Debug performance issues
- **[Production Best Practices](../deployment/production-best-practices.md)** - Production readiness
