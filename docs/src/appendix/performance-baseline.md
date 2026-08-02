# Performance Baseline

## Run — 2026-08-02

Every figure in this section is this host's baseline, measured under the environment stated
below — it is explicitly **not** a portable performance claim and **not** a cross-machine
regression signal against the 2026-05-27 run recorded further down this document, since the two
runs were captured on different hardware. Throughput and latency figures in this section come
from `criterion`; memory-per-Paladin and startup time come from a separate purpose-built harness
(`examples/muster_baseline.rs`), named explicitly in their own subsections below, since
`criterion` produces neither of those two metric families.

### Scope

This run covers the same active bench targets as the 2026-05-27 run:

- `config_benchmarks` (root crate)
- `battalion_benchmarks` (`paladin-battalion`)
- `sanctum_benchmarks` (`paladin-memory`)
- `garrison_benchmarks` (`paladin-memory`)
- `llm_serialization_benchmarks` (`paladin-llm`)

Run timestamp window (UTC): `2026-08-02T15:55:18` to `2026-08-02T16:16:50`

### Environment

| Field | Value |
|---|---|
| Commit SHA | `d20f1263585b7541adc38ca08eaf3fb9ee8e3eed` |
| OS | Debian GNU/Linux 12 (bookworm) |
| Kernel | Linux 6.8.0-136-generic |
| CPU | Intel(R) Xeon(R) CPU E3-1505M v5 @ 2.80GHz |
| Cores / Threads | 4 cores / 8 threads |
| Rust | `rustc 1.97.1 (8bab26f4f 2026-07-14)` |
| Cargo | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| Config Profile | `APP_ENV=test` |

Raw provenance commands, run immediately before this run's first benchmark:

```console
$ git rev-parse HEAD
d20f1263585b7541adc38ca08eaf3fb9ee8e3eed

$ cat /etc/os-release | grep -i PRETTY_NAME
PRETTY_NAME="Debian GNU/Linux 12 (bookworm)"

$ uname -r
6.8.0-136-generic

$ grep 'model name' /proc/cpuinfo | head -1
model name	: Intel(R) Xeon(R) CPU E3-1505M v5 @ 2.80GHz

$ nproc
8

$ lscpu | grep -i "^Core(s) per socket\|^Socket(s)\|^Thread(s) per core"
Thread(s) per core:                      2
Core(s) per socket:                      4
Socket(s):                               1

$ rustc -vV
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-unknown-linux-gnu
release: 1.97.1
LLVM version: 22.1.6

$ cargo --version
cargo 1.97.1 (c980f4866 2026-06-30)

$ date -u
Sun Aug  2 15:55:18 UTC 2026
```

**Sandbox constraints, stated plainly:** no Docker in this environment; crates.io returns HTTP
403 (every command below carries `--offline`); `cargo-llvm-cov` is not installable here. None of
these block the five bench targets — `criterion 0.5.1` is already vendored in the local cargo
registry.

### Methodology

Commands executed, each with `--offline` added to the 2026-05-27 run's flag set (`APP_ENV=test`,
`-- --noplot`) so the methodology stays comparable even though the figures are not:

```bash
APP_ENV=test cargo bench --offline --bench config_benchmarks -- --noplot
APP_ENV=test cargo bench --offline -p paladin-battalion --bench battalion_benchmarks -- --noplot
APP_ENV=test cargo bench --offline -p paladin-memory --bench sanctum_benchmarks -- --noplot
APP_ENV=test cargo bench --offline -p paladin-memory --bench garrison_benchmarks -- --noplot
APP_ENV=test cargo bench --offline -p paladin-llm --bench llm_serialization_benchmarks -- --noplot
```

Each command was run to completion, sequentially and never concurrently with another `cargo
bench` invocation, so that no build or measurement activity from one target could contaminate
another target's timing. The `cargo` build-progress output (`Compiling <crate> vX.Y.Z`) is
omitted below for brevity — this was a cold `bench`-profile build; the compile line counts were
363 (config), 3 (battalion), 16 (sanctum), 1 (garrison) and 31 (llm) lines respectively, mostly
shared across targets after the first cold build. What follows is each target's `criterion`
stdout, pasted verbatim from `Running benches/...` onward.

