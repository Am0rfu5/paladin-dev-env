# paladin-storage

SQL-backed repository adapters for the Paladin framework.

## Purpose

`paladin-storage` provides repository implementations for SQL persistence in SQLite and MySQL environments.

## Key Modules

- `sqlite_content_repository`: SQLite-backed content repository.
- `sqlite_user_repository`: SQLite-backed user repository.
- `mysql_content_repository`: MySQL-backed content repository.

## Usage

```rust
use paladin_storage::sqlite_content_repository;
use paladin_storage::sqlite_user_repository;

// Instantiate repository adapters in the infrastructure layer.
let _content_repo = std::any::type_name::<sqlite_content_repository::SqliteContentRepository>();
let _user_repo = std::any::type_name::<sqlite_user_repository::SqliteUserRepository>();
```

## Feature Flags

- `default = ["sqlite"]`
- `sqlite`: Enable SQLite repository adapters.
- `mysql`: Enable MySQL repository adapters.
