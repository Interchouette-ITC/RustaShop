//! Persistence facade. Compile exactly one backend: `persist-sqlx` (default) or `persist-seaorm`.
//!
//! Enable `SeaORM` with `--no-default-features --features persist-seaorm`. Enabling both
//! features (easy to do if you add `persist-seaorm` without disabling the default) fails at
//! compile time. Repository adapters register through [`PersistenceFactory`] once they exist.

#[cfg(all(feature = "persist-sqlx", feature = "persist-seaorm"))]
compile_error!("enable only one of persist-sqlx or persist-seaorm");

#[cfg(not(any(feature = "persist-sqlx", feature = "persist-seaorm")))]
compile_error!("enable persist-sqlx or persist-seaorm");

/// Compile-time persistence backend selected by Cargo features.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistenceBackend {
    /// Hand-written `SQLx` queries and migrations.
    Sqlx,
    /// `SeaORM` entities and migrations.
    Seaorm,
}

/// Resolves the compile-time persistence backend for composition roots.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PersistenceFactory;

impl PersistenceFactory {
    /// Backend compiled into this crate.
    #[must_use]
    pub const fn backend(self) -> PersistenceBackend {
        selected_backend()
    }

    /// Runs the selected backend's migrations using `DATABASE_URL`.
    ///
    /// # Errors
    ///
    /// Returns [`MigrateError`] when the URL is missing or the backend fails.
    pub async fn migrate_from_env(self) -> Result<(), MigrateError> {
        migrate_from_env().await
    }
}

/// Returns the backend compiled into this crate.
#[must_use]
pub const fn selected_backend() -> PersistenceBackend {
    #[cfg(feature = "persist-sqlx")]
    {
        PersistenceBackend::Sqlx
    }
    #[cfg(feature = "persist-seaorm")]
    {
        PersistenceBackend::Seaorm
    }
}

/// Connects with `DATABASE_URL` and runs pending migrations for the selected backend.
///
/// # Errors
///
/// Returns [`MigrateError`] when the database is unreachable or migrations fail.
pub async fn migrate_from_env() -> Result<(), MigrateError> {
    #[cfg(feature = "persist-sqlx")]
    {
        Ok(rustashop_persist_sqlx::migrate_from_env().await?)
    }
    #[cfg(feature = "persist-seaorm")]
    {
        Ok(rustashop_persist_seaorm::migrate_from_env().await?)
    }
}

/// Migration runner errors from the selected backend.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
    /// Underlying `SQLx` migration error.
    #[cfg(feature = "persist-sqlx")]
    #[error(transparent)]
    Sqlx(#[from] rustashop_persist_sqlx::MigrateError),
    /// Underlying `SeaORM` migration error.
    #[cfg(feature = "persist-seaorm")]
    #[error(transparent)]
    Seaorm(#[from] rustashop_persist_seaorm::MigrateError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_selects_compiled_backend() {
        let backend = PersistenceFactory.backend();
        #[cfg(feature = "persist-sqlx")]
        assert_eq!(backend, PersistenceBackend::Sqlx);
        #[cfg(feature = "persist-seaorm")]
        assert_eq!(backend, PersistenceBackend::Seaorm);
        assert_eq!(backend, selected_backend());
    }
}
