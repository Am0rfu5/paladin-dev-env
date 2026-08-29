#!/usr/bin/env bash
# publish-crates_test.sh
#
# Committed regression harness for scripts/publish-crates.sh (PUBOPS-03
# criterion 2, PUBOPS-04, plan 20-05). Mirrors
# tests/scripts/create-or-reuse-release_test.sh's fixture-lifecycle pattern:
# every fixture is built under a single `mktemp -d` scratch directory
# removed on exit via a trap, the real tree is only ever read, and no
# assertion ever touches the network or actually publishes anything.
#
# Two stubbed binaries are used, selected through the CURL_BIN and CARGO_BIN
# seams publish-crates.sh reads:
#
#   curl-stub.sh  -- recognises the two crates.io URL shapes the script
#                     calls (the versioned api/v1 pre-check and the sparse
#                     -index poll), reads a scripted HTTP status (and, for
#                     the index, a body) per crate name from fixture files,
#                     and appends every invocation's full argv to a call log
#                     -- so assertions can check the outcome AND that every
#                     call carried a User-Agent header and no -L/--location.
#   cargo-stub.sh -- recognises `publish [--dry-run] -p <crate>`, appends
#                     one line per invocation to a call log (so assertions
#                     can prove zero/one/N publish attempts occurred), and
#                     exits non-zero for any crate named in a
#                     `cargo_fail__<crate>` marker file.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard scripts in this repo follow.
#
# Usage:  ./tests/scripts/publish-crates_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/publish-crates.sh"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/publish-crates-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# --- Real-tree mutation baseline, captured before any fixture runs. --------
MUTATION_WATCH_PATHS=(scripts .github/workflows)
BEFORE_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"

# write_curl_stub DIR -> writes a scripted stand-in for `curl` into DIR.
# Recognises the versioned crates.io pre-check
# (https://crates.io/api/v1/crates/<name>/<version>, called with
# `-w '%{http_code}'`) and the sparse-index fetch
# (https://index.crates.io/<path>/<name>, called with `-sf`).
#
# Fixture files (SAFE_NAME = crate name with non-alnum chars -> '_'):
#   precheck_status__<SAFE_NAME>       -- single status, every call
#   precheck_status_seq__<SAFE_NAME>   -- one status per line, consumed in
#                                          order (last line repeats once
#                                          exhausted) -- for retry/backoff
#                                          cases
#   index_status__<SAFE_NAME> / index_status_seq__<SAFE_NAME> -- same shape,
#                                          for the sparse-index fetch's HTTP
#                                          outcome (2xx = body served, else a
#                                          curl -f style failure)
#   index_body__<SAFE_NAME> / index_body_seq__<SAFE_NAME>     -- the body
#                                          text served on a 2xx index status
#
# Every invocation appends its full argv (with headers, so `User-Agent` and
# the absence of `-L`/`--location` are directly assertable) to
# DIR/curl_call_log.
write_curl_stub() {
    local dir="$1"
    cat >"${dir}/curl-stub.sh" <<'STUB'
#!/usr/bin/env bash
set -uo pipefail
SCRATCH_DIR="$(cd "$(dirname "$0")" && pwd)"
CALL_LOG="${SCRATCH_DIR}/curl_call_log"

{
    printf '%s' "curl"
    for a in "$@"; do printf ' %q' "$a"; done
    printf '\n'
} >>"${CALL_LOG}"

URL=""
MODE="status"
for a in "$@"; do
    case "$a" in
        -sf) MODE="body" ;;
        -*) ;;
        *) URL="$a" ;;
    esac
done

if [[ "${URL}" == *"crates.io/api/v1/crates/"* ]]; then
    REST="${URL#*api/v1/crates/}"
    NAME="${REST%%/*}"
elif [[ "${URL}" == *"index.crates.io/"* ]]; then
    NAME="$(basename "${URL}")"
else
    echo "curl-stub: unrecognised URL: ${URL}" >&2
    exit 7
fi

SAFE_NAME="${NAME//[^a-zA-Z0-9_]/_}"

next_from_seq() {
    local base="$1"
    local seq_file="${SCRATCH_DIR}/${base}_seq__${SAFE_NAME}"
    local single_file="${SCRATCH_DIR}/${base}__${SAFE_NAME}"
    local counter_file="${SCRATCH_DIR}/${base}_n__${SAFE_NAME}"
    if [ -f "${seq_file}" ]; then
        local n=0
        [ -f "${counter_file}" ] && n="$(cat "${counter_file}")"
        n=$((n + 1))
        echo "${n}" >"${counter_file}"
        local total
        total="$(wc -l <"${seq_file}")"
        if [ "${n}" -gt "${total}" ]; then
            n="${total}"
        fi
        sed -n "${n}p" "${seq_file}"
    elif [ -f "${single_file}" ]; then
        cat "${single_file}"
    else
        echo ""
    fi
}

