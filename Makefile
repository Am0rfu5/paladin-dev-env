# Makefile for paladin project

# Variables
CARGO := cargo
DOCKER := docker
DOCKER_COMPOSE := docker-compose
PROJECT_NAME := paladin

# Docker compose files
COMPOSE_FILE := docker/docker-compose.yml
COMPOSE_DEV_FILE := docker/docker-compose.dev.yml
COMPOSE_TEST_FILE := docker/docker-compose.test.yml

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[0;36m
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m

##@ Development

.PHONY: help
help: ## Show this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\n$(CYAN)Usage:$(NC)\n  make $(YELLOW)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:[^#]*##/ { printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(CYAN)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: examples
examples: ## Show common usage examples
	@echo "$(CYAN)Common Usage Examples:$(NC)"
	@echo ""
	@echo "$(YELLOW)Development Workflow:$(NC)"
	@echo "  make setup                    # First time setup"
	@echo "  make dev                      # Start dev environment"
	@echo "  make watch                    # Watch for changes"
	@echo "  make test-integration-minio   # Test MinIO integration"
	@echo ""
	@echo "$(YELLOW)Testing:$(NC)"
	@echo "  make test-all                 # Run all tests"
	@echo "  make test-integration-docker  # Integration tests with Docker"
	@echo "  make ci-test                  # Full CI test suite"
	@echo ""
	@echo "$(YELLOW)Code Quality:$(NC)"
	@echo "  make clean-code               # Format, lint, and check"
	@echo "  make release VERSION=0.4.0    # Cut a release (bump, changelog, tag, push)"
	@echo "  make publish-dry-run          # Dependency-first cargo publish --dry-run"
	@echo "  make audit                    # Security audit"
	@echo "  make doc                      # Generate docs"
	@echo ""
	@echo "$(YELLOW)Services Management:$(NC)"
	@echo "  make services-up              # Start all services"
	@echo "  make redis-cli                # Connect to Redis"
	@echo "  make minio-console            # Open MinIO console"
	@echo "  make health                   # Check service health"

.PHONY: status
status: ## Show project status
	@echo "$(CYAN)Project Status:$(NC)"
	@echo ""
	@echo "$(YELLOW)Build Status:$(NC)"
	@$(CARGO) --version || echo "❌ Cargo not found"
	@rustc --version || echo "❌ Rust not found"
	@echo ""
	@echo "$(YELLOW)Docker Status:$(NC)"
	@$(DOCKER) --version || echo "❌ Docker not found"
	@$(DOCKER_COMPOSE) --version || echo "❌ Docker Compose not found"
	@echo ""
	@echo "$(YELLOW)Services Status:$(NC)"
	@$(MAKE) health
	@echo ""
	@echo "$(YELLOW)Git Status:$(NC)"
	@git status --porcelain || echo "Not a git repository"

# Include custom targets if they exist
-include Makefile.local setup
setup: ## Initial project setup
	@echo "$(CYAN)Setting up development environment...$(NC)"
	@rustup update stable
	@rustup component add rustfmt clippy
	@$(CARGO) install cargo-audit cargo-watch cargo-expand
	@$(CARGO) install --locked cargo-release
	@$(CARGO) install --locked cargo-deny cargo-cyclonedx
	@cp .env.example .env 2>/dev/null || echo ".env already exists"
	@echo "$(GREEN)✅ Setup complete!$(NC)"

.PHONY: dev
dev: ## Start development environment with hot reload
	@echo "$(CYAN)Starting development environment...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) -f $(COMPOSE_DEV_FILE) up -d
	@echo "$(GREEN)✅ Development environment started$(NC)"
	@echo "Services available at:"
	@echo "  - Application: http://localhost:8080"
	@echo "  - MinIO Console: http://localhost:9001"
	@echo "  - Redis Commander: http://localhost:8081"

.PHONY: dev-logs
dev-logs: ## Show development environment logs
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) -f $(COMPOSE_DEV_FILE) logs -f

.PHONY: watch
watch: ## Watch for file changes and run tests
	@echo "$(CYAN)Starting file watcher...$(NC)"
	@$(CARGO) watch -x check -x test -x 'clippy --all-targets'

##@ Testing

.PHONY: test
test: ## Run unit tests
	@echo "$(CYAN)Running unit tests...$(NC)"
	@$(CARGO) test --workspace --lib --bins

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "$(CYAN)Running documentation tests...$(NC)"
	@$(CARGO) test --workspace --doc

