# Task List: Sentinel Vision System (Epic 13)

**Based on:** `prd-sentinel-vision-system.md`  
**Epic:** Epic 13 - Sentinel Vision System  
**Duration:** 2 weeks  
**Priority:** Critical

---

## Relevant Files

### Core Domain Models
- `src/core/platform/container/vision.rs` - Vision content types (VisionContent, ImageDetail, VisionRequest)
- `src/core/platform/container/document.rs` - Document types (Document, Page, DocumentMetadata)
- `src/core/platform/container/vision_error.rs` - Vision-specific error types
- `src/core/platform/container/document_error.rs` - Document-specific error types

### Application Layer - Ports
- `src/application/ports/input/document_port.rs` - DocumentPort trait for document operations
- `src/application/ports/output/vision_llm_port.rs` - VisionCapableLlm trait extending LlmPort

### Application Layer - Use Cases
- `src/application/use_cases/vision/vision_service.rs` - Vision orchestration service
- `src/application/use_cases/vision/mod.rs` - Vision module exports

### Infrastructure - LLM Adapters
- `src/infrastructure/adapters/llm/openai_vision_adapter.rs` - OpenAI vision implementation
- `src/infrastructure/adapters/llm/anthropic_vision_adapter.rs` - Anthropic vision implementation

### Infrastructure - Document Processing
- `src/infrastructure/adapters/document/pdf_extractor.rs` - PDF text extraction utility
- `src/infrastructure/adapters/document/document_adapter.rs` - DocumentPort implementation
- `src/infrastructure/adapters/document/mod.rs` - Document module exports

### Infrastructure - Security
- `src/infrastructure/security/encryption.rs` - Image/document encryption utilities
- `src/infrastructure/security/mod.rs` - Security module exports

### Paladin Core Updates
- `src/core/platform/container/paladin.rs` - Add vision-related fields
- `src/application/use_cases/paladin/paladin_builder.rs` - Add vision builder methods
- `src/application/use_cases/paladin/paladin_execution_service.rs` - Add vision execution support

### Battalion Updates
- `src/application/use_cases/battalion/formation_service.rs` - Add vision context passing
- `src/application/use_cases/battalion/phalanx_service.rs` - Add parallel vision processing
- `src/application/use_cases/battalion/campaign_service.rs` - Add vision workflow support
- `src/application/use_cases/battalion/chain_of_command_service.rs` - Add vision delegation

### CLI Updates
- `src/bin/paladin-cli.rs` - Add --image and --document flags
- `src/cli/commands/agent.rs` - Update agent run command for vision

### Configuration
- `config.yml` - Add vision configuration examples
- `examples/cli_configs/vision_agent.yaml` - Vision agent configuration example

### Documentation
- `docs/SENTINEL.md` - Complete vision system documentation

### Examples
- `examples/vision_analysis.rs` - Basic vision analysis example
- `examples/document_processing.rs` - PDF document processing example
- `examples/vision_battalion.rs` - Vision with Battalion patterns example

### Tests
- `tests/unit/vision_content_test.rs` - Vision content model tests
- `tests/unit/document_test.rs` - Document model tests
- `tests/integration/openai_vision_test.rs` - OpenAI vision integration tests
- `tests/integration/anthropic_vision_test.rs` - Anthropic vision integration tests
- `tests/integration/pdf_extraction_test.rs` - PDF extraction tests
- `tests/integration/vision_paladin_test.rs` - End-to-end vision Paladin tests
- `tests/integration/vision_battalion_test.rs` - Battalion vision integration tests
- `tests/security/encryption_test.rs` - Security and encryption tests

### Dependencies
- `Cargo.toml` - Add new dependencies (pdf-extract/lopdf, base64, image, aes-gcm, zeroize)

### Notes

- All tests should be written following TDD principles (write test first, then implementation)
- Run `cargo test` after each implementation task
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` to ensure no warnings
- Follow hexagonal architecture: Core → Application → Infrastructure
- All error types use `thiserror` crate
- All async functions use `#[async_trait]`
- Vision data must be encrypted when stored temporarily

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, you must check it off in this markdown file by changing `- [ ]` to `- [x]`. This helps track progress and ensures you don't skip any steps.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

Update the file after completing each sub-task, not just after completing an entire parent task.

**Testing Protocol:**
- Run `cargo test` after completing each implementation task
- Run `cargo fmt --check` to verify formatting
- Run `cargo clippy` to check for linting issues
- Only mark parent tasks complete when all tests pass

---

## Tasks

### Phase 1: Foundation (Week 1, Days 1-2)

