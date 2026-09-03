//! `SeaORM` persistence: entities and migrations mirroring the `SQLx` schema.

#![warn(missing_docs)]

pub mod catalog;
pub mod entities;
pub mod migration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

pub use catalog::SeaOrmCatalogRepository;
pub use migration::Migrator;

/// SQL used by [`seed_catalog`] and `make db-seed`.
pub const CATALOG_SEED_SQL: &str = include_str!("../../../db/seeds/catalog.sql");

/// Applies pending `SeaORM` migrations against the given connection.
///
/// # Errors
///
/// Returns [`sea_orm::DbErr`] when migration execution fails.
pub async fn migrate(connection: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    migration::Migrator::up(connection, None).await
}

/// Inserts the catalog seed rows (`ON CONFLICT DO NOTHING`).
///
/// # Errors
///
/// Returns [`sea_orm::DbErr`] when the script fails.
pub async fn seed_catalog(connection: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    connection.execute_unprepared(CATALOG_SEED_SQL).await?;
    Ok(())
}

/// Connects with `DATABASE_URL` and returns a catalog repository.
///
/// # Errors
///
/// Returns [`MigrateError`] when the URL is missing or the database is unreachable.
pub async fn catalog_from_env() -> Result<SeaOrmCatalogRepository, MigrateError> {
    let url = std::env::var("DATABASE_URL").map_err(|_| MigrateError::MissingDatabaseUrl)?;
    let mut options = ConnectOptions::new(url);
    options.max_connections(5);
    let connection = Database::connect(options).await?;
    Ok(SeaOrmCatalogRepository::new(connection))
}

/// Connects with `DATABASE_URL` and runs pending migrations.
///
/// # Errors
///
/// Returns [`MigrateError`] when the database is unreachable or migrations fail.
pub async fn migrate_from_env() -> Result<(), MigrateError> {
    let url = std::env::var("DATABASE_URL").map_err(|_| MigrateError::MissingDatabaseUrl)?;
    let connection = Database::connect(&url).await?;
    migrate(&connection).await?;
    Ok(())
}

/// Migration runner errors.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
    /// `DATABASE_URL` is not set.
    #[error("DATABASE_URL must be set")]
    MissingDatabaseUrl,
    /// Underlying `SeaORM` database error.
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_sql_mirror_is_non_empty() {
        assert_ne!(migration::INIT_SQL.trim(), "");
    }
}
