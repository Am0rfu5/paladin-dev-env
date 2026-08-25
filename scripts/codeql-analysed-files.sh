#!/usr/bin/env bash
# codeql-analysed-files.sh
#
# D-13's evidence mechanism: extracts the analysed-.rs-file count from a
# CodeQL Rust run's own debug artifact, so "analysed 0 files" and "found 0
# issues" stay two independently-recorded numbers instead of collapsing into
# one report that reads as clean while having scanned nothing (the exact
# failure shape the Snyk evaluation exposed, see
# .github/instructions/security.instructions.md).
#
# Mechanism, confirmed empirically against the phase's own tracer run
# (18-01 Task 1) rather than assumed from documentation — RESEARCH.md's own
# Assumptions Log (A5) flagged this as unverified and requiring confirmation
# on the first real run:
#
#   * `debug: true` on `github/codeql-action/init` produces a run artifact
#     named `debug-artifacts`, downloadable with `gh run download`.
#   * That artifact's top level does NOT contain `src.zip` directly. It
#     contains `db-<language>.zip` -- the full CodeQL database archive for
#     that language (e.g. `db-rust.zip`) -- and `src.zip` lives NESTED
#     inside that database zip, at `<db-basename>/src.zip`. A script that
#     looks for `src.zip` at the artifact's top level finds nothing.
#   * `src.zip`'s entries are NOT 1:1 with "this repository's source tree".
#     It also archives the Rust toolchain's own standard-library source
#     (`home/runner/.rustup/toolchains/.../library/**/*.rs`, thousands of
#     files) and a handful of CodeQL's own bundled Rust builtins
#     (`opt/hostedtoolcache/CodeQL/**/*.rs`), alongside this repository's
#     actual checkout under `home/runner/work/<repo>/<repo>/**`. Reporting
#     the raw total entry count as "files analysed" would be its own
#     confusing-evidence defect -- a huge number dominated by vendored
#     stdlib source, not comparable to the 385 first-party denominator
#     D-13 names. This script scopes `analysed_rs_files` to the checkout's
#     own `crates/**/*.rs` and root `src/**/*.rs` -- the exact glob pair
#     that defines the 385 denominator -- and reports the raw archive
#     totals separately, clearly labelled, so neither number is silently
#     preferred over the other (per this task's own instruction not to).
#
# House style, matching scripts/check-advisory-register.sh and
# scripts/check-workflow-triggers.sh: offline apart from the single
# `gh run download` network call; writes only inside a `mktemp -d` working
# directory removed on exit; never mutates the tree; a missing run id, a
# run with no debug artifact, or an artifact with no nested `src.zip` is a
# named non-zero failure, never a silent zero.
#
# Usage:  ./scripts/codeql-analysed-files.sh <codeql-workflow-run-id>
# Exit:   0 and a machine-readable key=value block on stdout if the run's
#         debug artifact was downloaded and its nested src.zip parsed;
#         non-zero with a named failure message otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# The 385 denominator's own definition (D-13): crates/**/*.rs (246) + root
# src/**/*.rs (139), verified against this tree on 2026-08-25.
DENOMINATOR=385

# Three known feature-gated first-party paths (D-12's empirical probe list):
# web-server-gated, cli-gated, and the paladin-web crate's own default root.
FEATURE_GATED_PATHS=(
  "src/infrastructure/web/mod.rs"
  "src/application/cli/commands/agent.rs"
  "crates/paladin-web/src/lib.rs"
)

if [ "$#" -lt 1 ]; then
  echo "ERROR: missing required argument <codeql-workflow-run-id>." >&2
  echo "Usage: $0 <codeql-workflow-run-id>" >&2
  exit 1
fi
RUN_ID="$1"

if ! command -v gh >/dev/null 2>&1; then
  echo "ERROR: gh (GitHub CLI) is required to download the run's debug artifact." >&2
  exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo "ERROR: python3 is required for zip parsing." >&2
  exit 1
fi