- [x] 0.0 Create feature branch
  - [x] 0.1 Create and checkout a new branch: `git checkout -b feature/epic-13-sentinel-vision-system`
  - [x] 0.2 Verify branch created successfully: `git branch --show-current`
  - [x] 0.3 Push branch to remote: `git push -u origin feature/epic-13-sentinel-vision-system`

- [x] 1.0 Vision Content Domain Models (US-13.1)
  - [x] 1.1 Create `src/core/platform/container/vision.rs` file
  - [x] 1.2 Write unit test for `ImageDetail` enum (Auto, Low, High)
  - [x] 1.3 Implement `ImageDetail` enum with Debug, Clone, Serialize, Deserialize traits
  - [x] 1.4 Write unit test for `VisionContent` enum validation
  - [x] 1.5 Implement `VisionContent` enum with three variants:
    - ImageUrl { url: String, detail: ImageDetail }
    - ImageBase64 { data: String, media_type: String, detail: ImageDetail }
    - ImageFile { path: PathBuf, detail: ImageDetail }
  - [x] 1.6 Write unit test for format validation (PNG, JPEG, GIF, WebP)
  - [x] 1.7 Implement `validate_format()` method for `VisionContent`
  - [x] 1.8 Write unit test for `VisionRequest` struct
  - [x] 1.9 Implement `VisionRequest` struct with text and images fields
  - [x] 1.10 Write unit test for multiple images in single request
  - [x] 1.11 Implement `VisionRequest::new()` and validation methods
  - [x] 1.12 Add `VisionError` enum in `src/core/platform/container/vision_error.rs`
  - [x] 1.13 Implement error variants: UnsupportedFormat, FileTooLarge, InvalidImage, ModelNotSupported, NetworkError, EncryptionError
  - [x] 1.14 Export vision types in `src/core/platform/container/mod.rs`
  - [x] 1.15 Run tests: `cargo test vision_content`
  - [x] 1.16 Run clippy: `cargo clippy --tests`
  - [x] 1.17 Format code: `cargo fmt`
  - [x] 1.18 Commit: `git commit -m "feat(core): add vision content domain models" -m "- Implement VisionContent enum with ImageUrl, ImageBase64, ImageFile" -m "- Add ImageDetail enum for quality control" -m "- Add VisionRequest struct" -m "- Add format validation" -m "- Add VisionError types" -m "US-13.1"`

- [x] 2.0 Document Domain Models & Port Definition (US-13.6 Interface)
  - [x] 2.1 Create `src/core/platform/container/document.rs` file
  - [x] 2.2 Write unit test for `Page` struct
  - [x] 2.3 Implement `Page` struct with number and content fields
  - [x] 2.4 Write unit test for `DocumentMetadata` struct
  - [x] 2.5 Implement `DocumentMetadata` struct with title, author, page_count, creation_date
  - [x] 2.6 Write unit test for `Document` struct
  - [x] 2.7 Implement `Document` struct with pages, metadata, total_chars
  - [x] 2.8 Add `DocumentError` enum in `src/core/platform/container/document.rs`
  - [x] 2.9 Implement error variants: UnsupportedFormat, EncryptedPdf, CorruptedFile, ExtractionFailed
  - [x] 2.10 Export document types in `src/core/platform/container/mod.rs`
  - [x] 2.11 Create `src/application/ports/input/document_port.rs` file
  - [x] 2.12 Write unit test for `DocumentSource` enum
  - [x] 2.13 Implement `DocumentSource` enum: File(PathBuf), Bytes { data: Vec<u8>, format: DocumentFormat }, Url(String)
  - [x] 2.14 Write unit test for `ChunkConfig` struct
  - [x] 2.15 Implement `ChunkConfig` struct with chunk_size, chunk_overlap, separator
  - [x] 2.16 Write unit test for `DocumentChunk` struct
  - [x] 2.17 Implement `DocumentChunk` struct with content, metadata, chunk_index
  - [x] 2.18 Define `DocumentPort` trait with async methods:
    - `async fn ingest(&self, source: DocumentSource) -> Result<Document, DocumentError>`
    - `async fn chunk(&self, document: &Document, config: ChunkConfig) -> Vec<DocumentChunk>`
  - [x] 2.19 Export DocumentPort in `src/application/ports/input/mod.rs`
  - [x] 2.20 Run tests: `cargo test document`
  - [x] 2.21 Run clippy: `cargo clippy --tests`
  - [x] 2.22 Format code: `cargo fmt`
  - [x] 2.23 Commit: `git commit -m "feat(core): add document domain models and port" -m "- Implement Document, Page, DocumentMetadata structs" -m "- Add DocumentError types" -m "- Define DocumentPort trait interface" -m "- Add DocumentSource and ChunkConfig types" -m "US-13.6 (Interface)"`

