//! Herald Output Formatting Benchmarks
//!
//! Performance benchmarks for Herald formatters to ensure minimal overhead:
//! - JSON Herald: Target < 1ms for 10KB results
//! - Markdown Herald: Target < 2ms for 10KB results
//! - Table Herald: Target < 2ms for 10KB results

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use paladin::core::platform::container::herald::{
    BattalionResult, ExecutionMetadata, Herald, PaladinResult, StreamChunk,
};
use paladin::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};

/// Generate a Paladin result with specified output size
fn generate_paladin_result(output_size_kb: usize) -> PaladinResult {
    let output_size_bytes = output_size_kb * 1024;
    let repeated_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let repetitions = output_size_bytes / repeated_text.len();
    let output = repeated_text.repeat(repetitions);

    use paladin::application::ports::output::paladin_port::StopReason;
    PaladinResult {
        output,
        token_count: 100,
        execution_time_ms: 1000,
        loop_count: 1,
        stop_reason: StopReason::Completed,
        ..Default::default()
    }
}

/// Generate a Battalion result with specified number of Paladins and output size per Paladin
fn generate_battalion_result(paladin_count: usize, output_size_per_kb: usize) -> BattalionResult {
    use chrono::Utc;
    use paladin::core::platform::container::battalion::BattalionStatus;
    use paladin::core::platform::container::battalion::BattalionStrategy;
    use uuid::Uuid;

    let paladin_results: Vec<PaladinResult> = (0..paladin_count)
        .map(|_i| generate_paladin_result(output_size_per_kb))
        .collect();

    let per_paladin_times: std::collections::HashMap<String, u64> = (0..paladin_count)
        .map(|i| (format!("paladin_{}", i), 1000u64))
        .collect();

    BattalionResult {
        battalion_id: Uuid::new_v4(),
        battalion_name: "BenchmarkBattalion".to_string(),
        started_at: Utc::now(),
        completed_at: Utc::now(),
        final_output: "Benchmark output".to_string(),
        paladin_results,
        status: BattalionStatus::Completed,
        strategy_used: BattalionStrategy::Formation,
        strategy_selection_reasoning: None,
        strategy_selection_time_ms: 0,
        per_paladin_times,
        per_paladin_tokens: std::collections::HashMap::new(),
        total_tokens: 0,
        paladin_success_count: paladin_count,
        paladin_failure_count: 0,
    }
}

/// Benchmark JSON Herald with varying result sizes
fn benchmark_json_herald(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_herald");
    let herald = JsonHerald::new();

    // Benchmark single Paladin results
    for size_kb in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("paladin_result", size_kb),
            size_kb,
            |b, &kb| {
                let result = generate_paladin_result(kb);
                b.iter(|| black_box(herald.format_paladin_result(black_box(&result)).unwrap()));
            },
        );
    }

    // Benchmark Battalion results
    for paladin_count in [2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("battalion_result_2kb", paladin_count),
            paladin_count,
            |b, &count| {
                let result = generate_battalion_result(count, 2);
                b.iter(|| black_box(herald.format_battalion_result(black_box(&result)).unwrap()));
            },
        );
    }

    // Benchmark streaming chunks
    group.bench_function("stream_chunk", |b| {
        use chrono::Utc;
        use uuid::Uuid;
        let chunk = StreamChunk::builder()
            .chunk_id(Uuid::new_v4())
            .sequence_number(0)
            .timestamp(Utc::now())
            .content("Streaming chunk content here".to_string())
            .token_count(5)
            .is_final(false)
            .build()
            .unwrap();
        b.iter(|| black_box(herald.format_stream_chunk(black_box(&chunk)).unwrap()));
    });

    // Benchmark finalize stream
    group.bench_function("finalize_stream", |b| {
        use chrono::Utc;
        use paladin::application::ports::output::llm_port::TokenUsage;
        use uuid::Uuid;
        let metadata = ExecutionMetadata::builder()
            .execution_id(Uuid::new_v4())
            .start_time(Utc::now())
            .end_time(Utc::now())
            .duration_ms(1000)
            .model_used("gpt-4".to_string())
            .token_usage(TokenUsage {
                prompt_tokens: 250,
                completion_tokens: 250,
                total_tokens: 500,
            })
            .build()
            .unwrap();
        b.iter(|| black_box(herald.finalize_stream(black_box(&metadata)).unwrap()));
    });

    group.finish();
}

