# Changelog

All notable changes to the Paladin project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Epic 17: Flow DSL & Agent Rearrangement (Maneuver Pattern)

#### Flow DSL Parser
- **FlowParser**: String-based workflow orchestration with intuitive syntax
  - Sequential operator `->` for linear workflows (e.g., "A -> B -> C")
  - Parallel operator `,` for concurrent execution (e.g., "A, B, C")
  - Nested patterns with parentheses for complex workflows
  - Complete lexer, AST, and parser implementation in core layer
  - 57 comprehensive tests covering all syntax patterns
- **Error Handling**: Detailed `FlowParseError` types with position tracking
  - Helpful error messages for common syntax mistakes
  - Support for debugging complex nested expressions
  - Suggestion methods for error recovery

#### Maneuver Domain Model
- **Maneuver**: New Battalion pattern for declarative workflow definition
  - Parse flow expressions into executable agent graphs
  - Support for 10-30 agent workflows with automatic dependency resolution
  - Three error strategies: FailFast, ContinueParallel, IgnoreErrors
  - Two output formats: CombinedText, StructuredJson
  - 21 domain tests validating configuration and behavior
- **ManeuverConfig**: Comprehensive configuration with timeouts and validation
  - Per-agent timeout controls
  - Error strategy selection
  - Output format specification
  - Validation rules for agent count and flow complexity

#### Execution Engine
- **ManeuverExecutionService**: Async execution with dependency resolution
  - Parallel execution of independent agents
  - Sequential execution for dependent agents
  - Result aggregation based on output format
  - Error handling with configurable strategies
  - 3 integration tests verifying execution patterns
- **Flow Visualization**: ASCII and Mermaid diagram generation
  - ASCII art for terminal display and documentation
  - Mermaid diagrams for rich visualizations
  - Support for simple, nested, and complex flows
  - 12 tests covering all visualization scenarios

#### Commander Integration
- **Pattern Detection**: Automatic Maneuver pattern recognition
  - Parse flow expressions from input strings
  - Detect sequential and parallel patterns automatically
  - Seamless integration with existing Formation and Phalanx patterns
  - 16 tests for Commander Maneuver integration
- **CLI Commands**: Complete CLI support for Maneuver operations
  - `paladin maneuver create` - Generate Maneuver configurations
  - `paladin maneuver execute` - Execute flow expressions
  - `paladin maneuver validate` - Validate flow syntax
  - `paladin maneuver visualize` - Generate visualizations
  - 4 CLI command tests

#### Documentation & Examples
- **Comprehensive Documentation**: 1,349 lines of new documentation
  - `docs/MANEUVER.md` (1,333 lines) - Complete user guide
  - Updated `docs/BATTALION.md` with Maneuver pattern
  - Updated `docs/CLI_USAGE.md` with Maneuver commands
  - Updated main `README.md` with Maneuver overview
- **Production Examples**: 3 complete working examples (958 lines)
  - `maneuver_basic.rs` - Introduction to Flow DSL
  - `maneuver_nested_flow.rs` - Enterprise review pipeline
  - `maneuver_dynamic_flow.rs` - Runtime flow generation
- **Performance Benchmarks**: 7 benchmark suites (32 test cases)
  - Parse time benchmarks (4 complexity levels)
  - Visualization performance (ASCII and Mermaid)
  - Validation overhead measurement
  - Sequential and parallel execution benchmarks
  - Nested flow performance testing
  - Overhead comparison vs Formation/Phalanx patterns

#### Test Coverage
- **113 Total Tests**: Comprehensive coverage across all components
  - Parser: 57 tests (lexer, AST, error handling)
  - Domain: 21 tests (Maneuver, ManeuverConfig)
  - Execution: 3 tests (ManeuverExecutionService)
  - Commander: 16 tests (pattern detection, integration)
  - Visualization: 12 tests (ASCII, Mermaid)
  - CLI: 4 tests (command validation)
- **Benchmark Coverage**: 32 performance test cases
  - Parse performance: < 1ms for complex flows
  - Execution overhead: < 2% vs direct patterns
  - Memory efficiency validation
  - Scalability testing (3-20 agents)

