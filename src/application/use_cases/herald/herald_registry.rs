//! Herald Registry for managing output formatters
//!
//! The Herald Registry provides centralized management of output formatters,
//! allowing registration, retrieval, and listing of available Herald implementations.
//! The registry is thread-safe and supports concurrent access.
//!
//! # Examples
//!
//! ```rust,ignore
//! use paladin::application::use_cases::herald::HeraldRegistry;
//! use paladin::infrastructure::adapters::herald::JsonHerald;
//! use std::sync::Arc;
//!
//! let mut registry = HeraldRegistry::new();
//! registry.register("json", Arc::new(JsonHerald::new()));
//!
//! let herald = registry.get("json").unwrap();
//! ```

use crate::core::platform::container::herald::Herald;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe registry for managing Herald formatters
///
/// The `HeraldRegistry` provides a centralized location for storing and retrieving
/// Herald implementations. It uses a `RwLock` to ensure thread-safe concurrent access,
/// allowing multiple readers or a single writer at a time.
///
/// # Thread Safety
///
/// The registry is fully thread-safe and can be shared across multiple threads using
/// `Arc<HeraldRegistry>`. Read operations (get, list) can occur concurrently, while
/// write operations (register) require exclusive access.
#[derive(Clone)]
pub struct HeraldRegistry {
    /// Internal storage of Herald implementations keyed by name
    formatters: Arc<RwLock<HashMap<String, Arc<dyn Herald>>>>,
}

impl HeraldRegistry {
    /// Create a new empty Herald registry
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let registry = HeraldRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            formatters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a Herald formatter with a given name
    ///
    /// Adds a new formatter to the registry. If a formatter with the same name
    /// already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique identifier for this formatter (e.g., "json", "markdown")
    /// * `herald` - The Herald implementation to register
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut registry = HeraldRegistry::new();
    /// registry.register("json", Arc::new(JsonHerald::new()));
    /// ```
    pub fn register(&self, name: impl Into<String>, herald: Arc<dyn Herald>) {
        let mut formatters = self.formatters.write().unwrap();
        formatters.insert(name.into(), herald);
    }

    /// Retrieve a Herald formatter by name
    ///
    /// Returns a reference to the requested formatter if it exists in the registry.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the formatter to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(Arc<dyn Herald>)` if the formatter exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if let Some(herald) = registry.get("json") {
    ///     let formatted = herald.format_paladin_result(&result)?;
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<Arc<dyn Herald>> {
        let formatters = self.formatters.read().unwrap();
        formatters.get(name).cloned()
    }

    /// List all available formatter names
    ///
    /// Returns a vector containing the names of all registered formatters.
    /// The order of names is not guaranteed.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all registered formatter names.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let available_formatters = registry.list();
    /// println!("Available formatters: {:?}", available_formatters);
    /// ```
    pub fn list(&self) -> Vec<String> {
        let formatters = self.formatters.read().unwrap();
        formatters.keys().cloned().collect()
    }

    /// Check if a formatter is registered
    ///
    /// Returns `true` if a formatter with the given name exists in the registry.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the formatter to check
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if registry.contains("json") {
    ///     println!("JSON formatter is available");
    /// }
    /// ```
    pub fn contains(&self, name: &str) -> bool {
        let formatters = self.formatters.read().unwrap();
        formatters.contains_key(name)
    }

    /// Get the number of registered formatters
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// println!("Registry has {} formatters", registry.len());
    /// ```
    pub fn len(&self) -> usize {
        let formatters = self.formatters.read().unwrap();
        formatters.len()
    }

    /// Check if the registry is empty
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if registry.is_empty() {
    ///     println!("No formatters registered");
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        let formatters = self.formatters.read().unwrap();
        formatters.is_empty()
    }
}

impl Default for HeraldRegistry {
    /// Create a new registry with built-in formatters pre-registered
    ///
    /// The default registry includes:
    /// - "json" - JSON formatter (when available)
    /// - "markdown" - Markdown formatter (when available)
    /// - "table" - Table formatter (when available)
    ///
    /// Note: Built-in formatters will be registered once they are implemented
    /// in the infrastructure layer.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let registry = HeraldRegistry::default();
    /// // Built-in formatters are already registered
    /// let json_herald = registry.get("json").unwrap();
    /// ```
    fn default() -> Self {
        // TODO: Register built-in formatters once they are implemented
        // registry.register("json", Arc::new(JsonHerald::new()));
        // registry.register("markdown", Arc::new(MarkdownHerald::new()));
        // registry.register("table", Arc::new(TableHerald::new()));

        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::container::herald::{
        BattalionResult, ExecutionMetadata, Herald, HeraldError, PaladinError, PaladinResult,
        StreamChunk,
    };

    // Mock Herald for testing
    struct MockHerald {
        name: String,
    }

    impl MockHerald {
        fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }
    }

