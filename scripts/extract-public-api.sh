#!/bin/bash
# Extract public API surface using cargo-public-api
# Requires cargo-public-api: cargo install cargo-public-api
set -euo pipefail

OUTPUT_FILE="${1:-.project/current-exports.txt}"

echo "Extracting public API surface using cargo-public-api..."

# Check if cargo-public-api is installed
if ! command -v cargo-public-api &> /dev/null; then
    echo "❌ cargo-public-api not found. Installing..."
    cargo install cargo-public-api || {
        echo "❌ Failed to install cargo-public-api"
        echo "   This tool requires OpenSSL 3.0.0+"
        echo "   If in devcontainer, rebuild with: Ctrl+Shift+P -> 'Dev Containers: Rebuild Container'"
        exit 1
    }
fi

# Generate simplified public API list
echo "# Public API Surface - Generated $(date -u +"%Y-%m-%d %H:%M:%S UTC")" > "$OUTPUT_FILE"
echo "# This file tracks all publicly exported items from the paladin crate" >> "$OUTPUT_FILE"
echo "# Generated using cargo-public-api v$(cargo public-api --version | awk '{print $2}')" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Extract public API in simplified format
# Note: cargo public-api may emit warnings to stderr, but still succeeds
cargo public-api --simplified >> "$OUTPUT_FILE" 2>/dev/null || {
    echo "❌ Failed to generate API surface"
    echo "   Check that nightly toolchain is installed: rustup toolchain install nightly"
    exit 1
}

# Count total items
TOTAL=$(grep -c "^pub " "$OUTPUT_FILE" || echo "0")
echo "" >> "$OUTPUT_FILE"
echo "## Summary" >> "$OUTPUT_FILE"
echo "Total public items: $TOTAL" >> "$OUTPUT_FILE"

echo "✅ API surface extracted to $OUTPUT_FILE ($TOTAL items)"
