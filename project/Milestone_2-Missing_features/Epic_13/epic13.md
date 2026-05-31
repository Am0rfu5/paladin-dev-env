## Epic 13: Sentinel Vision System

**Theme:** Multi-Modal Input Processing  
**Duration:** 2 weeks  
**Priority:** Critical  
**Dependencies:** Epic 6 (Provider Expansion)  

### Description
Add vision and document processing capabilities to Paladins, enabling image analysis, PDF processing, and multi-modal inputs through supported LLM providers.

### User Stories

#### US-13.1: Vision Request Model
**As a** framework developer  
**I want** data structures for multi-modal requests  
**So that** the system can handle images and documents

**Acceptance Criteria:**
- [ ] `VisionContent` enum in `src/core/platform/container/vision.rs`
- [ ] Supports: ImageUrl, ImageBase64, ImageFile
- [ ] Supports multiple images in single request
- [ ] Image metadata: format, size, dimensions (if known)
- [ ] Validation for supported formats (PNG, JPEG, GIF, WebP)

**Definition of Done:**
```rust
pub enum VisionContent {
    ImageUrl {
        url: String,
        detail: ImageDetail,
    },
    ImageBase64 {
        data: String,
        media_type: String,
        detail: ImageDetail,
    },
    ImageFile {
        path: PathBuf,
        detail: ImageDetail,
    },
}

pub enum ImageDetail {
    Auto,
    Low,
    High,
}

pub struct VisionRequest {
    pub text: String,
    pub images: Vec<VisionContent>,
}
```

---

#### US-13.2: OpenAI Vision Support
**As a** developer  
**I want** to send images to GPT-4 Vision  
**So that** Paladins can analyze visual content

**Acceptance Criteria:**
- [ ] `OpenAILlmAdapter` supports vision requests
- [ ] Handles `gpt-4-vision-preview` and `gpt-4o` models
- [ ] Converts `VisionContent` to OpenAI message format
- [ ] Supports image URLs and base64 encoding
- [ ] Handles multiple images in single request
- [ ] Respects token limits for image tokens
- [ ] Integration test with mocked vision response

**Definition of Done:**
```rust
impl OpenAILlmAdapter {
    async fn generate_with_vision(
        &self,
        request: LlmRequest,
        vision: VisionRequest,
    ) -> Result<LlmResponse, LlmError>;
}

// LlmPort trait extension
#[async_trait]
pub trait VisionCapableLlm: LlmPort {
    async fn generate_with_vision(
        &self,
        request: LlmRequest,
        vision: VisionRequest,
    ) -> Result<LlmResponse, LlmError>;

    fn supports_vision(&self) -> bool;
}
```

---

#### US-13.3: Anthropic Vision Support
**As a** developer  
**I want** to send images to Claude  
**So that** I can use Anthropic's vision capabilities

**Acceptance Criteria:**
- [ ] `AnthropicLlmAdapter` supports vision requests
- [ ] Handles Claude 3 models (Opus, Sonnet, Haiku)
- [ ] Converts `VisionContent` to Anthropic message format
- [ ] Supports base64 images (required by Anthropic)
- [ ] Automatic URL-to-base64 conversion
- [ ] Integration test with mocked vision response

**Definition of Done:**
```rust
impl VisionCapableLlm for AnthropicLlmAdapter {
    async fn generate_with_vision(
        &self,
        request: LlmRequest,
        vision: VisionRequest,
    ) -> Result<LlmResponse, LlmError> {
        // Convert to Anthropic format with image content blocks
    }

    fn supports_vision(&self) -> bool {
        self.model.starts_with("claude-3")
    }
}
```

---

#### US-13.4: Paladin Vision API
**As a** developer  
**I want** to run Paladins with image inputs  
**So that** I can build vision-based agents

**Acceptance Criteria:**
- [ ] `Paladin::run_with_vision(task, images)` method
- [ ] `PaladinBuilder::enable_vision(true)` configuration
- [ ] Validation that LLM adapter supports vision
- [ ] CLI support: `--image <path>` flag
- [ ] YAML config: `images: [path1, path2]`

**Definition of Done:**
```rust
impl Paladin {
    pub async fn run_with_vision(
        &self,
        task: &str,
        images: Vec<VisionContent>,
    ) -> Result<PaladinResult, PaladinError>;
}

// CLI usage
// paladin agent run --config agent.yaml --input "Describe this" --image photo.jpg

// YAML config
// task: "Analyze these charts"
// images:
//   - "./chart1.png"
//   - "./chart2.png"
```

---

#### US-13.5: PDF Text Extraction
**As a** developer  
**I want** to extract text from PDF documents  
**So that** Paladins can process documents

**Acceptance Criteria:**
- [ ] `PdfExtractor` utility in `src/infrastructure/adapters/document/`
- [ ] Uses `pdf-extract` or `lopdf` crate
- [ ] Extracts text content preserving structure
- [ ] Handles multi-page documents
- [ ] Returns `Document` struct with pages and metadata
- [ ] Error handling for encrypted/malformed PDFs

**Definition of Done:**
```rust
pub struct PdfExtractor;

impl PdfExtractor {
    pub fn extract(path: &Path) -> Result<Document, DocumentError>;
    pub fn extract_bytes(bytes: &[u8]) -> Result<Document, DocumentError>;
}

pub struct Document {
    pub pages: Vec<Page>,
    pub metadata: DocumentMetadata,
    pub total_chars: usize,
}

pub struct Page {
    pub number: usize,
    pub content: String,
}
```

---

#### US-13.6: Document Ingestion Port
**As a** developer  
**I want** a standardized port for document processing  
**So that** I can support multiple document types

**Acceptance Criteria:**
- [ ] `DocumentPort` trait in `src/application/ports/input/document_port.rs`
- [ ] Supports: PDF, TXT, MD, DOCX (future)
- [ ] Returns chunked content for large documents
- [ ] Configurable chunk size and overlap
- [ ] Metadata extraction (title, author, date)

**Definition of Done:**
```rust
#[async_trait]
pub trait DocumentPort: Send + Sync {
    async fn ingest(&self, source: DocumentSource) -> Result<Document, DocumentError>;
    async fn chunk(&self, document: &Document, config: ChunkConfig) -> Vec<DocumentChunk>;
}

pub enum DocumentSource {
    File(PathBuf),
    Bytes { data: Vec<u8>, format: DocumentFormat },
    Url(String),
}

pub struct ChunkConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub separator: String,
}
```

---

### Epic 13 Completion Criteria
- [ ] All 6 user stories completed and tested
- [ ] OpenAI and Anthropic vision support
- [ ] PDF extraction functional
- [ ] Document ingestion port defined
- [ ] CLI supports `--image` and `--document` flags
- [ ] Documentation in `docs/SENTINEL.md`
- [ ] Example: `examples/vision_analysis.rs`
- [ ] Example: `examples/document_processing.rs`

---
