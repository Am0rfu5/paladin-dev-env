//! Qdrant vector database implementation of SanctumPort
//!
//! This adapter provides production-grade vector storage using Qdrant.
//! Suitable for:
//! - Production deployments with millions of vectors
//! - High-performance semantic search (< 500ms for 100K vectors)
//! - Distributed and scalable workloads
//!
//! Features:
//! - Automatic collection creation with proper configuration
//! - Cosine similarity search with HNSW indexing
//! - Metadata filtering via Qdrant filter DSL
//! - Batch operations for efficiency
//!
//! Configuration:
//! - Vector dimension must be specified at creation
//! - Collection is auto-created if it doesn't exist
//! - Supports both HTTP and gRPC protocols

use crate::application::ports::output::sanctum_port::{
    SanctumError, SanctumFilter, SanctumPort, SanctumQuery, SanctumSearchResult,
};
use crate::core::platform::container::sanctum::{Memory, MemoryType, SanctumEntry};
use async_trait::async_trait;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::r#match::MatchValue;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, Distance, Filter, PointStruct, Range, Value as QdrantValue, VectorParams,
    VectorsConfig,
};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Qdrant adapter for production vector storage
///
/// Provides high-performance semantic search with automatic collection management.
#[doc(hidden)]
pub struct QdrantSanctumAdapter {
    client: Qdrant,
    collection_name: String,
    vector_dimension: usize,
}

impl QdrantSanctumAdapter {
    /// Create a new Qdrant adapter
    ///
    /// # Arguments
    /// * `url` - Qdrant server URL (e.g., "http://localhost:6334")
    /// * `collection_name` - Name of the collection to use
    /// * `vector_dimension` - Dimension of embedding vectors
    ///
    /// # Example
    /// ```no_run
    /// use paladin::infrastructure::adapters::sanctum::QdrantSanctumAdapter;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let adapter = QdrantSanctumAdapter::new(
    ///     "http://localhost:6334",
    ///     "paladin_memories",
    ///     1536
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(
        url: &str,
        collection_name: &str,
        vector_dimension: usize,
    ) -> Result<Self, SanctumError> {
        let client = Qdrant::from_url(url).build().map_err(|e| {
            SanctumError::ConfigError(format!("Failed to create Qdrant client: {}", e))
        })?;

        let adapter = Self {
            client,
            collection_name: collection_name.to_string(),
            vector_dimension,
        };

        // Ensure collection exists
        adapter.ensure_collection_exists().await?;

        Ok(adapter)
    }

    /// Ensure the collection exists, creating it if necessary
    async fn ensure_collection_exists(&self) -> Result<(), SanctumError> {
        // Check if collection exists
        let collections = self.client.list_collections().await.map_err(|e| {
            SanctumError::StorageError(format!("Failed to list collections: {}", e))
        })?;

        let exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.collection_name);

        if !exists {
            // Create collection with cosine distance metric
            self.client
                .create_collection(
                    qdrant_client::qdrant::CreateCollectionBuilder::new(&self.collection_name)
                        .vectors_config(VectorsConfig {
                            config: Some(Config::Params(VectorParams {
                                size: self.vector_dimension as u64,
                                distance: Distance::Cosine.into(),
                                hnsw_config: None,
                                quantization_config: None,
                                on_disk: None,
                                datatype: None,
                                multivector_config: None,
                            })),
                        }),
                )
                .await
                .map_err(|e| {
                    SanctumError::StorageError(format!("Failed to create collection: {}", e))
                })?;
        }