if [ "${MODE}" = "status" ]; then
    STATUS="$(next_from_seq precheck_status)"
    [ -z "${STATUS}" ] && STATUS="500"
    printf '%s' "${STATUS}"
    exit 0
else
    STATUS="$(next_from_seq index_status)"
    [ -z "${STATUS}" ] && STATUS="404"
    BODY="$(next_from_seq index_body)"
    case "${STATUS}" in
        2*)
            printf '%s\n' "${BODY}"
            exit 0
            ;;
        *)
            exit 22
            ;;
    esac
fi
STUB
    chmod +x "${dir}/curl-stub.sh"
}

# write_cargo_stub DIR -> writes a scripted stand-in for `cargo`. Recognises
# `publish [--dry-run] -p <crate>`, appends one line per invocation to
# DIR/cargo_call_log, and exits non-zero (with an error line on stderr) for
# any crate named by a DIR/cargo_fail__<SAFE_NAME> marker file.
write_cargo_stub() {
    local dir="$1"
    cat >"${dir}/cargo-stub.sh" <<'STUB'
#!/usr/bin/env bash
set -uo pipefail
SCRATCH_DIR="$(cd "$(dirname "$0")" && pwd)"
CALL_LOG="${SCRATCH_DIR}/cargo_call_log"

echo "$*" >>"${CALL_LOG}"

NAME=""
prev=""
for a in "$@"; do
    if [ "${prev}" = "-p" ]; then
        NAME="$a"
    fi
    prev="$a"
done

SAFE_NAME="${NAME//[^a-zA-Z0-9_]/_}"
if [ -f "${SCRATCH_DIR}/cargo_fail__${SAFE_NAME}" ]; then
    echo "error: failed to publish ${NAME}" >&2
    exit 101
fi
echo "    Uploading ${NAME}"
exit 0
STUB
    chmod +x "${dir}/cargo-stub.sh"
}

# run_main DIR ARGS... -> runs the full guard (publish_crates_main) with
# CURL_BIN/CARGO_BIN pointed at DIR's stubs. Sets $LAST_OUTPUT/$LAST_STATUS.
run_main() {
    local dir="$1"
    shift
    LAST_OUTPUT="$(CURL_BIN="${dir}/curl-stub.sh" CARGO_BIN="${dir}/cargo-stub.sh" "${GUARD}" "$@" 2>&1)"
    LAST_STATUS=$?
}

# run_lib DIR FUNC ARGS... -> sources the guard with
# PUBLISH_CRATES_LIB_ONLY=1 and calls FUNC ARGS directly, with CURL_BIN /
# CARGO_BIN pointed at DIR's stubs. Sets $LAST_OUTPUT/$LAST_STATUS.
run_lib() {
    local dir="$1"
    shift
    LAST_OUTPUT="$(
        CURL_BIN="${dir}/curl-stub.sh" CARGO_BIN="${dir}/cargo-stub.sh" \
            PUBLISH_CRATES_LIB_ONLY=1 bash -c '
                source "$1"
                shift
                "$@"
            ' _ "${GUARD}" "$@" 2>&1
    )"
    LAST_STATUS=$?
}

assert_eq() {
    local desc="$1" expected="$2" actual="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if [ "${expected}" = "${actual}" ]; then
        echo "PASS: ${desc}"
    else
        echo "FAIL: ${desc} -- expected '${expected}', got '${actual}'"
        FAILED=$((FAILED + 1))
    fi
}

assert_status() {
    local desc="$1" expected="$2" actual="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if [ "${expected}" = "0" ] && [ "${actual}" -eq 0 ]; then
        echo "PASS: ${desc}"
    elif [ "${expected}" = "nonzero" ] && [ "${actual}" -ne 0 ]; then
        echo "PASS: ${desc}"
    else
        echo "FAIL: ${desc} -- expected status ${expected}, got ${actual}"
        FAILED=$((FAILED + 1))
    fi
}

assert_contains() {
    local desc="$1" needle="$2" haystack="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if grep -qF -- "${needle}" <<<"${haystack}"; then
        echo "PASS: ${desc}"
    else
        echo "FAIL: ${desc} -- expected output to contain '${needle}'"
        echo "${haystack}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
    fi
}

assert_not_contains() {
    local desc="$1" needle="$2" haystack="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if grep -qF -- "${needle}" <<<"${haystack}"; then
        echo "FAIL: ${desc} -- expected output NOT to contain '${needle}'"
        echo "${haystack}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
    else
        echo "PASS: ${desc}"
    fi
}

