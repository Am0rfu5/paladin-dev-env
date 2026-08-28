#!/usr/bin/env bash
# check-release-consistency_test.sh
#
# Regression harness for scripts/check-release-consistency.sh (PUBOPS-01,
# plan 20-01). Task 1's red-green scope: the six assertions its own
# <behavior> block specifies. Task 2 hardens this file to the full
# sibling-harness shape (mktemp scratch, write_metadata_fixture helper,
# assert_fire/assert_silent needle-pinning, an unmutated-tree assertion) --
# this file already borrows that shape's skeleton so the Task 2 extension is
# additive rather than a rewrite.
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

# write_metadata_fixture FILE PACKAGES_JSON -> writes a minimal
# `cargo metadata --format-version 1`-shaped document containing only the
# `packages` key this script's guard reads.
write_metadata_fixture() {
    local file="$1" packages_json="$2"
    printf '{"packages": [%s]}\n' "${packages_json}" > "${file}"
}

# pkg NAME VERSION [PUBLISH_JSON] -> echoes one packages[] element. Default
# PUBLISH_JSON is `null` (publishable); pass `[]` for a `publish = false` crate.
pkg() {
    local name="$1" version="$2" publish="${3:-null}"
    printf '{"name": "%s", "version": "%s", "publish": %s}' "${name}" "${version}" "${publish}"
}

# run_guard ARGS... -> sets $LAST_OUTPUT and $LAST_STATUS.
run_guard() {
    LAST_OUTPUT="$("${GUARD}" "$@" 2>&1)"
    LAST_STATUS=$?
}

# assert_fire DESC NEEDLE ARGS... -> expects non-zero exit AND $LAST_OUTPUT
# to contain NEEDLE (pins which status token fired, not just that something did).
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

# assert_silent DESC ARGS... -> expects zero exit.
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
write_metadata_fixture "${ALL_MATCH_FIXTURE}" \
    "$(pkg pkg-a 1.2.3), $(pkg pkg-b 1.2.3), $(pkg pkg-c 1.2.3)"

MIXED_FIXTURE="${SCRATCH}/mixed.json"
write_metadata_fixture "${MIXED_FIXTURE}" \
    "$(pkg pkg-a 1.2.3), $(pkg pkg-b 1.2.3-rc.1)"

EMPTY_FIXTURE="${SCRATCH}/empty.json"
write_metadata_fixture "${EMPTY_FIXTURE}" ""

# --- 1. All packages at 1.2.3, --tag v1.2.3: exit 0, output contains OK. ---
assert_silent "all packages match tag v1.2.3" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 2. Same fixture, --tag v9.9.9: exit non-zero, names every package. ----
ASSERTIONS=$((ASSERTIONS + 1))
run_guard --tag v9.9.9 --metadata-json "${ALL_MATCH_FIXTURE}"
if [ "${LAST_STATUS}" -eq 0 ]; then
    echo "FAIL: expected non-zero exit for: tag v9.9.9 mismatches every package (got 0)"
    FAILED=$((FAILED + 1))
elif ! grep -qF -- "pkg-a" <<<"${LAST_OUTPUT}" || \
     ! grep -qF -- "pkg-b" <<<"${LAST_OUTPUT}" || \
     ! grep -qF -- "pkg-c" <<<"${LAST_OUTPUT}"; then
    echo "FAIL: expected output to name all three packages (pkg-a, pkg-b, pkg-c)"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
else
    echo "PASS (fire): tag v9.9.9 mismatches every package, all three named"
fi

# --- 3. Mixed fixture (b is a prerelease suffix mismatch), --tag v1.2.3:
#        exit non-zero, names pkg-b. ----------------------------------------
assert_fire "prerelease suffix is a mismatch, not a match" "pkg-b" \
    --tag v1.2.3 --metadata-json "${MIXED_FIXTURE}"

# --- 4. Empty packages array: exit non-zero, ZERO_PACKAGES. ----------------
assert_fire "empty packages array is a named ZERO_PACKAGES failure" "ZERO_PACKAGES" \
    --metadata-json "${EMPTY_FIXTURE}" --tag v1.2.3

# --- 5. No --tag at all: exit non-zero, MISSING_TAG. ------------------------
assert_fire "no --tag at all is a named MISSING_TAG failure" "MISSING_TAG" \
    --metadata-json "${ALL_MATCH_FIXTURE}"

# --- 6. Running the same failing invocation twice is byte-identical. -------
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

echo ""
if [ "${FAILED}" -eq 0 ]; then
    echo "✅ ${ASSERTIONS} assertion(s) passed."
    exit 0
else
    echo "❌ ${FAILED}/${ASSERTIONS} assertion(s) failed."
    exit 1
fi
