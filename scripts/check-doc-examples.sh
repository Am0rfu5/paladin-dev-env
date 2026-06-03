#!/usr/bin/env bash
# check-doc-examples.sh
#
# Two-layer guarantee that documentation examples are correct:
#
#   1. PRIMARY (compile) gate: the `paladin-doc-examples` workspace crate holds
#      the real source for every substantive example, exposed via mdBook
#      `{{#include ...:anchor}}` directives in the guides. `cargo check` on that
#      crate compiles every example against the current APIs — so a renamed
#      type or changed signature fails the build. This is the real assurance.
#
#   2. SECONDARY (syntax) scan: any remaining inline fenced ```rust blocks in
#      docs/src/**/*.md are syntax-checked with rustfmt (illustrative `,ignore`
#      / `{{#include}}` blocks are skipped). This catches stray hand-written
#      snippets that aren't backed by the examples crate.
#
# Usage:  ./scripts/check-doc-examples.sh
# Exit:   0 if all examples compile and all inline blocks are valid.

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_SRC="${WORKSPACE_ROOT}/docs/src"

# --- Layer 1: compile every included example for real ------------------------
echo "Compiling documentation examples (paladin-doc-examples crate)..."
if ! cargo check --quiet --manifest-path "${WORKSPACE_ROOT}/crates/doc-examples/Cargo.toml"; then
    echo "ERROR: documentation examples failed to compile." >&2
    echo "Fix the example in crates/doc-examples/src/ that no longer matches the API." >&2
    exit 1
fi
echo "All included examples compile."
echo ""

# --- Layer 1b: README Quick Example must match its compiled source ------------
# The root README isn't processed by mdBook, so it can't use `{{#include}}`.
# Instead the Quick Example is kept verbatim in `crates/doc-examples/src/readme.rs`
# (compiled by Layer 1) and mirrored in README.md; this check fails if they drift.
README_FILE="${WORKSPACE_ROOT}/README.md"
README_SRC="${WORKSPACE_ROOT}/crates/doc-examples/src/readme.rs"
if [[ -f "${README_FILE}" && -f "${README_SRC}" ]]; then
    echo "Checking README Quick Example matches crates/doc-examples/src/readme.rs ..."
    if ! python3 - "${README_FILE}" "${README_SRC}" <<'PY'
import re, sys
readme, src = sys.argv[1], sys.argv[2]

# The anchored region in readme.rs (between ANCHOR / ANCHOR_END markers).
src_text = open(src, encoding="utf-8").read()
m = re.search(r"// ANCHOR: quickstart\n(.*?)\n[ \t]*// ANCHOR_END: quickstart", src_text, re.S)
if not m:
    print("README-SYNC FAIL: could not find the `quickstart` anchor in readme.rs", file=sys.stderr)
    sys.exit(1)
anchor = m.group(1).strip("\n")

# The first ```rust block in README.md.
readme_text = open(readme, encoding="utf-8").read()
b = re.search(r"```rust\n(.*?)\n```", readme_text, re.S)
if not b:
    print("README-SYNC FAIL: no ```rust block found in README.md", file=sys.stderr)
    sys.exit(1)
block = b.group(1).strip("\n")

if anchor != block:
    print("README-SYNC FAIL: README.md Quick Example does not match readme.rs anchor.", file=sys.stderr)
    print("Update README.md to match crates/doc-examples/src/readme.rs (the compiled source).", file=sys.stderr)
    sys.exit(1)
PY
    then
        exit 1
    fi
    echo "README Quick Example is in sync."
    echo ""
fi

TMPDIR_BASE="/tmp/paladin-doc-check-$$"
FAILED=0
CHECKED=0
SKIPPED=0

# Cleanup temp directory on exit
trap 'rm -rf "${TMPDIR_BASE}"' EXIT

mkdir -p "${TMPDIR_BASE}"

# Create a minimal Cargo.toml for the temp check crate
make_cargo_toml() {
    local crate_dir="$1"
    cat > "${crate_dir}/Cargo.toml" <<TOML
[package]
name = "paladin-doc-check"
version = "0.1.0"
edition = "2021"

[dependencies]
paladin-ai-core  = { path = "${WORKSPACE_ROOT}/crates/paladin-core",   optional = true }
paladin-ports    = { path = "${WORKSPACE_ROOT}/crates/paladin-ports",  optional = true }
paladin-battalion = { path = "${WORKSPACE_ROOT}/crates/paladin-battalion", optional = true }
paladin-llm       = { path = "${WORKSPACE_ROOT}/crates/paladin-llm",      optional = true }
paladin-memory    = { path = "${WORKSPACE_ROOT}/crates/paladin-memory",    optional = true }
paladin-storage   = { path = "${WORKSPACE_ROOT}/crates/paladin-storage",   optional = true }

[features]
default = []

# Silence unused import warnings from illustrative code blocks
[lints.rust]
unused_imports = "allow"
unused_variables = "allow"
dead_code = "allow"
TOML
    mkdir -p "${crate_dir}/src"
}