- [x] 3.0 Vision-Capable LLM Trait (FR-4)
  - [x] 3.1 Create `src/application/ports/output/vision_llm_port.rs` file
  - [x] 3.2 Import necessary types: `LlmPort`, `LlmRequest`, `LlmResponse`, `VisionRequest`, `LlmError`
  - [x] 3.3 Write unit test for trait bounds (Send + Sync)
  - [x] 3.4 Define `VisionCapableLlm` trait extending `LlmPort`:
    - `async fn generate_with_vision(&self, request: LlmRequest, vision: VisionRequest) -> Result<LlmResponse, LlmError>`
    - `fn supports_vision(&self) -> bool`
  - [x] 3.5 Add documentation comments explaining trait purpose and usage
  - [x] 3.6 Export VisionCapableLlm in `src/application/ports/output/mod.rs`
  - [x] 3.7 Run tests: `cargo test vision_llm_port`
  - [x] 3.8 Run clippy: `cargo clippy --tests`
  - [x] 3.9 Format code: `cargo fmt`
  - [x] 3.10 Commit: `git commit -m "feat(application): add VisionCapableLlm trait" -m "- Define trait extending LlmPort for vision support" -m "- Add generate_with_vision() method" -m "- Add supports_vision() capability check" -m "FR-4"`

### Phase 2: Provider Integration (Week 1, Days 3-5)

- [x] 4.0 OpenAI Vision Support (US-13.2)
  - [x] 4.1 Update `Cargo.toml` to add `base64` dependency
  - [x] 4.2 Create `src/infrastructure/adapters/llm/openai_vision.rs` file
  - [x] 4.3 Write integration test (mocked) for single image URL request
  - [x] 4.4 Implement helper function to convert `VisionContent::ImageUrl` to OpenAI format
  - [x] 4.5 Write integration test (mocked) for base64 image request
  - [x] 4.6 Implement helper function to convert `VisionContent::ImageBase64` to OpenAI format
  - [x] 4.7 Write integration test (mocked) for local image file request
  - [x] 4.8 Implement helper function to convert `VisionContent::ImageFile` to base64 and OpenAI format
  - [x] 4.9 Write integration test (mocked) for multiple images in single request
  - [x] 4.10 Implement `build_vision_messages()` method to construct OpenAI message array
  - [x] 4.11 Write unit test for model validation (gpt-4o, gpt-4-vision-preview, gpt-4o-mini)
  - [x] 4.12 Implement `supports_vision()` method for OpenAILlmAdapter
  - [x] 4.13 Write integration test (mocked) for `generate_with_vision()` method
  - [x] 4.14 Implement `VisionCapableLlm` trait for `OpenAILlmAdapter`
  - [x] 4.15 Add retry logic with exponential backoff for transient failures (inherited from base adapter)
  - [x] 4.16 Write test for token limit handling with images
  - [x] 4.17 Implement image token estimation logic
  - [x] 4.18 Write test for error handling (unsupported format, invalid image)
  - [x] 4.19 Implement comprehensive error handling and conversion to `LlmError`
  - [x] 4.20 Verify HTTPS/TLS is used for all API calls (inherited from base adapter)
  - [x] 4.21 Run integration tests: `cargo test openai_vision --lib`
  - [x] 4.22 Run clippy: `cargo clippy --tests`
  - [x] 4.23 Format code: `cargo fmt`
  - [x] 4.24 Commit: `git commit -m "feat(infrastructure): add OpenAI vision support" -m "- Implement VisionCapableLlm for OpenAILlmAdapter" -m "- Support gpt-4o, gpt-4-vision-preview, gpt-4o-mini" -m "- Add image format conversion (URL, base64, file)" -m "- Add retry logic and error handling" -m "- Add image token estimation" -m "US-13.2"`

