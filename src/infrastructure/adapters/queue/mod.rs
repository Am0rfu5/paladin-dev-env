//! Queue adapters — re-exported from `paladin-storage` (the `redis-queue` feature).
#[cfg(feature = "redis-queue")]
pub use paladin_storage::redis;
