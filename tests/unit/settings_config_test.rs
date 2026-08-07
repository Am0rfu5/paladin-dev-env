//! Integration tests for [`Settings`] — top-level configuration struct,
//! per-domain accessor methods, and YAML/TOML file loading.

use paladin::config::{
    FileStorageConfig, GarrisonSettings, HeraldConfig, JsonHeraldConfig, QdrantSanctumConfig,
    SanctumAdapterType, SanctumConfig, Settings, VisionConfig, VisionProviderConfig,
    VisionRetryConfig,
};
use serial_test::serial;
use std::env;
#[cfg(feature = "s3-storage")]
use std::time::Duration;

// ── File-storage tests ─────────────────────────────────────────────────────────

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

// ── Garrison tests ─────────────────────────────────────────────────────────────

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

// ── Herald tests ───────────────────────────────────────────────────────────────

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

    // `TableHerald` requires the `cli` feature (ADR-0023). With it enabled, "table"
    // constructs normally; without it, the "table" match arm is compiled out and the
    // request falls through to the existing `Unknown formatter` error path.
    if cfg!(feature = "cli") {
        assert!(herald.is_ok());
        let herald = herald.unwrap();
        assert_eq!(herald.name(), "table");
        assert_eq!(herald.mime_type(), "text/plain");
    } else {
        assert!(herald.is_err());
        let err_msg = herald.err().unwrap();
        assert!(err_msg.contains("Unknown formatter 'table'"));
    }
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
}

// ── Sanctum tests ──────────────────────────────────────────────────────────────

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

// ── Vision tests ───────────────────────────────────────────────────────────────

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
#[serial]
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

// ── Regression: file loading ───────────────────────────────────────────────────

/// Verify Settings::load_from_file round-trips config.test.yml and that every
/// domain's get_*_config() method returns sensible values.
#[test]
#[serial]
fn test_load_from_file_regression() {
    let settings =
        Settings::load_from_file("config.test.yml").expect("config.test.yml should load");

    // Server domain
    assert_eq!(settings.server.host, "127.0.0.1");
    assert_eq!(settings.server.port, 8080);

    // Garrison domain
    let garrison = settings.get_garrison_config();
    assert_eq!(garrison.garrison_type, "in_memory");
    assert_eq!(garrison.max_entries, 50);

    // Sanctum domain
    let sanctum = settings.get_sanctum_config();
    assert!(!sanctum.enabled);

    // Herald domain
    let herald = settings.get_herald_config();
    assert_eq!(herald.default_formatter, "json");

    // Citadel domain
    let citadel = settings.get_citadel_config();
    assert!(!citadel.enabled);
    assert_eq!(citadel.state_dir, "./test-states");

    // Vision domain
    let vision = settings.get_vision_config();
    assert_eq!(vision.retry.max_retries, 2);
    assert_eq!(vision.retry.initial_backoff_ms, 500);
}
