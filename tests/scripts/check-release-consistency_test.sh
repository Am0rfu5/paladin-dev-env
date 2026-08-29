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
# excludes, matching `paladin-doc-examples` in the real tree). Each package
# also gets a real scratch directory and a `manifest_path` pointing into it
# (`${SCRATCH}/pkg-manifests/<name>/Cargo.toml`), because clause 2 resolves
# a package's changelog as `CHANGELOG.md` in `dirname(manifest_path)` --
# `write_changelog_fixture` below writes into that same directory, so the
# two helpers agree on where a package's changelog lives regardless of call
# order.
write_metadata_fixture() {
    local file="$1"
    shift
    local entries=() pair name rest version publish_json pkg_dir manifest_path
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
        pkg_dir="${SCRATCH}/pkg-manifests/${name}"
        mkdir -p "${pkg_dir}"
        manifest_path="${pkg_dir}/Cargo.toml"
        : > "${manifest_path}"
        entries+=("{\"name\": \"${name}\", \"version\": \"${version}\", \"publish\": ${publish_json}, \"manifest_path\": \"${manifest_path}\"}")
    done
    local joined
    joined="$(IFS=,; echo "${entries[*]:-}")"
    printf '{"packages": [%s]}\n' "${joined}" > "${file}"
}

# write_changelog_fixture NAME CONTENT -> writes CONTENT to
# `${SCRATCH}/pkg-manifests/<NAME>/CHANGELOG.md`, the same per-package
# directory `write_metadata_fixture` derives that package's `manifest_path`
# from. Deliberately does not require `write_metadata_fixture` to have run
# first for the same name -- a case that wants "no CHANGELOG.md at all"
# simply never calls this helper for that package.
write_changelog_fixture() {
    local name="$1" content="$2"
    local pkg_dir="${SCRATCH}/pkg-manifests/${name}"
    mkdir -p "${pkg_dir}"
    printf '%s' "${content}" > "${pkg_dir}/CHANGELOG.md"
}

# write_ci_runs_fixture FILE RUN... -> writes a minimal
# `{"workflow_runs": [...]}` document in the shape the
# `actions/workflows/{file}/runs` gh api endpoint returns -- the fixture
# seam --ci-runs-json reads instead of calling the API, so this exercises
# the same parsing code path a live gh api response takes. Each RUN is
# "created_at|id|conclusion". Zero RUN arguments writes an empty array
# (the "no completed run" case).
write_ci_runs_fixture() {
    local file="$1"
    shift
    local entries=() run created_at id conclusion
    for run in "$@"; do
        IFS='|' read -r created_at id conclusion <<<"${run}"
        entries+=("{\"created_at\": \"${created_at}\", \"id\": ${id}, \"conclusion\": \"${conclusion}\"}")
    done
    local joined
    joined="$(IFS=,; echo "${entries[*]:-}")"
    printf '{"workflow_runs": [%s]}\n' "${joined}" > "${file}"
}

