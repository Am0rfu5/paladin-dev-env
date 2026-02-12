//! Battalion Orchestration Benchmarks
//!
//! Performance benchmarks for all Battalion patterns:
//! - Formation (Sequential)
//! - Phalanx (Concurrent)
//! - Campaign (Graph)
//! - Chain of Command (Hierarchical)
//! - Maneuver (Flow DSL)

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::application::ports::output::paladin_port::{
    PaladinPort, PaladinResult, PaladinStreamChunk, StopReason,
};
use paladin::application::use_cases::battalion::flow_visualizer::{
    FlowVisualizer, VisualizationFormat,
};
use paladin::application::use_cases::battalion::formation_service::FormationExecutionService;
use paladin::application::use_cases::battalion::maneuver_service::ManeuverExecutionService;
use paladin::application::use_cases::battalion::phalanx_service::PhalanxExecutionService;
use paladin::application::use_cases::paladin::error::PaladinError;
use paladin::core::base::entity::node::Node;
use paladin::core::platform::container::battalion::BattalionConfig;
use paladin::core::platform::container::battalion::formation::Formation;
use paladin::core::platform::container::battalion::maneuver::{Maneuver, ManeuverConfig};
use paladin::core::platform::container::battalion::parser::FlowParser;
use paladin::core::platform::container::battalion::phalanx::{AggregationStrategy, Phalanx};
use paladin::core::platform::container::paladin::{MaxLoops, Paladin, PaladinData, PaladinStatus};
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
            ..Default::default()
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
            max_loops: MaxLoops::Fixed(1),
            stop_words: vec![],
            status: PaladinStatus::Idle,
            vision_enabled: false,
            ..Default::default()
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

/*
/// Benchmark Campaign (graph-based) orchestration
/// TODO: Fix Campaign API - add_node/add_edge methods don't match actual implementation
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
*/

/*
/// Benchmark ChainOfCommand (hierarchical delegation)
/// TODO: Fix ChainOfCommand API - constructor signature doesn't match
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
*/

/// Benchmark Maneuver Flow DSL parsing
fn benchmark_maneuver_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_parsing");

    // Simple sequential flow
    group.bench_function("parse_simple_sequential", |b| {
        let flow = "agent1 -> agent2 -> agent3";
        b.iter(|| FlowParser::parse(black_box(flow)));
    });

    // Simple parallel flow
    group.bench_function("parse_simple_parallel", |b| {
        let flow = "(agent1, agent2, agent3)";
        b.iter(|| FlowParser::parse(black_box(flow)));
    });

    // Mixed nested flow
    group.bench_function("parse_nested_mixed", |b| {
        let flow = "intake -> (analyzer, summarizer) -> reviewer";
        b.iter(|| FlowParser::parse(black_box(flow)));
    });

    // Complex deeply nested flow
    group.bench_function("parse_complex_nested", |b| {
        let flow = "intake -> (technical -> (code_review, security_scan), business, legal) -> synthesis -> approval";
        b.iter(|| {
            FlowParser::parse(black_box(flow))
        });
    });

    group.finish();
}

/// Benchmark Maneuver Flow visualization
fn benchmark_maneuver_visualization(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_visualization");

    let simple_flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
    let nested_flow = FlowParser::parse("intake -> (analyzer, summarizer) -> reviewer").unwrap();
    let complex_flow = FlowParser::parse(
        "intake -> (technical -> (code_review, security_scan), business, legal) -> synthesis",
    )
    .unwrap();

    // ASCII visualization benchmarks
    group.bench_function("visualize_simple_ascii", |b| {
        b.iter(|| FlowVisualizer::visualize(black_box(&simple_flow), VisualizationFormat::Ascii));
    });

    group.bench_function("visualize_nested_ascii", |b| {
        b.iter(|| FlowVisualizer::visualize(black_box(&nested_flow), VisualizationFormat::Ascii));
    });

    group.bench_function("visualize_complex_ascii", |b| {
        b.iter(|| FlowVisualizer::visualize(black_box(&complex_flow), VisualizationFormat::Ascii));
    });

    // Mermaid visualization benchmarks
    group.bench_function("visualize_simple_mermaid", |b| {
        b.iter(|| FlowVisualizer::visualize(black_box(&simple_flow), VisualizationFormat::Mermaid));
    });

    group.bench_function("visualize_nested_mermaid", |b| {
        b.iter(|| FlowVisualizer::visualize(black_box(&nested_flow), VisualizationFormat::Mermaid));
    });

    group.bench_function("visualize_complex_mermaid", |b| {
        b.iter(|| {
            FlowVisualizer::visualize(black_box(&complex_flow), VisualizationFormat::Mermaid)
        });
    });

    group.finish();
}

