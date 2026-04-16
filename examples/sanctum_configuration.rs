//! # Sanctum Configuration Example
//!
//! This example demonstrates various configuration patterns for Sanctum across
//! different environments (development, staging, production).
//!
//! For complete configuration reference, see:
//! - docs/SANCTUM.md - Configuration guide
//! - config.yml - Application configuration file

use paladin::{
    config::application_settings::Settings,
    infrastructure::adapters::sanctum::InMemorySanctum,
};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sanctum Configuration Example");
    println!("==============================\n");

    // Example 1: Basic configuration
    println!("1. Basic InMemory Configuration");
    println!("================================\n");

    println!("Code:");
    println!("  let adapter = InMemorySanctum::new(10000);");
    println!();
    println!("✓ InMemory adapter with 10,000 entry capacity");
    println!("  - Best for: Development and testing");
    println!("  - Storage: In-process memory");
    println!("  - Persistence: None (resets on restart)\n");

    let _dev_adapter = InMemorySanctum::new(10000);

    // Example 2: Configuration file loading
    println!("2. Loading Configuration from config.yml");
    println!("=========================================\n");

    match Settings::new() {
        Ok(settings) => {
            println!("✓ Configuration loaded successfully");

            let sanctum_config = settings.get_sanctum_config();
            println!("\nSanctum Configuration:");
            println!("  Enabled: {}", sanctum_config.enabled);
            println!("  Adapter: {:?}", sanctum_config.adapter_type);

            if let Some(qdrant_config) = &sanctum_config.qdrant {
                println!("  Qdrant URL: {}", qdrant_config.url);
                println!("  Collection: {}", qdrant_config.collection_name);
                println!("  Dimension: {}", qdrant_config.vector_dimension);
            }
            println!();
        }
        Err(e) => {
            eprintln!("✗ Failed to load configuration: {}", e);
            eprintln!("  Make sure config.yml exists in the project root\n");
        }
    };

    // Example 3: Environment-specific configuration
    println!("3. Environment-Specific Configuration");
    println!("=====================================\n");

    println!("Development:");
    println!("  sanctum:");
    println!("    enabled: true");
    println!("    adapter_type: \"in_memory\"");
    println!();

    println!("Staging:");
    println!("  sanctum:");
    println!("    enabled: true");
    println!("    adapter_type: \"qdrant\"");
    println!("    qdrant:");
    println!("      url: \"http://qdrant-staging:6334\"");
    println!("      collection_name: \"paladin_staging\"");
    println!();

    println!("Production:");
    println!("  sanctum:");
    println!("    enabled: true");
    println!("    adapter_type: \"qdrant\"");
    println!("    qdrant:");
    println!("      url: \"http://qdrant-prod:6334\"");
    println!("      collection_name: \"paladin_production\"");
    println!();

    // Example 4: Runtime environment detection
    println!("4. Runtime Environment Detection");
    println!("================================\n");

    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    println!("Detected environment: {}", environment);

    let adapter_name = match environment.as_str() {
        "production" => "Qdrant (production cluster)",
        "staging" => "Qdrant (staging instance)",
        "development" => "InMemory (development)",
        _ => "InMemory (default)",
    };

    println!("Recommended adapter: {}\n", adapter_name);

    // Example 5: Configuration validation
    println!("5. Configuration Validation");
    println!("===========================\n");

    println!("For production deployments, validate:");
    println!("  ✓ Adapter type is valid (in_memory or qdrant)");
    println!("  ✓ If qdrant: URL is accessible");
    println!("  ✓ If qdrant: Collection exists");
    println!("  ✓ If qdrant: Dimension matches embedding model");
    println!("  ✓ If qdrant: Authentication configured (if required)\n");

    // Example 6: Common vector dimensions
    println!("6. Common Vector Dimensions");
    println!("===========================\n");

    println!("Choose dimension based on your embedding model:");
    println!("  - OpenAI text-embedding-ada-002: 1536");
    println!("  - OpenAI text-embedding-3-small: 1536");
    println!("  - OpenAI text-embedding-3-large: 3072");
    println!("  - Sentence-Transformers (all-MiniLM-L6-v2): 384");
    println!("  - Sentence-Transformers (all-mpnet-base-v2): 768");
    println!("  - BGE-small-en: 384");
    println!("  - BGE-base-en: 768");
    println!("  - BGE-large-en: 1024\n");

    println!("Example completed successfully!");
    println!("\nFor more configuration examples, see:");
    println!("  - docs/SANCTUM.md");
    println!("  - docs/SANCTUM_DEPLOYMENT.md");
    println!("  - config.yml");

    Ok(())
}