call_count() {
    local file="$1" pattern="$2"
    grep -cF -- "${pattern}" "${file}" 2>/dev/null || true
}

################################################################################
# Task 1 behaviour: registry-state pre-check and bounded index-visibility
# poll (12 cases from <behavior>).
################################################################################

# --- Case 1: pre-check 200 -> no publish attempted, already-at-this-version.
DIR1="${SCRATCH}/case1"
mkdir -p "${DIR1}"
write_curl_stub "${DIR1}"
write_cargo_stub "${DIR1}"
echo "200" >"${DIR1}/precheck_status__crate_a"
run_lib "${DIR1}" _pc_crate_published crate-a 1.0.0
assert_status "pre-check 200 returns success (published)" 0 "${LAST_STATUS}"
run_lib "${DIR1}" _pc_publish_one crate-a 1.0.0 false 180 5
ASSERTIONS=$((ASSERTIONS + 1))
CARGO_CALLS="$(call_count "${DIR1}/cargo_call_log" "crate-a")"
if [ -z "${CARGO_CALLS}" ] || [ "${CARGO_CALLS}" = "0" ]; then
    echo "PASS: pre-check 200 makes zero cargo publish calls"
else
    echo "FAIL: pre-check 200 made ${CARGO_CALLS} cargo publish call(s), expected 0"
    FAILED=$((FAILED + 1))
fi

# --- Case 2: pre-check 200 for a yanked version -> still
#     already-at-this-version, still no publish attempted (this script
#     decides purely from the HTTP status; a yanked version still returns
#     200 from the versioned endpoint).
DIR2="${SCRATCH}/case2"
mkdir -p "${DIR2}"
write_curl_stub "${DIR2}"
write_cargo_stub "${DIR2}"
echo "200" >"${DIR2}/precheck_status__crate_b"
run_lib "${DIR2}" _pc_crate_published crate-b 1.0.0
assert_status "pre-check 200 (yanked version) still returns success" 0 "${LAST_STATUS}"

# --- Case 3: pre-check 404, publish succeeds, first index poll finds the
#     version -> published-now, elapsed poll iterations == 1.
DIR3="${SCRATCH}/case3"
mkdir -p "${DIR3}"
write_curl_stub "${DIR3}"
write_cargo_stub "${DIR3}"
echo "404" >"${DIR3}/precheck_status__crate_c"
echo "200" >"${DIR3}/index_status__crate_c"
echo '{"vers":"1.0.0","yanked":false}' >"${DIR3}/index_body__crate_c"
run_lib "${DIR3}" _pc_publish_one crate-c 1.0.0 false 180 5
assert_status "pre-check 404 + publish + immediate index visibility succeeds" 0 "${LAST_STATUS}"
run_lib "${DIR3}" bash -c 'source "$0"; _pc_wait_for_index_visibility crate-c 1.0.0 180 5; echo "ITERATIONS=${PC_LAST_POLL_ITERATIONS}"' "${GUARD}"
assert_contains "first-check-success poll reports 1 iteration" "ITERATIONS=1" "${LAST_OUTPUT}"

# --- Case 4: pre-check 404, publish succeeds, index shows the version on
#     the third poll iteration -> published-now.
DIR4="${SCRATCH}/case4"
mkdir -p "${DIR4}"
write_curl_stub "${DIR4}"
write_cargo_stub "${DIR4}"
echo "404" >"${DIR4}/precheck_status__crate_d"
printf '404\n404\n200\n' >"${DIR4}/index_status_seq__crate_d"
printf '\n\n{"vers":"1.0.0","yanked":false}\n' >"${DIR4}/index_body_seq__crate_d"
run_lib "${DIR4}" bash -c 'source "$0"; _pc_wait_for_index_visibility crate-d 1.0.0 180 1; echo "ITERATIONS=${PC_LAST_POLL_ITERATIONS}"' "${GUARD}"
assert_status "third-iteration index visibility succeeds" 0 "${LAST_STATUS}"
assert_contains "third-iteration poll reports 3 iterations" "ITERATIONS=3" "${LAST_OUTPUT}"

