use crate::core::platform::container::garrison::EvictionStrategy;
use crate::infrastructure::adapters::file_storage::minio::MinioConfig;
use crate::infrastructure::adapters::notifications::{EmailAdapterConfig, SystemAdapterConfig};
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

/// Configuration for JSON Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonHeraldConfig {
    /// Enable pretty-printing (formatted with indentation)
    pub pretty: bool,
    /// Include metadata fields in output
    pub include_metadata: bool,
}

impl Default for JsonHeraldConfig {
    fn default() -> Self {
        Self {
            pretty: true,
            include_metadata: true,
        }
    }
}

/// Configuration for Markdown Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownHeraldConfig {
    /// Enable ANSI color codes for terminal output
    pub include_colors: bool,
    /// Heading level for main sections (1-6)
    pub heading_level: u8,
}

impl Default for MarkdownHeraldConfig {
    fn default() -> Self {
        Self {
            include_colors: true,
            heading_level: 2,
        }
    }
}

/// Configuration for Table Herald formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableHeraldConfig {
    /// Maximum width for table columns (characters)
    pub max_column_width: usize,
    /// Border style preset: "ascii", "rounded", "modern", "sharp", "none"
    pub border_style: String,
}

impl Default for TableHeraldConfig {
    fn default() -> Self {
        Self {
            max_column_width: 60,
            border_style: "rounded".to_string(),
        }
    }
}

/// Configuration for Herald output formatting system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeraldConfig {
    /// Default formatter to use: "json", "markdown", "table"
    pub default_formatter: String,
    /// JSON formatter configuration
    pub json: JsonHeraldConfig,
    /// Markdown formatter configuration
    pub markdown: MarkdownHeraldConfig,
    /// Table formatter configuration
    pub table: TableHeraldConfig,
}

impl Default for HeraldConfig {
    fn default() -> Self {
        Self {
            default_formatter: "json".to_string(),
            json: JsonHeraldConfig::default(),
            markdown: MarkdownHeraldConfig::default(),
            table: TableHeraldConfig::default(),
        }
    }
}

impl HeraldConfig {
    /// Validates herald configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate default_formatter
        let valid_formatters = ["json", "markdown", "table"];
        if !valid_formatters.contains(&self.default_formatter.as_str()) {
            return Err(format!(
                "Invalid default_formatter '{}'. Must be one of: {}",
                self.default_formatter,
                valid_formatters.join(", ")
            ));
        }

        // Validate markdown heading_level
        if !(1..=6).contains(&self.markdown.heading_level) {
            return Err(format!(
                "Invalid markdown heading_level {}. Must be between 1 and 6",
                self.markdown.heading_level
            ));
        }

        // Validate table max_column_width
        if self.table.max_column_width == 0 {
            return Err("table max_column_width must be greater than 0".to_string());
        }

        Ok(())
    }
}

/// Configuration for a single MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    /// Name/identifier for the server
    pub name: String,
    /// Type of server: "stdio" or "sse"
    pub server_type: String,
    /// Command to execute (for STDIO servers)
    pub command: Option<String>,
    /// Arguments for the command (for STDIO servers)
    pub args: Option<Vec<String>>,
    /// HTTP endpoint URL (for SSE servers)
    pub endpoint: Option<String>,
}

/// Configuration for Arsenal tool system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArsenalConfig {
    /// Default timeout for tool execution in seconds
    pub default_timeout_seconds: u64,
    /// Maximum number of concurrent tool executions
    pub max_concurrent_tools: usize,
    /// List of MCP servers to connect to
    pub mcp_servers: Vec<MCPServerConfig>,
}

/// Configuration for Citadel state persistence system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitadelConfig {
    /// Enable or disable state persistence
    pub enabled: bool,
    /// Directory where state files will be saved
    pub state_dir: String,
    /// Enable automatic saving after Paladin execution
    pub autosave_enabled: bool,
    /// Enable automatic state cleanup (remove old states)
    pub cleanup_enabled: bool,
    /// Maximum age of state files in days before cleanup (if cleanup_enabled is true)
    pub max_state_age_days: Option<u32>,
}

impl Default for ArsenalConfig {
    fn default() -> Self {
        Self {
            default_timeout_seconds: 30,
            max_concurrent_tools: 5,
            mcp_servers: Vec::new(),
        }
    }
}

impl Default for CitadelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            state_dir: "./paladin-states".to_string(),
            autosave_enabled: false,
            cleanup_enabled: false,
            max_state_age_days: Some(30),
        }
    }
}

impl CitadelConfig {
    /// Validates citadel configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate state_dir is not empty when enabled
        if self.enabled && self.state_dir.trim().is_empty() {
            return Err("state_dir cannot be empty when citadel is enabled".to_string());
        }

