#!/usr/bin/env bash
# check-doc-config.sh
#
# Validates every fenced ```yaml code block in docs/src/**/*.md so that
# configuration snippets in the documentation stay well-formed YAML.
#
# Scope: this is a *syntactic* gate — each block must parse as valid YAML.
# It deliberately does NOT attempt deep schema validation against the Rust
# config types: doc snippets are intentionally partial (a single `battalion:`
# section, a lone Paladin definition, etc.), so a top-level-key allowlist
# produces false positives. Faithful schema validation would require feeding
# each snippet through the framework's `serde` config loader in a Rust harness;
# that is tracked as a follow-up (see prd-new-documentation.md, OQ-7). The
# syntactic gate still catches the common real failure: malformed indentation
# or invalid YAML introduced while editing a config example.
#
# Usage:  ./scripts/check-doc-config.sh
# Exit:   0 if all YAML blocks parse, non-zero on failure.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_SRC="${WORKSPACE_ROOT}/docs/src"

if ! command -v python3 >/dev/null 2>&1; then
    echo "ERROR: python3 is required for YAML validation." >&2
    exit 1
fi

echo "Validating fenced YAML blocks in ${DOCS_SRC} ..."

python3 - "${DOCS_SRC}" <<'PY'
import os
import re
import sys

try:
    import yaml
except ImportError:
    print("ERROR: PyYAML is required (pip install pyyaml).", file=sys.stderr)
    sys.exit(2)

docs_src = sys.argv[1]
fence_re = re.compile(r"^```ya?ml\s*$")
end_re = re.compile(r"^```\s*$")

checked = 0
failed = 0

for root, _dirs, files in os.walk(docs_src):
    for name in sorted(files):
        if not name.endswith(".md"):
            continue
        path = os.path.join(root, name)
        rel = os.path.relpath(path, docs_src)
        with open(path, encoding="utf-8") as fh:
            lines = fh.readlines()

        in_block = False
        block = []
        block_no = 0
        for line in lines:
            if not in_block:
                if fence_re.match(line):
                    in_block = True
                    block = []
            elif end_re.match(line):
                in_block = False
                block_no += 1
                checked += 1
                snippet = "".join(block)
                try:
                    list(yaml.safe_load_all(snippet))
                except yaml.YAMLError as exc:
                    failed += 1
                    print(f"YAML PARSE FAIL: {rel} block #{block_no}\n{exc}\n---")
            else:
                block.append(line)

print(f"\nResults: {checked} YAML block(s) checked, {failed} failed")
sys.exit(1 if failed else 0)
PY