# --- Case 5: pre-check 404, publish succeeds, index never shows the
#     version, timeout 10 interval 5 -> failed, message names the timeout,
#     iterations == timeout / interval == 2.
DIR5="${SCRATCH}/case5"
mkdir -p "${DIR5}"
write_curl_stub "${DIR5}"
write_cargo_stub "${DIR5}"
echo "404" >"${DIR5}/precheck_status__crate_e"
echo "404" >"${DIR5}/index_status__crate_e"
run_lib "${DIR5}" bash -c 'source "$0"; rc=0; _pc_wait_for_index_visibility crate-e 1.0.0 10 5 || rc=$?; echo "ITERATIONS=${PC_LAST_POLL_ITERATIONS}"; exit "${rc}"' "${GUARD}"
assert_status "timeout case is a failure" nonzero "${LAST_STATUS}"
assert_contains "timeout message names the timeout value" "10s" "${LAST_OUTPUT}"
assert_contains "timeout case reports timeout/interval == 2 iterations" "ITERATIONS=2" "${LAST_OUTPUT}"

# --- Case 6: pre-check 429 twice then 404 -- retry path taken, publish
#     attempted, crate does not end failed on account of the rate limit.
DIR6="${SCRATCH}/case6"
mkdir -p "${DIR6}"
write_curl_stub "${DIR6}"
write_cargo_stub "${DIR6}"
printf '429\n429\n404\n' >"${DIR6}/precheck_status_seq__crate_f"
echo "200" >"${DIR6}/index_status__crate_f"
echo '{"vers":"1.0.0","yanked":false}' >"${DIR6}/index_body__crate_f"
run_lib "${DIR6}" _pc_publish_one crate-f 1.0.0 false 180 1
assert_status "429-then-404 pre-check eventually attempts and succeeds" 0 "${LAST_STATUS}"
CARGO_CALLS6="$(call_count "${DIR6}/cargo_call_log" "crate-f")"
assert_eq "429-then-404 makes exactly one cargo publish call" "1" "${CARGO_CALLS6:-0}"

# --- Case 7: pre-check 500 -> failed, no publish attempted.
DIR7="${SCRATCH}/case7"
mkdir -p "${DIR7}"
write_curl_stub "${DIR7}"
write_cargo_stub "${DIR7}"
echo "500" >"${DIR7}/precheck_status__crate_g"
run_lib "${DIR7}" _pc_publish_one crate-g 1.0.0 false 180 5
assert_status "pre-check 500 is a per-crate failure" nonzero "${LAST_STATUS}"
CARGO_CALLS7="$(call_count "${DIR7}/cargo_call_log" "crate-g")"
if [ -z "${CARGO_CALLS7}" ] || [ "${CARGO_CALLS7}" = "0" ]; then
    echo "PASS: pre-check 500 makes zero cargo publish calls"
    ASSERTIONS=$((ASSERTIONS + 1))
else
    echo "FAIL: pre-check 500 made ${CARGO_CALLS7} cargo publish call(s), expected 0"
    ASSERTIONS=$((ASSERTIONS + 1))
    FAILED=$((FAILED + 1))
fi

# --- Case 8: publish command exits non-zero -> failed.
DIR8="${SCRATCH}/case8"
mkdir -p "${DIR8}"
write_curl_stub "${DIR8}"
write_cargo_stub "${DIR8}"
echo "404" >"${DIR8}/precheck_status__crate_h"
touch "${DIR8}/cargo_fail__crate_h"
run_lib "${DIR8}" _pc_publish_one crate-h 1.0.0 false 180 5
assert_status "cargo publish failure is a per-crate failure" nonzero "${LAST_STATUS}"

# --- Case 9: invalid arguments (interval 0, or timeout < interval) exit
#     non-zero as a usage error before any network call.
DIR9="${SCRATCH}/case9"
mkdir -p "${DIR9}"
write_curl_stub "${DIR9}"
write_cargo_stub "${DIR9}"
run_main "${DIR9}" --version 1.0.0 --poll-interval 0
assert_status "poll-interval 0 is a usage error" nonzero "${LAST_STATUS}"
ASSERTIONS=$((ASSERTIONS + 1))
if [ ! -s "${DIR9}/curl_call_log" ]; then
    echo "PASS: poll-interval 0 makes no network call before failing"
else
    echo "FAIL: poll-interval 0 made a curl call before validating arguments"
    FAILED=$((FAILED + 1))
fi

DIR9B="${SCRATCH}/case9b"
mkdir -p "${DIR9B}"
write_curl_stub "${DIR9B}"
write_cargo_stub "${DIR9B}"
run_main "${DIR9B}" --version 1.0.0 --poll-timeout 3 --poll-interval 5
assert_status "poll-timeout smaller than poll-interval is a usage error" nonzero "${LAST_STATUS}"
ASSERTIONS=$((ASSERTIONS + 1))
if [ ! -s "${DIR9B}/curl_call_log" ]; then
    echo "PASS: poll-timeout < poll-interval makes no network call before failing"
else
    echo "FAIL: poll-timeout < poll-interval made a curl call before validating arguments"
    FAILED=$((FAILED + 1))
