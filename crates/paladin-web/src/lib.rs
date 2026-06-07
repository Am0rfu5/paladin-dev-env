//! Web server adapters for the Paladin AI orchestration framework.
//!
//! Provides HTTP server components built with `axum`, including user management
//! REST endpoints and content-delivery routes/adapters.

#![warn(missing_docs)]

/// Web-facing adapter integrations.
#[allow(missing_docs)]
pub mod adapters;
/// Application router composition for the user REST API.
pub mod app;
/// Authentication and RBAC middleware for the user REST API.
pub mod auth_middleware;
/// Content-delivery controller handlers (axum).
pub mod delivery_controller;
/// User management controller handlers.
#[allow(missing_docs)]
pub mod user_controller;
