#!/usr/bin/env bash
# benchmark-builds.sh — Measure clean and incremental build times for the
# Paladin workspace. Runs each scenario three times and reports the median.
#
# Usage: ./scripts/benchmark-builds.sh
# Run from the workspace root (directory containing Cargo.toml).

set -euo pipefail

RUNS=3

# Capture the 'real' time in seconds from a timed cargo command.
# Usage: time_build <label> <setup_cmd> <build_cmd>
# Returns: elapsed seconds written to stdout.
measure_seconds() {
    local setup_cmd="$1"
    local build_cmd="$2"

    eval "$setup_cmd" > /dev/null 2>&1

    local start
    start=$(date +%s%N)
    eval "$build_cmd" > /dev/null 2>&1
    local end
    end=$(date +%s%N)

    echo $(( (end - start) / 1000000 ))  # milliseconds
}

# Run a scenario RUNS times and print the median milliseconds.
run_scenario() {
    local label="$1"
    local setup_cmd="$2"
    local build_cmd="$3"

    echo -n "  Running '$label' (${RUNS}x):"
    local -a times=()
    for i in $(seq 1 $RUNS); do
        local ms
        ms=$(measure_seconds "$setup_cmd" "$build_cmd")
        times+=("$ms")
        echo -n " ${ms}ms"
    done
    echo

    # Sort and pick median
    IFS=$'\n' sorted=($(sort -n <<<"${times[*]}")); unset IFS
    local median="${sorted[$(( RUNS / 2 ))]}"
    echo "$median"
}

format_time() {
    local ms="$1"
    local seconds=$(( ms / 1000 ))
    local frac=$(( (ms % 1000) / 10 ))
    printf "%d.%02ds" "$seconds" "$frac"
}

echo "=== Paladin Build Benchmarks ==="
echo "Date: $(date '+%Y-%m-%d %H:%M:%S')"
echo "Rust: $(rustc --version)"
echo "Runs per scenario: $RUNS"
echo

echo "--- Workspace Scenarios ---"

echo "Scenario A: Clean build (cargo clean && cargo build --workspace)"
A_ms=$(run_scenario "A-clean" "cargo clean" "cargo build --workspace")

echo "Scenario B: Core incremental (touch paladin-core)"
B_ms=$(run_scenario "B-core-incr" \
    "touch crates/paladin-core/src/lib.rs" \
    "cargo build --workspace")

echo "Scenario C: LLM adapter incremental"
C_ms=$(run_scenario "C-llm-incr" \
    "touch crates/paladin-llm/src/lib.rs" \
    "cargo build --workspace")

echo "Scenario D: Memory adapter incremental"
D_ms=$(run_scenario "D-memory-incr" \
    "touch crates/paladin-memory/src/lib.rs" \
    "cargo build --workspace")

echo "Scenario E: Battalion-only incremental"
E_ms=$(run_scenario "E-battalion-only" \
    "touch crates/paladin-battalion/src/lib.rs" \
    "cargo build -p paladin-battalion")

echo
echo "=== Summary Table ==="
echo
echo "| Scenario | Description                  | Median Time |"
echo "|----------|------------------------------|-------------|"
echo "| A        | Clean build (workspace)      | $(format_time $A_ms) |"
echo "| B        | paladin-core incremental     | $(format_time $B_ms) |"
echo "| C        | paladin-llm incremental      | $(format_time $C_ms) |"
echo "| D        | paladin-memory incremental   | $(format_time $D_ms) |"
echo "| E        | paladin-battalion only       | $(format_time $E_ms) |"
echo
echo "Raw milliseconds: A=$A_ms B=$B_ms C=$C_ms D=$D_ms E=$E_ms"
