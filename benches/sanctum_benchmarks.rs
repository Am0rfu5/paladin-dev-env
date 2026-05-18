//! Sanctum Memory Benchmarks
//!
//! Performance benchmarks for Sanctum vector storage operations:
//! - Store operations (single and batch)
//! - Vector search (various scales)
//! - Filtering operations
//! - Update and delete operations
//!
//! ## Performance Targets
//! - InMemory adapter: < 100ms search at 10K vectors
//! - Qdrant adapter: < 500ms search at 100K vectors

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::core::platform::container::sanctum::{MemoryBuilder, MemoryType, SanctumEntry};
use paladin::infrastructure::adapters::sanctum::InMemorySanctum;
use paladin_ports::output::sanctum_port::{SanctumFilter, SanctumPort, SanctumQuery};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Create embedding vector of specified dimension
fn create_embedding(dimension: usize) -> Vec<f32> {
    (0..dimension).map(|i| (i as f32) * 0.01).collect()
}

/// Create a test memory entry
fn create_memory_entry(
    paladin_id: &str,
    content: &str,
    memory_type: MemoryType,
    importance: f32,
    dimension: usize,
) -> SanctumEntry {
    let memory = MemoryBuilder::new(paladin_id.to_string(), content.to_string())
        .memory_type(memory_type)
        .importance(importance)
        .build()
        .expect("Failed to build memory");

    let embedding = create_embedding(dimension);
    SanctumEntry::new(memory, embedding).expect("Failed to create SanctumEntry")
}