/// Benchmark Markdown Herald with varying result sizes
fn benchmark_markdown_herald(c: &mut Criterion) {
    let mut group = c.benchmark_group("markdown_herald");
    let herald = MarkdownHerald::new();

    // Benchmark single Paladin results
    for size_kb in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("paladin_result", size_kb),
            size_kb,
            |b, &kb| {
                let result = generate_paladin_result(kb);
                b.iter(|| black_box(herald.format_paladin_result(black_box(&result)).unwrap()));
            },
        );
    }

    // Benchmark Battalion results
    for paladin_count in [2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("battalion_result_2kb", paladin_count),
            paladin_count,
            |b, &count| {
                let result = generate_battalion_result(count, 2);
                b.iter(|| black_box(herald.format_battalion_result(black_box(&result)).unwrap()));
            },
        );
    }

    // Benchmark streaming chunks
    group.bench_function("stream_chunk", |b| {
        use chrono::Utc;
        use uuid::Uuid;
        let chunk = StreamChunk::builder()
            .chunk_id(Uuid::new_v4())
            .sequence_number(0)
            .timestamp(Utc::now())
            .content("Streaming chunk content here".to_string())
            .token_count(5)
            .is_final(false)
            .build()
            .unwrap();
        b.iter(|| black_box(herald.format_stream_chunk(black_box(&chunk)).unwrap()));
    });

    // Benchmark finalize stream
    group.bench_function("finalize_stream", |b| {
        use chrono::Utc;
        use paladin::application::ports::output::llm_port::TokenUsage;
        use uuid::Uuid;
        let metadata = ExecutionMetadata::builder()
            .execution_id(Uuid::new_v4())
            .start_time(Utc::now())
            .end_time(Utc::now())
            .duration_ms(1000)
            .model_used("gpt-4".to_string())
            .token_usage(TokenUsage {
                prompt_tokens: 250,
                completion_tokens: 250,
                total_tokens: 500,
            })
            .build()
            .unwrap();
        b.iter(|| black_box(herald.finalize_stream(black_box(&metadata)).unwrap()));
    });

    group.finish();
}

/// Benchmark Table Herald with varying result sizes
fn benchmark_table_herald(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_herald");
    let herald = TableHerald::default();

    // Benchmark single Paladin results
    for size_kb in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("paladin_result", size_kb),
            size_kb,
            |b, &kb| {
                let result = generate_paladin_result(kb);
                b.iter(|| black_box(herald.format_paladin_result(black_box(&result)).unwrap()));
            },
        );
    }

    // Benchmark Battalion results
    for paladin_count in [2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("battalion_result_2kb", paladin_count),
            paladin_count,
            |b, &count| {
                let result = generate_battalion_result(count, 2);
                b.iter(|| black_box(herald.format_battalion_result(black_box(&result)).unwrap()));
            },
        );
    }

    // Benchmark streaming (Table buffers, so chunks return None)
    group.bench_function("stream_chunk_buffered", |b| {
        use chrono::Utc;
        use uuid::Uuid;
        let chunk = StreamChunk::builder()
            .chunk_id(Uuid::new_v4())
            .sequence_number(0)
            .timestamp(Utc::now())
            .content("Streaming chunk content here".to_string())
            .token_count(5)
            .is_final(false)
            .build()
            .unwrap();
        b.iter(|| black_box(herald.format_stream_chunk(black_box(&chunk))));
    });

    // Benchmark finalize stream (Table renders here)
    group.bench_function("finalize_stream_with_chunks", |b| {
        use chrono::Utc;
        use paladin::application::ports::output::llm_port::TokenUsage;
        use uuid::Uuid;
        let metadata = ExecutionMetadata::builder()
            .execution_id(Uuid::new_v4())
            .start_time(Utc::now())
            .end_time(Utc::now())
            .duration_ms(1000)
            .model_used("gpt-4".to_string())
            .token_usage(TokenUsage {
                prompt_tokens: 250,
                completion_tokens: 250,
                total_tokens: 500,
            })
            .build()
            .unwrap();
        b.iter(|| black_box(herald.finalize_stream(black_box(&metadata)).unwrap()));
    });

    group.finish();
}

/// Benchmark formatter comparison with fixed 10KB result
fn benchmark_formatter_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatter_comparison_10kb");

    let json_herald = JsonHerald::new();
    let markdown_herald = MarkdownHerald::new();
    let table_herald = TableHerald::default();

    let result = generate_paladin_result(10);

    group.bench_function("json", |b| {
        b.iter(|| {
            black_box(
                json_herald
                    .format_paladin_result(black_box(&result))
                    .unwrap(),
            )
        });
    });

    group.bench_function("markdown", |b| {
        b.iter(|| {
            black_box(
                markdown_herald
                    .format_paladin_result(black_box(&result))
                    .unwrap(),
            )
        });
    });

    group.bench_function("table", |b| {
        b.iter(|| {
            black_box(
                table_herald
                    .format_paladin_result(black_box(&result))
                    .unwrap(),
            )
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_json_herald,
    benchmark_markdown_herald,
    benchmark_table_herald,
    benchmark_formatter_comparison
);
criterion_main!(benches);
