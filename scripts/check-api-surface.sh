#!/bin/bash
# Check for API surface changes compared to baseline
# Used in CI to detect accidental API changes
set -euo pipefail

BASELINE="${1:-project/current-exports.txt}"
TEMP_FILE=$(mktemp)

echo "🔍 Checking API surface for changes..."

# Generate current API surface
./scripts/extract-public-api.sh "$TEMP_FILE"

# Compare with baseline (ignore timestamps and summary sections)
if [ ! -f "$BASELINE" ]; then
    echo "⚠️  No baseline found at $BASELINE"
    echo "   Run: ./scripts/extract-public-api.sh $BASELINE"
    exit 1
fi

# Filter out generated timestamps and summary for comparison
FILTERED_BASELINE=$(mktemp)
FILTERED_CURRENT=$(mktemp)

grep -v "^# Public API Surface - Generated" "$BASELINE" | \
    grep -v "^Total public items:" > "$FILTERED_BASELINE" || true

grep -v "^# Public API Surface - Generated" "$TEMP_FILE" | \
    grep -v "^Total public items:" > "$FILTERED_CURRENT" || true

if diff -u "$FILTERED_BASELINE" "$FILTERED_CURRENT" > /dev/null 2>&1; then
    echo "✅ API surface unchanged"
    rm -f "$TEMP_FILE" "$FILTERED_BASELINE" "$FILTERED_CURRENT"
    exit 0
else
    echo "❌ API surface has changed!"
    echo ""
    echo "Differences:"
    diff -u "$FILTERED_BASELINE" "$FILTERED_CURRENT" || true
    echo ""
    echo "If this change is intentional:"
    echo "  1. Review the changes carefully"
    echo "  2. Update CHANGELOG.md with breaking changes"
    echo "  3. Update the baseline: ./scripts/extract-public-api.sh $BASELINE"
    rm -f "$TEMP_FILE" "$FILTERED_BASELINE" "$FILTERED_CURRENT"
    exit 1
fi