# Repo slug: prefer the checkout's own origin remote so this script follows
# a fork/rename automatically; fall back to the repository's own documented
# slug (matching the literal form already used in
# docs/src/appendix/branch-protection.md's gh api examples) if origin is
# absent or unparseable. Overridable for the script's own test harness.
REPO_SLUG="${CODEQL_ANALYSED_FILES_REPO:-}"
if [ -z "${REPO_SLUG}" ]; then
  ORIGIN_URL="$(git -C "${WORKSPACE_ROOT}" remote get-url origin 2>/dev/null || true)"
  REPO_SLUG="$(printf '%s' "${ORIGIN_URL}" | sed -E 's#^(https://github\.com/|git@github\.com:)##; s#\.git$##')"
fi
if [ -z "${REPO_SLUG}" ]; then
  REPO_SLUG="DF3NDR/paladin-dev-env"
fi

TMPDIR="$(mktemp -d)"
trap 'rm -rf "${TMPDIR}"' EXIT

ARTIFACT_DIR="${TMPDIR}/artifact"
mkdir -p "${ARTIFACT_DIR}"

echo "Downloading debug artifact for run ${RUN_ID} (repo ${REPO_SLUG}) ..." >&2
if ! gh run download "${RUN_ID}" --repo "${REPO_SLUG}" --name debug-artifacts --dir "${ARTIFACT_DIR}" 2>"${TMPDIR}/gh-download.err"; then
  echo "ERROR: gh run download failed for run id '${RUN_ID}' (repo ${REPO_SLUG})." >&2
  echo "       This means the run does not exist, or produced no 'debug-artifacts'" >&2
  echo "       artifact (the init step's debug:true input is what creates it)." >&2
  echo "       gh's own error:" >&2
  sed 's/^/       /' "${TMPDIR}/gh-download.err" >&2
  exit 1
fi

# Locate the single per-language database zip (db-<language>.zip) at the
# artifact's top level. Fail loudly, never guess, if there are zero or more
# than one -- either is a shape this script has not seen and should not
# silently pick a winner for.
mapfile -t DB_ZIPS < <(find "${ARTIFACT_DIR}" -maxdepth 1 -type f -name 'db-*.zip' | sort)
if [ "${#DB_ZIPS[@]}" -eq 0 ]; then
  echo "ERROR: no 'db-<language>.zip' file found at the top level of the downloaded" >&2
  echo "       debug artifact for run '${RUN_ID}'. Contents:" >&2
  find "${ARTIFACT_DIR}" -maxdepth 2 | sed 's/^/       /' >&2
  exit 1
fi
if [ "${#DB_ZIPS[@]}" -gt 1 ]; then
  echo "ERROR: multiple 'db-<language>.zip' files found for run '${RUN_ID}' -- this" >&2
  echo "       script assumes a single-language (rust) scan and refuses to guess" >&2
  echo "       which database to read. Found:" >&2
  printf '       %s\n' "${DB_ZIPS[@]}" >&2
  exit 1
fi
DB_ZIP="${DB_ZIPS[0]}"

# Extract the nested src.zip from inside the database zip, and run the
# actual entry-count analysis, all in one python3 pass so no intermediate
# unzipped tree is left behind. FEATURE_GATED_PATHS is passed as extra argv
# so this stays a single hermetic invocation.
python3 - "${DB_ZIP}" "${RUN_ID}" "${DENOMINATOR}" "${FEATURE_GATED_PATHS[@]}" <<'PY'
import re
import sys
import zipfile

db_zip_path = sys.argv[1]
run_id = sys.argv[2]
denominator = int(sys.argv[3])
feature_gated_paths = sys.argv[4:]

try:
    db_zip = zipfile.ZipFile(db_zip_path)
except (OSError, zipfile.BadZipFile) as exc:
    print(f"ERROR: could not open database zip {db_zip_path!r}: {exc}", file=sys.stderr)
    sys.exit(1)

src_zip_entries = [n for n in db_zip.namelist() if n.endswith('src.zip')]
if not src_zip_entries:
    print(f"ERROR: no nested 'src.zip' entry found inside {db_zip_path!r} -- "
          f"this run's debug artifact does not carry the D-13 evidence mechanism "
          f"this script depends on.", file=sys.stderr)
    sys.exit(1)
