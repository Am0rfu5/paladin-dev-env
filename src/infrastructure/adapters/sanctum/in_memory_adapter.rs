//! In-memory implementation of SanctumPort for development and testing
//!
//! This adapter provides a fast, thread-safe in-memory storage for semantic memories
//! with cosine similarity search. It's ideal for:
//! - Development and testing
//! - Small-scale deployments
//! - Prototyping
//!
//! Features:
//! - Thread-safe concurrent access with RwLock
//! - LRU eviction when capacity is reached
//! - Cosine similarity-based semantic search
//! - Metadata filtering (paladin_id, memory_type, importance, timestamps)
//!
//! Performance characteristics:
//! - Storage: O(1)
//! - Search: O(n) brute-force comparison (suitable for < 10K vectors)
//! - Memory overhead: All vectors kept in RAM

use crate::application::ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery, SanctumSearchResult,
};
use crate::core::platform::container::sanctum::SanctumEntry;
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Configuration for InMemorySanctum
#[derive(Debug, Clone)]
pub struct InMemorySanctumConfig {
    /// Maximum number of entries to store (LRU eviction when exceeded)
    pub max_entries: usize,
}

impl Default for InMemorySanctumConfig {
    fn default() -> Self {
        Self {
            max_entries: 10_000,
        }
    }
}

/// In-memory implementation of SanctumPort
///
/// Thread-safe storage with LRU eviction and brute-force semantic search.
/// Suitable for development, testing, and small-scale production (< 10K vectors).
pub struct InMemorySanctum {
    /// Entry storage keyed by memory ID (Uuid)
    storage: Arc<RwLock<HashMap<Uuid, SanctumEntry>>>,

    /// LRU tracking: oldest entries at the front
    lru_queue: Arc<RwLock<VecDeque<Uuid>>>,

    /// Configuration
    config: InMemorySanctumConfig,
}

impl InMemorySanctum {
    /// Create a new InMemorySanctum with the given capacity
    ///
    /// # Arguments
    /// * `max_entries` - Maximum number of entries before LRU eviction
    ///
    /// # Example
    /// ```
    /// use paladin::infrastructure::adapters::sanctum::InMemorySanctum;
    ///
    /// let sanctum = InMemorySanctum::new(1000);
    /// ```
    pub fn new(max_entries: usize) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            lru_queue: Arc::new(RwLock::new(VecDeque::new())),
            config: InMemorySanctumConfig { max_entries },
        }
    }

    /// Create a new InMemorySanctum with custom configuration
    pub fn with_config(config: InMemorySanctumConfig) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            lru_queue: Arc::new(RwLock::new(VecDeque::new())),
            config,
        }
    }

    /// Calculate cosine similarity between two vectors
    ///
    /// Returns a score between -1.0 and 1.0, where 1.0 means identical direction.
    fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, SanctumError> {
        if a.len() != b.len() {
            return Err(SanctumError::InvalidDimension(format!(
                "Vector dimensions don't match: {} vs {}",
                a.len(),
                b.len()
            )));
        }

        if a.is_empty() {
            return Err(SanctumError::InvalidDimension(
                "Cannot calculate similarity of empty vectors".to_string(),
            ));
        }

        // Dot product
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

        // Magnitudes
        let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            return Ok(0.0); // Treat zero vectors as orthogonal
        }

        Ok(dot_product / (mag_a * mag_b))
    }

    /// Check if an entry matches the given filter
    fn matches_filter(entry: &SanctumEntry, filter: &SanctumFilter) -> bool {
        // Filter by paladin_id
        if let Some(ref paladin_id) = filter.paladin_id
            && &entry.memory.paladin_id != paladin_id
        {
            return false;
        }

        // Filter by memory_type
        if let Some(memory_type) = filter.memory_type
            && entry.memory.memory_type != memory_type
        {
            return false;
        }

        // Filter by created_after
        if let Some(created_after) = filter.created_after
            && entry.memory.created_at < created_after
        {
            return false;
        }

        // Filter by created_before
        if let Some(created_before) = filter.created_before
            && entry.memory.created_at > created_before
        {
            return false;
        }

        // Filter by min_importance
        if let Some(min_importance) = filter.min_importance
            && entry.memory.importance < min_importance
        {
            return false;
        }

        // Filter by metadata (exact match for now)
        for (key, value) in &filter.metadata_filters {
            match entry.memory.metadata.get(key) {
                Some(entry_value) if entry_value == value => {}
                _ => return false,
            }
        }

        true
    }

    /// Perform LRU eviction if capacity is exceeded
    fn evict_if_needed(&self) {
        let storage = self
            .storage
            .read()
            .expect("Failed to acquire read lock on storage");

        if storage.len() >= self.config.max_entries {
            drop(storage); // Release read lock before acquiring write lock

            let mut lru = self
                .lru_queue
                .write()
                .expect("Failed to acquire write lock on LRU queue");
            let mut storage = self
                .storage
                .write()
                .expect("Failed to acquire write lock on storage");

            // Remove oldest entry
            if let Some(oldest_id) = lru.pop_front() {
                storage.remove(&oldest_id);
            }
        }
    }

    /// Update LRU queue when accessing an entry
    fn touch_entry(&self, id: &Uuid) {
        let mut lru = self
            .lru_queue
            .write()
            .expect("Failed to acquire write lock on LRU queue");

        // Remove from current position
        if let Some(pos) = lru.iter().position(|x| x == id) {
            lru.remove(pos);
        }

        // Add to back (most recently used)
        lru.push_back(*id);
    }
}

