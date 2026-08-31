#!/usr/bin/env bash
# finalize-release-body_test.sh
#
# Committed regression harness for scripts/finalize-release-body.sh
# (ARTIFACT-03/ARTIFACT-04, plan 21-03). Mirrors
# tests/scripts/create-or-reuse-release_test.sh's fixture-lifecycle pattern:
# every fixture is built under a single `mktemp -d` scratch directory removed
# on exit via a trap, the real tree is only ever read, and no assertion ever
# touches the network.
#
# Two testing strategies, matched to what each behavior actually needs:
#   - Pure composition (curated text + artifact inputs -> a body file) is
#     exercised directly against compose_release_body via the
#     FINALIZE_RELEASE_BODY_LIB_ONLY=1 sourcing seam, inside a `$(...)`
#     subshell (matching tests/scripts/package-release-binaries_test.sh's
#     pattern) so the sourced script's own `set -euo pipefail` never leaks
#     into this test script's shell -- no `gh`, no stub, no network, fast
#     and exact (`cmp`/`grep -F`).
#   - The read-modify-write round trip (does a second run over its own
#     previous output stay byte-identical?) is exercised against the real
#     script, run as a fresh `bash` subprocess, with a scripted `gh` stub on
#     the scratch PATH selected through the GH_BIN seam
#     finalize-release-body.sh reads -- the stub answers
#     `release view --json body -q .body` from a fixture file and records
#     `release edit --notes-file` invocations by copying the notes file back
#     into that same fixture file, so a second run reads exactly what the
#     first run published.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard scripts in this repo follow.
#
# Usage:  ./tests/scripts/finalize-release-body_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/finalize-release-body.sh"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/finalize-release-body-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# --- Real-tree mutation baseline, captured before any fixture runs. --------
MUTATION_WATCH_PATHS=(scripts .github/workflows)
BEFORE_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"

# compose_to SUBDIR NAME CURATED DIGEST IMAGE_REF SIZE_MB -> calls
# compose_release_body with the given inputs inside a subshell (so the
# sourced script's `set -euo pipefail` is scoped to that subshell only),
# writing to ${SCRATCH}/${SUBDIR}/${NAME}. Sets $LAST_COMPOSE_FILE. A
# non-zero exit from the subshell surfaces as this function's own exit
# status, so a caller wanting to assert on it can check `$?` immediately
# after calling compose_to.
compose_to() {
    local subdir="$1" name="$2" curated="$3" digest="$4" image_ref="$5" size_mb="$6"
    mkdir -p "${SCRATCH}/${subdir}"
    LAST_COMPOSE_FILE="${SCRATCH}/${subdir}/${name}"
    (
        # shellcheck source=scripts/finalize-release-body.sh
        FINALIZE_RELEASE_BODY_LIB_ONLY=1 source "${GUARD}"
        compose_release_body "${curated}" "${digest}" "${image_ref}" "${size_mb}" "${LAST_COMPOSE_FILE}"
    )
}

# assert_contains DESC FILE NEEDLE -> FILE must contain the literal NEEDLE.
assert_contains() {
    local desc="$1" file="$2" needle="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if grep -qF -- "${needle}" "${file}"; then
        echo "PASS: ${desc}"
    else
        echo "FAIL: ${desc} -- expected to find '${needle}' in ${file}"
        sed 's/^/  | /' "${file}"
        FAILED=$((FAILED + 1))
    fi
}

# assert_not_contains DESC FILE NEEDLE -> FILE must NOT contain NEEDLE.
assert_not_contains() {
    local desc="$1" file="$2" needle="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if grep -qF -- "${needle}" "${file}"; then
        echo "FAIL: ${desc} -- did not expect to find '${needle}' in ${file}"
        sed 's/^/  | /' "${file}"
        FAILED=$((FAILED + 1))
    else
        echo "PASS: ${desc}"
    fi
}

# assert_cmp DESC FILE_A FILE_B -> the two files must be byte-identical.
assert_cmp() {
    local desc="$1" file_a="$2" file_b="$3"
    ASSERTIONS=$((ASSERTIONS + 1))
    if cmp -s "${file_a}" "${file_b}"; then
        echo "PASS (cmp): ${desc}"
    else
        echo "FAIL: ${desc} -- ${file_a} and ${file_b} differ"
        diff "${file_a}" "${file_b}" 2>&1 | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
    fi
}

CURATED1="## [1.2.3] - 2026-01-01

### Added

