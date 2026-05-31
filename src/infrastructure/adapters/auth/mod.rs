//! In-memory opaque-token authentication adapter.

/// Concrete [`AuthPort`](paladin_ports::output::auth_port::AuthPort)
/// implementation backed by an in-memory store of hashed tokens.
pub mod in_memory_token_auth_adapter;

pub use in_memory_token_auth_adapter::InMemoryTokenAuthAdapter;
