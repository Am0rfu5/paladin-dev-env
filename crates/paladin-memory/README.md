# paladin-memory

Memory adapters for the Paladin framework.

## Purpose

`paladin-memory` provides conversation memory (Garrison) and long-term semantic memory (Sanctum) implementations plus supporting services.

## Key Modules

- `config`: Configuration types for memory systems.
- `garrison`: Conversation history adapters.
- `sanctum`: Vector memory adapters.
- `services`: Retrieval and extraction services.
- `prelude`: Re-exported commonly used memory traits and types.

## Usage

```rust
use paladin_memory::garrison;
use paladin_memory::sanctum;

// Select an adapter from garrison/sanctum based on deployment needs.
let _garrison_module = std::any::type_name::<garrison::in_memory_garrison::InMemoryGarrison>();
let _sanctum_module = std::any::type_name::<sanctum::in_memory_adapter::InMemorySanctumAdapter>();
```

## Feature Flags

- `default = []`
- `sqlite`: Enable SQLite-backed Garrison adapters.
- `qdrant`: Enable Qdrant-backed Sanctum adapters.
- `content-processing`: Enable token-aware memory processing helpers.