/// Benchmark single store operation
fn benchmark_store_single(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_store_single");

    for dimension in [384, 768, 1536].iter() {
        group.bench_with_input(
            BenchmarkId::new("dimension", dimension),
            dimension,
            |b, &dim| {
                b.to_async(&rt).iter_batched(
                    || {
                        let adapter = Arc::new(InMemorySanctum::new(10000));
                        let entry = create_memory_entry(
                            "paladin-bench-1",
                            "Test memory content for benchmarking",
                            MemoryType::Semantic,
                            0.8,
                            dim,
                        );
                        (adapter, entry)
                    },
                    |(adapter, entry)| async move {
                        adapter.store(black_box(entry)).await.unwrap();
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark batch store operations
fn benchmark_store_batch(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_store_batch");

    for batch_size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("batch_size", batch_size),
            batch_size,
            |b, &size| {
                b.to_async(&rt).iter_batched(
                    || {
                        let adapter = Arc::new(InMemorySanctum::new(10000));
                        let entries: Vec<SanctumEntry> = (0..size)
                            .map(|i| {
                                create_memory_entry(
                                    &format!("paladin-{}", i % 10),
                                    &format!("Memory content number {}", i),
                                    if i % 3 == 0 {
                                        MemoryType::Episodic
                                    } else if i % 3 == 1 {
                                        MemoryType::Semantic
                                    } else {
                                        MemoryType::Procedural
                                    },
                                    0.5 + (i % 5) as f32 * 0.1,
                                    384,
                                )
                            })
                            .collect();
                        (adapter, entries)
                    },
                    |(adapter, entries)| async move {
                        adapter.store_batch(black_box(entries)).await.unwrap();
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark vector search at various scales
fn benchmark_search_scale(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_search_scale");

    // Configure longer measurement time for accurate results
    group.sample_size(50);

    for vector_count in [100, 1_000, 5_000, 10_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("vector_count", vector_count),
            vector_count,
            |b, &count| {
                b.to_async(&rt).iter_batched(
                    || {
                        let adapter = Arc::new(InMemorySanctum::new(count + 1000));

                        // Pre-populate with vectors
                        let entries: Vec<SanctumEntry> = (0..count)
                            .map(|i| {
                                create_memory_entry(
                                    &format!("paladin-{}", i % 100),
                                    &format!("Memory content {}", i),
                                    MemoryType::Semantic,
                                    0.7,
                                    384,
                                )
                            })
                            .collect();

                        rt.block_on(async {
                            adapter.store_batch(entries).await.unwrap();
                        });

                        // Create query
                        let query_embedding = create_embedding(384);
                        let query = SanctumQuery::new(query_embedding, 10);

                        (adapter, query)
                    },
                    |(adapter, query)| async move {
                        adapter.search(black_box(query)).await.unwrap();
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark search with different top_k values
fn benchmark_search_topk(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_search_topk");

    // Pre-create adapter with 5000 vectors
    let adapter = Arc::new(InMemorySanctum::new(10000));
    let entries: Vec<SanctumEntry> = (0..5000)
        .map(|i| {
            create_memory_entry(
                &format!("paladin-{}", i % 50),
                &format!("Test content {}", i),
                MemoryType::Semantic,
                0.7,
                384,
            )
        })
        .collect();

    rt.block_on(async {
        adapter.store_batch(entries).await.unwrap();
    });

    for top_k in [1, 5, 10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("top_k", top_k), top_k, |b, &k| {
            b.to_async(&rt).iter(|| async {
                let query_embedding = create_embedding(384);
                let query = SanctumQuery::new(query_embedding, k);
                adapter.search(black_box(query)).await.unwrap();
            });
        });
    }

    group.finish();
}

/// Benchmark search with filters
fn benchmark_search_with_filters(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_search_filters");

    // Pre-create adapter with 5000 vectors
    let adapter = Arc::new(InMemorySanctum::new(10000));
    let entries: Vec<SanctumEntry> = (0..5000)
        .map(|i| {
            create_memory_entry(
                &format!("paladin-{}", i % 10), // 10 different paladins
                &format!("Test content {}", i),
                if i % 3 == 0 {
                    MemoryType::Episodic
                } else if i % 3 == 1 {
                    MemoryType::Semantic
                } else {
                    MemoryType::Procedural
                },
                0.5 + (i % 5) as f32 * 0.1,
                384,
            )
        })
        .collect();

    rt.block_on(async {
        adapter.store_batch(entries).await.unwrap();
    });

    // Benchmark: No filter
    group.bench_function("no_filter", |b| {
        b.to_async(&rt).iter(|| async {
            let query_embedding = create_embedding(384);
            let query = SanctumQuery::new(query_embedding, 10);
            adapter.search(black_box(query)).await.unwrap();
        });
    });

    // Benchmark: Filter by paladin_id
    group.bench_function("filter_paladin_id", |b| {
        b.to_async(&rt).iter(|| async {
            let query_embedding = create_embedding(384);
            let filter = SanctumFilter::new().paladin_id("paladin-5".to_string());
            let query = SanctumQuery::new(query_embedding, 10).filter(filter);
            adapter.search(black_box(query)).await.unwrap();
        });
    });

    // Benchmark: Filter by memory_type
    group.bench_function("filter_memory_type", |b| {
        b.to_async(&rt).iter(|| async {
            let query_embedding = create_embedding(384);
            let filter = SanctumFilter::new().memory_type(MemoryType::Semantic);
            let query = SanctumQuery::new(query_embedding, 10).filter(filter);
            adapter.search(black_box(query)).await.unwrap();
        });
    });

    // Benchmark: Filter by importance
    group.bench_function("filter_importance", |b| {
        b.to_async(&rt).iter(|| async {
            let query_embedding = create_embedding(384);
            let filter = SanctumFilter::new().min_importance(0.7);
            let query = SanctumQuery::new(query_embedding, 10).filter(filter);
            adapter.search(black_box(query)).await.unwrap();
        });
    });

    // Benchmark: Multiple filters combined
    group.bench_function("filter_combined", |b| {
        b.to_async(&rt).iter(|| async {
            let query_embedding = create_embedding(384);
            let filter = SanctumFilter::new()
                .paladin_id("paladin-5".to_string())
                .memory_type(MemoryType::Semantic)
                .min_importance(0.6);
            let query = SanctumQuery::new(query_embedding, 10).filter(filter);
            adapter.search(black_box(query)).await.unwrap();
        });
    });

    group.finish();
}

/// Benchmark update operations
fn benchmark_update(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_update");

    // Pre-create adapter with 1000 vectors
    let adapter = Arc::new(InMemorySanctum::new(10000));
    let entries: Vec<SanctumEntry> = (0..1000)
        .map(|i| {
            create_memory_entry(
                &format!("paladin-{}", i),
                &format!("Original content {}", i),
                MemoryType::Semantic,
                0.7,
                384,
            )
        })
        .collect();

    let entry_ids: Vec<String> = entries.iter().map(|e| e.memory.id.to_string()).collect();

    rt.block_on(async {
        adapter.store_batch(entries).await.unwrap();
    });

    group.bench_function("update_single", |b| {
        b.to_async(&rt).iter(|| {
            let adapter = adapter.clone();
            let entry_id = entry_ids[0].clone();
            async move {
                // Create updated entry with same ID
                let mut updated_memory = create_memory_entry(
                    "paladin-updated",
                    "Updated content",
                    MemoryType::Semantic,
                    0.9,
                    384,
                );
                updated_memory.memory.id = uuid::Uuid::parse_str(&entry_id).unwrap();

                adapter.update(black_box(updated_memory)).await.unwrap();
            }
        });
    });

    group.finish();
}

/// Benchmark delete operations
fn benchmark_delete(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_delete");

    group.bench_function("delete_single", |b| {
        b.to_async(&rt).iter_batched(
            || {
                let adapter = Arc::new(InMemorySanctum::new(10000));

                // Add entries
                let entries: Vec<SanctumEntry> = (0..100)
                    .map(|i| {
                        create_memory_entry(
                            &format!("paladin-{}", i),
                            &format!("Content {}", i),
                            MemoryType::Semantic,
                            0.7,
                            384,
                        )
                    })
                    .collect();

                let entry_id = entries[0].memory.id.to_string();

                rt.block_on(async {
                    adapter.store_batch(entries).await.unwrap();
                });

                (adapter, entry_id)
            },
            |(adapter, entry_id)| async move {
                adapter.delete(black_box(&entry_id)).await.unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark count operations
fn benchmark_count(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("sanctum_count");

    // Pre-create adapter with 5000 vectors
    let adapter = Arc::new(InMemorySanctum::new(10000));
    let entries: Vec<SanctumEntry> = (0..5000)
        .map(|i| {
            create_memory_entry(
                &format!("paladin-{}", i % 10),
                &format!("Content {}", i),
                if i % 2 == 0 {
                    MemoryType::Semantic
                } else {
                    MemoryType::Episodic
                },
                0.7,
                384,
            )
        })
        .collect();

    rt.block_on(async {
        adapter.store_batch(entries).await.unwrap();
    });

    // Benchmark: Count all
    group.bench_function("count_all", |b| {
        b.to_async(&rt).iter(|| async {
            adapter.count(None).await.unwrap();
        });
    });

    // Benchmark: Count with filter
    group.bench_function("count_with_filter", |b| {
        b.to_async(&rt).iter(|| async {
            let filter = SanctumFilter::new()
                .paladin_id("paladin-5".to_string())
                .memory_type(MemoryType::Semantic);
            adapter.count(Some(black_box(filter))).await.unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_store_single,
    benchmark_store_batch,
    benchmark_search_scale,
    benchmark_search_topk,
    benchmark_search_with_filters,
    benchmark_update,
    benchmark_delete,
    benchmark_count,
);

criterion_main!(benches);
