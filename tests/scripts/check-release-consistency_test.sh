#!/usr/bin/env bash
# check-release-consistency_test.sh
#
# Committed regression harness for scripts/check-release-consistency.sh
# (PUBOPS-01, plan 20-01). Mirrors tests/scripts/check-workflow-triggers_test.sh's
# fixture-lifecycle pattern: every fixture is built under a single
# `mktemp -d` scratch directory removed on exit via a trap, the real tree is
# only ever read, and a closing assertion double-checks nothing real was
# mutated -- extended here to cover the four paths this gate's own tests
# could plausibly touch by accident (Cargo.toml, crates/, CHANGELOG.md,
# .github/workflows/), since a future clause (SHA/CI-conclusion) reads
# .github/workflows/ too.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard itself follows.
#
# Usage:  ./tests/scripts/check-release-consistency_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/check-release-consistency.sh"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/check-release-consistency-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# --- Real-tree mutation baseline, captured before any fixture runs. --------
MUTATION_WATCH_PATHS=(Cargo.toml crates CHANGELOG.md .github/workflows)
BEFORE_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"

# write_metadata_fixture FILE PAIR... -> writes a minimal
# `cargo metadata --format-version 1`-shaped document containing only the
# `packages` key this guard reads. Each PAIR is `name=version` (publishable,
# `publish: null`) or `name=version:false` (a `publish = false` crate,
# reported by `cargo metadata` as `publish: []` -- the one exempt shape D-08
# excludes, matching `paladin-doc-examples` in the real tree).
write_metadata_fixture() {
    local file="$1"
    shift
    local entries=() pair name rest version publish_json
    for pair in "$@"; do
        name="${pair%%=*}"
        rest="${pair#*=}"
        if [[ "${rest}" == *:false ]]; then
            version="${rest%:false}"
            publish_json="[]"
        else
            version="${rest}"
            publish_json="null"
        fi
        entries+=("{\"name\": \"${name}\", \"version\": \"${version}\", \"publish\": ${publish_json}}")
    done
    local joined
    joined="$(IFS=,; echo "${entries[*]:-}")"
    printf '{"packages": [%s]}\n' "${joined}" > "${file}"
}

# run_guard ARGS... -> sets $LAST_OUTPUT and $LAST_STATUS.
run_guard() {
    LAST_OUTPUT="$("${GUARD}" "$@" 2>&1)"
    LAST_STATUS=$?
}

# assert-fire helper: DESC NEEDLE ARGS... -> expects non-zero exit AND
# $LAST_OUTPUT to contain NEEDLE (pins which status token/message fired, not
# just that something did).
assert_fire() {
    local desc="$1" needle="$2"
    shift 2
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "$@"
    if [ "${LAST_STATUS}" -eq 0 ]; then
        echo "FAIL: expected non-zero exit for: ${desc} (got 0)"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    if ! grep -qF -- "${needle}" <<<"${LAST_OUTPUT}"; then
        echo "FAIL: expected output to contain '${needle}' for: ${desc}"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (fire): ${desc}"
}

# assert_silent DESC ARGS... -> expects zero exit AND $LAST_OUTPUT to
# contain the OK status token.
assert_silent() {
    local desc="$1"
    shift
    ASSERTIONS=$((ASSERTIONS + 1))
    run_guard "$@"
    if [ "${LAST_STATUS}" -ne 0 ]; then
        echo "FAIL: expected zero exit (silent) for: ${desc} (got ${LAST_STATUS})"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    if ! grep -qF -- "OK" <<<"${LAST_OUTPUT}"; then
        echo "FAIL: expected output to contain 'OK' for: ${desc}"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
        return
    fi
    echo "PASS (silent): ${desc}"
}

ALL_MATCH_FIXTURE="${SCRATCH}/all-match.json"
write_metadata_fixture "${ALL_MATCH_FIXTURE}" pkg-a=1.2.3 pkg-b=1.2.3 pkg-c=1.2.3

MIXED_FIXTURE="${SCRATCH}/mixed.json"
write_metadata_fixture "${MIXED_FIXTURE}" pkg-a=1.2.3 pkg-b=1.2.3-rc.1

EMPTY_FIXTURE="${SCRATCH}/empty.json"
write_metadata_fixture "${EMPTY_FIXTURE}"

EXEMPT_ONLY_FIXTURE="${SCRATCH}/exempt-only.json"
write_metadata_fixture "${EXEMPT_ONLY_FIXTURE}" doc-examples-pkg=1.2.3:false