### Added - Epic 14: Autonomous Agent Features

#### Autonomous Planning Mode
- **Auto Loop Detection**: New `MaxLoops::Auto { max_subtasks: u32 }` variant enables intelligent loop optimization
  - Automatic task complexity analysis
  - Dynamic subtask decomposition for complex tasks  
  - Optimal loop count determination (simple tasks use fewer loops)
- **Planning Service**: New `PlanningService` with comprehensive task planning
  - Task complexity assessment
  - Structured plan generation with subtasks
  - Subtask execution and synthesis
  - Integration with Paladin execution flow
- **Planning Configuration**: `PlanningConfig` with enabled flag, max_subtasks, and complexity threshold
- **Domain Types**: `TaskPlan`, `Subtask`, `ComplexityLevel` for structured planning representation

#### Auto-Generate System Prompts
- **Prompt Generation Service**: New `PromptGenerationService` for LLM-powered prompt creation
  - Generate system prompts from natural language agent descriptions
  - Optimize prompts for specific agent roles and capabilities
  - Cache generated prompts for reuse
  - Support for prompt regeneration and manual overrides
- **Prompt Configuration**: `PromptConfig` with enabled flag and optional cache control
- **Builder Integration**: `agent_description()` method on PaladinBuilder for seamless prompt generation

#### Dynamic Temperature Adjustment
- **Temperature Service**: New `TemperatureService` with task-based temperature optimization
  - Automatic task type classification (factual, creative, balanced)
  - Temperature bounds configuration (min/max range)
  - Classification heuristics based on task keywords
  - Real-time temperature adjustment per task
- **Temperature Configuration**: `TemperatureConfig` with enabled flag, min/max bounds, and custom keywords
- **Task Types**: `TaskType` enum (Factual, Creative, Balanced) with appropriate temperature ranges

#### Intelligent Agent Handoffs
- **Handoff Service**: New `HandoffService` for delegation between specialist agents
  - Specialist discovery and routing
  - Task complexity assessment for delegation
  - Circuit breaker integration for reliability
  - Handoff depth limiting (prevent infinite delegation)
- **Handoff Configuration**: `HandoffConfig` with enabled flag, strategy, and max delegation depth  
- **Handoff Strategies**: `HandoffStrategy` enum (Automatic, ExplicitOnly) for control
- **Domain Types**: `HandoffDecision`, `HandoffMetadata` for structured delegation tracking

#### Handoff Tool Integration
- **Arsenal Integration**: New `HandoffTool` registered in Arsenal for LLM-accessible delegation
  - `delegate_to_specialist` function for explicit handoffs
  - JSON schema for LLM tool use
  - Specialist validation and routing
  - Seamless integration with agent execution loop

#### Configuration & Builder API
- **Autonomous Configuration**: New `AutonomousConfig` aggregating all autonomous features
  - Centralized configuration structure
  - YAML configuration support
  - CLI flag integration
  - Builder pattern support via `PaladinBuilder`
- **Builder Methods**: New autonomous feature methods on PaladinBuilder
  - `enable_planning(bool)` - Toggle autonomous planning
  - `agent_description(String)` - Set description for prompt generation
  - `enable_dynamic_temperature(bool)` - Toggle temperature adjustment
  - `enable_handoffs(bool)` - Toggle delegation capabilities

#### Documentation & Examples  
- **Comprehensive Guide**: New `docs/AUTONOMOUS.md` (400+ lines)
  - Introduction and features overview
  - Detailed user story documentation (all 5 features)
  - Configuration guide (YAML, CLI, Builder)
  - Best practices and performance considerations
  - Error handling and troubleshooting
  - Advanced usage patterns
  - Complete API reference
- **Working Examples**: 5 comprehensive example files (~1,400 lines)
  - `autonomous_planning.rs` - Planning mode with task decomposition
  - `autonomous_prompt_generation.rs` - Auto-prompt generation concepts
  - `dynamic_temperature.rs` - Temperature adjustment by task type
  - `agent_handoffs.rs` - Specialist delegation workflow
  - `autonomous_full_config.rs` - All features combined
- **Examples README**: Updated `examples/README.md` with autonomous section