# run_guard ARGS... -> sets $LAST_OUTPUT and $LAST_STATUS.
run_guard() {
    # Neutralize the ambient CI marker: when this harness itself runs inside
    # GitHub Actions (ci.yml's deny job runs `make test-shell-guards`), the
    # runner's GITHUB_ACTIONS=true would leak into every fixture invocation and
    # trip the guard's fail-closed MISSING_SHA clause (D-10), failing all
    # clause-1/2 cases that legitimately omit --sha. The MISSING_SHA behavior
    # itself is still covered by dedicated cases below that force
    # GITHUB_ACTIONS=true / GITHUB_ACTIONS='' via inline prefixes without
    # using this helper. Found live on PR #43's deny job (20-07 finding 4).
    LAST_OUTPUT="$(env -u GITHUB_ACTIONS "${GUARD}" "$@" 2>&1)"
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
# Clause 2 now applies to every publishable package, including these
# pre-clause-2 fixtures reused by assert_silent cases below -- give each a
# changelog section matching 1.2.3 so those cases stay silent on clause 2
# too, and only clause 1 (mismatch) fires where the test intends it to.
write_changelog_fixture pkg-a $'## [1.2.3] - 2026-01-01\n'
write_changelog_fixture pkg-b $'## [1.2.3] - 2026-01-01\n'
write_changelog_fixture pkg-c $'## [1.2.3] - 2026-01-01\n'

MIXED_FIXTURE="${SCRATCH}/mixed.json"
write_metadata_fixture "${MIXED_FIXTURE}" pkg-a=1.2.3 pkg-b=1.2.3-rc.1

EMPTY_FIXTURE="${SCRATCH}/empty.json"
write_metadata_fixture "${EMPTY_FIXTURE}"

EXEMPT_ONLY_FIXTURE="${SCRATCH}/exempt-only.json"
write_metadata_fixture "${EXEMPT_ONLY_FIXTURE}" doc-examples-pkg=1.2.3:false

SINGLE_FIXTURE="${SCRATCH}/single.json"
write_metadata_fixture "${SINGLE_FIXTURE}" pkg-solo=1.2.3
write_changelog_fixture pkg-solo $'## [1.2.3] - 2026-01-01\n'

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

# =============================================================================
# Clause 2: changelog-section agreement (D-08 clause 2, plan 20-02 Task 1).
# =============================================================================

# --- A. Heading matches the tag exactly ("## [1.2.3] - 2026-01-01" for
#        --tag v1.2.3"): silent. Also exercises clause 1 passing at the same
#        time (package is at version 1.2.3), so this pins clauses 1+2 both
#        green together. --------------------------------------------------
write_metadata_fixture "${SCRATCH}/cl2-a-meta.json" pkg-cl2-a=1.2.3
write_changelog_fixture pkg-cl2-a $'# Changelog\n\n## [Unreleased]\n\n## [1.2.3] - 2026-01-01\n'
assert_silent "changelog section present and matches the tag exactly" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-a-meta.json"

# --- B. Changelog exists but carries only "## [Unreleased]": fires, and the
#        message names both the file path and the missing version. --------
write_metadata_fixture "${SCRATCH}/cl2-b-meta.json" pkg-cl2-b=1.2.3
write_changelog_fixture pkg-cl2-b $'# Changelog\n\n## [Unreleased]\n'
assert_fire "changelog with only Unreleased fires, naming the file path" \
    "pkg-manifests/pkg-cl2-b/CHANGELOG.md" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-b-meta.json"
assert_fire "changelog with only Unreleased fires, naming the missing version" \
    "no section for version" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-b-meta.json"

# --- C. No CHANGELOG.md file at all: fires, with a message distinguishable
#        from case B's ("not found" vs "no section for version"). ---------
write_metadata_fixture "${SCRATCH}/cl2-c-meta.json" pkg-cl2-c=1.2.3
assert_fire "changelog missing entirely fires with a distinct 'not found' message" \
    "not found" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-c-meta.json"

# --- D. Heading "## [1.2.3-rc.1]" against --tag v1.2.3: fires -- a
#        prerelease variant does not satisfy a stable tag's exact bracketed
#        version. --------------------------------------------------------
write_metadata_fixture "${SCRATCH}/cl2-d-meta.json" pkg-cl2-d=1.2.3
write_changelog_fixture pkg-cl2-d $'## [1.2.3-rc.1] - 2026-01-01\n'
assert_fire "heading with a prerelease suffix does not satisfy a stable tag" \
    "pkg-cl2-d" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-d-meta.json"

# --- E. Heading "## [1.2.30]" against --tag v1.2.3: fires -- no prefix
#        matching on the bracketed version. -------------------------------
write_metadata_fixture "${SCRATCH}/cl2-e-meta.json" pkg-cl2-e=1.2.3
write_changelog_fixture pkg-cl2-e $'## [1.2.30] - 2026-01-01\n'
assert_fire "a longer version sharing a prefix does not satisfy the tag" \
    "pkg-cl2-e" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-e-meta.json"

# --- F. Two packages each missing a section: both are reported in one run,
#        in package-name order. --------------------------------------------
write_metadata_fixture "${SCRATCH}/cl2-f-meta.json" pkg-cl2-f1=1.2.3 pkg-cl2-f2=1.2.3
write_changelog_fixture pkg-cl2-f1 $'## [Unreleased]\n'
write_changelog_fixture pkg-cl2-f2 $'## [Unreleased]\n'
assert_fire "two packages missing sections in one run: f1 reported" "pkg-cl2-f1" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-f-meta.json"
assert_fire "two packages missing sections in one run: f2 reported" "pkg-cl2-f2" \
    --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-f-meta.json"

ASSERTIONS=$((ASSERTIONS + 1))
run_guard --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-f-meta.json"
pos_f1=$(grep -n "pkg-cl2-f1" <<<"${LAST_OUTPUT}" | head -n1 | cut -d: -f1)
pos_f2=$(grep -n "pkg-cl2-f2" <<<"${LAST_OUTPUT}" | head -n1 | cut -d: -f1)
if [ -n "${pos_f1}" ] && [ -n "${pos_f2}" ] && [ "${pos_f1}" -lt "${pos_f2}" ]; then
    echo "PASS (ordering): two packages missing sections are reported in package-name order"
else
    echo "FAIL: two packages missing sections were not reported in package-name order"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- G. Clause 1 (manifest mismatch) and clause 2 (changelog) are
#        independent: a package failing only clause 2 must not also report
#        a manifest-version mismatch. -------------------------------------
ASSERTIONS=$((ASSERTIONS + 1))
run_guard --tag v1.2.3 --metadata-json "${SCRATCH}/cl2-b-meta.json"
if [ "${LAST_STATUS}" -ne 0 ] && ! grep -qF -- "manifest version" <<<"${LAST_OUTPUT}"; then
    echo "PASS (independence): a changelog-only failure reports no manifest-version mismatch"
else
    echo "FAIL: a changelog-only failure unexpectedly reported a manifest-version mismatch"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# =============================================================================
# Clause 3: CI-conclusion agreement for the tagged SHA (D-10, PUBOPS-02,
# plan 20-02 Task 2). Fixture-driven throughout via --ci-runs-json, so no
# assertion in this section touches the network.
# =============================================================================

CI_SHA="deadbeef0000"

# --- 1. One completed run, conclusion success: silent. ---------------------
write_ci_runs_fixture "${SCRATCH}/ci-1-success.json" "2026-01-01T00:00:00Z|1|success"
assert_silent "one completed run, conclusion success, is silent" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-1-success.json"

# --- 2. One completed run, conclusion failure: fires, names the SHA and
#        tells the operator to re-run CI or re-tag. --------------------------
write_ci_runs_fixture "${SCRATCH}/ci-2-failure.json" "2026-01-01T00:00:00Z|1|failure"
assert_fire "one completed run, conclusion failure, fires naming the SHA" "${CI_SHA}" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-2-failure.json"
assert_fire "one completed run, conclusion failure, tells the operator to re-run CI" "re-run CI on main" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-2-failure.json"

# --- 3. Two completed runs for the same SHA, older success, newer failure:
#        fires -- the most recent run decides. ------------------------------
write_ci_runs_fixture "${SCRATCH}/ci-3-mostrecent.json" \
    "2026-01-01T00:00:00Z|1|success" \
    "2026-01-02T00:00:00Z|2|failure"
assert_fire "older success + newer failure: the most recent run decides" "concluded 'failure'" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-3-mostrecent.json"

# --- 4. Two completed runs with identical created_at, differing id: the
#        higher id decides, order-independent of array position. -----------
write_ci_runs_fixture "${SCRATCH}/ci-4-tiebreak.json" \
    "2026-01-01T00:00:00Z|1|success" \
    "2026-01-01T00:00:00Z|2|failure"
assert_fire "identical created_at, higher id (failure) decides" "concluded 'failure'" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-4-tiebreak.json"

write_ci_runs_fixture "${SCRATCH}/ci-4b-tiebreak-reversed.json" \
    "2026-01-01T00:00:00Z|2|failure" \
    "2026-01-01T00:00:00Z|1|success"
assert_fire "tie-break is array-order-independent: reversed order still picks higher id" "concluded 'failure'" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-4b-tiebreak-reversed.json"

# --- 5. Empty runs array: fires with a message saying no completed run was
#        found for the SHA. --------------------------------------------------
write_ci_runs_fixture "${SCRATCH}/ci-5-empty.json"
assert_fire "empty runs array fires with a 'no completed run' message" "no completed ci.yml run found" \
    --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" --sha "${CI_SHA}" \
    --ci-runs-json "${SCRATCH}/ci-5-empty.json"

# --- 6. --sha absent with GITHUB_ACTIONS=true: MISSING_SHA, non-zero -- the
#        CI-conclusion clause can never be silently absent on the CI path.
#        GITHUB_ACTIONS is forced via an inline prefix (not export/unset) so
#        this assertion is correct whether or not the harness itself is
#        currently running inside real GitHub Actions. ----------------------
ASSERTIONS=$((ASSERTIONS + 1))
LAST_OUTPUT="$(GITHUB_ACTIONS=true "${GUARD}" --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" 2>&1)"
LAST_STATUS=$?
if [ "${LAST_STATUS}" -ne 0 ] && grep -qF -- "MISSING_SHA" <<<"${LAST_OUTPUT}"; then
    echo "PASS (fire): GITHUB_ACTIONS=true with no --sha fires MISSING_SHA"
