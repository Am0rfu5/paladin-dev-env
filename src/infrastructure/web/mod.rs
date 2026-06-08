/// Build a `paladin-web` agent registry from configuration (Milestone 12, Epic 2).
#[cfg(feature = "web-server")]
pub mod agent_host;

#[cfg(feature = "web-server")]
pub use paladin_web::*;