fi

# --- Case 10: a crates file listing zero crates -- named failure, non-zero
#     exit.
DIR10="${SCRATCH}/case10"
mkdir -p "${DIR10}"
write_curl_stub "${DIR10}"
write_cargo_stub "${DIR10}"
: >"${DIR10}/empty-crates.txt"
run_main "${DIR10}" --version 1.0.0 --crates-file "${DIR10}/empty-crates.txt"
assert_status "an empty --crates-file is a named zero-crates failure" nonzero "${LAST_STATUS}"
assert_contains "zero-crates failure names the condition" "zero crates" "${LAST_OUTPUT}"

# --- Case 11: every crates.io request carries a User-Agent header and no
#     redirect-following option.
DIR11="${SCRATCH}/case11"
mkdir -p "${DIR11}"
write_curl_stub "${DIR11}"
write_cargo_stub "${DIR11}"
echo "404" >"${DIR11}/precheck_status__crate_i"
echo "200" >"${DIR11}/index_status__crate_i"
echo '{"vers":"1.0.0","yanked":false}' >"${DIR11}/index_body__crate_i"
run_lib "${DIR11}" _pc_publish_one crate-i 1.0.0 false 180 5
ASSERTIONS=$((ASSERTIONS + 1))
if [ -s "${DIR11}/curl_call_log" ] && ! grep -qL 'User-Agent' "${DIR11}/curl_call_log" 2>/dev/null; then
    :
fi
if grep -q 'User-Agent' "${DIR11}/curl_call_log" && [ "$(grep -c 'User-Agent' "${DIR11}/curl_call_log")" -eq "$(wc -l <"${DIR11}/curl_call_log")" ]; then
    echo "PASS: every curl call carries a User-Agent header"
else
    echo "FAIL: not every curl call carried a User-Agent header"
    cat "${DIR11}/curl_call_log" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi
ASSERTIONS=$((ASSERTIONS + 1))
if grep -qE -- '(^|[[:space:]])(-L|--location)([[:space:]]|$)' "${DIR11}/curl_call_log"; then
    echo "FAIL: a curl call passed -L/--location"
    FAILED=$((FAILED + 1))
else
    echo "PASS: no curl call passes -L/--location"
fi

################################################################################
# Task 2 behaviour: outcome table, abort-to-skipped semantics, and the
# no-crate-moved failure (9 cases from <behavior>).
################################################################################

FIXTURE_CRATES=(alpha bravo charlie delta echo foxtrot golf hotel india juliett kilo)
write_crates_file() {
    local dir="$1"
    printf '%s\n' "${FIXTURE_CRATES[@]}" >"${dir}/crates.txt"
}

# --- Case A: all eleven pre-check 404 and publish cleanly -> eleven rows
#     all published-now, exit 0.
DIRA="${SCRATCH}/caseA"
mkdir -p "${DIRA}"
write_curl_stub "${DIRA}"
write_cargo_stub "${DIRA}"
write_crates_file "${DIRA}"
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    echo "404" >"${DIRA}/precheck_status__${safe}"
    echo "200" >"${DIRA}/index_status__${safe}"
    echo "{\"vers\":\"1.0.0\",\"yanked\":false}" >"${DIRA}/index_body__${safe}"
done
run_main "${DIRA}" --version 1.0.0 --crates-file "${DIRA}/crates.txt" --poll-interval 1
assert_status "all-404 run exits zero" 0 "${LAST_STATUS}"
ASSERTIONS=$((ASSERTIONS + 1))
ROWS_A="$(grep -cE '^\| .+ \| published-now \|$' <<<"${LAST_OUTPUT}")"
if [ "${ROWS_A}" -eq 11 ]; then
    echo "PASS: all-404 run produces eleven published-now rows"
else
    echo "FAIL: all-404 run produced ${ROWS_A} published-now rows, expected 11"
    FAILED=$((FAILED + 1))
fi

# --- Case B: all eleven pre-check 200 -> eleven already-at-this-version
#     rows, exit non-zero, message names version/fully-published/runbook.
DIRB="${SCRATCH}/caseB"
mkdir -p "${DIRB}"
write_curl_stub "${DIRB}"
write_cargo_stub "${DIRB}"
write_crates_file "${DIRB}"
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    echo "200" >"${DIRB}/precheck_status__${safe}"
done
run_main "${DIRB}" --version 2.5.0 --crates-file "${DIRB}/crates.txt"
assert_status "all-already-published run exits non-zero" nonzero "${LAST_STATUS}"
ASSERTIONS=$((ASSERTIONS + 1))
ROWS_B="$(grep -cE '^\| .+ \| already-at-this-version \|$' <<<"${LAST_OUTPUT}")"
if [ "${ROWS_B}" -eq 11 ]; then
    echo "PASS: all-already-published run produces eleven already-at-this-version rows"
