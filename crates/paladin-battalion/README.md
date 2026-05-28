# paladin-battalion

Multi-agent orchestration runtime for the Paladin framework.

## Purpose

`paladin-battalion` implements orchestration execution patterns used to coordinate multiple Paladins.

## Key Modules

- `formation_service`: Sequential handoff execution.
- `phalanx_service`: Concurrent fan-out/fan-in execution.
- `campaign_service`: Directed graph orchestration.
- `chain_of_command_service`: Hierarchical delegation.
- `conclave_execution_service`, `council_service`, `grove_service`, `maneuver`, `commander`: Advanced strategy and routing patterns.

## Usage

```rust
use paladin_battalion::formation_service;
use paladin_battalion::phalanx_service;

// Choose an orchestration pattern module and construct its service in application code.
let _formation_module = std::any::type_name::<formation_service::FormationService>();
let _phalanx_module = std::any::type_name::<phalanx_service::PhalanxService>();
```

## Feature Flags

This crate has no feature flags.
