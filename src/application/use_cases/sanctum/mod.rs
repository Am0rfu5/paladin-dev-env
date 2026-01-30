/// Sanctum Use Cases - RAG and Memory Management
///
/// This module contains use cases for the Sanctum long-term memory system,
/// including RAG (Retrieval-Augmented Generation) and memory extraction services.
pub mod memory_extraction_service;
pub mod rag_retrieval_service;

// Re-exports
pub use memory_extraction_service::{
    ExtractedMemory, MemoryExtractionService, MemoryExtractionStrategy,
};
pub use rag_retrieval_service::{
    RagConfig, RagRetrievalService, RetrievalTrigger, retrieve_context_with_timeout,
};