else
    echo "FAIL: all-already-published run produced ${ROWS_B} already-at-this-version rows, expected 11"
    FAILED=$((FAILED + 1))
fi
assert_contains "no-crate-moved message names the version" "2.5.0" "${LAST_OUTPUT}"
assert_contains "no-crate-moved message says the tag appears fully published" "fully published" "${LAST_OUTPUT}"
assert_contains "no-crate-moved message points at the recovery runbook" "docs/src/appendix/release-recovery.md" "${LAST_OUTPUT}"

# --- Case C: four pre-check 200, seven 404-then-publish -> four
#     already-at-this-version, seven published-now, exit 0.
DIRC="${SCRATCH}/caseC"
mkdir -p "${DIRC}"
write_curl_stub "${DIRC}"
write_cargo_stub "${DIRC}"
write_crates_file "${DIRC}"
i=0
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    i=$((i + 1))
    if [ "${i}" -le 4 ]; then
        echo "200" >"${DIRC}/precheck_status__${safe}"
    else
        echo "404" >"${DIRC}/precheck_status__${safe}"
        echo "200" >"${DIRC}/index_status__${safe}"
        echo "{\"vers\":\"1.0.0\",\"yanked\":false}" >"${DIRC}/index_body__${safe}"
    fi
done
run_main "${DIRC}" --version 1.0.0 --crates-file "${DIRC}/crates.txt" --poll-interval 1
assert_status "partial-recovery run exits zero" 0 "${LAST_STATUS}"
ROWS_C_ALREADY="$(grep -cE '^\| .+ \| already-at-this-version \|$' <<<"${LAST_OUTPUT}")"
assert_eq "partial-recovery run has four already-at-this-version rows" "4" "${ROWS_C_ALREADY}"
ROWS_C_PUBLISHED="$(grep -cE '^\| .+ \| published-now \|$' <<<"${LAST_OUTPUT}")"
assert_eq "partial-recovery run has seven published-now rows" "7" "${ROWS_C_PUBLISHED}"

# --- Case D: crate three fails -> crates one/two keep outcomes, crate three
#     failed, crates four-eleven skipped, no publish for any of them, exit
#     non-zero, exactly three publish attempts total.
DIRD="${SCRATCH}/caseD"
mkdir -p "${DIRD}"
write_curl_stub "${DIRD}"
write_cargo_stub "${DIRD}"
write_crates_file "${DIRD}"
i=0
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    i=$((i + 1))
    echo "404" >"${DIRD}/precheck_status__${safe}"
    if [ "${i}" -eq 3 ]; then
        touch "${DIRD}/cargo_fail__${safe}"
    else
        echo "200" >"${DIRD}/index_status__${safe}"
        echo "{\"vers\":\"1.0.0\",\"yanked\":false}" >"${DIRD}/index_body__${safe}"
    fi
done
run_main "${DIRD}" --version 1.0.0 --crates-file "${DIRD}/crates.txt" --poll-interval 1
assert_status "mid-loop failure run exits non-zero" nonzero "${LAST_STATUS}"
assert_contains "crate one (alpha) stays published-now" "| alpha | published-now |" "${LAST_OUTPUT}"
assert_contains "crate two (bravo) stays published-now" "| bravo | published-now |" "${LAST_OUTPUT}"
assert_contains "crate three (charlie) is failed" "| charlie | failed |" "${LAST_OUTPUT}"
assert_contains "crate four (delta) is skipped" "| delta | skipped |" "${LAST_OUTPUT}"
assert_contains "crate eleven (kilo) is skipped" "| kilo | skipped |" "${LAST_OUTPUT}"
PUBLISH_ATTEMPTS_D="$(grep -c '^publish ' "${DIRD}/cargo_call_log" 2>/dev/null || true)"
assert_eq "mid-loop failure makes exactly three publish attempts" "3" "${PUBLISH_ATTEMPTS_D:-0}"

# --- Case E: dry-run over eleven crates -> eleven skipped rows, exit 0, no
#     -crate-moved rule does not fire.
DIRE="${SCRATCH}/caseE"
mkdir -p "${DIRE}"
write_curl_stub "${DIRE}"
write_cargo_stub "${DIRE}"
write_crates_file "${DIRE}"
run_main "${DIRE}" --version 1.0.0 --crates-file "${DIRE}/crates.txt" --dry-run
assert_status "dry-run over eleven crates exits zero" 0 "${LAST_STATUS}"
ROWS_E="$(grep -cE '^\| .+ \| skipped \|$' <<<"${LAST_OUTPUT}")"
assert_eq "dry-run produces eleven skipped rows" "11" "${ROWS_E}"
assert_not_contains "dry-run never fires the no-crate-moved message" "appears fully published" "${LAST_OUTPUT}"

