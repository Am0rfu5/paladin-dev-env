# Contributing to Paladin

Thank you for your interest in contributing to Paladin! This document provides guidelines and best practices for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Testing Guidelines](#testing-guidelines)
- [Code Quality Standards](#code-quality-standards)
- [Documentation](#documentation)
- [API Change Process](#api-change-process)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and considerate in all interactions.

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later (install via [rustup](https://rustup.rs/))
- **Docker**: For running integration tests with Redis, MinIO, MySQL
- **Git**: For version control

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/DF3NDR/paladin-dev-env.git
cd paladin

# Build the project
cargo build

# Run unit tests
cargo test

# Start service dependencies
make dev  # or docker-compose -f docker/docker-compose.dev.yml up -d
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test improvements

### 2. Make Your Changes

Follow the [Rust coding conventions](#rust-coding-conventions) and ensure your code:
- Compiles without errors
- Passes all tests
- Is properly formatted (`cargo fmt`)
- Has no clippy warnings (`cargo clippy`)

### 3. Write Tests

All code changes must include appropriate tests. See [Testing Guidelines](#testing-guidelines) below.

### 4. Run Quality Checks

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Run all tests
cargo test

# Run integration tests
make test-integration-docker
```

### 5. Commit Your Changes

Use conventional commit messages:

```bash
git commit -m "feat: add Council discussion pattern"
git commit -m "fix: resolve timeout in Phalanx aggregation"
git commit -m "docs: update Garrison memory documentation"
git commit -m "test: add integration tests for Grove routing"
```

Commit types:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/improvements
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Build/tooling changes

### 6. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear description of changes
- Link to related issues
- Test results
- Screenshots (if applicable)

## Testing Guidelines

Paladin uses comprehensive testing to ensure reliability and quality. All contributions must include appropriate tests.

### Test-Driven Development (TDD)

We follow the **Red-Green-Refactor** cycle:

1. **Red**: Write a failing test first
2. **Green**: Write minimal code to pass the test
3. **Refactor**: Improve code while keeping tests green

### Test Coverage Requirements

- **Unit tests**: ≥ 80% coverage for new code
- **Integration tests**: ≥ 70% coverage for public APIs
- **All public APIs must have doc tests**

### Test Types

#### 1. Unit Tests

Test individual functions, methods, and modules in isolation.

**Location**: Inline with code using `#[cfg(test)]` module or in `tests/unit/`

**Example**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paladin_builder_creates_valid_agent() {
        let llm_port = Arc::new(MockLlmAdapter::new());
        let paladin = PaladinBuilder::new(llm_port)
            .name("TestAgent")
            .system_prompt("Test prompt")
            .build()
            .expect("Should build successfully");

        assert_eq!(paladin.data.name, "TestAgent");
    }

    #[tokio::test]
    async fn test_council_executes_discussion() {
        // Test async code
        let result = council_service.execute(&council, &paladins, "input").await;
        assert!(result.is_ok());
    }
}
```

**Run unit tests**:
```bash
cargo test
cargo test test_name  # Run specific test
cargo test module_name::  # Run tests in module
```

#### 2. Integration Tests

Test interactions between multiple components, including external services (databases, LLMs, etc.).

**Location**: `tests/integration/`

**Example**:

```rust
// tests/integration/garrison_tests.rs
#[tokio::test]
async fn test_sqlite_garrison_persistence() {
    let garrison = SqliteGarrison::new("test.db").await.unwrap();

    garrison.store_message("paladin1", Message::User("Hello".into())).await.unwrap();
    let history = garrison.get_history("paladin1", 10).await.unwrap();

    assert_eq!(history.len(), 1);
}
```

**Run integration tests**:
```bash
cargo test --test integration_test_name
make test-integration-docker  # With Docker services
```

#### 3. Snapshot Tests

Test CLI output consistency using the [`insta`](https://insta.rs/) crate.

**Location**: `tests/cli/`

**Example**:

```rust
use insta::assert_snapshot;

#[test]
fn test_help_output() {
    let output = run_cli_command(&["--help"]);
    assert_snapshot!("help_text", output);
}
```

**Review snapshots**:
```bash
cargo test  # Run tests
cargo insta review  # Review new/changed snapshots
cargo insta accept  # Accept all snapshot changes
```

**Best practices**:
- Use descriptive snapshot names
- Keep snapshots small and focused
- Review snapshot changes carefully before accepting
- Commit snapshot files (`.snap`) to version control

#### 4. Live API Integration Tests

Test real LLM provider integrations (optional, requires API keys).

**Location**: `tests/integration/llm_live_api_tests.rs`

**Feature flag**: `live-api-tests`

**Recommended in DevContainer (persistent workflow)**:

```bash
cp .env.example .env
# Edit .env and set one or more keys:
# OPENAI_API_KEY=sk-...
# DEEPSEEK_API_KEY=...
# ANTHROPIC_API_KEY=...

# Load .env for current terminal session
set -a
. /workspace/.env
set +a
```

**Run live API tests**:
```bash
cargo test --features live-api-tests -- --ignored --nocapture
```

**Run only one provider**:

```bash
cargo test --features live-api-tests test_openai -- --ignored --nocapture
cargo test --features live-api-tests test_deepseek -- --ignored --nocapture
cargo test --features live-api-tests test_anthropic -- --ignored --nocapture
```

**Without API keys, tests will be ignored/skipped**:
```bash
cargo test --features live-api-tests
# Tests remain ignored unless --ignored is supplied
```

#### 5. Benchmark Tests

Performance benchmarks using Criterion.

**Location**: `benches/`

**Example**:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_formation(c: &mut Criterion) {
    c.bench_function("formation_3_agents", |b| {
        b.iter(|| {
            // Benchmark code
            black_box(formation.execute(input).await);
        });
    });
}

criterion_group!(benches, benchmark_formation);
criterion_main!(benches);
```

**Run benchmarks**:
```bash
cargo bench  # Run all benchmarks
cargo bench --no-run  # Check compilation only
```

### Running Different Test Types

```bash
# All tests
cargo test --all-features

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test file
cargo test --test garrison_tests

# With output
cargo test -- --nocapture

# Live API tests (requires API keys)
cargo test --features live-api-tests

# Benchmarks
cargo bench

# With coverage
cargo llvm-cov --html --output-dir target/coverage
cargo tarpaulin --out Html
```

### Mocking and Test Doubles

For testing code that depends on external services, create mocks:

```rust
use async_trait::async_trait;

struct MockLlmAdapter {
    responses: Vec<String>,
}

#[async_trait]
impl LlmPort for MockLlmAdapter {
    async fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        Ok(LlmResponse {
            content: self.responses[0].clone(),
            // ... other fields
        })
    }
}

// Use in tests
let mock = Arc::new(MockLlmAdapter::new());
let paladin = PaladinBuilder::new(mock).build()?;
```

### Test Organization

```
tests/
├── unit/              # Unit tests (if not inline)
│   ├── mod.rs
│   └── paladin_test.rs
├── integration/       # Integration tests
│   ├── mod.rs
│   ├── garrison_tests.rs
│   ├── arsenal_tests.rs
│   └── battalion_tests.rs
├── cli/               # CLI snapshot tests
│   ├── mod.rs
│   ├── table_output_test.rs
│   ├── error_output_test.rs
│   └── snapshots/     # Snapshot files (.snap)
└── fixtures/          # Test data and fixtures
    └── sample_data.json
```

## Code Quality Standards

### Rust Coding Conventions

1. **Follow Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
2. **Use `rustfmt`**: Automatic code formatting
3. **Use `clippy`**: Catch common mistakes
4. **Document public APIs**: All public items need rustdoc comments

### Code Formatting

```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt --check
```

Configuration in `rustfmt.toml`:
- Max width: 100 characters
- Use tabs: false (4 spaces)
- Edition: 2021

### Linting

```bash
# Run clippy with warnings as errors
cargo clippy -- -D warnings

# Fix auto-fixable issues
cargo clippy --fix
```

### Documentation

All public items must have documentation:

```rust
/// Creates a new Paladin agent with the specified configuration.
///
/// # Arguments
///
/// * `llm_port` - The LLM provider port for agent execution
///
/// # Returns
///
/// A configured `PaladinBuilder` instance
///
/// # Examples
///
/// ```
/// use paladin::prelude::*;
///
/// let builder = PaladinBuilder::new(llm_port)
///     .name("Assistant")
///     .system_prompt("You are helpful");
/// ```
pub fn new(llm_port: Arc<dyn LlmPort>) -> Self {
    // implementation
}
```

Generate and view documentation:
```bash
cargo doc --no-deps --open
```

### Security

- **Never commit API keys or secrets**
- Use environment variables for configuration
- Add sensitive values to `.gitignore`
- Run security audits: `cargo audit`

## Documentation

### Types of Documentation

1. **Code Documentation** (rustdoc)
   - Document all public APIs
   - Include examples in doc comments
   - Explain complex algorithms

2. **User Guides** (`docs/`)
   - Installation instructions
   - Quickstart guides
   - Feature documentation
   - Examples and tutorials

3. **Architecture Documentation** (`docs/Design/`)
   - System architecture
   - Design decisions
   - Technical specifications

4. **API Documentation** (generated)
   - Comprehensive API reference
   - Generated from rustdoc comments

### Documentation Guidelines

- Write clear, concise documentation
- Include code examples
- Keep documentation up-to-date with code changes
- Use proper markdown formatting
- Add diagrams where helpful

## API Change Process

Paladin maintains a **stable public API contract** defined in **[STABLE_API.md](STABLE_API.md)**. This document defines:

- **Stability guarantees** for all public types and traits
- **Versioning policy** (semantic versioning interpretation)
- **Stability tiers** (Stable 🟢, Unstable 🟡, Experimental 🔵, Deprecated 🔴)
- **Catalog of stable APIs** with fully qualified paths
- **Change approval process** for breaking changes
- **Migration guides** and deprecation lifecycle

**All changes to the public API must follow the process below.** See [STABLE_API.md](STABLE_API.md) for complete details on API stability and the catalog of stable types.

### What is Considered a Public API Change?

Changes to any of the following require the API change process:

- **Port traits** (all traits in `src/application/ports/`)
- **Domain entities** (types in `src/core/platform/container/`)
- **Builders** (PaladinBuilder, CommanderBuilder, etc.)
- **Configuration types** (ApplicationSettings, etc.)
- **Error types** (all public error enums)
- **Public exports** from `src/lib.rs`

### Process for Non-Breaking API Changes

**Non-breaking changes** include:
- Adding new methods with default implementations to traits
- Adding new types/modules
- Adding new optional parameters with defaults
- Expanding enum variants (with `#[non_exhaustive]`)

**Steps:**
1. Make the changes
2. Add comprehensive rustdoc with examples
3. Run API tracking: `./scripts/extract-public-api.sh`
4. Review the diff: `./scripts/check-api-surface.sh`
5. Update `CHANGELOG.md` under "Added" section
6. Submit PR with "feat:" prefix
7. After approval, update baseline: `./scripts/extract-public-api.sh project/current-exports.txt`

### Process for Breaking API Changes

**Breaking changes** include:
- Removing public types, traits, or methods
- Changing method signatures
- Removing trait methods
- Changing error types
- Renaming public items

**Steps:**
1. **Open an Issue First**
   - Describe the breaking change
   - Explain the motivation
   - Propose the migration path
   - Get consensus from maintainers

2. **Add Deprecation Warning (for removals)**
   ```rust
   #[deprecated(since = "0.2.0", note = "Use `NewType` instead. See MIGRATION.md for details.")]
   pub struct OldType { /* ... */ }
   ```

3. **Update Documentation**
   - Add migration guide to `docs/MIGRATION.md`
   - Update `STABLE_API.md` with new API
   - Update all examples
   - Update rustdoc with examples

4. **Run Deprecation Checks**
   ```bash
   ./scripts/check-deprecations.sh
   ```

5. **Update CHANGELOG**
   - Add entry under "Breaking Changes" section
   - Link to migration guide

6. **Submit PR**
   - Use "feat!:" or "fix!:" prefix (note the `!`)
   - Include breaking change details in PR description
   - Reference the tracking issue

7. **After Approval**
   - Update API baseline: `./scripts/extract-public-api.sh project/current-exports.txt`
   - Version will be bumped according to semver (0.x.0 → 0.y.0 or x.0.0 → y.0.0)

### API Tracking Scripts

```bash
# Extract current public API surface
./scripts/extract-public-api.sh project/current-exports.txt

# Check for API changes (CI uses this)
./scripts/check-api-surface.sh project/current-exports.txt

# Verify deprecation warnings compile correctly
./scripts/check-deprecations.sh
```

### CI Enforcement

The CI pipeline automatically:
- Checks for API surface changes
- Fails if API changed without updating baseline
- Validates deprecation warnings compile
- Ensures all public items have rustdoc

If CI fails due to API changes:
1. Review the diff shown in CI output
2. Verify changes are intentional
3. Follow the appropriate process above
4. Update the baseline if approved

### Examples of API Changes

**✅ Non-Breaking - Adding Optional Method**:
```rust
pub trait LlmPort: Send + Sync {
    async fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;

    // New method with default implementation
    async fn generate_with_retry(&self, request: &LlmRequest, retries: u32) -> Result<LlmResponse, LlmError> {
        // Default implementation
        self.generate(request).await
    }
}
```

**❌ Breaking - Changing Method Signature**:
```rust
// Old
async fn generate(&self, prompt: &str) -> Result<String, LlmError>;

// New (BREAKING!)
async fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;
```

**✅ Correct Way - Deprecate Then Remove**:
```rust
// Version 0.1.0 - Original
async fn generate(&self, prompt: &str) -> Result<String, LlmError>;

// Version 0.2.0 - Add new, deprecate old
#[deprecated(since = "0.2.0", note = "Use `generate_with_request` instead")]
async fn generate(&self, prompt: &str) -> Result<String, LlmError>;
async fn generate_with_request(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;

// Version 1.0.0 - Remove deprecated
async fn generate_with_request(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;
```

### Questions?

For questions about API changes:
- Review [STABLE_API.md](STABLE_API.md)
- Open an issue with the `api-stability` label
- Ask in GitHub Discussions

## Pull Request Process

### Before Submitting

1. ✅ All tests pass (`cargo test --all-features`)
2. ✅ Code is formatted (`cargo fmt --check`)
3. ✅ No clippy warnings (`cargo clippy -- -D warnings`)
4. ✅ Documentation is updated
5. ✅ Commit messages follow conventions
6. ✅ Branch is up-to-date with main/develop

### PR Description Template

```markdown
## Description
Brief description of changes

## Motivation
Why is this change necessary?

## Changes
- List of changes made
- Breaking changes (if any)

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests pass
- [ ] Benchmarks run (if applicable)

## Documentation
- [ ] README updated
- [ ] API documentation updated
- [ ] Examples added/updated

## Checklist
- [ ] Code follows project conventions
- [ ] Tests pass locally
- [ ] No clippy warnings
- [ ] Documentation complete
```

### Review Process

1. Automated checks run (CI/CD)
2. Code review by maintainers
3. Address review feedback
4. Approval and merge

## Community

### Getting Help

- **Documentation**: [docs/README.md](docs/README.md)
- **Examples**: [examples/](examples/)
- **Issues**: [GitHub Issues](https://github.com/DF3NDR/paladin-dev-env/issues)
- **Discussions**: [GitHub Discussions](https://github.com/DF3NDR/paladin-dev-env/discussions)

### Reporting Issues

When reporting issues, include:
- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Error messages and stack traces

### Feature Requests

Feature requests are welcome! Please:
- Search existing issues first
- Describe the use case
- Explain why the feature is valuable
- Consider contributing the implementation

## License

By contributing to Paladin, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Paladin! 🏰
