// Include the mysql_content_repository_test module directly.
// Requires `storage-mysql` feature to compile paladin_storage::mysql_content_repository.
#[cfg(feature = "storage-mysql")]
#[path = "repository/mysql_content_repository_test.rs"]
mod mysql_content_repository_test;
