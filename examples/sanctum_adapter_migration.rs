//! # Sanctum Adapter Migration Example
//!
//! **Note**: This example requires the "qdrant" feature flag for Qdrant migrations.
//!
//! Run with:
//! ```bash
//! cargo run --example sanctum_adapter_migration --features qdrant
//! ```
//!
//! For complete migration procedures and strategies, see:
//! - docs/SANCTUM_MIGRATION.md - Complete migration guide
//! - docs/SANCTUM.md - Adapter configuration

#[cfg(not(feature = "qdrant"))]
fn main() {
    eprintln!("❌ This example requires the 'qdrant' feature flag for migrations.");
    eprintln!("\nRun with:");
    eprintln!("  cargo run --example sanctum_adapter_migration --features qdrant");
    eprintln!("\nFor complete migration procedures, see docs/SANCTUM_MIGRATION.md");
    std::process::exit(1);
}

#[cfg(feature = "qdrant")]
fn main() {
    println!("Sanctum Adapter Migration Example");
    println!("==================================\n");

    println!("For complete migration strategies and procedures,");
    println!("please refer to the migration documentation:\n");

    println!("📚 docs/SANCTUM_MIGRATION.md");
    println!("   - Complete migration guide");
    println!("   - Export/import procedures");
    println!("   - Zero-downtime migrations");
    println!("   - Validation and verification");
    println!("   - Rollback strategies\n");

    println!("📖 docs/SANCTUM.md");
    println!("   - Adapter configuration");
    println!("   - Compatibility matrix");
    println!("   - Best practices\n");

    println!("Migration scenarios covered:");
    println!("  ✓ InMemory to Qdrant (development to production)");
    println!("  ✓ Qdrant to Qdrant (instance migration)");
    println!("  ✓ Partial migrations (filtered exports)");
    println!("  ✓ Incremental synchronization");
    println!("  ✓ Validation and testing");
}