        // Validate max_state_age_days if cleanup is enabled
        if self.cleanup_enabled {
            if let Some(max_age) = self.max_state_age_days {
                if max_age == 0 {
                    return Err(
                        "max_state_age_days must be greater than 0 when cleanup is enabled"
                            .to_string(),
                    );
                }
            } else {
                return Err(
                    "max_state_age_days must be specified when cleanup_enabled is true".to_string(),
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub source_type: String,
    pub url: String,
    pub prompt: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageServiceSettings {
    pub max_queue_size: Option<usize>,
    pub default_ttl_seconds: Option<i64>,
    pub enable_persistence: Option<bool>,
    pub worker_threads: Option<usize>,
    pub retry_attempts: Option<u32>,
    pub retry_delay_ms: Option<u64>,
}

/// Configuration for Redis queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_password: Option<String>,
    pub redis_db: u8,
    pub connection_timeout: Option<u64>,
    pub key_prefix: Option<String>,
    pub max_retries: Option<u32>,
    pub enable_priority_queues: Option<bool>,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            redis_host: "localhost".to_string(),
            redis_port: 6379,
            redis_password: None,
            redis_db: 0,
            connection_timeout: Some(30),
            key_prefix: Some("paladin:queue".to_string()),
            max_retries: Some(3),
            enable_priority_queues: Some(true),
        }
    }
}

/// Configuration for MinIO file storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStorageConfig {
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket: String,
    pub minio_region: Option<String>,
    pub minio_secure: Option<bool>,
    pub minio_path_style: Option<bool>,
    pub connection_timeout: Option<u64>,
    pub request_timeout: Option<u64>,
    pub max_idle_conns: Option<u32>,
    pub max_file_size: Option<u64>,
    pub allowed_extensions: Option<Vec<String>>,
}

impl Default for FileStorageConfig {
    fn default() -> Self {
        Self {
            minio_endpoint: "localhost:9000".to_string(),
            minio_access_key: "minioadmin".to_string(),
            minio_secret_key: "minioadmin".to_string(),
            minio_bucket: "paladin-files".to_string(),
            minio_region: None,
            minio_secure: Some(false),
            minio_path_style: Some(true),
            connection_timeout: Some(30),
            request_timeout: Some(300),
            max_idle_conns: Some(10),
            max_file_size: Some(100 * 1024 * 1024), // 100MB
            allowed_extensions: Some(vec![
                "txt".to_string(),
                "md".to_string(),
                "json".to_string(),
                "pdf".to_string(),
                "doc".to_string(),
                "docx".to_string(),
                "jpg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "html".to_string(),
                "css".to_string(),
                "xml".to_string(),
            ]),
        }
    }
}

/// Configuration for notification system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Enable/disable notification system
    pub enabled: bool,
    /// Email notification configuration
    pub email: Option<EmailAdapterConfig>,
    /// System notification configuration  
    pub system: Option<SystemAdapterConfig>,
    /// Global notification settings
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
    pub enable_delivery_tracking: bool,
    /// Rate limiting settings
    pub global_rate_limit_per_minute: Option<u32>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            email: Some(EmailAdapterConfig::default()),
            system: Some(SystemAdapterConfig::default()),
            max_retries: 3,
            retry_delay_seconds: 60,
            enable_delivery_tracking: true,
            global_rate_limit_per_minute: Some(100),
        }
    }
}

/// Configuration for Garrison memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarrisonSettings {
    /// Type of garrison storage: "in_memory" or "sqlite"
    pub garrison_type: String,
    /// Path to SQLite database file (only used if garrison_type is "sqlite")
    pub path: Option<String>,
    /// Maximum number of entries to keep in memory
    pub max_entries: usize,
    /// Maximum total tokens across all entries (None = no limit)
    pub max_tokens: Option<u32>,
    /// Tokenizer to use for token counting: "gpt-4", "gpt-3.5-turbo", etc.
    pub tokenizer: String,
    /// Eviction strategy: "importance_based", "fifo", or "sliding_window"
    pub eviction_strategy: String,
    /// Number of recent entries to always preserve
    pub preserve_recent_count: usize,
}

impl Default for GarrisonSettings {
    fn default() -> Self {
        Self {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        }
    }
}

impl GarrisonSettings {
    /// Validates garrison configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate garrison type
        if self.garrison_type != "in_memory" && self.garrison_type != "sqlite" {
            return Err(format!(
                "Invalid garrison_type '{}': must be 'in_memory' or 'sqlite'",
                self.garrison_type
            ));
        }

        // Validate SQLite path is provided when type is sqlite
        if self.garrison_type == "sqlite" && self.path.is_none() {
            return Err("SQLite garrison requires a 'path' to be specified".to_string());
        }