    impl Herald for MockHerald {
        fn format_paladin_result(&self, _result: &PaladinResult) -> Result<String, HeraldError> {
            Ok(format!("{} formatted", self.name))
        }

        fn format_battalion_result(
            &self,
            _result: &BattalionResult,
        ) -> Result<String, HeraldError> {
            Ok(format!("{} battalion formatted", self.name))
        }

        fn format_stream_chunk(&self, _chunk: &StreamChunk) -> Result<Option<String>, HeraldError> {
            Ok(Some(self.name.clone()))
        }

        fn finalize_stream(&self, _metadata: &ExecutionMetadata) -> Result<String, HeraldError> {
            Ok(format!("{} finalized", self.name))
        }

        fn format_error(&self, _error: &PaladinError) -> String {
            format!("{} error", self.name)
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn mime_type(&self) -> &str {
            "text/plain"
        }
    }

    #[test]
    fn test_new_registry_is_empty() {
        let registry = HeraldRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_register_and_get_formatter() {
        let registry = HeraldRegistry::new();
        let herald = Arc::new(MockHerald::new("test"));

        registry.register("test", herald);

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);

        let retrieved = registry.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test");
    }

    #[test]
    fn test_get_nonexistent_formatter() {
        let registry = HeraldRegistry::new();
        let result = registry.get("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_register_multiple_formatters() {
        let registry = HeraldRegistry::new();

        registry.register("json", Arc::new(MockHerald::new("json")));
        registry.register("markdown", Arc::new(MockHerald::new("markdown")));
        registry.register("table", Arc::new(MockHerald::new("table")));

        assert_eq!(registry.len(), 3);
        assert!(registry.contains("json"));
        assert!(registry.contains("markdown"));
        assert!(registry.contains("table"));
    }

    #[test]
    fn test_list_formatters() {
        let registry = HeraldRegistry::new();

        registry.register("json", Arc::new(MockHerald::new("json")));
        registry.register("markdown", Arc::new(MockHerald::new("markdown")));

        let mut list = registry.list();
        list.sort();

        assert_eq!(list.len(), 2);
        assert!(list.contains(&"json".to_string()));
        assert!(list.contains(&"markdown".to_string()));
    }

    #[test]
    fn test_register_replaces_existing() {
        let registry = HeraldRegistry::new();

        registry.register("test", Arc::new(MockHerald::new("first")));
        registry.register("test", Arc::new(MockHerald::new("second")));

        assert_eq!(registry.len(), 1);
        let herald = registry.get("test").unwrap();
        assert_eq!(herald.name(), "second");
    }

    #[test]
    fn test_contains() {
        let registry = HeraldRegistry::new();

        assert!(!registry.contains("json"));

        registry.register("json", Arc::new(MockHerald::new("json")));

        assert!(registry.contains("json"));
        assert!(!registry.contains("markdown"));
    }

    #[test]
    fn test_registry_is_cloneable() {
        let registry = HeraldRegistry::new();
        registry.register("test", Arc::new(MockHerald::new("test")));

        let cloned = registry.clone();
        assert_eq!(cloned.len(), 1);
        assert!(cloned.contains("test"));
    }

    #[test]
    fn test_registry_thread_safety() {
        use std::thread;

        let registry = Arc::new(HeraldRegistry::new());
        let mut handles = vec![];

        // Test concurrent registration
        for i in 0..10 {
            let registry_clone = Arc::clone(&registry);
            let handle = thread::spawn(move || {
                let name = format!("herald_{}", i);
                registry_clone.register(name.clone(), Arc::new(MockHerald::new(&name)));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(registry.len(), 10);
    }

    #[test]
    fn test_default_registry() {
        let registry = HeraldRegistry::default();
        // Default registry is empty until built-in formatters are implemented
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HeraldRegistry>();
    }
}