#### Testing & Quality
- **Comprehensive Testing**: 1,280+ tests passing including autonomous features
  - Unit tests for all services and domain logic
  - Integration tests for Paladin with autonomous features
  - MockLlmAdapter integration for deterministic testing
- **Code Quality**: Zero clippy warnings in strict mode
  - All code formatted with rustfmt
  - Comprehensive rustdoc for all public APIs
  - Error handling with thiserror patterns

#### Security Audit Results
- **Vulnerabilities**: 2 transitive dependency vulnerabilities identified (non-critical)
  - `rsa 0.9.10`: Marvin Attack timing sidechannel (RUSTSEC-2023-0071) - Medium severity, no upgrade available (from sqlx-mysql)
  - `tokio-tar 0.3.1`: PAX header parsing issue (RUSTSEC-2025-0111) - No upgrade available (from testcontainers, dev dependency only)
- **Unmaintained Crates**: 9 warnings about unmaintained transitive dependencies
  - All are indirect dependencies from test/dev dependencies
  - No immediate security risk to production code
  - Monitored for future upgrades when upstream updates available

### Added - Epic 13: Sentinel Vision System

#### Vision API & Multi-Modal Processing
- **Vision Content Types**: Support for three image input formats
  - `ImageUrl`: Process images from public web URLs
  - `ImageFile`: Load and analyze local image files with automatic base64 encoding
  - `ImageBase64`: Direct base64-encoded image input
- **Vision-Enabled Paladins**: New `enable_vision()` builder method and `execute_with_vision()` function
- **Image Detail Levels**: Control token usage and analysis depth
  - `Low`: ~85 tokens, fast processing for simple tasks
  - `High`: 170+ tokens, detailed analysis with fine-grained details
  - `Auto`: Automatic balancing based on image complexity
- **Multi-Provider Support**: Vision capabilities across LLM providers
  - OpenAI: GPT-4o, GPT-4o-mini with vision support
  - Anthropic: Claude 3 Opus, Sonnet, Haiku with vision capabilities

#### Document Processing System
- **PDF Extraction**: Comprehensive PDF text extraction via `PdfExtractor`
  - Multi-page document support
  - Metadata extraction (title, author, creation date, page count)
  - Character-accurate text extraction
  - Page-by-page content access
- **Intelligent Document Chunking**: Flexible chunking strategies via `ChunkConfig`
  - Configurable chunk sizes (characters per chunk)
  - Overlap control for context preservation
  - Custom separators (paragraphs, sentences, custom delimiters)
  - Three built-in configurations:
    - RAG-optimized: 500 chars, 100 overlap, paragraph-based
    - Summarization: 2000 chars, 200 overlap, paragraph-based
    - Sentence-based: 300 chars, 50 overlap, sentence-based
- **DocumentPort Interface**: Clean abstraction for document operations
  - Extract metadata and content from PDFs
  - Chunk documents with configurable strategies
  - Extensible to other document formats

#### Security & Data Protection
- **Vision Data Encryption**: ChaCha20-Poly1305 authenticated encryption
  - Secure at-rest encryption for image data
  - Automatic encryption for `ImageFile` and `ImageBase64` types
  - Decryption utilities for secure data access
- **Data Retention Policies**: Configurable retention for sensitive vision data
  - Time-based retention (e.g., 30 days)
  - Automatic cleanup of expired encrypted data
  - Audit logging for compliance
- **Audit Logging**: Comprehensive tracking of vision operations
  - Document processing events (PDF extraction, chunking)
  - Vision API calls (provider, model, image count, tokens)
  - Encryption/decryption operations
  - Security-related events (data retention, cleanup)

#### CLI Integration
- **Vision Analysis Commands**:
  ```bash
  paladin vision analyze --image path/to/image.jpg --prompt "Describe this image"
  paladin vision analyze --url https://example.com/image.jpg --detail high
  paladin vision batch --directory images/ --prompt "Classify image"
  ```
- **Document Processing Commands**:
  ```bash
  paladin document extract --pdf document.pdf --output text
  paladin document chunk --pdf report.pdf --chunk-size 500 --overlap 100
  paladin document analyze --pdf paper.pdf --prompt "Summarize key findings"
  ```
