//! In-memory Paladin registry — pure-Rust HashMap implementation.
//!
//! Provides a thread-safe, `HashMap`-based implementation of [`PaladinRegistry`]
//! for storing and retrieving [`Paladin`] instances by string ID.  Used
//! internally by [`commander`](crate::commander) and [`grove_service`](crate::grove_service)
//! to assemble temporary registries during execution.  Because this type
//! depends only on [`paladin-core`] and [`paladin-ports`] it carries zero
//! infrastructure overhead.

use paladin_core::platform::container::paladin::Paladin;
use paladin_ports::output::paladin_registry::{PaladinRegistry, RegistryError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// HashMap-backed Paladin registry with thread-safe concurrent access.
///
/// Uses a `RwLock<HashMap>` internally to allow multiple concurrent readers
/// or a single exclusive writer.
#[derive(Debug, Default)]
pub struct HashMapPaladinRegistry {
    paladins: RwLock<HashMap<String, Arc<Paladin>>>,
}

impl HashMapPaladinRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            paladins: RwLock::new(HashMap::new()),
        }
    }
}

impl PaladinRegistry for HashMapPaladinRegistry {
    fn register(&self, id: String, paladin: Arc<Paladin>) -> Result<(), RegistryError> {
        if id.is_empty() {
            return Err(RegistryError::InvalidId("ID cannot be empty".to_string()));
        }
        let mut map = self
            .paladins
            .write()
            .map_err(|e| RegistryError::AccessFailed(format!("Write lock poisoned: {e}")))?;
        if map.contains_key(&id) {
            return Err(RegistryError::DuplicateId(id));
        }
        map.insert(id, paladin);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<Arc<Paladin>> {
        let map = self.paladins.read().ok()?;
        map.get(id).cloned()
    }

    fn contains(&self, id: &str) -> bool {
        let Ok(map) = self.paladins.read() else {
            return false;
        };
        map.contains_key(id)
    }

    fn list_ids(&self) -> Vec<String> {
        let Ok(map) = self.paladins.read() else {
            return Vec::new();
        };
        map.keys().cloned().collect()
    }
}
