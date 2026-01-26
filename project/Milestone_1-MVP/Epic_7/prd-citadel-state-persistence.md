# Product Requirements Document: Citadel State Persistence

## Introduction/Overview

The Citadel State Persistence system is a critical component of the Paladin multi-agent orchestration framework that enables automatic saving and restoration of Paladin agents and Battalion orchestrations. This feature solves the problem of lost work and interrupted workflows by persisting the complete state of AI agents, including their conversation history (Garrison), execution records, and configuration, to the file system as human-readable JSON files.

The Citadel acts as a safety mechanism ensuring that long-running agent processes, complex multi-agent workflows, and valuable conversation contexts are never lost due to system failures, restarts, or intentional shutdown. By implementing state persistence, developers can build resilient AI applications that gracefully handle interruptions and support debugging through inspectable state files.

**Problem Statement:** Currently, Paladin agents and Battalion orchestrations exist only in memory. When a process crashes, the system restarts, or a developer needs to stop execution, all agent state, conversation context, and progress is lost. This creates significant friction for production deployments and makes debugging difficult.

**Solution:** The Citadel provides automatic state persistence to the file system after every Paladin execution completion, enabling complete restoration of agent state and resumption of Battalion workflows from the last completed Paladin checkpoint.

## Goals

1. **Automatic State Persistence**: Implement automatic saving of Paladin and Battalion state to JSON files after every execution completion.

2. **Complete State Restoration**: Enable full restoration of Paladin agents including configuration, Garrison memory, and execution history from saved state files.

3. **Battalion Checkpoint Recovery**: Support resuming Battalion workflows from the last successfully completed Paladin, avoiding re-execution of completed work.

4. **Developer-Friendly Format**: Store state as human-readable, inspectable JSON files that developers can review for debugging.

5. **Hexagonal Architecture Compliance**: Design Citadel as a port/adapter pattern maintaining clean separation between domain logic and infrastructure.

6. **Production Reliability**: Ensure robust error handling with fail-fast behavior for corrupted or incompatible state files.

## User Stories

### User Story 1: Automatic Paladin State Saving
**As a** developer building a production AI application  
**I want** Paladin state to automatically save after each execution  
**So that** agent context is preserved across system restarts without manual intervention

**Acceptance Criteria:**
- Paladin state is automatically written to JSON file after successful execution
- State includes configuration, Garrison entries, and execution history
- Files are named with Paladin ID and timestamp for easy identification
- Save operation does not block Paladin execution

### User Story 2: Restoring Paladin from Saved State
**As a** developer debugging an AI agent issue  
**I want** to restore a Paladin from a previously saved state file  
**So that** I can reproduce issues and continue work from a known checkpoint

**Acceptance Criteria:**
- PaladinBuilder supports `.restore_from(state_id)` method
- Restored Paladin has identical configuration, memory, and execution history
- Garrison context is fully restored enabling continued conversation
- Invalid or corrupted state files produce clear error messages

### User Story 3: Battalion Workflow Resumption
**As a** developer running long-duration multi-agent workflows  
**I want** Battalion orchestrations to resume from checkpoints  
**So that** partial work is not lost when failures occur mid-execution

**Acceptance Criteria:**
- Battalion state saves after each Paladin completes successfully
- On restoration, already-completed Paladins are skipped
- Formation resumes from next Paladin in sequence
- Phalanx re-executes only failed or incomplete Paladins
- Campaign workflow continues from last checkpoint node

### User Story 4: State Inspection for Debugging
**As a** developer troubleshooting agent behavior  
**I want** to inspect saved state files in human-readable format  
**So that** I can understand agent decisions and conversation flow

**Acceptance Criteria:**
- State files are valid, formatted JSON
- Garrison entries are readable with clear role and content fields
- Execution history shows timestamps and outcomes
- Configuration values are easily identifiable

### User Story 5: Configurable State Directory
**As a** developer deploying to different environments  
**I want** to configure where state files are saved  
**So that** I can control persistence location per deployment (dev, staging, prod)