.PHONY: test-integration
test-integration: ## Run integration tests (local mode)
	@echo "$(CYAN)Running integration tests in local mode...$(NC)"
	@./scripts/run_integration_tests.sh -m local

.PHONY: test-integration-docker
test-integration-docker: ## Run integration tests with docker-compose
	@echo "$(CYAN)Running integration tests with docker-compose...$(NC)"
	@./scripts/run_integration_tests.sh -m docker -v

.PHONY: test-integration-redis
test-integration-redis: ## Run Redis integration tests only
	@echo "$(CYAN)Running Redis integration tests...$(NC)"
	@./scripts/run_integration_tests.sh -t "redis" -m local

.PHONY: test-integration-minio
test-integration-minio: ## Run MinIO integration tests only
	@echo "$(CYAN)Running MinIO integration tests...$(NC)"
	@./scripts/run_integration_tests.sh -t "file_storage" -m local

.PHONY: test-all
test-all: test test-doc test-integration ## Run all tests
	@echo "$(GREEN)✅ All tests completed!$(NC)"

.PHONY: check-doc-examples
check-doc-examples: ## Compile doc examples (paladin-doc-examples crate) + syntax-scan inline rust blocks
	@echo "$(CYAN)Checking doc code examples...$(NC)"
	@./scripts/check-doc-examples.sh

.PHONY: check-doc-config
check-doc-config: ## Validate fenced YAML config snippets in docs/src parse correctly
	@echo "$(CYAN)Checking doc config snippets...$(NC)"
	@./scripts/check-doc-config.sh

.PHONY: test-ci
test-ci: ## Run tests in CI mode
	@echo "$(CYAN)Running tests in CI mode...$(NC)"
	@./scripts/run_integration_tests.sh -m ci

##@ Per-Crate Testing

.PHONY: test-core
test-core: ## Run tests for paladin-core
	@echo "$(CYAN)Running tests for paladin-core...$(NC)"
	@$(CARGO) test -p paladin-core

.PHONY: test-ports
test-ports: ## Run tests for paladin-ports
	@echo "$(CYAN)Running tests for paladin-ports...$(NC)"
	@$(CARGO) test -p paladin-ports

.PHONY: test-battalion
test-battalion: ## Run tests for paladin-battalion
	@echo "$(CYAN)Running tests for paladin-battalion...$(NC)"
	@$(CARGO) test -p paladin-battalion

.PHONY: test-llm
test-llm: ## Run tests for paladin-llm
	@echo "$(CYAN)Running tests for paladin-llm...$(NC)"
	@$(CARGO) test -p paladin-llm

.PHONY: test-memory
test-memory: ## Run tests for paladin-memory
	@echo "$(CYAN)Running tests for paladin-memory...$(NC)"
	@$(CARGO) test -p paladin-memory

.PHONY: test-storage
test-storage: ## Run tests for paladin-storage
	@echo "$(CYAN)Running tests for paladin-storage...$(NC)"
	@$(CARGO) test -p paladin-storage

.PHONY: test-notifications
test-notifications: ## Run tests for paladin-notifications
	@echo "$(CYAN)Running tests for paladin-notifications...$(NC)"
	@$(CARGO) test -p paladin-notifications

.PHONY: test-content
test-content: ## Run tests for paladin-content
	@echo "$(CYAN)Running tests for paladin-content...$(NC)"
	@$(CARGO) test -p paladin-content

.PHONY: test-web
test-web: ## Run tests for paladin-web
	@echo "$(CYAN)Running tests for paladin-web...$(NC)"
	@$(CARGO) test -p paladin-web

.PHONY: test-facade
test-facade: ## Run tests for paladin facade crate
	@echo "$(CYAN)Running tests for paladin (facade)...$(NC)"
	@$(CARGO) test -p paladin

##@ Code Quality

.PHONY: fmt
fmt: ## Format code
	@echo "$(CYAN)Formatting code...$(NC)"
	@$(CARGO) fmt --all

.PHONY: lint
lint: ## Run linter
	@echo "$(CYAN)Running linter...$(NC)"
	@$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: check
check: ## Check code without building
	@echo "$(CYAN)Checking code...$(NC)"
	@$(CARGO) check --workspace --all-targets

.PHONY: audit
audit: ## Run security audit (vulnerability advisories)
	@echo "$(CYAN)Running security audit...$(NC)"
	@# Exceptions are sourced from .cargo/audit.toml (single source of truth).
	@$(CARGO) audit