        Ok(())
    }

    /// Convert SanctumEntry to Qdrant PointStruct
    fn entry_to_point(&self, entry: &SanctumEntry) -> Result<PointStruct, SanctumError> {
        // Validate dimension
        if entry.embedding.len() != self.vector_dimension {
            return Err(SanctumError::InvalidDimension(format!(
                "Expected {} dimensions, got {}",
                self.vector_dimension,
                entry.embedding.len()
            )));
        }

        // Build payload with all metadata
        let mut payload = HashMap::new();
        payload.insert(
            "paladin_id".to_string(),
            QdrantValue::from(entry.memory.paladin_id.clone()),
        );
        payload.insert(
            "content".to_string(),
            QdrantValue::from(entry.memory.content.clone()),
        );
        payload.insert(
            "memory_type".to_string(),
            QdrantValue::from(format!("{:?}", entry.memory.memory_type)),
        );
        payload.insert(
            "importance".to_string(),
            QdrantValue::from(entry.memory.importance as f64),
        );
        payload.insert(
            "access_count".to_string(),
            QdrantValue::from(entry.memory.access_count as i64),
        );
        payload.insert(
            "created_at".to_string(),
            QdrantValue::from(entry.memory.created_at.timestamp()),
        );
        payload.insert(
            "last_accessed".to_string(),
            QdrantValue::from(entry.memory.last_accessed.timestamp()),
        );

        // Add custom metadata
        for (key, value) in &entry.memory.metadata {
            let qdrant_value = json_to_qdrant_value(value);
            payload.insert(format!("meta_{}", key), qdrant_value);
        }

        Ok(PointStruct::new(
            entry.memory.id.to_string(),
            entry.embedding.clone(),
            payload,
        ))
    }

    /// Convert Qdrant point back to SanctumEntry
    fn point_to_entry(
        &self,
        point: qdrant_client::qdrant::ScoredPoint,
    ) -> Result<SanctumEntry, SanctumError> {
        let payload = point.payload;

        // Extract memory fields
        let paladin_id = payload
            .get("paladin_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SanctumError::StorageError("Missing paladin_id".into()))?
            .to_string();

        let content = payload
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SanctumError::StorageError("Missing content".into()))?
            .to_string();

        let memory_type_str = payload
            .get("memory_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SanctumError::StorageError("Missing memory_type".into()))?;

        let memory_type = match memory_type_str.as_str() {
            "Episodic" => MemoryType::Episodic,
            "Semantic" => MemoryType::Semantic,
            "Procedural" => MemoryType::Procedural,
            _ => {
                return Err(SanctumError::StorageError(format!(
                    "Invalid memory_type: {}",
                    memory_type_str
                )));
            }
        };

        let importance = payload
            .get("importance")
            .and_then(|v| v.as_double().or_else(|| v.as_integer().map(|i| i as f64)))
            .ok_or_else(|| SanctumError::StorageError("Missing importance".into()))?
            as f32;

        let access_count = payload
            .get("access_count")
            .and_then(|v| v.as_integer())
            .ok_or_else(|| SanctumError::StorageError("Missing access_count".into()))?
            as u32;

        let created_at = payload
            .get("created_at")
            .and_then(|v| v.as_integer())
            .ok_or_else(|| SanctumError::StorageError("Missing created_at".into()))?;

        let last_accessed = payload
            .get("last_accessed")
            .and_then(|v| v.as_integer())
            .ok_or_else(|| SanctumError::StorageError("Missing last_accessed".into()))?;

        // Extract custom metadata
        let mut metadata = HashMap::new();
        for (key, value) in payload.iter() {
            if let Some(meta_key) = key.strip_prefix("meta_") {
                metadata.insert(meta_key.to_string(), qdrant_value_to_json(value));
            }
        }

        // Parse UUID from point ID
        let id_str = match point
            .id
            .as_ref()
            .and_then(|id| id.point_id_options.as_ref())
        {
            Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(uuid)) => uuid.clone(),
            Some(qdrant_client::qdrant::point_id::PointIdOptions::Num(num)) => num.to_string(),
            _ => return Err(SanctumError::StorageError("Invalid point ID".into())),
        };

        let id = Uuid::parse_str(&id_str)
            .map_err(|e| SanctumError::StorageError(format!("Invalid UUID: {}", e)))?;

        let memory = Memory {
            id,
            paladin_id,
            content,
            memory_type,
            importance,
            access_count,
            created_at: chrono::DateTime::from_timestamp(created_at, 0)
                .ok_or_else(|| SanctumError::StorageError("Invalid timestamp".into()))?,
            last_accessed: chrono::DateTime::from_timestamp(last_accessed, 0)
                .ok_or_else(|| SanctumError::StorageError("Invalid timestamp".into()))?,
            metadata,
        };

        let vector = match &point.vectors {
            None => return Err(SanctumError::StorageError("No vectors in point".into())),
            Some(v) => match &v.vectors_options {
                None => {
                    return Err(SanctumError::StorageError(
                        "No vector options in point".into(),
                    ));
                }
                Some(qdrant_client::qdrant::vectors_output::VectorsOptions::Vector(vec_output)) => {
                    #[allow(deprecated)]
                    {
                        vec_output.data.clone()
                    }
                }
                Some(_) => {
                    return Err(SanctumError::StorageError(
                        "Unexpected vector format".into(),
                    ));
                }
            },
        };

        SanctumEntry::new(memory, vector).map_err(SanctumError::StorageError)
    }

    /// Convert SanctumFilter to Qdrant Filter
    fn build_qdrant_filter(&self, filter: &SanctumFilter) -> Option<Filter> {
        let mut conditions = Vec::new();

        // Filter by paladin_id
        if let Some(ref paladin_id) = filter.paladin_id {
            conditions.push(Condition::matches("paladin_id", paladin_id.clone()));
        }

        // Filter by memory_type
        if let Some(memory_type) = filter.memory_type {
            conditions.push(Condition::matches(
                "memory_type",
                format!("{:?}", memory_type),
            ));
        }

        // Filter by importance
        if let Some(min_importance) = filter.min_importance {
            conditions.push(Condition::range(
                "importance",
                Range {
                    gte: Some(min_importance as f64),
                    ..Default::default()
                },
            ));
        }

        // Filter by created_at
        if let Some(created_after) = filter.created_after {
            conditions.push(Condition::range(
                "created_at",
                Range {
                    gte: Some(created_after.timestamp() as f64),
                    ..Default::default()
                },
            ));
        }

        if let Some(created_before) = filter.created_before {
            conditions.push(Condition::range(
                "created_at",
                Range {
                    lte: Some(created_before.timestamp() as f64),
                    ..Default::default()
                },
            ));
        }

        // Filter by metadata
        for (key, value) in &filter.metadata_filters {
            if let Some(match_value) = json_to_match_value(value) {
                conditions.push(Condition::matches(format!("meta_{}", key), match_value));
            }
        }

        if conditions.is_empty() {
            None
        } else {
            Some(Filter::must(conditions))
        }
    }
}