# --- Case F: GITHUB_STEP_SUMMARY pointed at a scratch file -> after the run
#     that file contains the same table that went to stdout.
DIRF="${SCRATCH}/caseF"
mkdir -p "${DIRF}"
write_curl_stub "${DIRF}"
write_cargo_stub "${DIRF}"
write_crates_file "${DIRF}"
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    echo "404" >"${DIRF}/precheck_status__${safe}"
    echo "200" >"${DIRF}/index_status__${safe}"
    echo "{\"vers\":\"1.0.0\",\"yanked\":false}" >"${DIRF}/index_body__${safe}"
done
SUMMARY_FILE="${DIRF}/step_summary.md"
: >"${SUMMARY_FILE}"
LAST_OUTPUT="$(CURL_BIN="${DIRF}/curl-stub.sh" CARGO_BIN="${DIRF}/cargo-stub.sh" GITHUB_STEP_SUMMARY="${SUMMARY_FILE}" "${GUARD}" --version 1.0.0 --crates-file "${DIRF}/crates.txt" --poll-interval 1 2>&1)"
LAST_STATUS=$?
assert_status "GITHUB_STEP_SUMMARY run exits zero" 0 "${LAST_STATUS}"
ASSERTIONS=$((ASSERTIONS + 1))
STDOUT_TABLE="$(sed -n '/^## Publish outcome/,$p' <<<"${LAST_OUTPUT}")"
SUMMARY_TABLE="$(cat "${SUMMARY_FILE}")"
if [ "${STDOUT_TABLE}" = "${SUMMARY_TABLE}" ]; then
    echo "PASS: GITHUB_STEP_SUMMARY table is byte-identical to the stdout table"
else
    echo "FAIL: GITHUB_STEP_SUMMARY table differs from the stdout table"
    FAILED=$((FAILED + 1))
fi

# --- Case G: GITHUB_STEP_SUMMARY unset -> run still succeeds, table still
#     reaches stdout.
DIRG="${SCRATCH}/caseG"
mkdir -p "${DIRG}"
write_curl_stub "${DIRG}"
write_cargo_stub "${DIRG}"
write_crates_file "${DIRG}"
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    echo "404" >"${DIRG}/precheck_status__${safe}"
    echo "200" >"${DIRG}/index_status__${safe}"
    echo "{\"vers\":\"1.0.0\",\"yanked\":false}" >"${DIRG}/index_body__${safe}"
done
LAST_OUTPUT="$(CURL_BIN="${DIRG}/curl-stub.sh" CARGO_BIN="${DIRG}/cargo-stub.sh" env -u GITHUB_STEP_SUMMARY "${GUARD}" --version 1.0.0 --crates-file "${DIRG}/crates.txt" --poll-interval 1 2>&1)"
LAST_STATUS=$?
assert_status "GITHUB_STEP_SUMMARY-unset run still succeeds" 0 "${LAST_STATUS}"
assert_contains "GITHUB_STEP_SUMMARY-unset run still emits the table to stdout" "## Publish outcome" "${LAST_OUTPUT}"

# --- Case H: table row order equals declared crate order in every case
#     above, and two runs over identical stub state produce byte-identical
#     tables.
ASSERTIONS=$((ASSERTIONS + 1))
EXPECTED_ORDER_A="$(printf '%s\n' "${FIXTURE_CRATES[@]}")"
ACTUAL_ORDER_A="$(grep -E '^\| .+ \| .+ \|$' <<<"${LAST_OUTPUT}" | grep -vF '| Crate | Outcome |' | sed -E 's/^\| ([^ ]+) \|.*/\1/')"
if [ "${EXPECTED_ORDER_A}" = "${ACTUAL_ORDER_A}" ]; then
    echo "PASS: table row order equals the declared crate order"
else
    echo "FAIL: table row order does not equal the declared crate order"
    echo "expected: ${EXPECTED_ORDER_A}"
    echo "actual:   ${ACTUAL_ORDER_A}"
    FAILED=$((FAILED + 1))
fi

DIRH2="${SCRATCH}/caseH2"
mkdir -p "${DIRH2}"
write_curl_stub "${DIRH2}"
write_cargo_stub "${DIRH2}"
write_crates_file "${DIRH2}"
for c in "${FIXTURE_CRATES[@]}"; do
    safe="${c//[^a-zA-Z0-9_]/_}"
    echo "200" >"${DIRH2}/precheck_status__${safe}"