**Acceptance Criteria:**
- Default state directory is `./citadel/` in project root
- State directory can be configured via `PaladinBuilder.save_state_dir(path)`
- Directory is created automatically if it doesn't exist
- Invalid paths produce clear error messages at build time

## Functional Requirements

### FR1: Paladin State Serialization
The system must serialize complete Paladin state including:
- FR1.1: Paladin configuration (Node<PaladinData>)
- FR1.2: All Garrison entries with timestamps, roles, and content
- FR1.3: Execution history with timestamps, inputs, outputs, and status
- FR1.4: Created and updated timestamps
- FR1.5: State format must be valid JSON conforming to PaladinState schema

### FR2: Automatic State Persistence
The system must automatically persist state:
- FR2.1: Trigger save after every Paladin execution completion (success or failure)
- FR2.2: Save to file system in configured directory (default: `./citadel/`)
- FR2.3: Generate filename as `paladin-{uuid}.json`
- FR2.4: Overwrite existing state file for same Paladin ID
- FR2.5: Log save operations with timestamps and file paths

### FR3: Paladin State Restoration
The system must support restoring Paladins:
- FR3.1: `PaladinBuilder.restore_from(state_id: Uuid)` loads state from file
- FR3.2: Restore all configuration values from PaladinData
- FR3.3: Restore all Garrison entries maintaining chronological order
- FR3.4: Restore execution history for debugging/audit purposes
- FR3.5: Fail with clear error if state file not found or invalid JSON

### FR4: Battalion State Serialization
The system must serialize Battalion state including:
- FR4.1: Battalion type (Formation, Phalanx, Campaign, Chain of Command)
- FR4.2: Battalion configuration and orchestration parameters
- FR4.3: Complete state of all constituent Paladins
- FR4.4: Checkpoint data indicating last completed Paladin/stage
- FR4.5: State format must be valid JSON conforming to BattalionState schema

### FR5: Battalion Checkpoint Restoration
The system must resume Battalion workflows:
- FR5.1: Identify last successfully completed Paladin from checkpoint
- FR5.2: Formation resumes from next Paladin in sequence
- FR5.3: Phalanx re-executes only incomplete/failed Paladins
- FR5.4: Campaign continues from checkpoint node in graph
- FR5.5: Chain of Command resumes delegation from last level

### FR6: File Storage Operations
The system must implement file-based CitadelPort:
- FR6.1: `save_paladin(state: &PaladinState)` writes to file system
- FR6.2: `load_paladin(id: Uuid)` reads from file system
- FR6.3: `save_battalion(state: &BattalionState)` writes to file system
- FR6.4: `load_battalion(id: Uuid)` reads from file system
- FR6.5: `list_saved()` returns summary of all saved states in directory

### FR7: Error Handling
The system must handle errors robustly:
- FR7.1: Corrupted JSON files must fail with `CitadelError::CorruptedState`
- FR7.2: Missing files must fail with `CitadelError::StateNotFound`
- FR7.3: Incompatible schema versions must fail with `CitadelError::IncompatibleVersion`
- FR7.4: File system permission errors must fail with `CitadelError::IoError`
- FR7.5: All errors must include descriptive messages for debugging

### FR8: Builder Integration
The system must integrate with PaladinBuilder:
- FR8.1: `enable_autosave()` method activates automatic state persistence
- FR8.2: `save_state_dir(path: &str)` configures storage directory
- FR8.3: `restore_from(state_id: Uuid)` loads from saved state
- FR8.4: Builder validation ensures state directory is writable
- FR8.5: Configuration conflicts produce clear build-time errors

### FR9: State Directory Management
The system must manage the state directory:
- FR9.1: Create state directory automatically if not exists
- FR9.2: Verify write permissions at initialization
- FR9.3: Organize files as flat structure (no subdirectories)
- FR9.4: File naming convention: `paladin-{uuid}.json`, `battalion-{uuid}.json`
- FR9.5: Support configurable path (relative or absolute)

