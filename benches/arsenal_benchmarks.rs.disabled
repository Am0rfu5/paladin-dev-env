//! Arsenal Tool Invocation Benchmarks
//!
//! Performance benchmarks for Arsenal tool operations:
//! - Tool registration and discovery
//! - Tool invocation overhead
//! - Parameter validation
//! - Result formatting
//! - Concurrent tool execution

use async_trait::async_trait;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::core::platform::container::arsenal::{Arsenal, ArsenalError, ToolDefinition, ToolParameter};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Mock tool implementation for benchmarking
#[derive(Clone)]
struct BenchmarkTool {
    name: String,
    parameter_count: usize,
    execution_delay_micros: u64,
}

impl BenchmarkTool {
    fn new(name: &str, parameter_count: usize, delay_micros: u64) -> Self {
        Self {
            name: name.to_string(),
            parameter_count,
            execution_delay_micros: delay_micros,
        }
    }

    fn definition(&self) -> ToolDefinition {
        let mut parameters = Vec::new();
        for i in 0..self.parameter_count {
            parameters.push(ToolParameter {
                name: format!("param_{}", i),
                description: format!("Parameter {}", i),
                required: true,
                param_type: "string".to_string(),
            });
        }

        ToolDefinition {
            name: self.name.clone(),
            description: format!("Benchmark tool: {}", self.name),
            parameters,
        }
    }

    async fn execute(&self, _params: &HashMap<String, Value>) -> Result<Value, ArsenalError> {
        // Simulate tool execution time
        if self.execution_delay_micros > 0 {
            tokio::time::sleep(tokio::time::Duration::from_micros(
                self.execution_delay_micros,
            ))
            .await;
        }

        Ok(json!({
            "tool": self.name,
            "result": "success"
        }))
    }
}

/// Create arsenal with specified number of tools
fn create_arsenal(tool_count: usize, params_per_tool: usize) -> Arsenal {
    let mut arsenal = Arsenal::new();

    for i in 0..tool_count {
        let tool = BenchmarkTool::new(&format!("tool_{}", i), params_per_tool, 0);
        arsenal.register_tool(tool.definition());
    }

    arsenal
}