#[async_trait]
impl SanctumPort for InMemorySanctum {
    /// Store a single entry in memory
    async fn store(&self, entry: SanctumEntry) -> Result<(), SanctumError> {
        // Check for capacity and evict if needed
        self.evict_if_needed();

        let id = entry.memory.id;

        let mut storage = self
            .storage
            .write()
            .expect("Failed to acquire write lock on storage");

        storage.insert(id, entry);
        drop(storage); // Release write lock

        // Update LRU queue
        self.touch_entry(&id);

        Ok(())
    }

    /// Store multiple entries in batch
    async fn store_batch(&self, entries: Vec<SanctumEntry>) -> Result<(), SanctumError> {
        for entry in entries {
            self.store(entry).await?;
        }
        Ok(())
    }

    /// Perform semantic search with cosine similarity
    async fn search(&self, query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError> {
        let storage = self
            .storage
            .read()
            .expect("Failed to acquire read lock on storage");

        let mut results = Vec::new();

        // Calculate similarity for all entries that match the filter
        for entry in storage.values() {
            // Apply filter if provided
            if let Some(ref filter) = query.filter
                && !Self::matches_filter(entry, filter)
            {
                continue;
            }

            // Calculate cosine similarity
            let score = Self::cosine_similarity(&query.embedding, &entry.embedding)?;

            // Apply min_score filter
            if let Some(min_score) = query.min_score
                && score < min_score
            {
                continue;
            }

            results.push(SanctumSearchResult {
                entry: entry.clone(),
                score,
            });
        }

        // Sort by score descending (highest similarity first)
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply top_k limit
        results.truncate(query.top_k);

        Ok(results)
    }

    /// Delete an entry by ID
    async fn delete(&self, id: &str) -> Result<bool, SanctumError> {
        // Parse the id string to Uuid
        let uuid = Uuid::parse_str(id)
            .map_err(|e| SanctumError::NotFound(format!("Invalid UUID format: {}", e)))?;

        let mut storage = self
            .storage
            .write()
            .expect("Failed to acquire write lock on storage");

        let removed = storage.remove(&uuid).is_some();

        if removed {
            drop(storage); // Release write lock

            // Remove from LRU queue
            let mut lru = self
                .lru_queue
                .write()
                .expect("Failed to acquire write lock on LRU queue");

            if let Some(pos) = lru.iter().position(|x| x == &uuid) {
                lru.remove(pos);
            }
        }

        Ok(removed)
    }

    /// Update an existing entry
    async fn update(&self, entry: SanctumEntry) -> Result<(), SanctumError> {
        let id = entry.memory.id;

        let mut storage = self
            .storage
            .write()
            .expect("Failed to acquire write lock on storage");

        // Check if entry exists
        if !storage.contains_key(&id) {
            return Err(SanctumError::NotFound(format!("Entry not found: {}", id)));
        }

        storage.insert(id, entry);
        drop(storage); // Release write lock

        // Update LRU queue (treat update as access)
        self.touch_entry(&id);

        Ok(())
    }

    /// Count entries matching optional filter
    async fn count(&self, filter: Option<SanctumFilter>) -> Result<usize, SanctumError> {
        let storage = self
            .storage
            .read()
            .expect("Failed to acquire read lock on storage");

        if let Some(filter) = filter {
            Ok(storage
                .values()
                .filter(|entry| Self::matches_filter(entry, &filter))
                .count())
        } else {
            Ok(storage.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = InMemorySanctum::cosine_similarity(&a, &b).unwrap();
        assert!((similarity - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let similarity = InMemorySanctum::cosine_similarity(&a, &b).unwrap();
        assert!((similarity - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![-1.0, 0.0, 0.0];
        let similarity = InMemorySanctum::cosine_similarity(&a, &b).unwrap();
        assert!((similarity - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_dimension_mismatch() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let result = InMemorySanctum::cosine_similarity(&a, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_cosine_similarity_empty() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        let result = InMemorySanctum::cosine_similarity(&a, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = InMemorySanctum::cosine_similarity(&a, &b).unwrap();
        assert_eq!(similarity, 0.0);
    }
}