### FR10: Documentation and Logging
The system must provide visibility:
- FR10.1: Log state save operations at INFO level with file path
- FR10.2: Log state load operations at INFO level with state ID
- FR10.3: Log restoration events with Paladin/Battalion identifiers
- FR10.4: Rustdoc documentation for all public APIs
- FR10.5: Example code demonstrating save/restore workflows

## Non-Goals (Out of Scope)

### NG1: Multi-Version State History
The Citadel will NOT support versioning or keeping multiple historical states per Paladin. Each save operation overwrites the previous state file. Rationale: Simplifies implementation for MVP; versioning can be added in future epic if needed.

### NG2: Cloud Storage Backends
The Citadel will NOT support cloud storage (S3, MinIO, Azure Blob) in this epic. Only local file system persistence is in scope. Rationale: File system is sufficient for MVP and can be extended via adapter pattern later.

### NG3: Database Persistence
The Citadel will NOT use SQLite or other databases for state storage. JSON files only. Rationale: Human-readable files prioritized for debugging; database backend can be added as alternative adapter.

### NG4: Time-Based Autosave
The Citadel will NOT implement time-based autosave intervals. Only post-execution autosave is supported. Rationale: Time-based saving adds complexity around concurrent execution; can be added later if use cases emerge.

### NG5: Differential/Incremental Saves
The Citadel will NOT implement differential or incremental state updates. Each save is a complete state snapshot. Rationale: Full snapshots are simpler and more reliable for MVP.

### NG6: State Migration Tools
The Citadel will NOT provide migration utilities for schema version upgrades. Incompatible versions will fail to load. Rationale: Schema stability expected for MVP; migration tooling can be added when schema evolves.

### NG7: Partial Recovery
The Citadel will NOT attempt partial recovery from corrupted state files. All errors fail fast with clear messages. Rationale: Fail-fast approach is safer for production; partial recovery risks subtle bugs.

### NG8: Cross-Platform State Portability
The Citadel will NOT guarantee state files are portable across different Paladin versions or platforms. Rationale: Version compatibility can be addressed in future through schema versioning.

## Design Considerations

### Domain Model (Core Layer)

**Location:** `src/core/platform/container/citadel.rs`

The domain layer defines pure business types with no I/O dependencies:

```rust
/// Serializable Paladin state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaladinState {
    pub paladin: Paladin,
    pub garrison: Vec<GarrisonEntry>,
    pub execution_history: Vec<ExecutionRecord>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Serializable Battalion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattalionState {
    pub battalion_type: String,
    pub config: BattalionConfig,
    pub paladin_states: Vec<PaladinState>,
    pub checkpoint: Option<CheckpointData>,
}

/// Summary for listing saved states
#[derive(Debug, Clone)]
pub struct StateSummary {
    pub id: Uuid,
    pub state_type: StateType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub file_path: PathBuf,
}
```

### Port Definition (Application Layer)

**Location:** `src/application/ports/output/citadel_port.rs`

```rust
#[async_trait]
pub trait CitadelPort: Send + Sync {
    async fn save_paladin(&self, state: &PaladinState) -> Result<(), CitadelError>;
    async fn load_paladin(&self, id: Uuid) -> Result<Option<PaladinState>, CitadelError>;
    async fn save_battalion(&self, state: &BattalionState) -> Result<(), CitadelError>;
    async fn load_battalion(&self, id: Uuid) -> Result<Option<BattalionState>, CitadelError>;
    async fn list_saved(&self) -> Result<Vec<StateSummary>, CitadelError>;
}
```

### Adapter Implementation (Infrastructure Layer)

**Location:** `src/infrastructure/adapters/citadel/file_citadel.rs`

```rust
pub struct FileCitadel {
    state_dir: PathBuf,
}

impl FileCitadel {
    pub fn new(state_dir: impl Into<PathBuf>) -> Result<Self, CitadelError>;
    fn paladin_path(&self, id: Uuid) -> PathBuf;
    fn battalion_path(&self, id: Uuid) -> PathBuf;
}
```