if len(src_zip_entries) > 1:
    print(f"ERROR: multiple nested entries ending in 'src.zip' found inside "
          f"{db_zip_path!r}, refusing to guess which one: {src_zip_entries!r}",
          file=sys.stderr)
    sys.exit(1)

with db_zip.open(src_zip_entries[0]) as fh:
    src_zip_bytes = fh.read()

import io
try:
    src_zip = zipfile.ZipFile(io.BytesIO(src_zip_bytes))
except zipfile.BadZipFile as exc:
    print(f"ERROR: nested src.zip at {src_zip_entries[0]!r} did not parse as a zip: {exc}",
          file=sys.stderr)
    sys.exit(1)

names = src_zip.namelist()
total_entries = len(names)
if total_entries == 0:
    print("ERROR: nested src.zip contains zero entries -- treating this as a named "
          "failure rather than reporting a silent zero-files-analysed count.",
          file=sys.stderr)
    sys.exit(1)

# Checkout-root prefix: GitHub Actions runners check out to
# /home/runner/work/<repo>/<repo>/ -- capture the repo-relative path without
# hardcoding the repo name, since it appears twice in the prefix.
checkout_re = re.compile(r'^home/runner/work/([^/]+)/\1/(.*)$')
repo_relative = {}
toolchain_count = 0
other_vendored_count = 0
for n in names:
    if not n.endswith('.rs'):
        continue
    m = checkout_re.match(n)
    if m:
        repo_relative[m.group(2)] = n
    elif '.rustup/' in n:
        toolchain_count += 1
    else:
        other_vendored_count += 1

total_rs_entries = sum(1 for n in names if n.endswith('.rs'))

# analysed_rs_files: scoped to exactly the two globs that define the 385
# denominator (crates/**/*.rs, root src/**/*.rs) -- the only scoping that
# makes `difference` a meaningful number rather than an artifact of
# archiving the whole Rust toolchain alongside the checkout.
denominator_scoped = [
    p for p in repo_relative
    if p.startswith('crates/') or p.startswith('src/')
]
analysed_rs_files = len(denominator_scoped)
difference = denominator - analysed_rs_files

probe_fixture_entries = sum(1 for p in repo_relative if p.startswith('fixtures/codeql-probe/'))

print(f"run_id={run_id}")
print(f"analysed_rs_files={analysed_rs_files}")
print(f"denominator={denominator}")
print(f"difference={difference}")
print(f"probe_fixture_entries={probe_fixture_entries}")
for path in feature_gated_paths:
    present = 'yes' if path in repo_relative else 'no'
    print(f"feature_gated_present={path}:{present}")

# Transparency fields (Rule 2 -- never let one number stand in silently for
# a materially different one): the raw archive totals, so a small
# analysed_rs_files is never mistaken for "src.zip only had a handful of
# files in it" when the archive in fact carries thousands of vendored
# toolchain entries.
print(f"src_zip_total_rs_entries={total_rs_entries}")
print(f"src_zip_checkout_rs_entries={len(repo_relative)}")
print(f"src_zip_toolchain_stdlib_rs_entries={toolchain_count}")
print(f"src_zip_other_vendored_rs_entries={other_vendored_count}")

print(
    "note: analysed_rs_files counts src.zip entries under this checkout's own "
    "crates/**/*.rs and root src/**/*.rs (the 385 denominator's own scope), not "
    "src.zip's raw total -- src.zip also archives the Rust toolchain's standard "
    "library source and a few CodeQL-bundled Rust builtins alongside the "
    "checkout, so the raw total (src_zip_total_rs_entries) is not comparable to "
    "the denominator. This is evidence that CodeQL's internal rust-analyzer pass "
    "archived these files, not proof it produced complete per-file database "
    "facts for each one -- cross-check against 'codeql database print-baseline' "
    "was attempted against this run's own logs and found no such output; no "
    "print-baseline invocation appears in the debug artifact's log files, so no "
    "disagreement could be recorded either way (see 18-CODEQL-EVIDENCE.md)."
)
PY
