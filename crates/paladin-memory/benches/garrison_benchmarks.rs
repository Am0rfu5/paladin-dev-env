use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin_core::platform::container::garrison::{
    ConversationRole, GarrisonConfig, GarrisonEntry,
};
use paladin_memory::garrison::InMemoryGarrison;
use paladin_ports::output::garrison_port::GarrisonPort;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn create_entry(index: usize) -> GarrisonEntry {
    GarrisonEntry::with_token_count(
        ConversationRole::User,
        format!("benchmark-message-{}", index),
        12,
    )
}

async fn prefill_garrison(target_size: usize) -> Arc<InMemoryGarrison> {
    let garrison = Arc::new(InMemoryGarrison::new(GarrisonConfig::new(
        target_size + 100,
        None,
    )));

    for i in 0..target_size {
        garrison
            .remember(create_entry(i))
            .await
            .expect("store benchmark entry");
    }

    garrison
}

fn benchmark_garrison_write(c: &mut Criterion) {
    let rt = Runtime::new().expect("runtime");
    let mut group = c.benchmark_group("garrison/write");

    for size in [100usize, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &target| {
            b.to_async(&rt).iter_batched(
                || {
                    let garrison = Arc::new(InMemoryGarrison::new(GarrisonConfig::new(
                        target + 100,
                        None,
                    )));
                    let entries: Vec<GarrisonEntry> = (0..target).map(create_entry).collect();
                    (garrison, entries)
                },
                |(garrison, entries)| async move {
                    for entry in entries {
                        garrison.remember(black_box(entry)).await.expect("remember");
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn benchmark_garrison_read(c: &mut Criterion) {
    let rt = Runtime::new().expect("runtime");
    let mut group = c.benchmark_group("garrison/read_recent");

    for size in [100usize, 1_000, 10_000] {
        let garrison = rt.block_on(prefill_garrison(size));

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let garrison = Arc::clone(&garrison);
            b.to_async(&rt).iter(|| async {
                let _ = garrison
                    .recall_recent(black_box(50))
                    .await
                    .expect("recall recent");
            });
        });
    }

    group.finish();
}

criterion_group!(
    garrison_benches,
    benchmark_garrison_write,
    benchmark_garrison_read
);
criterion_main!(garrison_benches);