.PHONY: deny
deny: ## Run cargo-deny (licenses, bans, advisories, sources)
	@echo "$(CYAN)Running cargo-deny check...$(NC)"
	@$(CARGO) deny check

.PHONY: security
security: audit deny ## Run all dependency security & license checks

.PHONY: sbom
sbom: ## Generate a CycloneDX SBOM (paladin.cdx.json)
	@echo "$(CYAN)Generating CycloneDX SBOM...$(NC)"
	@command -v cargo-cyclonedx >/dev/null 2>&1 || { \
		echo "$(RED)cargo-cyclonedx not found. Install with 'cargo install --locked cargo-cyclonedx'.$(NC)"; \
		exit 1; \
	}
	@$(CARGO) cyclonedx --all --format json
	@echo "$(GREEN)✅ SBOM generated (see bom.json / <crate>.cdx.json)$(NC)"

.PHONY: doc
doc: ## Generate documentation
	@echo "$(CYAN)Generating documentation...$(NC)"
	@$(CARGO) doc --workspace --no-deps --open

.PHONY: clean-code
clean-code: fmt lint check ## Format, lint, and check code

.PHONY: hooks
hooks: ## Install git pre-commit and pre-push hooks
	@echo "$(CYAN)Installing pre-commit hooks...$(NC)"
	@command -v pre-commit >/dev/null 2>&1 || { \
		echo "$(RED)pre-commit not found. Install it with 'pipx install pre-commit' (see CONTRIBUTING.md).$(NC)"; \
		exit 1; \
	}
	@pre-commit install
	@pre-commit install --hook-type pre-push
	@echo "$(GREEN)✅ Git hooks installed (pre-commit + pre-push)$(NC)"

##@ Build

.PHONY: build
build: ## Build the project
	@echo "$(CYAN)Building project...$(NC)"
	@$(CARGO) build --workspace

.PHONY: build-release
build-release: ## Build release version
	@echo "$(CYAN)Building release version...$(NC)"
	@$(CARGO) build --release --workspace

.PHONY: build-docker
build-docker: ## Build Docker image
	@echo "$(CYAN)Building Docker image...$(NC)"
	@$(DOCKER) build -f docker/Dockerfile -t $(PROJECT_NAME):latest .

##@ Docker Services

.PHONY: services-up
services-up: ## Start all services
	@echo "$(CYAN)Starting all services...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) up -d
	@echo "$(GREEN)✅ Services started$(NC)"

.PHONY: services-down
services-down: ## Stop all services
	@echo "$(CYAN)Stopping all services...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) down
	@echo "$(GREEN)✅ Services stopped$(NC)"

.PHONY: services-restart
services-restart: services-down services-up ## Restart all services

.PHONY: services-logs
services-logs: ## Show service logs
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) logs -f

.PHONY: services-ps
services-ps: ## Show running services
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) ps

.PHONY: redis-cli
redis-cli: ## Connect to Redis CLI
	@echo "$(CYAN)Connecting to Redis...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) exec redis redis-cli

.PHONY: minio-console
minio-console: ## Open MinIO console
	@echo "$(CYAN)Opening MinIO console...$(NC)"
	@echo "MinIO Console: http://localhost:9001"
	@echo "Credentials: minioadmin/minioadmin"

##@ Database & Storage

.PHONY: db-reset
db-reset: ## Reset database
	@echo "$(CYAN)Resetting database...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) exec paladin-app rm -f database.db || true
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) restart paladin-app

.PHONY: storage-reset
storage-reset: ## Reset MinIO storage
	@echo "$(CYAN)Resetting MinIO storage...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) exec minio rm -rf /data/* || true
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) restart minio minio-init

.PHONY: data-reset
data-reset: db-reset storage-reset ## Reset all data

##@ Utilities

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(CYAN)Cleaning build artifacts...$(NC)"
	@$(CARGO) clean
	@$(DOCKER) system prune -f

.PHONY: clean-docker
clean-docker: ## Clean Docker containers and volumes
	@echo "$(CYAN)Cleaning Docker resources...$(NC)"
	@$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) down -v --remove-orphans
	@$(DOCKER_COMPOSE) -f $(COMPOSE_TEST_FILE) down -v --remove-orphans || true
	@$(DOCKER) system prune -f

.PHONY: deps-update
deps-update: ## Update dependencies
	@echo "$(CYAN)Updating dependencies...$(NC)"
	@$(CARGO) update

.PHONY: deps-tree
deps-tree: ## Show dependency tree
	@$(CARGO) tree

