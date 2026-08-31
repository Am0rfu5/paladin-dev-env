#!/usr/bin/env bash
# package-release-binaries_test.sh
#
# Committed regression harness for scripts/package-release-binaries.sh
# (ARTIFACT-02, ARTIFACT-05, plan 21-02). Mirrors
# tests/scripts/create-or-reuse-release_test.sh's fixture-lifecycle pattern:
# every fixture is built under a single `mktemp -d` scratch directory removed
# on exit via a trap, the real tree is only ever read, and no assertion ever
# touches the network or a real toolchain build.
#
# Fixtures accumulate into $FAILED rather than exiting on the first
# mismatch, matching the "report everything, don't short-circuit" house
# style the guard scripts in this repo follow.
#
# Usage:  ./tests/scripts/package-release-binaries_test.sh
#         Or via `make test-shell-guards`.
# Exit:   0 if every assertion passes; non-zero otherwise, with a report of
#         which assertion(s) failed.

set -uo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GUARD="${WORKSPACE_ROOT}/scripts/package-release-binaries.sh"

if [ ! -f "${GUARD}" ]; then
    echo "ERROR: guard script not found at ${GUARD}" >&2
    exit 1
fi

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/package-release-binaries-test.XXXXXX")"
cleanup() {
    rm -rf "${SCRATCH}"
}
trap cleanup EXIT

FAILED=0
ASSERTIONS=0

# --- Real-tree mutation baseline, captured before any fixture runs. --------
MUTATION_WATCH_PATHS=(scripts .github/workflows)
BEFORE_STATUS="$(cd "${WORKSPACE_ROOT}" && git status --porcelain -- "${MUTATION_WATCH_PATHS[@]}")"

# A no-op stand-in for `strip` so fixtures do not need real object files --
# the guard script's own contract only requires *a* command that exits 0 for
# each present binary; what it does to the file is out of scope for this
# harness (Task 1's <behavior> row about strip is proven separately by the
# plan's `cargo build --bins ...` local verify step, not by this script).
STRIP_STUB="${SCRATCH}/strip-noop.sh"
cat > "${STRIP_STUB}" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
chmod +x "${STRIP_STUB}"

# run_script DIR ARGS... -> runs the guard against DIR with the shared
# no-op strip stub. Sets $LAST_OUTPUT and $LAST_STATUS.
run_script() {
    local dir="$1"
    shift
    LAST_OUTPUT="$(bash "${GUARD}" --release-dir "${dir}" --strip-cmd "${STRIP_STUB}" "$@" 2>&1)"
    LAST_STATUS=$?
}

# --- Case 1 (happy path): three fake binaries archive in manifest order and
#     produce a non-empty checksum naming the archive. ----------------------
DIR1="${SCRATCH}/case1"
mkdir -p "${DIR1}"
printf 'fake-paladin' > "${DIR1}/paladin"
printf 'fake-paladin-cli' > "${DIR1}/paladin-cli"
printf 'fake-paladin-server' > "${DIR1}/paladin-server"
run_script "${DIR1}" --target x86_64-unknown-linux-gnu --archive-name paladin-linux-amd64

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LAST_STATUS}" -eq 0 ]; then
    echo "PASS: happy-path run exits 0"
else
    echo "FAIL: happy-path run expected exit 0, got ${LAST_STATUS}"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
ACTUAL_MEMBERS="$(tar tzf "${DIR1}/paladin-linux-amd64.tar.gz" 2>/dev/null | tr '\n' ',' )"
if [ "${ACTUAL_MEMBERS}" = "paladin,paladin-cli,paladin-server," ]; then
    echo "PASS: archive contains exactly the three members in manifest order"
else
    echo "FAIL: expected members 'paladin,paladin-cli,paladin-server,' got '${ACTUAL_MEMBERS}'"
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if [ -s "${DIR1}/paladin-linux-amd64.tar.gz.sha256" ] \
    && grep -qF "paladin-linux-amd64.tar.gz" "${DIR1}/paladin-linux-amd64.tar.gz.sha256"; then
    echo "PASS: checksum file is non-empty and names the archive"
else
    echo "FAIL: checksum file missing, empty, or does not name the archive"
    FAILED=$((FAILED + 1))
fi

