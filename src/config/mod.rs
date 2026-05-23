// ── Sub-modules ────────────────────────────────────────────────────────────────
#[allow(missing_docs)]
pub mod arsenal;
#[allow(missing_docs)]
pub mod citadel;
#[allow(missing_docs)]
pub mod env_utils;
#[allow(missing_docs)]
pub mod file_storage;
#[allow(missing_docs)]
pub mod herald;
#[allow(missing_docs)]
pub mod notifications;
#[allow(missing_docs)]
pub mod queue;
#[allow(missing_docs)]
pub mod scheduler;
#[allow(missing_docs)]
pub mod setup;
#[allow(missing_docs)]
pub mod user_config;
#[allow(missing_docs)]
pub mod web_server;

// ── Re-exports ─────────────────────────────────────────────────────────────────
// Kept for backwards compatibility — consumers can write
// `use paladin::config::XxxConfig;` without going through a sub-module path.
pub use crate::config::herald::{
    HeraldConfig, JsonHeraldConfig, MarkdownHeraldConfig, TableHeraldConfig,
};
// Vision configuration types live in the paladin-llm crate (Task 5.0)
pub use crate::config::arsenal::{ArsenalConfig, MCPServerConfig};
pub use crate::config::citadel::CitadelConfig;
pub use crate::config::file_storage::FileStorageConfig;
#[cfg(feature = "notifications")]
pub use crate::config::notifications::NotificationConfig;
pub use crate::config::queue::QueueConfig;
pub use crate::config::scheduler::SchedulerConfig;
pub use crate::config::web_server::{MessageServiceSettings, ServerConfig, SourceConfig};
pub use paladin_llm::config::vision::{VisionConfig, VisionProviderConfig, VisionRetryConfig};
// Garrison, Sanctum, RAG and MemoryExtraction config types live in the paladin-memory crate (Task 6.0)
pub use paladin_memory::config::garrison::GarrisonSettings;
pub use paladin_memory::config::rag::{
    MemoryExtractionConfig, MemoryExtractionStrategy, RagConfig,
};
pub use paladin_memory::config::sanctum::{QdrantSanctumConfig, SanctumAdapterType, SanctumConfig};
// LLM configuration types live in the paladin-llm crate (Task 5.0)
pub use paladin_llm::config::llm::{LlmConfig, LlmProviderConfig};

// ── Imports for Settings ───────────────────────────────────────────────────────
use crate::config::env_utils::EnvOverridable;
#[cfg(feature = "s3-storage")]
use crate::infrastructure::adapters::file_storage::minio::MinioConfig;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(feature = "s3-storage")]
use std::time::Duration;

// ── Settings ───────────────────────────────────────────────────────────────────

/// Top-level application configuration struct.
///
/// Load from YAML/TOML config files and environment variable overrides via
/// [`Settings::new`] or [`Settings::load_from_file`].
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub llm_type: String,
    pub llm_url: String,
    pub llm_api_key: String,
    pub server: ServerConfig,
    pub sources: Vec<SourceConfig>,
    pub max_file_size: u64,
    pub message_service: Option<MessageServiceSettings>,
    pub queue: Option<QueueConfig>,
    pub file_storage: Option<FileStorageConfig>,
    #[cfg(feature = "notifications")]
    pub notifications: Option<NotificationConfig>,
    pub garrison: Option<GarrisonSettings>,
    pub sanctum: Option<SanctumConfig>,
    pub rag: Option<RagConfig>,
    pub memory_extraction: Option<MemoryExtractionConfig>,
    pub arsenal: Option<ArsenalConfig>,
    pub citadel: Option<CitadelConfig>,
    pub llm: Option<LlmConfig>,
    pub herald: Option<HeraldConfig>,
    pub vision: Option<VisionConfig>,
    pub scheduler: Option<SchedulerConfig>,
}

impl Settings {
    /// Load settings from config files and `APP_*` environment variables.
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .add_source(File::with_name("config").required(true))
            .add_source(Environment::with_prefix("APP"));

        if let Ok(env) = std::env::var("APP_ENV") {
            builder =
                builder.add_source(File::with_name(&format!("config.{}", env)).required(false));
        }

