//! `SeaORM` persistence: entities and migrations mirroring the `SQLx` schema.

#![warn(missing_docs)]

pub mod entities;
pub mod migration;

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

pub use migration::Migrator;

/// Applies pending `SeaORM` migrations against the given connection.
///
/// # Errors
///
/// Returns [`sea_orm::DbErr`] when migration execution fails.
pub async fn migrate(connection: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    migration::Migrator::up(connection, None).await
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
