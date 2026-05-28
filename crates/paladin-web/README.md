# paladin-web

Web server adapters for the Paladin framework.

## Purpose

`paladin-web` hosts HTTP-facing adapters and controllers that expose Paladin capabilities through web frameworks.

## Key Modules

- `adapters`: Web framework adapter implementations.
- `user_controller`: User-oriented controller entry points.

## Usage

```rust
use paladin_web::adapters;
use paladin_web::user_controller;

// Wire web adapters/controllers into your runtime-specific server bootstrap.
let _adapters_module = std::any::type_name::<adapters::user_web_adapter::UserWebAdapter>();
let _controller_module = std::any::type_name::<user_controller::UserController>();
```

## Feature Flags

This crate has no feature flags.