### Results

Where a target reports throughput, it is noted in prose beneath its table; none of these five
targets configure `criterion`'s `Throughput` API, so none report a throughput column —
`not reported by this target` applies uniformly here, matching the 2026-05-27 run.

#### Root Config Benchmarks

```
     Running benches/config_benchmarks.rs (target/release/deps/config_benchmarks-4a3a86920e32946e)
Gnuplot not found, using plotters backend
Benchmarking config/settings_new
Benchmarking config/settings_new: Warming up for 3.0000 s
Benchmarking config/settings_new: Collecting 100 samples in estimated 8.6646 s (10k iterations)
Benchmarking config/settings_new: Analyzing
config/settings_new     time:   [845.85 µs 867.75 µs 892.14 µs]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

Benchmarking config/domain_accessors
Benchmarking config/domain_accessors: Warming up for 3.0000 s
Benchmarking config/domain_accessors: Collecting 100 samples in estimated 5.0110 s (338k iterations)
Benchmarking config/domain_accessors: Analyzing
config/domain_accessors time:   [14.801 µs 15.205 µs 15.621 µs]
```

| Benchmark | Time (lower .. upper) |
|---|---|
| `config/settings_new` | `845.85 µs .. 892.14 µs` |
| `config/domain_accessors` | `14.801 µs .. 15.621 µs` |

Throughput: not reported by this target.

#### Battalion Benchmarks

```
     Running benches/battalion_benchmarks.rs (target/release/deps/battalion_benchmarks-563e8b63e68221be)
Gnuplot not found, using plotters backend
Benchmarking battalion/formation_3_agents
Benchmarking battalion/formation_3_agents: Warming up for 3.0000 s
Benchmarking battalion/formation_3_agents: Collecting 100 samples in estimated 5.0093 s (1.5M iterations)
Benchmarking battalion/formation_3_agents: Analyzing
battalion/formation_3_agents
                        time:   [3.2486 µs 3.2976 µs 3.3513 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

Benchmarking battalion/phalanx_5_agents
Benchmarking battalion/phalanx_5_agents: Warming up for 3.0000 s
Benchmarking battalion/phalanx_5_agents: Collecting 100 samples in estimated 5.0686 s (162k iterations)
Benchmarking battalion/phalanx_5_agents: Analyzing
battalion/phalanx_5_agents
                        time:   [30.310 µs 30.917 µs 31.598 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking battalion/campaign_branching_dag
Benchmarking battalion/campaign_branching_dag: Warming up for 3.0000 s
Benchmarking battalion/campaign_branching_dag: Collecting 100 samples in estimated 5.0077 s (853k iterations)
Benchmarking battalion/campaign_branching_dag: Analyzing
battalion/campaign_branching_dag
                        time:   [5.8464 µs 5.9567 µs 6.0727 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

| Benchmark | Time (lower .. upper) |
|---|---|
| `battalion/formation_3_agents` | `3.2486 µs .. 3.3513 µs` |
| `battalion/phalanx_5_agents` | `30.310 µs .. 31.598 µs` |
| `battalion/campaign_branching_dag` | `5.8464 µs .. 6.0727 µs` |

Throughput: not reported by this target.

#### Sanctum Benchmarks

```
     Running benches/sanctum_benchmarks.rs (target/release/deps/sanctum_benchmarks-0c016aceeba83630)
