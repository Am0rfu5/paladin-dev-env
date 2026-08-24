//! Arsenal registry service implementation.
//!
//! Provides in-memory storage and management of tool (Armament) registrations.

use crate::core::platform::container::arsenal::Armament;
use async_trait::async_trait;
use paladin_ports::output::arsenal_port::ArsenalRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory implementation of the Arsenal registry.
///
/// `ArsenalRegistryService` maintains a thread-safe, in-memory collection
/// of registered tools. It uses `RwLock` for concurrent read access while
/// ensuring exclusive write access during modifications.
///
/// # Examples
///
/// ```rust
/// use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
/// use paladin_ports::output::arsenal_port::ArsenalRegistry;
/// use paladin::core::platform::container::arsenal::Armament;
/// use serde_json::json;
///
/// # #[tokio::main]
/// # async fn main() {
/// let registry = ArsenalRegistryService::new();
///
/// let tool = Armament {
///     name: "calculator".to_string(),
///     description: "Performs calculations".to_string(),
///     parameters: json!({"type": "object"}),
///     required_params: vec!["operation".to_string()],
/// };
///
/// registry.register(tool).await;
/// assert!(registry.get("calculator").await.is_some());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ArsenalRegistryService {
    /// Thread-safe storage for registered tools
    armaments: Arc<RwLock<HashMap<String, Armament>>>,
}

impl ArsenalRegistryService {
    /// Creates a new empty Arsenal registry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
    ///
    /// let registry = ArsenalRegistryService::new();
    /// ```
    pub fn new() -> Self {
        Self {
            armaments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Returns the number of registered tools.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let registry = ArsenalRegistryService::new();
    /// assert_eq!(registry.count().await, 0);
    /// # }
    /// ```
    pub async fn count(&self) -> usize {
        self.armaments.read().await.len()
    }

    /// Clears all registered tools.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use paladin::application::services::arsenal::arsenal_registry_service::ArsenalRegistryService;
    /// # use paladin_ports::output::arsenal_port::ArsenalRegistry;
    /// # use paladin::core::platform::container::arsenal::Armament;
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let registry = ArsenalRegistryService::new();
    /// let tool = Armament {
    ///     name: "test".to_string(),
    ///     description: "test".to_string(),
    ///     parameters: json!({}),
    ///     required_params: vec![],
    /// };
    /// registry.register(tool).await;
    /// registry.clear().await;
    /// assert_eq!(registry.count().await, 0);
    /// # }
    /// ```
    pub async fn clear(&self) {
        self.armaments.write().await.clear();
    }
}

impl Default for ArsenalRegistryService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ArsenalRegistry for ArsenalRegistryService {
    async fn register(&self, armament: Armament) {
        let mut armaments = self.armaments.write().await;
        armaments.insert(armament.name.clone(), armament);
    }

    async fn unregister(&self, name: &str) -> Option<Armament> {
        let mut armaments = self.armaments.write().await;
        armaments.remove(name)
    }

    async fn get(&self, name: &str) -> Option<Armament> {
        let armaments = self.armaments.read().await;
        armaments.get(name).cloned()
    }

    async fn list(&self) -> Vec<Armament> {
        self.armaments.read().await.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_armament(name: &str) -> Armament {
        Armament {
            name: name.to_string(),
            description: format!("{name} tool"),
            parameters: json!({"type": "object"}),
            required_params: vec![],
        }
    }

    #[tokio::test]
    async fn list_returns_empty_vec_for_a_fresh_registry() {
        let registry = ArsenalRegistryService::new();
        assert!(registry.list().await.is_empty());
    }

    #[tokio::test]
    async fn list_returns_every_registered_armament() {
        let registry = ArsenalRegistryService::new();
        registry.register(make_armament("alpha")).await;
        registry.register(make_armament("beta")).await;
        registry.register(make_armament("gamma")).await;

        let mut names: Vec<String> = registry.list().await.into_iter().map(|a| a.name).collect();
        names.sort();

        assert_eq!(names, vec!["alpha", "beta", "gamma"]);
    }

    #[tokio::test]
    async fn list_excludes_unregistered_armaments() {
        let registry = ArsenalRegistryService::new();
        registry.register(make_armament("keep")).await;
        registry.register(make_armament("drop")).await;

        registry.unregister("drop").await;

        let names: Vec<String> = registry.list().await.into_iter().map(|a| a.name).collect();
        assert_eq!(names, vec!["keep"]);
    }
}
