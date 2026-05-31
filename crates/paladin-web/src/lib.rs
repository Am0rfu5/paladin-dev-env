//! Web server adapters for the Paladin AI orchestration framework.
//!
//! Provides HTTP server components using `actix-web` and `axum`, including
//! user management REST endpoints and content delivery adapters.

#![warn(missing_docs)]

/// Web-facing adapter integrations.
#[allow(missing_docs)]
pub mod adapters;
/// Authentication and RBAC middleware for the user REST API.
pub mod auth_middleware;
/// User management controller handlers.
#[allow(missing_docs)]
pub mod user_controller;
