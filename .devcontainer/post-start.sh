#!/bin/bash
# Post-start script for DevContainer
# Runs every time the container starts

set -e

echo "🌟 Starting Paladin development environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

cd /workspace

# Fix ownership of target directory if needed (common issue with Docker volumes)
if [ -d "target" ]; then
    TARGET_OWNER=$(stat -c '%U' target)
    if [ "$TARGET_OWNER" != "vscode" ]; then
        echo -e "${BLUE}🔧 Fixing target directory ownership...${NC}"
        sudo chown -R vscode:vscode target
        echo "  ✅ Target directory ownership fixed"
    fi
fi

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

    # Configure shell to auto-load workspace .env for interactive terminals
    ENV_LOADER_LINE='[ -f /workspace/.env ] && set -a && . /workspace/.env && set +a'
    if ! grep -Fq "$ENV_LOADER_LINE" /home/vscode/.bashrc; then
        echo "" >> /home/vscode/.bashrc
        echo "# Load Paladin workspace env vars" >> /home/vscode/.bashrc
        echo "$ENV_LOADER_LINE" >> /home/vscode/.bashrc
        echo "  ✅ Added .env auto-load hook to ~/.bashrc"
    fi
else
    echo "  ⚠️  .env file missing (use .env.example as template)"
fi

# Wire the host-credential loader into interactive shells. It must be sourced AFTER
# the .env loader above: .env declares the LLM key names with empty values, and
# paladin-env.sh fills any that are unset-or-empty from ~/.config/paladin.
PALADIN_ENV_LINE='[ -f /workspace/.devcontainer/paladin-env.sh ] && . /workspace/.devcontainer/paladin-env.sh'
if ! grep -Fq "$PALADIN_ENV_LINE" /home/vscode/.bashrc; then
    {
        echo ""
        echo "# Load host-provided credentials (see .devcontainer/paladin-env.sh)"
        echo "$PALADIN_ENV_LINE"
    } >> /home/vscode/.bashrc
    echo "  ✅ Added host-credential loader to ~/.bashrc"
fi

if [ -d /home/vscode/.config/paladin ]; then
    # shellcheck source=paladin-env.sh
    . /workspace/.devcontainer/paladin-env.sh
    echo "  ✅ Host credential mount active ($(ls -1 /home/vscode/.config/paladin 2>/dev/null | wc -l) file(s)) — run 'paladin-keys' to list"
else
    echo "  ⚠️  No host credential mount at ~/.config/paladin"
    echo "     On the HOST:  mkdir -p ~/.config/paladin"
    echo "     Then one file per key, e.g. ~/.config/paladin/gemini_api_key"
fi

# Claude Code state mount guard. Docker silently creates a missing bind-mount
# source as a ROOT-OWNED directory, after which the vscode user cannot write to
# it and Claude Code loses all state without any visible error — detect that and
# say what to do about it.
CLAUDE_STATE_DIR="${CLAUDE_STATE_DIR:-/home/vscode/.claude}"
if [ ! -d "$CLAUDE_STATE_DIR" ]; then
    echo -e "${YELLOW}⚠️  No Claude Code state mount at $CLAUDE_STATE_DIR${NC}"
    echo "     Sessions and login will NOT survive a rebuild."
    echo "     On the HOST:  mkdir -p ~/.claude-paladin && chmod 700 ~/.claude-paladin"
    echo "     Then run Dev Containers: Rebuild Container."
elif [ ! -w "$CLAUDE_STATE_DIR" ]; then
    CLAUDE_STATE_OWNER=$(stat -c '%U' "$CLAUDE_STATE_DIR")
    echo -e "${RED}❌ Claude Code state mount at $CLAUDE_STATE_DIR is not writable (owned by $CLAUDE_STATE_OWNER)${NC}"
    echo "     Docker creates a missing bind-mount source root-owned; fix ownership on the HOST:"
    echo '     sudo chown -R "$(id -u):$(id -g)" ~/.claude-paladin'
else
    TRANSCRIPT_COUNT=$(find "$CLAUDE_STATE_DIR/projects/-workspace" -maxdepth 1 -name '*.jsonl' 2>/dev/null | wc -l)
    echo "  ✅ Claude Code state mount active ($TRANSCRIPT_COUNT session transcript(s) for this workspace)"
    [ -f "$CLAUDE_STATE_DIR/.claude.json" ] || echo "     Authenticate Claude Code once; the login then persists across rebuilds."
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
