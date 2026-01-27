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

/// Benchmark Campaign (graph-based) orchestration
fn benchmark_campaign(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let port = Arc::new(BenchmarkMockPort::new(10_000)); // 10ms latency
    let service = PaladinExecutionService::new(port.clone());
    let battalion_service = BattalionExecutionService::new(service);

    let mut group = c.benchmark_group("campaign");

    // Linear graph (essentially a Formation)
    group.bench_function("linear_3_nodes", |b| {
        let p1 = create_benchmark_paladin("p1");
        let p2 = create_benchmark_paladin("p2");
        let p3 = create_benchmark_paladin("p3");

        let mut campaign = Campaign::new();
        campaign.add_node("p1", p1).unwrap();
        campaign.add_node("p2", p2).unwrap();
        campaign.add_node("p3", p3).unwrap();
        campaign.add_edge("p1", "p2").unwrap();
        campaign.add_edge("p2", "p3").unwrap();
        campaign.set_entry_node("p1").unwrap();

        b.iter(|| {
            runtime.block_on(async { battalion_service.execute(black_box(&campaign), "benchmark").await })
        });
    });

    // Diamond graph (parallel + merge)
    group.bench_function("diamond_4_nodes", |b| {
        let p1 = create_benchmark_paladin("p1");
        let p2 = create_benchmark_paladin("p2");
        let p3 = create_benchmark_paladin("p3");
        let p4 = create_benchmark_paladin("p4");

        let mut campaign = Campaign::new();
        campaign.add_node("p1", p1).unwrap();
        campaign.add_node("p2", p2).unwrap();
        campaign.add_node("p3", p3).unwrap();
        campaign.add_node("p4", p4).unwrap();
        campaign.add_edge("p1", "p2").unwrap();
        campaign.add_edge("p1", "p3").unwrap();
        campaign.add_edge("p2", "p4").unwrap();
        campaign.add_edge("p3", "p4").unwrap();
        campaign.set_entry_node("p1").unwrap();

        b.iter(|| {
            runtime.block_on(async { battalion_service.execute(black_box(&campaign), "benchmark").await })
        });
    });

    // Complex graph (10 nodes, mixed topology)
    group.bench_function("complex_10_nodes", |b| {
        let mut campaign = Campaign::new();
        
        // Create 10 paladins
        for i in 0..10 {
            let p = create_benchmark_paladin(&format!("p{}", i));
            campaign.add_node(&format!("p{}", i), p).unwrap();
        }

        // Create a mixed topology:
        // p0 -> p1, p2
        // p1 -> p3, p4
        // p2 -> p5, p6
        // p3, p4, p5 -> p7
        // p6 -> p8
        // p7, p8 -> p9
        campaign.add_edge("p0", "p1").unwrap();
        campaign.add_edge("p0", "p2").unwrap();
        campaign.add_edge("p1", "p3").unwrap();
        campaign.add_edge("p1", "p4").unwrap();
        campaign.add_edge("p2", "p5").unwrap();
        campaign.add_edge("p2", "p6").unwrap();
        campaign.add_edge("p3", "p7").unwrap();
        campaign.add_edge("p4", "p7").unwrap();
        campaign.add_edge("p5", "p7").unwrap();
        campaign.add_edge("p6", "p8").unwrap();
        campaign.add_edge("p7", "p9").unwrap();
        campaign.add_edge("p8", "p9").unwrap();
        campaign.set_entry_node("p0").unwrap();

        b.iter(|| {
            runtime.block_on(async { battalion_service.execute(black_box(&campaign), "benchmark").await })
        });
    });

    group.finish();
}

/// Benchmark ChainOfCommand (hierarchical delegation)
fn benchmark_chain_of_command(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let port = Arc::new(BenchmarkMockPort::new(10_000)); // 10ms latency
    let service = PaladinExecutionService::new(port.clone());
    let battalion_service = BattalionExecutionService::new(service);

    let mut group = c.benchmark_group("chain_of_command");

    // 2-level hierarchy
    group.bench_function("2_levels_3_subordinates", |b| {
        let commander = create_benchmark_paladin("commander");
        let sub1 = create_benchmark_paladin("sub1");
        let sub2 = create_benchmark_paladin("sub2");
        let sub3 = create_benchmark_paladin("sub3");

        let chain = ChainOfCommand::new(commander, vec![sub1, sub2, sub3]);

        b.iter(|| {
            runtime.block_on(async { battalion_service.execute(black_box(&chain), "benchmark").await })
        });
    });

    // 3-level hierarchy (commander -> 2 lieutenants -> 4 soldiers)
    group.bench_function("3_levels_deep", |b| {
        let commander = create_benchmark_paladin("commander");
        
        // Lieutenant 1 with 2 soldiers
        let soldier1 = create_benchmark_paladin("soldier1");
        let soldier2 = create_benchmark_paladin("soldier2");
        let lieutenant1 = ChainOfCommand::new(
            create_benchmark_paladin("lieutenant1"),
            vec![soldier1, soldier2],
        );

        // Lieutenant 2 with 2 soldiers
        let soldier3 = create_benchmark_paladin("soldier3");
        let soldier4 = create_benchmark_paladin("soldier4");
        let lieutenant2 = ChainOfCommand::new(
            create_benchmark_paladin("lieutenant2"),
            vec![soldier3, soldier4],
        );

        // Top-level chain
        let chain = ChainOfCommand::new_with_subchains(
            commander,
            vec![Box::new(lieutenant1), Box::new(lieutenant2)],
        );

        b.iter(|| {
            runtime.block_on(async { battalion_service.execute(black_box(&chain), "benchmark").await })
        });
    });

    // Wide hierarchy (1 commander -> 10 subordinates)
    group.bench_function("wide_10_subordinates", |b| {
        let commander = create_benchmark_paladin("commander");
        let subordinates: Vec<Paladin> = (0..10)
            .map(|i| create_benchmark_paladin(&format!("sub{}", i)))
            .collect();

        let chain = ChainOfCommand::new(commander, subordinates);

        b.iter(|| {
            runtime.block_on(async { battalion_service.execute(black_box(&chain), "benchmark").await })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_formation,
    benchmark_phalanx,
    benchmark_aggregation_strategies,
    benchmark_orchestration_overhead,
    benchmark_campaign,
    benchmark_chain_of_command
);
criterion_main!(benches);
