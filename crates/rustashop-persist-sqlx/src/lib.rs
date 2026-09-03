//! `SQLx` persistence: versioned SQL migrations, queries, and catalog repositories.

pub mod catalog;

use sqlx::postgres::PgPool;

pub use catalog::SqlxCatalogRepository;

/// Applies embedded `SQLx` migrations against the given pool.
///
/// # Errors
///
/// Returns [`sqlx::migrate::MigrateError`] when migration execution fails.
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}

/// Connects with `DATABASE_URL` and runs embedded migrations.
///
/// # Errors
///
/// Returns [`MigrateError`] when the database is unreachable or migrations fail.
pub async fn migrate_from_env() -> Result<(), MigrateError> {
    let url = std::env::var("DATABASE_URL").map_err(|_| MigrateError::MissingDatabaseUrl)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await?;
    migrate(&pool).await?;
    Ok(())
}

/// Migration runner errors.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
    /// `DATABASE_URL` is not set.
    #[error("DATABASE_URL must be set")]
    MissingDatabaseUrl,
    /// Underlying `SQLx` connection error.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    /// Underlying `SQLx` migration error.
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
}