### Builder Integration

```rust
impl PaladinBuilder {
    pub fn enable_autosave(mut self) -> Self {
        self.config.autosave_enabled = true;
        self
    }
    
    pub fn save_state_dir(mut self, path: impl Into<String>) -> Self {
        self.config.state_dir = Some(path.into());
        self
    }
    
    pub fn restore_from(mut self, state_id: Uuid) -> Result<Self, PaladinError> {
        // Load state and populate builder fields
    }
}
```

### File Structure

```
citadel/
├── paladin-{uuid1}.json
├── paladin-{uuid2}.json
├── battalion-{uuid3}.json
└── battalion-{uuid4}.json
```

### JSON Schema Example

```json
{
  "paladin": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "data": {
      "system_prompt": "You are a helpful assistant...",
      "name": "ResearchPaladin",
      "model": "gpt-4",
      "temperature": 0.7,
      "max_loops": 3,
      "status": "Completed"
    }
  },
  "garrison": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "role": "User",
      "content": "Research quantum computing",
      "timestamp": "2026-01-25T10:30:00Z"
    }
  ],
  "execution_history": [
    {
      "timestamp": "2026-01-25T10:30:15Z",
      "input": "Research quantum computing",
      "output": "Quantum computing leverages...",
      "status": "Success"
    }
  ],
  "created_at": "2026-01-25T10:30:00Z",
  "updated_at": "2026-01-25T10:30:15Z"
}
```

## Technical Considerations

### Dependencies

- **Epic 1 (Paladin Domain)**: Required for Paladin entity and configuration
- **Epic 2 (Garrison Memory)**: Required for GarrisonEntry serialization
- **serde/serde_json**: For JSON serialization (already in dependencies)
- **chrono**: For timestamp handling (already in dependencies)
- **tokio::fs**: For async file operations

### Hexagonal Architecture Compliance

The Citadel follows strict hexagonal architecture:

1. **Core Layer** (`citadel.rs`): Pure domain types with no I/O
2. **Application Layer** (`citadel_port.rs`): Port trait defining operations
3. **Infrastructure Layer** (`file_citadel.rs`): Concrete file system adapter