        // Validate max_entries
        if self.max_entries == 0 {
            return Err("max_entries must be greater than 0".to_string());
        }

        // Validate preserve_recent_count doesn't exceed max_entries
        if self.preserve_recent_count > self.max_entries {
            return Err(format!(
                "preserve_recent_count ({}) cannot exceed max_entries ({})",
                self.preserve_recent_count, self.max_entries
            ));
        }

        // Validate eviction strategy
        if self.eviction_strategy != "importance_based"
            && self.eviction_strategy != "fifo"
            && self.eviction_strategy != "sliding_window"
        {
            return Err(format!(
                "Invalid eviction_strategy '{}': must be 'importance_based', 'fifo', or 'sliding_window'",
                self.eviction_strategy
            ));
        }

        Ok(())
    }

    /// Converts settings string to EvictionStrategy enum
    /// Convert the eviction_strategy string to EvictionStrategy enum
    /// Returns the enum value, defaulting to ImportanceBased for unknown strings
    pub fn get_eviction_strategy(&self) -> EvictionStrategy {
        match self.eviction_strategy.as_str() {
            "fifo" => EvictionStrategy::FIFO,
            "sliding_window" => EvictionStrategy::SlidingWindow,
            "importance_based" => EvictionStrategy::ImportanceBased,
            _ => EvictionStrategy::ImportanceBased, // default for unknown
        }
    }
}

/// Type of Sanctum adapter to use
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SanctumAdapterType {
    /// In-memory storage (ephemeral, fast, for development/testing)
    #[default]
    InMemory,
    /// Qdrant vector database (persistent, production-grade)
    Qdrant,
}

/// Configuration for Qdrant vector database adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantSanctumConfig {
    /// Qdrant server URL (e.g., "http://localhost:6334")
    pub url: String,
    /// Collection name to use for storing memories
    pub collection_name: String,
    /// Vector dimension (must match embedding model)
    /// Common values: 1536 (OpenAI text-embedding-3-small), 3072 (text-embedding-3-large)
    pub vector_dimension: usize,
}

impl Default for QdrantSanctumConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:6334".to_string(),
            collection_name: "paladin_memories".to_string(),
            vector_dimension: 1536,
        }
    }
}

/// Configuration for Sanctum long-term memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctumConfig {
    /// Enable or disable Sanctum system
    pub enabled: bool,
    /// Type of adapter to use
    pub adapter_type: SanctumAdapterType,
    /// Qdrant-specific configuration (required if adapter_type is Qdrant)
    pub qdrant: Option<QdrantSanctumConfig>,
}

impl Default for SanctumConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            adapter_type: SanctumAdapterType::InMemory,
            qdrant: None,
        }
    }
}

impl SanctumConfig {
    /// Validates sanctum configuration
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            // If disabled, no validation needed
            return Ok(());
        }

        // Validate Qdrant configuration is present when adapter type is Qdrant
        if self.adapter_type == SanctumAdapterType::Qdrant {
            let qdrant = self.qdrant.as_ref().ok_or_else(|| {
                "Qdrant adapter requires 'qdrant' configuration section".to_string()
            })?;

            // Validate URL is not empty
            if qdrant.url.trim().is_empty() {
                return Err("Qdrant URL cannot be empty".to_string());
            }

            // Validate collection name is not empty
            if qdrant.collection_name.trim().is_empty() {
                return Err("Qdrant collection_name cannot be empty".to_string());
            }

            // Validate vector dimension is reasonable (common embedding sizes)
            if qdrant.vector_dimension == 0 {
                return Err("Qdrant vector_dimension must be greater than 0".to_string());
            }

            if qdrant.vector_dimension > 10000 {
                return Err(format!(
                    "Qdrant vector_dimension {} seems unusually large (max 10000)",
                    qdrant.vector_dimension
                ));
            }
        }

        Ok(())
    }

    /// Get the adapter type as a string for display
    pub fn adapter_type_str(&self) -> &str {
        match self.adapter_type {
            SanctumAdapterType::InMemory => "in_memory",
            SanctumAdapterType::Qdrant => "qdrant",
        }
    }
}

/// Configuration for individual LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProviderConfig {
    /// API key for the provider (can use ${ENV_VAR} syntax)
    pub api_key: String,
    /// Base URL for the API endpoint
    pub base_url: Option<String>,
    /// Default model to use
    pub default_model: Option<String>,
    /// Default temperature (0.0-2.0)
    pub default_temperature: Option<f32>,
    /// Default timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Maximum retries for failed requests
    pub max_retries: Option<u32>,
}