- A feature.
"

# --- Case 1: no marker, digest + ref + size present -> curated unchanged,
#     marker, a container-image section with a runnable pull line, and an
#     advisory image-size section. -------------------------------------------
compose_to case1 body1.md "${CURATED1}" "abc123" "ghcr.io/df3ndr/paladin-dev-env:1.2.3" "450"
BODY1="${LAST_COMPOSE_FILE}"
assert_contains "case1: curated text preserved" "${BODY1}" "### Added"
assert_contains "case1: marker present" "${BODY1}" "<!-- paladin:release-artifacts -->"
assert_contains "case1: pull line pinned to digest" "${BODY1}" 'docker pull ghcr.io/df3ndr/paladin-dev-env@sha256:abc123'
assert_contains "case1: image size section present" "${BODY1}" "### Image size"
assert_contains "case1: 450 MB reads as within target" "${BODY1}" "within target"

# --- Case 2: digest already carries the sha256: prefix -> exactly one
#     prefix reaches the pull line, never doubled. ---------------------------
compose_to case2 body2.md "${CURATED1}" "sha256:def456" "ghcr.io/df3ndr/paladin-dev-env:1.2.3" ""
BODY2="${LAST_COMPOSE_FILE}"
assert_contains "case2: pre-prefixed digest used verbatim" "${BODY2}" '@sha256:def456'
assert_not_contains "case2: digest is never double-prefixed" "${BODY2}" 'sha256:sha256:'

# --- Case 3: no digest and no image ref -> no container-image section at
#     all, even though a size was supplied. -----------------------------------
compose_to case3 body3.md "${CURATED1}" "" "" "300"
BODY3="${LAST_COMPOSE_FILE}"
assert_not_contains "case3: no digest/ref -> no container-image section" "${BODY3}" "### Container image"
assert_contains "case3: image size section still present" "${BODY3}" "### Image size"

# --- Case 4: no artifact inputs at all -> states no artifacts were
#     recorded, never an empty heading. ---------------------------------------
compose_to case4 body4.md "${CURATED1}" "" "" ""
BODY4="${LAST_COMPOSE_FILE}"
assert_contains "case4: no inputs -> explicit no-artifacts statement" "${BODY4}" "No artifacts were recorded for this run."
assert_not_contains "case4: no container-image section" "${BODY4}" "### Container image"
assert_not_contains "case4: no image-size section" "${BODY4}" "### Image size"

# --- Case 5: size exactly 500 reads as within target; 501 reads as over
#     target -- neither changes the composer's own exit status. --------------
compose_to case5a body5a.md "${CURATED1}" "" "" "500"
assert_contains "case5a: 500 MB reads as within target" "${LAST_COMPOSE_FILE}" "within target"
compose_to case5b body5b.md "${CURATED1}" "" "" "501"
assert_contains "case5b: 501 MB reads as over target" "${LAST_COMPOSE_FILE}" "over target"

# --- Case 6: read-modify-write round trip via the real script + a stubbed
#     `gh` -- running the composer a second time over its own previous
#     output produces a byte-identical result. --------------------------------
write_gh_stub() {
    local dir="$1"
    cat > "${dir}/gh-stub.sh" <<'STUB'
#!/usr/bin/env bash
set -uo pipefail
SCRATCH_DIR="$(cd "$(dirname "$0")" && pwd)"
CALL_LOG="${SCRATCH_DIR}/call_log"

if [ "${1:-}" = "release" ] && [ "${2:-}" = "view" ]; then
    echo "VIEW" >> "${CALL_LOG}"
    cat "${SCRATCH_DIR}/current_body" 2>/dev/null || true
    exit 0
elif [ "${1:-}" = "release" ] && [ "${2:-}" = "edit" ]; then
    echo "EDIT" >> "${CALL_LOG}"
    args=("$@")
    i=0
    NOTES_FILE=""
    while [ "$i" -lt "${#args[@]}" ]; do
        a="${args[$i]}"
        if [ "$a" = "--notes-file" ]; then
            i=$((i + 1))
            NOTES_FILE="${args[$i]}"
        fi
        i=$((i + 1))
    done
    if [ -n "${NOTES_FILE}" ]; then
        cp "${NOTES_FILE}" "${SCRATCH_DIR}/current_body"
    fi
    exit 0
else
    echo "UNKNOWN gh invocation: $*" >&2
    exit 1
fi
STUB
    chmod +x "${dir}/gh-stub.sh"
}