Dependencies flow inward only:
- Core: No imports from application or infrastructure
- Application: Imports core only
- Infrastructure: Implements application ports using core types

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum CitadelError {
    #[error("State not found: {0}")]
    StateNotFound(Uuid),
    
    #[error("Corrupted state file: {0}")]
    CorruptedState(String),
    
    #[error("Incompatible state version: expected {expected}, found {found}")]
    IncompatibleVersion { expected: String, found: String },
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

### Thread Safety

- `FileCitadel` must be `Send + Sync` for concurrent usage
- File operations use `tokio::fs` for async compatibility
- No shared mutable state (state_dir is immutable after construction)

### Testing Strategy

**Unit Tests (≥80% coverage):**
- State serialization/deserialization roundtrip
- Filename generation from UUIDs
- Error variant construction
- StateSummary creation

**Integration Tests:**
- Save and load Paladin state end-to-end
- Save and load Battalion state end-to-end
- Directory creation on first use
- File overwrites on subsequent saves
- Error handling for missing files, corrupted JSON
- List saved states functionality

**Test Organization:**
- Unit: `src/core/platform/container/citadel.rs` (inline tests)
- Integration: `tests/integration/citadel_integration_test.rs`

### Performance Considerations

- Async I/O prevents blocking during saves
- JSON serialization may be slow for large Garrisons (optimize later if needed)
- File system locality preferred (SSD recommended for production)

### Configuration

Add to `config.yml`:

```yaml
citadel:
  state_dir: "./citadel"  # Default state directory
  autosave_enabled: true   # Enable automatic saving
```

## Success Metrics

### Functional Metrics

1. **State Persistence Success Rate**: 100% of Paladin executions successfully save state
2. **State Restoration Success Rate**: 100% of valid state files successfully restore
3. **Battalion Resumption Accuracy**: 100% of Battalion checkpoints correctly skip completed Paladins

### Quality Metrics

4. **Test Coverage**: ≥80% unit test coverage, ≥70% integration test coverage
5. **Code Quality**: Zero clippy warnings, all code formatted with rustfmt
6. **Documentation**: 100% of public APIs have rustdoc comments

### Reliability Metrics

7. **Error Detection**: 100% of corrupted state files detected and reported clearly
8. **File System Errors**: All permission/I/O errors handled gracefully with descriptive messages

### Developer Experience Metrics

9. **State File Readability**: All state files validate as human-readable JSON
10. **Builder API Usability**: Zero ambiguity in enable_autosave/restore_from usage based on documentation

### Implementation Metrics

11. **Architecture Compliance**: Zero violations of hexagonal architecture boundaries
12. **Delivery**: Feature completed within 2-week effort estimate

## Open Questions

### Q1: State Schema Versioning
**Question:** Should we include a schema version field in state files for future compatibility?  
**Impact:** Medium - affects future migration strategy  
**Recommendation:** Add `"schema_version": "1.0.0"` field to both PaladinState and BattalionState for forward compatibility, even though migration tools are out of scope.

### Q2: Partial Garrison Serialization
**Question:** For very large Garrisons, should we limit serialization to most recent N entries?  
**Impact:** Low - unlikely in MVP  
**Recommendation:** Serialize complete Garrison for MVP; add truncation configuration in future epic if needed.

### Q3: Concurrent Save Operations
**Question:** How should concurrent saves to the same state file be handled?  
**Impact:** Low - single Paladin executions are sequential  
**Recommendation:** Document that autosave is designed for sequential execution; concurrent access is developer's responsibility.

### Q4: State Cleanup Strategy
**Question:** Should there be automatic cleanup of old state files?  
**Impact:** Medium - affects disk usage over time  
**Recommendation:** Out of scope for MVP; document that developers should manage state directory lifecycle.

### Q5: Integration with Existing Repositories
**Question:** Should Citadel integrate with existing repository infrastructure (MySQL/SQLite)?  
**Impact:** Medium - affects architecture consistency  
**Recommendation:** No for MVP - maintain separation. Citadel is state snapshots; repositories are operational data. Can integrate later via additional adapters.

---

## Appendix: Example Usage

### Basic Paladin with Autosave

```rust
use paladin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_port = Arc::new(OpenAIAdapter::new("gpt-4")?);
    
    let paladin = PaladinBuilder::new(llm_port)
        .system_prompt("You are a research assistant")
        .name("ResearchPaladin")
        .enable_autosave()
        .save_state_dir("./my_app/state")
        .build()?;
    
    // State automatically saved after execution
    let result = paladin.execute("Research quantum computing").await?;
    println!("Result: {}", result);
    
    Ok(())
}
```

### Restoring from Saved State

```rust
use paladin::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llm_port = Arc::new(OpenAIAdapter::new("gpt-4")?);
    let state_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
    
    let paladin = PaladinBuilder::new(llm_port)
        .restore_from(state_id)?
        .build()?;
    
    // Continue conversation from restored Garrison
    let result = paladin.execute("Continue from where we left off").await?;
    
    Ok(())
}
```

### Battalion with Checkpoint Recovery

```rust
use paladin::battalion::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formation = FormationBuilder::new()
        .add_paladin(research_paladin)
        .add_paladin(analysis_paladin)
        .add_paladin(summary_paladin)
        .enable_checkpoints()
        .save_state_dir("./workflows")
        .build()?;
    
    // If failure occurs after analysis_paladin completes,
    // resuming will skip research_paladin and analysis_paladin
    let result = formation.execute("Analyze market trends").await?;
    
    Ok(())
}
```

---

**Document Version:** 1.0  
**Created:** 2026-01-25  
**Epic:** Epic 7 - Citadel State Persistence  
**Priority:** Medium  
**Target Audience:** Junior to Mid-Level Rust Developers
