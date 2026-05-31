//! Herald Registry for managing output formatters
//!
//! The Herald Registry provides centralized management of output formatters,
//! allowing registration, retrieval, and listing of available Herald implementations.
//! The registry is thread-safe and supports concurrent access.
//!
//! # Auto-Registration
//!
//! When using `HeraldRegistry::default()`, three built-in formatters are automatically
//! registered and ready to use:
//!
//! - **"json"** - Structured JSON output with metadata
//! - **"markdown"** - Human-readable Markdown with colors and formatting
//! - **"table"** - Formatted tables with borders and alignment
//!
//! # Examples
//!
//! ## Zero-Config Pattern (Recommended)
//!
//! ```rust,ignore
//! use paladin::application::services::herald::HeraldRegistry;
//!
//! // Built-in formatters are pre-registered
//! let registry = HeraldRegistry::default();
//!
//! // Immediately use formatters without registration
//! let json_herald = registry.get("json").unwrap();
//! let markdown_herald = registry.get("markdown").unwrap();
//! let table_herald = registry.get("table").unwrap();
//!
//! // List all available formatters
//! let formatters = registry.list();
//! assert_eq!(formatters.len(), 3);
//! ```
//!
//! ## Manual Registration
//!
//! ```rust,ignore
//! use paladin::application::services::herald::HeraldRegistry;
//! use paladin::infrastructure::adapters::herald::JsonHerald;
//! use std::sync::Arc;
//!
//! let registry = HeraldRegistry::new();  // Start with empty registry
//! registry.register("json", Arc::new(JsonHerald::new()));
//!
//! let herald = registry.get("json").unwrap();
//! ```
//!
//! ## Custom Formatters
//!
//! ```rust,ignore
//! use paladin::application::services::herald::HeraldRegistry;
//! use paladin::core::platform::container::herald::Herald;
//! use std::sync::Arc;
//!
//! // Start with built-in formatters
//! let registry = HeraldRegistry::default();
//!
//! // Add custom formatter
//! registry.register("xml", Arc::new(MyXmlHerald::new()));
//! registry.register("csv", Arc::new(MyCsvHerald::new()));
//!
//! // Override built-in formatter with custom config
//! let custom_json = JsonHerald::with_config(my_config);
//! registry.register("json", Arc::new(custom_json));
//! ```

use crate::core::platform::container::herald::Herald;
use crate::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};
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
    /// The default registry automatically includes three built-in formatters:
    /// - **"json"** - JSON formatter with default configuration (pretty-printed, includes metadata)
    /// - **"markdown"** - Markdown formatter with default configuration (colors enabled, H2 headings)
    /// - **"table"** - Table formatter with default configuration (rounded borders, auto-width)
    ///
    /// These formatters are immediately available for use without manual registration.
    /// Custom formatters can still be registered using the `register()` method,
    /// and built-in formatters can be overridden by registering with the same name.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Built-in formatters are pre-registered
    /// let registry = HeraldRegistry::default();
    ///
    /// // Immediately use built-in formatters
    /// let json_herald = registry.get("json").unwrap();
    /// let formatted = json_herald.format_paladin_result(&result)?;
    ///
    /// // Can still add custom formatters
    /// registry.register("custom", Arc::new(MyCustomHerald::new()));
    ///
    /// // Can override built-in formatters
    /// registry.register("json", Arc::new(JsonHerald::with_config(custom_config)));
    /// ```
    fn default() -> Self {
        let registry = Self::new();

        // Auto-register built-in formatters with default configurations
        registry.register("json", Arc::new(JsonHerald::new()));
        registry.register("markdown", Arc::new(MarkdownHerald::new()));
        registry.register("table", Arc::new(TableHerald::default()));

        registry
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

        // Default registry should have 3 built-in formatters
        assert_eq!(registry.len(), 3);
        assert!(!registry.is_empty());

        // Verify all built-in formatters are registered
        assert!(registry.contains("json"));
        assert!(registry.contains("markdown"));
        assert!(registry.contains("table"));

        // Verify formatters are retrievable
        assert!(registry.get("json").is_some());
        assert!(registry.get("markdown").is_some());
        assert!(registry.get("table").is_some());

        // Verify formatter names match
        assert_eq!(registry.get("json").unwrap().name(), "json");
        assert_eq!(registry.get("markdown").unwrap().name(), "markdown");
        assert_eq!(registry.get("table").unwrap().name(), "table");
    }

    #[test]
    fn test_default_registry_can_add_custom_formatters() {
        let registry = HeraldRegistry::default();

        // Start with 3 built-in formatters
        assert_eq!(registry.len(), 3);

        // Add custom formatter
        registry.register("custom", Arc::new(MockHerald::new("custom")));

        // Should now have 4 formatters
        assert_eq!(registry.len(), 4);
        assert!(registry.contains("custom"));

        // Built-in formatters should still be present
        assert!(registry.contains("json"));
        assert!(registry.contains("markdown"));
        assert!(registry.contains("table"));
    }

    #[test]
    fn test_default_registry_can_override_builtin_formatters() {
        let registry = HeraldRegistry::default();

        // Verify json formatter is built-in
        let original_json = registry.get("json").unwrap();
        assert_eq!(original_json.name(), "json");

        // Override json formatter with mock
        registry.register("json", Arc::new(MockHerald::new("custom-json")));

        // Should still have 3 formatters (replaced, not added)
        assert_eq!(registry.len(), 3);

        // Verify formatter was replaced
        let new_json = registry.get("json").unwrap();
        assert_eq!(new_json.name(), "custom-json");

        // Other built-in formatters should still be present
        assert_eq!(registry.get("markdown").unwrap().name(), "markdown");
        assert_eq!(registry.get("table").unwrap().name(), "table");
    }

    #[test]
    fn test_new_vs_default_registry() {
        let new_registry = HeraldRegistry::new();
        let default_registry = HeraldRegistry::default();

        // new() creates empty registry
        assert!(new_registry.is_empty());
        assert_eq!(new_registry.len(), 0);

        // default() creates registry with built-in formatters
        assert!(!default_registry.is_empty());
        assert_eq!(default_registry.len(), 3);

        // Can still register formatters in new() registry
        new_registry.register("json", Arc::new(MockHerald::new("json")));
        assert_eq!(new_registry.len(), 1);
    }

    #[test]
    fn test_registry_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HeraldRegistry>();
    }
}
