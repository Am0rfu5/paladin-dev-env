# Performance Baseline

## Scope
This baseline covers the active Epic 3 benchmark targets:

- `config_benchmarks` (root crate)
- `battalion_benchmarks` (`paladin-battalion`)
- `sanctum_benchmarks` (`paladin-memory`)
- `garrison_benchmarks` (`paladin-memory`)
- `llm_serialization_benchmarks` (`paladin-llm`)

Run timestamp window (UTC): `2026-05-27T22:58:29` to `2026-05-27T23:08:23`

## Environment

| Field | Value |
|---|---|
| Commit SHA | `f4156ff6360aa976d03b2bdb40775e52e1e991be` |
| OS | Debian GNU/Linux 12 (bookworm) |
| Kernel | Linux 6.8.0-111-generic |
| CPU | Intel Xeon E3-1505M v5 @ 2.80GHz |
| Cores / Threads | 4 cores / 8 threads |
| Rust | `rustc 1.95.0 (59807616e 2026-04-14)` |
| Cargo | `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| Config Profile | `APP_ENV=test` |

## Methodology

Commands executed:

```bash
APP_ENV=test cargo bench --bench config_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-battalion --bench battalion_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-memory --bench sanctum_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-memory --bench garrison_benchmarks -- --noplot
APP_ENV=test cargo bench -p paladin-llm --bench llm_serialization_benchmarks -- --noplot
```

Raw benchmark log:

- `project/Milestone_7-Production-Hardening/Epic_3/artifacts/task6-benchmark-run-postfix-20260527-225829.log`

Notes:

- Criterion ran with default warmup/sample settings unless benchmark code specifies overrides.
- Plot rendering used the plotters backend (`gnuplot` not installed).
- The config benchmark uses `APP_ENV=test` to load the schema-compatible config profile.

## Results

### Root Config Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `config/settings_new` | `1.2543 ms .. 1.4626 ms` |
| `config/domain_accessors` | `18.215 us .. 19.968 us` |

### Battalion Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `battalion/formation_3_agents` | `3.6108 us .. 3.7968 us` |
| `battalion/phalanx_5_agents` | `42.619 us .. 44.681 us` |
| `battalion/campaign_branching_dag` | `7.3903 us .. 7.7433 us` |

### Sanctum Benchmarks

Store operations:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_store_single/dimension/384` | `954.62 ns .. 1.0286 us` |
| `sanctum_store_single/dimension/768` | `1.1671 us .. 1.2927 us` |
| `sanctum_store_single/dimension/1536` | `923.90 ns .. 1.0118 us` |
| `sanctum_store_batch/batch_size/10` | `5.4577 us .. 5.8535 us` |
| `sanctum_store_batch/batch_size/50` | `27.079 us .. 28.449 us` |
| `sanctum_store_batch/batch_size/100` | `52.216 us .. 54.761 us` |
| `sanctum_store_batch/batch_size/500` | `416.83 us .. 436.68 us` |

Search scale:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_search_scale/vector_count/100` | `204.96 us .. 214.11 us` |
| `sanctum_search_scale/vector_count/1000` | `2.7224 ms .. 2.7941 ms` |
| `sanctum_search_scale/vector_count/5000` | `14.927 ms .. 15.240 ms` |
| `sanctum_search_scale/vector_count/10000` | `30.458 ms .. 31.241 ms` |

Search top-k and filters:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_search_topk/top_k/1` | `14.862 ms .. 15.252 ms` |
| `sanctum_search_topk/top_k/5` | `14.944 ms .. 15.276 ms` |
| `sanctum_search_topk/top_k/10` | `15.779 ms .. 16.710 ms` |
| `sanctum_search_topk/top_k/50` | `15.085 ms .. 15.538 ms` |
| `sanctum_search_topk/top_k/100` | `15.034 ms .. 15.586 ms` |
| `sanctum_search_filters/no_filter` | `13.899 ms .. 14.341 ms` |
| `sanctum_search_filters/filter_paladin_id` | `1.4558 ms .. 1.5001 ms` |
| `sanctum_search_filters/filter_memory_type` | `4.5904 ms .. 4.7344 ms` |
| `sanctum_search_filters/filter_importance` | `8.2067 ms .. 8.4407 ms` |
| `sanctum_search_filters/filter_combined` | `105.31 us .. 110.03 us` |

Mutation/count operations:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_update/update_single` | `3.5600 us .. 3.6261 us` |
| `sanctum_delete/delete_single` | `48.010 us .. 50.556 us` |
| `sanctum_count/count_all` | `55.712 ns .. 60.129 ns` |
| `sanctum_count/count_with_filter` | `129.76 us .. 153.33 us` |

### Garrison Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `garrison/write/100` | `14.313 us .. 15.070 us` |
| `garrison/write/1000` | `134.61 us .. 140.43 us` |
| `garrison/write/10000` | `1.4570 ms .. 1.5865 ms` |
| `garrison/read_recent/100` | `3.8229 us .. 3.8732 us` |
| `garrison/read_recent/1000` | `3.8187 us .. 3.9446 us` |
| `garrison/read_recent/10000` | `5.5296 us .. 6.0342 us` |

### LLM Serialization Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `llm/serialize_request` | `2.1024 us .. 2.1942 us` |
| `llm/deserialize_response` | `999.13 ns .. 1.1325 us` |
| `llm/response_roundtrip` | `2.1588 us .. 2.2568 us` |

## Sanctum Comparison Notes (Post-Migration vs Pre-Migration)

Comparison method:

- Searched project docs and benchmark artifacts for pre-migration sanctum timing data.
- Checked `docs/SANCTUM_BENCHMARKS.md` and found benchmark templates/targets but no populated historical timing table.
- Used the current run as the first trustworthy post-migration baseline.

Observed variance and interpretation:

- `sanctum_search_scale/vector_count/10000` measured `30.458 ms .. 31.241 ms`, which is below the documented target of `< 100 ms`.
- Intra-run spread for this key metric is approximately `2.57%` of the lower bound (`(31.241 - 30.458) / 30.458`).
- Because no trustworthy pre-migration numeric baseline was found, cross-era variance is marked as unavailable.

## Historical Data Availability

Trustworthy historical data found:

- None for pre-migration sanctum timings in repository-tracked artifacts.

Areas without prior comparable baseline:

- Sanctum pre-migration numeric benchmark times.
- Newly introduced Epic 3 benchmarks: battalion crate-local suite, garrison crate-local suite, llm serialization suite, and root config benchmarks under the current migration structure.

## Coverage Cross-Check

All active benchmark targets are represented in this report:

- `config_benchmarks`: covered
- `battalion_benchmarks`: covered
- `sanctum_benchmarks`: covered
- `garrison_benchmarks`: covered
- `llm_serialization_benchmarks`: covered