Gnuplot not found, using plotters backend
Benchmarking sanctum_store_single/dimension/384
Benchmarking sanctum_store_single/dimension/384: Warming up for 3.0000 s
Benchmarking sanctum_store_single/dimension/384: Collecting 100 samples in estimated 5.0150 s (1.5M iterations)
Benchmarking sanctum_store_single/dimension/384: Analyzing
sanctum_store_single/dimension/384
                        time:   [670.51 ns 689.39 ns 708.55 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
Benchmarking sanctum_store_single/dimension/768
Benchmarking sanctum_store_single/dimension/768: Warming up for 3.0000 s
Benchmarking sanctum_store_single/dimension/768: Collecting 100 samples in estimated 5.0003 s (1.2M iterations)
Benchmarking sanctum_store_single/dimension/768: Analyzing
sanctum_store_single/dimension/768
                        time:   [764.90 ns 809.12 ns 854.27 ns]
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe
Benchmarking sanctum_store_single/dimension/1536
Benchmarking sanctum_store_single/dimension/1536: Warming up for 3.0000 s
Benchmarking sanctum_store_single/dimension/1536: Collecting 100 samples in estimated 5.0001 s (975k iterations)
Benchmarking sanctum_store_single/dimension/1536: Analyzing
sanctum_store_single/dimension/1536
                        time:   [643.57 ns 664.22 ns 686.72 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking sanctum_store_batch/batch_size/10
Benchmarking sanctum_store_batch/batch_size/10: Warming up for 3.0000 s
Benchmarking sanctum_store_batch/batch_size/10: Collecting 100 samples in estimated 5.0337 s (172k iterations)
Benchmarking sanctum_store_batch/batch_size/10: Analyzing
sanctum_store_batch/batch_size/10
                        time:   [4.2926 µs 4.4034 µs 4.5311 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking sanctum_store_batch/batch_size/50
Benchmarking sanctum_store_batch/batch_size/50: Warming up for 3.0000 s
Benchmarking sanctum_store_batch/batch_size/50: Collecting 100 samples in estimated 5.1515 s (35k iterations)
Benchmarking sanctum_store_batch/batch_size/50: Analyzing
sanctum_store_batch/batch_size/50
                        time:   [21.046 µs 21.473 µs 21.913 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
Benchmarking sanctum_store_batch/batch_size/100
Benchmarking sanctum_store_batch/batch_size/100: Warming up for 3.0000 s
Benchmarking sanctum_store_batch/batch_size/100: Collecting 100 samples in estimated 5.8053 s (20k iterations)
Benchmarking sanctum_store_batch/batch_size/100: Analyzing
sanctum_store_batch/batch_size/100
                        time:   [46.128 µs 47.765 µs 49.694 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
Benchmarking sanctum_store_batch/batch_size/500
Benchmarking sanctum_store_batch/batch_size/500: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.9s, enable flat sampling, or reduce sample count to 50.
Benchmarking sanctum_store_batch/batch_size/500: Collecting 100 samples in estimated 7.9382 s (5050 iterations)
Benchmarking sanctum_store_batch/batch_size/500: Analyzing
sanctum_store_batch/batch_size/500
                        time:   [338.36 µs 344.54 µs 351.48 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

Benchmarking sanctum_search_scale/vector_count/100
Benchmarking sanctum_search_scale/vector_count/100: Warming up for 3.0000 s
Benchmarking sanctum_search_scale/vector_count/100: Collecting 50 samples in estimated 5.2344 s (27k iterations)
Benchmarking sanctum_search_scale/vector_count/100: Analyzing
sanctum_search_scale/vector_count/100
                        time:   [195.67 µs 197.54 µs 199.69 µs]
Found 2 outliers among 50 measurements (4.00%)
  2 (4.00%) high mild
Benchmarking sanctum_search_scale/vector_count/1000
Benchmarking sanctum_search_scale/vector_count/1000: Warming up for 3.0000 s
Benchmarking sanctum_search_scale/vector_count/1000: Collecting 50 samples in estimated 5.3749 s (2550 iterations)
Benchmarking sanctum_search_scale/vector_count/1000: Analyzing
sanctum_search_scale/vector_count/1000
                        time:   [2.1077 ms 2.1795 ms 2.2588 ms]
Found 5 outliers among 50 measurements (10.00%)
  2 (4.00%) high mild
  3 (6.00%) high severe
Benchmarking sanctum_search_scale/vector_count/5000
Benchmarking sanctum_search_scale/vector_count/5000: Warming up for 3.0000 s
Benchmarking sanctum_search_scale/vector_count/5000: Collecting 50 samples in estimated 5.1949 s (450 iterations)
Benchmarking sanctum_search_scale/vector_count/5000: Analyzing
sanctum_search_scale/vector_count/5000
                        time:   [11.400 ms 11.518 ms 11.650 ms]
Found 2 outliers among 50 measurements (4.00%)
  1 (2.00%) high mild
  1 (2.00%) high severe
Benchmarking sanctum_search_scale/vector_count/10000
Benchmarking sanctum_search_scale/vector_count/10000: Warming up for 3.0000 s
Benchmarking sanctum_search_scale/vector_count/10000: Collecting 50 samples in estimated 5.8070 s (250 iterations)
Benchmarking sanctum_search_scale/vector_count/10000: Analyzing
sanctum_search_scale/vector_count/10000
                        time:   [23.298 ms 23.568 ms 23.860 ms]
Found 2 outliers among 50 measurements (4.00%)
  2 (4.00%) high mild

Benchmarking sanctum_search_topk/top_k/1
Benchmarking sanctum_search_topk/top_k/1: Warming up for 3.0000 s
Benchmarking sanctum_search_topk/top_k/1: Collecting 100 samples in estimated 5.7739 s (500 iterations)
Benchmarking sanctum_search_topk/top_k/1: Analyzing
sanctum_search_topk/top_k/1
                        time:   [11.530 ms 11.608 ms 11.690 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking sanctum_search_topk/top_k/5
Benchmarking sanctum_search_topk/top_k/5: Warming up for 3.0000 s
Benchmarking sanctum_search_topk/top_k/5: Collecting 100 samples in estimated 5.7302 s (500 iterations)
Benchmarking sanctum_search_topk/top_k/5: Analyzing
sanctum_search_topk/top_k/5
                        time:   [11.623 ms 11.693 ms 11.765 ms]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
Benchmarking sanctum_search_topk/top_k/10
Benchmarking sanctum_search_topk/top_k/10: Warming up for 3.0000 s
Benchmarking sanctum_search_topk/top_k/10: Collecting 100 samples in estimated 5.8072 s (500 iterations)
Benchmarking sanctum_search_topk/top_k/10: Analyzing
sanctum_search_topk/top_k/10
                        time:   [11.556 ms 11.660 ms 11.769 ms]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
Benchmarking sanctum_search_topk/top_k/50
Benchmarking sanctum_search_topk/top_k/50: Warming up for 3.0000 s
Benchmarking sanctum_search_topk/top_k/50: Collecting 100 samples in estimated 5.7895 s (500 iterations)
Benchmarking sanctum_search_topk/top_k/50: Analyzing
sanctum_search_topk/top_k/50
                        time:   [11.493 ms 11.615 ms 11.748 ms]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking sanctum_search_topk/top_k/100
Benchmarking sanctum_search_topk/top_k/100: Warming up for 3.0000 s
Benchmarking sanctum_search_topk/top_k/100: Collecting 100 samples in estimated 5.7531 s (500 iterations)
Benchmarking sanctum_search_topk/top_k/100: Analyzing
sanctum_search_topk/top_k/100
                        time:   [11.512 ms 11.630 ms 11.757 ms]
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe

Benchmarking sanctum_search_filters/no_filter
Benchmarking sanctum_search_filters/no_filter: Warming up for 3.0000 s
Benchmarking sanctum_search_filters/no_filter: Collecting 100 samples in estimated 5.7774 s (500 iterations)
Benchmarking sanctum_search_filters/no_filter: Analyzing
sanctum_search_filters/no_filter
                        time:   [11.475 ms 11.567 ms 11.672 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking sanctum_search_filters/filter_paladin_id
Benchmarking sanctum_search_filters/filter_paladin_id: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.1s, enable flat sampling, or reduce sample count to 60.
Benchmarking sanctum_search_filters/filter_paladin_id: Collecting 100 samples in estimated 6.1313 s (5050 iterations)
Benchmarking sanctum_search_filters/filter_paladin_id: Analyzing
sanctum_search_filters/filter_paladin_id
                        time:   [1.2163 ms 1.2600 ms 1.3104 ms]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking sanctum_search_filters/filter_memory_type
Benchmarking sanctum_search_filters/filter_memory_type: Warming up for 3.0000 s
Benchmarking sanctum_search_filters/filter_memory_type: Collecting 100 samples in estimated 5.0750 s (1300 iterations)
Benchmarking sanctum_search_filters/filter_memory_type: Analyzing
sanctum_search_filters/filter_memory_type
                        time:   [3.8355 ms 3.8643 ms 3.8946 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Benchmarking sanctum_search_filters/filter_importance
Benchmarking sanctum_search_filters/filter_importance: Warming up for 3.0000 s
Benchmarking sanctum_search_filters/filter_importance: Collecting 100 samples in estimated 5.5147 s (800 iterations)
Benchmarking sanctum_search_filters/filter_importance: Analyzing
sanctum_search_filters/filter_importance
                        time:   [6.8925 ms 6.9521 ms 7.0148 ms]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
Benchmarking sanctum_search_filters/filter_combined
Benchmarking sanctum_search_filters/filter_combined: Warming up for 3.0000 s
Benchmarking sanctum_search_filters/filter_combined: Collecting 100 samples in estimated 5.1689 s (50k iterations)
Benchmarking sanctum_search_filters/filter_combined: Analyzing
sanctum_search_filters/filter_combined
                        time:   [97.551 µs 99.089 µs 100.77 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking sanctum_update/update_single
Benchmarking sanctum_update/update_single: Warming up for 3.0000 s
Benchmarking sanctum_update/update_single: Collecting 100 samples in estimated 5.0038 s (1.4M iterations)
Benchmarking sanctum_update/update_single: Analyzing
sanctum_update/update_single
                        time:   [3.5163 µs 3.5622 µs 3.6110 µs]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking sanctum_delete/delete_single
Benchmarking sanctum_delete/delete_single: Warming up for 3.0000 s
Benchmarking sanctum_delete/delete_single: Collecting 100 samples in estimated 6.0380 s (20k iterations)
Benchmarking sanctum_delete/delete_single: Analyzing
sanctum_delete/delete_single
                        time:   [44.607 µs 45.439 µs 46.326 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

Benchmarking sanctum_count/count_all
Benchmarking sanctum_count/count_all: Warming up for 3.0000 s
Benchmarking sanctum_count/count_all: Collecting 100 samples in estimated 5.0002 s (104M iterations)
Benchmarking sanctum_count/count_all: Analyzing
sanctum_count/count_all time:   [47.729 ns 48.410 ns 49.167 ns]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
Benchmarking sanctum_count/count_with_filter
Benchmarking sanctum_count/count_with_filter: Warming up for 3.0000 s
Benchmarking sanctum_count/count_with_filter: Collecting 100 samples in estimated 5.1871 s (56k iterations)
Benchmarking sanctum_count/count_with_filter: Analyzing
sanctum_count/count_with_filter
                        time:   [92.951 µs 95.023 µs 97.120 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

Store operations:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_store_single/dimension/384` | `670.51 ns .. 708.55 ns` |
| `sanctum_store_single/dimension/768` | `764.90 ns .. 854.27 ns` |
| `sanctum_store_single/dimension/1536` | `643.57 ns .. 686.72 ns` |
| `sanctum_store_batch/batch_size/10` | `4.2926 µs .. 4.5311 µs` |
| `sanctum_store_batch/batch_size/50` | `21.046 µs .. 21.913 µs` |
| `sanctum_store_batch/batch_size/100` | `46.128 µs .. 49.694 µs` |
| `sanctum_store_batch/batch_size/500` | `338.36 µs .. 351.48 µs` |

Search scale:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_search_scale/vector_count/100` | `195.67 µs .. 199.69 µs` |
| `sanctum_search_scale/vector_count/1000` | `2.1077 ms .. 2.2588 ms` |
| `sanctum_search_scale/vector_count/5000` | `11.400 ms .. 11.650 ms` |
| `sanctum_search_scale/vector_count/10000` | `23.298 ms .. 23.860 ms` |

Search top-k and filters:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_search_topk/top_k/1` | `11.530 ms .. 11.690 ms` |
| `sanctum_search_topk/top_k/5` | `11.623 ms .. 11.765 ms` |
| `sanctum_search_topk/top_k/10` | `11.556 ms .. 11.769 ms` |
| `sanctum_search_topk/top_k/50` | `11.493 ms .. 11.748 ms` |
| `sanctum_search_topk/top_k/100` | `11.512 ms .. 11.757 ms` |
| `sanctum_search_filters/no_filter` | `11.475 ms .. 11.672 ms` |
| `sanctum_search_filters/filter_paladin_id` | `1.2163 ms .. 1.3104 ms` |
| `sanctum_search_filters/filter_memory_type` | `3.8355 ms .. 3.8946 ms` |
| `sanctum_search_filters/filter_importance` | `6.8925 ms .. 7.0148 ms` |
| `sanctum_search_filters/filter_combined` | `97.551 µs .. 100.77 µs` |

Mutation/count operations:

| Benchmark | Time (lower .. upper) |
|---|---|
| `sanctum_update/update_single` | `3.5163 µs .. 3.6110 µs` |
| `sanctum_delete/delete_single` | `44.607 µs .. 46.326 µs` |
| `sanctum_count/count_all` | `47.729 ns .. 49.167 ns` |
| `sanctum_count/count_with_filter` | `92.951 µs .. 97.120 µs` |

Throughput: not reported by this target.

#### Garrison Benchmarks

```
     Running benches/garrison_benchmarks.rs (target/release/deps/garrison_benchmarks-0b9b5fe6e32766ad)
Gnuplot not found, using plotters backend
Benchmarking garrison/write/100
Benchmarking garrison/write/100: Warming up for 3.0000 s
Benchmarking garrison/write/100: Collecting 100 samples in estimated 5.6922 s (30k iterations)
Benchmarking garrison/write/100: Analyzing
garrison/write/100      time:   [12.933 µs 13.198 µs 13.474 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking garrison/write/1000
Benchmarking garrison/write/1000: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.5s, enable flat sampling, or reduce sample count to 50.
Benchmarking garrison/write/1000: Collecting 100 samples in estimated 9.4642 s (5050 iterations)
Benchmarking garrison/write/1000: Analyzing
garrison/write/1000     time:   [125.56 µs 127.41 µs 129.26 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking garrison/write/10000
Benchmarking garrison/write/10000: Warming up for 3.0000 s
Benchmarking garrison/write/10000: Collecting 100 samples in estimated 5.7651 s (300 iterations)
Benchmarking garrison/write/10000: Analyzing
garrison/write/10000    time:   [1.2406 ms 1.2679 ms 1.2985 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking garrison/read_recent/100
Benchmarking garrison/read_recent/100: Warming up for 3.0000 s
Benchmarking garrison/read_recent/100: Collecting 100 samples in estimated 5.0149 s (1.3M iterations)
Benchmarking garrison/read_recent/100: Analyzing
garrison/read_recent/100
                        time:   [3.8830 µs 3.9534 µs 4.0285 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking garrison/read_recent/1000
Benchmarking garrison/read_recent/1000: Warming up for 3.0000 s
Benchmarking garrison/read_recent/1000: Collecting 100 samples in estimated 5.0141 s (1.3M iterations)
Benchmarking garrison/read_recent/1000: Analyzing
garrison/read_recent/1000
                        time:   [4.0197 µs 4.0795 µs 4.1452 µs]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
Benchmarking garrison/read_recent/10000
Benchmarking garrison/read_recent/10000: Warming up for 3.0000 s
Benchmarking garrison/read_recent/10000: Collecting 100 samples in estimated 5.0183 s (1.3M iterations)
Benchmarking garrison/read_recent/10000: Analyzing
garrison/read_recent/10000
                        time:   [3.9725 µs 4.0409 µs 4.1144 µs]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```

| Benchmark | Time (lower .. upper) |
|---|---|
| `garrison/write/100` | `12.933 µs .. 13.474 µs` |
| `garrison/write/1000` | `125.56 µs .. 129.26 µs` |
| `garrison/write/10000` | `1.2406 ms .. 1.2985 ms` |
| `garrison/read_recent/100` | `3.8830 µs .. 4.0285 µs` |
| `garrison/read_recent/1000` | `4.0197 µs .. 4.1452 µs` |
| `garrison/read_recent/10000` | `3.9725 µs .. 4.1144 µs` |

Throughput: not reported by this target.

#### LLM Serialization Benchmarks

```
     Running benches/llm_serialization_benchmarks.rs (target/release/deps/llm_serialization_benchmarks-989de85b1ea64b13)
Gnuplot not found, using plotters backend
Benchmarking llm/serialize_request
Benchmarking llm/serialize_request: Warming up for 3.0000 s
Benchmarking llm/serialize_request: Collecting 100 samples in estimated 5.0079 s (2.4M iterations)
Benchmarking llm/serialize_request: Analyzing
llm/serialize_request   time:   [2.0561 µs 2.1093 µs 2.1654 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking llm/deserialize_response
Benchmarking llm/deserialize_response: Warming up for 3.0000 s
Benchmarking llm/deserialize_response: Collecting 100 samples in estimated 5.0030 s (4.6M iterations)
Benchmarking llm/deserialize_response: Analyzing
llm/deserialize_response
                        time:   [1.1177 µs 1.1719 µs 1.2357 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Benchmarking llm/response_roundtrip
Benchmarking llm/response_roundtrip: Warming up for 3.0000 s
Benchmarking llm/response_roundtrip: Collecting 100 samples in estimated 5.0026 s (2.3M iterations)
Benchmarking llm/response_roundtrip: Analyzing
llm/response_roundtrip  time:   [2.2058 µs 2.2645 µs 2.3272 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

| Benchmark | Time (lower .. upper) |
|---|---|
| `llm/serialize_request` | `2.0561 µs .. 2.1654 µs` |
| `llm/deserialize_response` | `1.1177 µs .. 1.2357 µs` |
| `llm/response_roundtrip` | `2.2058 µs .. 2.3272 µs` |

Throughput: not reported by this target.

### Memory-per-Paladin

This figure comes from `examples/muster_baseline.rs`, a purpose-built recorded harness — **not**
from `criterion`, which produces neither memory nor startup figures. The harness constructs 1000
`Paladin` aggregates (via the same `PaladinBuilder` path this workspace's other examples use),
holds them all alive in one `Vec` so nothing is dropped mid-measurement, and reads
`/proc/self/status`'s `VmRSS:` line before and after.

```console
$ APP_ENV=test cargo run --offline --release --example muster_baseline
    Finished `release` profile [optimized] target(s) in 0.62s
     Running `target/release/examples/muster_baseline`
startup_to_first_paladin_ms=0
rss_before_kb=2716
rss_after_kb=3184
paladins_mustered=1000
rss_delta_kb=468
bytes_per_paladin=479
```

Arithmetic, reproducible from the printed lines above: `bytes_per_paladin = (rss_delta_kb * 1024)
/ paladins_mustered` = `(468 * 1024) / 1000` = `479` bytes/Paladin (integer division).

### Startup Time

This figure also comes from `examples/muster_baseline.rs`, not from `criterion`. Two figures are
recorded, with distinct labelled scopes:

- **In-process, to first Paladin (`startup_to_first_paladin_ms`):** elapsed time from
  `Instant::now()` captured as the first statement in `main` to the moment the first `Paladin` is
  fully constructed. This **excludes** pre-`main` dynamic-link and Rust runtime initialization
  time. Measured at `0` ms (sub-millisecond; the mock LLM port and `PaladinBuilder` path used here
  do no I/O).
- **Whole-process wall clock (`wall_clock_ms`):** the entire process invocation timed from the
  shell with `date +%s%N` immediately before and after running the already-built release binary
  directly (bypassing `cargo run`'s own startup overhead). This **includes** pre-`main` dynamic
  link and Rust runtime init time that the in-process figure excludes.

```console
$ BIN=target/release/examples/muster_baseline-529a1b0e92b724e8
$ START_NS=$(date +%s%N)
$ APP_ENV=test "$BIN"
$ END_NS=$(date +%s%N)
$ echo "wall_clock_ms=$(( (END_NS - START_NS) / 1000000 ))"
wall_clock_ms=8
startup_to_first_paladin_ms=0
rss_before_kb=2724
rss_after_kb=3192
paladins_mustered=1000
rss_delta_kb=468
bytes_per_paladin=479
```

### Not produced by this run

QUAL-05 additionally names the **Paladin execution loop** and **Arsenal invocation** as metric
families this baseline should cover. Neither has a shipped bench target: the Milestone-1
`paladin_benchmarks.rs`, `herald_benchmarks.rs` and `arsenal_benchmarks.rs` suites are not present
in the tree (confirmed by `find` against `benches/` and every crate's `benches/` directory).
Writing two new criterion suites is feature work inside a measurement phase, and a new
benchmark's first run is by definition not a baseline *against* anything — there is nothing prior
to compare it to. Both are recorded here as **deferred with reason**; no owner is assigned in
this phase (Phase 3's CONTEXT.md records the same disposition under D-12).

---

## Run — 2026-05-27 (superseded)

> Superseded by the 2026-08-02 run above. Figures below are retained unedited, on their original
> (different) hardware, and are never merged, averaged, or diffed against the 2026-08-02 run.

### Scope
This baseline covers the active Epic 3 benchmark targets:

- `config_benchmarks` (root crate)
- `battalion_benchmarks` (`paladin-battalion`)
- `sanctum_benchmarks` (`paladin-memory`)
- `garrison_benchmarks` (`paladin-memory`)
- `llm_serialization_benchmarks` (`paladin-llm`)

Run timestamp window (UTC): `2026-05-27T22:58:29` to `2026-05-27T23:08:23`

### Environment

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

### Methodology

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

### Results

#### Root Config Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `config/settings_new` | `1.2543 ms .. 1.4626 ms` |
| `config/domain_accessors` | `18.215 us .. 19.968 us` |

#### Battalion Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `battalion/formation_3_agents` | `3.6108 us .. 3.7968 us` |
| `battalion/phalanx_5_agents` | `42.619 us .. 44.681 us` |
| `battalion/campaign_branching_dag` | `7.3903 us .. 7.7433 us` |

#### Sanctum Benchmarks

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

#### Garrison Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `garrison/write/100` | `14.313 us .. 15.070 us` |
| `garrison/write/1000` | `134.61 us .. 140.43 us` |
| `garrison/write/10000` | `1.4570 ms .. 1.5865 ms` |
| `garrison/read_recent/100` | `3.8229 us .. 3.8732 us` |
| `garrison/read_recent/1000` | `3.8187 us .. 3.9446 us` |
| `garrison/read_recent/10000` | `5.5296 us .. 6.0342 us` |

#### LLM Serialization Benchmarks

| Benchmark | Time (lower .. upper) |
|---|---|
| `llm/serialize_request` | `2.1024 us .. 2.1942 us` |
| `llm/deserialize_response` | `999.13 ns .. 1.1325 us` |
| `llm/response_roundtrip` | `2.1588 us .. 2.2568 us` |

### Sanctum Comparison Notes (Post-Migration vs Pre-Migration)

Comparison method:

- Searched project docs and benchmark artifacts for pre-migration sanctum timing data.
- Checked `docs/SANCTUM_BENCHMARKS.md` and found benchmark templates/targets but no populated historical timing table.
- Used the current run as the first trustworthy post-migration baseline.

Observed variance and interpretation:

- `sanctum_search_scale/vector_count/10000` measured `30.458 ms .. 31.241 ms`, which is below the documented target of `< 100 ms`.
- Intra-run spread for this key metric is approximately `2.57%` of the lower bound (`(31.241 - 30.458) / 30.458`).
- Because no trustworthy pre-migration numeric baseline was found, cross-era variance is marked as unavailable.

### Historical Data Availability

Trustworthy historical data found:

- None for pre-migration sanctum timings in repository-tracked artifacts.

Areas without prior comparable baseline:

- Sanctum pre-migration numeric benchmark times.
- Newly introduced Epic 3 benchmarks: battalion crate-local suite, garrison crate-local suite, llm serialization suite, and root config benchmarks under the current migration structure.

### Coverage Cross-Check

All active benchmark targets are represented in this report:

- `config_benchmarks`: covered
- `battalion_benchmarks`: covered
- `sanctum_benchmarks`: covered
- `garrison_benchmarks`: covered
- `llm_serialization_benchmarks`: covered
