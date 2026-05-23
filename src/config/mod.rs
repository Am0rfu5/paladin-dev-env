// Internal modules (public for testing, not part of stable API)
#[allow(missing_docs)]
pub mod application_settings;

// Re-export the canonical entry-points so callers can write
// `use paladin::config::Settings;` (or `crate::config::Settings;`).
// All domain config types are also re-exported here for forwards-compatibility.
#[allow(unused_imports)]
pub use application_settings::*;
#[allow(missing_docs)]
pub mod arsenal;
#[allow(missing_docs)]
pub mod citadel;
#[allow(missing_docs)]
pub mod env_utils;
#[allow(missing_docs)]
pub mod file_storage;
#[allow(missing_docs)]
pub mod herald;
#[allow(missing_docs)]
pub mod notifications;
#[allow(missing_docs)]
pub mod queue;
#[allow(missing_docs)]
pub mod scheduler;
#[allow(missing_docs)]
pub mod setup;
#[allow(missing_docs)]
pub mod user_config;
#[allow(missing_docs)]
pub mod web_server;
