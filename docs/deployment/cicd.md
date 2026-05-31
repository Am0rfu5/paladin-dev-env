# CI/CD Guide

Complete guide for setting up continuous integration and deployment pipelines for Paladin using GitHub Actions.

## Table of Contents

- [Overview](#overview)
- [GitHub Actions Workflows](#github-actions-workflows)
- [CI Pipeline](#ci-pipeline)
- [Docker Build Pipeline](#docker-build-pipeline)
- [Release Pipeline](#release-pipeline)
- [Integration Testing](#integration-testing)
- [Security Scanning](#security-scanning)
- [Deployment Automation](#deployment-automation)
- [Best Practices](#best-practices)

## Overview

Paladin uses GitHub Actions for CI/CD with the following pipelines:
- **CI**: Build, test, lint on every PR
- **Docker**: Build and publish multi-arch images
- **Release**: Automated releases with semantic versioning
- **Integration**: Integration tests with Docker services
- **Security**: Dependency scanning and vulnerability checks

## GitHub Actions Workflows

### Workflow Structure

```
.github/
├── workflows/
│   ├── ci.yml                    # Main CI pipeline
│   ├── docker-publish.yml        # Docker image builds
│   ├── release.yml               # Release automation
│   ├── integration-tests.yml     # Integration testing
│   └── security.yml              # Security scanning
└── dependabot.yml                # Dependency updates
```

## CI Pipeline

### ci.yml

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Check
        run: cargo check --all-features

  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust ${{ matrix.rust }}
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}

      - name: Run tests
        run: cargo test --all-features

      - name: Run doc tests
        run: cargo test --doc --all-features

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview

      - name: Install cargo-llvm-cov
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Generate coverage
        run: cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

      - name: Upload to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
          fail_ci_if_error: true
```

## Docker Build Pipeline

### docker-publish.yml

```yaml
name: Docker

on:
  push:
    branches: [ main ]
    tags: [ 'v*.*.*' ]
  pull_request:
    branches: [ main ]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Set up QEMU
        uses: docker/setup-qemu-action@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Log in to Container Registry
        if: github.event_name != 'pull_request'
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          tags: |
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{version}}
            type=semver,pattern={{major}}.{{minor}}
            type=semver,pattern={{major}}
            type=sha
            type=raw,value=latest,enable={{is_default_branch}}

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: ${{ github.event_name != 'pull_request' }}
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

## Release Pipeline

### release.yml

```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

permissions:
  contents: write
  packages: write

jobs:
  build-release:
    name: Build Release
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross-compilation tools (Linux ARM64)
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: |
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package (Unix)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar czf paladin-${{ github.ref_name }}-${{ matrix.target }}.tar.gz paladin
          mv paladin-${{ github.ref_name }}-${{ matrix.target }}.tar.gz ${{ github.workspace }}/

      - name: Package (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          7z a paladin-${{ github.ref_name }}-${{ matrix.target }}.zip paladin.exe
          move paladin-${{ github.ref_name }}-${{ matrix.target }}.zip ${{ github.workspace }}/

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: release-${{ matrix.target }}
          path: |
            paladin-*.tar.gz
            paladin-*.zip

  create-release:
    name: Create Release
    needs: build-release
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download artifacts
        uses: actions/download-artifact@v3

      - name: Generate changelog
        id: changelog
        run: |
          # Extract changelog for this version
          VERSION="${{ github.ref_name }}"
          awk "/^## \[$VERSION\]/,/^## \[/" CHANGELOG.md | head -n -1 > release_notes.md

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            release-*/paladin-*.tar.gz
            release-*/paladin-*.zip
          body_path: release_notes.md
          draft: false
          prerelease: ${{ contains(github.ref_name, '-') }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Integration Testing

### integration-tests.yml

```yaml
name: Integration Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday

jobs:
  integration-tests:
    name: Integration Tests
    runs-on: ubuntu-latest

    services:
      redis:
        image: redis:7-alpine
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 6379:6379

      minio:
        image: minio/minio:latest
        env:
          MINIO_ROOT_USER: minioadmin
          MINIO_ROOT_PASSWORD: minioadmin
        options: >-
          --health-cmd "curl -f http://localhost:9000/minio/health/live"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 9000:9000

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Wait for services
        run: |
          timeout 60 bash -c 'until curl -f http://localhost:9000/minio/health/live; do sleep 2; done'
          timeout 60 bash -c 'until redis-cli -h localhost ping; do sleep 2; done'

      - name: Run integration tests
        run: cargo test --features integration-tests --test '*_integration_test'
        env:
          REDIS_URL: redis://localhost:6379
          MINIO_ENDPOINT: localhost:9000
          MINIO_ACCESS_KEY: minioadmin
          MINIO_SECRET_KEY: minioadmin
          RUST_LOG: debug

      - name: Integration test coverage
        run: |
          cargo install cargo-llvm-cov
          cargo llvm-cov --features integration-tests --test '*_integration_test' --lcov --output-path integration-lcov.info

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: integration-lcov.info
          flags: integration
```

## Security Scanning

### security.yml

```yaml
name: Security

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * 1'  # Weekly on Monday

jobs:
  audit:
    name: Cargo Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install cargo-audit
        run: cargo install cargo-audit

      - name: Run cargo audit
        run: cargo audit

  deny:
    name: Cargo Deny
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install cargo-deny
        run: cargo install cargo-deny

      - name: Run cargo deny
        run: cargo deny check

  snyk:
    name: Snyk Security Scan
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run Snyk
        uses: snyk/actions/rust@master
        env:
          SNYK_TOKEN: ${{ secrets.SNYK_TOKEN }}
        with:
          args: --severity-threshold=high
```

## Deployment Automation

### Deploy to Kubernetes

```yaml
name: Deploy

on:
  push:
    tags:
      - 'v*.*.*'
  workflow_dispatch:
    inputs:
      environment:
        description: 'Environment to deploy to'
        required: true
        type: choice
        options:
          - staging
          - production

jobs:
  deploy:
    name: Deploy to ${{ github.event.inputs.environment || 'production' }}
    runs-on: ubuntu-latest
    environment:
      name: ${{ github.event.inputs.environment || 'production' }}
      url: https://paladin.${{ github.event.inputs.environment || 'prod' }}.example.com

    steps:
      - uses: actions/checkout@v4

      - name: Configure kubectl
        uses: azure/k8s-set-context@v3
        with:
          method: kubeconfig
          kubeconfig: ${{ secrets.KUBE_CONFIG }}

      - name: Deploy with Helm
        run: |
          helm upgrade --install paladin ./paladin-chart \
            --namespace paladin \
            --create-namespace \
            --set image.tag=${{ github.ref_name }} \
            --set secrets.openaiApiKey=${{ secrets.OPENAI_API_KEY }} \
            --values values-${{ github.event.inputs.environment || 'production' }}.yaml \
            --wait

      - name: Verify deployment
        run: |
          kubectl rollout status deployment/paladin -n paladin
          kubectl get pods -n paladin
```

## Best Practices

### 1. Branch Protection

Configure branch protection rules in GitHub:

```yaml
# Required status checks
- CI / check
- CI / test (ubuntu-latest, stable)
- CI / test (macos-latest, stable)
- CI / coverage
- Integration Tests

# Required reviews: 1
# Dismiss stale reviews: true
# Require linear history: true
```

### 2. Secrets Management

Store secrets in GitHub repository settings:

```bash
# Required secrets
GITHUB_TOKEN          # Auto-provided
OPENAI_API_KEY        # For integration tests
SNYK_TOKEN            # For security scanning
KUBE_CONFIG           # For K8s deployment
```

### 3. Caching Strategy

```yaml
# Cache Cargo dependencies
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

### 4. Concurrency Control

```yaml
# Cancel in-progress runs for same PR
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

### 5. Conditional Workflows

```yaml
# Skip CI for docs-only changes
on:
  push:
    paths-ignore:
      - '**.md'
      - 'docs/**'
```

### 6. Matrix Testing

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta, nightly]
  fail-fast: false  # Continue other jobs on failure
```

### 7. Artifact Retention

```yaml
- uses: actions/upload-artifact@v3
  with:
    name: test-results
    path: target/test-results/
    retention-days: 30
```

### 8. Notifications

```yaml
- name: Slack Notification
  if: failure()
  uses: 8398a7/action-slack@v3
  with:
    status: ${{ job.status }}
    webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

## Next Steps

- **[Production Best Practices](production-best-practices.md)** - Production checklist
- **[Monitoring](../operations/monitoring.md)** - Observability setup
- **[Docker Deployment](docker.md)** - Docker deployment guide