#[async_trait]
impl SanctumPort for QdrantSanctumAdapter {
    /// Store a single entry in Qdrant
    async fn store(&self, entry: SanctumEntry) -> Result<(), SanctumError> {
        let point = self.entry_to_point(&entry)?;

        self.client
            .upsert_points(qdrant_client::qdrant::UpsertPointsBuilder::new(
                &self.collection_name,
                vec![point],
            ))
            .await
            .map_err(|e| SanctumError::StorageError(format!("Failed to store entry: {}", e)))?;

        Ok(())
    }

    /// Store multiple entries in batch
    async fn store_batch(&self, entries: Vec<SanctumEntry>) -> Result<(), SanctumError> {
        let points: Result<Vec<_>, _> = entries.iter().map(|e| self.entry_to_point(e)).collect();
        let points = points?;

        self.client
            .upsert_points(qdrant_client::qdrant::UpsertPointsBuilder::new(
                &self.collection_name,
                points,
            ))
            .await
            .map_err(|e| SanctumError::StorageError(format!("Failed to store batch: {}", e)))?;

        Ok(())
    }

    /// Perform semantic search with optional filtering
    async fn search(&self, query: SanctumQuery) -> Result<Vec<SanctumSearchResult>, SanctumError> {
        // Validate dimension
        if query.embedding.len() != self.vector_dimension {
            return Err(SanctumError::InvalidDimension(format!(
                "Expected {} dimensions, got {}",
                self.vector_dimension,
                query.embedding.len()
            )));
        }

        // Build search request
        let mut search_builder = qdrant_client::qdrant::SearchPointsBuilder::new(
            &self.collection_name,
            query.embedding,
            query.top_k as u64,
        )
        .with_payload(true)
        .with_vectors(true); // Important: request vectors in results

        if let Some(min_score) = query.min_score {
            search_builder = search_builder.score_threshold(min_score);
        }

        if let Some(ref filter) = query.filter
            && let Some(qdrant_filter) = self.build_qdrant_filter(filter)
        {
            search_builder = search_builder.filter(qdrant_filter);
        }

        // Execute search
        let search_result = self
            .client
            .search_points(search_builder)
            .await
            .map_err(|e| SanctumError::SearchError(format!("Search failed: {}", e)))?;

        // Convert results
        let results: Result<Vec<_>, _> = search_result
            .result
            .into_iter()
            .map(|point| {
                let score = point.score;
                let entry = self.point_to_entry(point)?;
                Ok(SanctumSearchResult { entry, score })
            })
            .collect();

        results
    }

