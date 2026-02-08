//! CLI configuration module

pub mod battalion_config;
pub mod loader;
pub mod paladin_config;

pub use battalion_config::BattalionYamlConfig;
pub use loader::{load_paladin_config, load_battalion_config};
pub use paladin_config::PaladinYamlConfig;
