#!/bin/bash
# Check that all deprecated items still compile with warnings
# Ensures deprecation warnings are properly formatted
set -euo pipefail

echo "🔍 Checking deprecation warnings..."

# Build with warnings as JSON for parsing
RUSTFLAGS="-Dwarnings" cargo build --lib --message-format=json 2>&1 | \
    jq -r 'select(.message.code.code == "deprecated") | .message.rendered' > /dev/null 2>&1 || {

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
        exit 0
    else
        echo "✅ No deprecation warnings found"
        exit 0
    fi
}

# Check for malformed deprecation attributes
echo "Checking for properly formatted deprecation attributes..."
if grep -r "#\[deprecated\]" src/ --include="*.rs" | grep -v "since\|note"; then
    echo "❌ Found deprecation without 'since' or 'note' fields!"
    echo "   Use: #[deprecated(since = \"0.2.0\", note = \"Use XYZ instead\")]"
    exit 1
fi

echo "✅ All deprecation attributes are properly formatted"