else
    echo "FAIL: GITHUB_ACTIONS=true with no --sha did not fire MISSING_SHA (status=${LAST_STATUS})"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
    FAILED=$((FAILED + 1))
fi

# --- 7. --sha absent with GITHUB_ACTIONS unset (forced empty via inline
#        prefix): the offline clauses still run and the report states the
#        CI-conclusion clause was not checked; the exit code reflects the
#        offline clauses alone (here: pass, since ALL_MATCH_FIXTURE's
#        clauses 1+2 are green). --------------------------------------------
ASSERTIONS=$((ASSERTIONS + 1))
LAST_OUTPUT="$(GITHUB_ACTIONS='' "${GUARD}" --tag v1.2.3 --metadata-json "${ALL_MATCH_FIXTURE}" 2>&1)"
LAST_STATUS=$?
if [ "${LAST_STATUS}" -eq 0 ] && grep -qF -- "was not checked" <<<"${LAST_OUTPUT}"; then
    echo "PASS (silent+note): no --sha, not in GitHub Actions -- offline clauses pass, CI clause stated not-checked"
else
    echo "FAIL: no --sha / GITHUB_ACTIONS unset did not pass with a not-checked note (status=${LAST_STATUS})"
    echo "${LAST_OUTPUT}" | sed 's/^/  | /'
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