.PHONY: size
size: ## Show binary size
	@echo "$(CYAN)Binary sizes:$(NC)"
	@ls -lh target/release/$(PROJECT_NAME) 2>/dev/null || echo "No release binary found. Run 'make build-release' first."

##@ CI/CD

.PHONY: ci-setup
ci-setup: ## Setup CI environment
	@echo "$(CYAN)Setting up CI environment...$(NC)"
	@rustup component add rustfmt clippy

.PHONY: ci-test
ci-test: ## Run CI test suite
	@echo "$(CYAN)Running CI test suite...$(NC)"
	@$(MAKE) clean-code
	@$(MAKE) test
	@$(MAKE) test-doc
	@$(MAKE) audit
	@$(MAKE) test-ci

.PHONY: release-check
release-check: ## Check if ready for release
	@echo "$(CYAN)Checking release readiness...$(NC)"
	@$(MAKE) clean-code
	@$(MAKE) test
	@echo "$(CYAN)Running doc tests (excluding paladin-ports: doctests reference root crate not yet published)...$(NC)"
	@$(CARGO) test --workspace --doc --exclude paladin-ports
	@$(MAKE) audit
	@$(MAKE) build-release
	@echo "$(GREEN)✅ Release check passed!$(NC)"

.PHONY: publish-dry-run
publish-dry-run: release-check ## Run dependency-first `cargo publish --dry-run` for all crates
	@echo "$(CYAN)Running dependency-first publish dry-runs...$(NC)"
	@$(CARGO) publish --dry-run -p paladin-core || true
	@$(CARGO) publish --dry-run -p paladin-ports || true
	@$(CARGO) publish --dry-run -p paladin-battalion || true
	@$(CARGO) publish --dry-run -p paladin-llm || true
	@$(CARGO) publish --dry-run -p paladin-memory || true
	@$(CARGO) publish --dry-run -p paladin-web || true
	@$(CARGO) publish --dry-run -p paladin-notifications || true
	@$(CARGO) publish --dry-run -p paladin-content || true
	@$(CARGO) publish --dry-run -p paladin-storage || true
	@$(CARGO) publish --dry-run -p paladin || true
	@echo "$(YELLOW)Dry-run publish command sequence completed. See docs/RELEASE_CHECKLIST.md for interpretation and publish-order gating.$(NC)"

.PHONY: release
release: ## Cut a release: bump version (lockstep), finalize changelog, commit, tag, push. Usage: make release VERSION=0.4.0
	@if [ -z "$(VERSION)" ]; then \
		echo "$(RED)❌ VERSION is required. Usage: make release VERSION=0.4.0$(NC)"; \
		exit 1; \
	fi
	@echo "$(VERSION)" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$$' || { \
		echo "$(RED)❌ VERSION '$(VERSION)' is not a valid semver string (e.g. 0.4.0 or 0.4.0-rc.1).$(NC)"; \
		exit 1; \
	}
	@command -v cargo-release >/dev/null 2>&1 || { \
		echo "$(RED)❌ cargo-release not found. Install with 'cargo install --locked cargo-release'.$(NC)"; \
		exit 1; \
	}
	@# Release-branch protection (Milestone 10 Epic 5): tags may only be cut from
	@# an up-to-date `main`. The CI guard in release.yml is authoritative; this is
	@# fast local feedback. RELEASE_ALLOW_ANY_BRANCH=1 bypasses only the branch-name
	@# check (for rare hotfix branches) — see docs/BRANCH_PROTECTION.md.
	@if [ "$(RELEASE_ALLOW_ANY_BRANCH)" = "1" ]; then \
		echo "$(YELLOW)⚠  RELEASE_ALLOW_ANY_BRANCH=1 — skipping main-branch check (CI still enforces main-only).$(NC)"; \
	else \
		CURRENT_BRANCH=$$(git rev-parse --abbrev-ref HEAD); \
		if [ "$$CURRENT_BRANCH" != "main" ]; then \
			echo "$(RED)❌ Releases must be cut from 'main' (current branch: $$CURRENT_BRANCH).$(NC)"; \
			echo "$(RED)   Merge your changes via PR, then 'git checkout main && git pull --ff-only'.$(NC)"; \
			echo "$(RED)   Override (hotfix only): RELEASE_ALLOW_ANY_BRANCH=1. See docs/BRANCH_PROTECTION.md.$(NC)"; \
			exit 1; \
		fi; \
	fi
	@echo "$(CYAN)Verifying local branch is up to date with origin/main...$(NC)"
	@git fetch --quiet origin main || { echo "$(RED)❌ Failed to fetch origin/main.$(NC)"; exit 1; }
	@if [ -n "$$(git rev-list HEAD..origin/main)" ]; then \
		echo "$(RED)❌ HEAD is behind origin/main. Run 'git pull --ff-only origin main' before releasing.$(NC)"; \
		exit 1; \
	fi
	@echo "$(CYAN)Cutting release v$(VERSION)...$(NC)"
	@$(MAKE) release-check
	@echo "$(CYAN)Bumping all crates to $(VERSION) (lockstep)...$(NC)"
	@$(CARGO) release version "$(VERSION)" --execute --no-confirm --workspace
	@echo "$(CYAN)Finalizing CHANGELOG.md...$(NC)"
	@DATE=$$(date +%Y-%m-%d); \
		perl -0pi -e "s/## \\[Unreleased\\]/## [Unreleased]\n\n## [$(VERSION)] - $$DATE/" CHANGELOG.md
	@echo "$(CYAN)Committing, tagging, and pushing...$(NC)"
	@git add -u
	@git commit -m "chore(release): version $(VERSION)"
	@git tag -a "v$(VERSION)" -m "Release $(VERSION)"
	@git push origin HEAD
	@git push origin "v$(VERSION)"
	@echo "$(GREEN)✅ Release v$(VERSION) tagged and pushed. CI (release.yml) will publish to crates.io.$(NC)"