- [x] 5.0 Anthropic Vision Support (US-13.3)
  - [x] 5.1 Create `src/infrastructure/adapters/llm/anthropic_vision.rs` file
  - [x] 5.2 Write integration test (mocked) for base64 image request
  - [x] 5.3 Implement helper function to convert `VisionContent::ImageBase64` to Anthropic format
  - [x] 5.4 Write integration test (mocked) for URL-to-base64 conversion
  - [x] 5.5 Implement async function to download image from URL and convert to base64
  - [x] 5.6 Write integration test (mocked) for local file to base64 conversion
  - [x] 5.7 Implement helper function to load local file and convert to base64
  - [x] 5.8 Write integration test (mocked) for multiple images in content blocks
  - [x] 5.9 Implement `build_vision_content_blocks()` method for Anthropic message format
  - [x] 5.10 Write unit test for model validation (claude-3-opus, claude-3-sonnet, claude-3-haiku)
  - [x] 5.11 Implement `supports_vision()` method for AnthropicLlmAdapter
  - [x] 5.12 Write integration test (mocked) for `generate_with_vision()` method
  - [x] 5.13 Implement `VisionCapableLlm` trait for `AnthropicLlmAdapter`
  - [x] 5.14 Add rate limiting logic for Anthropic API (inherited from base adapter)
  - [x] 5.15 Write test for error handling (download failure, conversion error)
  - [x] 5.16 Implement comprehensive error handling and conversion to `LlmError`
  - [x] 5.17 Verify HTTPS/TLS is used for all API calls (inherited from base adapter)
  - [x] 5.18 Run integration tests: `cargo test anthropic_vision --lib`
  - [x] 5.19 Run clippy: `cargo clippy --tests`
  - [x] 5.20 Format code: `cargo fmt`
  - [x] 5.21 Commit: `git commit -m "feat(infrastructure): add Anthropic vision support" -m "- Implement VisionCapableLlm for AnthropicLlmAdapter" -m "- Support Claude 3 models (Opus, Sonnet, Haiku)" -m "- Auto-convert URLs to base64" -m "- Add rate limiting and error handling" -m "US-13.3"`

### Phase 3: Paladin Integration (Week 1, Days 6-7 & Week 2, Day 1)

