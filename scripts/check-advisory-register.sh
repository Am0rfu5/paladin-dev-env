#!/usr/bin/env bash
# check-advisory-register.sh
#
# Enforces that SECURITY-EXCEPTIONS.md (the governance register), deny.toml
# and .cargo/audit.toml (the two mechanical suppression surfaces) and
# Cargo.lock (the live dependency graph) all agree, per ADR-0024. Three
# clauses are asserted, and every failure found is reported rather than
# stopping at the first:
#
#   1. Class-set equality, exactly. One register class partition must equal
#      .cargo/audit.toml's ignore set exactly; the union of every register
#      class partition must equal deny.toml's ignore set exactly. Comparison
#      is over sets (order-insensitive) of raw strings (case-sensitive, so an
#      identifier differing only in letter case fails). Which partition
#      corresponds to which TOML file is discovered structurally by set
#      equality -- this script never hardcodes a class-name literal to decide
#      the partition, so it cannot depend on a class label's exact spelling.
#   2. Register coverage, both directions. Every identifier suppressed in
#      either TOML file must have a register row with all eleven schema
#      fields present and non-empty. Every register row must correspond to a
#      live suppression in at least one TOML file -- a row with no matching
#      suppression is stale bookkeeping, not documented risk.
#   3. Crate liveness. Every register row's `crate` field (comma-separated if
#      more than one candidate crate applies along a transitive path) must
#      name at least one crate present in Cargo.lock, checked with the same
#      anchored per-crate form `grep -c '^name = "<crate>"$'` uses. A
#      suppression whose crate has left the dependency graph is deleted, not
#      backfilled with governance.
#
# A one-sided-empty case is also asserted: the register parsing to zero rows
# while either TOML file holds a non-empty ignore array is its own distinct
# failure. Zero rows and two empty arrays is a legitimate pass.
#
# Class information comes ONLY from the register's `class` field, read via
# tomllib. This script never scrapes either TOML file's inline comment
# wording to recover class information -- a script that depends on exact
# comment prose breaks silently the next time someone rewords it.
#
# A missing SECURITY-EXCEPTIONS.md, deny.toml, .cargo/audit.toml or
# Cargo.lock is a named non-zero failure, never a silently-empty comparison.
#
# This script only reads; it writes nothing and creates no temporary file.
# Running it twice in succession, with no change to any input, produces
# identical output and the same exit code.
#
# Usage:  ./scripts/check-advisory-register.sh
# Exit:   0 if the register and both suppression files agree on all clauses;
#         non-zero otherwise.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REGISTER="${WORKSPACE_ROOT}/SECURITY-EXCEPTIONS.md"
DENY_TOML="${WORKSPACE_ROOT}/deny.toml"
AUDIT_TOML="${WORKSPACE_ROOT}/.cargo/audit.toml"
CARGO_LOCK="${WORKSPACE_ROOT}/Cargo.lock"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for TOML parsing." >&2
    exit 1
fi

echo "🔍 Checking the advisory exception register against deny.toml, .cargo/audit.toml and Cargo.lock ..."