# --- Case 2 (missing binary): exits non-zero, names the target and the
#     missing binary, and creates no archive. --------------------------------
DIR2="${SCRATCH}/case2"
mkdir -p "${DIR2}"
printf 'fake-paladin' > "${DIR2}/paladin"
printf 'fake-paladin-cli' > "${DIR2}/paladin-cli"
# paladin-server deliberately absent
run_script "${DIR2}" --target x86_64-unknown-linux-gnu --archive-name paladin-linux-amd64

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LAST_STATUS}" -ne 0 ] \
    && grep -qF "::error::expected binaries not built for x86_64-unknown-linux-gnu" <<<"${LAST_OUTPUT}" \
    && grep -qF "paladin-server" <<<"${LAST_OUTPUT}"; then
    echo "PASS: missing binary is a named failure citing the target and the absent binary"
else
    echo "FAIL: expected a named ::error:: for the missing binary"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if [ ! -e "${DIR2}/paladin-linux-amd64.tar.gz" ]; then
    echo "PASS: no archive was created when a binary is missing"
else
    echo "FAIL: an archive was created despite a missing binary"
    FAILED=$((FAILED + 1))
fi

# --- Case 3 (portability): with sha256sum absent from PATH, the checksum is
#     produced by `shasum -a 256` and the digest matches. --------------------
PORT_BIN="${SCRATCH}/portable-bin"
mkdir -p "${PORT_BIN}"
for tool in tar gzip shasum bash; do
    real_path="$(command -v "${tool}" 2>/dev/null || true)"
    if [ -z "${real_path}" ]; then
        echo "SKIP: portability case requires '${tool}' on the test host's PATH; not found" >&2
    else
        ln -s "${real_path}" "${PORT_BIN}/${tool}"
    fi
done

DIR3="${SCRATCH}/case3"
mkdir -p "${DIR3}"
printf 'fake-paladin' > "${DIR3}/paladin"
printf 'fake-paladin-cli' > "${DIR3}/paladin-cli"
printf 'fake-paladin-server' > "${DIR3}/paladin-server"

if [ -x "${PORT_BIN}/tar" ] && [ -x "${PORT_BIN}/gzip" ] && [ -x "${PORT_BIN}/shasum" ] && [ -x "${PORT_BIN}/bash" ]; then
    # Invoke bash by its real, resolved-before-the-PATH-swap path -- PATH is
    # about to be narrowed to PORT_BIN for the *script's* lookups
    # (sha256sum must not resolve), and the interpreter launching it must
    # not depend on that same narrowed PATH to find itself.
    BASH_BIN="$(command -v bash)"
    # BASH_ENV (this devcontainer sources .devcontainer/paladin-env.sh for
    # every non-interactive bash) must not leak into the narrowed PATH
    # below -- that sourced file needs its own tools (e.g. `basename`) that
    # PORT_BIN deliberately does not carry, which is orthogonal to what
    # this case is testing.
    LAST_OUTPUT="$(PATH="${PORT_BIN}" BASH_ENV="" "${BASH_BIN}" "${GUARD}" --target x86_64-unknown-linux-gnu --release-dir "${DIR3}" --archive-name paladin-linux-amd64 --strip-cmd "${STRIP_STUB}" 2>&1)"
    LAST_STATUS=$?

    ASSERTIONS=$((ASSERTIONS + 1))
    if [ "${LAST_STATUS}" -eq 0 ]; then
        echo "PASS: run succeeds with sha256sum absent from PATH"
    else
        echo "FAIL: expected exit 0 with sha256sum absent from PATH, got ${LAST_STATUS}"
        echo "${LAST_OUTPUT}" | sed 's/^/  | /'
        FAILED=$((FAILED + 1))
    fi

    ASSERTIONS=$((ASSERTIONS + 1))
    EXPECTED_DIGEST="$(shasum -a 256 "${DIR3}/paladin-linux-amd64.tar.gz" 2>/dev/null | awk '{print $1}')"
    ACTUAL_DIGEST="$(awk '{print $1}' "${DIR3}/paladin-linux-amd64.tar.gz.sha256" 2>/dev/null)"
    if [ -n "${EXPECTED_DIGEST}" ] && [ "${EXPECTED_DIGEST}" = "${ACTUAL_DIGEST}" ]; then
        echo "PASS: the shasum -a 256 fallback digest matches an independently computed digest"
    else
        echo "FAIL: digest mismatch (expected '${EXPECTED_DIGEST}', got '${ACTUAL_DIGEST}')"
        FAILED=$((FAILED + 1))
    fi
else
    echo "SKIP: portability case skipped -- tar, gzip and shasum must all be present on the test host" >&2
fi

# --- The real tree must never be mutated by this test: scripts/ and
#     .github/workflows/ (every fixture lives under $SCRATCH). --------------
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