done
run_main "${DIRH2}" --version 3.0.0 --crates-file "${DIRH2}/crates.txt"
FIRST_RUN_OUTPUT="${LAST_OUTPUT}"
run_main "${DIRH2}" --version 3.0.0 --crates-file "${DIRH2}/crates.txt"
SECOND_RUN_OUTPUT="${LAST_OUTPUT}"
ASSERTIONS=$((ASSERTIONS + 1))
if [ "${FIRST_RUN_OUTPUT}" = "${SECOND_RUN_OUTPUT}" ]; then
    echo "PASS: two identical runs produce byte-identical output"
else
    echo "FAIL: two identical runs over identical stub state produced different output"
    FAILED=$((FAILED + 1))
fi

# --- Case I: every crate appears exactly once in the table, and no crate
#     carries a state outside the four named ones.
ASSERTIONS=$((ASSERTIONS + 1))
ALL_STATES_VALID="true"
while IFS= read -r state; do
    case "${state}" in
        published-now | already-at-this-version | skipped | failed) ;;
        *) ALL_STATES_VALID="false" ;;
    esac
done < <(grep -E '^\| .+ \| .+ \|$' <<<"${SECOND_RUN_OUTPUT}" | grep -vF '| Crate | Outcome |' | sed -E 's/^\| [^ ]+ \| ([^ ]+) \|$/\1/')
ROW_COUNT_I="$(grep -E '^\| .+ \| .+ \|$' <<<"${SECOND_RUN_OUTPUT}" | grep -vcF '| Crate | Outcome |')"
if [ "${ALL_STATES_VALID}" = "true" ] && [ "${ROW_COUNT_I}" -eq 11 ]; then
    echo "PASS: every crate appears exactly once with a valid state"
else
    echo "FAIL: table has ${ROW_COUNT_I} rows (expected 11) or an invalid state"
    FAILED=$((FAILED + 1))
fi

# --- Case J: a leading "v" on --version is stripped, matching the
#     check-release-consistency.sh convention (release.yml passes the raw
#     tag, e.g. "v1.0.0"); the outcome table and crates.io calls use the
#     bare version string.
DIRJ="${SCRATCH}/caseJ"
mkdir -p "${DIRJ}"
write_curl_stub "${DIRJ}"
write_cargo_stub "${DIRJ}"
echo "404" >"${DIRJ}/precheck_status__crate_j"
echo "200" >"${DIRJ}/index_status__crate_j"
echo '{"vers":"1.0.0","yanked":false}' >"${DIRJ}/index_body__crate_j"
CRATES_FILE_J="${DIRJ}/crates.txt"
echo "crate-j" >"${CRATES_FILE_J}"
run_main "${DIRJ}" --version v1.0.0 --crates-file "${CRATES_FILE_J}" --poll-interval 1
assert_status "a leading v on --version still succeeds" 0 "${LAST_STATUS}"
assert_contains "the outcome table header uses the stripped version, not 'v1.0.0'" "## Publish outcome -- 1.0.0" "${LAST_OUTPUT}"
assert_not_contains "the outcome table header does not carry the leading v" "## Publish outcome -- v1.0.0" "${LAST_OUTPUT}"
ASSERTIONS=$((ASSERTIONS + 1))
if grep -qF "crates.io/api/v1/crates/crate-j/1.0.0" "${DIRJ}/curl_call_log" 2>/dev/null; then
    echo "PASS: the crates.io pre-check URL uses the stripped version (no leading v)"
else
    echo "FAIL: the crates.io pre-check URL did not use the stripped bare version"
    cat "${DIRJ}/curl_call_log" 2>&1 | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

echo ""
if [ "${FAILED}" -eq 0 ]; then
    echo "✅ ${ASSERTIONS} assertion(s) passed."
else
    echo "❌ ${FAILED}/${ASSERTIONS} assertion(s) failed."
fi

# --- The real tree must never be mutated by this test. ---------------------
ASSERTIONS=$((ASSERTIONS + 1))
AFTER_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"
if [ "${BEFORE_STATUS}" = "${AFTER_STATUS}" ]; then
    echo "PASS (no mutation): git status --porcelain -- scripts .github/workflows is unchanged"
else
    echo "FAIL: scripts/ or .github/workflows/ was mutated by this test run:"
    echo "before: ${BEFORE_STATUS}" | sed 's/^/  | /'
    echo "after:  ${AFTER_STATUS}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

if [ "${FAILED}" -eq 0 ]; then
    exit 0
else
    exit 1
fi