- **Security Commands**:
  ```bash
  paladin vision encrypt --image sensitive.jpg --output encrypted.bin
  paladin vision decrypt --input encrypted.bin --output decrypted.jpg
  paladin security audit --filter vision --since "30 days ago"
  ```

#### YAML Configuration Support
- **Vision Configuration Section**:
  ```yaml
  vision:
    default_detail: "auto"
    max_images_per_request: 10
    supported_formats: ["png", "jpg", "jpeg", "gif", "webp"]
    enable_encryption: true
  ```
- **Document Processing Configuration**:
  ```yaml
  document:
    pdf:
      max_pages: 1000
      chunk_size: 500
      chunk_overlap: 100
      separator: "\n\n"
  ```
- **Security Configuration**:
  ```yaml
  security:
    vision:
      encryption_enabled: true
      data_retention_days: 30
      audit_logging: true
  ```

#### Battalion Integration
- **Formation Pattern**: Sequential vision pipelines
  - Example: Image Analyzer → Detail Extractor → Insight Generator
  - Output of each stage feeds into the next
  - Perfect for multi-stage vision analysis workflows
- **Phalanx Pattern**: Parallel image processing
  - Process multiple images concurrently with ~3x speedup
  - Each image analyzed by a separate vision-enabled Paladin
  - Results aggregated at completion
- **Campaign Pattern**: Graph-based vision workflows
  - Complex vision processing DAGs
  - Conditional branching based on vision analysis results
  - Mix vision and non-vision tasks in same graph
- **Chain of Command Pattern**: Hierarchical vision delegation
  - Commander Paladin delegates vision tasks to specialist Paladins
  - Automatic load balancing across vision-capable Paladins
  - Escalation for complex or ambiguous visual content

#### Documentation
- **Comprehensive Guide**: `docs/SENTINEL.md` (600+ lines)
  - 13 major sections covering entire vision system
  - Getting started tutorials
  - Supported providers and models
  - Paladin Vision API reference
  - Document processing workflows
  - CLI usage with 8+ command examples
  - YAML configuration templates
  - Security best practices
  - Battalion integration patterns
  - Error handling strategies
  - Performance optimization tips
  - Troubleshooting guide (7 common issues)
- **Code Examples**: Three comprehensive working examples
  - `examples/vision_analysis.rs`: Single-image analysis with detail levels (200 lines)
  - `examples/document_processing.rs`: PDF extraction and chunking strategies (280 lines)
  - `examples/vision_battalion.rs`: Formation and Phalanx patterns (320 lines)
- **README Updates**: Vision & Multi-Modal Processing section
  - Key features overview
  - Quick start code samples
  - Supported content types
  - Document processing examples
  - CLI command references
  - Battalion integration notes
  - Links to comprehensive documentation

### Technical Details

#### Architecture
- **Hexagonal Architecture Compliance**: All vision components follow ports/adapters pattern
  - Vision domain entities in `core/platform/container/`
  - Vision port definitions in `application/ports/output/`
  - Provider-specific adapters in `infrastructure/adapters/llm/`
- **Test-Driven Development**: Comprehensive test coverage
  - 1146 library tests passing (including vision tests)
  - Unit tests for all vision content types
  - Integration tests with mocked API responses
  - Error path testing for invalid formats
  - Security tests for encryption/decryption

#### Dependencies
- **New Dependencies**:
  - `pdf-extract`: PDF text extraction
  - `lopdf`: Low-level PDF manipulation
  - Additional cryptographic dependencies for vision encryption

#### Performance
- **Benchmarks Available**: Vision-specific performance tests
  - Image encoding/decoding: ~50ms per 2MB image
  - Batch processing: ~3x speedup with Phalanx pattern
  - PDF extraction: ~200ms per 100-page document
  - Document chunking: ~10ms for 10k character document

### Security

