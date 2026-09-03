//! `SQLx` persistence: versioned SQL migrations, queries, and catalog repositories.

pub mod cart;
pub mod catalog;

use sqlx::postgres::PgPool;

pub use catalog::SqlxCatalogRepository;

/// SQL used by [`seed_catalog`] and `make db-seed`.
pub const CATALOG_SEED_SQL: &str = include_str!("../../../db/seeds/catalog.sql");

/// Applies embedded `SQLx` migrations against the given pool.
///
/// # Errors
///
/// Returns [`sqlx::migrate::MigrateError`] when migration execution fails.
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}

/// Inserts the catalog seed rows (`ON CONFLICT DO NOTHING`).
///
/// # Errors
///
/// Returns [`sqlx::Error`] when a statement fails.
pub async fn seed_catalog(pool: &PgPool) -> Result<(), sqlx::Error> {
    for statement in CATALOG_SEED_SQL.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        sqlx::query(statement).execute(pool).await?;
    }
    Ok(())
}

/// Connects with `DATABASE_URL` and returns a catalog repository.
///
/// # Errors
///
/// Returns [`MigrateError`] when the URL is missing or the database is unreachable.
pub async fn catalog_from_env() -> Result<SqlxCatalogRepository, MigrateError> {
    let url = std::env::var("DATABASE_URL").map_err(|_| MigrateError::MissingDatabaseUrl)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    Ok(SqlxCatalogRepository::new(pool))
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
