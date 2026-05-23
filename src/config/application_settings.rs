#[cfg(feature = "s3-storage")]
use crate::infrastructure::adapters::file_storage::minio::MinioConfig;

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(feature = "s3-storage")]
use std::time::Duration;

// Facade config re-exports (moved to dedicated modules in Task 4.0)
pub use crate::config::herald::{
    HeraldConfig, JsonHeraldConfig, MarkdownHeraldConfig, TableHeraldConfig,
};

// Vision configuration types moved to paladin-llm crate (Task 5.0)
pub use paladin_llm::config::vision::{VisionConfig, VisionProviderConfig, VisionRetryConfig};

pub use crate::config::arsenal::{ArsenalConfig, MCPServerConfig};
pub use crate::config::citadel::CitadelConfig;
pub use crate::config::file_storage::FileStorageConfig;
#[cfg(feature = "notifications")]
pub use crate::config::notifications::NotificationConfig;
pub use crate::config::queue::QueueConfig;
pub use crate::config::scheduler::SchedulerConfig;
pub use crate::config::web_server::{MessageServiceSettings, ServerConfig, SourceConfig};

// Garrison, Sanctum, RAG, and MemoryExtraction config types moved to paladin-memory crate (Task 6.0)
pub use paladin_memory::config::garrison::GarrisonSettings;
pub use paladin_memory::config::rag::{
    MemoryExtractionConfig, MemoryExtractionStrategy, RagConfig,
};
pub use paladin_memory::config::sanctum::{QdrantSanctumConfig, SanctumAdapterType, SanctumConfig};

