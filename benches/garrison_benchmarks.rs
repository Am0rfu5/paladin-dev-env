//! Garrison Memory Benchmarks
//!
//! Performance benchmarks for Garrison memory operations:
//! - Add entries (single and batch)
//! - Retrieve entries
//! - Search operations
//! - Eviction strategies
//! - Token counting

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::core::platform::container::garrison::{
    ConversationHistory, ConversationRole, EvictionStrategy, GarrisonConfig, GarrisonEntry,
};

/// Create a garrison with specific configuration
fn create_garrison(max_entries: usize, eviction: EvictionStrategy) -> ConversationHistory {
    let config = GarrisonConfig::new(max_entries, None)
        .with_eviction_strategy(eviction);
    ConversationHistory::new(config)
}

/// Create a test entry with specified size
fn create_entry(role: ConversationRole, size: usize) -> GarrisonEntry {
    GarrisonEntry::new(role, "x".repeat(size))
}

/// Benchmark adding single entries
fn benchmark_add_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_add_single");

    // Test different entry sizes
    for size in [10, 100, 1000, 10_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("entry_size", size),
            size,
            |b, &entry_size| {
                b.iter_batched(
                    || create_garrison(1000, EvictionStrategy::FIFO),
                    |mut garrison| {
                        let entry = create_entry(ConversationRole::User, entry_size);
                        garrison.add(black_box(entry));
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark batch additions
fn benchmark_add_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_add_batch");

    // Test different batch sizes
    for batch_size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("batch_size", batch_size),
            batch_size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let garrison = create_garrison(1000, EvictionStrategy::FIFO);
                        let entries: Vec<GarrisonEntry> = (0..size)
                            .map(|i| {
                                let role = if i % 2 == 0 {
                                    ConversationRole::User
                                } else {
                                    ConversationRole::Assistant
                                };
                                create_entry(role, 100)
                            })
                            .collect();
                        (garrison, entries)
                    },
                    |(mut garrison, entries)| {
                        for entry in entries {
                            garrison.add(black_box(entry));
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark retrieval operations
fn benchmark_retrieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_retrieve");

    // Setup: Create garrison with entries
    let mut garrison = create_garrison(1000, EvictionStrategy::FIFO);
    for i in 0..100 {
        let role = if i % 2 == 0 {
            ConversationRole::User
        } else {
            ConversationRole::Assistant
        };
        garrison.add(create_entry(role, 100));
    }

    // Benchmark get_recent
    group.bench_function("get_last_10", |b| {
        b.iter(|| {
            let entries = garrison.get_recent(black_box(10));
            black_box(entries);
        });
    });

    group.bench_function("get_last_50", |b| {
        b.iter(|| {
            let entries = garrison.get_recent(black_box(50));
            black_box(entries);
        });
    });

    // Benchmark get_all
    group.bench_function("get_all", |b| {
        b.iter(|| {
            let entries = garrison.get_all();
            black_box(entries);
        });
    });

    group.finish();
}

/// Benchmark eviction strategies
fn benchmark_eviction_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_eviction");

    let strategies = vec![
        ("fifo", EvictionStrategy::FIFO),
        ("sliding_window", EvictionStrategy::SlidingWindow),
    ];

    for (name, strategy) in strategies {
        group.bench_function(name, |b| {
            b.iter_batched(
                || {
                    // Create garrison with capacity of 50
                    let mut garrison = create_garrison(50, strategy.clone());
                    // Fill to capacity
                    for i in 0..50 {
                        let role = if i % 2 == 0 {
                            ConversationRole::User
                        } else {
                            ConversationRole::Assistant
                        };
                        garrison.add(create_entry(role, 100));
                    }
                    garrison
                },
                |mut garrison| {
                    // Add one more to trigger eviction
                    garrison.add(black_box(create_entry(ConversationRole::User, 100)));
                    garrison
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

/// Benchmark memory pressure scenarios
fn benchmark_memory_pressure(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_memory_pressure");

    // Scenario: Rapid additions at capacity
    group.bench_function("rapid_additions_at_capacity", |b| {
        b.iter_batched(
            || create_garrison(100, EvictionStrategy::FIFO),
            |mut garrison| {
                // Add 200 entries (will cause 100 evictions)
                for i in 0..200 {
                    let role = if i % 2 == 0 {
                        ConversationRole::User
                    } else {
                        ConversationRole::Assistant
                    };
                    garrison.add(black_box(create_entry(role, 100)));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark conversation window operations
fn benchmark_windowing(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_windowing");

    // Setup: Create garrison with many entries
    let mut garrison = create_garrison(1000, EvictionStrategy::FIFO);
    for i in 0..500 {
        let role = if i % 2 == 0 {
            ConversationRole::User
        } else {
            ConversationRole::Assistant
        };
        garrison.add(create_entry(role, 200));
    }

    // Benchmark different window sizes
    for window_size in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::new("window_size", window_size),
            window_size,
            |b, &size| {
                b.iter(|| {
                    let entries = garrison.get_recent(black_box(size));
                    black_box(entries);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark clear operations
fn benchmark_clear(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_clear");

    // Test clearing different sized garrisons
    for entry_count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("entry_count", entry_count),
            entry_count,
            |b, &count| {
                b.iter_batched(
                    || {
                        let mut garrison = create_garrison(2000, EvictionStrategy::FIFO);
                        for i in 0..count {
                            let role = if i % 2 == 0 {
                                ConversationRole::User
                            } else {
                                ConversationRole::Assistant
                            };
                            garrison.add(create_entry(role, 100));
                        }
                        garrison
                    },
                    |mut garrison| {
                        garrison.clear();
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark mixed operations (realistic workload)
fn benchmark_mixed_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("garrison_mixed");

    group.bench_function("realistic_conversation", |b| {
        b.iter_batched(
            || create_garrison(200, EvictionStrategy::FIFO),
            |mut garrison| {
                // Simulate a conversation: 10 turns (20 messages)
                for _i in 0..10 {
                    // User message
                    garrison.add(black_box(create_entry(ConversationRole::User, 150)));
                    // Retrieve context (last 10 messages)
                    let _ = garrison.get_recent(10);
                    // Assistant response
                    garrison.add(black_box(create_entry(ConversationRole::Assistant, 300)));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_add_single,
    benchmark_add_batch,
    benchmark_retrieve,
    benchmark_eviction_strategies,
    benchmark_memory_pressure,
    benchmark_windowing,
    benchmark_clear,
    benchmark_mixed_operations
);
criterion_main!(benches);
