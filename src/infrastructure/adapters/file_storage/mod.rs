//! File-storage adapters — re-exported from `paladin-storage` (the `s3` feature).
#[cfg(feature = "s3-storage")]
pub use paladin_storage::minio;
