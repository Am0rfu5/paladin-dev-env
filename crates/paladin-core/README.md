# paladin-core

Pure domain types for the Paladin framework with zero infrastructure dependencies.

## Purpose

`paladin-core` defines foundational entities and value objects using the Node pattern.

## Key Modules

- `base`: Core primitives like `Node<T>`, collections, messages, and fields.
- `platform`: Domain entities for Paladins, Battalions, Garrison, Arsenal, Citadel, Herald, and Sanctum.

## Usage

```rust
use paladin_core::base;
use paladin_core::platform;

// Import domain primitives and aggregate types from the core layer.
let _node_module = base::node::Node::<String>::new("paladin".to_string());
let _status = platform::task::TaskStatus::Pending;
```

## Feature Flags

This crate has no feature flags.
