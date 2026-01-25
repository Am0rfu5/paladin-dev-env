//! Battalion Orchestration Benchmarks
//!
//! Performance benchmarks for all four Battalion patterns:
//! - Formation (Sequential)
//! - Phalanx (Concurrent)
//! - Campaign (Graph)
//! - Chain of Command (Hierarchical)

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::application::ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStreamChunk, StopReason,
};
use paladin::application::use_cases::battalion::formation_service::FormationExecutionService;
use paladin::application::use_cases::battalion::phalanx_service::PhalanxExecutionService;
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::battalion::BattalionConfig;
use paladin::core::platform::container::battalion::formation::Formation;
use paladin::core::platform::container::battalion::phalanx::{AggregationStrategy, Phalanx};
use paladin::core::platform::container::paladin::{Paladin, PaladinData, PaladinStatus};
use std::sync::Arc;
use std::time::Duration;

/// Fast mock Paladin port for benchmarking (minimal latency)
struct BenchmarkMockPort {
    latency_micros: u64,
}

impl BenchmarkMockPort {
    fn new(latency_micros: u64) -> Self {
        Self { latency_micros }
    }
}

#[async_trait::async_trait]
impl PaladinPort for BenchmarkMockPort {
    async fn execute(
        &self,
        _paladin: &Paladin,
        input: &str,
    ) -> Result<PaladinResult, PaladinError> {
        // Simulate minimal processing latency
        if self.latency_micros > 0 {
            tokio::time::sleep(Duration::from_micros(self.latency_micros)).await;
        }

        Ok(PaladinResult {
            output: format!("Output: {}", input),
            token_count: 10,
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

fn create_benchmark_paladin(name: &str) -> Paladin {
    Node::new(
        PaladinData {
            system_prompt: format!("Benchmark paladin: {}", name),
            name: name.to_string(),
            user_name: "benchmark".to_string(),
            model: "mock".to_string(),
            temperature: 0.7,
            max_loops: 1,
            stop_words: vec![],
            status: PaladinStatus::Idle,
        },
        Some(name.to_string()),
    )
}

/// Benchmark Formation with varying number of Paladins
fn benchmark_formation(c: &mut Criterion) {
    let mut group = c.benchmark_group("formation");

    // Test with 0 latency to measure pure orchestration overhead
    for paladin_count in [3, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("zero_latency", paladin_count),
            paladin_count,
            |b, &count| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(0));
                let service = FormationExecutionService::new(port);

                let paladins: Vec<Paladin> = (0..count)
                    .map(|i| create_benchmark_paladin(&format!("p{}", i)))
                    .collect();

                let config = BattalionConfig::default();
                let formation = Formation::new(paladins, config).unwrap();

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&formation), "benchmark").await
                    })
                });
            },
        );
    }

    // Test with 100μs latency per Paladin (more realistic)
    for paladin_count in [3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("100us_latency", paladin_count),
            paladin_count,
            |b, &count| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(100));
                let service = FormationExecutionService::new(port);

                let paladins: Vec<Paladin> = (0..count)
                    .map(|i| create_benchmark_paladin(&format!("p{}", i)))
                    .collect();

                let config = BattalionConfig::default();
                let formation = Formation::new(paladins, config).unwrap();

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&formation), "benchmark").await
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Phalanx with varying number of concurrent Paladins
fn benchmark_phalanx(c: &mut Criterion) {
    let mut group = c.benchmark_group("phalanx");

    // Test concurrent execution with zero latency
    for paladin_count in [3, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_zero_latency", paladin_count),
            paladin_count,
            |b, &count| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(0));
                let service = PhalanxExecutionService::new(port);

                let paladins: Vec<Paladin> = (0..count)
                    .map(|i| create_benchmark_paladin(&format!("p{}", i)))
                    .collect();

                let config = BattalionConfig::default();
                let phalanx = Phalanx::new(paladins, config)
                    .unwrap()
                    .with_aggregation(AggregationStrategy::CollectAll);

                b.iter(|| {
                    runtime
                        .block_on(async { service.execute(black_box(&phalanx), "benchmark").await })
                });
            },
        );
    }

    // Test with 100μs latency - should show concurrency benefit
    for paladin_count in [3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_100us_latency", paladin_count),
            paladin_count,
            |b, &count| {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let port = Arc::new(BenchmarkMockPort::new(100));
                let service = PhalanxExecutionService::new(port);

                let paladins: Vec<Paladin> = (0..count)
                    .map(|i| create_benchmark_paladin(&format!("p{}", i)))
                    .collect();

                let config = BattalionConfig::default();
                let phalanx = Phalanx::new(paladins, config)
                    .unwrap()
                    .with_aggregation(AggregationStrategy::CollectAll);

                b.iter(|| {
                    runtime
                        .block_on(async { service.execute(black_box(&phalanx), "benchmark").await })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark different aggregation strategies
fn benchmark_aggregation_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregation_strategies");

    let strategies = vec![
        ("collect_all", AggregationStrategy::CollectAll),
        ("first_success", AggregationStrategy::FirstSuccess),
        ("majority", AggregationStrategy::Majority),
    ];

    for (name, strategy) in strategies {
        group.bench_function(name, |b| {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let port = Arc::new(BenchmarkMockPort::new(0));
            let service = PhalanxExecutionService::new(port);

            // Use 5 Paladins (minimum 3 for Majority)
            let paladins: Vec<Paladin> = (0..5)
                .map(|i| create_benchmark_paladin(&format!("p{}", i)))
                .collect();

            let config = BattalionConfig::default();
            let phalanx = Phalanx::new(paladins, config)
                .unwrap()
                .with_aggregation(strategy.clone());

            b.iter(|| {
                runtime.block_on(async { service.execute(black_box(&phalanx), "benchmark").await })
            });
        });
    }

    group.finish();
}

/// Benchmark orchestration overhead (Formation vs Phalanx with same work)
fn benchmark_orchestration_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("orchestration_overhead");

    // Formation - sequential with 5 Paladins, 0 latency
    group.bench_function("formation_5_paladins_zero_latency", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let port = Arc::new(BenchmarkMockPort::new(0));
        let service = FormationExecutionService::new(port);

        let paladins: Vec<Paladin> = (0..5)
            .map(|i| create_benchmark_paladin(&format!("p{}", i)))
            .collect();

        let config = BattalionConfig::default();
        let formation = Formation::new(paladins, config).unwrap();

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&formation), "benchmark").await })
        });
    });

    // Phalanx - concurrent with 5 Paladins, 0 latency
    group.bench_function("phalanx_5_paladins_zero_latency", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let port = Arc::new(BenchmarkMockPort::new(0));
        let service = PhalanxExecutionService::new(port);

        let paladins: Vec<Paladin> = (0..5)
            .map(|i| create_benchmark_paladin(&format!("p{}", i)))
            .collect();

        let config = BattalionConfig::default();
        let phalanx = Phalanx::new(paladins, config)
            .unwrap()
            .with_aggregation(AggregationStrategy::CollectAll);

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&phalanx), "benchmark").await })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_formation,
    benchmark_phalanx,
    benchmark_aggregation_strategies,
    benchmark_orchestration_overhead
);
criterion_main!(benches);
