## Epic 7: Citadel State Persistence

### Overview

**Priority:** Medium  
**Effort:** 2 weeks  
**Dependencies:** Epics 1, 2  
**Team:** 1 developer

**Objective:** Implement the Citadel persistence layer for autosave and state restoration of Paladins and Battalions.

### Technical Design

#### Domain Layer

**citadel.rs - State Management**

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
```

#### Application Layer

**ports/output/citadel_port.rs**

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

### Builder Integration

```rust
impl PaladinBuilder {
    /// Enable automatic state persistence
    pub fn enable_autosave(self) -> Self;
    
    /// Set directory for state files
    pub fn save_state_dir(self, path: &str) -> Self;
    
    /// Restore from saved state
    pub fn restore_from(self, state_id: Uuid) -> Self;
}
```

### Acceptance Criteria

- [ ] Paladin state persists across restarts
- [ ] Autosave triggers on configurable events
- [ ] Battalion workflows can resume from checkpoints
- [ ] State files are human-readable JSON

---