SINGLE_FIXTURE="${SCRATCH}/single.json"
write_metadata_fixture "${SINGLE_FIXTURE}" pkg-solo=1.2.3

# --- 1. All packages at 1.2.3, --tag v1.2.3: exit 0, output contains OK. ---
assert_silent "all packages match tag v1.2.3" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 2. Same fixture, --tag v9.9.9: exit non-zero, names every package
#        (one call per name below, so each pins its own needle). -----------
assert_fire "tag v9.9.9 mismatch names pkg-a" "pkg-a" \
    --tag v9.9.9 --metadata-json "${ALL_MATCH_FIXTURE}"
assert_fire "tag v9.9.9 mismatch names pkg-b" "pkg-b" \
    --tag v9.9.9 --metadata-json "${ALL_MATCH_FIXTURE}"
assert_fire "tag v9.9.9 mismatch names pkg-c" "pkg-c" \
    --tag v9.9.9 --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 3. Mixed fixture (b is a prerelease suffix mismatch), --tag v1.2.3:
#        exit non-zero, names pkg-b. ----------------------------------------
assert_fire "prerelease suffix is a mismatch, not a match" "pkg-b" \
    --tag v1.2.3 --metadata-json "${MIXED_FIXTURE}"

# --- 4. Empty packages array: exit non-zero, ZERO_PACKAGES. ----------------
assert_fire "empty packages array is a named ZERO_PACKAGES failure" "ZERO_PACKAGES" \
    --metadata-json "${EMPTY_FIXTURE}" --tag v1.2.3

# --- 4a. A package IS present but carries publish = false: the publishable
#         set is still empty -> ZERO_PACKAGES, not a vacuous OK. This proves
#         the `publish` filter itself, not just the empty-array shortcut. --
assert_fire "the only package present is publish=false -> ZERO_PACKAGES, not OK" "ZERO_PACKAGES" \
    --metadata-json "${EXEMPT_ONLY_FIXTURE}" --tag v1.2.3

# --- 5. No --tag at all: exit non-zero, MISSING_TAG. ------------------------
assert_fire "no --tag at all is a named MISSING_TAG failure" "MISSING_TAG" \
    --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 6. Exactly one publishable package: still a real verdict, not a
#        special case. ------------------------------------------------------
assert_silent "single-element publishable set produces a real OK verdict" \
    --tag v1.2.3 --metadata-json "${SINGLE_FIXTURE}"

# --- 7. A tag supplied without the leading "v" behaves identically to one
#        with it. ------------------------------------------------------------
assert_silent "--tag 1.2.3 (no leading v) behaves identically to --tag v1.2.3" \
    --tag 1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 8. An unknown flag is a usage error (non-zero), not a silent no-op. ---
assert_fire "an unknown flag is a usage error" "unknown flag" \
    --bogus-flag foo --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 9. Running the same failing invocation twice is byte-identical. -------
ASSERTIONS=$((ASSERTIONS + 1))
out1="$("${GUARD}" --tag v9.9.9 --metadata-json "${ALL_MATCH_FIXTURE}" 2>&1)"
status1=$?
out2="$("${GUARD}" --tag v9.9.9 --metadata-json "${ALL_MATCH_FIXTURE}" 2>&1)"
status2=$?
if [ "${out1}" = "${out2}" ] && [ "${status1}" -eq "${status2}" ]; then
    echo "PASS (idempotent): two runs of the same failing invocation are byte-identical"
else
    echo "FAIL: two runs of the same failing invocation were not byte-identical"
    FAILED=$((FAILED + 1))
fi

# --- The real tree must never be mutated by this test: Cargo.toml, crates/,
#     CHANGELOG.md and .github/workflows/ (this guard only reads Cargo
#     manifests today, but a later plan's clauses read the other three). ---
ASSERTIONS=$((ASSERTIONS + 1))
AFTER_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"
if [ "${BEFORE_STATUS}" = "${AFTER_STATUS}" ]; then
    echo "PASS (no mutation): git status --porcelain -- Cargo.toml crates CHANGELOG.md .github/workflows is unchanged"
else
    echo "FAIL: Cargo.toml, crates/, CHANGELOG.md or .github/workflows/ was mutated by this test run:"
    echo "before: ${BEFORE_STATUS}" | sed 's/^/  | /'
    echo "after:  ${AFTER_STATUS}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

echo ""
if [ "${FAILED}" -eq 0 ]; then
    echo "✅ ${ASSERTIONS} assertion(s) passed."
    exit 0
else
    echo "❌ ${FAILED}/${ASSERTIONS} assertion(s) failed."
    exit 1
fi
