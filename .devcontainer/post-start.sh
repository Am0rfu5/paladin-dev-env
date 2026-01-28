#!/bin/bash
# Post-start script for DevContainer
# Runs every time the container starts

set -e

echo "🌟 Starting Paladin development environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

cd /workspace

# Check if we're in a git repository
if [ -d ".git" ]; then
    echo -e "${BLUE}📊 Git status:${NC}"
    git status -sb
    echo ""
fi

# Check for uncommitted changes
if [ -d ".git" ]; then
    if ! git diff-index --quiet HEAD -- 2>/dev/null; then
        echo -e "${BLUE}⚠️  You have uncommitted changes${NC}"
    fi
fi

# Display system information
echo -e "${BLUE}💻 System information:${NC}"
echo "  Rust version: $(rustc --version)"
echo "  Cargo version: $(cargo --version)"
echo "  CPU cores: $(nproc)"
echo "  Memory: $(free -h | awk '/^Mem:/ {print $2}')"
echo ""

# Check if services are needed
if [ -f "docker-compose.yml" ] || [ -f "docker/docker-compose.yml" ]; then
    echo -e "${BLUE}🐳 Docker Compose available${NC}"
    echo "  Run 'pd-dev' to start all services"
    echo "  Run 'pd-services' to start supporting services only"
    echo ""
fi

# Quick health check
echo -e "${BLUE}🏥 Quick health check:${NC}"

# Check if cargo works
if cargo --version &> /dev/null; then
    echo "  ✅ Cargo is working"
else
    echo "  ❌ Cargo check failed"
fi

# Check if we can build
if [ -f "Cargo.toml" ]; then
    echo "  ✅ Cargo.toml found"
else
    echo "  ❌ Cargo.toml not found"
fi

# Check for .env file
if [ -f ".env" ]; then
    echo "  ✅ .env file exists"
else
    echo "  ⚠️  .env file missing (use .env.example as template)"
fi

echo ""
echo -e "${GREEN}✨ Ready to code!${NC}"
echo ""

# Optional: Auto-start services if configured
# Uncomment the following lines to auto-start services on container start
# if [ "$AUTO_START_SERVICES" = "true" ]; then
#     echo "🚀 Auto-starting services..."
#     make services-up &
# fi
