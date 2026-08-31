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

# --- Case 4 (unknown target): a target triple with no manifest entry at all
#     is a named failure, not a silently-empty expected list. ---------------
DIR4="${SCRATCH}/case4"
mkdir -p "${DIR4}"
run_script "${DIR4}" --target totally-unknown-triple --archive-name whatever

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LAST_STATUS}" -ne 0 ] && grep -qF "unknown target" <<<"${LAST_OUTPUT}"; then
    echo "PASS: an unrecognised target triple is a named failure"
else
    echo "FAIL: expected a named failure for an unrecognised target triple"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- Case 5 (narrowed manifest): exercised through the
#     PACKAGE_RELEASE_BINARIES_LIB_ONLY sourcing seam with a locally
#     overridden expected_binaries_for_target -- the shipped manifest is
#     never edited. A target narrowed to a single binary archives exactly
#     that binary and nothing else. --------------------------------------
DIR5="${SCRATCH}/case5"
mkdir -p "${DIR5}"
printf 'only-bin' > "${DIR5}/paladin"
printf 'not-expected' > "${DIR5}/paladin-cli"
LIB_CASE5_OUTPUT="$(
    # shellcheck source=scripts/package-release-binaries.sh
    PACKAGE_RELEASE_BINARIES_LIB_ONLY=1 source "${GUARD}"
    expected_binaries_for_target() {
        case "$1" in
            narrow-target) echo "paladin" ;;
            *) return 1 ;;
        esac
    }
    package_release_binaries_main --target narrow-target --release-dir "${DIR5}" --archive-name narrow-archive --strip-cmd "${STRIP_STUB}" 2>&1
)"
LIB_CASE5_STATUS=$?

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LIB_CASE5_STATUS}" -eq 0 ]; then
    echo "PASS: a manifest narrowed to a single binary runs successfully"
else
    echo "FAIL: narrowed-manifest run expected exit 0, got ${LIB_CASE5_STATUS}"
    echo "${LIB_CASE5_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
NARROW_MEMBERS="$(tar tzf "${DIR5}/narrow-archive.tar.gz" 2>/dev/null | tr '\n' ',')"
if [ "${NARROW_MEMBERS}" = "paladin," ]; then
    echo "PASS: a narrowed manifest archives exactly that one binary and nothing else"
else
    echo "FAIL: expected archive members 'paladin,' got '${NARROW_MEMBERS}'"
    FAILED=$((FAILED + 1))
fi

# --- Case 6 (empty manifest): a target whose manifest resolves to an empty
#     list is a hard, named failure -- never a silent no-op archive. --------
DIR6="${SCRATCH}/case6"
mkdir -p "${DIR6}"
LIB_CASE6_OUTPUT="$(
    # shellcheck source=scripts/package-release-binaries.sh
    PACKAGE_RELEASE_BINARIES_LIB_ONLY=1 source "${GUARD}"
    expected_binaries_for_target() {
        case "$1" in
            empty-target) echo "" ;;
            *) return 1 ;;
        esac
    }
    package_release_binaries_main --target empty-target --release-dir "${DIR6}" --archive-name empty-archive --strip-cmd "${STRIP_STUB}" 2>&1
)"
LIB_CASE6_STATUS=$?

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LIB_CASE6_STATUS}" -ne 0 ] && grep -qF "::error::" <<<"${LIB_CASE6_OUTPUT}" && grep -qiF "empty" <<<"${LIB_CASE6_OUTPUT}"; then
    echo "PASS: a manifest resolving to an empty list is a named failure"
else
    echo "FAIL: expected a named ::error:: for an empty resolved manifest"
    echo "${LIB_CASE6_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

ASSERTIONS=$((ASSERTIONS + 1))
if [ ! -e "${DIR6}/empty-archive.tar.gz" ]; then
    echo "PASS: an empty resolved manifest creates no archive"
else
    echo "FAIL: an archive was created despite an empty resolved manifest"
    FAILED=$((FAILED + 1))
fi

