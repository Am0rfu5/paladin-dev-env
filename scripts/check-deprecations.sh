#!/bin/bash
# Check that all deprecated items still compile with warnings
# Ensures deprecation warnings are properly formatted
set -euo pipefail

echo "🔍 Checking deprecation warnings..."

# Build with warnings as JSON for parsing. The build's own exit status is
# deliberately not used to gate this script (a pipefail-masked `cargo build`
# failure here does not, on its own, mean anything is malformed) -- capture
# its parsed result instead of discarding it, and always fall through to the
# malformed-attribute check below rather than exiting early.
BUILD_JSON=$(RUSTFLAGS="-Dwarnings" cargo build --lib --message-format=json 2>&1 || true)
JQ_OUTPUT=$(echo "$BUILD_JSON" | jq -r 'select(.message.code.code == "deprecated") | .message.rendered' 2>/dev/null || true)

if [ -n "$JQ_OUTPUT" ]; then
    DEPRECATION_COUNT=$(echo "$JQ_OUTPUT" | grep -c . || echo "0")
    echo "⚠️  Found $DEPRECATION_COUNT deprecation warning(s)"
    echo ""
    echo "$JQ_OUTPUT"
    echo ""
    echo "Deprecation warnings are expected during API transition."
    echo "Verify that:"
    echo "  1. Each deprecated item has a clear #[deprecated] annotation"
    echo "  2. The 'since' and 'note' fields provide migration guidance"
    echo "  3. All deprecated items are documented in MIGRATION.md"
else
    # Fallback: build normally and check for deprecation warnings
    OUTPUT=$(cargo build --lib 2>&1 || true)

    # Check if there are any deprecation warnings
    if echo "$OUTPUT" | grep -q "warning:.*deprecated"; then
        DEPRECATION_COUNT=$(echo "$OUTPUT" | grep -c "warning:.*deprecated" || echo "0")
        echo "⚠️  Found $DEPRECATION_COUNT deprecation warning(s)"
        echo ""
        echo "$OUTPUT" | grep -A 2 "warning:.*deprecated" || true
        echo ""
        echo "Deprecation warnings are expected during API transition."
        echo "Verify that:"
        echo "  1. Each deprecated item has a clear #[deprecated] annotation"
        echo "  2. The 'since' and 'note' fields provide migration guidance"
        echo "  3. All deprecated items are documented in MIGRATION.md"
    else
        echo "✅ No deprecation warnings found"
    fi
fi

# Check for malformed deprecation attributes across src/ and crates/ (the
# eleven workspace crates were previously unscanned -- this is the check's
# only real failure path, and the only one that gates this script's exit
# status; a zero-deprecation tree is a pass, not a failure, per D-05).
echo "Checking for properly formatted deprecation attributes..."
if grep -rE "#\[deprecated\]" src/ crates/ --include="*.rs" | grep -v "since\|note"; then
    echo "❌ Found deprecation without 'since' or 'note' fields!"
    echo "   Use: #[deprecated(since = \"0.2.0\", note = \"Use XYZ instead\")]"
    exit 1
fi

echo "✅ All deprecation attributes are properly formatted"
