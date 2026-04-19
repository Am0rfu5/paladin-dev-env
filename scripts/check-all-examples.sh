#!/bin/bash
# Script to check that all examples compile successfully
# Usage: ./scripts/check-all-examples.sh

set -e

EXAMPLES_DIR="examples"
FAILED_EXAMPLES=()
SUCCESS_COUNT=0
TOTAL_COUNT=0

echo "=========================================="
echo "Checking all examples in $EXAMPLES_DIR"
echo "=========================================="
echo ""

# Find all .rs files in examples directory
for example in "$EXAMPLES_DIR"/*.rs; do
    if [ -f "$example" ]; then
        TOTAL_COUNT=$((TOTAL_COUNT + 1))
        example_name=$(basename "$example" .rs)
        
        echo -n "[$TOTAL_COUNT] Checking $example_name... "
        
        # Try to compile the example
        if cargo check --example "$example_name" --all-features 2>&1 | grep -q "error"; then
            echo "❌ FAILED"
            FAILED_EXAMPLES+=("$example_name")
        else
            echo "✅ OK"
            SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        fi
    fi
done

echo ""
echo "=========================================="
echo "Summary"
echo "=========================================="
echo "Total examples: $TOTAL_COUNT"
echo "Successful: $SUCCESS_COUNT"
echo "Failed: ${#FAILED_EXAMPLES[@]}"

if [ ${#FAILED_EXAMPLES[@]} -gt 0 ]; then
    echo ""
    echo "Failed examples:"
    for failed in "${FAILED_EXAMPLES[@]}"; do
        echo "  - $failed"
    done
    echo ""
    echo "To see detailed errors for a specific example:"
    echo "  cargo check --example <example_name> --all-features"
    exit 1
else
    echo ""
    echo "✅ All examples compile successfully!"
    exit 0
fi