REPORT=$(python3 - "${REGISTER}" "${DENY_TOML}" "${AUDIT_TOML}" "${CARGO_LOCK}" <<'PY'
import os
import re
import sys
import tomllib

register_path, deny_path, audit_path, lock_path = sys.argv[1:5]

failures = []

# --- Missing-input handling: never default a missing file to an empty set. ---
missing = []
for label, path in (
    ("SECURITY-EXCEPTIONS.md", register_path),
    ("deny.toml", deny_path),
    (".cargo/audit.toml", audit_path),
    ("Cargo.lock", lock_path),
):
    if not os.path.isfile(path):
        missing.append(label)

if missing:
    print("MISSING_INPUT")
    for label in missing:
        print(f"FAIL: required input file is absent: {label}")
    sys.exit(0)

# --- Extract the register's fenced TOML payload between explicit markers. ---
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

rows = register_data.get("exception", [])

# --- Parse deny.toml and .cargo/audit.toml with tomllib. Never with a regex
# over comments, and never by scraping either file's inline comment text to
# recover class information. ---
with open(deny_path, "rb") as fh:
    deny_data = tomllib.load(fh)
with open(audit_path, "rb") as fh:
    audit_data = tomllib.load(fh)

deny_ignore = set(deny_data.get("advisories", {}).get("ignore", []))
audit_ignore = set(audit_data.get("advisories", {}).get("ignore", []))

# --- One-sided-empty case: zero register rows but a non-empty ignore array
# somewhere is its own distinct failure. Zero rows and two empty arrays is a
# legitimate pass. ---
if not rows and (deny_ignore or audit_ignore):
    print("ONE_SIDED_EMPTY")
    print("FAIL: the register parses to zero rows, but at least one configuration "
          "file holds a non-empty ignore array.")
    if deny_ignore:
        print(f"  deny.toml ignore: {sorted(deny_ignore)}")
    if audit_ignore:
        print(f"  .cargo/audit.toml ignore: {sorted(audit_ignore)}")
    sys.exit(0)

GOVERNANCE_FIELDS = [
    "id", "class", "crate", "path", "why_present", "why_not_fixable",
    "owner", "review_date", "scope", "compensating_control", "revisit_condition",
]

# --- Clause 1: class-set equality, exactly. Partition register rows by their
# `class` field value -- discovered structurally, never by hardcoding a
# class-name literal to decide which partition is "the vulnerability one". A
# partition equalling .cargo/audit.toml's ignore set exactly satisfies the
# first half of this clause; the union of every partition must equal
# deny.toml's ignore set exactly. ---
class_groups = {}
for row in rows:
    cls = row.get("class")
    class_groups.setdefault(cls, set()).add(row.get("id"))

audit_class_match = any(ids == audit_ignore for ids in class_groups.values())
if not audit_class_match:
    partitions_str = "; ".join(
        f"{cls!r}: {sorted(ids)}" for cls, ids in class_groups.items()
    )
    failures.append(
        "CLAUSE1_AUDIT_MISMATCH: no register class partition equals .cargo/audit.toml's "
        f"ignore set exactly. .cargo/audit.toml ignore={sorted(audit_ignore)}; register "
        f"class partitions: {partitions_str}."
    )

all_register_ids = set()
for ids in class_groups.values():
    all_register_ids |= ids

if all_register_ids != deny_ignore:
    only_register = sorted(all_register_ids - deny_ignore)
    only_deny = sorted(deny_ignore - all_register_ids)
    failures.append(
        "CLAUSE1_DENY_MISMATCH: the union of every register class partition does not equal "
        f"deny.toml's ignore set exactly. Only in register: {only_register}. "
        f"Only in deny.toml: {only_deny}."
    )

# --- Clause 2: register coverage, both directions. ---
all_config_ids = deny_ignore | audit_ignore
rows_by_id = {}
for row in rows:
    rows_by_id.setdefault(row.get("id"), []).append(row)

for cid in sorted(all_config_ids):
    matching = rows_by_id.get(cid)
    if not matching:
        failures.append(
            f"CLAUSE2_UNCOVERED: identifier {cid} appears in a configuration file but has "
            "no register row."
        )
        continue
    for row in matching:
        empty_fields = [f for f in GOVERNANCE_FIELDS if not str(row.get(f, "")).strip()]
        if empty_fields:
            failures.append(
                f"CLAUSE2_INCOMPLETE_ROW: register row {cid} is missing/blank field(s): "
                f"{', '.join(empty_fields)}."
            )

for row in rows:
    rid = row.get("id")
    if rid not in all_config_ids:
        failures.append(
            f"CLAUSE2_STALE_ROW: register row {rid} has no matching suppression in either "
            "deny.toml or .cargo/audit.toml."
        )

# --- Clause 3: crate liveness against Cargo.lock, using the same anchored
# per-crate form `grep -c '^name = "<crate>"$'` uses. ---
lock_text = open(lock_path, encoding="utf-8").read()
lock_names = set(re.findall(r'^name = "([^"]+)"$', lock_text, re.MULTILINE))

for row in rows:
    rid = row.get("id")
    crate_field = str(row.get("crate", ""))
    candidates = [c.strip() for c in crate_field.split(",") if c.strip()]
    if not candidates:
        failures.append(f"CLAUSE3_NO_CRATE: register row {rid} has an empty crate field.")
        continue
    if not any(c in lock_names for c in candidates):
        failures.append(
            f"CLAUSE3_DEAD_CRATE: register row {rid}'s crate field ({crate_field!r}) names "
            "no crate present in Cargo.lock."
        )

if failures:
    print("FAIL")
    for f in failures:
        print(f"FAIL: {f}")
    sys.exit(0)

print("OK")
print(f"{len(rows)} register row(s) checked against {len(deny_ignore)} deny.toml and "
      f"{len(audit_ignore)} .cargo/audit.toml ignore entries; all clauses satisfied.")
sys.exit(0)
PY
)

STATUS_LINE=$(head -n1 <<<"${REPORT}")
DETAIL=$(tail -n +2 <<<"${REPORT}")

if [ "${STATUS_LINE}" = "OK" ]; then
    echo "✅ ${DETAIL}"
    exit 0
else
    echo "❌ Advisory exception register check failed (${STATUS_LINE})"
    echo ""
    echo "${DETAIL}"
    echo ""
    echo "If this failure is unexpected:"
    echo "  1. An uncovered identifier needs a new row in SECURITY-EXCEPTIONS.md."
    echo "  2. A stale row needs its suppression restored in deny.toml/.cargo/audit.toml,"
    echo "     or the row deleted from the register."
    echo "  3. A dead crate means the suppression's dependency left Cargo.lock; delete the"
    echo "     entry from deny.toml/.cargo/audit.toml rather than backfilling it here."
    exit 1
fi