/// Benchmark Maneuver validation
fn benchmark_maneuver_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_validation");

    // Create agent pool
    let mut agents = std::collections::HashMap::new();
    for i in 0..10 {
        agents.insert(
            format!("agent{}", i),
            create_benchmark_paladin(&format!("agent{}", i)),
        );
    }

    // Simple flow validation
    group.bench_function("validate_simple", |b| {
        let flow = FlowParser::parse("agent1 -> agent2 -> agent3").unwrap();
        let config = ManeuverConfig::default();

        b.iter(|| {
            Maneuver::new(
                "bench",
                black_box(agents.clone()),
                black_box(flow.clone()),
                black_box(config.clone()),
            )
        });
    });

    // Nested flow validation
    group.bench_function("validate_nested", |b| {
        let flow = FlowParser::parse("agent1 -> (agent2, agent3) -> agent4").unwrap();
        let config = ManeuverConfig::default();

        b.iter(|| {
            Maneuver::new(
                "bench",
                black_box(agents.clone()),
                black_box(flow.clone()),
                black_box(config.clone()),
            )
        });
    });

    // Complex flow validation
    group.bench_function("validate_complex", |b| {
        let flow =
            FlowParser::parse("agent1 -> (agent2 -> (agent3, agent4), agent5, agent6) -> agent7")
                .unwrap();
        let config = ManeuverConfig::default();

        b.iter(|| {
            Maneuver::new(
                "bench",
                black_box(agents.clone()),
                black_box(flow.clone()),
                black_box(config.clone()),
            )
        });
    });

    group.finish();
}