// LLM configuration types moved to paladin-llm crate (Task 5.0)
pub use paladin_llm::config::llm::{LlmConfig, LlmProviderConfig};

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

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;
        let config: Settings = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get queue configuration with environment variable overrides
    pub fn get_queue_config(&self) -> QueueConfig {
        let mut config = self.queue.clone().unwrap_or_default();

        // Override with environment variables if present
        if let Ok(host) = std::env::var("APP_REDIS_HOST") {
            config.redis_host = host;
        }

        if let Ok(port_str) = std::env::var("APP_REDIS_PORT")
            && let Ok(port) = port_str.parse::<u16>()
        {
            config.redis_port = port;
        }

        if let Ok(password) = std::env::var("APP_REDIS_PASSWORD") {
            config.redis_password = Some(password);
        }

        if let Ok(db_str) = std::env::var("APP_REDIS_DB")
            && let Ok(db) = db_str.parse::<u8>()
        {
            config.redis_db = db;
        }

        if let Ok(timeout_str) = std::env::var("APP_REDIS_CONNECTION_TIMEOUT")
            && let Ok(timeout) = timeout_str.parse::<u64>()
        {
            config.connection_timeout = Some(timeout);
        }

        if let Ok(prefix) = std::env::var("APP_REDIS_KEY_PREFIX") {
            config.key_prefix = Some(prefix);
        }

        if let Ok(retries_str) = std::env::var("APP_REDIS_MAX_RETRIES")
            && let Ok(retries) = retries_str.parse::<u32>()
        {
            config.max_retries = Some(retries);
        }

        if let Ok(priority_str) = std::env::var("APP_REDIS_ENABLE_PRIORITY_QUEUES")
            && let Ok(enable) = priority_str.parse::<bool>()
        {
            config.enable_priority_queues = Some(enable);
        }

        config
    }

    /// Get file storage configuration with environment variable overrides
    pub fn get_file_storage_config(&self) -> FileStorageConfig {
        let mut config = self.file_storage.clone().unwrap_or_default();

        // Override with environment variables if present
        if let Ok(endpoint) = std::env::var("APP_MINIO_ENDPOINT") {
            config.minio_endpoint = endpoint;
        }

        if let Ok(access_key) = std::env::var("APP_MINIO_ACCESS_KEY") {
            config.minio_access_key = access_key;
        }

        if let Ok(secret_key) = std::env::var("APP_MINIO_SECRET_KEY") {
            config.minio_secret_key = secret_key;
        }

        if let Ok(bucket) = std::env::var("APP_MINIO_BUCKET") {
            config.minio_bucket = bucket;
        }

        if let Ok(region) = std::env::var("APP_MINIO_REGION") {
            config.minio_region = Some(region);
        }

        if let Ok(secure_str) = std::env::var("APP_MINIO_SECURE")
            && let Ok(secure) = secure_str.parse::<bool>()
        {
            config.minio_secure = Some(secure);
        }

        if let Ok(path_style_str) = std::env::var("APP_MINIO_PATH_STYLE")
            && let Ok(path_style) = path_style_str.parse::<bool>()
        {
            config.minio_path_style = Some(path_style);
        }

        if let Ok(timeout_str) = std::env::var("APP_MINIO_CONNECTION_TIMEOUT")
            && let Ok(timeout) = timeout_str.parse::<u64>()
        {
            config.connection_timeout = Some(timeout);
        }

        if let Ok(request_timeout_str) = std::env::var("APP_MINIO_REQUEST_TIMEOUT")
            && let Ok(timeout) = request_timeout_str.parse::<u64>()
        {
            config.request_timeout = Some(timeout);
        }

        if let Ok(max_conns_str) = std::env::var("APP_MINIO_MAX_IDLE_CONNS")
            && let Ok(max_conns) = max_conns_str.parse::<u32>()
        {
            config.max_idle_conns = Some(max_conns);
        }

        if let Ok(max_size_str) = std::env::var("APP_MINIO_MAX_FILE_SIZE")
            && let Ok(max_size) = max_size_str.parse::<u64>()
        {
            config.max_file_size = Some(max_size);
        }

        if let Ok(extensions_str) = std::env::var("APP_MINIO_ALLOWED_EXTENSIONS") {
            let extensions: Vec<String> = extensions_str
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect();
            if !extensions.is_empty() {
                config.allowed_extensions = Some(extensions);
            }
        }

        config
    }

    /// Get notification configuration with environment variable overrides
    #[cfg(feature = "notifications")]
    pub fn get_notification_config(&self) -> NotificationConfig {
        let mut config = self.notifications.clone().unwrap_or_default();

        // Override with environment variables if present
        if let Ok(enabled_str) = std::env::var("APP_NOTIFICATIONS_ENABLED")
            && let Ok(enabled) = enabled_str.parse::<bool>()
        {
            config.enabled = enabled;
        }

        // Email configuration overrides
        if let Some(ref mut email_config) = config.email {
            if let Ok(smtp_host) = std::env::var("APP_EMAIL_SMTP_HOST") {
                email_config.smtp_host = smtp_host;
            }
            if let Ok(smtp_port_str) = std::env::var("APP_EMAIL_SMTP_PORT")
                && let Ok(smtp_port) = smtp_port_str.parse::<u16>()
            {
                email_config.smtp_port = smtp_port;
            }
            if let Ok(username) = std::env::var("APP_EMAIL_USERNAME") {
                email_config.username = username;
            }
            if let Ok(password) = std::env::var("APP_EMAIL_PASSWORD") {
                email_config.password = password;
            }
            if let Ok(from_address) = std::env::var("APP_EMAIL_FROM_ADDRESS") {
                email_config.from_address = from_address;
            }
            if let Ok(from_name) = std::env::var("APP_EMAIL_FROM_NAME") {
                email_config.from_name = Some(from_name);
            }
            if let Ok(use_tls_str) = std::env::var("APP_EMAIL_USE_TLS")
                && let Ok(use_tls) = use_tls_str.parse::<bool>()
            {
                email_config.use_tls = use_tls;
            }
        }

        config
    }

    /// Get garrison configuration with environment variable overrides
    pub fn get_garrison_config(&self) -> GarrisonSettings {
        let mut config = self.garrison.clone().unwrap_or_default();

        // Override with environment variables if present
        if let Ok(garrison_type) = std::env::var("APP_GARRISON_TYPE") {
            config.garrison_type = garrison_type;
        }

        if let Ok(path) = std::env::var("APP_GARRISON_PATH") {
            config.path = Some(path);
        }

        if let Ok(max_entries_str) = std::env::var("APP_GARRISON_MAX_ENTRIES")
            && let Ok(max_entries) = max_entries_str.parse::<usize>()
        {
            config.max_entries = max_entries;
        }

        if let Ok(max_tokens_str) = std::env::var("APP_GARRISON_MAX_TOKENS")
            && let Ok(max_tokens) = max_tokens_str.parse::<u32>()
        {
            config.max_tokens = Some(max_tokens);
        }

        if let Ok(tokenizer) = std::env::var("APP_GARRISON_TOKENIZER") {
            config.tokenizer = tokenizer;
        }

        if let Ok(eviction_strategy) = std::env::var("APP_GARRISON_EVICTION_STRATEGY") {
            config.eviction_strategy = eviction_strategy;
        }

        if let Ok(preserve_recent_str) = std::env::var("APP_GARRISON_PRESERVE_RECENT_COUNT")
            && let Ok(preserve_recent) = preserve_recent_str.parse::<usize>()
        {
            config.preserve_recent_count = preserve_recent;
        }

        config
    }

    /// Get sanctum configuration with environment variable overrides
    pub fn get_sanctum_config(&self) -> SanctumConfig {
        let mut config = self.sanctum.clone().unwrap_or_default();

        // Override with environment variables if present
        if let Ok(enabled_str) = std::env::var("APP_SANCTUM_ENABLED")
            && let Ok(enabled) = enabled_str.parse::<bool>()
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

        // Qdrant-specific overrides
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

        if let Ok(dimension_str) = std::env::var("APP_SANCTUM_QDRANT_VECTOR_DIMENSION")
            && let Ok(dimension) = dimension_str.parse::<usize>()
        {
            let mut qdrant = config.qdrant.unwrap_or_default();
            qdrant.vector_dimension = dimension;
            config.qdrant = Some(qdrant);
        }

        config
    }

    /// Get citadel configuration with environment variable overrides
    pub fn get_citadel_config(&self) -> CitadelConfig {
        let mut config = self.citadel.clone().unwrap_or_default();

        // Override with environment variables if present
        if let Ok(enabled_str) = std::env::var("APP_CITADEL_ENABLED")
            && let Ok(enabled) = enabled_str.parse::<bool>()
        {
            config.enabled = enabled;
        }

        if let Ok(state_dir) = std::env::var("APP_CITADEL_STATE_DIR") {
            config.state_dir = state_dir;
        }

        if let Ok(autosave_str) = std::env::var("APP_CITADEL_AUTOSAVE_ENABLED")
            && let Ok(autosave) = autosave_str.parse::<bool>()
        {
            config.autosave_enabled = autosave;
        }

        if let Ok(cleanup_str) = std::env::var("APP_CITADEL_CLEANUP_ENABLED")
            && let Ok(cleanup) = cleanup_str.parse::<bool>()
        {
            config.cleanup_enabled = cleanup;
        }

        if let Ok(max_age_str) = std::env::var("APP_CITADEL_MAX_STATE_AGE_DAYS")
            && let Ok(max_age) = max_age_str.parse::<u32>()
        {
            config.max_state_age_days = Some(max_age);
        }

        config
    }

    /// Get herald configuration with environment variable overrides
    pub fn get_herald_config(&self) -> HeraldConfig {
        let mut config = self.herald.clone().unwrap_or_default();

        // Override default formatter with environment variable if present
        if let Ok(default_formatter) = std::env::var("HERALD_DEFAULT_FORMATTER") {
            config.default_formatter = default_formatter;
        }
        if let Ok(default_formatter) = std::env::var("APP_HERALD_DEFAULT_FORMATTER") {
            config.default_formatter = default_formatter;
        }

        // JSON configuration overrides
        if let Ok(pretty_str) = std::env::var("APP_HERALD_JSON_PRETTY")
            && let Ok(pretty) = pretty_str.parse::<bool>()
        {
            config.json.pretty = pretty;
        }
        if let Ok(include_metadata_str) = std::env::var("APP_HERALD_JSON_INCLUDE_METADATA")
            && let Ok(include_metadata) = include_metadata_str.parse::<bool>()
        {
            config.json.include_metadata = include_metadata;
        }

        // Markdown configuration overrides
        if let Ok(include_colors_str) = std::env::var("APP_HERALD_MARKDOWN_INCLUDE_COLORS")
            && let Ok(include_colors) = include_colors_str.parse::<bool>()
        {
            config.markdown.include_colors = include_colors;
        }
        if let Ok(heading_level_str) = std::env::var("APP_HERALD_MARKDOWN_HEADING_LEVEL")
            && let Ok(heading_level) = heading_level_str.parse::<u8>()
            && (1..=6).contains(&heading_level)
        {
            config.markdown.heading_level = heading_level;
        }

        // Table configuration overrides
        if let Ok(max_column_width_str) = std::env::var("APP_HERALD_TABLE_MAX_COLUMN_WIDTH")
            && let Ok(max_column_width) = max_column_width_str.parse::<usize>()
            && max_column_width > 0
        {
            config.table.max_column_width = max_column_width;
        }
        if let Ok(border_style) = std::env::var("APP_HERALD_TABLE_BORDER_STYLE") {
            config.table.border_style = border_style;
        }

        config
    }

    /// Create a default Herald instance from configuration
    ///
    /// This method reads the `herald.default_formatter` setting and creates
    /// the appropriate Herald implementation with its specific configuration.
    ///
    /// # Returns
    ///
    /// Returns `Arc<dyn Herald>` containing the configured default formatter,
    /// or an error if the formatter name is invalid or cannot be created.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let settings = Settings::new()?;
    /// let herald = settings.create_default_herald()?;
    /// let formatted = herald.format_paladin_result(&result)?;
    /// ```
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

    /// Get vision configuration with environment variable overrides
    pub fn get_vision_config(&self) -> VisionConfig {
        let mut config = self.vision.clone().unwrap_or_default();

        // Retry configuration overrides
        if let Ok(max_retries_str) = std::env::var("APP_VISION_RETRY_MAX_RETRIES")
            && let Ok(max_retries) = max_retries_str.parse::<u32>()
        {
            config.retry.max_retries = max_retries;
        }

        if let Ok(initial_backoff_str) = std::env::var("APP_VISION_RETRY_INITIAL_BACKOFF_MS")
            && let Ok(initial_backoff) = initial_backoff_str.parse::<u64>()
        {
            config.retry.initial_backoff_ms = initial_backoff;
        }

        if let Ok(backoff_multiplier_str) = std::env::var("APP_VISION_RETRY_BACKOFF_MULTIPLIER")
            && let Ok(backoff_multiplier) = backoff_multiplier_str.parse::<f64>()
        {
            config.retry.backoff_multiplier = backoff_multiplier;
        }

        // OpenAI vision configuration overrides
        if let Ok(max_tokens_str) = std::env::var("APP_VISION_OPENAI_MAX_TOKENS")
            && let Ok(max_tokens) = max_tokens_str.parse::<usize>()
        {
            config.openai.max_tokens = max_tokens;
        }

        // Anthropic vision configuration overrides
        if let Ok(max_tokens_str) = std::env::var("APP_VISION_ANTHROPIC_MAX_TOKENS")
            && let Ok(max_tokens) = max_tokens_str.parse::<usize>()
        {
            config.anthropic.max_tokens = max_tokens;
        }

        config
    }

    #[cfg(feature = "s3-storage")]
    /// Convert FileStorageConfig to MinioConfig
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
                max_file_size: Some(50 * 1024 * 1024), // 50MB
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
        use std::env;

        // Set environment variables
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

        // Cleanup
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

    // Garrison struct-level tests moved to crates/paladin-memory/src/config/garrison.rs (Task 6.0)

    // LLM config tests moved to crates/paladin-llm/src/config/llm.rs (Task 5.0)

    // Herald tests moved to src/config/herald.rs (Task 4.0)

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

    // Sanctum struct-level tests moved to crates/paladin-memory/src/config/sanctum.rs (Task 6.0)

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

    // Vision struct-level tests moved to crates/paladin-llm/src/config/vision.rs (Task 5.0)

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
        use std::env;

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