# --- Case 7 (exact-name matching): a directory entry named 'paladin-cli'
#     does not satisfy the expected entry 'paladin' -- no prefix or glob
#     matching. ---------------------------------------------------------
DIR7="${SCRATCH}/case7"
mkdir -p "${DIR7}"
printf 'not-paladin' > "${DIR7}/paladin-cli"
printf 'server' > "${DIR7}/paladin-server"
# 'paladin' itself is deliberately absent -- only 'paladin-cli' exists.
run_script "${DIR7}" --target x86_64-unknown-linux-gnu --archive-name exact-name

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LAST_STATUS}" -ne 0 ] && grep -qF "expected binaries not built for x86_64-unknown-linux-gnu: paladin" <<<"${LAST_OUTPUT}" \
    && ! grep -qF "paladin-cli" <<<"$(grep 'expected binaries not built' <<<"${LAST_OUTPUT}")"; then
    echo "PASS: an entry named 'paladin-cli' does not satisfy the expected entry 'paladin'"
else
    echo "FAIL: expected 'paladin' to be reported missing despite 'paladin-cli' being present"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- Case 8 (extra executable ignored): the manifest, not the directory
#     listing, decides membership -- an extra unexpected executable is
#     archived without it. -----------------------------------------------
DIR8="${SCRATCH}/case8"
mkdir -p "${DIR8}"
printf 'a' > "${DIR8}/paladin"
printf 'b' > "${DIR8}/paladin-cli"
printf 'c' > "${DIR8}/paladin-server"
printf 'd' > "${DIR8}/paladin-extra-tool"
run_script "${DIR8}" --target x86_64-unknown-linux-gnu --archive-name extra-archive

ASSERTIONS=$((ASSERTIONS + 1))
EXTRA_MEMBER_COUNT="$(tar tzf "${DIR8}/extra-archive.tar.gz" 2>/dev/null | wc -l | tr -d ' ')"
if [ "${LAST_STATUS}" -eq 0 ] && [ "${EXTRA_MEMBER_COUNT}" = "3" ]; then
    echo "PASS: an extra unexpected executable in the directory is not archived (member count == manifest length)"
else
    echo "FAIL: expected exactly 3 archive members (manifest length), got '${EXTRA_MEMBER_COUNT}' (status ${LAST_STATUS})"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- Case 9 (non-regular-file entry): a directory sharing a binary's name
#     counts as missing, not present. -------------------------------------
DIR9="${SCRATCH}/case9"
mkdir -p "${DIR9}"
mkdir -p "${DIR9}/paladin" # a directory, not a regular file, named like a binary
printf 'b' > "${DIR9}/paladin-cli"
printf 'c' > "${DIR9}/paladin-server"
run_script "${DIR9}" --target x86_64-unknown-linux-gnu --archive-name dir-entry-archive

ASSERTIONS=$((ASSERTIONS + 1))
if [ "${LAST_STATUS}" -ne 0 ] && grep -qF "expected binaries not built for x86_64-unknown-linux-gnu: paladin" <<<"${LAST_OUTPUT}" \
    && ! grep -qF "paladin-cli" <<<"$(grep 'expected binaries not built' <<<"${LAST_OUTPUT}")"; then
    echo "PASS: a directory sharing a binary's name counts as missing, not present"
else
    echo "FAIL: expected only 'paladin' (the directory entry) reported missing"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- Case 10 (deterministic ordering): two consecutive runs over identical
#     inputs produce the same tar tzf member order. -----------------------
DIR10A="${SCRATCH}/case10a"
DIR10B="${SCRATCH}/case10b"
mkdir -p "${DIR10A}" "${DIR10B}"
for d in "${DIR10A}" "${DIR10B}"; do
    printf 'a' > "${d}/paladin"
    printf 'b' > "${d}/paladin-cli"
    printf 'c' > "${d}/paladin-server"
done
run_script "${DIR10A}" --target x86_64-unknown-linux-gnu --archive-name order-archive
run_script "${DIR10B}" --target x86_64-unknown-linux-gnu --archive-name order-archive
tar tzf "${DIR10A}/order-archive.tar.gz" > "${SCRATCH}/order1.txt" 2>/dev/null
tar tzf "${DIR10B}/order-archive.tar.gz" > "${SCRATCH}/order2.txt" 2>/dev/null

ASSERTIONS=$((ASSERTIONS + 1))
if cmp -s "${SCRATCH}/order1.txt" "${SCRATCH}/order2.txt"; then
    echo "PASS: two consecutive runs over identical inputs produce the same tar tzf member order"
else
    echo "FAIL: member order differs between two runs over identical inputs"
    FAILED=$((FAILED + 1))
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