/// Benchmark Maneuver sequential execution
fn benchmark_maneuver_sequential(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_sequential");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let port = Arc::new(BenchmarkMockPort::new(0)); // Zero latency for overhead measurement
    let service = ManeuverExecutionService::new(port.clone());

    // Test with varying chain lengths
    for agent_count in [3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("zero_latency", agent_count),
            agent_count,
            |b, &count| {
                // Build flow expression: "agent0 -> agent1 -> agent2 -> ..."
                let flow_expr = (0..count)
                    .map(|i| format!("agent{}", i))
                    .collect::<Vec<_>>()
                    .join(" -> ");

                let flow = FlowParser::parse(&flow_expr).unwrap();

                let mut agents = std::collections::HashMap::new();
                for i in 0..count {
                    agents.insert(
                        format!("agent{}", i),
                        create_benchmark_paladin(&format!("agent{}", i)),
                    );
                }

                let config = ManeuverConfig::default();
                let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&maneuver), "benchmark").await
                    })
                });
            },
        );
    }

    // Test with 100μs latency
    let port_latency = Arc::new(BenchmarkMockPort::new(100));
    let service_latency = ManeuverExecutionService::new(port_latency);

    for agent_count in [3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("100us_latency", agent_count),
            agent_count,
            |b, &count| {
                let flow_expr = (0..count)
                    .map(|i| format!("agent{}", i))
                    .collect::<Vec<_>>()
                    .join(" -> ");

                let flow = FlowParser::parse(&flow_expr).unwrap();

                let mut agents = std::collections::HashMap::new();
                for i in 0..count {
                    agents.insert(
                        format!("agent{}", i),
                        create_benchmark_paladin(&format!("agent{}", i)),
                    );
                }

                let config = ManeuverConfig::default();
                let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

                b.iter(|| {
                    runtime.block_on(async {
                        service_latency
                            .execute(black_box(&maneuver), "benchmark")
                            .await
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Maneuver parallel execution
fn benchmark_maneuver_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_parallel");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let port = Arc::new(BenchmarkMockPort::new(0));
    let service = ManeuverExecutionService::new(port.clone());

    // Test with varying parallel branch counts
    for agent_count in [3, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("zero_latency", agent_count),
            agent_count,
            |b, &count| {
                // Build flow expression: "(agent0, agent1, agent2, ...)"
                let flow_expr = format!(
                    "({})",
                    (0..count)
                        .map(|i| format!("agent{}", i))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                let flow = FlowParser::parse(&flow_expr).unwrap();

                let mut agents = std::collections::HashMap::new();
                for i in 0..count {
                    agents.insert(
                        format!("agent{}", i),
                        create_benchmark_paladin(&format!("agent{}", i)),
                    );
                }

                let config = ManeuverConfig::default();
                let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

                b.iter(|| {
                    runtime.block_on(async {
                        service.execute(black_box(&maneuver), "benchmark").await
                    })
                });
            },
        );
    }

    // Test with 100μs latency to show parallel speedup
    let port_latency = Arc::new(BenchmarkMockPort::new(100));
    let service_latency = ManeuverExecutionService::new(port_latency);

    for agent_count in [3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("100us_latency", agent_count),
            agent_count,
            |b, &count| {
                let flow_expr = format!(
                    "({})",
                    (0..count)
                        .map(|i| format!("agent{}", i))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                let flow = FlowParser::parse(&flow_expr).unwrap();

                let mut agents = std::collections::HashMap::new();
                for i in 0..count {
                    agents.insert(
                        format!("agent{}", i),
                        create_benchmark_paladin(&format!("agent{}", i)),
                    );
                }

                let config = ManeuverConfig::default();
                let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

                b.iter(|| {
                    runtime.block_on(async {
                        service_latency
                            .execute(black_box(&maneuver), "benchmark")
                            .await
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Maneuver nested (mixed) execution
fn benchmark_maneuver_nested(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_nested");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let port = Arc::new(BenchmarkMockPort::new(0));
    let service = ManeuverExecutionService::new(port.clone());

    // Simple nested: "a -> (b, c) -> d"
    group.bench_function("simple_nested", |b| {
        let flow = FlowParser::parse("agent0 -> (agent1, agent2) -> agent3").unwrap();

        let mut agents = std::collections::HashMap::new();
        for i in 0..4 {
            agents.insert(
                format!("agent{}", i),
                create_benchmark_paladin(&format!("agent{}", i)),
            );
        }

        let config = ManeuverConfig::default();
        let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&maneuver), "benchmark").await })
        });
    });

    // Complex nested: "a -> (b -> (c, d), e, f) -> g"
    group.bench_function("complex_nested", |b| {
        let flow =
            FlowParser::parse("agent0 -> (agent1 -> (agent2, agent3), agent4, agent5) -> agent6")
                .unwrap();

        let mut agents = std::collections::HashMap::new();
        for i in 0..7 {
            agents.insert(
                format!("agent{}", i),
                create_benchmark_paladin(&format!("agent{}", i)),
            );
        }

        let config = ManeuverConfig::default();
        let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&maneuver), "benchmark").await })
        });
    });

    // Very complex nested (enterprise review pipeline)
    group.bench_function("enterprise_pipeline", |b| {
        let flow = FlowParser::parse(
            "intake -> (technical -> (code_review, security_scan), business, legal) -> synthesis -> approval"
        ).unwrap();

        let agent_names = vec![
            "intake", "technical", "code_review", "security_scan",
            "business", "legal", "synthesis", "approval"
        ];

        let mut agents = std::collections::HashMap::new();
        for name in agent_names {
            agents.insert(name.to_string(), create_benchmark_paladin(name));
        }

        let config = ManeuverConfig::default();
        let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

        b.iter(|| {
            runtime.block_on(async {
                service.execute(black_box(&maneuver), "benchmark").await
            })
        });
    });

    group.finish();
}

/// Benchmark Maneuver orchestration overhead vs other patterns
fn benchmark_maneuver_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("maneuver_vs_patterns");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let port = Arc::new(BenchmarkMockPort::new(0)); // Zero latency to measure pure overhead

    // Compare 3-agent sequential execution
    group.bench_function("formation_3_sequential", |b| {
        let service = FormationExecutionService::new(port.clone());
        let paladins = vec![
            create_benchmark_paladin("agent0"),
            create_benchmark_paladin("agent1"),
            create_benchmark_paladin("agent2"),
        ];
        let config = BattalionConfig::default();
        let formation = Formation::new(paladins, config).unwrap();

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&formation), "benchmark").await })
        });
    });

    group.bench_function("maneuver_3_sequential", |b| {
        let service = ManeuverExecutionService::new(port.clone());
        let flow = FlowParser::parse("agent0 -> agent1 -> agent2").unwrap();

        let mut agents = std::collections::HashMap::new();
        for i in 0..3 {
            agents.insert(
                format!("agent{}", i),
                create_benchmark_paladin(&format!("agent{}", i)),
            );
        }

        let config = ManeuverConfig::default();
        let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&maneuver), "benchmark").await })
        });
    });

    // Compare 5-agent parallel execution
    group.bench_function("phalanx_5_parallel", |b| {
        let service = PhalanxExecutionService::new(port.clone());
        let paladins = vec![
            create_benchmark_paladin("agent0"),
            create_benchmark_paladin("agent1"),
            create_benchmark_paladin("agent2"),
            create_benchmark_paladin("agent3"),
            create_benchmark_paladin("agent4"),
        ];
        let config = BattalionConfig::default();
        let phalanx = Phalanx::new(paladins, config)
            .unwrap()
            .with_aggregation(AggregationStrategy::CollectAll);

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&phalanx), "benchmark").await })
        });
    });

    group.bench_function("maneuver_5_parallel", |b| {
        let service = ManeuverExecutionService::new(port.clone());
        let flow = FlowParser::parse("(agent0, agent1, agent2, agent3, agent4)").unwrap();

        let mut agents = std::collections::HashMap::new();
        for i in 0..5 {
            agents.insert(
                format!("agent{}", i),
                create_benchmark_paladin(&format!("agent{}", i)),
            );
        }

        let config = ManeuverConfig::default();
        let maneuver = Maneuver::new("bench", agents, flow, config).unwrap();

        b.iter(|| {
            runtime.block_on(async { service.execute(black_box(&maneuver), "benchmark").await })
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
    // Maneuver benchmarks
    benchmark_maneuver_parsing,
    benchmark_maneuver_visualization,
    benchmark_maneuver_validation,
    benchmark_maneuver_sequential,
    benchmark_maneuver_parallel,
    benchmark_maneuver_nested,
    benchmark_maneuver_overhead // TODO: Fix Campaign and ChainOfCommand benchmarks - require API updates
                                // benchmark_campaign,
                                // benchmark_chain_of_command
);
criterion_main!(benches);
