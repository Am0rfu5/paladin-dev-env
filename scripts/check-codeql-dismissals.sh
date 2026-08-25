#!/usr/bin/env bash
# check-codeql-dismissals.sh
#
# Enforces CODEQL-DISMISSALS.md (the governance register for dismissed
# CodeQL code-scanning alerts) is internally consistent and not stale.
# `check-advisory-register.sh` and `check-workflow-triggers.sh` already
# proved this register-plus-guard shape (ADR-0036); this script applies the
# identical discipline to CodeQL alert-dismissal governance (T-18-16..20).
#
# SCOPE, STATED PLAINLY: this guard validates the register's internal
# consistency and staleness ONLY. It makes no network call and cannot and
# does not verify that the register matches GitHub's live dismissed-alert
# set -- that reconciliation is the documented manual `gh api` command in
# the register's own header (CODEQL-DISMISSALS.md), a human step this guard
# does not perform. A green run here means "the register is well-formed and
# not stale," never "this matches what GitHub currently shows."
#
# This script is offline and makes no network call. It parses the register's
# fenced TOML payload structurally with tomllib -- it never scrapes prose --
# and accumulates every violation found rather than stopping at the first.
# It only reads: it writes nothing and creates no temporary file, so running
# it twice in succession, with no change to any input, produces identical
# output and the same exit code.
#
# Five clauses are asserted, all accumulated into one shared failure list
# before the verdict is decided:
#
#   1. Schema. Every entry carries all eleven required fields
#      (alert_number, rule_id, path, why_present, why_dismissed,
#      dismissed_reason, owner, review_date, scope, compensating_control,
#      revisit_condition), each present and non-empty.
#   2. Drift. The header's declared dismissal count ("Declared dismissals:
#      N") equals the number of structured entries -- the exact defect
#      class the v0.8.0 audit found in SECURITY-EXCEPTIONS.md, where the
#      prose count silently drifted from the payload and no gate noticed.
#   3. Staleness. Every review_date parses as ISO-8601 (YYYY-MM-DD) and is
#      not earlier than today -- a governed dismissal whose review date has
#      passed is an ungoverned dismissal wearing a date.
#   4. Uniqueness. No alert_number appears twice.
#   5. Reachability. Every entry's `path` (optionally suffixed `:LINE`)
#      either names a file that exists under the repository root, or the
#      entry's `scope` field explicitly records that the file was removed
#      and why -- a row pointing at nothing is stale bookkeeping, the same
#      staleness discipline check-advisory-register.sh's Clause 3 applies
#      against Cargo.lock.
#
# A missing register file is a named non-zero failure, never a silently-empty
# comparison. An empty register (zero entries) whose declared count is also
# zero is a legitimate pass, and the output says so explicitly rather than
# reporting success over nothing.
#
# Usage:  ./scripts/check-codeql-dismissals.sh [register-path] [repo-root]
#         The optional positional arguments override, in order, the register
#         file to check (default: <repo-root>/CODEQL-DISMISSALS.md) and the
#         repository root the reachability clause resolves relative `path`
#         entries against (default: this script's own repo root). They exist
#         so this guard's own regression test can point it at fixtures
#         without ever mutating the real tree.
# Exit:   0 if the register is well-formed, self-consistent and not stale;
#         non-zero otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REGISTER="${1:-${WORKSPACE_ROOT}/CODEQL-DISMISSALS.md}"
REPO_ROOT="${2:-${WORKSPACE_ROOT}}"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for TOML parsing." >&2
    exit 1
fi

echo "🔍 Checking the CodeQL dismissal register for schema, drift, staleness, uniqueness and reachability ..."

