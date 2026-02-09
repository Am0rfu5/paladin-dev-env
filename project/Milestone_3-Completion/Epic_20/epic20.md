## Epic 20: Vision Pipeline Completion

**Theme:** Complete multi-modal vision API integration  
**Duration:** 1–2 weeks  
**Priority:** High  
**Dependencies:** Epic 19  
**Origin:** Epic 13 (Sentinel Vision System) inline TODOs

### Description

Epic 13 established the vision type system and adapter scaffolding, but left the actual API calls and execution service integration as `TODO` stubs. This epic completes the vision pipeline end-to-end: making real API calls to OpenAI and Anthropic vision endpoints and wiring the execution service.

### User Stories

#### US-20.1: Implement OpenAI Vision API Call

**As a** developer  
**I want** the OpenAI vision adapter to make actual API calls  
**So that** image analysis works with GPT-4 Vision

**Acceptance Criteria:**
- [ ] `OpenAIVisionAdapter` sends multimodal content (image_url parts) to OpenAI Chat Completions API
- [ ] Supports both URL-based and base64-encoded images
- [ ] Parses and returns structured vision response
- [ ] Handles API errors (rate limits, invalid images, unsupported formats)
- [ ] Implements retry with exponential backoff
- [ ] Unit tests with mocked HTTP responses
- [ ] Integration test structure for live API validation (gated behind feature flag or env var)

**Source Files:**
- `src/infrastructure/adapters/llm/openai_vision.rs` — line 212

---

#### US-20.2: Implement Anthropic Vision API Call

**As a** developer  
**I want** the Anthropic vision adapter to make actual API calls  
**So that** image analysis works with Claude Vision

**Acceptance Criteria:**
- [ ] `AnthropicVisionAdapter` sends multimodal content blocks to Anthropic Messages API
- [ ] Supports both URL-based and base64-encoded images
- [ ] Handles Anthropic-specific content block format (type: "image", source: {type, media_type, data})
- [ ] Parses and returns structured vision response
- [ ] Handles API errors appropriately
- [ ] Unit tests with mocked HTTP responses

**Source Files:**
- `src/infrastructure/adapters/llm/anthropic_vision.rs` — line 220

---

#### US-20.3: Complete Vision Execution in PaladinExecutionService

**As a** developer  
**I want** the execution service to perform full vision processing  
**So that** Paladins can analyze images end-to-end

**Acceptance Criteria:**
- [ ] `PaladinExecutionService::execute_with_vision()` builds multimodal prompts
- [ ] Calls the appropriate vision adapter based on provider
- [ ] Parses vision results and integrates into the reasoning loop
- [ ] Respects existing execution configuration (max_loops, stop_words, timeout)
- [ ] Unit tests with mock vision adapter
- [ ] Example updated: `examples/sentinel_vision.rs` or equivalent

**Source Files:**
- `src/application/use_cases/paladin/paladin_execution_service.rs` — line 371

---

### Epic 20 Completion Criteria

- [ ] OpenAI and Anthropic vision adapters make real HTTP API calls
- [ ] `PaladinExecutionService` vision execution fully implemented
- [ ] All tests pass with mocked responses
- [ ] `cargo clippy` clean, `cargo fmt` clean

---