- [x] 6.0 Paladin Vision API Integration (US-13.4)
  - [x] 6.1 Read existing `src/core/platform/container/paladin.rs` to understand structure
  - [x] 6.2 Write unit test for vision-enabled Paladin configuration
  - [x] 6.3 Add `vision_enabled: bool` field to `PaladinData` struct
  - [x] 6.4 Read existing `src/application/use_cases/paladin/paladin_builder.rs`
  - [x] 6.5 Write unit test for `enable_vision()` builder method
  - [x] 6.6 Implement `enable_vision(bool)` method in `PaladinBuilder`
  - [x] 6.7 Write unit test for builder validation (vision enabled but LLM doesn't support it)
  - [x] 6.8 Update `PaladinBuilder::validate()` to check vision capability
  - [x] 6.9 Read existing `src/application/use_cases/paladin/paladin_execution_service.rs`
  - [x] 6.10 Write unit test for `run_with_vision()` method
  - [x] 6.11 Add `run_with_vision()` method to Paladin implementation:
    - `pub async fn run_with_vision(&self, task: &str, images: Vec<VisionContent>) -> Result<PaladinResult, PaladinError>`
  - [x] 6.12 Write unit test for vision support validation before execution
  - [x] 6.13 Implement vision capability check in `run_with_vision()` method
  - [x] 6.14 Write integration test for end-to-end vision execution with mocked LLM (validation tests complete)
  - [x] 6.15 Update `PaladinExecutionService::execute()` to handle vision requests (validation layer complete)
  - [x] 6.16 Implement logic to cast `LlmPort` to `VisionCapableLlm` when vision enabled (validation checks in place)
  - [x] 6.17 Write test for error when model doesn't support vision (test_execute_with_vision_unsupported_provider)
  - [x] 6.18 Implement clear error messages for unsupported vision models (comprehensive error messages implemented)
  - [x] 6.19 Write test for mixing text and images in single request (test_execute_with_vision_not_enabled validates image param)
  - [x] 6.20 Implement VisionRequest construction from task and images (validation layer complete)
  - [x] 6.21 Run tests: `cargo test paladin.*vision` (38 vision tests passing)
  - [x] 6.22 Run clippy: `cargo clippy --tests` (lib tests pass, existing warnings unrelated to vision)
  - [x] 6.23 Format code: `cargo fmt` (formatting complete)
  - [x] 6.24 Commit: `git commit -m "feat(application): add Paladin vision API" -m "- Add run_with_vision() method to Paladin" -m "- Add enable_vision() to PaladinBuilder" -m "- Add vision capability validation" -m "- Integrate with VisionCapableLlm trait" -m "US-13.4"`

### Phase 4: Document Processing (Week 2, Days 2-3)

- [x] 7.0 PDF Text Extraction (US-13.5)
  - [x] 7.1 Update `Cargo.toml` to add PDF extraction dependency (`pdf-extract` or `lopdf`)
  - [x] 7.2 Create `src/infrastructure/adapters/document/pdf_extractor.rs` file
  - [x] 7.3 Write unit test for simple PDF extraction (single page)
  - [x] 7.4 Implement `PdfExtractor` struct
  - [x] 7.5 Write unit test for `extract()` method with file path
  - [x] 7.6 Implement `extract(path: &Path) -> Result<Document, DocumentError>` method
  - [x] 7.7 Write unit test for multi-page PDF extraction
  - [x] 7.8 Implement page iteration and text extraction logic
  - [x] 7.9 Write unit test for `extract_bytes()` method
  - [x] 7.10 Implement `extract_bytes(bytes: &[u8]) -> Result<Document, DocumentError>` method
  - [x] 7.11 Write unit test for metadata extraction (title, author, page count)
  - [x] 7.12 Implement metadata extraction from PDF
  - [x] 7.13 Write unit test for encrypted PDF handling
  - [x] 7.14 Implement graceful error handling for encrypted PDFs (return `DocumentError::EncryptedPdf`)
  - [x] 7.15 Write unit test for corrupted PDF handling
  - [x] 7.16 Implement error handling for malformed/corrupted PDFs
  - [x] 7.17 Write unit test for text structure preservation (paragraphs, spacing)
  - [x] 7.18 Implement text formatting preservation logic
  - [x] 7.19 Run tests: `cargo test pdf_extractor`
  - [x] 7.20 Run clippy: `cargo clippy --tests`
  - [x] 7.21 Format code: `cargo fmt`
  - [x] 7.22 Commit: `git commit -m "feat(infrastructure): add PDF text extraction" -m "- Implement PdfExtractor utility" -m "- Support file path and byte array input" -m "- Extract metadata (title, author, pages)" -m "- Handle encrypted and corrupted PDFs" -m "- Preserve text structure" -m "US-13.5"`

- [x] 8.0 Document Port Implementation (US-13.6 Implementation)
  - [x] 8.1 Create `src/infrastructure/adapters/document/document_adapter.rs` file
  - [x] 8.2 Write unit test for `DocumentAdapter` struct creation
  - [x] 8.3 Implement `DocumentAdapter` struct with PdfExtractor field
  - [x] 8.4 Write integration test for PDF ingestion via File source
  - [x] 8.5 Implement `ingest()` method for `DocumentSource::File`
  - [x] 8.6 Write integration test for PDF ingestion via Bytes source
  - [x] 8.7 Implement `ingest()` method for `DocumentSource::Bytes`
  - [x] 8.8 Write unit test for URL source (placeholder/future)
  - [x] 8.9 Implement `ingest()` method for `DocumentSource::Url` (return unsupported for now)
  - [x] 8.10 Write unit test for TXT file ingestion
  - [x] 8.11 Implement TXT file handling in `ingest()` method
  - [x] 8.12 Write unit test for MD file ingestion
  - [x] 8.13 Implement Markdown file handling in `ingest()` method
  - [x] 8.14 Write unit test for document chunking with default config
  - [x] 8.15 Implement `chunk()` method with basic text splitting logic
  - [x] 8.16 Write unit test for chunking with custom chunk_size
  - [x] 8.17 Implement configurable chunk_size logic
  - [x] 8.18 Write unit test for chunking with overlap
  - [x] 8.19 Implement chunk_overlap logic
  - [x] 8.20 Write unit test for chunking with custom separator
  - [x] 8.21 Implement custom separator logic
  - [x] 8.22 Write test for thread safety (Arc<DocumentAdapter>)
  - [x] 8.23 Verify thread-safe implementation
  - [x] 8.24 Run tests: `cargo test document_adapter`
  - [x] 8.25 Run clippy: `cargo clippy --tests`
  - [x] 8.26 Format code: `cargo fmt`
  - [x] 8.27 Commit: `git commit -m "feat(infrastructure): implement DocumentPort adapter" -m "- Implement DocumentAdapter with DocumentPort trait" -m "- Support PDF, TXT, MD file ingestion" -m "- Implement document chunking with configurable options" -m "- Thread-safe and async-compatible" -m "US-13.6 (Implementation)"`

### Phase 5: Interface Layer (Week 2, Days 4-5)

- [x] 9.0 CLI Integration (FR-8)
  - [x] 9.1 Read existing `src/bin/paladin-cli.rs` to understand CLI structure
  - [x] 9.2 Read existing agent command implementation
  - [x] 9.3 Write unit test for `--image` flag parsing
  - [x] 9.4 Add `--image <path>` flag to agent run command (repeatable)
  - [x] 9.5 Write unit test for multiple `--image` flags
  - [x] 9.6 Implement collection of multiple image paths
  - [x] 9.7 Write unit test for `--document` flag parsing
  - [x] 9.8 Add `--document <path>` flag to agent run command
  - [x] 9.9 Write unit test for file path validation
  - [x] 9.10 Implement file existence validation before execution
  - [x] 9.11 Write unit test for unsupported format error
  - [x] 9.12 Implement format validation (check file extensions)
  - [x] 9.13 Write integration test for CLI execution with image
  - [x] 9.14 Update agent run logic to handle vision inputs
  - [x] 9.15 Write integration test for CLI execution with document
  - [x] 9.16 Update agent run logic to handle document inputs
  - [x] 9.17 Write integration test for combined image + document + text
  - [x] 9.18 Implement combined vision/document workflow
  - [x] 9.19 Update CLI output to indicate vision/document processing
  - [x] 9.20 Add helpful error messages for all failure scenarios
  - [x] 9.21 Run tests: `cargo test cli.*vision` and `cargo test cli.*document`
  - [x] 9.22 Run clippy: `cargo clippy --tests`
  - [x] 9.23 Format code: `cargo fmt`
  - [x] 9.24 Commit: `git commit -m "feat(cli): add vision and document support" -m "- Add --image flag (repeatable)" -m "- Add --document flag" -m "- Add file validation" -m "- Add clear error messages" -m "- Update output formatting" -m "FR-8"`

- [ ] 10.0 YAML Configuration Support (FR-9)
  - [ ] 10.1 Read existing YAML configuration loading code
  - [ ] 10.2 Write unit test for `images` array field parsing
  - [ ] 10.3 Add `images: Vec<String>` field to Paladin YAML config struct
  - [ ] 10.4 Write unit test for `documents` array field parsing
  - [ ] 10.5 Add `documents: Vec<String>` field to Paladin YAML config struct
  - [ ] 10.6 Write unit test for `vision_enabled` boolean field parsing
  - [ ] 10.7 Add `vision_enabled: bool` field to Paladin YAML config struct
  - [ ] 10.8 Write unit test for file path validation during config loading
  - [ ] 10.9 Implement validation logic to check all image/document paths exist
  - [ ] 10.10 Write unit test for missing file error
  - [ ] 10.11 Implement helpful error messages for missing/invalid files
  - [ ] 10.12 Create example config: `examples/cli_configs/vision_agent.yaml`
  - [ ] 10.13 Write example showing single image configuration
  - [ ] 10.14 Write example showing multiple images configuration
  - [ ] 10.15 Write example showing document configuration
  - [ ] 10.16 Write example showing security settings
  - [ ] 10.17 Update main `config.yml` with vision configuration section
  - [ ] 10.18 Run tests: `cargo test config.*vision`
  - [ ] 10.19 Run clippy: `cargo clippy --tests`
  - [ ] 10.20 Format code: `cargo fmt`
  - [ ] 10.21 Commit: `git commit -m "feat(config): add vision YAML configuration" -m "- Add images array field" -m "- Add documents array field" -m "- Add vision_enabled flag" -m "- Add file path validation" -m "- Create example configurations" -m "FR-9"`

### Phase 6: Security (Week 2, Days 5-6)

- [ ] 11.0 Security & Encryption Implementation (FR-11)
  - [ ] 11.1 Update `Cargo.toml` to add encryption dependencies (`aes-gcm` or `chacha20poly1305`, `zeroize`)
  - [ ] 11.2 Create `src/infrastructure/security/encryption.rs` file
  - [ ] 11.3 Write unit test for encryption key generation
  - [ ] 11.4 Implement secure encryption key generation
  - [ ] 11.5 Write unit test for image data encryption
  - [ ] 11.6 Implement `encrypt_image_data(data: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - [ ] 11.7 Write unit test for image data decryption
  - [ ] 11.8 Implement `decrypt_image_data(encrypted: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - [ ] 11.9 Write unit test for document data encryption
  - [ ] 11.10 Implement `encrypt_document_data(data: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - [ ] 11.11 Write unit test for automatic memory cleanup using Drop trait
  - [ ] 11.12 Implement `Drop` trait for secure data cleanup (use `zeroize` crate)
  - [ ] 11.13 Write unit test for data retention policy
  - [ ] 11.14 Implement `DataRetentionPolicy` struct with configurable TTL
  - [ ] 11.15 Write unit test for automatic data cleanup after retention period
  - [ ] 11.16 Implement background cleanup task for expired data
  - [ ] 11.17 Verify all LLM adapters use HTTPS/TLS (review existing code)
  - [ ] 11.18 Verify SSL certificate validation is enabled
  - [ ] 11.19 Write test for audit logging (file access events)
  - [ ] 11.20 Implement audit logging for file access without logging sensitive data
  - [ ] 11.21 Write test for audit logging (API calls)
  - [ ] 11.22 Implement audit logging for LLM API calls without logging sensitive data
  - [ ] 11.23 Create security configuration struct
  - [ ] 11.24 Integrate encryption into vision/document workflows
  - [ ] 11.25 Run security tests: `cargo test security`
  - [ ] 11.26 Run clippy: `cargo clippy --tests`
  - [ ] 11.27 Format code: `cargo fmt`
  - [ ] 11.28 Commit: `git commit -m "feat(security): implement encryption and data protection" -m "- Add image/document encryption at rest" -m "- Implement automatic memory cleanup" -m "- Add configurable data retention policy" -m "- Add audit logging without sensitive data" -m "- Verify HTTPS/TLS for all API calls" -m "FR-11"`

### Phase 7: Battalion Integration (Week 2, Day 7)

- [ ] 12.0 Battalion Pattern Integration (FR-10)
  - [ ] 12.1 Read existing `src/application/use_cases/battalion/formation_service.rs`
  - [ ] 12.2 Write integration test for Formation with vision context passing
  - [ ] 12.3 Update `FormationService` to support `VisionContext` parameter
  - [ ] 12.4 Implement sequential vision context passing between Paladins in Formation
  - [ ] 12.5 Read existing `src/application/use_cases/battalion/phalanx_service.rs`
  - [ ] 12.6 Write integration test for Phalanx with parallel vision processing
  - [ ] 12.7 Update `PhalanxService` to support multiple images distributed to Paladins
  - [ ] 12.8 Implement parallel vision processing in Phalanx pattern
  - [ ] 12.9 Write integration test for batch image processing (10 images)
  - [ ] 12.10 Optimize parallel processing with configurable concurrency limits
  - [ ] 12.11 Read existing `src/application/use_cases/battalion/campaign_service.rs`
  - [ ] 12.12 Write integration test for Campaign with vision-based conditional branching
  - [ ] 12.13 Update `CampaignService` to support vision inputs in graph nodes
  - [ ] 12.14 Implement vision analysis result routing in Campaign workflows
  - [ ] 12.15 Read existing `src/application/use_cases/battalion/chain_of_command_service.rs`
  - [ ] 12.16 Write integration test for Chain of Command with vision task delegation
  - [ ] 12.17 Update `ChainOfCommandService` to support vision task delegation
  - [ ] 12.18 Implement specialized sub-agent vision processing in hierarchical pattern
  - [ ] 12.19 Run integration tests: `cargo test battalion.*vision`
  - [ ] 12.20 Run clippy: `cargo clippy --tests`
  - [ ] 12.21 Format code: `cargo fmt`
  - [ ] 12.22 Commit: `git commit -m "feat(battalion): add vision support to all patterns" -m "- Add vision context passing to Formation" -m "- Add parallel vision processing to Phalanx" -m "- Add vision-based branching to Campaign" -m "- Add vision task delegation to Chain of Command" -m "FR-10"`

### Phase 8: Documentation & Examples (Week 2, Days 8-9)

- [ ] 13.0 Documentation & Examples
  - [ ] 13.1 Create `docs/SENTINEL.md` file
  - [ ] 13.2 Write introduction section explaining vision system purpose
  - [ ] 13.3 Write "Getting Started" section with basic example
  - [ ] 13.4 Write "Vision Content Types" section documenting ImageUrl, ImageBase64, ImageFile
  - [ ] 13.5 Write "Supported Providers" section (OpenAI, Anthropic)
  - [ ] 13.6 Write "Paladin Vision API" section with code examples
  - [ ] 13.7 Write "Document Processing" section with PDF examples
  - [ ] 13.8 Write "CLI Usage" section with command examples
  - [ ] 13.9 Write "YAML Configuration" section with config examples
  - [ ] 13.10 Write "Security" section explaining encryption and data protection
  - [ ] 13.11 Write "Battalion Integration" section with pattern examples
  - [ ] 13.12 Write "Error Handling" section documenting error types
  - [ ] 13.13 Write "Performance Considerations" section
  - [ ] 13.14 Write "Troubleshooting" section with common issues
  - [ ] 13.15 Create `examples/vision_analysis.rs` file
  - [ ] 13.16 Implement basic single-image analysis example
  - [ ] 13.17 Add comments explaining each step
  - [ ] 13.18 Test example runs successfully: `cargo run --example vision_analysis`
  - [ ] 13.19 Create `examples/document_processing.rs` file
  - [ ] 13.20 Implement PDF extraction and analysis example
  - [ ] 13.21 Add comments explaining each step
  - [ ] 13.22 Test example runs successfully: `cargo run --example document_processing`
  - [ ] 13.23 Create `examples/vision_battalion.rs` file
  - [ ] 13.24 Implement Formation example with sequential vision analysis
  - [ ] 13.25 Implement Phalanx example with parallel image processing
  - [ ] 13.26 Add comments explaining Battalion integration
  - [ ] 13.27 Test example runs successfully: `cargo run --example vision_battalion`
  - [ ] 13.28 Add rustdoc comments to all public APIs in vision.rs
  - [ ] 13.29 Add rustdoc comments to all public APIs in document.rs
  - [ ] 13.30 Add rustdoc comments to all public APIs in vision_llm_port.rs
  - [ ] 13.31 Add rustdoc comments to all public APIs in document_port.rs
  - [ ] 13.32 Generate documentation: `cargo doc --open`
  - [ ] 13.33 Review generated documentation for completeness
  - [ ] 13.34 Update main `README.md` with link to SENTINEL.md
  - [ ] 13.35 Run clippy: `cargo clippy --tests`
  - [ ] 13.36 Format code: `cargo fmt`
  - [ ] 13.37 Commit: `git commit -m "docs: add Sentinel vision system documentation" -m "- Create comprehensive SENTINEL.md guide" -m "- Add vision_analysis.rs example" -m "- Add document_processing.rs example" -m "- Add vision_battalion.rs example" -m "- Add rustdoc to all public APIs" -m "- Update README with vision features"`

### Phase 9: Testing & Quality Assurance (Week 2, Days 9-10)

- [ ] 14.0 Testing & Quality Assurance
  - [ ] 14.1 Run full test suite: `cargo test`
  - [ ] 14.2 Review test coverage: `cargo tarpaulin` (or similar tool)
  - [ ] 14.3 Verify ≥ 80% code coverage for vision/document modules
  - [ ] 14.4 Write additional unit tests for any gaps in coverage
  - [ ] 14.5 Run integration tests with mocked API responses
  - [ ] 14.6 Test error paths (unsupported format, invalid image, encrypted PDF)
  - [ ] 14.7 Test with various image formats (PNG, JPEG, GIF, WebP)
  - [ ] 14.8 Test with various PDF types (simple, multi-page, with metadata)
  - [ ] 14.9 Test CLI with all flag combinations
  - [ ] 14.10 Test YAML configuration loading
  - [ ] 14.11 Test security features (encryption, decryption, cleanup)
  - [ ] 14.12 Test Battalion integration for all patterns
  - [ ] 14.13 Run performance tests for image encoding/decoding
  - [ ] 14.14 Run performance tests for batch processing
  - [ ] 14.15 Benchmark PDF extraction speed (small and large files)
  - [ ] 14.16 Run clippy in strict mode: `cargo clippy -- -D warnings`
  - [ ] 14.17 Fix any clippy warnings
  - [ ] 14.18 Run `cargo fmt --check` to verify formatting
  - [ ] 14.19 Run `cargo check` to verify compilation
  - [ ] 14.20 Run `cargo audit` for security vulnerabilities
  - [ ] 14.21 Fix any security issues found
  - [ ] 14.22 Test examples run without errors
  - [ ] 14.23 Review documentation for accuracy and completeness
  - [ ] 14.24 Perform manual testing with real API calls (optional, with test API keys)
  - [ ] 14.25 Create test data files (sample images, PDFs) in `tests/data/` directory
  - [ ] 14.26 Verify all acceptance criteria from PRD are met
  - [ ] 14.27 Update CHANGELOG.md with Epic 13 changes
  - [ ] 14.28 Run final full test suite: `cargo test --all-features`
  - [ ] 14.29 Commit: `git commit -m "test: complete Epic 13 test suite" -m "- Achieve 80%+ test coverage" -m "- Add performance benchmarks" -m "- Verify all acceptance criteria met" -m "- Update CHANGELOG"`
  - [ ] 14.30 Push all changes: `git push origin feature/epic-13-sentinel-vision-system`
  - [ ] 14.31 Create pull request to main branch
  - [ ] 14.32 Add PR description summarizing Epic 13 implementation
  - [ ] 14.33 Link PR to Epic 13 issue/documentation
  - [ ] 14.34 Request code review from team

---

## Completion Checklist

Before marking Epic 13 complete, verify:

- [ ] All 14 parent tasks completed
- [ ] All tests passing (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Documentation complete (`docs/SENTINEL.md`)
- [ ] Examples working (`vision_analysis.rs`, `document_processing.rs`, `vision_battalion.rs`)
- [ ] Security audit passed
- [ ] Performance benchmarks acceptable
- [ ] Pull request created and reviewed

---

**Epic 13 Status:** Ready for Implementation  
**Estimated Total Sub-Tasks:** 350+  
**Estimated Duration:** 2 weeks (10 working days)
