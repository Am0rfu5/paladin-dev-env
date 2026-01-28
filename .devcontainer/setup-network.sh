#!/bin/bash
# Setup script for DevContainer network
# This ensures the Docker network exists before starting the container

set -e

NETWORK_NAME="paladin-dev-network"

echo "🔍 Checking for Docker network: $NETWORK_NAME"

if docker network ls | grep -q "$NETWORK_NAME"; then
    echo "✅ Network '$NETWORK_NAME' already exists"
else
    echo "📡 Creating Docker network: $NETWORK_NAME"
    docker network create "$NETWORK_NAME"
    echo "✅ Network created successfully"
fi

echo ""
echo "🎯 Network is ready for DevContainer"