/// Benchmark tool registration
fn benchmark_tool_registration(c: &mut Criterion) {
    let mut group = c.benchmark_group("arsenal_registration");

    // Test registering tools with different parameter counts
    for param_count in [0, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("param_count", param_count),
            param_count,
            |b, &count| {
                b.iter_batched(
                    || Arsenal::new(),
                    |mut arsenal| {
                        let tool = BenchmarkTool::new("test_tool", count, 0);
                        arsenal.register_tool(black_box(tool.definition()));
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark tool discovery/lookup
fn benchmark_tool_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("arsenal_discovery");

    // Test discovery with different arsenal sizes
    for tool_count in [10, 50, 100, 500].iter() {
        let arsenal = create_arsenal(*tool_count, 5);

        group.bench_with_input(
            BenchmarkId::new("tool_count", tool_count),
            &arsenal,
            |b, arsenal| {
                b.iter(|| {
                    // Look up middle tool
                    let tool_name = format!("tool_{}", tool_count / 2);
                    let result = arsenal.get_tool(black_box(&tool_name));
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark listing all tools
fn benchmark_list_tools(c: &mut Criterion) {
    let mut group = c.benchmark_group("arsenal_list_tools");

    for tool_count in [10, 50, 100, 500].iter() {
        let arsenal = create_arsenal(*tool_count, 5);

        group.bench_with_input(
            BenchmarkId::new("tool_count", tool_count),
            &arsenal,
            |b, arsenal| {
                b.iter(|| {
                    let tools = arsenal.list_tools();
                    black_box(tools);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark parameter validation
fn benchmark_parameter_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("arsenal_validation");

    let arsenal = create_arsenal(10, 5);
    let tool = arsenal.get_tool("tool_0").unwrap();

    // Create valid parameters
    let mut valid_params = HashMap::new();
    for i in 0..5 {
        valid_params.insert(format!("param_{}", i), json!("value"));
    }

    group.bench_function("valid_parameters", |b| {
        b.iter(|| {
            let result = arsenal.validate_parameters(
                black_box(&tool),
                black_box(&valid_params),
            );
            black_box(result);
        });
    });

    // Create invalid parameters (missing required param)
    let mut invalid_params = HashMap::new();
    for i in 0..4 {
        // Only 4 params, tool requires 5
        invalid_params.insert(format!("param_{}", i), json!("value"));
    }

    group.bench_function("invalid_parameters", |b| {
        b.iter(|| {
            let result = arsenal.validate_parameters(
                black_box(&tool),
                black_box(&invalid_params),
            );
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark tool invocation overhead (zero latency)
fn benchmark_invocation_overhead(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("arsenal_invocation_overhead");

    group.bench_function("zero_latency", |b| {
        b.to_async(&rt).iter(|| async {
            let tool = BenchmarkTool::new("test_tool", 3, 0);
            let mut params = HashMap::new();
            params.insert("param_0".to_string(), json!("value0"));
            params.insert("param_1".to_string(), json!("value1"));
            params.insert("param_2".to_string(), json!("value2"));

            let result = tool.execute(black_box(&params)).await;
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark tool invocation with varying latencies
fn benchmark_invocation_latency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("arsenal_invocation_latency");

    // Test different tool execution times
    for latency_micros in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("latency_micros", latency_micros),
            latency_micros,
            |b, &delay| {
                b.to_async(&rt).iter(|| async move {
                    let tool = BenchmarkTool::new("test_tool", 3, delay);
                    let mut params = HashMap::new();
                    params.insert("param_0".to_string(), json!("value0"));
                    params.insert("param_1".to_string(), json!("value1"));
                    params.insert("param_2".to_string(), json!("value2"));

                    let result = tool.execute(black_box(&params)).await;
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark concurrent tool invocations
fn benchmark_concurrent_invocations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("arsenal_concurrent_invocations");

    for concurrent_count in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent", concurrent_count),
            concurrent_count,
            |b, &count| {
                b.to_async(&rt).iter(|| async move {
                    let tool = BenchmarkTool::new("test_tool", 3, 1_000); // 1ms per tool
                    let mut params = HashMap::new();
                    params.insert("param_0".to_string(), json!("value0"));
                    params.insert("param_1".to_string(), json!("value1"));
                    params.insert("param_2".to_string(), json!("value2"));

                    let mut tasks = Vec::new();
                    for _ in 0..count {
                        let tool_clone = tool.clone();
                        let params_clone = params.clone();
                        tasks.push(tokio::spawn(async move {
                            tool_clone.execute(&params_clone).await
                        }));
                    }

                    let results = futures::future::join_all(tasks).await;
                    black_box(results);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark result serialization
fn benchmark_result_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("arsenal_serialization");

    // Test different result sizes
    for size in [10, 100, 1_000, 10_000].iter() {
        let result = json!({
            "data": "x".repeat(*size),
            "status": "success"
        });

        group.bench_with_input(
            BenchmarkId::new("result_size", size),
            &result,
            |b, result| {
                b.iter(|| {
                    let serialized = serde_json::to_string(black_box(result)).unwrap();
                    black_box(serialized);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark mixed operations (realistic workload)
fn benchmark_mixed_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("arsenal_mixed");

    group.bench_function("realistic_workflow", |b| {
        b.to_async(&rt).iter(|| async {
            // Create arsenal with 20 tools
            let arsenal = create_arsenal(20, 5);

            // List all tools (discovery)
            let tools = arsenal.list_tools();
            black_box(&tools);

            // Look up 3 specific tools
            for i in [5, 10, 15] {
                let tool = arsenal.get_tool(&format!("tool_{}", i));
                black_box(tool);
            }

            // Execute 3 tools concurrently
            let mut tasks = Vec::new();
            for i in [5, 10, 15] {
                let tool = BenchmarkTool::new(&format!("tool_{}", i), 5, 1_000);
                let mut params = HashMap::new();
                for j in 0..5 {
                    params.insert(format!("param_{}", j), json!("value"));
                }
                tasks.push(tokio::spawn(async move { tool.execute(&params).await }));
            }

            let results = futures::future::join_all(tasks).await;
            black_box(results);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_tool_registration,
    benchmark_tool_discovery,
    benchmark_list_tools,
    benchmark_parameter_validation,
    benchmark_invocation_overhead,
    benchmark_invocation_latency,
    benchmark_concurrent_invocations,
    benchmark_result_serialization,
    benchmark_mixed_operations
);
criterion_main!(benches);