        builder.build()?.try_deserialize()
    }

    /// Load settings from a TOML file at the given path.
    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;
        let config: Settings = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get queue configuration with environment variable overrides.
    pub fn get_queue_config(&self) -> QueueConfig {
        let mut cfg = self.queue.clone().unwrap_or_default();
        cfg.apply_env_overrides();
        cfg
    }

    /// Get file storage configuration with environment variable overrides.
    pub fn get_file_storage_config(&self) -> FileStorageConfig {
        let mut cfg = self.file_storage.clone().unwrap_or_default();
        cfg.apply_env_overrides();
        cfg
    }

    /// Get notification configuration with environment variable overrides.
    #[cfg(feature = "notifications")]
    pub fn get_notification_config(&self) -> NotificationConfig {
        let mut cfg = self.notifications.clone().unwrap_or_default();
        cfg.apply_env_overrides();
        cfg
    }

    /// Get garrison configuration with environment variable overrides.
    pub fn get_garrison_config(&self) -> GarrisonSettings {
        let mut config = self.garrison.clone().unwrap_or_default();

        if let Ok(garrison_type) = std::env::var("APP_GARRISON_TYPE") {
            config.garrison_type = garrison_type;
        }
        if let Ok(path) = std::env::var("APP_GARRISON_PATH") {
            config.path = Some(path);
        }
        if let Ok(v) = std::env::var("APP_GARRISON_MAX_ENTRIES")
            && let Ok(max_entries) = v.parse::<usize>()
        {
            config.max_entries = max_entries;
        }
        if let Ok(v) = std::env::var("APP_GARRISON_MAX_TOKENS")
            && let Ok(max_tokens) = v.parse::<u32>()
        {
            config.max_tokens = Some(max_tokens);
        }
        if let Ok(tokenizer) = std::env::var("APP_GARRISON_TOKENIZER") {
            config.tokenizer = tokenizer;
        }
        if let Ok(eviction_strategy) = std::env::var("APP_GARRISON_EVICTION_STRATEGY") {
            config.eviction_strategy = eviction_strategy;
        }
        if let Ok(v) = std::env::var("APP_GARRISON_PRESERVE_RECENT_COUNT")
            && let Ok(preserve_recent) = v.parse::<usize>()
        {
            config.preserve_recent_count = preserve_recent;
        }

        config
    }

    /// Get sanctum configuration with environment variable overrides.
    pub fn get_sanctum_config(&self) -> SanctumConfig {
        let mut config = self.sanctum.clone().unwrap_or_default();

        if let Ok(v) = std::env::var("APP_SANCTUM_ENABLED")
            && let Ok(enabled) = v.parse::<bool>()
        {
            config.enabled = enabled;
        }
        if let Ok(adapter_type) = std::env::var("APP_SANCTUM_ADAPTER_TYPE") {
            match adapter_type.to_lowercase().as_str() {
                "in_memory" => config.adapter_type = SanctumAdapterType::InMemory,
                "qdrant" => config.adapter_type = SanctumAdapterType::Qdrant,
                _ => {
                    log::warn!(
                        "Invalid APP_SANCTUM_ADAPTER_TYPE '{}', using default",
                        adapter_type
                    );
                }
            }
        }
        if let Ok(url) = std::env::var("APP_SANCTUM_QDRANT_URL") {
            let mut qdrant = config.qdrant.unwrap_or_default();
            qdrant.url = url;
            config.qdrant = Some(qdrant);
        }
        if let Ok(collection) = std::env::var("APP_SANCTUM_QDRANT_COLLECTION_NAME") {
            let mut qdrant = config.qdrant.unwrap_or_default();
            qdrant.collection_name = collection;
            config.qdrant = Some(qdrant);
        }
        if let Ok(v) = std::env::var("APP_SANCTUM_QDRANT_VECTOR_DIMENSION")
            && let Ok(dimension) = v.parse::<usize>()
        {
            let mut qdrant = config.qdrant.unwrap_or_default();
            qdrant.vector_dimension = dimension;
            config.qdrant = Some(qdrant);
        }

        config
    }

    /// Get citadel configuration with environment variable overrides.
    pub fn get_citadel_config(&self) -> CitadelConfig {
        let mut cfg = self.citadel.clone().unwrap_or_default();
        cfg.apply_env_overrides();
        cfg
    }

    /// Get herald configuration with environment variable overrides.
    pub fn get_herald_config(&self) -> HeraldConfig {
        let mut cfg = self.herald.clone().unwrap_or_default();
        cfg.apply_env_overrides();
        cfg
    }

    /// Create a default [`Herald`](crate::core::platform::container::herald::Herald) instance
    /// from the current herald configuration.
    ///
    /// Reads `herald.default_formatter` and creates the appropriate Herald
    /// implementation.  Returns an error string if the formatter name is
    /// unrecognised.
    pub fn create_default_herald(
        &self,
    ) -> Result<std::sync::Arc<dyn crate::core::platform::container::herald::Herald>, String> {
        use crate::infrastructure::adapters::herald::{JsonHerald, MarkdownHerald, TableHerald};
        use std::sync::Arc;

        let config = self.get_herald_config();

        match config.default_formatter.as_str() {
            "json" => {
                let json_config =
                    crate::infrastructure::adapters::herald::json_herald::JsonHeraldConfig {
                        pretty: config.json.pretty,
                        include_metadata: config.json.include_metadata,
                    };
                let herald = JsonHerald::with_config(json_config);
                Ok(Arc::new(herald))
            }
            "markdown" => {
                let markdown_config = crate::infrastructure::adapters::herald::markdown_herald::MarkdownHeraldConfig {
                    include_colors: config.markdown.include_colors,
                    heading_level: config.markdown.heading_level,
                };
                let herald = MarkdownHerald::with_config(markdown_config);
                Ok(Arc::new(herald))
            }
            "table" => {
                let table_config =
                    crate::infrastructure::adapters::herald::table_herald::TableHeraldConfig {
                        max_column_width: config.table.max_column_width,
                        border_style: config.table.border_style.clone(),
                    };
                let herald = TableHerald::new(table_config);
                Ok(Arc::new(herald))
            }
            other => Err(format!(
                "Unknown formatter '{}'. Valid options: json, markdown, table",
                other
            )),
        }
    }

    /// Get vision configuration with environment variable overrides.
    pub fn get_vision_config(&self) -> VisionConfig {
        let mut config = self.vision.clone().unwrap_or_default();

        if let Ok(v) = std::env::var("APP_VISION_RETRY_MAX_RETRIES")
            && let Ok(max_retries) = v.parse::<u32>()
        {
            config.retry.max_retries = max_retries;
        }
        if let Ok(v) = std::env::var("APP_VISION_RETRY_INITIAL_BACKOFF_MS")
            && let Ok(initial_backoff) = v.parse::<u64>()
        {
            config.retry.initial_backoff_ms = initial_backoff;
        }
        if let Ok(v) = std::env::var("APP_VISION_RETRY_BACKOFF_MULTIPLIER")
            && let Ok(backoff_multiplier) = v.parse::<f64>()
        {
            config.retry.backoff_multiplier = backoff_multiplier;
        }
        if let Ok(v) = std::env::var("APP_VISION_OPENAI_MAX_TOKENS")
            && let Ok(max_tokens) = v.parse::<usize>()
        {
            config.openai.max_tokens = max_tokens;
        }
        if let Ok(v) = std::env::var("APP_VISION_ANTHROPIC_MAX_TOKENS")
            && let Ok(max_tokens) = v.parse::<usize>()
        {
            config.anthropic.max_tokens = max_tokens;
        }

        config
    }

    /// Convert the file-storage config to a [`MinioConfig`].
    #[cfg(feature = "s3-storage")]
    pub fn to_minio_config(&self) -> MinioConfig {
        let fs_config = self.get_file_storage_config();

        MinioConfig {
            endpoint: fs_config.minio_endpoint,
            access_key: fs_config.minio_access_key,
            secret_key: fs_config.minio_secret_key,
            bucket: fs_config.minio_bucket,
            region: fs_config.minio_region,
            secure: fs_config.minio_secure.unwrap_or(false),
            path_style: fs_config.minio_path_style.unwrap_or(true),
            connection_timeout: Duration::from_secs(fs_config.connection_timeout.unwrap_or(30)),
            request_timeout: Duration::from_secs(fs_config.request_timeout.unwrap_or(300)),
            max_idle_conns: fs_config.max_idle_conns.unwrap_or(10),
            max_retries: fs_config.max_idle_conns.unwrap_or(3),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            llm_type: "openai".to_string(),
            llm_url: "https://api.openai.com/v1".to_string(),
            llm_api_key: "".to_string(),
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            sources: Vec::new(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            message_service: Some(MessageServiceSettings {
                max_queue_size: Some(10000),
                default_ttl_seconds: Some(3600),
                enable_persistence: Some(false),
                worker_threads: Some(4),
                retry_attempts: Some(3),
                retry_delay_ms: Some(1000),
            }),
            queue: Some(QueueConfig::default()),
            file_storage: Some(FileStorageConfig::default()),
            #[cfg(feature = "notifications")]
            notifications: Some(NotificationConfig::default()),
            garrison: Some(GarrisonSettings::default()),
            sanctum: Some(SanctumConfig::default()),
            rag: Some(RagConfig::default()),
            memory_extraction: Some(MemoryExtractionConfig::default()),
            arsenal: Some(ArsenalConfig::default()),
            citadel: Some(CitadelConfig::default()),
            llm: Some(LlmConfig::default()),
            herald: Some(HeraldConfig::default()),
            vision: Some(VisionConfig::default()),
            scheduler: Some(SchedulerConfig::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_settings_with_file_storage_config() {
        // Temporarily remove devcontainer APP_MINIO_* env vars so they don't
        // override the values we set explicitly on the Settings struct.
        let minio_vars = [
            "APP_MINIO_ENDPOINT",
            "APP_MINIO_ACCESS_KEY",
            "APP_MINIO_SECRET_KEY",
            "APP_MINIO_BUCKET",
            "APP_MINIO_REGION",
            "APP_MINIO_SECURE",
            "APP_MINIO_PATH_STYLE",
            "APP_MINIO_CONNECTION_TIMEOUT",
            "APP_MINIO_REQUEST_TIMEOUT",
            "APP_MINIO_MAX_IDLE_CONNS",
            "APP_MINIO_MAX_FILE_SIZE",
            "APP_MINIO_ALLOWED_EXTENSIONS",
        ];
        let saved: Vec<(&str, Option<String>)> = minio_vars
            .iter()
            .map(|k| (*k, std::env::var(k).ok()))
            .collect();
        unsafe {
            for k in &minio_vars {
                env::remove_var(k);
            }
        }

        let settings = Settings {
            file_storage: Some(FileStorageConfig {
                minio_endpoint: "custom-minio:9000".to_string(),
                minio_access_key: "custom-access".to_string(),
                minio_secret_key: "custom-secret".to_string(),
                minio_bucket: "custom-bucket".to_string(),
                minio_region: Some("eu-west-1".to_string()),
                minio_secure: Some(true),
                minio_path_style: Some(false),
                connection_timeout: Some(45),
                request_timeout: Some(450),
                max_idle_conns: Some(15),
                max_file_size: Some(50 * 1024 * 1024),
                allowed_extensions: Some(vec!["rs".to_string(), "toml".to_string()]),
            }),
            ..Default::default()
        };

        let config = settings.get_file_storage_config();
        assert_eq!(config.minio_endpoint, "custom-minio:9000");
        assert_eq!(config.minio_access_key, "custom-access");
        assert_eq!(config.minio_secret_key, "custom-secret");
        assert_eq!(config.minio_bucket, "custom-bucket");
        assert_eq!(config.minio_region, Some("eu-west-1".to_string()));

        // Restore env vars
        unsafe {
            for (k, v) in saved {
                match v {
                    Some(val) => env::set_var(k, val),
                    None => env::remove_var(k),
                }
            }
        }
    }

    #[test]
    #[cfg(feature = "s3-storage")]
    fn test_to_minio_config_conversion() {
        let settings = Settings::default();
        let minio_config = settings.to_minio_config();

        assert_eq!(minio_config.endpoint, "localhost:9000");
        assert_eq!(minio_config.access_key, "minioadmin");
        assert_eq!(minio_config.secret_key, "minioadmin");
        assert_eq!(minio_config.bucket, "paladin-files");
        assert!(!minio_config.secure);
        assert!(minio_config.path_style);
        assert_eq!(minio_config.connection_timeout, Duration::from_secs(30));
        assert_eq!(minio_config.request_timeout, Duration::from_secs(300));
        assert_eq!(minio_config.max_idle_conns, 10);
    }

    #[test]
    #[serial]
    fn test_garrison_config_defaults() {
        let settings = Settings::default();
        let garrison_config = settings.get_garrison_config();

        assert_eq!(garrison_config.garrison_type, "in_memory");
        assert_eq!(garrison_config.max_entries, 100);
        assert_eq!(garrison_config.max_tokens, Some(4000));
        assert_eq!(garrison_config.tokenizer, "gpt-4");
        assert_eq!(garrison_config.eviction_strategy, "importance_based");
        assert_eq!(garrison_config.preserve_recent_count, 10);
        assert!(garrison_config.path.is_none());
    }

    #[test]
    #[serial]
    fn test_garrison_config_with_overrides() {
        let settings = Settings {
            garrison: Some(GarrisonSettings {
                garrison_type: "sqlite".to_string(),
                path: Some("./test_garrison.db".to_string()),
                max_entries: 200,
                max_tokens: Some(8000),
                tokenizer: "gpt-3.5-turbo".to_string(),
                eviction_strategy: "fifo".to_string(),
                preserve_recent_count: 20,
            }),
            ..Default::default()
        };

        let config = settings.get_garrison_config();
        assert_eq!(config.garrison_type, "sqlite");
        assert_eq!(config.path, Some("./test_garrison.db".to_string()));
        assert_eq!(config.max_entries, 200);
        assert_eq!(config.max_tokens, Some(8000));
        assert_eq!(config.tokenizer, "gpt-3.5-turbo");
        assert_eq!(config.eviction_strategy, "fifo");
        assert_eq!(config.preserve_recent_count, 20);
    }

    #[test]
    #[serial]
    fn test_garrison_config_env_overrides() {
        unsafe {
            env::set_var("APP_GARRISON_TYPE", "sqlite");
            env::set_var("APP_GARRISON_PATH", "./env_garrison.db");
            env::set_var("APP_GARRISON_MAX_ENTRIES", "500");
            env::set_var("APP_GARRISON_MAX_TOKENS", "16000");
            env::set_var("APP_GARRISON_TOKENIZER", "claude-v1");
            env::set_var("APP_GARRISON_EVICTION_STRATEGY", "sliding_window");
            env::set_var("APP_GARRISON_PRESERVE_RECENT_COUNT", "50");
        }

        let settings = Settings::default();
        let config = settings.get_garrison_config();

        assert_eq!(config.garrison_type, "sqlite");
        assert_eq!(config.path, Some("./env_garrison.db".to_string()));
        assert_eq!(config.max_entries, 500);
        assert_eq!(config.max_tokens, Some(16000));
        assert_eq!(config.tokenizer, "claude-v1");
        assert_eq!(config.eviction_strategy, "sliding_window");
        assert_eq!(config.preserve_recent_count, 50);

        unsafe {
            env::remove_var("APP_GARRISON_TYPE");
            env::remove_var("APP_GARRISON_PATH");
            env::remove_var("APP_GARRISON_MAX_ENTRIES");
            env::remove_var("APP_GARRISON_MAX_TOKENS");
            env::remove_var("APP_GARRISON_TOKENIZER");
            env::remove_var("APP_GARRISON_EVICTION_STRATEGY");
            env::remove_var("APP_GARRISON_PRESERVE_RECENT_COUNT");
        }
    }

    #[test]
    fn test_create_default_herald_json() {
        let settings = Settings {
            herald: Some(HeraldConfig {
                default_formatter: "json".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "json");
        assert_eq!(herald.mime_type(), "application/json");
    }

    #[test]
    fn test_create_default_herald_markdown() {
        let settings = Settings {
            herald: Some(HeraldConfig {
                default_formatter: "markdown".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "markdown");
        assert_eq!(herald.mime_type(), "text/markdown");
    }

    #[test]
    fn test_create_default_herald_table() {
        let settings = Settings {
            herald: Some(HeraldConfig {
                default_formatter: "table".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "table");
        assert_eq!(herald.mime_type(), "text/plain");
    }

    #[test]
    fn test_create_default_herald_invalid_formatter() {
        let settings = Settings {
            herald: Some(HeraldConfig {
                default_formatter: "invalid".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let herald = settings.create_default_herald();
        assert!(herald.is_err());
        let err_msg = herald.err().unwrap();
        assert!(err_msg.contains("Unknown formatter 'invalid'"));
    }

    #[test]
    fn test_create_default_herald_with_custom_config() {
        let settings = Settings {
            herald: Some(HeraldConfig {
                default_formatter: "json".to_string(),
                json: JsonHeraldConfig {
                    pretty: false,
                    include_metadata: false,
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        // Config is passed correctly to JsonHerald (verified via unit tests)
    }

    #[test]
    #[serial]
    fn test_get_sanctum_config_env_enabled() {
        unsafe {
            env::set_var("APP_SANCTUM_ENABLED", "true");
        }

        let settings = Settings {
            sanctum: Some(SanctumConfig {
                enabled: false,
                adapter_type: SanctumAdapterType::InMemory,
                qdrant: None,
            }),
            ..Default::default()
        };

        let config = settings.get_sanctum_config();
        assert!(config.enabled);

        unsafe {
            env::remove_var("APP_SANCTUM_ENABLED");
        }
    }

    #[test]
    #[serial]
    fn test_get_sanctum_config_env_adapter_type() {
        unsafe {
            env::set_var("APP_SANCTUM_ADAPTER_TYPE", "qdrant");
        }

        let settings = Settings {
            sanctum: Some(SanctumConfig {
                enabled: true,
                adapter_type: SanctumAdapterType::InMemory,
                qdrant: None,
            }),
            ..Default::default()
        };

        let config = settings.get_sanctum_config();
        assert_eq!(config.adapter_type, SanctumAdapterType::Qdrant);

        unsafe {
            env::remove_var("APP_SANCTUM_ADAPTER_TYPE");
        }
    }

    #[test]
    #[serial]
    fn test_get_sanctum_config_env_qdrant_url() {
        unsafe {
            env::set_var("APP_SANCTUM_QDRANT_URL", "http://custom:6334");
        }

        let settings = Settings {
            sanctum: Some(SanctumConfig {
                enabled: true,
                adapter_type: SanctumAdapterType::Qdrant,
                qdrant: Some(QdrantSanctumConfig {
                    url: "http://localhost:6334".to_string(),
                    collection_name: "test".to_string(),
                    vector_dimension: 1536,
                }),
            }),
            ..Default::default()
        };

        let config = settings.get_sanctum_config();
        assert_eq!(config.qdrant.unwrap().url, "http://custom:6334");

        unsafe {
            env::remove_var("APP_SANCTUM_QDRANT_URL");
        }
    }

    #[test]
    #[serial]
    fn test_get_vision_config_defaults() {
        let settings = Settings::default();
        let config = settings.get_vision_config();

        assert_eq!(config.retry.max_retries, 3);
        assert_eq!(config.retry.initial_backoff_ms, 1000);
        assert_eq!(config.retry.backoff_multiplier, 2.0);
        assert_eq!(config.openai.max_tokens, 4096);
        assert_eq!(config.anthropic.max_tokens, 4096);
    }

    #[test]
    #[serial]
    fn test_get_vision_config_env_overrides() {
        unsafe {
            env::set_var("APP_VISION_RETRY_MAX_RETRIES", "5");
            env::set_var("APP_VISION_RETRY_INITIAL_BACKOFF_MS", "2000");
            env::set_var("APP_VISION_RETRY_BACKOFF_MULTIPLIER", "3.0");
            env::set_var("APP_VISION_OPENAI_MAX_TOKENS", "8192");
            env::set_var("APP_VISION_ANTHROPIC_MAX_TOKENS", "8192");
        }

        let settings = Settings::default();
        let config = settings.get_vision_config();

        assert_eq!(config.retry.max_retries, 5);
        assert_eq!(config.retry.initial_backoff_ms, 2000);
        assert_eq!(config.retry.backoff_multiplier, 3.0);
        assert_eq!(config.openai.max_tokens, 8192);
        assert_eq!(config.anthropic.max_tokens, 8192);

        unsafe {
            env::remove_var("APP_VISION_RETRY_MAX_RETRIES");
            env::remove_var("APP_VISION_RETRY_INITIAL_BACKOFF_MS");
            env::remove_var("APP_VISION_RETRY_BACKOFF_MULTIPLIER");
            env::remove_var("APP_VISION_OPENAI_MAX_TOKENS");
            env::remove_var("APP_VISION_ANTHROPIC_MAX_TOKENS");
        }
    }

    #[test]
    fn test_settings_with_vision_config() {
        let settings = Settings {
            vision: Some(VisionConfig {
                retry: VisionRetryConfig {
                    max_retries: 5,
                    initial_backoff_ms: 500,
                    backoff_multiplier: 1.5,
                },
                openai: VisionProviderConfig { max_tokens: 2048 },
                anthropic: VisionProviderConfig { max_tokens: 2048 },
            }),
            ..Default::default()
        };

        let config = settings.get_vision_config();
        assert_eq!(config.retry.max_retries, 5);
        assert_eq!(config.retry.initial_backoff_ms, 500);
        assert_eq!(config.retry.backoff_multiplier, 1.5);
        assert_eq!(config.openai.max_tokens, 2048);
        assert_eq!(config.anthropic.max_tokens, 2048);
    }
}
