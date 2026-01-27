//! Paladin Execution Benchmarks
//!
//! Performance benchmarks for individual Paladin execution including:
//! - Single execution with varying complexity
//! - Multi-loop execution
//! - Different LLM latencies
//! - Stop word detection overhead

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::application::ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStreamChunk, StopReason,
};
use paladin::application::use_cases::paladin::paladin_execution_service::PaladinExecutionService;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};
use paladin::application::use_cases::paladin::error::PaladinError;
use std::sync::Arc;
use std::time::Duration;

/// Mock Paladin port for benchmarking with configurable latency
struct BenchmarkMockPort {
    latency_micros: u64,
    output_size: usize,
}

impl BenchmarkMockPort {
    fn new(latency_micros: u64, output_size: usize) -> Self {
        Self {
            latency_micros,
            output_size,
        }
    }
}

#[async_trait::async_trait]
impl PaladinPort for BenchmarkMockPort {
    async fn execute(
        &self,
        _paladin: &Paladin,
        input: &str,
    ) -> Result<PaladinResult, PaladinError> {
        // Simulate LLM latency
        if self.latency_micros > 0 {
            tokio::time::sleep(Duration::from_micros(self.latency_micros)).await;
        }

        // Generate output of specified size
        let output = if self.output_size > 0 {
            "x".repeat(self.output_size)
        } else {
            format!("Response to: {}", input)
        };

        Ok(PaladinResult {
            output,
            token_count: (self.output_size / 4) as u32, // ~4 chars per token
            execution_time_ms: (self.latency_micros as f64 / 1000.0) as u64,
            loop_count: 1,
            stop_reason: StopReason::Completed,
        })
    }

    async fn execute_stream(
        &self,
        _paladin: &Paladin,
        _input: &str,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<PaladinStreamChunk, PaladinError>>, PaladinError>
    {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok(rx)
    }

    fn validate(&self, _paladin: &Paladin) -> Result<(), PaladinError> {
        Ok(())
    }
}

fn create_benchmark_paladin(max_loops: u32, stop_words: Vec<String>) -> Paladin {
    Node::new(
        PaladinData {
            system_prompt: "You are a helpful AI assistant.".to_string(),
            name: "benchmark_paladin".to_string(),
            user_name: "benchmark".to_string(),
            model: "mock".to_string(),
            temperature: 0.7,
            max_loops,
            stop_words,
            status: PaladinStatus::Idle,
        },
        Some("benchmark".to_string()),
    )
}

/// Benchmark single Paladin execution with zero latency (pure overhead)
fn benchmark_single_execution_zero_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("paladin_single_execution");

    // Zero latency - measures pure orchestration overhead
    group.bench_function("zero_latency", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let port = Arc::new(BenchmarkMockPort::new(0, 100));
        let service = PaladinExecutionService::new(port, None, None);
        let paladin = create_benchmark_paladin(1, vec![]);

        b.iter(|| {
            runtime.block_on(async {
                service.execute(black_box(&paladin), "Test input").await
            })
        });
    });

    group.finish();
}

/// Benchmark Paladin execution with varying LLM latencies
fn benchmark_varying_latencies(c: &mut Criterion) {
    let mut group = c.benchmark_group("paladin_latency");

    // Test different simulated LLM latencies
    for latency_ms in [1, 10, 50, 100, 500].iter() {
        let latency_micros = latency_ms * 1000;
        group.bench_with_input(
            BenchmarkId::new("latency_ms", latency_ms),
            &latency_micros,
            |b, &latency| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(latency, 100));
                let service = PaladinExecutionService::new(port, None, None);
                let paladin = create_benchmark_paladin(1, vec![]);

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&paladin), "Test input").await
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark multi-loop execution
fn benchmark_multi_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("paladin_multi_loop");

    for loop_count in [1, 3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("loops", loop_count),
            loop_count,
            |b, &loops| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(100, 100)); // 100μs per call
                let service = PaladinExecutionService::new(port, None, None);
                let paladin = create_benchmark_paladin(loops, vec![]);

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&paladin), "Test input").await
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark output size handling
fn benchmark_output_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("paladin_output_size");

    // Test different output sizes (characters)
    for size in [100, 1000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("chars", size),
            size,
            |b, &output_size| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(0, output_size));
                let service = PaladinExecutionService::new(port, None, None);
                let paladin = create_benchmark_paladin(1, vec![]);

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&paladin), "Test input").await
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark stop word detection overhead
fn benchmark_stop_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("paladin_stop_words");

    // Benchmark with no stop words
    group.bench_function("no_stop_words", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let port = Arc::new(BenchmarkMockPort::new(0, 1000));
        let service = PaladinExecutionService::new(port, None, None);
        let paladin = create_benchmark_paladin(1, vec![]);

        b.iter(|| {
            runtime.block_on(async {
                service.execute(black_box(&paladin), "Test input").await
            })
        });
    });

    // Benchmark with 10 stop words
    group.bench_function("10_stop_words", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let port = Arc::new(BenchmarkMockPort::new(0, 1000));
        let service = PaladinExecutionService::new(port, None, None);
        let stop_words: Vec<String> = (0..10)
            .map(|i| format!("STOP{}", i))
            .collect();
        let paladin = create_benchmark_paladin(1, stop_words);

        b.iter(|| {
            runtime.block_on(async {
                service.execute(black_box(&paladin), "Test input").await
            })
        });
    });

    // Benchmark with 100 stop words
    group.bench_function("100_stop_words", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let port = Arc::new(BenchmarkMockPort::new(0, 1000));
        let service = PaladinExecutionService::new(port, None, None);
        let stop_words: Vec<String> = (0..100)
            .map(|i| format!("STOP{}", i))
            .collect();
        let paladin = create_benchmark_paladin(1, stop_words);

        b.iter(|| {
            runtime.block_on(async {
                service.execute(black_box(&paladin), "Test input").await
            })
        });
    });

    group.finish();
}

/// Benchmark concurrent Paladin executions
fn benchmark_concurrent_executions(c: &mut Criterion) {
    let mut group = c.benchmark_group("paladin_concurrent");

    for concurrency in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent", concurrency),
            concurrency,
            |b, &concurrent_count| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(1000, 100)); // 1ms per call
                let service = Arc::new(PaladinExecutionService::new(port, None, None));
                let paladin = Arc::new(create_benchmark_paladin(1, vec![]));

                b.iter(|| {
                    runtime.block_on(async {
                        let mut handles = vec![];
                        for _ in 0..concurrent_count {
                            let service_clone = service.clone();
                            let paladin_clone = paladin.clone();
                            handles.push(tokio::spawn(async move {
                                service_clone.execute(&paladin_clone, "Test input").await
                            }));
                        }
                        for handle in handles {
                            let _ = handle.await;
                        }
                    })
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_single_execution_zero_latency,
    benchmark_varying_latencies,
    benchmark_multi_loop,
    benchmark_output_sizes,
    benchmark_stop_words,
    benchmark_concurrent_executions
);
criterion_main!(benches);