/// Configuration for all LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Default provider to use if not specified
    /// Options: "openai", "deepseek", "anthropic"
    pub default_provider: Option<String>,
    /// OpenAI configuration
    pub openai: Option<LlmProviderConfig>,
    /// DeepSeek configuration
    pub deepseek: Option<LlmProviderConfig>,
    /// Anthropic configuration
    pub anthropic: Option<LlmProviderConfig>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            default_provider: Some("openai".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        }
    }
}

impl LlmConfig {
    /// Validate LLM configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Check if default provider is configured
        if let Some(default) = &self.default_provider {
            match default.as_str() {
                "openai" if self.openai.is_none() => {
                    return Err(ConfigError::Message(
                        "Default provider is 'openai' but openai config is not present".to_string(),
                    ));
                }
                "deepseek" if self.deepseek.is_none() => {
                    return Err(ConfigError::Message(
                        "Default provider is 'deepseek' but deepseek config is not present"
                            .to_string(),
                    ));
                }
                "anthropic" if self.anthropic.is_none() => {
                    return Err(ConfigError::Message(
                        "Default provider is 'anthropic' but anthropic config is not present"
                            .to_string(),
                    ));
                }
                "openai" | "deepseek" | "anthropic" => {}
                _ => {
                    return Err(ConfigError::Message(format!(
                        "Invalid default provider: {}. Must be 'openai', 'deepseek', or 'anthropic'",
                        default
                    )));
                }
            }
        }

        // Validate provider configs if present
        if let Some(openai) = &self.openai
            && openai.api_key.is_empty()
        {
            return Err(ConfigError::Message(
                "OpenAI API key cannot be empty".to_string(),
            ));
        }

        if let Some(deepseek) = &self.deepseek
            && deepseek.api_key.is_empty()
        {
            return Err(ConfigError::Message(
                "DeepSeek API key cannot be empty".to_string(),
            ));
        }