#### Known Vulnerabilities (from `cargo audit`)
- **RUSTSEC-2023-0071**: RSA timing sidechannel in `rsa 0.9.10` (Medium severity)
  - **Impact**: Potential key recovery through timing attacks
  - **Source**: Transitive dependency via `sqlx-mysql`
  - **Status**: No fixed upgrade available
  - **Mitigation**: Affects MySQL TLS certificate validation (optional feature)
  - **Risk Assessment**: Low for Paladin use case (MySQL connections are internal)
  
- **RUSTSEC-2025-0111**: tokio-tar PAX header parsing vulnerability
  - **Impact**: File smuggling attacks via malformed TAR archives
  - **Source**: Dev dependency via `testcontainers`
  - **Status**: No fixed upgrade available
  - **Mitigation**: Only used in test environment, not production code
  - **Risk Assessment**: Low (development-only dependency)

#### Unmaintained Dependencies (Warnings)
- `ansi_term 0.12.1` (via structopt): Consider migrating to `clap 4.x`
- `atty 0.2.14` (via structopt): Replaced by `is-terminal` in modern Rust
- `dotenv 0.15.0`: Consider migrating to `dotenvy`
- `fxhash 0.2.1` (via scraper): Low risk, internal to scraper crate
- `gcc 0.3.55` (via fasthash-sys): Build-time only dependency
- `number_prefix 0.4.0` (via indicatif): No security impact
- `proc-macro-error 1.0.4` (via structopt): Compile-time only
- `rustls-pemfile 2.2.0` (via testcontainers): Dev dependency only

**Action Plan**: Monitor for updates to `sqlx` and consider migrating from `structopt` to `clap 4.x` in future release.

### Testing

#### Test Coverage
- **Total Tests**: 1146 passing (0 failed)
- **Test Execution**: 7.33s for full library test suite
- **Coverage**: ≥80% for vision and document modules
  - Vision content types: 100% coverage
  - Document extraction: 95% coverage
  - Security (encryption): 90% coverage

#### Test Categories
- **Unit Tests**: 1000+ tests for core functionality
- **Integration Tests**: Mocked API responses for vision providers
- **Security Tests**: Encryption, decryption, audit logging
- **Error Path Tests**: Invalid formats, corrupted data, missing files

### Code Quality

#### Static Analysis
- **Clippy**: PASSED with `-D warnings` (library code)
- **Formatting**: PASSED `cargo fmt --check`
- **Compilation**: CLEAN with `cargo check --all-features`

#### Documentation Quality
- All public APIs have rustdoc comments
- Three comprehensive code examples (800+ lines total)
- User guide with 13 major sections
- Troubleshooting guide with 7 common issues

### Breaking Changes
None. All changes are additive and backward compatible.

### Migration Guide
No migration required. Existing Paladin code works without modification.

To use new vision features:
```rust
// Enable vision on a Paladin
let paladin = PaladinBuilder::new(llm_port)
    .system_prompt("You are a vision-enabled AI assistant")
    .enable_vision(true)
    .build()?;

// Process images
let content = vec![VisionContent::ImageUrl {
    url: "https://example.com/image.jpg".to_string(),
    detail: ImageDetail::Auto,
}];

let result = service.execute_with_vision(&paladin, "Describe this image", content).await?;
```

### Contributors
- John Amatulli (jamatulli) - Epic 13 implementation and documentation

---

## [0.1.0] - Previous Releases

### Added
- Core Paladin platform with Hexagonal Architecture
- Multi-provider LLM support (OpenAI, DeepSeek, Anthropic)
- Battalion orchestration patterns (Formation, Phalanx, Campaign, Chain of Command)
- Arsenal MCP integration for external tools
- Garrison memory and context system
- Citadel state persistence
- Herald output formatting
- Comprehensive CLI (paladin-cli)
- User management system with authentication
- Content processing pipeline
- Redis queue integration
- MinIO file storage integration
- MySQL and SQLite repository support
- Security features (TLS verification, audit logging)
- Docker development environment

### Technical Foundation
- Test-Driven Development (TDD) methodology
- Domain-Driven Design (DDD) principles
- Three-layer hexagonal architecture
- Comprehensive test suite (1146+ tests)
- Continuous integration ready

[Unreleased]: https://github.com/jamatulli/paladin/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jamatulli/paladin/releases/tag/v0.1.0
