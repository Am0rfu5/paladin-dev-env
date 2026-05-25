/*
SQL Database Application Repository

Re-exports repository port traits from `paladin-ports`.  The trait definitions
live in `paladin_ports::output::repository_port` so that infrastructure crates
(e.g. `paladin-storage`) can implement them without depending on this facade.

TODO Should this be renamed as the ContentSqlRepository in the content_sql_store.rs?
*/
pub use paladin_ports::output::repository_port::{
    ContentListRepository, ContentRepository, MigrationManager, RepositoryError, RepositoryStats,
    SqlStore, TransactionManager,
};