        if let Some(anthropic) = &self.anthropic
            && anthropic.api_key.is_empty()
        {
            return Err(ConfigError::Message(
                "Anthropic API key cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the provider config for a specific provider
    pub fn get_provider_config(&self, provider_name: &str) -> Option<&LlmProviderConfig> {
        match provider_name.to_lowercase().as_str() {
            "openai" => self.openai.as_ref(),
            "deepseek" => self.deepseek.as_ref(),
            "anthropic" => self.anthropic.as_ref(),
            _ => None,
        }
    }

    /// Get the default provider name
    pub fn get_default_provider_name(&self) -> Option<String> {
        self.default_provider.clone()
    }
}

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
    pub notifications: Option<NotificationConfig>,
    pub garrison: Option<GarrisonSettings>,
    pub sanctum: Option<SanctumConfig>,
    pub arsenal: Option<ArsenalConfig>,
    pub citadel: Option<CitadelConfig>,
    pub llm: Option<LlmConfig>,
    pub herald: Option<HeraldConfig>,
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
            notifications: Some(NotificationConfig::default()),
            garrison: Some(GarrisonSettings::default()),
            sanctum: Some(SanctumConfig::default()),
            arsenal: Some(ArsenalConfig::default()),
            citadel: Some(CitadelConfig::default()),
            llm: Some(LlmConfig::default()),
            herald: Some(HeraldConfig::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    fn test_default_file_storage_config() {
        let config = FileStorageConfig::default();
        assert_eq!(config.minio_endpoint, "localhost:9000");
        assert_eq!(config.minio_access_key, "minioadmin");
        assert_eq!(config.minio_secret_key, "minioadmin");
        assert_eq!(config.minio_bucket, "paladin-files");
        assert_eq!(config.minio_secure, Some(false));
        assert_eq!(config.minio_path_style, Some(true));
        assert_eq!(config.connection_timeout, Some(30));
        assert_eq!(config.request_timeout, Some(300));
        assert_eq!(config.max_idle_conns, Some(10));
        assert_eq!(config.max_file_size, Some(100 * 1024 * 1024));
        assert!(config.allowed_extensions.is_some());
    }

    #[test]
    #[serial]
    fn test_file_storage_config_env_override() {
        // Set environment variables
        unsafe {
            env::set_var("APP_MINIO_ENDPOINT", "minio-server:9000");
            env::set_var("APP_MINIO_ACCESS_KEY", "testuser");
            env::set_var("APP_MINIO_SECRET_KEY", "testpass");
            env::set_var("APP_MINIO_BUCKET", "test-bucket");
            env::set_var("APP_MINIO_REGION", "us-east-1");
            env::set_var("APP_MINIO_SECURE", "true");
            env::set_var("APP_MINIO_PATH_STYLE", "false");
            env::set_var("APP_MINIO_CONNECTION_TIMEOUT", "60");
            env::set_var("APP_MINIO_REQUEST_TIMEOUT", "600");
            env::set_var("APP_MINIO_MAX_IDLE_CONNS", "20");
            env::set_var("APP_MINIO_MAX_FILE_SIZE", "209715200"); // 200MB
            env::set_var("APP_MINIO_ALLOWED_EXTENSIONS", "pdf,doc,docx,jpg,png");
        }
        let settings = Settings::default();
        let config = settings.get_file_storage_config();

        assert_eq!(config.minio_endpoint, "minio-server:9000");
        assert_eq!(config.minio_access_key, "testuser");
        assert_eq!(config.minio_secret_key, "testpass");
        assert_eq!(config.minio_bucket, "test-bucket");
        assert_eq!(config.minio_region, Some("us-east-1".to_string()));
        assert_eq!(config.minio_secure, Some(true));
        assert_eq!(config.minio_path_style, Some(false));
        assert_eq!(config.connection_timeout, Some(60));
        assert_eq!(config.request_timeout, Some(600));
        assert_eq!(config.max_idle_conns, Some(20));
        assert_eq!(config.max_file_size, Some(209715200));
        assert_eq!(
            config.allowed_extensions,
            Some(vec![
                "pdf".to_string(),
                "doc".to_string(),
                "docx".to_string(),
                "jpg".to_string(),
                "png".to_string()
            ])
        );

        // Clean up
        unsafe {
            env::remove_var("APP_MINIO_ENDPOINT");
            env::remove_var("APP_MINIO_ACCESS_KEY");
            env::remove_var("APP_MINIO_SECRET_KEY");
            env::remove_var("APP_MINIO_BUCKET");
            env::remove_var("APP_MINIO_REGION");
            env::remove_var("APP_MINIO_SECURE");
            env::remove_var("APP_MINIO_PATH_STYLE");
            env::remove_var("APP_MINIO_CONNECTION_TIMEOUT");
            env::remove_var("APP_MINIO_REQUEST_TIMEOUT");
            env::remove_var("APP_MINIO_MAX_IDLE_CONNS");
            env::remove_var("APP_MINIO_MAX_FILE_SIZE");
            env::remove_var("APP_MINIO_ALLOWED_EXTENSIONS");
        }
    }

    #[test]
    fn test_settings_with_file_storage_config() {
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
    }

    #[test]
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
    fn test_queue_config_compatibility() {
        // Ensure existing queue config functionality still works
        let settings = Settings::default();
        let queue_config = settings.get_queue_config();

        assert_eq!(queue_config.redis_host, "localhost");
        assert_eq!(queue_config.redis_port, 6379);
        assert_eq!(queue_config.redis_db, 0);
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

    #[test]
    fn test_garrison_settings_validation_success() {
        let valid_settings = GarrisonSettings {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        };

        assert!(valid_settings.validate().is_ok());
    }

    #[test]
    fn test_garrison_settings_validation_invalid_type() {
        let invalid_settings = GarrisonSettings {
            garrison_type: "invalid_type".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        };

        assert!(invalid_settings.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_sqlite_without_path() {
        let invalid_settings = GarrisonSettings {
            garrison_type: "sqlite".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        };

        assert!(invalid_settings.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_zero_max_entries() {
        let invalid_settings = GarrisonSettings {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 0,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 10,
        };

        assert!(invalid_settings.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_preserve_exceeds_max() {
        let invalid_settings = GarrisonSettings {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 10,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "importance_based".to_string(),
            preserve_recent_count: 20,
        };

        assert!(invalid_settings.validate().is_err());
    }

    #[test]
    fn test_garrison_settings_validation_invalid_eviction() {
        let invalid_settings = GarrisonSettings {
            garrison_type: "in_memory".to_string(),
            path: None,
            max_entries: 100,
            max_tokens: Some(4000),
            tokenizer: "gpt-4".to_string(),
            eviction_strategy: "invalid_strategy".to_string(),
            preserve_recent_count: 10,
        };

        assert!(invalid_settings.validate().is_err());
    }

    #[test]
    fn test_garrison_get_eviction_strategy() {
        let settings_importance = GarrisonSettings {
            eviction_strategy: "importance_based".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            settings_importance.get_eviction_strategy(),
            crate::core::platform::container::garrison::EvictionStrategy::ImportanceBased
        ));

        let settings_fifo = GarrisonSettings {
            eviction_strategy: "fifo".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            settings_fifo.get_eviction_strategy(),
            crate::core::platform::container::garrison::EvictionStrategy::FIFO
        ));

        let settings_sliding = GarrisonSettings {
            eviction_strategy: "sliding_window".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            settings_sliding.get_eviction_strategy(),
            crate::core::platform::container::garrison::EvictionStrategy::SlidingWindow
        ));

        // Unknown strategy defaults to ImportanceBased
        let settings_invalid = GarrisonSettings {
            eviction_strategy: "invalid".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            settings_invalid.get_eviction_strategy(),
            crate::core::platform::container::garrison::EvictionStrategy::ImportanceBased
        ));
    }

    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();
        assert_eq!(config.default_provider, Some("openai".to_string()));
        assert!(config.openai.is_none());
        assert!(config.deepseek.is_none());
        assert!(config.anthropic.is_none());
    }

    #[test]
    fn test_llm_config_validate_default_provider_must_be_configured() {
        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());

        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());

        let config = LlmConfig {
            default_provider: Some("anthropic".to_string()),
            openai: None,
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_invalid_provider_name() {
        let config = LlmConfig {
            default_provider: Some("invalid_provider".to_string()),
            openai: Some(LlmProviderConfig {
                api_key: "key".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_empty_api_key() {
        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(LlmProviderConfig {
                api_key: "".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
            deepseek: None,
            anthropic: None,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_llm_config_validate_success() {
        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: Some(LlmProviderConfig {
                api_key: "test-key".to_string(),
                base_url: Some("https://api.deepseek.com/v1".to_string()),
                default_model: Some("deepseek-chat".to_string()),
                default_temperature: Some(0.7),
                timeout_seconds: Some(300),
                max_retries: Some(3),
            }),
            anthropic: None,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_llm_config_get_provider_config() {
        let openai_config = LlmProviderConfig {
            api_key: "openai-key".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        };

        let config = LlmConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(openai_config.clone()),
            deepseek: None,
            anthropic: None,
        };

        let retrieved = config.get_provider_config("openai");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().api_key, "openai-key");

        let not_found = config.get_provider_config("deepseek");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_llm_config_get_provider_config_case_insensitive() {
        let deepseek_config = LlmProviderConfig {
            api_key: "deepseek-key".to_string(),
            base_url: None,
            default_model: None,
            default_temperature: None,
            timeout_seconds: None,
            max_retries: None,
        };

        let config = LlmConfig {
            default_provider: Some("deepseek".to_string()),
            openai: None,
            deepseek: Some(deepseek_config.clone()),
            anthropic: None,
        };

        assert!(config.get_provider_config("DeepSeek").is_some());
        assert!(config.get_provider_config("DEEPSEEK").is_some());
        assert!(config.get_provider_config("deepseek").is_some());
    }

    #[test]
    fn test_llm_config_get_default_provider_name() {
        let config = LlmConfig {
            default_provider: Some("anthropic".to_string()),
            openai: None,
            deepseek: None,
            anthropic: Some(LlmProviderConfig {
                api_key: "key".to_string(),
                base_url: None,
                default_model: None,
                default_temperature: None,
                timeout_seconds: None,
                max_retries: None,
            }),
        };

        assert_eq!(
            config.get_default_provider_name(),
            Some("anthropic".to_string())
        );
    }

    #[test]
    fn test_citadel_config_default() {
        let config = CitadelConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.state_dir, "./paladin-states");
        assert!(!config.autosave_enabled);
        assert!(!config.cleanup_enabled);
        assert_eq!(config.max_state_age_days, Some(30));
    }

    #[test]
    fn test_citadel_config_validate_valid() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./states".to_string(),
            autosave_enabled: true,
            cleanup_enabled: true,
            max_state_age_days: Some(7),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_citadel_config_validate_empty_state_dir_when_enabled() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "   ".to_string(), // Empty after trim
            autosave_enabled: false,
            cleanup_enabled: false,
            max_state_age_days: Some(30),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("state_dir cannot be empty when citadel is enabled")
        );
    }

    #[test]
    fn test_citadel_config_validate_cleanup_no_max_age() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./states".to_string(),
            autosave_enabled: false,
            cleanup_enabled: true,
            max_state_age_days: None,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("max_state_age_days must be specified when cleanup_enabled is true")
        );
    }

    #[test]
    fn test_citadel_config_validate_cleanup_zero_max_age() {
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./states".to_string(),
            autosave_enabled: false,
            cleanup_enabled: true,
            max_state_age_days: Some(0),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("max_state_age_days must be greater than 0 when cleanup is enabled")
        );
    }

    #[test]
    #[serial]
    fn test_citadel_config_env_override() {
        unsafe {
            env::set_var("APP_CITADEL_ENABLED", "true");
            env::set_var("APP_CITADEL_STATE_DIR", "/custom/states");
            env::set_var("APP_CITADEL_AUTOSAVE_ENABLED", "true");
            env::set_var("APP_CITADEL_CLEANUP_ENABLED", "true");
            env::set_var("APP_CITADEL_MAX_STATE_AGE_DAYS", "60");
        }

        let settings = Settings::default();
        let config = settings.get_citadel_config();

        assert!(config.enabled);
        assert_eq!(config.state_dir, "/custom/states");
        assert!(config.autosave_enabled);
        assert!(config.cleanup_enabled);
        assert_eq!(config.max_state_age_days, Some(60));

        unsafe {
            env::remove_var("APP_CITADEL_ENABLED");
            env::remove_var("APP_CITADEL_STATE_DIR");
            env::remove_var("APP_CITADEL_AUTOSAVE_ENABLED");
            env::remove_var("APP_CITADEL_CLEANUP_ENABLED");
            env::remove_var("APP_CITADEL_MAX_STATE_AGE_DAYS");
        }
    }

    #[test]
    fn test_citadel_config_deserialization_from_yml() {
        // Test that CitadelConfig can be deserialized with all fields
        let config = CitadelConfig {
            enabled: true,
            state_dir: "./test-states".to_string(),
            autosave_enabled: true,
            cleanup_enabled: false,
            max_state_age_days: Some(14),
        };

        assert!(config.enabled);
        assert_eq!(config.state_dir, "./test-states");
        assert!(config.autosave_enabled);
        assert!(!config.cleanup_enabled);
        assert_eq!(config.max_state_age_days, Some(14));
    }

    #[test]
    fn test_herald_config_default() {
        let config = HeraldConfig::default();
        assert_eq!(config.default_formatter, "json");
        assert!(config.json.pretty);
        assert!(config.json.include_metadata);
        assert!(config.markdown.include_colors);
        assert_eq!(config.markdown.heading_level, 2);
        assert_eq!(config.table.max_column_width, 60);
        assert_eq!(config.table.border_style, "rounded");
    }

    #[test]
    fn test_herald_config_validate_valid() {
        let config = HeraldConfig {
            default_formatter: "markdown".to_string(),
            json: JsonHeraldConfig {
                pretty: true,
                include_metadata: true,
            },
            markdown: MarkdownHeraldConfig {
                include_colors: true,
                heading_level: 3,
            },
            table: TableHeraldConfig {
                max_column_width: 80,
                border_style: "ascii".to_string(),
            },
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_herald_config_validate_invalid_formatter() {
        let config = HeraldConfig {
            default_formatter: "invalid".to_string(),
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid default_formatter"));
    }

    #[test]
    fn test_herald_config_validate_invalid_heading_level_low() {
        let config = HeraldConfig {
            default_formatter: "markdown".to_string(),
            markdown: MarkdownHeraldConfig {
                include_colors: true,
                heading_level: 0,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid markdown heading_level")
        );
    }

    #[test]
    fn test_herald_config_validate_invalid_heading_level_high() {
        let config = HeraldConfig {
            default_formatter: "markdown".to_string(),
            markdown: MarkdownHeraldConfig {
                include_colors: true,
                heading_level: 7,
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid markdown heading_level")
        );
    }

    #[test]
    fn test_herald_config_validate_zero_column_width() {
        let config = HeraldConfig {
            default_formatter: "table".to_string(),
            table: TableHeraldConfig {
                max_column_width: 0,
                border_style: "rounded".to_string(),
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("table max_column_width must be greater than 0")
        );
    }

    #[test]
    #[serial]
    fn test_herald_config_env_override() {
        unsafe {
            env::set_var("HERALD_DEFAULT_FORMATTER", "markdown");
            env::set_var("APP_HERALD_JSON_PRETTY", "false");
            env::set_var("APP_HERALD_JSON_INCLUDE_METADATA", "false");
            env::set_var("APP_HERALD_MARKDOWN_INCLUDE_COLORS", "false");
            env::set_var("APP_HERALD_MARKDOWN_HEADING_LEVEL", "4");
            env::set_var("APP_HERALD_TABLE_MAX_COLUMN_WIDTH", "100");
            env::set_var("APP_HERALD_TABLE_BORDER_STYLE", "modern");
        }

        let settings = Settings::default();
        let config = settings.get_herald_config();

        assert_eq!(config.default_formatter, "markdown");
        assert!(!config.json.pretty);
        assert!(!config.json.include_metadata);
        assert!(!config.markdown.include_colors);
        assert_eq!(config.markdown.heading_level, 4);
        assert_eq!(config.table.max_column_width, 100);
        assert_eq!(config.table.border_style, "modern");

        unsafe {
            env::remove_var("HERALD_DEFAULT_FORMATTER");
            env::remove_var("APP_HERALD_JSON_PRETTY");
            env::remove_var("APP_HERALD_JSON_INCLUDE_METADATA");
            env::remove_var("APP_HERALD_MARKDOWN_INCLUDE_COLORS");
            env::remove_var("APP_HERALD_MARKDOWN_HEADING_LEVEL");
            env::remove_var("APP_HERALD_TABLE_MAX_COLUMN_WIDTH");
            env::remove_var("APP_HERALD_TABLE_BORDER_STYLE");
        }
    }

    #[test]
    #[serial]
    fn test_herald_config_app_prefix_override() {
        unsafe {
            env::set_var("APP_HERALD_DEFAULT_FORMATTER", "table");
        }

        let settings = Settings::default();
        let config = settings.get_herald_config();

        assert_eq!(config.default_formatter, "table");

        unsafe {
            env::remove_var("APP_HERALD_DEFAULT_FORMATTER");
        }
    }

    #[test]
    fn test_herald_config_deserialization() {
        let config = HeraldConfig {
            default_formatter: "json".to_string(),
            json: JsonHeraldConfig {
                pretty: false,
                include_metadata: false,
            },
            markdown: MarkdownHeraldConfig {
                include_colors: false,
                heading_level: 1,
            },
            table: TableHeraldConfig {
                max_column_width: 120,
                border_style: "sharp".to_string(),
            },
        };

        assert_eq!(config.default_formatter, "json");
        assert!(!config.json.pretty);
        assert!(!config.json.include_metadata);
        assert!(!config.markdown.include_colors);
        assert_eq!(config.markdown.heading_level, 1);
        assert_eq!(config.table.max_column_width, 120);
        assert_eq!(config.table.border_style, "sharp");
    }

    #[test]
    fn test_create_default_herald_json() {
        let mut settings = Settings::default();
        settings.herald = Some(HeraldConfig {
            default_formatter: "json".to_string(),
            ..Default::default()
        });

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "json");
        assert_eq!(herald.mime_type(), "application/json");
    }

    #[test]
    fn test_create_default_herald_markdown() {
        let mut settings = Settings::default();
        settings.herald = Some(HeraldConfig {
            default_formatter: "markdown".to_string(),
            ..Default::default()
        });

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "markdown");
        assert_eq!(herald.mime_type(), "text/markdown");
    }

    #[test]
    fn test_create_default_herald_table() {
        let mut settings = Settings::default();
        settings.herald = Some(HeraldConfig {
            default_formatter: "table".to_string(),
            ..Default::default()
        });

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "table");
        assert_eq!(herald.mime_type(), "text/plain");
    }

    #[test]
    fn test_create_default_herald_invalid_formatter() {
        let mut settings = Settings::default();
        settings.herald = Some(HeraldConfig {
            default_formatter: "invalid".to_string(),
            ..Default::default()
        });

        let herald = settings.create_default_herald();
        assert!(herald.is_err());
        let err_msg = herald.err().unwrap();
        assert!(err_msg.contains("Unknown formatter 'invalid'"));
    }

    #[test]
    fn test_create_default_herald_with_custom_config() {
        let mut settings = Settings::default();
        settings.herald = Some(HeraldConfig {
            default_formatter: "json".to_string(),
            json: JsonHeraldConfig {
                pretty: false,
                include_metadata: false,
            },
            ..Default::default()
        });

        let herald = settings.create_default_herald();
        assert!(herald.is_ok());
        // Config is passed correctly to JsonHerald (verified via unit tests)
    }

    // Sanctum Configuration Tests
    #[test]
    fn test_default_sanctum_config() {
        let config = SanctumConfig::default();

        assert!(!config.enabled);
        assert_eq!(config.adapter_type, SanctumAdapterType::InMemory);
        assert!(config.qdrant.is_none());
    }

    #[test]
    fn test_sanctum_validation_disabled() {
        let config = SanctumConfig {
            enabled: false,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: None,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sanctum_validation_in_memory() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::InMemory,
            qdrant: None,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sanctum_validation_qdrant_missing_config() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: None,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Qdrant adapter requires"));
    }

    #[test]
    fn test_sanctum_validation_qdrant_valid() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: Some(QdrantSanctumConfig {
                url: "http://localhost:6334".to_string(),
                collection_name: "test".to_string(),
                vector_dimension: 1536,
            }),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sanctum_validation_empty_url() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: Some(QdrantSanctumConfig {
                url: "   ".to_string(),
                collection_name: "test".to_string(),
                vector_dimension: 1536,
            }),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("URL cannot be empty"));
    }

    #[test]
    fn test_sanctum_validation_zero_dimension() {
        let config = SanctumConfig {
            enabled: true,
            adapter_type: SanctumAdapterType::Qdrant,
            qdrant: Some(QdrantSanctumConfig {
                url: "http://localhost:6334".to_string(),
                collection_name: "test".to_string(),
                vector_dimension: 0,
            }),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be greater than 0"));
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
}