DIR6="${SCRATCH}/case6"
mkdir -p "${DIR6}"
write_gh_stub "${DIR6}"
printf '%s' "${CURATED1}" > "${DIR6}/current_body"

ASSERTIONS=$((ASSERTIONS + 1))
if FINALIZE_RELEASE_BODY_LIB_ONLY=0 GH_BIN="${DIR6}/gh-stub.sh" bash "${GUARD}" \
    --tag v1.2.3 --image-digest abc123 \
    --image-ref "ghcr.io/df3ndr/paladin-dev-env:1.2.3" --image-size-mb 450 \
    --output "${DIR6}/output_run1.md" >"${DIR6}/run1.log" 2>&1; then
    echo "PASS: case6 first finalize run exits 0"
else
    echo "FAIL: case6 first finalize run did not exit 0"
    sed 's/^/  | /' "${DIR6}/run1.log"
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if FINALIZE_RELEASE_BODY_LIB_ONLY=0 GH_BIN="${DIR6}/gh-stub.sh" bash "${GUARD}" \
    --tag v1.2.3 --image-digest abc123 \
    --image-ref "ghcr.io/df3ndr/paladin-dev-env:1.2.3" --image-size-mb 450 \
    --output "${DIR6}/output_run2.md" >"${DIR6}/run2.log" 2>&1; then
    echo "PASS: case6 second finalize run exits 0"
else
    echo "FAIL: case6 second finalize run did not exit 0"
    sed 's/^/  | /' "${DIR6}/run2.log"
    FAILED=$((FAILED + 1))
fi

assert_cmp "case6: second run over its own previous output is byte-identical to the first" \
    "${DIR6}/output_run1.md" "${DIR6}/output_run2.md"

ASSERTIONS=$((ASSERTIONS + 1))
EDIT_CALLS="$(grep -cx 'EDIT' "${DIR6}/call_log" 2>/dev/null || true)"
EDIT_CALLS="${EDIT_CALLS:-0}"
if [ "${EDIT_CALLS}" = "2" ]; then
    echo "PASS (call-count): case6 makes exactly one release-edit call per run (2 total)"
else
    echo "FAIL: case6 expected 2 release-edit calls, got ${EDIT_CALLS}"
    FAILED=$((FAILED + 1))
fi

# --- Missing --tag: usage error. ---------------------------------------------
ASSERTIONS=$((ASSERTIONS + 1))
DIR_MISSING_TAG="${SCRATCH}/case-missing-tag"
mkdir -p "${DIR_MISSING_TAG}"
MISSING_TAG_OUTPUT="$(FINALIZE_RELEASE_BODY_LIB_ONLY=0 GH_BIN="${DIR6}/gh-stub.sh" bash "${GUARD}" --image-digest abc123 2>&1)"
MISSING_TAG_STATUS=$?
if [ "${MISSING_TAG_STATUS}" -ne 0 ] && grep -qF -- "--tag is required" <<<"${MISSING_TAG_OUTPUT}"; then
    echo "PASS (fire): missing --tag is a usage error"
else
    echo "FAIL: expected a non-zero exit naming '--tag is required'"
    echo "${MISSING_TAG_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- Unknown flag: usage error, no gh call at all. ---------------------------
ASSERTIONS=$((ASSERTIONS + 1))
UNKNOWN_FLAG_OUTPUT="$(FINALIZE_RELEASE_BODY_LIB_ONLY=0 GH_BIN="${DIR6}/gh-stub.sh" bash "${GUARD}" --tag v1.2.3 --bogus-flag foo 2>&1)"
UNKNOWN_FLAG_STATUS=$?
if [ "${UNKNOWN_FLAG_STATUS}" -ne 0 ] && grep -qF -- "unknown flag" <<<"${UNKNOWN_FLAG_OUTPUT}"; then
    echo "PASS (fire): an unknown flag is a usage error"
else
    echo "FAIL: expected a non-zero exit naming the unknown flag"
    echo "${UNKNOWN_FLAG_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- The real tree must never be mutated by this test: scripts/ and
#     .github/workflows/ (this guard only invokes the stubbed GH_BIN, never
#     the real `gh`, and never writes into the repo tree). -------------------
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

echo ""
if [ "${FAILED}" -eq 0 ]; then
    echo "✅ ${ASSERTIONS} assertion(s) passed."
    exit 0
else
    echo "❌ ${FAILED}/${ASSERTIONS} assertion(s) failed."
    exit 1
fi