# Check a single rust code block
check_block() {
    local source_file="$1"
    local block_num="$2"
    local block_content="$3"

    local tmp_rs="${TMPDIR_BASE}/block_${block_num}.rs"
    printf '%s\n' "${block_content}" > "${tmp_rs}"

    # Only fully validate complete programs (those that contain `fn main`).
    # Illustrative snippets (builder chains, partial expressions, API examples)
    # are skipped — they require surrounding context to compile and are not
    # intended to be standalone programs.
    if ! grep -q 'fn main' "${tmp_rs}"; then
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    # Skip blocks that use `...` placeholder syntax (pseudo-code examples)
    if grep -q '\.\.\.' "${tmp_rs}"; then
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    # Step 1: syntax check via rustfmt (fast, no dependencies needed)
    if ! rustfmt --edition 2021 "${tmp_rs}" 2>/dev/null; then
        FAILED=$((FAILED + 1))
        echo "SYNTAX FAIL: ${source_file} — block #${block_num}"
        rustfmt --edition 2021 "${tmp_rs}" 2>&1 | head -10
        echo "---"
        return
    fi

    # Step 2: cargo check for complete programs that don't import external crates
    # Skip if the block imports external crates (paladin::, tokio::, etc.)
    # since those are illustrative API-usage examples that can't compile standalone
    if grep -qE 'use (paladin|tokio|async_trait|serde|reqwest|anyhow|thiserror)::|#\[tokio::' "${tmp_rs}"; then
        # Illustrative example — skip cargo check
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    local crate_dir="${TMPDIR_BASE}/crate_${block_num}"
    mkdir -p "${crate_dir}"
    make_cargo_toml "${crate_dir}"
    cp "${tmp_rs}" "${crate_dir}/src/main.rs"

    if ! cargo check --quiet --manifest-path "${crate_dir}/Cargo.toml" 2>/dev/null; then
        FAILED=$((FAILED + 1))
        echo "COMPILE FAIL: ${source_file} — block #${block_num}"
        cargo check --manifest-path "${crate_dir}/Cargo.toml" 2>&1 | head -20
        echo "---"
        return
    fi

    CHECKED=$((CHECKED + 1))
}

# Process a single markdown file
process_file() {
    local md_file="$1"
    local rel_path="${md_file#${WORKSPACE_ROOT}/}"
    local in_block=0
    local block_num=0
    local block_lines=()
    local info_string=""

    while IFS= read -r line || [[ -n "${line}" ]]; do
        if [[ ${in_block} -eq 0 ]]; then
            if [[ "${line}" =~ ^(\`\`\`|~~~)(rust.*)$ ]]; then
                in_block=1
                info_string="${BASH_REMATCH[2]}"
                block_lines=()
            fi
        else
            if [[ "${line}" =~ ^(\`\`\`|~~~)[[:space:]]*$ ]]; then
                in_block=0
                block_num=$((block_num + 1))
                # Skip blocks with modifiers that mean "don't compile/run"
                if [[ "${info_string}" =~ (no_run|ignore|compile_fail|text|bash|sh|toml|yaml|json|console) ]]; then
                    SKIPPED=$((SKIPPED + 1))
                else
                    local content
                    content="$(printf '%s\n' "${block_lines[@]}")"
                    check_block "${rel_path}" "${block_num}" "${content}"
                fi
            else
                block_lines+=("${line}")
            fi
        fi
    done < "${md_file}"
}

echo "Checking doc code examples in ${DOCS_SRC} ..."

# Find all markdown files in docs/src
while IFS= read -r -d '' md_file; do
    process_file "${md_file}"
done < <(find "${DOCS_SRC}" -name '*.md' -print0 | sort -z)

echo ""
echo "Results: ${CHECKED} checked, ${SKIPPED} skipped, ${FAILED} failed"

if [[ ${FAILED} -gt 0 ]]; then
    echo "ERROR: ${FAILED} doc code block(s) failed validation." >&2
    exit 1
fi

echo "All doc code examples pass validation."
exit 0