REPORT=$(python3 - "${REGISTER}" "${REPO_ROOT}" <<'PY'
import datetime
import os
import re
import sys
import tomllib

register_path, repo_root = sys.argv[1:3]

failures = []

# --- Missing-input handling: never default a missing register to empty. ---
if not os.path.isfile(register_path):
    print("MISSING_REGISTER")
    print(f"FAIL: required register file is absent: {register_path}")
    sys.exit(0)

BEGIN_MARKER = "<!-- BEGIN MACHINE-READABLE REGISTER -->"
END_MARKER = "<!-- END MACHINE-READABLE REGISTER -->"

register_text = open(register_path, encoding="utf-8").read()

begin_count = register_text.count(BEGIN_MARKER)
end_count = register_text.count(END_MARKER)

if begin_count != 1 or end_count != 1:
    print("MARKER_ERROR")
    print(f"FAIL: expected exactly one BEGIN and one END marker in {register_path}, "
          f"found {begin_count} BEGIN marker(s) and {end_count} END marker(s).")
    sys.exit(0)

begin_idx = register_text.index(BEGIN_MARKER) + len(BEGIN_MARKER)
end_idx = register_text.index(END_MARKER)
between = register_text[begin_idx:end_idx]

fence_matches = re.findall(r"```toml\n(.*?)\n```", between, re.DOTALL)
if len(fence_matches) != 1:
    print("MARKER_ERROR")
    print("FAIL: expected exactly one fenced ```toml block between the BEGIN and END "
          f"markers, found {len(fence_matches)}.")
    sys.exit(0)

register_toml_text = fence_matches[0]

try:
    register_data = tomllib.loads(register_toml_text)
except tomllib.TOMLDecodeError as exc:
    print("MARKER_ERROR")
    print(f"FAIL: the fenced block between the markers did not parse as TOML: {exc}")
    sys.exit(0)

rows = register_data.get("dismissal", [])

# --- Declared count. ---
m = re.search(r"Declared dismissals:\s*([0-9]+)", register_text)
if not m:
    print("MARKER_ERROR")
    print(f"FAIL: no 'Declared dismissals: N' line found in {register_path}.")
    sys.exit(0)
declared = int(m.group(1))

# --- An empty register with declared count 0 is a legitimate pass, stated
# explicitly rather than reported as success over nothing. ---
if declared == 0 and not rows:
    print("OK")
    print("0 governed dismissal(s) declared and 0 entries found -- an empty register "
          "with an honest declared count is a valid state.")
    sys.exit(0)

REQUIRED_FIELDS = [
    "alert_number", "rule_id", "path", "why_present", "why_dismissed",
    "dismissed_reason", "owner", "review_date", "scope",
    "compensating_control", "revisit_condition",
]

# --- Clause 2: drift. Declared count must equal the number of entries. ---
if declared != len(rows):
    failures.append(
        f"CLAUSE_DRIFT: header declares {declared} dismissal(s) but the register "
        f"payload holds {len(rows)} entr{'y' if len(rows) == 1 else 'ies'}."
    )

# --- Clause 1: schema. Every entry carries all eleven fields, non-empty. ---
for idx, row in enumerate(rows):
    alert_label = row.get("alert_number", f"<row {idx}>")
    missing = [f for f in REQUIRED_FIELDS if str(row.get(f, "")).strip() == ""]
    if missing:
        failures.append(
            f"CLAUSE_SCHEMA: dismissal entry (alert_number={alert_label!r}) is missing/blank "
            f"field(s): {', '.join(missing)}."
        )

# --- Clause 3: staleness. review_date must be ISO-8601 and not in the past. ---
today = datetime.date.today()
for row in rows:
    alert_label = row.get("alert_number", "<unknown>")
    review_date = str(row.get("review_date", "")).strip()
    if not review_date:
        continue  # already reported by CLAUSE_SCHEMA above
    if not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", review_date):
        failures.append(
            f"CLAUSE_STALENESS: dismissal entry (alert_number={alert_label!r}) has a "
            f"review_date {review_date!r} that is not ISO-8601 (YYYY-MM-DD)."
        )
        continue
    try:
        parsed = datetime.date.fromisoformat(review_date)
    except ValueError:
        failures.append(
            f"CLAUSE_STALENESS: dismissal entry (alert_number={alert_label!r}) has a "
            f"review_date {review_date!r} that does not parse as a real calendar date."
        )
        continue
    if parsed < today:
        failures.append(
            f"CLAUSE_STALENESS: dismissal entry (alert_number={alert_label!r}) has a "
            f"review_date {review_date!r} that has already passed (today is "
            f"{today.isoformat()}) -- a governed dismissal must be re-argued on a date, "
            f"not expire into silence."
        )

# --- Clause 4: uniqueness. No alert_number appears twice. ---
seen = {}
for row in rows:
    num = row.get("alert_number")
    seen.setdefault(num, 0)
    seen[num] += 1
for num, count in seen.items():
    if count > 1:
        failures.append(
            f"CLAUSE_UNIQUENESS: alert_number {num!r} appears {count} times in the register "
            "-- each dismissed alert must have exactly one governing row."
        )

# --- Clause 5: reachability. path exists under repo_root, or scope records
# the file's removal explicitly. ---
for row in rows:
    alert_label = row.get("alert_number", "<unknown>")
    path_field = str(row.get("path", "")).strip()
    if not path_field:
        continue  # already reported by CLAUSE_SCHEMA above
    # Strip an optional trailing ":LINE" (or ":LINE:COL") location suffix.
    file_part = re.sub(r"(:[0-9]+){1,2}$", "", path_field)
    resolved = os.path.join(repo_root, file_part)
    if os.path.exists(resolved):
        continue
    scope_field = str(row.get("scope", "")).strip().lower()
    if "removed" in scope_field:
        continue
    failures.append(
        f"CLAUSE_REACHABILITY: dismissal entry (alert_number={alert_label!r})'s path "
        f"{path_field!r} does not exist under {repo_root}, and its scope field does not "
        "explicitly record that the file was removed and why -- a row pointing at "
        "nothing is stale bookkeeping."
    )

if failures:
    print("FAIL")
    for f in failures:
        print(f"FAIL: {f}")
    sys.exit(0)

print("OK")
print(f"{len(rows)} governed dismissal(s) checked; schema, drift, staleness, uniqueness "
      "and reachability clauses all pass.")
sys.exit(0)
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ CodeQL dismissal register check failed (${STATUS_LINE})"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "This guard only validates the register's internal consistency and staleness --"
    echo "it does not and cannot compare against GitHub's live dismissed-alert set. Run"
    echo "the reconciliation command in CODEQL-DISMISSALS.md's header for that."
    echo ""
    echo "If this failure is unexpected:"
    echo "  1. CLAUSE_SCHEMA: fill in the missing field(s) on the named entry."
    echo "  2. CLAUSE_DRIFT: update 'Declared dismissals: N' to match the entry count,"
    echo "     or add/remove an entry so the two agree."
    echo "  3. CLAUSE_STALENESS: re-review the dismissal and set a future review_date,"
    echo "     or remove the dismissal and re-open the alert on the platform."
    echo "  4. CLAUSE_UNIQUENESS: merge or renumber the duplicate alert_number entries."
    echo "  5. CLAUSE_REACHABILITY: if the file was intentionally removed, say so in the"
    echo "     entry's scope field; otherwise correct the path."
    exit 1
fi