##@ Monitoring & Debug

.PHONY: health
health: ## Check service health
	@echo "$(CYAN)Checking service health...$(NC)"
	@echo "Redis:"
	@curl -f http://localhost:6379 2>/dev/null && echo "✅ Redis OK" || echo "❌ Redis DOWN"
	@echo "MinIO:"
	@curl -f http://localhost:9000/minio/health/live 2>/dev/null && echo "✅ MinIO OK" || echo "❌ MinIO DOWN"
	@echo "Application:"
	@curl -f http://localhost:8080/health 2>/dev/null && echo "✅ App OK" || echo "❌ App DOWN or no health endpoint"

.PHONY: bench
bench: ## Run benchmarks
	@echo "$(CYAN)Running benchmarks...$(NC)"
	@$(CARGO) bench --workspace

.PHONY: profile
profile: ## Profile the application
	@echo "$(CYAN)Profiling application...$(NC)"
	@$(CARGO) build --release
	@echo "Run: perf record target/release/$(PROJECT_NAME)"
	@echo "Then: perf report"

##@ DevContainer

.PHONY: devcontainer-build
devcontainer-build: ## Build the DevContainer image
	@echo "$(CYAN)Building DevContainer image...$(NC)"
	@$(DOCKER) build -f .devcontainer/Dockerfile.dev -t $(PROJECT_NAME)-devcontainer:latest .
	@echo "$(GREEN)✅ DevContainer image built$(NC)"

.PHONY: devcontainer-validate
devcontainer-validate: ## Validate DevContainer setup
	@echo "$(CYAN)Validating DevContainer...$(NC)"
	@.devcontainer/validate.sh

.PHONY: devcontainer-network
devcontainer-network: ## Create DevContainer network
	@.devcontainer/setup-network.sh

.PHONY: devcontainer-services
devcontainer-services: ## Start DevContainer services
	@echo "$(CYAN)Starting DevContainer services...$(NC)"
	@$(DOCKER_COMPOSE) -f .devcontainer/docker-compose.yml up -d redis minio mysql
	@echo "$(GREEN)✅ Services started$(NC)"

.PHONY: devcontainer-services-down
devcontainer-services-down: ## Stop DevContainer services
	@echo "$(CYAN)Stopping DevContainer services...$(NC)"
	@$(DOCKER_COMPOSE) -f .devcontainer/docker-compose.yml down
	@echo "$(GREEN)✅ Services stopped$(NC)"

.PHONY: devcontainer-push
devcontainer-push: ## Push DevContainer image to registry
	@echo "$(CYAN)Pushing DevContainer image...$(NC)"
	@$(DOCKER) tag $(PROJECT_NAME)-devcontainer:latest ghcr.io/df3ndr/$(PROJECT_NAME)-devcontainer:latest
	@$(DOCKER) push ghcr.io/df3ndr/$(PROJECT_NAME)-devcontainer:latest
	@echo "$(GREEN)✅ Image pushed$(NC)"

##@ Help

.PHONY:
