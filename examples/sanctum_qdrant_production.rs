//! # Sanctum with Qdrant Production Example
//!
//! **Note**: This example requires the "qdrant" feature flag enabled.
//! 
//! Run with:
//! ```bash
//! cargo run --example sanctum_qdrant_production --features qdrant
//! ```
//!
//! For complete Qdrant usage examples with real embeddings, see:
//! - docs/SANCTUM.md - Comprehensive usage guide
//! - docs/SANCTUM_DEPLOYMENT.md - Production deployment strategies

#[cfg(not(feature = "qdrant"))]
fn main() {
    eprintln!("❌ This example requires the 'qdrant' feature flag.");
    eprintln!("\nRun with:");
    eprintln!("  cargo run --example sanctum_qdrant_production --features qdrant");
    eprintln!("\nFor complete Qdrant usage examples, see docs/SANCTUM.md");
    std::process::exit(1);
}

#[cfg(feature = "qdrant")]
fn main() {
    println!("Sanctum with Qdrant Production Example");
    println!("======================================\n");
    
    println!("For complete Qdrant production examples with real embeddings,");
    println!("please refer to the comprehensive documentation:\n");
    
    println!("📚 docs/SANCTUM.md");
    println!("   - Complete usage guide");
    println!("   - Qdrant setup and configuration");
    println!("   - Embedding integration (OpenAI, etc.)");
    println!("   - Production best practices\n");
    
    println!("🚀 docs/SANCTUM_DEPLOYMENT.md");
    println!("   - Docker Compose setup");
    println!("   - Kubernetes deployment");
    println!("   - Monitoring and observability");
    println!("   - Scaling strategies\n");
    
    println!("Key topics covered:");
    println!("  ✓ Setting up Qdrant with Docker");
    println!("  ✓ Production configuration");
    println!("  ✓ OpenAI embedding integration");
    println!("  ✓ Performance tuning");
    println!("  ✓ Batch operations");
    println!("  ✓ Monitoring and metrics");
}
