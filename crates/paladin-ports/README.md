# paladin-ports

Hexagonal architecture contracts for the Paladin framework.

## Purpose

`paladin-ports` contains trait-based input and output ports that define boundaries between core/application logic and external adapters.

## Key Modules

- `input`: Inbound contracts for ingestion and document-processing workflows.
- `output`: Outbound contracts for LLMs, memory, storage, notifications, and tool systems.

## Usage

```rust
use paladin_ports::input;
use paladin_ports::output;

// Adapter crates implement these traits to integrate external systems.
let _input_namespace = std::any::type_name::<input::content_input_port::ContentInputPort>();
let _output_namespace = std::any::type_name::<output::llm_port::LlmPort>();
```

## Feature Flags

This crate has no feature flags.