    /// Delete an entry by ID
    async fn delete(&self, id: &str) -> Result<bool, SanctumError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| SanctumError::NotFound(format!("Invalid UUID: {}", e)))?;

        // Check if exists first
        let point_ids: Vec<_> = vec![uuid.to_string()]
            .into_iter()
            .map(|s| s.into())
            .collect();
        let result = self
            .client
            .get_points(
                qdrant_client::qdrant::GetPointsBuilder::new(&self.collection_name, point_ids)
                    .with_payload(false),
            )
            .await
            .map_err(|e| SanctumError::StorageError(format!("Failed to check existence: {}", e)))?;

        if result.result.is_empty() {
            return Ok(false);
        }

        // Delete the point
        self.client
            .delete_points(
                qdrant_client::qdrant::DeletePointsBuilder::new(&self.collection_name)
                    .points(vec![uuid.to_string()]),
            )
            .await
            .map_err(|e| SanctumError::StorageError(format!("Failed to delete entry: {}", e)))?;

        Ok(true)
    }

    /// Update an existing entry (upsert)
    async fn update(&self, entry: SanctumEntry) -> Result<(), SanctumError> {
        // Qdrant upsert handles both insert and update
        self.store(entry).await
    }

    /// Count entries matching optional filter
    async fn count(&self, filter: Option<SanctumFilter>) -> Result<usize, SanctumError> {
        let mut count_builder =
            qdrant_client::qdrant::CountPointsBuilder::new(&self.collection_name);

        if let Some(f) = filter
            && let Some(qdrant_filter) = self.build_qdrant_filter(&f)
        {
            count_builder = count_builder.filter(qdrant_filter);
        }

        let count_result = self
            .client
            .count(count_builder)
            .await
            .map_err(|e| SanctumError::StorageError(format!("Failed to count: {}", e)))?;

        Ok(count_result.result.unwrap().count as usize)
    }
}

/// Convert JSON Value to Qdrant Value
fn json_to_qdrant_value(value: &Value) -> QdrantValue {
    match value {
        Value::String(s) => QdrantValue::from(s.clone()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                QdrantValue::from(i)
            } else if let Some(f) = n.as_f64() {
                QdrantValue::from(f)
            } else {
                QdrantValue::from(0)
            }
        }
        Value::Bool(b) => QdrantValue::from(*b),
        _ => QdrantValue::from(value.to_string()),
    }
}

/// Convert JSON Value to MatchValue for filtering
fn json_to_match_value(value: &Value) -> Option<MatchValue> {
    match value {
        Value::String(s) => Some(MatchValue::from(s.clone())),
        Value::Number(n) => n.as_i64().map(MatchValue::from),
        Value::Bool(b) => Some(MatchValue::from(*b)),
        _ => None,
    }
}

/// Convert Qdrant Value to JSON Value
fn qdrant_value_to_json(value: &QdrantValue) -> Value {
    if let Some(s) = value.as_str() {
        Value::String(s.to_string())
    } else if let Some(i) = value.as_integer() {
        Value::Number(i.into())
    } else if let Some(f) = value.as_double() {
        Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| 0.into()))
    } else if let Some(b) = value.as_bool() {
        Value::Bool(b)
    } else {
        Value::String(format!("{:?}", value))
    }
}
