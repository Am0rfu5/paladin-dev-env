//! Paladin with RAG (Retrieval-Augmented Generation) Example
//!
//! Demonstrates the RAG configuration and conceptual workflow added in Epic 12.
//!
//! **RAG Capabilities:**
//! - Automatic memory extraction after execution  
//! - Automatic context retrieval before execution
//! - Building agent knowledge base over sessions
//! - Improving responses with historical context
//!
//! **Components:**
//! - `RagRetrievalService` - Automatic context retrieval from Sanctum
//! - `MemoryExtractionService` - Automatic memory storage
//! - `PaladinExecutionService` - Integrated RAG execution flow
//!
//! **Note**: This is a conceptual demonstration. For a working example,
//! see `paladin_with_sanctum.rs` which demonstrates memory operations,
//! and configure RAG via `config.yml` or `examples/cli_configs/paladin_rag.yaml`.
//!
//! Run with: cargo run --example paladin_with_rag

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║  Paladin RAG (Retrieval-Augmented Generation) Configuration      ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // Overview: What is RAG?
    // ========================================================================
    println!("📚 What is RAG?\n");
    println!("   Retrieval-Augmented Generation enhances Paladin agents by:");
    println!("   • Automatically storing important information from conversations");
    println!("   • Retrieving relevant context before generating responses");
    println!("   • Building long-term knowledge across multiple sessions");
    println!("   • Improving response quality with historical context\n");

    // ========================================================================
    // Architecture Overview
    // ========================================================================
    println!("🏗️  Architecture Overview\n");
    println!("   RAG Flow:");
    println!("   ┌─────────────────┐");
    println!("   │ User Input      │");
    println!("   └────────┬────────┘");
    println!("            ↓");
    println!("   ┌─────────────────────────┐");
    println!("   │ 1. Retrieve Context     │ ← RagRetrievalService");
    println!("   │    (from Sanctum)       │   • Embed query");
    println!("   └────────┬────────────────┘   • Search memories");
    println!("            ↓                    • Rank by relevance");
    println!("   ┌─────────────────────────┐");
    println!("   │ 2. Inject Context       │");
    println!("   │    (into prompt)        │");
    println!("   └────────┬────────────────┘");
    println!("            ↓");
    println!("   ┌─────────────────────────┐");
    println!("   │ 3. LLM Generation       │ ← PaladinExecutionService");
    println!("   │    (with context)       │");
    println!("   └────────┬────────────────┘");
    println!("            ↓");
    println!("   ┌─────────────────────────┐");
    println!("   │ 4. Extract Memories     │ ← MemoryExtractionService");
    println!("   │    (store in Sanctum)   │   • Identify key facts");
    println!("   └────────┬────────────────┘   • Generate embeddings");
    println!("            ↓                    • Store for future use");
    println!("   ┌─────────────────┐");
    println!("   │ Response        │");
    println!("   └─────────────────┘\n");

    // ========================================================================
    // Configuration Examples
    // ========================================================================
    println!("⚙️  Configuration Examples\n");

    println!("   1. Sanctum Configuration (config.yml):\n");
    println!("   ```yaml");
    println!("   sanctum:");
    println!("     provider: qdrant  # or 'in_memory'");
    println!("     qdrant:");
    println!("       url: http://localhost:6333");
    println!("       collection_name: paladin_memories");
    println!("       vector_size: 1536  # OpenAI embeddings");
    println!("       distance: cosine");
    println!("   ```\n");

    println!("   2. RAG Retrieval Configuration:\n");
    println!("   ```yaml");
    println!("   rag:");
    println!("     top_k: 5  # Retrieve top 5 relevant memories");
    println!("     min_similarity: 0.7  # Only memories above 70% match");
    println!("     max_tokens: 2000  # Context budget limit");
    println!("     timeout_seconds: 5  # Retrieval timeout");
    println!("   ```\n");

    println!("   3. Memory Extraction Configuration:\n");
    println!("   ```yaml");
    println!("   memory_extraction:");
    println!("     enabled: true");
    println!("     strategy: on_completion  # or 'every_turn', 'threshold'");
    println!("   ```\n");

    // ========================================================================
    // Usage Patterns
    // ========================================================================
    println!("💡 Usage Patterns\n");

    println!("   Pattern 1: Automatic RAG (Recommended)");
    println!("   ───────────────────────────────────────");
    println!("   Configure via config.yml, RAG happens automatically:");
    println!("   • No code changes required");
    println!("   • Context retrieval before each execution");
    println!("   • Memory extraction after completion\n");

    println!("   Pattern 2: Programmatic Configuration");
    println!("   ──────────────────────────────────────");
    println!("   ```rust");
    println!("   use paladin::application::use_cases::sanctum::{{");
    println!("       RagRetrievalService, MemoryExtractionService");
    println!("   }};");
    println!();
    println!("   let rag_service = Arc::new(RagRetrievalService::new(");
    println!("       sanctum_port, embedding_port, rag_config");
    println!("   ));");
    println!();
    println!("   let memory_service = Arc::new(MemoryExtractionService::new(");
    println!("       llm_port, embedding_port, sanctum_port");
    println!("   ));");
    println!();
    println!("   let execution_service = PaladinExecutionService::new(llm_port)");
    println!("       .with_rag_retrieval(rag_service)");
    println!("       .with_memory_extraction(memory_service);");
    println!("   ```\n");

    // ========================================================================
    // Example Workflow
    // ========================================================================
    println!("📝 Example Workflow\n");

    println!("   Session 1 (Building Knowledge):");
    println!("   ─────────────────────────────────────────────────────────────");
    println!("   User: \"What is Rust's ownership system?\"");
    println!("   → No prior context (first interaction)");
    println!("   → Paladin responds with explanation");
    println!("   → Memory extracted: \"Rust ownership ensures memory safety\"");
    println!("   → Stored in Sanctum with embedding\n");

    println!("   User: \"How does borrowing work?\"");
    println!("   → Retrieves: Previous memory about ownership");
    println!("   → Context injected into prompt");
    println!("   → Paladin responds referencing ownership concept");
    println!("   → Memory extracted: \"Borrowing allows references...\"");
    println!("   → Stored in Sanctum\n");

    println!("   Session 2 (Using Knowledge):");
    println!("   ─────────────────────────────────────────────────────────────");
    println!("   User: \"Explain Rust's memory management\"");
    println!("   → Retrieves: Top 3 memories (ownership, borrowing, etc.)");
    println!("   → Context injected: Previous explanations");
    println!("   → Paladin: \"Based on our earlier discussion...\"");
    println!("   → Response quality improved with historical context\n");

    // ========================================================================
    // Performance Tuning
    // ========================================================================
    println!("🎯 Performance Tuning\n");

    println!("   Parameter         | Effect                  | Recommendation");
    println!("   ──────────────────┼─────────────────────────┼───────────────────");
    println!("   top_k             | Number of memories      | Start with 5");
    println!("   min_similarity    | Relevance threshold     | 0.6-0.8 range");
    println!("   max_tokens        | Context budget          | 1000-2000");
    println!("   timeout_seconds   | Retrieval time limit    | 5 seconds");
    println!();
    println!("   Trade-offs:");
    println!("   • Higher top_k → More context but slower, more expensive");
    println!("   • Lower similarity → More memories but less relevant");
    println!("   • Higher max_tokens → Better context but token cost\n");

    // ========================================================================
    // Best Practices
    // ========================================================================
    println!("✅ Best Practices\n");

    println!("   1. Start Simple:");
    println!("      • Use in_memory Sanctum for development");
    println!("      • Default RAG configuration");
    println!("      • OnCompletion extraction strategy\n");

    println!("   2. Tune Gradually:");
    println!("      • Monitor retrieval quality");
    println!("      • Adjust similarity threshold");
    println!("      • Optimize top_k based on use case\n");

    println!("   3. Production Setup:");
    println!("      • Deploy Qdrant for scalability");
    println!("      • Use production embeddings (OpenAI, etc.)");
    println!("      • Set appropriate timeouts");
    println!("      • Monitor memory storage costs\n");

    println!("   4. Memory Management:");
    println!("      • Set eviction policies for old memories");
    println!("      • Use metadata filtering (paladin_id, date, etc.)");
    println!("      • Regular cleanup of low-importance memories\n");

    // ========================================================================
    // Troubleshooting
    // ========================================================================
    println!("🔧 Troubleshooting\n");

    println!("   Issue: No memories retrieved");
    println!("   Solution:");
    println!("   • Lower min_similarity threshold");
    println!("   • Verify Sanctum has stored memories");
    println!("   • Check embeddings are being generated\n");

    println!("   Issue: Irrelevant context retrieved");
    println!("   Solution:");
    println!("   • Increase min_similarity threshold");
    println!("   • Reduce top_k to fewer, better matches");
    println!("   • Improve memory extraction prompts\n");

    println!("   Issue: Slow execution");
    println!("   Solution:");
    println!("   • Reduce top_k");
    println!("   • Lower timeout_seconds");
    println!("   • Optimize Sanctum queries (indexes, filters)\n");

    // ========================================================================
    // Additional Resources
    // ========================================================================
    println!("📚 Additional Resources\n");

    println!("   Documentation:");
    println!("   • docs/SANCTUM.md - Complete RAG guide");
    println!("   • docs/SANCTUM_DEPLOYMENT.md - Production deployment");
    println!("   • docs/guides/memory-management.md - Memory strategies\n");

    println!("   Examples:");
    println!("   • examples/paladin_with_sanctum.rs - Memory operations");
    println!("   • examples/sanctum_qdrant_production.rs - Qdrant setup");
    println!("   • examples/cli_configs/paladin_rag.yaml - Full config\n");

    println!("   Tests:");
    println!("   • tests/integration/rag_integration_tests.rs - Config validation");
    println!("   • tests/unit/sanctum/rag_retrieval_service_test.rs - Unit tests");
    println!("   • tests/unit/sanctum/memory_extraction_service_test.rs - Unit tests\n");

    // ========================================================================
    // Summary
    // ========================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("🎉 Summary");
    println!("═══════════════════════════════════════════════════════════════════\n");

    println!("RAG in Paladin enables:");
    println!("✨ Automatic knowledge retention across sessions");
    println!("✨ Context-aware responses using historical data");
    println!("✨ Improved accuracy through memory retrieval");
    println!("✨ Scalable long-term memory with vector databases\n");

    println!("Get started:");
    println!("1. Configure Sanctum in config.yml");
    println!("2. Enable RAG with default settings");
    println!("3. Run your Paladin agent");
    println!("4. Watch memories accumulate and context improve!\n");

    println!("For a working demonstration, see:");
    println!("   cargo run --example paladin_with_sanctum\n");
}
